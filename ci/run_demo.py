#!/usr/bin/env python3
"""Run one demo/example end to end under hluk and assert its output.

Cross-platform (Linux + Windows): the same script backs `just demos` on both,
and the `demos` CI job on both runners.  It only *runs* a demo — the guest
rootfs CPIOs are built beforehand (by `just demos` on Linux, or restored from
the CI cache) — so this needs no Docker and works wherever hluk does.

Usage: run_demo.py <demo>
  demos: supply-chain php2shell autonomous http-flask http-express http-kestrel
         pptx agent-fw-local
"""
import os
import subprocess
import sys
import tempfile
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BUILD = ROOT / "build-elfloader"
HLUK = ROOT / "target" / "release" / ("hluk.exe" if os.name == "nt" else "hluk")


def initrd(runtime: str) -> Path:
    p = BUILD / f"{runtime}-rootfs.cpio"
    if not p.is_file():
        sys.exit(f"::error::missing {p} — build it first (just demos <demo> / CI rootfs job)")
    return p


def run(args, timeout=120) -> str:
    """Run hluk to completion, return combined stdout+stderr."""
    cp = subprocess.run(
        [str(HLUK), *args], cwd=ROOT, capture_output=True, text=True, timeout=timeout
    )
    out = cp.stdout + cp.stderr
    print(out)
    return out


def expect(out: str, needle: str, demo: str):
    if needle not in out:
        sys.exit(f"::error::{demo}: expected {needle!r} in output")
    print(f"ok: {demo} — found {needle!r}")


class _DrainedServer:
    """A demo server subprocess whose stdout+stderr are captured to a file.

    The HTTP demos never close their stdout while they serve, which bites a
    naive harness two ways: reading the pipe before the process exits blocks
    until it does, and *not* reading it lets a chatty server fill the ~64 KiB
    pipe buffer and block the guest mid-request.  Capturing to a regular file
    sidesteps both — the OS never blocks the writer, and the log is read only
    after the process has stopped.  `stop()` ends it with a bounded terminate →
    kill, and reading a file never blocks, so a wedged guest — e.g. http-express
    accepting the TCP connection but never answering on the nested-virt arm64
    runner — can no longer hold the harness.  That wedge is what pinned the arm64
    nightly's demos job at its 90-minute ceiling and, by holding a runner VM the
    whole time, starved its co-tenant jobs into losing contact with the server.
    """

    _KEEP = 256 * 1024  # total log kept for error messages (the whole log if smaller)
    _HEAD = 64 * 1024   # ... of which this much is the head, where a startup error sits

    def __init__(self, cmd, cwd=None):
        self._log = tempfile.TemporaryFile()
        self._result = None
        try:
            self.proc = subprocess.Popen(cmd, cwd=cwd, stdout=self._log, stderr=subprocess.STDOUT)
        except BaseException:
            self._log.close()  # Popen never started; nothing else will close it
            raise

    def exited(self):
        """The process's exit code, or None while it is still running."""
        return self.proc.poll()

    def log_bytes(self) -> int:
        """How much the server has written to its log so far.

        `fstat` reads the size without moving the file offset, which the
        still-running child shares through its inherited stdout fd.  Returns 0
        once stopped (the log is closed and its size no longer matters).
        """
        if self._log.closed:
            return 0
        return os.fstat(self._log.fileno()).st_size

    def stop(self, grace: float = 10) -> str:
        """Stop the server and return its log (head and tail if it is large).

        Idempotent and bounded: SIGTERM, then SIGKILL, waiting at most `grace`
        for each — so at worst ~2×`grace`, never the open-ended wait that hung
        the job (reading a regular file never blocks).
        """
        if self._result is not None:
            return self._result
        self.proc.terminate()
        try:
            self.proc.wait(timeout=grace)
        except subprocess.TimeoutExpired:
            self.proc.kill()
            try:
                self.proc.wait(timeout=grace)
            except subprocess.TimeoutExpired:
                # SIGKILL could not reap it within the grace (e.g. stuck in an
                # uninterruptible syscall).  Don't pretend it stopped silently.
                print(
                    f"::warning::hluk (pid {self.proc.pid}) did not exit after SIGKILL",
                    file=sys.stderr,
                )
        data = b""
        try:
            if not self._log.closed:
                size = os.fstat(self._log.fileno()).st_size
                self._log.seek(0)
                if size <= self._KEEP:
                    data = self._log.read(size)
                else:
                    # Keep the head — where a bind/startup error or traceback
                    # sits — and the most recent tail, dropping the middle.  Read
                    # fixed windows (never to EOF) so the counts stay consistent
                    # even if a not-yet-reaped child is still appending past size.
                    tail_len = self._KEEP - self._HEAD
                    head = self._log.read(self._HEAD)
                    self._log.seek(size - tail_len)
                    tail = self._log.read(tail_len)
                    omitted = size - self._KEEP
                    data = head + f"\n...[{omitted} bytes omitted]...\n".encode() + tail
        except OSError:
            pass  # best-effort diagnostics: a read error must not break the stop
        finally:
            try:
                self._log.close()
            except OSError:
                pass
        self._result = data.decode("utf-8", "replace")
        return self._result


# Without the pipe's backpressure a flooding-but-unresponsive server would grow
# the capture file unbounded across the ~300 s poll window; fail the demo well
# before it can fill the runner's disk and take its co-tenants down with it.
_MAX_LOG = 64 * 1024 * 1024


def _fail(demo: str, reason: str, server: "_DrainedServer"):
    """Stop the server, print its captured log, then emit a one-line error.

    The log goes to the step's stdout, not into the `::error::` annotation: a
    workflow command keeps only its first line and caps the annotation's size,
    so a multi-line log embedded there would be lost.
    """
    log = server.stop()
    if log.strip():
        print(f"--- {demo} server log ---\n{log}")
    sys.exit(f"::error::{demo}: {reason}")


