// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::{hluk_with_stdin_scratch, require_rootfs, snapshot_dir};
use hyperlight_unikraft::{
    Exec, Mount, NetworkPolicy, OciTag, SNAPSHOT_TAG, SandboxBuilder, Snapshot, run,
};

#[test]
fn dotnet_jit_inline_code() {
    let rootfs = require_rootfs("dotnet-jit");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    run(&mut sandbox, "Console.WriteLine(\"hluk-dotnet-ok\");").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("hluk-dotnet-ok"),
        "expected guest to print 'hluk-dotnet-ok', got: {output:?}",
    );
}

#[test]
fn dotnet_jit_exec_file() {
    let rootfs = require_rootfs("dotnet-jit");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/Hello.cs");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello"),
        "expected Hello.cs to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn dotnet_jit_snapshot_round_trip() {
    let rootfs = require_rootfs("dotnet-jit");
    let snap_dir = snapshot_dir("dotnet-jit-snap");

    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    let snap = sandbox.snapshot().unwrap();
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    snap.save(&snap_dir, &tag).unwrap();

    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    let snap = Arc::new(Snapshot::load(&snap_dir, tag).unwrap());
    let (mut sandbox, cfg2) = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    run(&mut sandbox, "Console.WriteLine(\"restored-dotnet-ok\");").unwrap();
    let output = cfg2.drain_output();
    assert!(
        output.contains("restored-dotnet-ok"),
        "expected restored guest to print 'restored-dotnet-ok', got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&snap_dir);
}

#[test]
fn dotnet_jit_multiple_runs() {
    let rootfs = require_rootfs("dotnet-jit");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();

    run(&mut sandbox, "Console.WriteLine(\"run1-ok\");").unwrap();
    let output1 = cfg.drain_output();
    assert!(
        output1.contains("run1-ok"),
        "expected 'run1-ok' from first dispatch, got: {output1:?}"
    );

    run(&mut sandbox, "Console.WriteLine($\"x={1 + 1}\");").unwrap();
    let output2 = cfg.drain_output();
    assert!(
        output2.contains("x=2"),
        "expected 'x=2' from second dispatch, got: {output2:?}"
    );
}

#[test]
fn dotnet_jit_math() {
    let rootfs = require_rootfs("dotnet-jit");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/Math.cs");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("=== .NET Math Demo ==="),
        "expected Math.cs header, got: {output:?}"
    );
    assert!(
        output.contains("Math demo done"),
        "expected Math.cs to complete, got: {output:?}"
    );
}

#[test]
fn dotnet_jit_stdin_piped() {
    let rootfs = require_rootfs("dotnet-jit");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/StdinEcho.cs");
    let output = hluk_with_stdin_scratch(&rootfs, &script, b"hello from host\nline two\n", 768);
    assert!(
        output.contains("lines=2"),
        "expected 2 lines, got: {output:?}"
    );
    assert!(
        output.contains("echo: hello from host"),
        "expected first line, got: {output:?}"
    );
    assert!(
        output.contains("stdin-done"),
        "expected stdin-done marker, got: {output:?}"
    );
}

#[test]
fn dotnet_jit_env_vars() {
    let rootfs = require_rootfs("dotnet-jit");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    cfg.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ])
    .unwrap();
    run(
        &mut sandbox,
        Exec::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/EnvVars.cs"),
        ),
    )
    .unwrap();
    let output = cfg.drain_output();
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

#[test]
fn dotnet_jit_fs() {
    let rootfs = require_rootfs("dotnet-jit");
    let mount_dir = std::env::temp_dir().join(format!("hluk-dotnet-jit-fs-{}", std::process::id()));
    std::fs::create_dir_all(&mount_dir).unwrap();

    let mounts = vec![Mount::rw(&mount_dir, "/mnt/host")];
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .mounts(mounts)
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/FsOps.cs");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();

    assert!(
        output.contains("read-back: hello from dotnet"),
        "expected FsOps.cs to read back its write, got: {output:?}",
    );
    assert!(
        output.contains("line-count: 3") && output.contains("fs-ops-done"),
        "expected FsOps.cs to finish, got: {output:?}",
    );
    // The write landed on the host side of the mount.
    assert!(mount_dir.join("dotnet_fs.txt").exists());

    let _ = std::fs::remove_dir_all(&mount_dir);
}

#[test]
fn dotnet_jit_threading() {
    let rootfs = require_rootfs("dotnet-jit");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/Threads.cs");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();

    assert!(
        output.contains("sum-of-squares: 140"),
        "expected Task fan-out result 140, got: {output:?}",
    );
    assert!(
        output.contains("thread-counter: 4") && output.contains("threads-done"),
        "expected 4 threads to increment the counter, got: {output:?}",
    );
}

#[test]
fn dotnet_jit_http_get() {
    let rootfs = require_rootfs("dotnet-jit");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/HttpGet.cs");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();

    assert!(
        output.contains("Status: 200"),
        "expected HttpGet.cs to get Status: 200, got: {output:?}",
    );
}
