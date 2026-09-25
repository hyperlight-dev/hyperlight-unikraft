// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;

use common::{hluk_with_stdin, require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, SandboxBuilder};

#[test]
fn bash_inline_code() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("echo 'hluk-bash-ok'").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("hluk-bash-ok"),
        "expected guest to print 'hluk-bash-ok', got: {output:?}",
    );
}

#[test]
fn bash_exec_file() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/hello.sh");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.sh to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn bash_snapshot_round_trip() {
    let rootfs = require_rootfs("bash");
    let snap_dir = temp_dir("bash-snap");

    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.run("echo 'restored-bash-ok'").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("restored-bash-ok"),
        "expected restored guest to print 'restored-bash-ok', got: {output:?}",
    );
}

#[test]
fn bash_multiple_runs() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("x=42").unwrap();
    sandbox.run("echo \"x=$x\"").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("x=42"),
        "expected 'x=42' after multiple runs, got: {output:?}",
    );
}

#[test]
fn bash_coreutils() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/coreutils.sh");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();

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
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox
        .run(Exec::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/env_vars.sh"),
        ))
        .unwrap();
    let output = sandbox.drain_output();
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

/// `exit` ends the shell, which is the runtime: the call ends with that
/// status, and the next call starts a fresh shell.
#[test]
fn bash_exit_ends_the_call() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("echo leaving; exit 0; echo not reached")
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\r\n");
    assert!(
        sandbox.run("exit 3").is_err(),
        "exit 3 did not fail the call"
    );
    sandbox.run("echo still here").unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}

/// Reading past the end of stdin keeps returning end of input: before,
/// the second `read` after it waited forever and the guest deadlocked.
#[test]
fn bash_read_past_end_of_input() {
    let rootfs = require_rootfs("bash");
    let dir = temp_dir("bash-eof");
    let script = dir.path().join("eof.sh");
    std::fs::write(
        &script,
        "read a; echo \"first:$? a=$a\"\nread b; echo \"second:$?\"\nread c; echo \"third:$?\"\n",
    )
    .unwrap();
    // No newline after the last byte: the first read gets it and the end.
    let output = hluk_with_stdin(&rootfs, &script, b"x");
    for line in ["first:1 a=x", "second:1", "third:1"] {
        assert!(output.contains(line), "{line} missing from: {output:?}");
    }
}

/// `call` on an image whose driver does not serve calls fails the call
/// and runs nothing: the function name is not taken for code.
#[test]
fn bash_refuses_a_call() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs).boot().unwrap();
    let err = sandbox.call("echo ran-as-code", "{}").unwrap_err();
    assert!(
        matches!(err, hyperlight_unikraft::Error::CallFailed { .. }),
        "{err}"
    );
    let output = sandbox.drain_output();
    assert!(!output.contains("ran-as-code"), "{output}");
    assert!(output.contains("does not serve Call calls"), "{output}");
    // The driver serves the next call as usual.
    sandbox.run("echo still-here").unwrap();
}
