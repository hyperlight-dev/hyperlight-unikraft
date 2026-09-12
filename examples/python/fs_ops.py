"""Filesystem operations — demonstrates hostfs mount."""

import os
import tempfile

# Write a file, read it back, stat it.
path = "/mnt/host/hello.txt"
with open(path, "w") as f:
    f.write("Hello from the guest!\n")

with open(path) as f:
    content = f.read()
    print(f"Read back: {content.strip()}")

info = os.stat(path)
print(f"Size: {info.st_size} bytes")

# List the mount directory.
entries = os.listdir("/mnt/host")
print(f"Files in /mnt/host: {entries}")

# Create a subdirectory and a file inside it.
os.makedirs("/mnt/host/subdir", exist_ok=True)
with open("/mnt/host/subdir/nested.txt", "w") as f:
    f.write("nested content\n")

for root, dirs, files in os.walk("/mnt/host"):
    for name in files:
        full = os.path.join(root, name)
        print(f"  {full} ({os.path.getsize(full)} bytes)")

# Write a sentinel that the host can verify.
with open("/mnt/host/sentinel.txt", "w") as f:
    f.write("guest-was-here\n")

# Clean up everything except the sentinel.
os.unlink("/mnt/host/subdir/nested.txt")
os.rmdir("/mnt/host/subdir")
os.unlink(path)
print("Cleanup done.")
