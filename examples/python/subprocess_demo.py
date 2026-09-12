"""Subprocess demo — Python spawning Python via vfork+execve.

Demonstrates that subprocess.run() works in the Unikraft guest.
Python 3.12 uses posix_spawn/vfork internally for subprocess,
so this works without os.fork() support.

sys.executable is set by sitecustomize.py in the rootfs.
"""
import subprocess
import sys

assert sys.executable, "sys.executable is empty — sitecustomize.py missing from rootfs?"
PY = sys.executable

# 1. Child computes fibonacci(10), parent reads the result
r = subprocess.run(
    [PY, "-c", "def fib(n):\n a,b=0,1\n for _ in range(n): a,b=b,a+b\n return a\nprint(fib(10))"],
    capture_output=True, text=True, timeout=5,
)
assert r.returncode == 0 and r.stdout.strip() == "55", f"fib(10) failed: {r}"
print(f"1. child computed fib(10) = {r.stdout.strip()}")

# 2. Parent sends data via stdin, child transforms and returns it
r = subprocess.run(
    [PY, "-c", "import sys, json; data=json.load(sys.stdin); print(json.dumps({k:v*2 for k,v in data.items()}))"],
    input='{"a": 1, "b": 2, "c": 3}',
    capture_output=True, text=True, timeout=5,
)
assert r.returncode == 0
import json
result = json.loads(r.stdout)
assert result == {"a": 2, "b": 4, "c": 6}, f"json transform failed: {result}"
print(f"2. child doubled values: {result}")

# 3. Child uses -m to run a stdlib module (base64 encode)
r = subprocess.run(
    [PY, "-m", "base64", "-e"],
    input="hello world", capture_output=True, text=True, timeout=5,
)
assert r.returncode == 0 and r.stdout.strip() == "aGVsbG8gd29ybGQ=", f"-m base64 failed: {r}"
print(f"3. child base64-encoded: {r.stdout.strip()}")

# 4. Child exits with non-zero, parent captures stderr
r = subprocess.run(
    [PY, "-c", "import sys; print('something went wrong', file=sys.stderr); sys.exit(1)"],
    capture_output=True, text=True, timeout=5,
)
assert r.returncode == 1 and "something went wrong" in r.stderr, f"exit+stderr failed: {r}"
print(f"4. child exited with rc={r.returncode}, stderr={r.stderr.strip()!r}")

# 5. Three children run sequentially, results collected
squares = []
for i in range(5):
    r = subprocess.run([PY, "-c", f"print({i}**2)"], capture_output=True, text=True, timeout=5)
    assert r.returncode == 0
    squares.append(int(r.stdout.strip()))
assert squares == [0, 1, 4, 9, 16], f"sequential failed: {squares}"
print(f"5. five children computed squares: {squares}")

print("Subprocess demo passed")
