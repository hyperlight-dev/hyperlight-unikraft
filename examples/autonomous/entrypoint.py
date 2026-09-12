#!/usr/bin/env python3
"""Autonomous appliance — the entire workload lives in the initrd.

hluk boots this guest and the host sends only a fixed, app-agnostic launcher
that runs /entrypoint.py; nothing app-specific comes from the host. This is the
urunc deployment model: the image carries the kernel + rootfs, and the guest
runs its baked-in entrypoint with no host input, no mounts, no network.
"""
import platform
import time


def primes_below(n):
    sieve = bytearray([1]) * n
    sieve[0:2] = b"\x00\x00"
    for i in range(2, int(n**0.5) + 1):
        if sieve[i]:
            sieve[i * i : n : i] = bytearray(len(range(i * i, n, i)))
    return [i for i in range(n) if sieve[i]]


def main():
    print("== autonomous appliance ==")
    print(f"python {platform.python_version()} on {platform.system()}")
    t0 = time.perf_counter()
    p = primes_below(100_000)
    dt = (time.perf_counter() - t0) * 1000
    print(f"primes below 100000: {len(p)} (largest {p[-1]}) in {dt:.1f} ms")
    print("done")


if __name__ == "__main__":
    main()
