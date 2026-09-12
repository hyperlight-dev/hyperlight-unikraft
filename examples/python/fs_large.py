"""Large file I/O — exercises chunked hostfs reads and writes.

Writes and reads back data larger than a single transfer chunk (32 KB)
to verify that multi-chunk I/O works end-to-end.  The host's PEB I/O
stacks must be sized to hold at least one chunk plus FlatBuffer
overhead; this test catches silent failures from undersized stacks.
"""

import os

MOUNT = "/mnt/host"
PATH = os.path.join(MOUNT, "large.bin")

# 96 KB — three full 32 KB chunks.
SIZE = 96 * 1024
PATTERN = b"ABCDEFGHIJKLMNOP"  # 16 bytes, repeats evenly into SIZE
data = PATTERN * (SIZE // len(PATTERN))
assert len(data) == SIZE

# Write
with open(PATH, "wb") as f:
    written = f.write(data)
print(f"Wrote {written} bytes")
assert written == SIZE, f"short write: {written} != {SIZE}"

# Stat — verify size on disk
info = os.stat(PATH)
print(f"Stat size: {info.st_size} bytes")
assert info.st_size == SIZE, f"stat size mismatch: {info.st_size} != {SIZE}"

# Read back and verify
with open(PATH, "rb") as f:
    got = f.read()
print(f"Read {len(got)} bytes")
assert len(got) == SIZE, f"short read: {len(got)} != {SIZE}"
assert got == data, "data mismatch after round-trip"

# Clean up
os.unlink(PATH)
print("large file round-trip OK")
