# Async host calls, version 1

This extends `HostCall` without changing synchronous calls or the driver ioctl.
The host owns the futures; only guest threads and request IDs live in the VM.
The public async driving APIs require a Tokio time driver. VM entries are still
synchronous: an executor timeout cannot interrupt a CPU-bound guest entry.

## Negotiation and submission

At boot and on resume the kernel calls `AsyncHostCallVersion() -> u64`.
Version 1 hosts record the handshake and answer 1. A missing function means
no async support. A builder registering async functions rejects a kernel that
does not negotiate version 1, before handing the sandbox to its caller.

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

All integers are little-endian; no padding. The batch occupies at most
`HOST_CALL_MAX + 20` bytes, enough for one maximum-size result. The host-to-guest
pool reserves an extra page for byte-parameter framing. Results are batched
without waiting for other operations to finish; excess completions stay on the
host for another entry. The kernel validates the entire batch before waking any
waiter: lengths, tags, version, duplicate IDs and IDs without a waiter are errors.
An output buffer too small receives ENOBUFS and the required length, as before.

Test vector: request 1 returning JSON `7`:

```
01 00 00 00 01 00 00 00
01 00 00 00 00 00 00 00 02 00 00 00 00 37
```

The host retains a prepared batch until the entry accepts it. Failed async
steps poison the sandbox rather than silently losing completions.

## Lifetime and cancellation

Dropping a polled, unfinished `step_async` or a higher-level driving future
poisons the sandbox and drops its pending host futures and results. Execution
and snapshots then fail with `PoisonedSandbox`; an explicit successful restore
is the recovery path. Dropping an unpolled future does nothing. Cancellation
is cooperative between VM entries. No automatic replay is permitted: user
operations may have external side effects already.

Snapshots are rejected while any operation is outstanding, including a queued
or completed-but-undelivered one. Thus a supported snapshot contains no pending
IDs to reconstruct. Restoring such a snapshot may discard pending work in the
current timeline. The kernel calls `CancelAsyncHostCall(id)` if it cannot allocate a waiter or
the waiting thread terminates. Cancellation is idempotent and drops the future
when the host next drives the engine; a result no longer awaited is discarded.
There is no public per-call cancellation API or reconstruction of snapshot waits
in version 1.

## Migration

`run`, `call`, `join` and `step` retain synchronous behavior for synchronous
handlers. With async registrations, use `run_async`, `call_async`, `join_async`
or `submit` followed by `step_async`. Sync driving refuses pending async work.
Timers and existing socket readiness are raced with host-call completions;
their migration into the same pending-operation protocol is a later change.

## Validation

Build the kernels and Python runtime with `just build-kernel` and
`just build-rootfs python`. Run `cargo test --test async_calls` for the end-to-end
tests, and `cargo test --lib malformed_batches_do_not_partially_wake_waiters --
--ignored` for the guest parser test (which needs a hypervisor and rootfs, unlike
the ordinary library unit tests).
