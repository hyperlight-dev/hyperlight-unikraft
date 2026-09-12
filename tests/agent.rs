// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::{require_rootfs, snapshot_dir};
use hyperlight_unikraft::{
    Exec, NetworkPolicy, OciTag, SNAPSHOT_TAG, SandboxBuilder, Snapshot, run,
};

// ── Agent (full) tests ───────────────────────────────────────────

#[test]
fn agent_hello() {
    let rootfs = require_rootfs("agent");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/hello.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello from the Hyperlight agent"),
        "expected agent hello.py output, got: {output:?}",
    );
}

#[test]
fn agent_data_science() {
    let rootfs = require_rootfs("agent");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/data_science.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("slope") && output.contains("sklearn"),
        "expected data_science.py to run, got: {output:?}",
    );
}

#[test]
fn agent_shell_subprocess() {
    let rootfs = require_rootfs("agent");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/shell_commands.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello from hush shell!"),
        "expected shell_commands.py to run, got: {output:?}",
    );
}

#[test]
fn agent_ssl_available() {
    let rootfs = require_rootfs("agent");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .boot()
        .unwrap();
    run(
        &mut sandbox,
        r#"
import ssl
print(f'ssl={ssl.OPENSSL_VERSION}')
import sqlite3
print('sqlite3-ok')
import ctypes
print('ctypes-ok')
"#,
    )
    .unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("ssl="),
        "SSL module not available: {output:?}"
    );
    assert!(
        output.contains("sqlite3-ok"),
        "sqlite3 not available: {output:?}"
    );
    assert!(
        output.contains("ctypes-ok"),
        "ctypes not available: {output:?}"
    );
}

#[test]
fn agent_snapshot_round_trip() {
    let rootfs = require_rootfs("agent");
    let snap_dir = snapshot_dir("agent-snap");

    // Save
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .boot()
        .unwrap();
    let snap = sandbox.snapshot().unwrap();
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    snap.save(&snap_dir, &tag).unwrap();

    // Restore + run hello.py from snapshot
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    let snap = Arc::new(Snapshot::load(&snap_dir, tag).unwrap());
    let (mut sandbox, cfg2) = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/hello.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg2.drain_output();
    assert!(
        output.contains("Hello from the Hyperlight agent"),
        "expected agent hello.py to work after snapshot restore, got: {output:?}",
    );
    assert!(
        output.contains("numpy"),
        "expected numpy available after restore, got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&snap_dir);
}

#[test]
fn agent_verify_all_packages() {
    let rootfs = require_rootfs("agent");
    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/verify_packages.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("ALL") && output.contains("packages verified"),
        "expected all packages to verify, got: {output:?}",
    );
    assert!(
        !output.contains("FAILED"),
        "some packages failed to import: {output:?}",
    );
}

#[test]
fn agent_pip_install() {
    let rootfs = require_rootfs("agent");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/pip_install.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1536)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Success!"),
        "expected pip_install.py to install and import six, got: {output:?}",
    );
}

// ── Agent custom rootfs test ─────────────────────────────────────

#[test]
fn agent_custom_packages() {
    let rootfs = require_rootfs("agent-custom");
    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/custom/custom_packages.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("pydantic=") && output.contains("yaml="),
        "expected custom rootfs to have pydantic and pyyaml, got: {output:?}",
    );
    assert!(
        output.contains("custom-rootfs-ok"),
        "expected custom rootfs test to pass, got: {output:?}",
    );
}
