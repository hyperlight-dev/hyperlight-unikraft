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
        output.contains("2 admin") && output.contains("1 editor"),
        "uniq -c pipeline didn't count roles: {output:?}"
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

/// Pipes, `$(...)` substitutions and redirects of external commands work:
/// the shell's vfork child ran on the kernel's TLS rather than the shell's,
/// and crashed the guest when it made a system call before its `execve`.
#[test]
fn bash_pipes_and_substitutions() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("echo hello | tr a-z A-Z").unwrap();
    assert_eq!(sandbox.drain_output(), "HELLO\n");
    sandbox.run(r#"x=$(echo hi); echo "got $x""#).unwrap();
    assert_eq!(sandbox.drain_output(), "got hi\n");
    sandbox
        .run("/bin/echo to-file > /tmp/f; cat /tmp/f")
        .unwrap();
    assert_eq!(sandbox.drain_output(), "to-file\n");
    sandbox
        .run(r#"printf 'b\na\nb\n' | sort | uniq -c | wc -l"#)
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "2");
    // An exec'd program inherits fd 2 as the shell's own stderr file.
    sandbox.run("ls /nope; true").unwrap();
    assert!(sandbox.drain_stderr().contains("No such file"));
    sandbox.drain_output();
    // fd 2 is a file of its own: redirects still move it.
    sandbox.run("ls /nope 2>&1 | wc -l").unwrap();
    assert_eq!(sandbox.drain_output().trim(), "1");
    sandbox.run("ls /nope 2>/dev/null; echo rc=$?").unwrap();
    assert_eq!(sandbox.drain_stderr(), "");
    assert_eq!(sandbox.drain_output(), "rc=1\n");
    sandbox.run("echo to-err >&2").unwrap();
    assert_eq!(sandbox.drain_stderr(), "to-err\n");
    // A command that fails to execute: its vfork child exits without an
    // execve, and must not free the shell's stack on its way out.
    sandbox.run("nope; sleep 0.1; echo after").unwrap();
    // The kernel's messages about it are on stderr, without the NUL
    // bytes they had.
    let stderr = sandbox.drain_stderr();
    assert!(
        stderr.contains("Unable to load") && !stderr.contains('\0'),
        "{stderr:?}"
    );
    assert!(sandbox.drain_output().ends_with("after\n"));
    // Its kernel message outgrows the kernel's 191-byte line: with an
    // odd-length prefix the byte limit falls inside an 'é', and the cut
    // must back off to a whole character, or the host aborts the call.
    sandbox
        .run("p=/xy; i=0; while [ $i -lt 100 ]; do p=${p}\u{e9}; i=$((i+1)); done; $p; echo after")
        .unwrap();
    // The cut message ends its line: the next kernel message starts one.
    let stderr = sandbox.drain_stderr();
    assert!(
        stderr.contains("\u{e9}\n[") && stderr.ends_with('\n'),
        "{stderr:?}"
    );
    assert!(sandbox.drain_output().ends_with("after\n"));
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
    assert_eq!(sandbox.drain_output(), "leaving\n");
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
