"""Host-mount workload — writes and reads back a file on a mounted directory.

Run with `--mount DIR:/mnt/bench`.  The bench snapshot is saved without
mounts, so a snapshot mode also measures the mount the kernel builds on
resume; each file is named at random so parallel guests do not collide.
"""
import os

path = f"/mnt/bench/{os.urandom(4).hex()}.txt"
with open(path, "w") as f:
    f.write("ok\n")
with open(path) as f:
    assert f.read() == "ok\n"
os.remove(path)
print("ok")
