"""Browser terminal for the az-cli shell: one micro-VM per websocket.

The page (index.html, xterm.js) opens a websocket; the server starts
examples/az-cli/run.sh under a pseudo-terminal for it, with a fresh
~/.azure of its own, and relays bytes both ways until either side closes.
The micro-VM has no inbound port: the server owns the socket.

    pip install websockets
    python3 examples/az-cli/web/server.py [--port 8080]

Listens on 127.0.0.1 only: each session is a shell that can sign in to
Azure.
"""

import argparse
import asyncio
import fcntl
import json
import os
import pty
import shutil
import signal
import struct
import tempfile
import termios
from http import HTTPStatus
from pathlib import Path

from websockets.asyncio.server import serve
from websockets.datastructures import Headers
from websockets.http11 import Response

HERE = Path(__file__).resolve().parent
RUN_SH = HERE.parent / "run.sh"
PAGE = (HERE / "index.html").read_bytes()


def page(connection, request):
    """Serve the page on every path but /ws, which is the websocket."""
    if request.path == "/ws":
        return None
    headers = Headers({"Content-Type": "text/html; charset=utf-8"})
    return Response(HTTPStatus.OK, "OK", headers, PAGE)


def set_size(fd, rows, cols):
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", rows, cols, 0, 0))


async def session(ws):
    # A sign-in belongs to its session: each gets its own ~/.azure,
    # removed when it ends.
    az_home = tempfile.mkdtemp(prefix="hluk-az-")
    pid, fd = pty.fork()
    if pid == 0:
        os.environ["AZ_HOME"] = az_home
        os.execv(str(RUN_SH), [str(RUN_SH)])

    loop = asyncio.get_running_loop()
    out = asyncio.Queue()
    # The pty reads as EOF (EIO) once the guest exits.
    def readable():
        try:
            data = os.read(fd, 65536)
        except OSError:
            data = b""
        out.put_nowait(data)
        if not data:
            loop.remove_reader(fd)
    loop.add_reader(fd, readable)
    print(f"session {pid}: started", flush=True)

    async def to_browser():
        while data := await out.get():
            await ws.send(data)
        await ws.close()

    async def from_browser():
        # Text frames are control messages (resize); binary frames are keys.
        async for msg in ws:
            if isinstance(msg, str):
                ctl = json.loads(msg)
                if ctl.get("type") == "resize":
                    set_size(fd, ctl["rows"], ctl["cols"])
            else:
                os.write(fd, msg)

    tasks = [asyncio.create_task(to_browser()), asyncio.create_task(from_browser())]
    try:
        await asyncio.wait(tasks, return_when=asyncio.FIRST_COMPLETED)
    finally:
        for t in tasks:
            t.cancel()
        loop.remove_reader(fd)
        try:
            os.kill(pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        await loop.run_in_executor(None, os.waitpid, pid, 0)
        os.close(fd)
        shutil.rmtree(az_home, ignore_errors=True)
        print(f"session {pid}: ended", flush=True)


async def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--port", type=int, default=8080)
    args = ap.parse_args()
    async with serve(session, "127.0.0.1", args.port, process_request=page):
        print(f"http://localhost:{args.port}", flush=True)
        await asyncio.get_running_loop().create_future()


if __name__ == "__main__":
    asyncio.run(main())
