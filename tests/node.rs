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
fn node_exec_file() {
    let rootfs = require_rootfs("node");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/hello.js");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.js to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn node_inline_code() {
    let rootfs = require_rootfs("node");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    run(&mut sandbox, "console.log('hluk-node-ok')").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("hluk-node-ok"),
        "expected guest to print 'hluk-node-ok', got: {output:?}",
    );
}

#[test]
fn node_snapshot_round_trip() {
    let rootfs = require_rootfs("node");
    let snap_dir = snapshot_dir("node-snap");
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs.clone())
        .scratch_mb(512)
        .boot()
        .unwrap();
    let snap = sandbox.snapshot().unwrap();
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    snap.save(&snap_dir, &tag).unwrap();

    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    let snap = Arc::new(Snapshot::load(&snap_dir, tag).unwrap());
    let (mut sandbox, cfg2) = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    run(&mut sandbox, "console.log('restored-node-ok')").unwrap();
    let output = cfg2.drain_output();
    assert!(
        output.contains("restored-node-ok"),
        "expected restored node output, got: {output:?}",
    );
    let _ = std::fs::remove_dir_all(&snap_dir);
}

#[test]
fn node_stdin_piped() {
    let rootfs = require_rootfs("node");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/stdin_echo.js");
    let output = hluk_with_stdin_scratch(&rootfs, &script, b"hello from host\nline two\n", 512);
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
fn node_async_timers() {
    let rootfs = require_rootfs("node");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/async_timers.js");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    let code = std::fs::read_to_string(&script).unwrap();
    run(&mut sandbox, &*code).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("results: timer-1, timer-2"),
        "expected chained timer results, got: {output:?}",
    );
    assert!(
        output.contains("async-done"),
        "expected async-done marker, got: {output:?}",
    );
}

#[test]
fn node_fs_ops() {
    let rootfs = require_rootfs("node");
    let mount_dir = std::env::temp_dir().join(format!("hluk-node-fs-ops-{}", std::process::id()));
    std::fs::create_dir_all(&mount_dir).unwrap();

    let mounts = vec![Mount::rw(&mount_dir, "/mnt/host")];
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/fs_ops.js");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Cleanup done"),
        "expected fs_ops.js to print 'Cleanup done', got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&mount_dir);
}

#[test]
fn node_http_get() {
    let rootfs = require_rootfs("node");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/http_get.js");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Status: 200"),
        "expected http_get.js to get Status: 200, got: {output:?}",
    );
}

#[test]
fn node_env_vars() {
    let rootfs = require_rootfs("node");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
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
        Exec::File(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/env_vars.js")),
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
