import subprocess, sys

# vfork pipe init: unikraft needs a warmup subprocess + stdout write
# before long-running children with capture_output work reliably
subprocess.run(["/bin/true"], capture_output=True)
sys.stdout.write("\n")
sys.stdout.flush()

result = subprocess.run(
    [sys.executable, "-m", "pip", "install", "six"],
    capture_output=True, text=True,
)
print(result.stdout)
if result.returncode != 0:
    print(result.stderr)
    sys.exit(result.returncode)

import six
print(f"Installed and imported six {six.__version__}")
