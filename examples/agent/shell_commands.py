"""Shell commands via subprocess — BusyBox hush is in the agent rootfs."""
import subprocess

# Run shell commands via BusyBox
commands = [
    "echo 'Hello from hush shell!'",
    "uname -a",
    "ls /usr/local/bin/",
    "cat /etc/passwd",
]

for cmd in commands:
    result = subprocess.run(
        ["/bin/sh", "-c", cmd],
        capture_output=True,
        text=True,
    )
    print(f"$ {cmd}")
    if result.stdout:
        print(result.stdout, end="")
    if result.returncode != 0:
        print(f"  (exit {result.returncode})")
    print()
