# Async host calls, version 1

This extends `HostCall` without changing synchronous calls or the driver ioctl.
The host owns the futures; only guest threads and request IDs live in the
waiters, not copies of request arguments or Rust future state.
The public async driving APIs require Tokio time and I/O drivers (`enable_all()`
on a manually built runtime). VM entries are still
synchronous: an executor timeout cannot interrupt a CPU-bound guest entry.

## Negotiation and submission

At boot and on resume the kernel calls `AsyncHostCallVersion() -> u64`.
Version 1 hosts record the handshake and answer 1. A missing function means
no async support. A builder registering async functions rejects a kernel that
does not negotiate version 1 and the interruption capability described below, before
handing the sandbox to its caller.

The kernel also calls `AsyncHostCallInterruptVersion() -> u64` at boot and resume.
This lets the host record that the kernel supports interrupting pending calls on
restore; the host answers 1. Interruption happens entirely in the guest, without
a restore-dispatch host call. The base protocol and completion batch version
remain 1.

`HostCall(name, args)` retains its existing reply:

| Tag | Payload | Meaning |
| --- | --- | --- |
| 0 | UTF-8 bytes | Immediate success |
| 1 | UTF-8 bytes | Immediate error |
| 2 | nonzero little-endian u64 | Pending request ID (exactly 9 bytes total) |

Synchronous handlers use exactly the existing path. Async handlers enqueue an
owned, lazy future and return a pending ID; the factory is invoked only when
the executor polls it, outside the VM entry and any registration lock.
IDs increase monotonically within a sandbox, including across restore; overflow
and more than 1024 outstanding operations fail explicitly before invoking the
handler. The argument and result limits are unchanged.

The kernel releases all transport buffers/locks before parking the calling
thread. Each waiter owns its output destination and length; shared host-call
buffers must not be retained across a scheduler switch. The hlcall device opts
out of the vnode ioctl lock, since holding it across the wait would serialize
unrelated guest threads. Other devices keep their existing locking.

## Command process trees

The runnable [sandbox commands example](../examples/sandbox_commands.rs) starts
two commands in one sandbox and concurrently awaits their output and exit
statuses. It then demonstrates root exit automatically terminating a sleeping
descendant, with output drained concurrently until EOF.

From the repository root on Linux, with Docker, just, GCC, Rust, and KVM:

```sh
just build-rootfs c
mkdir -p build-elfloader/bins/c
gcc -O2 -Wall -static-pie -fPIE -o build-elfloader/bins/c/command examples/c/command.c
cargo run --example sandbox_commands
```

The guest executable is the existing C command fixture; `just build-test-bins`
also builds it. No Rust, Go, or .NET guest toolchains are needed for the commands
above. Stdout and stderr are drained concurrently with the process waits;
each line is reported as it arrives, prefixed with the command name. The exit
summary follows both streams reaching EOF. ANSI escapes make name prefixes bold
and exit summaries bold green for success or bold red for failure.
Only the current line is collected,
rather than the entire output (a very long line can still use unbounded memory).

`AppSandbox::into_commands()` consumes an idle compiled-runtime sandbox and
returns a `command::CommandSandbox`. This opt-in API runs executable paths,
not interpreter snippets:

```rust,no_run
use hyperlight_unikraft::{SandboxBuilder, command::Stdio};

# async fn example() -> hyperlight_unikraft::Result<()> {
let sandbox = SandboxBuilder::from_initrd("build-elfloader/c-rootfs.cpio")
    .boot()?
    .into_commands()?;
let first = sandbox.command("/usr/local/bin/first")
    .arg("one argument with spaces")
    .stdout(Stdio::piped())
    .spawn()?;
let second = sandbox.command("/usr/local/bin/second").spawn()?;
let (first, second) = futures::try_join!(
    first.wait_with_output(),
    second.wait_with_output(),
)?;
assert!(first.status.success() && second.status.success());
# Ok(())
# }
```

Each host-launched root and all its descendants share one execution identity.
It survives guest `exec` and reparenting. That execution owns its future set,
socket table, socket-readiness wait, and scheduler deadlines. IDs and limits
remain sandbox-wide: 1024 outstanding host operations and 1024 host sockets.
The host routes at most 1024 active execution contexts at once; completed
contexts are removed before another spawn. Their handles retain status and
buffered output.
Execution contexts separate progress and resource ownership, not guest security
boundaries.