def serve_and_get(args, expected: str, demo: str):
    """Start an in-guest HTTP server, GET / until it answers, assert the body."""
    server = _DrainedServer([str(HLUK), *args], cwd=ROOT)
    try:
        body = None
        for _ in range(60):
            if server.exited() is not None:
                _fail(demo, "server exited before answering", server)
            if server.log_bytes() > _MAX_LOG:
                _fail(demo, f"server logged over {_MAX_LOG >> 20} MiB without answering", server)
            try:
                with urllib.request.urlopen("http://127.0.0.1:8080/", timeout=3) as r:
                    body = r.read().decode(errors="replace")
                    break
            except Exception:
                time.sleep(2)
        if body is None:
            _fail(demo, "server never came up on :8080", server)
        print(f"response: {body.strip()}")
        if expected not in body:
            _fail(demo, f"unexpected response (wanted {expected!r})", server)
        print(f"ok: {demo} — got expected response")
    finally:
        server.stop()


def demo_supply_chain():
    # Reproduce the demo's guest dir (victim app + malicious package + input),
    # then run the victim in a scoped sandbox: mount only, no network.
    import shutil

    src = ROOT / "demos" / "supply-chain"
    guest = src / "guest"
    (guest / "reqeusts").mkdir(parents=True, exist_ok=True)
    (guest / "workspace").mkdir(parents=True, exist_ok=True)
    shutil.copy(src / "victim_app.py", guest / "victim_app.py")
    for f in (src / "reqeusts").glob("*.py"):
        shutil.copy(f, guest / "reqeusts" / f.name)
    (guest / "workspace" / "input.txt").write_text("Hello from the host filesystem\n")
    out = run([
        "run", "--initrd", str(initrd("python")), "--scratch-mb", "256",
        "--mount", f"{guest}:/host",
        "--exec", "import sys; sys.path.insert(0, '/host'); exec(open('/host/victim_app.py').read())",
    ])
    expect(out, "Attack CONTAINED", "supply-chain")


def demo_php2shell():
    # The attacker's payload runs as if a PHP engine flaw had handed them
    # code execution, with the document root mounted read-only and no
    # network: every phase (host secrets, env secrets, C2, cloud metadata,
    # a persistent webshell) must be contained.
    webroot = ROOT / "demos" / "php2shell" / "webroot"
    payload = ROOT / "demos" / "php2shell" / "payload.php"
    out = run([
        "run", "--initrd", str(initrd("php")), "--scratch-mb", "128",
        "--mount", f"{webroot}:/app:ro", "--env", "DOCROOT=/app",
        str(payload),
    ])
    expect(out, "Attack CONTAINED", "php2shell")


def demo_autonomous():
    out = run(["run", "--initrd", str(initrd("autonomous")), "--scratch-mb", "256"])
    expect(out, "autonomous appliance", "autonomous")


def demo_http(runtime, scratch, tail, expected):
    serve_and_get(
        ["run", "--initrd", str(initrd(runtime)), "--scratch-mb", scratch,
         "--net", "--port", "8080", *tail],
        expected, runtime,
    )


def demo_pptx():
    with tempfile.TemporaryDirectory() as out:
        code = ("from pptx import Presentation; p=Presentation(); "
                "s=p.slides.add_slide(p.slide_layouts[0]); s.shapes.title.text='Hyperlight'; "
                "s.placeholders[1].text='non-LLM smoke test'; p.save('/out/smoke.pptx'); "
                "print('pptx written')")
        run(["run", "--initrd", str(initrd("pptx")), "--scratch-mb", "512",
             "--mount", f"{out}:/out", "--exec", code])
        f = Path(out) / "smoke.pptx"
        if not f.is_file() or f.stat().st_size == 0:
            sys.exit("::error::pptx: no file generated")
        # .pptx is a zip; the local-file-header magic is 'PK\x03\x04'
        if f.read_bytes()[:4] != b"PK\x03\x04":
            sys.exit("::error::pptx: output is not a valid pptx (zip)")
        print(f"ok: pptx — {f.stat().st_size} bytes")


def demo_agent_fw_local():
    out = run(
        ["run", "--initrd", str(initrd("agent-fw-local")), "--scratch-mb", "1536",
         str(ROOT / "examples" / "agent-framework" / "local.py")],
        timeout=1200,
    )
    expect(out, "Agent:", "agent-fw-local")


DEMOS = {
    "supply-chain": demo_supply_chain,
    "php2shell": demo_php2shell,
    "autonomous": demo_autonomous,
    "http-flask": lambda: demo_http("http-flask", "256",
                                    [str(ROOT / "examples/http-server/flask/server.py")],
                                    "Hello from Flask on Hyperlight!"),
    "http-express": lambda: demo_http("http-express", "512",
                                      [str(ROOT / "examples/http-server/express/server.js")],
                                      "Hello from Express on Hyperlight!"),
    "http-kestrel": lambda: demo_http("http-kestrel", "256",
                                      ["--exec", "/app/KestrelHyperlight"],
                                      "Hello from Kestrel on Hyperlight!"),
    "pptx": demo_pptx,
    "agent-fw-local": demo_agent_fw_local,
}


def main():
    if len(sys.argv) != 2 or sys.argv[1] not in DEMOS:
        sys.exit(f"usage: {sys.argv[0]} <{'|'.join(DEMOS)}>")
    if not HLUK.is_file():
        sys.exit(f"::error::hluk not built at {HLUK}")
    DEMOS[sys.argv[1]]()


if __name__ == "__main__":
    main()
