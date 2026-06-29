import subprocess, sys, importlib, os, signal

proc = subprocess.Popen(
    [sys.executable, "-m", "pip", "install", "six"],
)

for _ in range(50_000_000):
    try:
        importlib.invalidate_caches()
        import six
        print(f"Installed and imported six {six.__version__}")
        try:
            os.kill(proc.pid, signal.SIGKILL)
        except OSError:
            pass
        break
    except ImportError:
        pass