Polling a child's wait, stdin, stdout, or stderr drives that tree's host
operations. There is no background task or sandbox-wide async runner. VM entry
is serialized, and embedder futures are polled outside the VM lock. Entering
for one tree can still schedule runnable guest threads from another, or
observe another tree's elapsed guest timers. It does **not** poll that other
tree's queued embedder or network futures. Guest IPC between independent
trees may require the host to drive both.

The root defines the command's lifetime. When it exits, the guest automatically
sends SIGKILL to all remaining descendants, including reparented descendants
and descendants in other sessions. Other command trees are unaffected.
`wait()` waits for the root and command cleanup, returning the root's status
without aggregating descendant exit codes. `try_wait()` reports the same
completion condition without driving progress. Output already written is
forwarded before cleanup finishes, so piped output still needs concurrent draining.
`wait_with_output()` and `Command::output()` handle both drains and cleanup.
`Command::status()` waits for command cleanup without draining piped output.
Cancellation of any wait preserves the execution and prepared
completion batches. Like a process wait, waiting closes stdin still held in
the child's `stdin` field; take the handle first to manage its lifetime yourself.
Dropping a child parks its execution without entering the VM. Pending host
futures and host timer registrations are dropped, and host sockets are closed.
Guest threads cannot run until the execution is reattached and driven again.
If another task is inside a pipe poll, teardown finishes when that poll returns.
Taken stream handles become invalid (`BrokenPipe`), including after reattachment.
Untaken streams and buffered bytes are preserved, not closed.

Save `child.execution_id()` before dropping it, then use
`sandbox.restore_child(id)` to acquire a new exclusive handle. The first drive
reconstructs resources and interrupts unfinished calls with `EINTR`; the guest
chooses whether to retry. `sandbox.children()` lists all retained commands in
execution-ID order, including attached and completed commands. Each `ChildInfo`
reports `execution_id`, the guest `pid` if known, `detached`, and `completed`
(root, descendant and worker cleanup finished). Call `info.restore()` to
reacquire that command from its originating sandbox. Metadata is an observation,
not a live status view: restoration rechecks the current attachment state.
Listing neither drives commands nor keeps the sandbox alive; entries remain
usable across detach but fail after forgetting the command or dropping its sandbox.
An attached child is not necessarily being polled. `parked_children()` remains
a convenience for listing only detached execution IDs.
Completed executions and their output remain available until `forget_child(id)`;
that method requires a detached, completed command. To discard an unfinished
command, restore it, kill it, drop it, then forget it.

Listeners and unconnected sockets are recreated under the existing network
policy; established connections become dead (EOF on read, EPIPE on write),
just as on snapshot restore. Rebinding may fail and is reported by the socket
restore path. Timer registrations are rebuilt from guest deadlines, not restarted
for their original duration. Time outside the VM continues to advance.
Completed data replies are retained, but socket-dependent replies are interrupted
because their host resources no longer exist. Cancellation cannot undo side
effects or make arbitrary retries safe.
`kill()` sends SIGKILL to every guest process with that execution identity,
including descendants reparented to guest init or moved into a new session.
It is synchronous: one VM entry stops the tree and its workers, cancelling
pending host calls without polling them. Stdin is closed, including a taken
handle. Unforwarded output is discarded; bytes already in the host buffers
remain readable, followed by EOF. No output draining or Tokio runtime is needed
to call `kill()`. After success, `wait()` is immediately ready.
Killing an already completed tree needs no VM entry, and other command trees
are not signalled. As with other VM entries, CPU-bound guest code requires the
existing interrupt-handle mechanism to preempt.
Keep the child alive while driving taken stream handles, even after `wait()`
returns; dropping the child invalidates those handles. Dropping every sandbox, child, and
stream handle releases the VM and any unfinished work.

Streams implement `futures::io::AsyncRead` / `AsyncWrite`. Each host pipe holds
at most 64 KiB, with 4 KiB transport chunks and ordinary backpressure. Drain
piped output concurrently with `wait()` to avoid a full-pipe deadlock.
`output()` and `wait_with_output()` collect into unbounded host vectors, like
the corresponding standard-library APIs. Separate stream and wait handles
can be polled on separate tasks without replacing each other's wakeups.

