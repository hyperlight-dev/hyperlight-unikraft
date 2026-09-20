# Clock

The guest has a monotonic clock, a wall clock and sleeps, all built on the CPU's timestamp counter (TSC).  There is no timer interrupt in the VM.

## Sources

At boot the kernel asks the host two things through host functions: `GetTscHz`, the TSC frequency, which the host measures once against its own clock (the guest and the host run on the same unscaled counter), and `GetWallClockNs`, the host's current Unix time.

- **Monotonic** (`CLOCK_MONOTONIC`, Python `time.monotonic()` / `time.perf_counter()`): TSC ticks since boot divided by the frequency.
- **Wall clock** (`CLOCK_REALTIME`, `time.time()`, `datetime.now()`): the time fetched at boot plus the monotonic clock.  The TSC keeps counting while the VM is halted between steps or inside a host call, so the wall clock stays in step with the host's without further queries.
- **Sleeps** (`nanosleep`, `clock_nanosleep` on either clock, relative or absolute; Python `time.sleep`, `asyncio.sleep`, socket timeouts): the thread blocks with a deadline.  When every thread is blocked the kernel yields with the time to the earliest deadline and the host parks until then; see [execution.md](execution.md).

The images ship no zoneinfo, so local time is UTC unless the image adds tzdata and sets `TZ`.

## Snapshots

A snapshot captures the TSC along with memory, so a restored guest's monotonic clock continues from the moment of the snapshot: a `sleep(10)` snapshotted with 4 s left has 4 s left.  The wall clock would report that moment too, so on the `resume` entry the kernel calls `GetWallClockNs` again and re-anchors it on the host's time, accounting for the time the image spent on disk and for a different host's clock.
