// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use common::{guest_log, hluk_with_stdin, require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, SandboxBuilder, load_snapshot, snapshot_scratch_mb};

#[tokio::test]
async fn bash_inline_code() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("echo 'hluk-bash-ok'").await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("hluk-bash-ok"),
        "expected guest to print 'hluk-bash-ok', got: {output:?}",
    );
}

#[tokio::test]
async fn bash_exec_file() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/hello.sh");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.sh to produce output containing 'Hello', got: {output:?}",
    );
}

#[tokio::test]
async fn bash_snapshot_round_trip() {
    let rootfs = require_rootfs("bash");
    let snap_dir = temp_dir("bash-snap");

    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();
    assert_eq!(
        snapshot_scratch_mb(&load_snapshot(&snap_dir).unwrap()),
        Some(256),
        "the snapshot records the guest's memory, through the save and the load"
    );

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.run("echo 'restored-bash-ok'").await.unwrap();
    let output = sandbox.drain_output();
    assert_eq!(
        snapshot_scratch_mb(&sandbox.snapshot().unwrap()),
        Some(256),
        "a guest restored from the snapshot knows its size from it"
    );
    assert!(
        output.contains("restored-bash-ok"),
        "expected restored guest to print 'restored-bash-ok', got: {output:?}",
    );
}

#[tokio::test]
async fn bash_multiple_runs() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run("x=42").await.unwrap();
    sandbox.run("echo \"x=$x\"").await.unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("x=42"),
        "expected 'x=42' after multiple runs, got: {output:?}",
    );
}

#[tokio::test]
async fn bash_coreutils() {
    let rootfs = require_rootfs("bash");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/bash/coreutils.sh");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).await.unwrap();
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

#[tokio::test]
async fn bash_env_vars() {
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
        .await
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
#[tokio::test]
async fn bash_pipes_and_substitutions() {
    let rootfs = require_rootfs("bash");
    // Its kernel's messages are checked: at a level RUST_LOG cannot change.
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .guest_log_level(tracing::level_filters::LevelFilter::ERROR)
        .boot()
        .unwrap();
    sandbox.run("echo hello | tr a-z A-Z").await.unwrap();
    assert_eq!(sandbox.drain_output(), "HELLO\n");
    sandbox.run(r#"x=$(echo hi); echo "got $x""#).await.unwrap();
    assert_eq!(sandbox.drain_output(), "got hi\n");
    sandbox
        .run("/bin/echo to-file > /tmp/f; cat /tmp/f")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "to-file\n");
    sandbox
        .run(r#"printf 'b\na\nb\n' | sort | uniq -c | wc -l"#)
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output().trim(), "2");
    // An exec'd program inherits fd 2 as the shell's own stderr file.
    sandbox.run("ls /nope; true").await.unwrap();
    assert!(sandbox.drain_stderr().contains("No such file"));
    sandbox.drain_output();
    // fd 2 is a file of its own: redirects still move it.
    sandbox.run("ls /nope 2>&1 | wc -l").await.unwrap();
    assert_eq!(sandbox.drain_output().trim(), "1");
    sandbox
        .run("ls /nope 2>/dev/null; echo rc=$?")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_stderr(), "");
    assert_eq!(sandbox.drain_output(), "rc=1\n");
    sandbox.run("echo to-err >&2").await.unwrap();
    assert_eq!(sandbox.drain_stderr(), "to-err\n");
    // A command that fails to execute: its vfork child exits without an
    // execve, and must not free the shell's stack on its way out.
    let (run, log) = guest_log(sandbox.run("nope; sleep 0.1; echo after")).await;
    run.unwrap();
    // The shell says why on stderr.  The kernel's messages about it are
    // in Hyperlight's guest log, with where they come from, not in the
    // program's output.
    let stderr = sandbox.drain_stderr();
    assert!(
        stderr.contains("can't execute 'nope'") && !stderr.contains("ERR:"),
        "{stderr:?}"
    );
    assert!(
        log.iter().any(|l| l.starts_with("[libposix_process] ")
            && l.contains("/bin/nope: Unable to load (-2)")
            && l.contains("(execve.c:")),
        "{log:?}"
    );
    assert!(sandbox.drain_output().ends_with("after\n"));
    // Its kernel message outgrows the kernel's 191-byte line: with an
    // odd-length prefix the byte limit falls inside an 'é', and the cut
    // must back off to a whole character (else its first byte would be
    // sent as '?').
    let (run, log) = guest_log(sandbox.run(
        "p=/xy; i=0; while [ $i -lt 100 ]; do p=${p}\u{e9}; i=$((i+1)); done; $p; echo after",
    ))
    .await;
    run.unwrap();
    assert!(
        log.iter()
            .any(|l| l.contains("/xy\u{e9}\u{e9}") && l.contains("\u{e9} (") && !l.contains('?')),
        "{log:?}"
    );
    assert!(sandbox.drain_output().ends_with("after\n"));
    // Bytes that are not UTF-8 arrive as '?', not as a message the host
    // refuses.
    let (run, log) = guest_log(sandbox.run("p=$(printf '/x\\377y'); $p; echo after")).await;
    run.unwrap();
    assert!(log.iter().any(|l| l.contains("/x?y")), "{log:?}");
    assert!(sandbox.drain_output().ends_with("after\n"));
}

/// The guest log level is the host's to choose: off, the kernel sends
/// none of its messages.
#[tokio::test]
async fn bash_guest_log_level_off_sends_nothing() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .guest_log_level(tracing::level_filters::LevelFilter::OFF)
        .boot()
        .unwrap();
    let (run, log) = guest_log(sandbox.run("nope; echo after")).await;
    run.unwrap();
    assert_eq!(log, Vec::<String>::new());
    assert!(!sandbox.drain_stderr().contains("ERR:"));
}

