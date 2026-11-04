#!/usr/bin/env python3
"""Regression tests for ci/run_demo.py's demo-server handling (_DrainedServer).

Two ways the HTTP demos used to hang the harness, both covered here:
  * reading hluk's stdout before it exits blocked forever (a wedged guest — the
    arm64 nightly's `demos` job pinned at its 90-minute ceiling); and
  * never reading it let a chatty server fill the ~64 KiB pipe and block the
    guest mid-request.
`_DrainedServer` captures stdout to a file and always stops the process within
a bounded time, so neither can hang the harness.

Run with: python ci/test_run_demo.py   (or, from the repo root: python -m unittest -v ci.test_run_demo)
"""
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import run_demo  # noqa: E402

BOUND = 20  # generous ceiling for a stop(); real stops are ~instant


def _server(code: str) -> "run_demo._DrainedServer":
    return run_demo._DrainedServer([sys.executable, "-c", code])


def _wait(predicate, timeout: float) -> bool:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.02)
    return predicate()


class DrainedServerTest(unittest.TestCase):
    def test_stops_a_live_server_promptly(self):
        # A server that outlives any CI job: stop() must return at once, not
        # block on the never-closing stdout (the original 90-minute hang).
        srv = _server("import time; print('serving', flush=True); time.sleep(600)")
        self.addCleanup(srv.stop)
        start = time.monotonic()
        srv.stop()
        elapsed = time.monotonic() - start
        self.assertLess(elapsed, BOUND, f"stop() blocked for {elapsed:.1f}s on a live server")
        self.assertIsNotNone(srv.exited(), "server must be stopped after stop()")

    def test_captures_server_output(self):
        # Output produced before exit is drained and returned for the error
        # messages, without a read racing the process lifetime.
        srv = _server("print('hello from the server')")
        self.addCleanup(srv.stop)
        self.assertTrue(_wait(lambda: srv.exited() is not None, 10), "server did not exit")
        self.assertIn("hello from the server", srv.stop())

    def test_drains_chatty_server_without_blocking(self):
        # ~2 MiB of stdout — beyond even a pipe grown to Linux's ~1 MiB max (the
        # default is ~64 KiB) — then exit.  Capturing to a file means the writer
        # never blocks, so the child runs to completion; a regression to an
        # undrained pipe would wedge it once the buffer filled and never exit.
        srv = _server(
            "import sys\n"
            "for _ in range(2048): sys.stdout.write('x' * 1024 + '\\n')\n"
            "sys.stdout.flush()\n"
        )
        self.addCleanup(srv.stop)
        self.assertTrue(
            _wait(lambda: srv.exited() is not None, 15),
            "chatty server blocked on a full stdout pipe",
        )
        self.assertIn("x", srv.stop())  # the log tail was captured

    @unittest.skipIf(sys.platform == "win32", "SIGTERM semantics are POSIX-only")
    def test_falls_back_to_kill_when_sigterm_is_ignored(self):
        # The child ignores SIGTERM, so stop() must escalate to SIGKILL.  A
        # marker file (written right after the handler is installed) makes the
        # readiness wait deterministic — no sleep racing interpreter startup.
        with tempfile.TemporaryDirectory() as d:
            marker = Path(d) / "ready"
            srv = _server(
                "import signal, time\n"
                "signal.signal(signal.SIGTERM, signal.SIG_IGN)\n"
                f"open({str(marker)!r}, 'w').close()\n"
                "time.sleep(600)\n"
            )
            self.addCleanup(srv.stop)
            self.assertTrue(_wait(marker.exists, 10), "child never signalled readiness")
            start = time.monotonic()
            srv.stop(grace=1)
            elapsed = time.monotonic() - start
        # ~1s grace, with tolerance for timer granularity on a loaded runner.
        self.assertGreaterEqual(elapsed, 0.9, "kill fallback not exercised (SIGTERM wasn't ignored)")
        self.assertLess(elapsed, BOUND, f"kill fallback blocked for {elapsed:.1f}s")
        self.assertIsNotNone(srv.exited(), "server must be killed after SIGTERM is ignored")

    def test_large_log_keeps_head_and_tail(self):
        # A log past _KEEP must still surface the start (where a startup error
        # lives) and the end, not only the tail.
        srv = _server(
            "import sys\n"
            "print('HEAD_MARKER', flush=True)\n"
            "for _ in range(512): sys.stdout.write('y' * 1024 + '\\n')\n"  # ~512 KiB > _KEEP
            "print('TAIL_MARKER')\n"
        )
        self.addCleanup(srv.stop)
        self.assertTrue(_wait(lambda: srv.exited() is not None, 15), "server did not exit")
        out = srv.stop()
        self.assertIn("HEAD_MARKER", out)
        self.assertIn("TAIL_MARKER", out)
        self.assertIn("omitted", out)


if __name__ == "__main__":
    unittest.main()