The initial API accepts UTF-8 arguments, per-command environment overrides,
and a guest working directory. Executables require absolute guest paths:
there is no shell parsing or PATH search. Stdio defaults to `/dev/null`;
`Stdio::piped()` opts into a host pipe. Independent roots do not inherit each
other's command descriptors. Cross-tree socket sharing is not supported.

Command protocol version 5 requires rebuilt C, Rust, Go, or .NET AOT exec
driver images. Other runtimes and snapshots restored without the command
handshake are rejected explicitly by `into_commands()`. Command sandboxes
do not expose the legacy interpreter API. Existing `AppSandbox` driving and
snapshot behavior remain unchanged.

### Parked command snapshots

Drop every child before calling `CommandSandbox::snapshot()`. This captures
the whole VM and the host-side command registry: execution IDs, root statuses,
stream states and buffered bytes, and retained completion records. It contains
no futures or live host I/O resources. The original commands remain parked and
can be reacquired independently of the snapshot.

```rust,ignore
let id = child.execution_id();
drop(child);
let snapshot = sandbox.snapshot()?;

let child = sandbox.restore_child(id)?;
let output = child.wait_with_output().await?;

// All children must be dropped before replacing the current guest state.
sandbox.restore(&snapshot)?;
let child = sandbox.restore_child(id)?;
```

`SandboxBuilder::from_command_snapshot(&snapshot)` boots an independent clone.
Supply host functions, mounts, network policy and listening ports on that builder,
then call `boot()?.into_commands()?`. Boot and restore do not run command threads
or invoke their async handlers. Use `parked_children()` and `restore_child(id)`
to drive them. In-place restore preserves execution-ID allocation high-water
marks, so IDs from discarded branches are not reused for new commands.

The `CommandSnapshot` object is an in-memory snapshot; it is not the raw
`Snapshot` accepted by the legacy disk/OCI helpers. Its command metadata must not
be omitted. Guest timer deadlines follow the existing snapshot clock policy;
live detach/reattach does not restart their duration. Connections are not cloned,
and pending resource-dependent operations are interrupted on first drive.

Snapshots reject attached children. A concurrent lifecycle transition or an
in-flight poll still finishing detachment returns `Error::Wait` with
`WouldBlock`; retry after that operation finishes. Taken handles from before a
detach or restore remain invalid and cannot consume restored output.

### Attribution and control wire format

The exec driver negotiates version 5 with `HLCALL_IOC_COMMAND_DRIVER` (ioctl 6, `u64`).
Workers attach with `HLCALL_IOC_EXECUTION` (ioctl 5, `u64`). The kernel inherits
this identity through thread creation. Before a host call whose owner changed,
the transport sends `ExecutionContext(u64) -> i32` under the same transport
lock as the following call. Socket rescans and resource destruction use the
resource owner's identity, never the currently polled host handle.

Before scheduling on each entry, `ExecutionActive() -> u64` selects the command
being driven. Only that execution's threads and execution-zero infrastructure
are eligible to run. Other trees remain queued even when their timers expire;
their deadlines and socket readiness are not serviced by this entry. Runnable
foreign threads do not prevent returning to the host. This gate is independent
of ordinary blocking and wakeups. Spawn and kill select their target too.
Shared supervisor and guest-init bookkeeping can run, but not another command's
application threads. This is scheduling isolation, not a PID/filesystem namespace:
applications deliberately synchronizing across trees still require both trees
to be driven. CPU-bound guest code still requires the interrupt handle to preempt.

`ExecutionRestore() -> i32` marks the first entry after resource teardown.
After delivering retained completion records, the kernel reconstructs only
the selected execution's sockets, retires its old request IDs, and finishes
remaining async waits with `EINTR` before scheduling. An empty version-1
completion batch is valid for this entry. Pipe workers retry interrupted
forwarding without closing the channel; host output chunks commit atomically
so interrupted backpressured writes cannot duplicate a partially forwarded chunk.

`CommandSpawn` carries one NUL-delimited string: version, execution ID,
argument count (excluding argv[0]), environment count, piped-stream bitmask
(stdin/stdout/stderr), cwd, executable, arguments, and `KEY=VALUE` entries.
The numeric fields are decimal. A final NUL is required. `CommandKill`
carries the execution ID as a decimal string. It uses
`HLCALL_IOC_EXECUTION_KILL` (ioctl 9, `u64`) to signal all matching processes,
excluding guest init and the calling driver supervisor. It uses inherited
execution identity, not current parent PIDs or process groups.
During whole-tree termination, the kernel cancels execution-owned async waits
and refuses new forwarding operations so cleanup does not need another host
completion batch. Socket readiness workers stop waiting on the host while
process teardown closes their sockets.
The normal named-call completion
acknowledges control delivery, **not** process exit.

