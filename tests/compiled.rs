// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Tests for compiled-language runtimes: C, C++, Rust, Go, .NET AOT.
//!
//! The guest binaries are Linux ELF executables built from `examples/`
//! by `just build-test-bins` into `build-elfloader/bins/<runtime>/` —
//! on Linux, the same way rootfs images are.  Each test mounts that
//! directory into the guest via hostfs and dispatches the guest path,
//! so the tests need no host toolchain and run on every host OS.

mod common;

use common::{BIN_MOUNT, require_bin, require_bins, require_rootfs, temp_dir};
use hyperlight_unikraft::{Mount, SandboxBuilder};

/// Environment handed to the `env_vars` examples.
const ENV: &[(&str, &str)] = &[
    ("MY_VAR", "hello_world"),
    ("DEBUG", "1"),
    ("GREETING", "hi there"),
];

/// Boot `runtime` with its prebuilt binaries at [`BIN_MOUNT`], run each
/// guest `path` in turn, and return the captured output.
async fn run_bins(
    runtime: &str,
    scratch_mb: usize,
    env: &[(&str, &str)],
    paths: &[&str],
) -> String {
    let rootfs = require_rootfs(runtime);
    let mounts = vec![Mount::rw(require_bins(runtime), BIN_MOUNT)];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(scratch_mb)
        .mounts(mounts)
        .boot()
        .unwrap();
    if !env.is_empty() {
        sandbox.set_env_vars(env);
    }
    for path in paths {
        sandbox.run(*path).await.unwrap();
    }
    sandbox.drain_output()
}

/// Snapshot `runtime` with an empty [`BIN_MOUNT`], restore it with the
/// prebuilt binaries mounted, run `path`, and return the output.
async fn run_bin_from_snapshot(runtime: &str, scratch_mb: usize, path: &str) -> String {
    let rootfs = require_rootfs(runtime);
    let snap_dir = temp_dir(&format!("{runtime}-snap"));
    let empty_mount = temp_dir(&format!("{runtime}-snap-mount"));

    let mounts_save = vec![Mount::rw(empty_mount.path(), BIN_MOUNT)];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(scratch_mb)
        .mounts(mounts_save)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mounts_run = vec![Mount::rw(require_bins(runtime), BIN_MOUNT)];
    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .mounts(mounts_run)
        .boot()
        .unwrap();
    sandbox.run(path).await.unwrap();
    sandbox.drain_output()
}

fn assert_env(output: &str) {
    assert!(
        output.contains("MY_VAR=hello_world"),
        "expected MY_VAR=hello_world, got: {output:?}"
    );
    assert!(
        output.contains("DEBUG=1"),
        "expected DEBUG=1, got: {output:?}"
    );
    assert!(
        output.contains("GREETING=hi there"),
        "expected GREETING=hi there, got: {output:?}"
    );
}

// ── C ────────────────────────────────────────────────────────────

#[tokio::test]
async fn c_hello() {
    let output = run_bins("c", 64, &[], &["/mnt/bin/hello"]).await;
    assert!(
        output.contains("Hello from C on Hyperlight"),
        "expected C hello output, got: {output:?}"
    );
}

/// Waiters released together by a broadcast all wake: musl's condition
/// variables hand each one's successor to the mutex with FUTEX_REQUEUE,
/// which the kernel once refused, leaving all but one asleep.
#[tokio::test]
async fn c_condvar_broadcast_wakes_every_waiter() {
    let (status, output) = run_c_check("condvar").await;
    assert!(
        status.is_ok() && output.contains("every waiter woke, 50 rounds"),
        "expected every waiter to wake, got {status:?}: {output}"
    );
}

/// FUTEX_REQUEUE and FUTEX_CMP_REQUEUE as Linux defines them: a requeue
/// wakes only as many as asked, moves the next ones, returns when the
/// target is the source, and a requeued waiter that times out says so.
#[tokio::test]
async fn c_futex_requeue_matches_linux() {
    let (status, output) = run_c_check("futex_requeue").await;
    assert!(
        status.is_ok() && output.contains("futex requeue: all checks pass"),
        "expected every futex check to pass, got {status:?}: {output}"
    );
}

/// sigsuspend() for a signal the caller blocks runs its handler before the
/// old mask comes back, so `while (!got) sigsuspend(&old)` takes one call,
/// for a signal queued before the call or arriving during it.
#[tokio::test]
async fn c_sigsuspend_runs_the_handler() {
    let (status, output) = run_c_check("sigsuspend").await;
    assert!(
        status.is_ok() && output.contains("sigsuspend: all checks pass"),
        "expected every sigsuspend check to pass, got {status:?}: {output}"
    );
}

