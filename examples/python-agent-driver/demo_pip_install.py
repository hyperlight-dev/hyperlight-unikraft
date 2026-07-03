import subprocess, sys, os

target = "/tmp/pypackages"
os.makedirs(target, exist_ok=True)

result = subprocess.run(
    [sys.executable, "-m", "pip", "install", "--target", target, "six"],
    capture_output=True, text=True,
)
print(result.stdout)
if result.returncode != 0:
    print(result.stderr)
    sys.exit(result.returncode)

sys.path.insert(0, target)
import six
print(f"Installed and imported six {six.__version__}")