`HLCALL_IOC_COMMAND_EVENT` (ioctl 7, two `i32`s) forwards `CommandEvent(kind,
value)`: kind 0 is a positive root PID or negative spawn errno, kind 1 a raw
Linux wait status, kind 2 a positive stream error errno, and kind 3/value 0
closes host stdin when its guest reader is gone. `ExecutionDone()` marks the
end of all context-owned guest threads, including readiness and pipe workers.
Before forwarding kind 1, the kernel signals all remaining application
processes in that execution. Normal root exit leaves output forwarding workers
alive to drain data already written; the host closes stdin, including taken
handles, so input forwarding cannot keep the completed command alive.

`HLCALL_IOC_COMMAND_IO` (ioctl 8) contains `u64 stream`, a buffer pointer,
`u64 len`, and `u64 done`. Streams 1/2 write stdout/stderr; zero length reports
EOF. Stream 0 reads stdin in chunks of up to 4096 bytes. `CommandIo` replies
use tag 0 plus read bytes (empty for writes), tag 1 plus a little-endian Linux
errno, or the standard tag 2 pending ID. These futures belong to the same
execution as the requesting worker.

Each boundary reports `ExecutionTimers(Vec<u8>)`: pairs of little-endian
`u64 execution_id, u64 nanoseconds_until_wakeup`. The host takes the earliest
deadline for the selected execution, replacing its previous boundary's set; absent timers
are removed. Legacy execution zero retains the original `Yield` protocol.
The control device releases its read vnode lock while the supervisor waits,
so control ioctls and independent worker opens cannot deadlock behind it.

## Completion

`step_async(Vec<u8>)` runs the normal step pump after delivering a batch:

```
u32 version = 1
u32 count (1..64)
repeat count times:
    u64 request_id
    u32 reply_length
    u8 reply[reply_length]  // existing 0/1-tagged HostCall result
```

All integers are little-endian; no padding. The batch fits the host-to-guest
transport after reserving one receive buffer for control traffic and 4 KiB
for framing. With the current geometry, up to 14 maximum-size replies or 64
small replies fit in an entry. Before polling another completion, the host
reserves room for a maximum-size reply. This leaves unconsumed work in
`FuturesUnordered`, with no separate overflow slot, allocation or queue.
Results are batched without waiting for other operations to finish.
The kernel validates the entire batch before waking any
waiter: lengths, tags, version, duplicate IDs and IDs without a waiter are errors.
An output buffer too small receives ENOBUFS and the required length, as before.

Test vector: request 1 returning JSON `7`:

```
01 00 00 00 01 00 00 00
01 00 00 00 00 00 00 00 02 00 00 00 00 37
```

The host retains a prepared batch until the entry accepts it. Failed async
steps poison the sandbox rather than silently losing completions.
The inbox stages abortable futures directly; only the heterogeneous future
itself is boxed, not another async wrapper around cancellation.

## Lifetime and cancellation

For the legacy `AppSandbox` APIs, dropping a polled, unfinished `step` or a higher-level driving future
poisons the sandbox and drops its pending host futures and results. Execution
and snapshots then fail with `PoisonedSandbox`; an explicit successful restore
is the recovery path. Dropping an unpolled future does nothing. Cancellation
is cooperative between VM entries. No automatic replay is permitted: user
operations may have external side effects already.

Internal scheduler timers and socket registrations are rebound when driving
moves to a different Tokio runtime, even if the scheduler deadline is unchanged.
This does not migrate runtime-bound resources captured by user futures or
reconstruct pending DNS lookups. Complete pending host calls, or discard them
through cancellation/restore, before destroying their runtime.

Snapshots permit outstanding host calls, including queued or undelivered
results. Restore interrupts them as described below, discarding pending host
work in the current timeline. The kernel calls `CancelAsyncHostCall(id)` if it cannot allocate a waiter or
the waiting thread terminates. Cancellation is idempotent and drops the future
when the host next drives the engine; a result no longer awaited is discarded.
There is no public per-call cancellation API.

## Restoring pending host calls