/// A vfork() parent resumes for its own child's exit, not for another
/// thread of its that ends while the child runs.
#[tokio::test]
async fn c_vfork_parent_waits_for_its_child() {
    let (status, output) = run_c_check("vfork_wake").await;
    assert!(
        status.is_ok() && output.contains("vfork wake: all checks pass"),
        "expected the vfork check to pass, got {status:?}: {output}"
    );
}

/// mremap() resizes a mapping in place, with Linux's errors, and musl's
/// probes with it find the main thread's whole stack, not one page.
#[tokio::test]
async fn c_mremap_resizes_in_place() {
    let (status, output) = run_c_check("mremap").await;
    assert!(
        status.is_ok() && output.contains("mremap: all checks pass"),
        "expected every mremap check to pass, got {status:?}: {output}"
    );
}

/// AVX is off until a program uses it, then on: a blocked thread keeps its
/// SSE registers across the switch, and AVX registers survive another
/// thread using them.  Restoring a snapshot from before turns AVX off
/// again; one from after keeps it on, in this VM or a new one.
#[cfg(target_arch = "x86_64")]
#[tokio::test]
async fn c_avx_is_enabled_on_first_use() {
    use hyperlight_unikraft::{AppSandbox, Exec};

    async fn run(sandbox: &mut AppSandbox, args: &str, expect: &str) {
        let status = sandbox
            .run(Exec::Guest(format!("{BIN_MOUNT}/lazy_avx{args}")))
            .await;
        let output = sandbox.drain_output();
        assert!(
            status.is_ok() && output.contains(expect),
            "expected `{expect}` from lazy_avx{args}, got {status:?}: {output}"
        );
    }

    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("c"))
        .scratch_mb(64)
        .mounts(vec![Mount::rw(require_bin("c", "lazy_avx"), BIN_MOUNT)])
        .boot()
        .unwrap();
    let before = sandbox.snapshot().unwrap();
    let status = sandbox
        .run(Exec::Guest(format!("{BIN_MOUNT}/lazy_avx")))
        .await;
    let output = sandbox.drain_output();
    if status.is_ok() && output.contains("lazy avx: no AVX on this CPU") {
        return;
    }
    assert!(
        status.is_ok() && output.contains("lazy avx: all checks pass"),
        "expected every lazy AVX check to pass, got {status:?}: {output}"
    );
    let dir = temp_dir("lazy-avx");
    let after = sandbox.snapshot_to(dir.path()).unwrap();

    sandbox.restore(before).unwrap();
    run(&mut sandbox, "", "lazy avx: all checks pass").await;
    sandbox.restore(after).unwrap();
    run(
        &mut sandbox,
        " restored",
        "lazy avx: restored, all checks pass",
    )
    .await;

    // The same snapshot loaded into a new VM
    let mut resumed = SandboxBuilder::from_snapshot_dir(dir.path())
        .unwrap()
        .mounts(vec![Mount::rw(require_bin("c", "lazy_avx"), BIN_MOUNT)])
        .boot()
        .unwrap();
    run(
        &mut resumed,
        " restored",
        "lazy avx: restored, all checks pass",
    )
    .await;
}

/// What a process maps goes when it ends, by exit, a signal or an exec
/// from a thread: 56 children that each map 16 MiB run in a 64 MiB guest.
/// A thread that execs a program without libc leaves the kernel no stale
/// child TID to write when that program exits.
#[tokio::test]
async fn c_process_memory_is_released() {
    require_bin("c", "no_libc");
    let (status, output) = run_c_check("process_memory").await;
    assert!(
        status.is_ok() && output.contains("process memory: all checks pass"),
        "expected every process memory check to pass, got {status:?}: {output}"
    );
}

/// munmap() and mprotect() of a range that begins in a 2 MiB block with
/// nothing mapped reach the pages past the boundary: they once kept their
/// memory, which a later mapping there read, and their protection.
#[tokio::test]
async fn c_page_boundary_unmap_and_protect() {
    let (status, output) = run_c_check("page_boundary").await;
    assert!(
        status.is_ok() && output.contains("page boundary: all checks pass"),
        "expected every page boundary check to pass, got {status:?}: {output}"
    );
}

/// Run a C check program, keeping its output when it fails: the output
/// says which check did.
async fn run_c_check(name: &str) -> (hyperlight_unikraft::Result<()>, String) {
    let mounts = vec![Mount::rw(require_bin("c", name), BIN_MOUNT)];
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("c"))
        .scratch_mb(64)
        .mounts(mounts)
        .boot()
        .unwrap();
    let status = sandbox.run(format!("{BIN_MOUNT}/{name}")).await;
    (status, sandbox.drain_output())
}

