// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::{hluk_with_stdin, require_rootfs, snapshot_dir};
use hyperlight_unikraft::{Exec, OciTag, SNAPSHOT_TAG, SandboxBuilder, Snapshot, run};

#[test]
fn bash_inline_code() {
    let rootfs = require_rootfs("bash");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, "echo 'hluk-bash-ok'").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("hluk-bash-ok"),
        "expected guest to print 'hluk-bash-ok', got: {output:?}",
    );
}

#[test]
fn bash_exec_file() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/hello.sh");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.sh to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn bash_snapshot_round_trip() {
    let rootfs = require_rootfs("bash");
    let snap_dir = snapshot_dir("bash-snap");

    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let snap = sandbox.snapshot().unwrap();
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    snap.save(&snap_dir, &tag).unwrap();

    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    let snap = Arc::new(Snapshot::load(&snap_dir, tag).unwrap());
    let (mut sandbox, cfg2) = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    run(&mut sandbox, "echo 'restored-bash-ok'").unwrap();
    let output = cfg2.drain_output();
    assert!(
        output.contains("restored-bash-ok"),
        "expected restored guest to print 'restored-bash-ok', got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&snap_dir);
}

#[test]
fn bash_multiple_runs() {
    let rootfs = require_rootfs("bash");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, "x=42").unwrap();
    run(&mut sandbox, "echo \"x=$x\"").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("x=42"),
        "expected 'x=42' after multiple runs, got: {output:?}",
    );
}

#[test]
fn bash_coreutils() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/coreutils.sh");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();

    assert!(
        output.contains("=== cat ==="),
        "missing cat section: {output:?}"
    );
    assert!(
        output.contains("alice:admin:login"),
        "cat didn't print file: {output:?}"
    );
    assert!(
        output.contains("=== grep admin ==="),
        "missing grep section: {output:?}"
    );
    assert!(
        output.contains("=== sort ==="),
        "missing sort section: {output:?}"
    );
    assert!(
        output.contains("=== awk table ==="),
        "missing awk section: {output:?}"
    );
    assert!(
        output.contains("=== ls ==="),
        "missing ls section: {output:?}"
    );
    assert!(
        output.contains("=== find *.txt ==="),
        "missing find section: {output:?}"
    );
    assert!(
        output.contains("=== sed s/viewer/readonly/ ==="),
        "missing sed section: {output:?}"
    );
    assert!(
        output.contains("=== seq ==="),
        "missing seq section: {output:?}"
    );
    assert!(
        output.contains("=== hexdump ==="),
        "missing hexdump section: {output:?}"
    );
    assert!(output.contains("Done"), "script didn't finish: {output:?}");
}

#[test]
fn bash_stdin_piped() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/stdin_echo.sh");
    let output = hluk_with_stdin(&rootfs, &script, b"hello from host\nline two\n");
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
fn bash_shell_interactive() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/shell.sh");
    let output = hluk_with_stdin(&rootfs, &script, b"echo hello-from-shell\necho 42\n");
    assert!(
        output.contains("hello-from-shell"),
        "expected echo output, got: {output:?}"
    );
    assert!(
        output.contains("42"),
        "expected second echo, got: {output:?}"
    );
}

#[test]
fn bash_env_vars() {
    let rootfs = require_rootfs("bash");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
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
        Exec::File(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/env_vars.sh")),
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
