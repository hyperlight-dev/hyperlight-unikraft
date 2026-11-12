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

Dropping a polled, unfinished `step_async` or a higher-level driving future
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
unchanged waits. No per-socket wait map, network completion variant or pruning
path remains in the pending-operation engine. The future does not retain socket
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
when `run_async` returns. Snapshots can capture it without another `step_async`.
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

During driving, fresh block-list
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
and synchronous migration adapters use that same engine.

| Platform | Validation available for this change |
| --- | --- |
| Linux x86_64 / KVM | Unit, guest, CLI/project and wire-parser execution |
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

## Migrating callers

The asynchronous entry points are `run_async`, `call_async`, `join_async` and
`step_async`. Existing `run`, `call`, `join` and bounded `step(timeout)` callers
remain supported during migration by a lazily created, sandbox-owned Tokio
runtime. These adapters must not be called from within a Tokio runtime.
Both paths drive the same pending-operation engine; a bounded synchronous step
retains pending work when its timeout expires. Cancellation of an asynchronous
drive still poisons the sandbox. Boot, submission, capture and restore stay
synchronous.