/// A program that exits non-zero fails the call, as it would fail a
/// shell: the exec driver collects the child's status instead of taking
/// the closed pipe for a success.
#[tokio::test]
async fn c_exit_status_fails_the_call() {
    let rootfs = require_rootfs("c");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(64)
        .mounts(vec![Mount::rw(require_bins("c"), BIN_MOUNT)])
        .boot()
        .unwrap();
    sandbox.run("/mnt/bin/hello").await.unwrap();
    assert!(
        sandbox.run("/mnt/bin/status").await.is_err(),
        "exit status 3 was reported as success"
    );
    sandbox
        .run("/mnt/bin/hello")
        .await
        .expect("the guest serves calls after a failed one");
    assert!(sandbox.drain_output().contains("Hello from C"));
}

#[tokio::test]
async fn c_snapshot_round_trip() {
    let output = run_bin_from_snapshot("c", 64, "/mnt/bin/hello").await;
    assert!(
        output.contains("Hello from C on Hyperlight"),
        "expected C hello from snapshot, got: {output:?}"
    );
}

#[tokio::test]
async fn c_multi_binary_mount() {
    let output = run_bins("c", 64, &[], &["/mnt/bin/hello", "/mnt/bin/goodbye"]).await;
    assert!(
        output.contains("Hello from C on Hyperlight"),
        "expected hello output, got: {output:?}"
    );
    assert!(
        output.contains("Goodbye from C on Hyperlight"),
        "expected goodbye output, got: {output:?}"
    );
}

/// A file the exec driver cannot execute fails the call with 126, and the
/// driver says which and why, as a shell does: the kernel keeps quiet
/// about an execve that fails as a PATH search expects.
#[tokio::test]
async fn c_unexecutable_file_says_why() {
    let rootfs = require_rootfs("c");
    let dir = temp_dir("c-not-elf");
    let script = dir.path().join("not-elf");
    std::fs::write(&script, "no shebang, no ELF\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(64)
        .echo_output(false)
        .mounts(vec![Mount::rw(dir.path(), "/mnt/x")])
        .boot()
        .unwrap();
    assert!(matches!(
        sandbox.run("/mnt/x/not-elf").await,
        Err(hyperlight_unikraft::Error::CallFailed { status: 126 })
    ));
    let stderr = sandbox.drain_stderr();
    assert!(
        stderr.contains("hl: cannot execute /mnt/x/not-elf: Exec format error"),
        "{stderr:?}"
    );
    // A file without an execute bit is not run either, and says so: the
    // mount carries the host's mode bits.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(matches!(
            sandbox.run("/mnt/x/not-elf").await,
            Err(hyperlight_unikraft::Error::CallFailed { status: 126 })
        ));
        let stderr = sandbox.drain_stderr();
        assert!(
            stderr.contains("hl: cannot execute /mnt/x/not-elf: Permission denied"),
            "{stderr:?}"
        );
    }
}

#[tokio::test]
async fn cpp_hello() {
    let output = run_bins("c", 64, &[], &["/mnt/bin/hello_cpp"]).await;
    assert!(
        output.contains("Hello from C++ on Hyperlight"),
        "expected C++ hello output, got: {output:?}"
    );
}

/// A plain program as the guest's entry point: no driver, nothing to
/// dispatch.  It runs to completion during boot, and `join` hands back its
/// exit status -- what `hluk run --entry` mirrors.
#[tokio::test]
async fn c_entry_point_exit_status() {
    let rootfs = require_rootfs("c");
    let bins = require_bins("c");
    for (bin, status, marker) in [("hello", 0, "Hello from C"), ("status", 3, "")] {
        let mut sandbox = SandboxBuilder::from_initrd(&rootfs)
            .scratch_mb(64)
            .mounts(vec![Mount::rw(&bins, BIN_MOUNT)])
            .entry(format!("{BIN_MOUNT}/{bin}"))
            .boot()
            .unwrap();
        assert!(
            !sandbox.has_driver(),
            "{bin}: a plain program is not a driver"
        );
        assert!(
            sandbox.submit("x").is_err(),
            "{bin}: nothing in the guest serves calls"
        );
        assert_eq!(sandbox.join().await.unwrap(), status, "{bin}");
        let output = sandbox.drain_output();
        assert!(output.contains(marker), "{bin}: got {output:?}");
    }
}

