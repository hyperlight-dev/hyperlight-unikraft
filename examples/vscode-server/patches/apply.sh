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
# Usage: Run from examples/vscode-server/ after kraft fetches the source.
#   bash patches/apply.sh

set -euo pipefail

UNIKRAFT=".unikraft/unikraft"

if [ ! -d "$UNIKRAFT" ]; then
    echo "Error: $UNIKRAFT not found. Run 'kraft build' first." >&2
    exit 1
fi

cp patches/epoll.c.patched "$UNIKRAFT/lib/posix-poll/epoll.c"
cp patches/exit.c.patched  "$UNIKRAFT/lib/posix-process/exit.c"

echo "Patches applied. Rebuild with: kraft-hyperlight --no-prompt build --plat hyperlight --arch x86_64"
