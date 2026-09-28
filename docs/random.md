# Random numbers

The guest has a cryptographically secure random source with the usual POSIX interfaces, seeded from the CPU (x86_64) or the host (arm64).

## Source

At boot the kernel keys a ChaCha20 generator (Unikraft's `ukrandom`) with 256 bits from the CPU's hardware random-number instructions (RDSEED/RDRAND).  On arm64 those instructions (RNDR) are optional and Apple's cores lack them, so the kernel asks the host instead: `GetRandomBytes`, from the host OS's generator.  `getrandom(2)`, `/dev/urandom` and `/dev/random` read from it and never block, and runtimes use them as on Linux: Python's `os.urandom`, `secrets` and `random` (which seeds itself from `os.urandom`), OpenSSL for TLS.

There is no periodic reseeding thread, and no way to seed the generator deterministically; reproducible randomness is an application concern (`random.Random(n)`).

## Snapshots

A snapshot captures the generator's state along with memory, so two guests restored from the same image would draw the same bytes.  On the `resume` entry the kernel reseeds the generator from the same source before any guest code runs, so everything that reads it (`os.urandom`, `secrets`, `getrandom`, TLS) is fresh in every clone.

> Note: a `random.Random` object (the `random` module's own included) that existed when the snapshot was taken is not reseeded; it continues its sequence in every clone.  Objects created after the restore, and `random.seed()`, draw a fresh seed.