`snapshot()`, `snapshot_to(dir)`, and `snapshot_with_scratch_mb(mb)` allow
pending requests at a completed step boundary without a policy argument.
Capturing does not poll, cancel or otherwise alter the original timeline's work.

On restore, old host futures and undelivered results are discarded. The kernel
keeps already-delivered guest results and completes every unfinished host-call
waiter with `EINTR`, after resource restoration and before running guest threads.
Old request IDs are discarded; no calls are requeued, and no completion batches
or async runtime are needed to deliver interruption.

Existing closure registration stays unchanged. There are no restore hooks or
saved request descriptors. Restoring in another process, without a handler, or
with a different handler has the same interruption behavior; the host never
invokes the original function automatically. Neither Rust futures nor captured
host resources are serialized.

Interruption becomes ioctl `EINTR`, not a JSON error or a completion tag.
Driver wrappers surface a transport exception (Python's helper raises
`RuntimeError`); they do not retry the host-call ioctl. Ordinary closure errors
retain the guest host-error path.

The guest/application decides whether to retry. External effects are **not
rolled back**, and interruption does not imply retrying is safe. Application
libraries may independently retry interrupted operations. The kernel's readiness
waiter retries its idempotent wait after interruption, using the restored sockets;
it never replays a send or connect. Timers and resource restoration remain separate.

Updated kernels and snapshot contract 10 require fresh snapshots. Async handler
registration rejects kernels without interruption-capability negotiation. This
change needs no additional rootfs rebuild beyond the existing Python GIL fix.

## Migration

Driving is async-only, including when all registered handlers are
synchronous:

| Removed API | Async API |
| --- | --- |
| `run(exec)` / `run_async(exec).await` | `run(exec).await` |
| `call(name, input)` / `call_async(name, input).await` | `call(name, input).await` |
| `join()` / `join_async().await` | `join().await` |
| `step(timeout)` / `step_async().await` | `step().await` |

The `_async` public names have been removed. Boot, `submit`, snapshots and restore
remain synchronous. Use Tokio with time and I/O enabled; the CLI now provides
that runtime itself. Synchronous applications can use `runtime.block_on(...)`
at their outer boundary, not inside a running async executor. The library has
no blocking driver or internally created runtime. Boot, submit and resume can
enqueue lazy host calls without an executor; subsequent async driving polls them.

**A timeout is not a probe.** There is no bounded synchronous step or zero-timeout
probe. `step` has no timeout argument: dropping a polled unfinished future, including through
`tokio::time::timeout`, poisons the sandbox. To do other work while waiting,
keep the driving future alive and poll it again rather than cancelling it.
Completed steps are safe boundaries for accessing the sandbox.

Only Rust API names change in this migration; the guest entry named
`step_async` and snapshot compatibility are unchanged by the rename.

## Shared pending-operation engine

The scheduler's earliest deadline, reported by the existing `Yield` host call,
is driven as a timer in the same `FuturesUnordered` as host calls. An unchanged
deadline retains its timer; a changed or removed deadline replaces or cancels
it when async driving next waits. Deadlines are absolute, so time spent outside
the driver counts toward expiry. Timer expiry is an internal wake, with no
request ID or completion record sent to the guest. If host-call results are
ready too, their completion entry also services the timer: no extra VM entry
or timer-registration host call is needed.

Scheduler timers do not prevent snapshots. Restore discards the old host timer;
the resumed guest reports its next deadline as before. Poison-on-drop and
teardown clear timers together with pending calls. Socket readiness now uses
Tokio TCP/UDP registrations, created lazily and cached until close, restore or
socket replacement. A listener wait
retains at most one accepted connection (or accept error) for the guest's next
`accept`; both the guest's immediate poll and the async wait see it. Dropping a readiness
wait does not lose that connection. Policy checks, nonblocking socket operations
and Linux errno translation remain in the existing host networking layer.

Network readiness uses an ordinary typed `net_wait_async()` host call. One
kernel waiter per sandbox submits it and parks through the same guest waiter
mechanism as other async calls. The kernel-only waiter has no Unikraft TLS, so it
does not inherit a subprocess's descriptor table. Its host future polls the current socket
interests; interest changes wake it, and cached Tokio registrations survive
unchanged waits. The pending-operation engine needs no per-socket wait map, network completion
variant or pruning path. The future does not retain socket
handles between polls, so close/rebind still works within one VM entry.