/// `exit` ends the shell, which is the runtime: the call ends with that
/// status, and the next call starts a fresh shell.
#[tokio::test]
async fn bash_exit_ends_the_call() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("echo leaving; exit 0; echo not reached")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\n");
    assert!(
        sandbox.run("exit 3").await.is_err(),
        "exit 3 did not fail the call"
    );
    sandbox.run("echo still here").await.unwrap();
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
#[tokio::test]
async fn bash_refuses_a_call() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs).boot().unwrap();
    let err = sandbox.call("echo ran-as-code", "{}").await.unwrap_err();
    assert!(
        matches!(err, hyperlight_unikraft::Error::CallFailed { .. }),
        "{err}"
    );
    let output = sandbox.drain_output();
    assert!(!output.contains("ran-as-code"), "{output}");
    assert!(output.contains("does not serve Call calls"), "{output}");
    // The driver serves the next call as usual.
    sandbox.run("echo still-here").await.unwrap();
}

/// Assigning RANDOM seeds $RANDOM, as in bash: a seed repeats its
/// sequence, and RANDOM stays the generator's, not a plain variable,
/// until export makes it one.  A subshell seeds its own from the kernel.
#[tokio::test]
async fn bash_random_assignment_seeds() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("bash"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run(
            "RANDOM=42; a=\"$RANDOM $RANDOM\"; RANDOM=42; b=\"$RANDOM $RANDOM\"\n\
             [ \"$a\" = \"$b\" ] && echo repeats\n\
             [ \"$RANDOM$RANDOM$RANDOM\" != 424242 ] && echo generated\n\
             a=$(echo $RANDOM$RANDOM); b=$(echo $RANDOM$RANDOM); [ \"$a\" != \"$b\" ] && echo subshells\n\
             f() { local RANDOM=5; echo \"local $(echo $RANDOM)\"; }; f\n\
             export RANDOM=7; RANDOM=8; [ \"$RANDOM$RANDOM\" = 88 ] && echo variable",
        ).await
        .unwrap();
    assert_eq!(
        sandbox.drain_output(),
        "repeats\ngenerated\nsubshells\nlocal 5\nvariable\n"
    );
}