/// Builder env vars reach a program that is the entry point: the kernel
/// fetches the environment on its way to main(), before any call could
/// refresh it, so they must be set before the boot.
#[tokio::test]
async fn c_entry_point_sees_builder_env() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("c"))
        .scratch_mb(64)
        .mounts(vec![Mount::rw(require_bins("c"), BIN_MOUNT)])
        .entry(format!("{BIN_MOUNT}/env_vars"))
        .env("MY_VAR", "hello_world")
        .env("DEBUG", "1")
        .boot()
        .unwrap();
    assert_eq!(sandbox.join().await.unwrap(), 0);
    let output = sandbox.drain_output();
    assert!(output.contains("MY_VAR=hello_world"), "got: {output:?}");
    assert!(output.contains("DEBUG=1"), "got: {output:?}");
}

#[tokio::test]
async fn c_env_vars() {
    assert_env(&run_bins("c", 64, ENV, &["/mnt/bin/env_vars"]).await);
}

// ── Rust ─────────────────────────────────────────────────────────

#[tokio::test]
async fn rust_hello() {
    let output = run_bins("rust", 64, &[], &["/mnt/bin/hello"]).await;
    assert!(
        output.contains("Hello from Rust on Hyperlight"),
        "expected Rust hello output, got: {output:?}"
    );
}

#[tokio::test]
async fn rust_snapshot_round_trip() {
    let output = run_bin_from_snapshot("rust", 64, "/mnt/bin/hello").await;
    assert!(
        output.contains("Hello from Rust on Hyperlight"),
        "expected Rust hello from snapshot, got: {output:?}"
    );
}

#[tokio::test]
async fn rust_env_vars() {
    assert_env(&run_bins("rust", 64, ENV, &["/mnt/bin/env_vars"]).await);
}

// ── Go ───────────────────────────────────────────────────────────

#[tokio::test]
async fn go_hello() {
    let output = run_bins("go", 128, &[], &["/mnt/bin/hello"]).await;
    assert!(
        output.contains("Hello from Go on Hyperlight"),
        "expected Go hello output, got: {output:?}"
    );
}

#[tokio::test]
async fn go_snapshot_round_trip() {
    let output = run_bin_from_snapshot("go", 128, "/mnt/bin/hello").await;
    assert!(
        output.contains("Hello from Go on Hyperlight"),
        "expected Go hello from snapshot, got: {output:?}"
    );
}

#[tokio::test]
async fn go_env_vars() {
    assert_env(&run_bins("go", 128, ENV, &["/mnt/bin/env_vars"]).await);
}

// ── .NET AOT ─────────────────────────────────────────────────────

#[tokio::test]
async fn dotnet_aot_hello() {
    let output = run_bins("dotnet-aot", 256, &[], &["/mnt/bin/Hello"]).await;
    assert!(
        output.contains("Hello from .NET AOT on Hyperlight"),
        "expected .NET AOT hello output, got: {output:?}"
    );
}

#[tokio::test]
async fn dotnet_aot_snapshot_round_trip() {
    let output = run_bin_from_snapshot("dotnet-aot", 256, "/mnt/bin/Hello").await;
    assert!(
        output.contains("Hello from .NET AOT on Hyperlight"),
        "expected .NET AOT hello from snapshot, got: {output:?}"
    );
}

#[tokio::test]
async fn dotnet_aot_env_vars() {
    assert_env(&run_bins("dotnet-aot", 256, ENV, &["/mnt/bin/EnvVars"]).await);
}

#[tokio::test]
async fn dotnet_aot_capabilities() {
    // Guest filesystem (ramfs) + host filesystem (a --mount dir) + concurrency
    // (Task fan-out) in a prebuilt native-AOT executable.  The binary comes
    // from BIN_MOUNT; it writes to guest /tmp and to a second, writable host
    // mount at /mnt/out, which we verify on the host side.
    let rootfs = require_rootfs("dotnet-aot");
    let out_dir = temp_dir("dotnet-aot-caps");

    let mounts = vec![
        Mount::rw(require_bins("dotnet-aot"), BIN_MOUNT),
        Mount::rw(out_dir.path(), "/mnt/out"),
    ];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();
    sandbox.run("/mnt/bin/Caps").await.unwrap();
    let output = sandbox.drain_output();

    assert!(
        output.contains("guest-fs-read-back: aot-guest-fs-data"),
        "expected AOT Caps to read back its guest-fs (ramfs) write, got: {output:?}"
    );
    assert!(
        output.contains("host-fs-read-back: aot-host-fs-data"),
        "expected AOT Caps to read back its host-fs write, got: {output:?}"
    );
    assert!(
        output.contains("sum-of-squares: 140") && output.contains("aot-caps-done"),
        "expected AOT Caps concurrency result, got: {output:?}"
    );
    // The host-fs write went through hostfs and landed on the host side.
    assert!(
        out_dir.path().join("aot_host_fs.txt").exists(),
        "expected AOT Caps host-fs write to appear on the host mount"
    );
}
