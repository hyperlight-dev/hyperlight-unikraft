import os

DATA = "/mnt/data"

total = 0
for name in sorted(os.listdir(DATA)):
    with open(os.path.join(DATA, name)) as f:
        words = len(f.read().split())
    total += words
    print(f"{words:6}  {name}")
print(f"{total:6}  total")
