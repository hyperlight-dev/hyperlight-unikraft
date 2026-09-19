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

use common::{BIN_MOUNT, require_bins, require_rootfs, temp_dir};
use hyperlight_unikraft::{Mount, SandboxBuilder};

/// Environment handed to the `env_vars` examples.
const ENV: &[(&str, &str)] = &[
    ("MY_VAR", "hello_world"),
    ("DEBUG", "1"),
    ("GREETING", "hi there"),
];

/// Boot `runtime` with its prebuilt binaries at [`BIN_MOUNT`], run each
/// guest `path` in turn, and return the captured output.
fn run_bins(runtime: &str, scratch_mb: usize, env: &[(&str, &str)], paths: &[&str]) -> String {
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
        sandbox.run(*path).unwrap();
    }
    sandbox.drain_output()
}

/// Snapshot `runtime` with an empty [`BIN_MOUNT`], restore it with the
/// prebuilt binaries mounted, run `path`, and return the output.
fn run_bin_from_snapshot(runtime: &str, scratch_mb: usize, path: &str) -> String {
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
    sandbox.run(path).unwrap();
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

#[test]
fn c_hello() {
    let output = run_bins("c", 64, &[], &["/mnt/bin/hello"]);
    assert!(
        output.contains("Hello from C on Hyperlight"),
        "expected C hello output, got: {output:?}"
    );
}

/// A program that exits non-zero fails the call, as it would fail a
/// shell: the exec driver collects the child's status instead of taking
/// the closed pipe for a success.
#[test]
fn c_exit_status_fails_the_call() {
    let rootfs = require_rootfs("c");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(64)
        .mounts(vec![Mount::rw(require_bins("c"), BIN_MOUNT)])
        .boot()
        .unwrap();
    sandbox.run("/mnt/bin/hello").unwrap();
    assert!(
        sandbox.run("/mnt/bin/status").is_err(),
        "exit status 3 was reported as success"
    );
    sandbox
        .run("/mnt/bin/hello")
        .expect("the guest serves calls after a failed one");
    assert!(sandbox.drain_output().contains("Hello from C"));
}

#[test]
fn c_snapshot_round_trip() {
    let output = run_bin_from_snapshot("c", 64, "/mnt/bin/hello");
    assert!(
        output.contains("Hello from C on Hyperlight"),
        "expected C hello from snapshot, got: {output:?}"
    );
}

#[test]
fn c_multi_binary_mount() {
    let output = run_bins("c", 64, &[], &["/mnt/bin/hello", "/mnt/bin/goodbye"]);
    assert!(
        output.contains("Hello from C on Hyperlight"),
        "expected hello output, got: {output:?}"
    );
    assert!(
        output.contains("Goodbye from C on Hyperlight"),
        "expected goodbye output, got: {output:?}"
    );
}

#[test]
fn cpp_hello() {
    let output = run_bins("c", 64, &[], &["/mnt/bin/hello_cpp"]);
    assert!(
        output.contains("Hello from C++ on Hyperlight"),
        "expected C++ hello output, got: {output:?}"
    );
}

/// A plain program as the guest's entry point: no driver, nothing to
/// dispatch.  It runs to completion during boot, and `join` hands back its
/// exit status -- what `hluk run --entry` mirrors.
#[test]
fn c_entry_point_exit_status() {
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
        assert_eq!(sandbox.join().unwrap(), status, "{bin}");
        let output = sandbox.drain_output();
        assert!(output.contains(marker), "{bin}: got {output:?}");
    }
}

/// Builder env vars reach a program that is the entry point: the kernel
/// fetches the environment on its way to main(), before any call could
/// refresh it, so they must be set before the boot.
#[test]
fn c_entry_point_sees_builder_env() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("c"))
        .scratch_mb(64)
        .mounts(vec![Mount::rw(require_bins("c"), BIN_MOUNT)])
        .entry(format!("{BIN_MOUNT}/env_vars"))
        .env("MY_VAR", "hello_world")
        .env("DEBUG", "1")
        .boot()
        .unwrap();
    assert_eq!(sandbox.join().unwrap(), 0);
    let output = sandbox.drain_output();
    assert!(output.contains("MY_VAR=hello_world"), "got: {output:?}");
    assert!(output.contains("DEBUG=1"), "got: {output:?}");
}

#[test]
fn c_env_vars() {
    assert_env(&run_bins("c", 64, ENV, &["/mnt/bin/env_vars"]));
}

// ── Rust ─────────────────────────────────────────────────────────

#[test]
fn rust_hello() {
    let output = run_bins("rust", 64, &[], &["/mnt/bin/hello"]);
    assert!(
        output.contains("Hello from Rust on Hyperlight"),
        "expected Rust hello output, got: {output:?}"
    );
}

#[test]
fn rust_snapshot_round_trip() {
    let output = run_bin_from_snapshot("rust", 64, "/mnt/bin/hello");
    assert!(
        output.contains("Hello from Rust on Hyperlight"),
        "expected Rust hello from snapshot, got: {output:?}"
    );
}

#[test]
fn rust_env_vars() {
    assert_env(&run_bins("rust", 64, ENV, &["/mnt/bin/env_vars"]));
}

// ── Go ───────────────────────────────────────────────────────────

#[test]
fn go_hello() {
    let output = run_bins("go", 128, &[], &["/mnt/bin/hello"]);
    assert!(
        output.contains("Hello from Go on Hyperlight"),
        "expected Go hello output, got: {output:?}"
    );
}

#[test]
fn go_snapshot_round_trip() {
    let output = run_bin_from_snapshot("go", 128, "/mnt/bin/hello");
    assert!(
        output.contains("Hello from Go on Hyperlight"),
        "expected Go hello from snapshot, got: {output:?}"
    );
}

#[test]
fn go_env_vars() {
    assert_env(&run_bins("go", 128, ENV, &["/mnt/bin/env_vars"]));
}

// ── .NET AOT ─────────────────────────────────────────────────────

#[test]
fn dotnet_aot_hello() {
    let output = run_bins("dotnet-aot", 256, &[], &["/mnt/bin/Hello"]);
    assert!(
        output.contains("Hello from .NET AOT on Hyperlight"),
        "expected .NET AOT hello output, got: {output:?}"
    );
}

#[test]
fn dotnet_aot_snapshot_round_trip() {
    let output = run_bin_from_snapshot("dotnet-aot", 256, "/mnt/bin/Hello");
    assert!(
        output.contains("Hello from .NET AOT on Hyperlight"),
        "expected .NET AOT hello from snapshot, got: {output:?}"
    );
}

#[test]
fn dotnet_aot_env_vars() {
    assert_env(&run_bins("dotnet-aot", 256, ENV, &["/mnt/bin/EnvVars"]));
}

#[test]
fn dotnet_aot_capabilities() {
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
    sandbox.run("/mnt/bin/Caps").unwrap();
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