/// An empty argument or environment string reaches a program, as on
/// Linux: hush passes them when it re-executes itself for a pipeline.
#[tokio::test]
async fn bash_empty_strings_reach_programs() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("bash"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("sh -c 'echo \"[$1]\"' sh ''\nX=1; set | grep -q '^X=' && echo piped")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "[]\npiped\n");
}

/// Commands run back to back, with no pause for the guest to idle in, give
/// their memory back as they end: a loop of thousands fits in a small guest.
#[tokio::test]
async fn bash_commands_back_to_back_give_their_memory_back() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("bash"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("for i in $(seq 3000); do /bin/true || echo \"failed at $i\"; done; echo loop-done")
        .await
        .unwrap();
    assert_eq!(sandbox.drain_output(), "loop-done\n");
}

/// A file written in small pieces is copied a bounded number of times
/// over, not once per piece: the kernel grows a file by half at least.
/// Grown to the exact size each time, 50 MB in 4 KiB pieces took minutes,
/// and the guest ran nothing else meanwhile.
#[tokio::test]
async fn bash_writing_a_file_in_pieces_is_not_quadratic() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let started = Instant::now();
    sandbox
        .run("yes 2>/dev/null | head -c 50000000 > /tmp/pieces")
        .await
        .unwrap();
    let took = started.elapsed();
    sandbox
        .run("wc -c < /tmp/pieces; md5sum /tmp/pieces")
        .await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("50000000\n") && output.contains("c4915951b5afe50190151c7cfeb3989b"),
        "expected the file whole, got: {output:?}"
    );
    // About half a second here; 163 s when every piece copied the file. The bound
    // leaves room for a slow runner without reaching the quadratic case.
    assert!(
        took < Duration::from_secs(60),
        "50 MB in pieces took {took:?}: the file is being copied for every piece"
    );
}

/// A file appended to between closes reads back whole: the room a write
/// leaves past the file's end stays the file's, and the next open's append
/// grows into it.
#[tokio::test]
async fn bash_appends_between_closes_read_back_whole() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("for i in $(seq 1 20000); do echo \"line $i of the log\" >> /tmp/log; done; wc -l < /tmp/log; md5sum /tmp/log").await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("20000\n") && output.contains("c6630dd553242c57170979d6d113445a"),
        "expected the log whole, got: {output:?}"
    );
}

/// What a truncate cut off does not come back when the file grows again
/// into the room it had, by a write or by a truncate: the gap reads as zeros.
#[tokio::test]
async fn bash_a_file_grown_again_after_a_cut_reads_zeros_in_the_gap() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("printf aaaaaaaaaa > /tmp/t; truncate -s 5 /tmp/t; printf bbb >> /tmp/t; truncate -s 12 /tmp/t; printf ccc >> /tmp/t; od -An -c /tmp/t | tr -s ' '").await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("a a a a a b b b \\0 \\0 \\0 \\0 c c c"),
        "expected aaaaa bbb, four zeros, ccc, got: {output:?}"
    );
}

/// A file moved to another directory keeps growing as before: the record of
/// how much of its buffer has memory behind it moves with the buffer.
#[tokio::test]
async fn bash_a_file_moved_across_directories_grows_on() {
    let rootfs = require_rootfs("bash");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox
        .run("mkdir -p /var/moved; printf x > /tmp/a; mv /tmp/a /var/moved/a; printf y >> /var/moved/a; yes 2>/dev/null | head -c 30000 >> /var/moved/a; mv /var/moved/a /tmp/b; printf z >> /tmp/b; head -c 2 /tmp/b; echo; wc -c < /tmp/b; tail -c 1 /tmp/b; echo").await
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("xy\n") && output.contains("30003\n") && output.ends_with("z\n"),
        "expected the moved file whole and grown, got: {output:?}"
    );
}
