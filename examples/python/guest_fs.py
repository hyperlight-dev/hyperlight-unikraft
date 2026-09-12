"""Guest filesystem operations — exercises the in-memory ramfs.

The guest root is a RAM filesystem extracted from the initrd CPIO.
Writes go to RAM (no host I/O) and are ephemeral — lost on VM
shutdown or snapshot restore.
"""

import os
import stat

# ── Write and read back a file in /tmp ────────────────────────────

os.makedirs("/tmp/hluk-test", exist_ok=True)

path = "/tmp/hluk-test/hello.txt"
with open(path, "w") as f:
    f.write("Hello from the guest ramfs!\n")

with open(path) as f:
    content = f.read()
    assert content == "Hello from the guest ramfs!\n", f"unexpected: {content!r}"
    print(f"Read back: {content.strip()}")

# ── stat ──────────────────────────────────────────────────────────

info = os.stat(path)
print(f"Size: {info.st_size} bytes")
assert info.st_size == len("Hello from the guest ramfs!\n")
assert stat.S_ISREG(info.st_mode)

# ── mkdir and nested writes ───────────────────────────────────────

os.makedirs("/tmp/hluk-test/a/b/c", exist_ok=True)
with open("/tmp/hluk-test/a/b/c/deep.txt", "w") as f:
    f.write("nested\n")

with open("/tmp/hluk-test/a/b/c/deep.txt") as f:
    assert f.read() == "nested\n"

# ── listdir ───────────────────────────────────────────────────────

entries = sorted(os.listdir("/tmp/hluk-test"))
print(f"Entries: {entries}")
assert "hello.txt" in entries
assert "a" in entries

# ── rename ────────────────────────────────────────────────────────

os.rename(path, "/tmp/hluk-test/renamed.txt")
assert os.path.exists("/tmp/hluk-test/renamed.txt")
assert not os.path.exists(path)

# ── read initrd files (proves CPIO extraction works) ──────────────

# Python stdlib should be available from the initrd.
import sys
print(f"Python {sys.version_info.major}.{sys.version_info.minor}")
assert os.path.isdir("/usr/local/lib") or os.path.isdir("/usr/lib")

# ── clean up ──────────────────────────────────────────────────────

import shutil
shutil.rmtree("/tmp/hluk-test")
assert not os.path.exists("/tmp/hluk-test")

print("Guest filesystem tests passed.")
