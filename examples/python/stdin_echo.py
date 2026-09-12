"""Read from stdin and echo each line."""
import sys

lines = []
for line in sys.stdin:
    lines.append(line.rstrip("\n"))

print(f"lines={len(lines)}")
for line in lines:
    print(f"echo: {line}")
print("stdin-done")
