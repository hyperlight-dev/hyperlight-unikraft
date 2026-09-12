#!/usr/bin/env python3
"""Run one demo/example end to end under hluk and assert its output.

Cross-platform (Linux + Windows): the same script backs `just demos` on both,
and the `demos` CI job on both runners.  It only *runs* a demo — the guest
rootfs CPIOs are built beforehand (by `just demos` on Linux, or restored from
the CI cache) — so this needs no Docker and works wherever hluk does.

Usage: run_demo.py <demo>
  demos: supply-chain autonomous http-flask http-express http-kestrel pptx
         agent-fw-local
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


def serve_and_get(args, expected: str, demo: str):
    """Start an in-guest HTTP server, GET / until it answers, assert the body."""
    proc = subprocess.Popen(
        [str(HLUK), *args], cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
    )
    try:
        body = None
        for _ in range(60):
            if proc.poll() is not None:
                sys.exit(f"::error::{demo}: server exited before answering\n{proc.stdout.read()}")
            try:
                with urllib.request.urlopen("http://127.0.0.1:8080/", timeout=3) as r:
                    body = r.read().decode(errors="replace")
                    break
            except Exception:
                time.sleep(2)
        if body is None:
            sys.exit(f"::error::{demo}: server never came up on :8080\n{proc.stdout.read()}")
        print(f"response: {body.strip()}")
        if expected not in body:
            sys.exit(f"::error::{demo}: unexpected response (wanted {expected!r})")
        print(f"ok: {demo} — got expected response")
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            proc.kill()


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