The reply is a normal tag-0 `i32`: 1 means recheck readiness and 0 means no interests.
A pending response
has the usual request ID, counts against the shared outstanding-call limit, and
is delivered in normal completion batches alongside embedder and policy replies.
Only timers remain internal wakes. Infrastructure I/O errors use the generic
fallible-completion path, return `Error::Wait` and poison the sandbox.

**Readiness waits follow ordinary pending-call rules.** Snapshots
capture them without cancelling the original work; on
restore the kernel waiter receives `EINTR` and rechecks the recreated sockets.
No restore hook or serialized request metadata is needed. Application socket
operations and poll/select remain nonblocking in the driver; their POSIX layer
still decides which guest threads park.

After closing sockets, a final readiness completion may still be outstanding
when `run` returns. Snapshots can capture it without another `step`.
The CLI uses the same interruption behavior for
warm-cache and manual snapshots, preserving network-enabled warm-up.

## Typed networking calls

The kernel negotiates async support before starting the workload, including
driverless entry binaries that can wait on networking before the first idle.
It negotiates again with the restoring host on resume.

The kernel uses `net_connect_async`, `net_send_async`, and `net_sendto_async` for operations that
may need a fresh block-list DNS check. These carry the same arguments as their
integer-returning predecessors, but return a byte-vector envelope:

- tag 0 followed by a little-endian `i32` result (including negative Linux errno);
- tag 2 followed by the request ID, exactly as for a pending `HostCall`.

The existing completion batches deliver these typed results to the same guest
waiter mechanism. Unlike an embedder `HostCall` result, the four bytes after tag
0 are an integer, not UTF-8. Static policy decisions retain a single host call
with an immediate reply, without allocating a future or parking a guest thread.

During `run`, `call`, `join` and `step`, fresh block-list
lookups are driven outside VM entries in the shared futures set. Resolver
failure or the 250 ms deadline denies access with `EACCES`; no TCP connect or
datagram transmission is attempted before authorization. UDP connect sends
nothing and is allowed without a policy check; each subsequent send checks its
destination. A stream sendto uses its connected peer, not the supplied address.
The existing process-wide limit of 32
resolver threads remains in force. System resolver calls cannot be interrupted:
cancellation discards their answers, and their slots remain occupied until the
workers finish. Snapshot restore interruption and poison-on-drop apply to these
pending typed calls just as to embedder calls. Restored typed calls fail with
`EINTR`, just like unfinished embedder calls.
Socket libraries, including Python's blocking socket operations, may retry
`EINTR` themselves; that is distinct from the host engine replaying a future.

Fresh DNS checks always use lazy async calls, including during boot, submit and
resume. Older kernels using the legacy integer-returning network calls receive
`EOPNOTSUPP` if authorization would
require a live DNS lookup; they must be rebuilt to use the deferred path.
Kernel binaries and the snapshot contract have changed, so existing snapshots
must be recreated.

## Validation

Build the kernels and Python runtime with `just build-kernel` and
`just build-rootfs python`. Run
`cargo test --test async_calls --test restore_host_calls` for the end-to-end
tests, including fresh-process pending-call interruption, and `cargo test --lib --
--ignored` for guest parser and restore tests (which need a hypervisor and rootfs, unlike
the ordinary library unit tests).

### Driving performance baseline

Run `cargo test --release --test driving_bench -- --ignored --nocapture` on a
host with a hypervisor and the local Python rootfs. The ignored benchmark uses
the existing test runner, with no extra benchmark dependencies. It measures
the async driver twice to expose ordering and host-load effects. Boot and five
warm-up calls are excluded; profiling is enabled throughout. A single
current-thread Tokio runtime is reused.

Each `BENCH drive` row reports median and p95 call latency, throughput, VM entries
per call and total driving-thread CPU time for the measured samples. CPU time
is available only on Linux, in 10 ms procfs ticks; `0.0` is below that resolution,
not proof of zero work. Peer-thread CPU is not included.

Workloads cover immediate guest calls, synchronous host functions, 5 ms guest
timers, sixteen concurrent 5 ms host calls, a 500 ms idle wait and persistent
TCP/UDP echo connections. The `tcp-wait` and `udp-wait` cases delay each peer reply
by 2 ms to exercise readiness rather than only the already-ready fast path.
Async host handlers overlap during fan-out.

