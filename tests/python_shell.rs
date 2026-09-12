// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::{require_rootfs, snapshot_dir};
use hyperlight_unikraft::{Exec, OciTag, SNAPSHOT_TAG, SandboxBuilder, Snapshot, run};

#[test]
fn python_shell_hello() {
    let rootfs = require_rootfs("python-shell");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/hello.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello from the Hyperlight agent"),
        "expected agent hello.py to run on python-shell, got: {output:?}",
    );
}

#[test]
fn python_shell_ssl_available() {
    let rootfs = require_rootfs("python-shell");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
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
        "SSL not available in python-shell: {output:?}"
    );
    assert!(
        output.contains("sqlite3-ok"),
        "sqlite3 not available in python-shell: {output:?}"
    );
    assert!(
        output.contains("ctypes-ok"),
        "ctypes not available in python-shell: {output:?}"
    );
}

#[test]
fn python_shell_shell_subprocess() {
    let rootfs = require_rootfs("python-shell");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/agent/shell_commands.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello from hush shell!"),
        "expected shell_commands.py to work on python-shell, got: {output:?}",
    );
}

#[test]
fn python_shell_no_numpy() {
    let rootfs = require_rootfs("python-shell");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    // python-shell should NOT have numpy — it's the slim rootfs
    run(
        &mut sandbox,
        r#"
try:
    import numpy
    print('numpy-found')
except ImportError:
    print('numpy-missing-ok')
"#,
    )
    .unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("numpy-missing-ok"),
        "python-shell should NOT have numpy, got: {output:?}",
    );
}

#[test]
fn python_shell_snapshot_round_trip() {
    let rootfs = require_rootfs("python-shell");
    let snap_dir = snapshot_dir("python-shell-snap");

    // Save
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
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
        "expected hello.py to work after python-shell snapshot restore, got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&snap_dir);
}
