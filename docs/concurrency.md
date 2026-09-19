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

## Cooperative step model

Between host entries the guest is never *running*: when every guest thread is blocked the kernel hands the vCPU back to the host with the time until its next timer, and the host parks (VM halted, in `poll(2)` on the guest's host sockets) until the guest could make progress. An idle guest costs no CPU; a blocking `accept()` or `sleep()` in the guest is served by the host waking it. Named calls (`Exec`) run on the runtime driver's own thread, and a guest whose entry point is a plain program is driven with [`AppSandbox::join`] instead. How the host drives a guest, the kinds of guest, `run`/`submit`/`step`/`join`, snapshots and restore are in [execution.md](execution.md); what a restore does to the clocks and the random source is in [clock.md](clock.md) and [random.md](random.md).

## Threads

Threading works — create, join, locks, events, and thread pools all function correctly.  A call returns when its main code returns; a thread it started, daemon or not, is left behind and keeps making progress on the steps that follow. `asyncio` works too: its event loop's self-pipe is an `AF_UNIX` `socketpair()`, served by the kernel's `posix-unixsocket` (see `examples/python/asyncio_demo.py`).

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

## Examples

- `examples/python/threading_demo.py` — threads with locks and shared state
- `examples/python/threaded_select.py` — threaded server with `select()`
- `examples/python/subprocess_demo.py` — spawning child processes via `subprocess.run()`
- `examples/python/tcp_echo.py` — TCP echo server/client
