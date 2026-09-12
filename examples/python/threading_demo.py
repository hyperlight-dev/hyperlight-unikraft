"""Threading demo — exercises guest-internal concurrency.

Spawns worker threads that do CPU-bound work (summing numbers).
Each thread computes a partial sum; the main thread joins them
and verifies the result.

This exercises Unikraft's cooperative scheduler: threads yield at
Python's eval-loop check interval (sys.getswitchinterval).
"""

import threading

N = 100_000
NUM_THREADS = 4
CHUNK = N // NUM_THREADS

results = [0] * NUM_THREADS


def partial_sum(tid, start, end):
    total = 0
    for i in range(start, end):
        total += i
    results[tid] = total


threads = []
for t in range(NUM_THREADS):
    start = t * CHUNK
    end = start + CHUNK
    th = threading.Thread(target=partial_sum, args=(t, start, end))
    threads.append(th)
    th.start()

for th in threads:
    th.join()

total = sum(results)
expected = sum(range(N))
assert total == expected, f"sum mismatch: got {total}, expected {expected}"
print(f"Sum of 0..{N-1} = {total} (using {NUM_THREADS} threads)")
print("Threading demo passed.")
