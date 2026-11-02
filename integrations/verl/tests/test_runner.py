"""Tests of the runner: against a fake hluk, everywhere, and in real micro-VMs with RUN_ALL set."""

import json
import os
import re
import stat
import subprocess
import sys
import time
from unittest.mock import patch

import pytest

from verl_hyperlight import runner as hyperlight


def run_case(url, code, stdin, timeout, memory_limit_mb):
    """``run_code`` as verl calls it, with the compile timeout the run's."""
    return hyperlight.run_code(url, code, stdin, timeout, timeout, memory_limit_mb, "python")


# A stand-in for hluk: it logs each command line and acts out FAKE_HLUK_MODE.
FAKE_HLUK = f"""#!{sys.executable}
import json, os, sys, time
args = sys.argv[1:]
if args == ["--version"]:
    print(os.environ.get("FAKE_HLUK_VERSION", "hluk 0.19.0"))
    sys.exit(0)
entry = {{"args": args, "pid": os.getpid()}}
if "--mount" in args:
    shared = args[args.index("--mount") + 1].split(":")[0]
    entry["solution"] = open(os.path.join(shared, "solution.py"), encoding="utf-8", newline="").read()
    entry["stdin"] = open(os.path.join(shared, "stdin"), encoding="utf-8", newline="").read()
with open(os.environ["FAKE_HLUK_LOG"], "a") as log:
    log.write(json.dumps(entry) + "\\n")
if "--mount" not in args:  # the prepare run
    sys.exit(int(os.environ.get("FAKE_HLUK_PREPARE_RC", "0")))
mode = os.environ.get("FAKE_HLUK_MODE", "success")
if mode == "fail_before":
    sys.stderr.write('error: no runtime image called "x"\\n')
    sys.exit(1)
if mode == "hang_before":
    time.sleep(60)
if mode == "note_before":
    sys.stderr.write("note: the guest gets 256 MiB but the memory limit hluk runs under is 200 MiB\\n")
sys.stderr.write({hyperlight._STARTED_MARKER!r} + "\\n")
sys.stderr.flush()
if mode in ("success", "note_before"):
    sys.stdout.write("3\\n")
    sys.stderr.write("warning\\n")
elif mode == "runtime_error":
    sys.stderr.write("Traceback\\n")
    sys.exit(1)
elif mode == "hang":
    sys.stdout.write("partial")
    sys.stdout.flush()
    time.sleep(60)
elif mode == "flood":
    chunk = b"x" * (1 << 20)
    for _ in range(40):
        sys.stdout.buffer.write(chunk)
"""


