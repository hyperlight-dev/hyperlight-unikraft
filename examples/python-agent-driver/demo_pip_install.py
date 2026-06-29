import subprocess, sys

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
