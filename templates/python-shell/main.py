import subprocess
import sys

print(f"Hello from {{name}}!")
print(f"Python {sys.version_info.major}.{sys.version_info.minor}, inside a Hyperlight micro-VM")

# /bin/sh is BusyBox's hush. It runs in NOMMU mode here (vfork + execve,
# not fork), and pipes and $(...) work.
for cmd in ["uname -srm", "which python3 sh", "echo pipes work | tr a-z A-Z"]:
    result = subprocess.run(["/bin/sh", "-c", cmd], capture_output=True, text=True)
    print(f"$ {cmd}")
    print(result.stdout, end="")
