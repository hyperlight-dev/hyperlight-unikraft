"""Stdlib-heavy workload — json, re, collections, hashlib."""
import json
import re
import collections
import hashlib

# JSON round-trip
data = {f"key_{i}": list(range(50)) for i in range(100)}
blob = json.dumps(data)
json.loads(blob)

# Regex compilation + matching
pattern = re.compile(r'(\w+)@(\w+)\.(\w+)')
for i in range(1000):
    pattern.findall(f"user{i}@example{i}.com is valid")

# Collections
c = collections.Counter(blob)
d = collections.OrderedDict(sorted(c.items()))

# Hashing
for i in range(500):
    hashlib.sha256(blob.encode()).hexdigest()

print("ok")
