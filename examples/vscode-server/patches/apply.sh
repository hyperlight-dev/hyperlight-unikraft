#!/bin/bash
# Apply Unikraft source patches required for VS Code Server.
#
# These patches fix two issues in the plat-hyperlight branch:
#
# 1. epoll.c — MULTIPROCESS epoll busy-loop starves cooperative scheduler
#    The MULTIPROCESS-specific epoll_pwait2 path calls time_block_until()
#    (which blocks the CPU via a host hcall) without ever yielding to the
#    cooperative scheduler. Worker threads created via clone(CLONE_THREAD)
#    never run. Fix: add uk_sched_yield() before the loop and inside it.
#
# 2. exit.c — _exit() from non-last thread halts the entire VM
#    The Hyperlight-specific code calls uk_pm_shutdown() whenever ANY
#    thread calls _exit() in pid <= 2, even if other threads are still
#    running. This kills the VS Code Server when a worker thread exits.
#    Fix: only halt when the calling thread is the last one in the process.
#
# 3. time.c — monotonic clock overflows after ~7 seconds
#    ukplat_monotonic_clock() computes (tsc_delta * 10^9) / tsc_freq,
#    but tsc_delta * 10^9 overflows uint64 after ~7.4s at 2.5 GHz.
#    This causes the clock to wrap, breaking Node.js timer assertions.
#    Fix: split into secs + remainder to keep intermediates in range.
#
# 4. hostsock.c — writev drops all but the first iovec; stack overflow
#    hostsock_write() only sends iov[0], silently dropping the rest.
#    Node.js uses writev for chunked HTTP responses, so static assets
#    (CSS, JS) arrive as 0-byte bodies.  Also, hostsock_sendmsg() and
#    hostsock_recvmsg() allocate 32 KB on the stack for multi-iovec
#    flattening, which overflows the 64 KB thread stacks used by the
#    cooperative scheduler.
#    Fix: iterate iovecs in write(); use a static buffer in sendmsg/
#    recvmsg instead of stack allocation.
#
# Usage: Run from examples/vscode-server/ after kraft fetches the source.
#   bash patches/apply.sh

set -euo pipefail

UNIKRAFT=".unikraft/unikraft"

if [ ! -d "$UNIKRAFT" ]; then
    echo "Error: $UNIKRAFT not found. Run 'kraft build' first." >&2
    exit 1
fi

cp patches/epoll.c.patched    "$UNIKRAFT/lib/posix-poll/epoll.c"
cp patches/exit.c.patched     "$UNIKRAFT/lib/posix-process/exit.c"
cp patches/time.c.patched     "$UNIKRAFT/plat/hyperlight/x86/time.c"
cp patches/hostsock.c.patched "$UNIKRAFT/lib/hostsock/hostsock.c"
cp patches/clone.c.patched    "$UNIKRAFT/lib/posix-process/clone.c"
cp patches/clone_arch.c.patched "$UNIKRAFT/lib/posix-process/arch/x86_64/clone.c"
cp patches/execve.c.patched   "$UNIKRAFT/lib/posix-process/execve.c"
cp patches/cow.c.patched      "$UNIKRAFT/plat/hyperlight/cow.c"
cp patches/syscall.S.patched  "$UNIKRAFT/plat/common/x86/syscall.S"
cp patches/mmap.c.patched     "$UNIKRAFT/lib/ukmmap/mmap.c"
cp patches/system_error.c.patched "$UNIKRAFT/lib/posix-process/signal/system_error.c"
cp patches/deliver.c.patched     "$UNIKRAFT/lib/posix-process/signal/deliver.c"
cp patches/uk_syscall_binary.c.patched "$UNIKRAFT/lib/syscall_shim/uk_syscall_binary.c"

echo "Patches applied. Rebuild with: kraft-hyperlight --no-prompt build --plat hyperlight --arch x86_64"
