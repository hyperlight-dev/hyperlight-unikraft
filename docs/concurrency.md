# Concurrency

Unikraft guests on Hyperlight run on a **single vCPU** with a **cooperative scheduler**.  This page covers what that means for threads and multiprocess.

## Execution model

```
┌─────────────────────────────────┐
│  Single vCPU                    │
│                                 │
│  Thread A ──yield──→ Thread B   │
│  Thread B ──yield──→ Thread A   │
│                                 │
│  (no preemption, one core)      │
└─────────────────────────────────┘
```

- **One vCPU.**  The Hyperlight platform boots a single logical CPU. There is no multi-core execution.
- **Cooperative scheduling.**  The kernel uses Unikraft's `ukschedcoop` — a non-preemptive round-robin scheduler.  A thread runs until it explicitly yields, blocks on I/O, or exits.  It is never interrupted by a timer tick.  Hyperlight does support [hardware interrupt injection][hw-int] (a paravirtualised timer that delivers vector 0x20 at a configurable rate), which could enable preemptive scheduling in the future.
- **Single address space.**  All threads share one address space. There is no process isolation in the traditional sense.

For a full description of Unikraft's process model and the syscalls available at each feature level, see the upstream [posix-process README][pp-readme].

[hw-int]: https://github.com/hyperlight-dev/hyperlight/blob/main/src/hyperlight_host/src/hypervisor/virtual_machine/x86_64/hw_interrupts.rs
[pp-readme]: https://github.com/unikraft/unikraft/blob/staging/lib/posix-process/README.md

## Threads

Threading works — create, join, locks, events, and thread pools all function correctly.  `asyncio` does not work (needs `AF_UNIX` / `socketpair()`, which is not supported).

A CPU-bound thread that never yields will starve all other threads because there is no preemption.

## Multiprocess

Unikraft supports `vfork()+execve()`, which runtimes use to spawn child processes.  Python 3.12's `subprocess.run()` uses this path internally via `posix_spawn()`.

1. **`vfork()`** — creates a child sharing the parent's address space. The parent suspends until the child calls `execve()` or `_exit()`.
2. **`execve()`** — loads a new ELF binary (must be PIE) into the address space via the elfloader's binfmt handler.  The child gets a new stack and begins executing the new program.
3. The parent resumes and can read the child's stdout/stderr.

This is evidenced by the `examples/python/subprocess_demo.py` example, which spawns Python child processes for computation, stdin/stdout piping, module execution, and error handling.

### What does NOT work

- **`os.fork()`** — returns `ENOTSUP`.  `clone()` without `CLONE_VM` is rejected.  This means:
  - `multiprocessing.Process` (uses `fork` start method on Linux)
  - `os.popen()`
  - Any library that calls `fork()` directly
- **Parallel children** — `vfork()` suspends the parent, so children run sequentially.
- **Environment variable propagation** — env vars set in the parent may not be visible in the child after `execve()`.

## Examples

- `examples/python/threading_demo.py` — threads with locks and shared state
- `examples/python/threaded_select.py` — threaded server with `select()`
- `examples/python/subprocess_demo.py` — spawning child processes via `subprocess.run()`
- `examples/python/tcp_echo.py` — TCP echo server/client