@pytest.fixture
def fake_hluk(tmp_path, monkeypatch):
    path = tmp_path / "hluk"
    path.write_text(FAKE_HLUK)
    path.chmod(path.stat().st_mode | stat.S_IEXEC)
    log = tmp_path / "log"
    monkeypatch.setenv("HLUK_PATH", str(path))
    monkeypatch.setenv("FAKE_HLUK_LOG", str(log))
    monkeypatch.setenv("XDG_CACHE_HOME", str(tmp_path / "cache"))
    monkeypatch.setenv("HLUK_CACHE_DIR", str(tmp_path / "hluk-cache"))
    monkeypatch.setattr(hyperlight, "_prepare_failures", {})
    monkeypatch.setattr(hyperlight, "_ready", set())
    monkeypatch.setattr(hyperlight, "_early_failures", {})
    monkeypatch.setattr(hyperlight, "_hluk_paths", {})

    def calls():
        return [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []

    return calls


# --- Unit tests: hluk is faked, so these run anywhere ---


def test_command_and_success(fake_hluk):
    response, error = hyperlight.run_code("hyperlight://agent", "print(3)  # é", "1 2\n", 10, 7, 768, "python")
    assert error is None
    assert response["status"] == "Success"
    assert response["run_result"]["stdout"] == "3\n"
    assert response["run_result"]["stderr"] == "warning\n"
    prepare, run = fake_hluk()
    assert prepare["args"] == ["run", *hyperlight._guest_args("agent", 768), "--exec", "pass"]
    assert run["solution"] == "print(3)  # é" and run["stdin"] == "1 2\n"
    args = run["args"]
    assert args[args.index("--runtime") + 1] == "agent"
    assert args[args.index("--scratch-mb") + 1] == str(hyperlight._guest_memory_mb(768))
    assert args[args.index("--mount") + 1].endswith(":/verl:ro")
    assert args[args.index("--exec") + 1] == hyperlight._RUNNER
    assert args[args.index("--warm-exec") + 1].startswith("import array, bisect")


def test_prepares_once_and_defaults(fake_hluk):
    for _ in range(3):
        run_case("hyperlight://", "pass", None, 5, None)
    calls = fake_hluk()
    assert len(calls) == 4  # one prepare run, three test cases
    for call in calls:
        assert call["args"][:3] == ["run", "--runtime", "python"]
        assert "--scratch-mb" not in call["args"]


def test_nonzero_exit_is_runtime_error(fake_hluk, monkeypatch):
    monkeypatch.setenv("FAKE_HLUK_MODE", "runtime_error")
    response, _ = run_case("hyperlight://", "raise ValueError", "", 5, 256)
    # Sandbox Fusion's shape for a solution that failed, which verl scores as a runtime error
    assert response["status"] == "Failed"
    assert response["run_result"]["status"] == "Finished" and response["run_result"]["return_code"] == 1


def test_failure_before_the_code_ran_is_retried_once(fake_hluk, monkeypatch):
    run_case("hyperlight://", "pass", None, 5, 256)
    monkeypatch.setenv("FAKE_HLUK_MODE", "fail_before")
    run_case("hyperlight://", "pass", None, 5, 256)
    assert [("--mount" not in call["args"]) for call in fake_hluk()] == [True, False, False, False]
    monkeypatch.setenv("FAKE_HLUK_MODE", "hang_before")
    run_case("hyperlight://", "pass", None, 1, 256)
    assert len(fake_hluk()) == 5  # a timeout is not retried: it would double the case's time


def test_hluk_failure_is_sandbox_error(fake_hluk, monkeypatch):
    monkeypatch.setenv("FAKE_HLUK_MODE", "fail_before")
    response, _ = run_case("hyperlight://x", "pass", "", 5, 256)
    assert response["status"] == "SandboxError" and response["run_result"] is None


def test_timeout(fake_hluk, monkeypatch):
    monkeypatch.setenv("FAKE_HLUK_MODE", "hang")
    response, _ = run_case("hyperlight://", "while True: pass", "", 1, 256)
    assert response["status"] == "Failed"
    assert response["run_result"]["status"] == "TimeLimitExceeded"
    assert response["run_result"]["stdout"] == "partial"


def test_timeout_before_the_code_ran_is_sandbox_error(fake_hluk, monkeypatch):
    monkeypatch.setenv("FAKE_HLUK_MODE", "hang_before")
    response, _ = run_case("hyperlight://", "pass", None, 1, 256)
    assert response["status"] == "SandboxError" and "timed out" in response["message"]
    # One slow restore is load, not a missing snapshot: no new prepare run.
    monkeypatch.setenv("FAKE_HLUK_MODE", "success")
    run_case("hyperlight://", "pass", None, 5, 256)
    assert [("--mount" not in call["args"]) for call in fake_hluk()] == [True, False, False]


def test_repeated_timeouts_before_the_code_ran_prepare_again(fake_hluk, monkeypatch):
    run_case("hyperlight://", "pass", None, 5, 256)
    monkeypatch.setenv("FAKE_HLUK_MODE", "hang_before")
    for _ in range(hyperlight.EARLY_FAILURES_BEFORE_PREPARE):
        run_case("hyperlight://", "pass", None, 1, 256)
    monkeypatch.setenv("FAKE_HLUK_MODE", "success")
    run_case("hyperlight://", "pass", None, 5, 256)
    assert [("--mount" not in call["args"]) for call in fake_hluk()] == [True, False, False, False, False, True, False]


def test_prepare_lock_sits_beside_hluks_cache(fake_hluk, monkeypatch, tmp_path):
    run_case("hyperlight://", "pass", None, 5, 256)
    assert (tmp_path / "hluk-cache" / "verl-prepare.lock").exists()


def test_unlockable_filesystem_is_a_clear_error(fake_hluk, monkeypatch):
    # filelock 3.x raises NotImplementedError where the filesystem has no flock.
    import filelock

    def no_flock(*args, **kwargs):
        raise NotImplementedError("FileSystem does not appear to support flock")

    monkeypatch.setattr(filelock, "FileLock", no_flock)
    response, error = run_case("hyperlight://", "pass", None, 5, 256)
    assert error is None and response["status"] == "SandboxError" and "cannot lock" in response["message"]


def test_failures_are_logged(fake_hluk, monkeypatch, caplog):
    monkeypatch.setenv("FAKE_HLUK_PREPARE_RC", "1")
    run_case("hyperlight://", "pass", None, 5, 256)
    assert "preparing runtime 'python' failed" in caplog.text
    caplog.clear()
    monkeypatch.setattr(hyperlight, "_prepare_failures", {})
    monkeypatch.setenv("FAKE_HLUK_PREPARE_RC", "0")
    monkeypatch.setenv("FAKE_HLUK_MODE", "fail_before")
    run_case("hyperlight://", "pass", None, 5, 256)
    assert "hluk exited with 1 before the code ran" in caplog.text


def test_what_hluk_says_before_the_code_ran_is_logged_not_returned(fake_hluk, monkeypatch, caplog):
    monkeypatch.setenv("FAKE_HLUK_MODE", "note_before")
    monkeypatch.setattr(hyperlight, "_notes_logged", set())
    for _ in range(2):
        response, error = run_case("hyperlight://", "print(3)", None, 5, 256)
        assert error is None and response["status"] == "Success"
        assert response["run_result"]["stdout"] == "3\n"
        assert response["run_result"]["stderr"] == "warning\n"
    # The same note comes with every case; it is logged once.
    assert caplog.text.count("hluk said before the code ran: note: the guest gets 256 MiB") == 1


def test_warm_imports_cover_the_call_based_wrapper():
    # Sandbox Fusion's fn_name wrapper imports these in every call-based case; uncached, each
    # would be compiled again in every VM.
    import inspect

    utils = pytest.importorskip("verl.utils.reward_score.sandbox_fusion.utils")

    source = inspect.getsource(utils._process_single_case)
    wrapper = source[source.index("wrapper_code") : source.index("# === User's Original Code START")]
    imported = set(re.findall(r"^import (\w+)$", wrapper, re.M))
    assert imported and imported <= set(hyperlight.WARM_IMPORTS)


def test_output_is_bounded(fake_hluk, monkeypatch):
    monkeypatch.setenv("FAKE_HLUK_MODE", "flood")
    response, _ = run_case("hyperlight://", "pass", None, 30, 256)
    assert len(response["run_result"]["stdout"]) == hyperlight.MAX_OUTPUT_BYTES


def test_truncated_output_fails_the_case(fake_hluk, monkeypatch):
    # What was kept could match the expected output where the whole would not: not an answer
    monkeypatch.setenv("FAKE_HLUK_MODE", "flood")
    response, _ = run_case("hyperlight://", "pass", None, 30, 256)
    assert response["status"] == "Failed" and response["run_result"]["status"] == "Error"
    assert "stdout truncated" in response["run_result"]["stderr"]


def test_lone_surrogate_is_not_fatal(fake_hluk):
    response, error = run_case("hyperlight://", "print('\\udc80')", "\udc80", 5, 256)
    assert error is None and response["status"] == "Success"


@pytest.mark.skipif(hyperlight._setpriv_path() is None, reason="needs util-linux setpriv")
def test_hluk_dies_with_its_worker(fake_hluk, monkeypatch, tmp_path):
    # A worker killed mid-case (a reward-manager timeout, an OOM kill) must not leave a VM running.
    monkeypatch.setenv("FAKE_HLUK_MODE", "hang")
    worker = subprocess.Popen(
        [
            sys.executable,
            "-c",
            "from verl_hyperlight import runner; runner.run_code('hyperlight://', 'pass', None, 60, 60, 256)",
        ]
    )
    deadline = time.monotonic() + 120
    while not (cases := [c for c in fake_hluk() if "--mount" in c["args"]]) and time.monotonic() < deadline:
        time.sleep(0.1)
    assert cases, "the worker never started its test case"
    hluk_pid = cases[0]["pid"]
    worker.kill()
    worker.wait()
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        try:
            os.kill(hluk_pid, 0)
        except ProcessLookupError:
            return
        time.sleep(0.1)
    os.kill(hluk_pid, 9)
    raise AssertionError("hluk outlived its worker")


@pytest.mark.parametrize("version", ["hluk 0.18.0", "hluk 0.18.0+dev.44ee170", "hluk 0.19.0-rc1", ""])
def test_old_hluk_is_refused(fake_hluk, monkeypatch, version):
    monkeypatch.setenv("FAKE_HLUK_VERSION", version)
    response, _ = run_case("hyperlight://", "pass", None, 5, 256)
    assert response["status"] == "SandboxError" and "needs hluk 0.19.0 or later" in response["message"]
    assert fake_hluk() == []


def test_each_worker_prepares(fake_hluk, monkeypatch):
    run_case("hyperlight://", "pass", None, 5, 256)
    # A worker started later (a new process) prepares again: a fast restore if the snapshot is
    # still in hluk's cache, a pull under the lock if it is not.
    monkeypatch.setattr(hyperlight, "_ready", set())
    run_case("hyperlight://", "pass", None, 5, 256)
    assert [("--mount" not in call["args"]) for call in fake_hluk()] == [True, False, True, False]


def test_prepare_redone_after_hluk_keeps_failing(fake_hluk, monkeypatch):
    # One failing case (its retry included) is transient; a few in a row (hluk's cache cleaned,
    # say) prepare the guest again.
    def prepares():
        return sum("--mount" not in call["args"] for call in fake_hluk())

    run_case("hyperlight://", "pass", None, 5, 256)
    monkeypatch.setenv("FAKE_HLUK_MODE", "fail_before")
    run_case("hyperlight://", "pass", None, 5, 256)
    monkeypatch.setenv("FAKE_HLUK_MODE", "success")
    run_case("hyperlight://", "pass", None, 5, 256)
    assert prepares() == 1
    monkeypatch.setenv("FAKE_HLUK_MODE", "fail_before")
    for _ in range(hyperlight.EARLY_FAILURES_BEFORE_PREPARE):
        run_case("hyperlight://", "pass", None, 5, 256)
    assert prepares() > 1


def test_timeout_kills_what_hluk_started(fake_hluk, monkeypatch, tmp_path):
    # HLUK_PATH may be a wrapper that does not exec hluk; the whole group goes on a timeout.
    wrapper = tmp_path / "wrapper"
    wrapper.write_text(f'#!/bin/sh\n{os.environ["HLUK_PATH"]} "$@"\n')
    wrapper.chmod(0o755)
    monkeypatch.setenv("HLUK_PATH", str(wrapper))
    monkeypatch.setenv("FAKE_HLUK_MODE", "hang")
    start = time.monotonic()
    response, _ = run_case("hyperlight://", "pass", None, 1, 256)
    assert time.monotonic() - start < 10
    assert response["run_result"]["status"] == "TimeLimitExceeded"


def test_files_are_written_byte_for_byte(fake_hluk):
    run_case("hyperlight://", "a = 1\r\nprint(a)\n", "1\r\n2\n", 5, 256)
    run = fake_hluk()[-1]
    assert run["solution"] == "a = 1\r\nprint(a)\n" and run["stdin"] == "1\r\n2\n"


def test_failed_prepare_is_remembered_then_retried(fake_hluk, monkeypatch):
    monkeypatch.setenv("FAKE_HLUK_PREPARE_RC", "1")
    for _ in range(3):
        response, _ = run_case("hyperlight://", "pass", None, 5, 256)
        assert response["status"] == "SandboxError"
    assert len(fake_hluk()) == 1  # later cases fail fast instead of each retrying
    monkeypatch.setattr(hyperlight, "PREPARE_RETRY_AFTER", 0)
    monkeypatch.setenv("FAKE_HLUK_PREPARE_RC", "0")
    response, _ = run_case("hyperlight://", "pass", None, 5, 256)
    assert response["status"] == "Success"


def test_cases_dir_rejects_colons(monkeypatch, tmp_path):
    monkeypatch.setattr(hyperlight, "_cases", None)
    monkeypatch.setattr(hyperlight.tempfile, "gettempdir", lambda: "/scratch/job:1/tmp")
    monkeypatch.setenv("XDG_CACHE_HOME", str(tmp_path / "a:b"))
    for _ in range(2):  # refused every time, not only the first
        with pytest.raises(OSError):
            hyperlight._cases_dir()
    monkeypatch.setenv("XDG_CACHE_HOME", str(tmp_path / "cache"))
    assert hyperlight._cases_dir() == str(tmp_path / "cache" / "verl" / "hyperlight" / "cases")


def test_a_rootfs_path_is_booted_warm(fake_hluk):
    run_case("hyperlight:///images/my-rootfs.cpio", "pass", None, 5, 256)
    for call in fake_hluk():
        args = call["args"]
        assert args[args.index("--initrd") + 1] == "/images/my-rootfs.cpio" and "--warm" in args
        assert "--runtime" not in args


@pytest.mark.parametrize(
    "url, runtime",
    [
        ("hyperlight://", "python"),
        ("hyperlight:///", "python"),
        ("hyperlight://Agent", "agent"),
        ("HYPERLIGHT://agent/", "agent"),
    ],
)
def test_image_names(fake_hluk, url, runtime):
    run_case(url, "pass", None, 5, 768)
    args = fake_hluk()[0]["args"]
    assert args[args.index("--runtime") + 1] == runtime


@pytest.mark.parametrize("version", ["hluk 0.19.0", "hluk 0.19.0+dev.1234567", "hluk 0.20.0-rc1", "hluk 1.0.0"])
def test_recent_hluk_is_accepted(fake_hluk, monkeypatch, version):
    monkeypatch.setenv("FAKE_HLUK_VERSION", version)
    response, _ = run_case("hyperlight://", "pass", None, 5, 256)
    assert response["status"] == "Success"


def test_a_trailing_slash_on_a_rootfs_path_is_dropped(fake_hluk):
    run_case("hyperlight:///images/my-rootfs.cpio/", "pass", None, 5, 256)
    args = fake_hluk()[0]["args"]
    assert args[args.index("--initrd") + 1] == "/images/my-rootfs.cpio"


def test_limits_below_what_the_image_runs_in_are_refused(fake_hluk, tmp_path):
    # The guest kernel does not enforce a process's own limit, so such a limit would not hold
    for url in ("hyperlight://agent", "hyperlight://ghcr.io/hyperlight-dev/hyperlight-unikraft/agent:initrd-v0.19.0"):
        response, error = run_case(url, "pass", None, 5, 256)
        assert response is None and "use at least 720" in error
    rootfs = tmp_path / "big.cpio"
    with open(rootfs, "wb") as f:
        f.truncate(400 << 20)
    response, error = run_case(f"hyperlight://{rootfs}", "pass", None, 5, 256)
    assert response is None and "below what" in error
    assert fake_hluk() == []
    response, error = run_case("hyperlight://agent", "pass", None, 5, 720)
    assert error is None and response["status"] == "Success"


def test_registered_with_verl():
    from importlib.metadata import entry_points

    plugins = {e.name: e.value for e in entry_points(group="verl.plugins")}
    assert plugins.get("hyperlight") == "verl_hyperlight.plugin"


def test_a_reference_keeps_its_case(fake_hluk):
    run_case("hyperlight://ghcr.io/me/img:Release-A", "pass", None, 5, 256)
    args = fake_hluk()[0]["args"]
    assert args[args.index("--runtime") + 1] == "ghcr.io/me/img:Release-A"


@pytest.mark.parametrize(
    "url, message",
    [
        ("hyperlight://build/rootfs.cpio", "absolute path"),
        ("hyperlight://python:latest", "neither a published image's name nor a full reference"),
    ],
)
def test_guests_hluk_cannot_take_are_refused(fake_hluk, url, message):
    response, error = run_case(url, "pass", None, 5, 256)
    assert response is None and message in error
    assert fake_hluk() == []


def test_a_missing_rootfs_floor_is_not_kept(tmp_path):
    rootfs = tmp_path / "late.cpio"
    assert hyperlight._min_guest_memory_mb(str(rootfs)) == 0  # not there yet
    with open(rootfs, "wb") as f:
        f.truncate(400 << 20)
    assert hyperlight._min_guest_memory_mb(str(rootfs)) >= 1024


# A stand-in for verl's sandbox_fusion utils: call_sandbox_api answers for the HTTP service.
STUB_UTILS = """
def call_sandbox_api(sandbox_fusion_url, code, stdin, compile_timeout, run_timeout, memory_limit_mb, language="python",
                     request_timeout=None):
    return {"status": "Success", "from": "service", "request_timeout": request_timeout}, None
"""


def _with_stub_verl(tmp_path, utils_source, script):
    package = tmp_path / "verl" / "utils" / "reward_score" / "sandbox_fusion"
    package.mkdir(parents=True)
    for d in (package, *package.parents):
        if d == tmp_path:
            break
        (d / "__init__.py").touch()
    (package / "utils.py").write_text(utils_source)
    env = {**os.environ, "PYTHONPATH": str(tmp_path), "HLUK_PATH": str(tmp_path / "no-hluk")}
    out = subprocess.run([sys.executable, "-c", script], capture_output=True, text=True, env=env, timeout=60)
    assert out.returncode == 0, out.stderr
    return out.stdout.splitlines()


PLUGIN_SCRIPT = """
import verl_hyperlight.plugin
from verl.utils.reward_score.sandbox_fusion import utils
print(utils.call_sandbox_api("http://service/run_code", "pass", None, 5, 5, 256, request_timeout=9))
print(utils.call_sandbox_api("hyperlight://", "pass", None, 5, 5, 256))
"""


def test_plugin_routes_hyperlight_urls_of_a_verl_without_runners(tmp_path):
    service, hyperlight_ = _with_stub_verl(tmp_path, STUB_UTILS, PLUGIN_SCRIPT)
    assert "'from': 'service'" in service
    # A parameter a later verl adds reaches it: the arguments pass through as they come
    assert "'request_timeout': 9" in service
    # Here, not to the service: the stand-in has no hluk to run
    assert "Hyperlight backend: cannot run" in hyperlight_


def test_plugin_registers_with_a_verl_with_runners(tmp_path):
    # Such a verl dispatches by scheme itself: the plugin only registers, and leaves call_sandbox_api as it is
    utils_source = STUB_UTILS + (
        "RUNNERS = {}\ndef register_sandbox_runner(scheme, runner):\n    RUNNERS[scheme] = runner\n"
    )
    script = PLUGIN_SCRIPT + "print(sorted(utils.RUNNERS), utils.RUNNERS['hyperlight'].__name__)\n"
    _, hyperlight_, registered = _with_stub_verl(tmp_path, utils_source, script)
    assert "'from': 'service'" in hyperlight_
    assert registered == "['hyperlight'] run_code"


def test_hluk_missing(monkeypatch):
    monkeypatch.delenv("HLUK_PATH", raising=False)
    monkeypatch.setattr(hyperlight, "_hluk_paths", {})
    with patch("verl_hyperlight.runner.shutil.which", return_value=None):
        response, error = run_case("hyperlight://", "pass", None, 5, 256)
    assert response is None and "hluk not found" in error


def test_unsupported_language():
    response, error = hyperlight.run_code("hyperlight://", "int main(){}", None, 5, 5, 256, "cpp")
    assert response is None and "unsupported language" in error


# --- Integration tests: run real micro-VMs where hluk and a hypervisor are available ---

HLUK_AVAILABLE = hyperlight._hluk_path() is not None and os.access("/dev/kvm", os.R_OK | os.W_OK)
needs_hluk = pytest.mark.skipif(
    not (os.getenv("RUN_ALL") and HLUK_AVAILABLE), reason="requires RUN_ALL, hluk and /dev/kvm"
)
# The guest: the python image of the hluk on PATH, and in CI the repository's own build of it
URL = "hyperlight://" + (os.path.abspath(os.environ["HLUK_INITRD"]) if os.getenv("HLUK_INITRD") else "")


# The memory tests' run timeout: on the Linux arm64 runners (VMs on Apple silicon, so nested
# virtualization) the guest's first touch of each page is many times slower: filling a 405 MiB
# guest took over 10 s there, where x86 takes under 1 s, so these get several times that.
# 120 s still timed out on the 900 MiB case there (#242), so give them more again.
MEMORY_TEST_TIMEOUT = 360


def _why(response, error):
    """A real case's outcome, for an assertion message: its status and the ends of its output."""
    if not response or not response.get("run_result"):
        return response, error
    r = response["run_result"]
    return {**r, "stdout": r["stdout"][-300:], "stderr": r["stderr"][-300:]}


def _score(code, inputs, outputs):
    """The pass rate as verl scores Sandbox Fusion's replies: a case passes when the solution
    finished and its stdout, trailing newlines aside, is the expected output."""
    responses = [run_case(URL, code, stdin, 10, 256)[0] for stdin in inputs]
    passed = [
        r["status"] == "Success" and r["run_result"]["stdout"].rstrip("\n") == expected.rstrip("\n")
        for r, expected in zip(responses, outputs)
    ]
    return sum(passed) / len(passed), responses


@needs_hluk
def test_real_stdin_solution():
    code = "a, b = map(int, input().split())\nprint(a + b)"
    score, metadata = _score(code, ["1 2\n", "3 4\n"], ["3\n", "7\n"])
    assert score == 1.0, metadata


@needs_hluk
def test_real_unflushed_output_and_large_stdin():
    # A last answer without a newline, after reading a 1 MiB input.
    code = "import sys\nsys.stdout.write(str(len(sys.stdin.buffer.read())))"
    score, metadata = _score(code, ["x" * (1 << 20)], [str(1 << 20)])
    assert score == 1.0, metadata


@needs_hluk
def test_real_runs_like_a_script():
    # What `python solution.py` does at exit and on the way: a main() in a thread with a bigger
    # stack (a common competitive-programming idiom), atexit, __file__, and pickling a class
    # the solution defined.
    code = """
import atexit, pickle, sys, threading
class P:
    pass
pickle.dumps(P())
atexit.register(lambda: print("bye"))
def main():
    pickle.dumps(P())
    print(__file__.endswith("solution.py"), input())
threading.stack_size(1 << 26)
threading.Thread(target=main).start()
"""
    score, metadata = _score(code, ["7\n"], ["True 7\nbye\n"])
    assert score == 1.0, metadata


@needs_hluk
def test_real_random_and_path_per_case():
    # Each VM reseeds random from its own entropy, and the solution's directory is sys.path[0].
    code = "import random, sys\nprint(sys.path[0], random.random())"
    first, _ = run_case(URL, code, None, 10, 256)
    second, _ = run_case(URL, code, None, 10, 256)
    assert first["run_result"]["stdout"].startswith("/tmp/verl ")
    assert first["run_result"]["stdout"] != second["run_result"]["stdout"]


@needs_hluk
def test_real_stdin_is_not_translated_and_threads_of_threads_finish():
    # As under `python solution.py` on Linux: "\r" reaches the solution, and a thread started by
    # a thread is waited for too.
    code = """
import threading, time
def worker(s):
    time.sleep(0.2)
    print(len(s))
def main():
    threading.Thread(target=worker, args=(input(),)).start()
threading.Thread(target=main).start()
"""
    score, metadata = _score(code, ["abc\r\n"], ["4\n"])
    assert score == 1.0, metadata


@needs_hluk
def test_real_bulk_output_and_unclosed_executor():
    # 200k lines in well under the timeout (the guest console would line-buffer them), and an
    # executor never shut down does not keep the case open, as at a real interpreter exit.
    code = """
from concurrent.futures import ThreadPoolExecutor
ex = ThreadPoolExecutor(4)
total = sum(ex.map(lambda x: x * x, range(10)))
for i in range(200000):
    print(i)
print(total)
"""
    expected = "".join(f"{i}\n" for i in range(200000)) + "285\n"
    score, metadata = _score(code, [""], [expected])
    assert score == 1.0, metadata[0]["status"]
    assert metadata[0]["run_result"]["execution_time"] < 5


@needs_hluk
def test_real_stderr_heavy_solution():
    # Debug output on stderr is not line-buffered out of the VM one line at a time, and a
    # traceback after it still arrives whole.
    code = "import sys\nfor i in range(50000):\n    print(i, file=sys.stderr)\nprint('ok')\nraise ValueError('boom')"
    response, _ = run_case(URL, code, None, 10, 256)
    run = response["run_result"]
    assert run["stdout"] == "ok\n" and run["execution_time"] < 5
    assert "ValueError: boom" in run["stderr"] and "49999" in run["stderr"]


@needs_hluk
def test_real_line_by_line_io():
    # input() and print(flush=True) flush on every call; that must not cost a VM exit per line.
    code = "n = int(input())\nfor _ in range(n):\n    a, b = map(int, input().split())\n    print(a + b, flush=True)"
    stdin = "100000\n" + "".join(f"{i} {i}\n" for i in range(100000))
    response, _ = run_case(URL, code, stdin, 20, 256)
    run = response["run_result"]
    assert run["stdout"] == "".join(f"{2 * i}\n" for i in range(100000))
    assert run["execution_time"] < 5


# The fast-I/O template of PyRival, common in competitive-programming solutions.
FAST_IO = """
import os, sys
from io import BytesIO, IOBase
BUFSIZE = 8192
class FastIO(IOBase):
    newlines = 0
    def __init__(self, file):
        self._fd = file.fileno()
        self.buffer = BytesIO()
        self.writable = "x" in file.mode or "r" not in file.mode
        self.write = self.buffer.write if self.writable else None
    def read(self):
        while True:
            b = os.read(self._fd, max(os.fstat(self._fd).st_size, BUFSIZE))
            if not b:
                break
            ptr = self.buffer.tell()
            self.buffer.seek(0, 2), self.buffer.write(b), self.buffer.seek(ptr)
        self.newlines = 0
        return self.buffer.read()
    def readline(self):
        while self.newlines == 0:
            b = os.read(self._fd, max(os.fstat(self._fd).st_size, BUFSIZE))
            self.newlines = b.count(b"\\n") + (not b)
            ptr = self.buffer.tell()
            self.buffer.seek(0, 2), self.buffer.write(b), self.buffer.seek(ptr)
        self.newlines -= 1
        return self.buffer.readline()
    def flush(self):
        if self.writable:
            os.write(self._fd, self.buffer.getvalue())
            self.buffer.truncate(0), self.buffer.seek(0)
class IOWrapper(IOBase):
    def __init__(self, file):
        self.buffer = FastIO(file)
        self.flush = self.buffer.flush
        self.writable = self.buffer.writable
        self.write = lambda s: self.buffer.write(s.encode("ascii"))
        self.read = lambda: self.buffer.read().decode("ascii")
        self.readline = lambda: self.buffer.readline().decode("ascii")
sys.stdin, sys.stdout = IOWrapper(sys.stdin), IOWrapper(sys.stdout)
input = lambda: sys.stdin.readline().rstrip("\\r\\n")
"""


@needs_hluk
def test_real_fast_io_template_and_detached_stdout():
    code = FAST_IO + "n = int(input())\nprint(n * sum(map(int, input().split())))"
    score, metadata = _score(code, ["2\n1 2 3\n"], ["12\n"])
    assert score == 1.0, metadata
    # Detaching or closing sys.stdout leaves os.write to fd 1 working, as on the host.
    code = "import io, os, sys\nsys.stdout.close()\nos.write(1, b'b\\n')"
    response, _ = run_case(URL, code, None, 10, 256)
    assert response["run_result"]["stdout"] == "b\n", response["run_result"]["stderr"]


@needs_hluk
@pytest.mark.parametrize(
    "code",
    [
        "out = open(1, 'wb')\nprint('first')\nout.write(b'ans\\n')",
        "class D:\n    def __del__(self):\n        print('bye')\nd = D()\nprint('hi')",
        "import os\nout = os.fdopen(1, 'w', 1 << 20)\nout.write('ans\\n')",
        "import sys\nclass D:\n    def __del__(self):\n        sys.stdout.write('bye\\n')\nd = D()\nprint('hi')",
        "import sys\nclass D:\n    def __del__(self):\n        print('bye')\nd = D()\nprint('hi')\nsys.exit(0)",
        "import os, sys\nprint('a')\nos.write(sys.stdout.fileno(), b'b\\n')\nprint('c', flush=True)\nos._exit(0)",
        "import os\nprint('a', flush=True)\nos.write(1, b'b\\n')\nprint('c')",
        "import os\nout = open(os.dup(1), 'w')\nout.write('x\\n')\nprint('y')",
        "import subprocess, sys\nprint('a', flush=True)\nsubprocess.run([sys.executable, '-c', 'print(1)'])\n"
        "print('c')",
        "out = open(1, 'w')\nfor i in range(200000):\n    out.write(f'{i}\\n')",
        "import sys\nprint('ans')\nsys.stdout.close()",
        "import os\nprint('a', flush=True)\nos._exit(status=0)",
        "import os\nos.write(1, b'x\\n')\nopen('/dev/stdout', 'wb').write(b'y\\n')\nprint('z')",
        "import os\nprint('a', flush=True)\nfd = os.open('/dev/stdout', os.O_WRONLY)\nos.write(fd, b'b\\n')",
        "import sys\nprint(open('/dev/stdin').read() == '', file=open('/dev/stderr', 'w'))\nprint('ok')",
        "from pathlib import Path\nprint(repr(Path('/dev/stdin').read_text()))",
        "f = open('/dev/stdout', 'a')\nf.write('hello\\n')",
        "import sys\nclass W:\n    def __init__(s):\n        s.o = sys.stdout\n"
        "    def write(s, x):\n        s.o.write(x)\nsys.stdout = W()\nprint(42)",
        "import sys\nsys.stderr.detach()\nprint(42)",
        "import sys\nsys.stdin = sys.stdin.buffer\nprint(repr(sys.stdin.read()), repr(input()) if False else '')",
        "import io, sys\nsys.stdin = io.TextIOWrapper(sys.stdin.buffer, encoding='utf-8')\ntry:\n    print(input())\n"
        "except EOFError:\n    print('eof')",
        "import os, sys\nout = os.dup(1)\nsys.stdout.close()\ntry:\n    input()\n"
        "except EOFError:\n    os.write(out, b'eof\\n')",
        "import os, sys\nhere = os.path.dirname(os.path.abspath(__file__))\n"
        "open(os.path.join(here, 'a.txt'), 'w').write('y')\n"
        "print(open(os.path.join(here, 'a.txt')).read(), os.getcwd() == here == sys.path[0])",
        "import os, tempfile\nprint(42)\nd = tempfile.gettempdir()\n"
        "for n in os.listdir(d):\n    p = os.path.join(d, n)\n    if os.path.isfile(p):\n        os.remove(p)",
        "x = input('q? ')\nprint(x)",
        "try:\n    input()\nexcept EOFError as e:\n    print('eof', e)",
        "import sys\nprint('ans')\nsys.stdout.flush()\nsys.stdout = None",
        "import io, sys\nprint('first')\nsys.stdout = io.StringIO()\nprint('hello')\n"
        "sys.__stdout__.write(sys.stdout.getvalue())",
    ],
)
def test_real_output_matches_host_python(code, tmp_path):
    # Output order across sys.stdout, the fd and other file objects on it, os._exit, and what is
    # written while the interpreter exits (module-level file buffers, __del__), as under
    # `python solution.py` on the host with stdout a pipe; and none of it costs a VM exit a line.
    run_dir, temp_dir = tmp_path / "run", tmp_path / "tmp"
    run_dir.mkdir()
    temp_dir.mkdir()
    script = run_dir / "solution.py"
    script.write_text(code)
    stdin = "hi\n" if "input('q? ')" in code else ""
    # As Sandbox Fusion runs it, and so that what the code does to its temp dir stays in tmp_path.
    host = subprocess.run(
        [sys.executable, str(script)],
        input=stdin,
        capture_output=True,
        text=True,
        timeout=30,
        cwd=run_dir,
        env={**os.environ, "TMPDIR": str(temp_dir)},
    )
    response, _ = run_case(URL, code, stdin, 10, 256)
    run = response["run_result"]
    assert (run["stdout"], run["return_code"]) == (host.stdout, host.returncode)
    assert run["execution_time"] < 5


@needs_hluk
def test_real_heavy_output():
    # Megabytes on stdout and stderr: appending to a growing file in the guest gets slower as it
    # grows, which the runner's large writes keep from adding up.
    code = "import sys\nfor i in range(10**6):\n    print(i)\n    print(i, file=sys.stderr)"
    response, _ = run_case(URL, code, None, 30, 256)
    run = response["run_result"]
    assert run["stdout"] == "".join(f"{i}\n" for i in range(10**6))
    assert run["execution_time"] < 8


@needs_hluk
def test_real_daemon_threads_stop_at_exit():
    # Python stops daemon threads at exit; what one would print afterwards is not output.
    code = (
        "import threading, time\ndone = False\ndef d():\n    while not done:\n        time.sleep(0.001)\n"
        "    while True:\n        print('late', flush=True)\n        time.sleep(0.001)\n"
        "threading.Thread(target=d, daemon=True).start()\nprint('end')\ndone = True"
    )
    for _ in range(3):
        response, _ = run_case(URL, code, None, 10, 256)
        assert response["run_result"]["stdout"] == "end\n"


def test_the_guest_memory_holds_a_limit_and_what_the_image_needs():
    for runtime, limit in (("agent", 720), ("python", 256)):
        args = hyperlight._guest_args(runtime, limit)
        guest = int(args[args.index("--scratch-mb") + 1])
        assert guest == hyperlight._guest_memory_mb(limit) >= hyperlight._min_guest_memory_mb(runtime)


@pytest.mark.skipif(not os.environ.get("VERL_HYPERLIGHT_TEST_AGENT"), reason="pulls the 400 MB agent image")
@needs_hluk
def test_real_numpy_reseeded_per_case():
    code = "import numpy as np\nprint(np.random.randint(1 << 30))"
    first, _ = run_case("hyperlight://agent", code, None, 30, 1024)
    second, _ = run_case("hyperlight://agent", code, None, 30, 1024)
    assert first["run_result"]["stdout"] != second["run_result"]["stdout"]


@needs_hluk
def test_real_timeout():
    response, error = run_case(URL, "while True: pass", None, 2, 256)
    assert error is None
    assert response["run_result"]["status"] == "TimeLimitExceeded"


@needs_hluk
def test_real_isolation(tmp_path):
    # The host's files, environment and network are out of reach, and each test case gets a fresh
    # guest: what one leaves behind is gone for the next.
    host_file = tmp_path / "host-file"
    host_file.write_text("host data")
    code = f"""
import os, socket
print(os.path.exists({str(host_file)!r}))
print(os.environ.get("VERL_HYPERLIGHT_TEST_HOST_VAR"))
print(os.path.exists("/tmp/left_behind"))
open("/tmp/left_behind", "w").close()
try:
    socket.create_connection(("1.1.1.1", 53), timeout=2)
    print("connected")
except OSError:
    print("no network")
"""
    expected = "False\nNone\nFalse\nno network\n"
    with patch.dict(os.environ, {"VERL_HYPERLIGHT_TEST_HOST_VAR": "host value"}):
        score, metadata = _score(code, ["", ""], [expected, expected])
    assert score == 1.0, metadata


def _host_memory_available_mb():
    try:
        with open("/proc/meminfo") as f:
            return next(int(line.split()[1]) >> 10 for line in f if line.startswith("MemAvailable:"))
    except (OSError, StopIteration, ValueError):
        return 0


@needs_hluk
@pytest.mark.parametrize(
    "solution_mb, memory_limit_mb",
    [
        (340, 384),
        # Sandbox Fusion's usual limit, in a 1.4 GB guest: only where the host can back it beside
        # the test run, which the Linux arm64 runners (3 GB VMs) cannot
        pytest.param(
            900,
            1024,
            marks=pytest.mark.skipif(_host_memory_available_mb() < 3072, reason="needs 3 GiB of free host memory"),
        ),
    ],
)
def test_real_memory_limit_is_the_solutions(solution_mb, memory_limit_mb):
    # memory_limit_mb is what the solution may allocate, as under Sandbox Fusion, not the VM's
    # whole memory, which also holds the guest's kernel and interpreter: in a guest only the size
    # of the limit, the solution would run out of memory (even 300 MiB does in a 405 MiB guest)
    code = f"x = bytearray({solution_mb} << 20)\nx[::4096] = b'\\x01' * len(x[::4096])\nprint('ok')"
    response, error = run_case(URL, code, None, MEMORY_TEST_TIMEOUT, memory_limit_mb)
    why = _why(response, error)
    assert response and response["run_result"], why
    assert response["status"] == "Success" and response["run_result"]["stdout"] == "ok\n", why


@needs_hluk
def test_real_memory_limit():
    response, error = run_case(URL, "x = bytearray(512 * 1024 * 1024)", None, MEMORY_TEST_TIMEOUT, 256)
    why = _why(response, error)
    assert response and response["run_result"], why
    r = response["run_result"]
    # Out of memory, not out of time or some other crash: Python's MemoryError, or the guest
    # kernel killing the process when it touches a page there is no memory for, which hluk
    # reports as the guest exiting with 137 (the kernel's own message goes to its log; this
    # solution does not exit with 137 itself)
    assert r["status"] == "Finished", why
    assert (r["return_code"] == 1 and "MemoryError" in r["stderr"]) or r["return_code"] == 137, why


def test_url_detection():
    assert hyperlight.is_hyperlight_url("hyperlight://")
    assert hyperlight.is_hyperlight_url("hyperlight://agent")
    assert not hyperlight.is_hyperlight_url("http://localhost:8080/run_code")
    assert not hyperlight.is_hyperlight_url(None)