The benchmark asserts one VM entry for immediate/synchronous-host calls, two
for timer/idle calls, and less than 100 ms of driving-thread CPU for the measured
500 ms idle wait. Delayed-peer cases must include readiness wakes, not just
single-entry calls with already-ready replies. Latencies are reported,
not asserted: shared-host load and Tokio's millisecond timer granularity make
universal timing thresholds flaky.
Use repeated runs on the same host to assess regressions.

#### Historical blocking/async comparison (2026-11-06)

Linux x86_64/KVM, Intel Core i9-13900, release build, local Python rootfs.
Recorded before removal of the blocking API. These are ranges of the two medians
per mode in one ABBA run after lifecycle
hardening, not confidence intervals or cross-machine performance guarantees:

| Workload | Blocking median | Async median | VM entries/call, blocking / async |
| --- | --- | --- | --- |
| Immediate | 18.5-18.6 us | 18.5-19.5 us | 1 / 1 |
| Synchronous host function | 31.6-31.9 us | 31.8-32.0 us | 1 / 1 |
| 5 ms guest timer | 5.18-5.50 ms | 6.40-6.54 ms | 2 / 2 |
| Sixteen 5 ms host calls | 89.36-89.49 ms | 11.18-11.21 ms | 1 / 2.00-2.05 |
| TCP echo | 62.3-84.6 us | 59.4-62.8 us | 1.01-1.03 / 1.00-1.03 |
| TCP echo, 2 ms peer delay | 2.77-2.94 ms | 2.42-2.79 ms | 2 / 2 |
| UDP echo | 63.4-68.8 us | 63.8-65.9 us | 1.01 / 1.01-1.05 |
| UDP echo, 2 ms peer delay | 2.75-2.76 ms | 2.44-2.82 ms | 2 / 2 |
| 500 ms idle wait | 500.38-500.55 ms | 501.13-502.12 ms | 2 / 2 |

Both idle paths measured below one 10 ms CPU tick. Immediate calls retained
their single-entry fast path. Async fan-out was about eight times faster than
serialized blocking handlers. Tokio's timer granularity adds roughly 1 ms to
short sleeps here; it does not cause extra VM entries. Network latency tails
varied between repetitions, so these samples do not establish a universal
network speedup. The pre-hardening run had the same entry counts and comparable
fan-out medians (89.50-89.56 ms blocking, 11.17-11.19 ms async).

### Lifecycle and portability coverage

The ordinary unit tests now repeatedly fill all 1024 host-call slots, cancel and
refill a quarter, and verify exact 64-result batches without duplicate or
cancelled IDs. Separate clear tests cover queued, running and prepared-batch
phases and verify that all receiver futures are dropped. Socket tests churn 32
registrations across eight runtime lifetimes, closing/rebinding half each time.
A retained-timer regression reproduces the panic from polling a timer after its
original runtime shuts down and verifies rebinding on the next drive.

Guest tests complete 130 concurrent waiters in exactly three completion entries
over three rounds, and cancel/restore sixteen pending calls over eight rounds.
Existing tests retain coverage for backpressure, unread-data spinning, idle CPU,
out-of-order replies, close/restore and batched timer/network wakes.

There is one production `FuturesUnordered` driving the sandbox. Readiness tests
use the shared engine. A DNS future's local answer/deadline `select!` is part of
that one operation, not a second sandbox driver. There is no separate blocking
socket waiter or step loop.

| Platform | Validation available for this change |
| --- | --- |
| Linux x86_64 / KVM | Unit, guest, CLI/project, wire-parser and release benchmark execution |
| Linux x86_64 / MSHV | Feature-enabled library compiles; MSHV execution not performed |
| Windows x86_64 / WHP | All test targets cross-compile; no Windows execution host available |
| macOS arm64 / HVF; Linux arm64 | Native execution and cross-compilation not performed here |

Run `cargo test --lib`, `cargo test --test async_calls`, `cargo test --test step`,
`cargo test --doc` and `cargo clippy --all-targets -- -D warnings`.
The step suite's Go checkpoint test additionally requires its compiled fixture.
Windows compile coverage uses
`cargo check --tests --target x86_64-pc-windows-gnu`; this is not a replacement
for running the tests under WHP. MSHV compile coverage uses
`cargo check --features mshv --lib`. Native macOS/arm64 validation remains a release
gate on hosts with those hypervisors and matching guest fixtures.
