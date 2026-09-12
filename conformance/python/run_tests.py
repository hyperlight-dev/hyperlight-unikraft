"""
Single CPython test module runner for Hyperlight-Unikraft.

Runs ONE test module via unittest and prints a structured RESULT line.
The host-side conformance driver invokes this once per module via
snapshot restore, so each module gets a fresh guest.

Usage (inline via --exec):
  hluk snapshot exec <dir> --exec "MODULE='test_abc'; exec(open('/path/to/run_tests.py').read())"

Or more practically, the host passes the module name embedded in the
inline code string, and this script is included in the rootfs.
"""
import sys
import time
import unittest

def run_module(mod_name):
    """Import and run a single test module, print structured result."""
    t0 = time.monotonic()

    try:
        mod = __import__(f"test.{mod_name}", fromlist=[mod_name])
    except ImportError:
        elapsed = (time.monotonic() - t0) * 1000
        print(f"RESULT {mod_name} status=SKIP tests=0 fail=0 error=0 skip=0 time={elapsed:.1f}")
        return
    except Exception:
        elapsed = (time.monotonic() - t0) * 1000
        print(f"RESULT {mod_name} status=ERROR tests=0 fail=0 error=1 skip=0 time={elapsed:.1f}")
        return

    try:
        loader = unittest.TestLoader()
        suite = loader.loadTestsFromModule(mod)
        if suite.countTestCases() == 0:
            elapsed = (time.monotonic() - t0) * 1000
            print(f"RESULT {mod_name} status=SKIP tests=0 fail=0 error=0 skip=0 time={elapsed:.1f}")
            return

        # Suppress test output — we parse the structured RESULT line
        import io
        runner = unittest.TextTestRunner(stream=io.StringIO(), verbosity=0)
        result = runner.run(suite)
        elapsed = (time.monotonic() - t0) * 1000

        tests = result.testsRun
        fail = len(result.failures)
        error = len(result.errors)
        skip = len(result.skipped)

        status = "FAIL" if (fail > 0 or error > 0) else "PASS"
        print(f"RESULT {mod_name} status={status} tests={tests} fail={fail} error={error} skip={skip} time={elapsed:.1f}")

    except Exception as e:
        elapsed = (time.monotonic() - t0) * 1000
        print(f"RESULT {mod_name} status=ERROR tests=0 fail=0 error=1 skip=0 time={elapsed:.1f}")

# Module name is injected by the host via: MODULE='test_abc'\n<this script>
if 'MODULE' in dir() or 'MODULE' in globals():
    run_module(MODULE)
else:
    print("ERROR: MODULE variable not set")
    sys.exit(1)
