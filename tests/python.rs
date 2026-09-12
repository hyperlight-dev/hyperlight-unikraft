// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::sync::Arc;

use common::{hluk_with_stdin, require_rootfs, snapshot_dir};
use hyperlight_unikraft::{
    Exec, Mount, NetworkPolicy, OciTag, SNAPSHOT_TAG, SandboxBuilder, Snapshot, run,
};

#[test]
fn python_inline_code() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, "print('hluk-test-ok')").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("hluk-test-ok"),
        "expected guest to print 'hluk-test-ok', got: {output:?}",
    );
}

#[test]
fn python_exec_file() {
    let rootfs = require_rootfs("python");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/hello.py");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.py to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn python_snapshot_round_trip() {
    let rootfs = require_rootfs("python");
    let snap_dir = snapshot_dir("py-snap");

    // Save
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let snap = sandbox.snapshot().unwrap();
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    snap.save(&snap_dir, &tag).unwrap();

    // Restore + run
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    let snap = Arc::new(Snapshot::load(&snap_dir, tag).unwrap());
    let (mut sandbox, cfg2) = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    run(&mut sandbox, "print('restored-ok')").unwrap();
    let output = cfg2.drain_output();
    assert!(
        output.contains("restored-ok"),
        "expected restored guest to print 'restored-ok', got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&snap_dir);
}

#[test]
fn python_multiple_runs() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    run(&mut sandbox, "x = 1 + 1").unwrap();
    run(&mut sandbox, "print(f'x={x}')").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("x=2"),
        "expected 'x=2' after multiple runs, got: {output:?}",
    );
    run(&mut sandbox, "import sys; print(sys.version)").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("3.12"),
        "expected sys.version to contain '3.12', got: {output:?}",
    );
}

#[test]
fn python_env_vars() {
    let rootfs = require_rootfs("python");
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
        Exec::File(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/env_vars.py")),
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
fn python_env_vars_inline() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    cfg.set_env_vars(&[("SECRET", "42")]).unwrap();
    run(&mut sandbox, "import os; print(os.environ['SECRET'])").unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("42"),
        "expected '42' from SECRET env var, got: {output:?}",
    );
}

#[test]
fn python_env_vars_snapshot_restore() {
    let rootfs = require_rootfs("python");
    let snap_dir = snapshot_dir("py-env-snap");

    // Save snapshot (no env vars set at save time)
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let snap = sandbox.snapshot().unwrap();
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    snap.save(&snap_dir, &tag).unwrap();

    // Restore + set env vars AFTER restore
    let tag: OciTag = SNAPSHOT_TAG.parse().unwrap();
    let snap = Arc::new(Snapshot::load(&snap_dir, tag).unwrap());
    let (mut sandbox, cfg2) = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    cfg2.set_env_vars(&[("RESTORED_VAR", "from_snapshot")])
        .unwrap();
    run(
        &mut sandbox,
        r#"
import os
v = os.environ.get('RESTORED_VAR', 'NOT_FOUND')
print(f'RESTORED_VAR={v}')
"#,
    )
    .unwrap();
    let output = cfg2.drain_output();
    eprintln!("snapshot env output: {output:?}");
    assert!(
        output.contains("RESTORED_VAR=from_snapshot"),
        "expected env var set after snapshot restore, got: {output:?}",
    );

    let _ = std::fs::remove_dir_all(&snap_dir);
}

/// Env vars are stateful across dispatches without restore.
#[test]
fn python_env_vars_stateful_across_dispatches() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    // 1. Run with no env vars set — STATEFUL_VAR should not exist.
    run(
        &mut sandbox,
        r#"
import os
v = os.environ.get('STATEFUL_VAR', 'NOT_FOUND')
print(f'step1: STATEFUL_VAR={v}')
"#,
    )
    .unwrap();
    let out1 = cfg.drain_output();
    assert!(
        out1.contains("step1: STATEFUL_VAR=NOT_FOUND"),
        "expected no STATEFUL_VAR before setting, got: {out1:?}",
    );

    // 2. Set env vars, then run — should see them.
    cfg.set_env_vars(&[("STATEFUL_VAR", "persisted")]).unwrap();
    run(
        &mut sandbox,
        r#"
import os
v = os.environ.get('STATEFUL_VAR', 'NOT_FOUND')
print(f'step2: STATEFUL_VAR={v}')
"#,
    )
    .unwrap();
    let out2 = cfg.drain_output();
    assert!(
        out2.contains("step2: STATEFUL_VAR=persisted"),
        "expected STATEFUL_VAR=persisted after setting, got: {out2:?}",
    );

    // 3. Run AGAIN without setting env vars or restoring.
    run(
        &mut sandbox,
        r#"
import os
v = os.environ.get('STATEFUL_VAR', 'NOT_FOUND')
print(f'step3: STATEFUL_VAR={v}')
"#,
    )
    .unwrap();
    let out3 = cfg.drain_output();
    assert!(
        out3.contains("step3: STATEFUL_VAR=persisted"),
        "expected env vars to persist across dispatches without restore, got: {out3:?}",
    );
}

#[test]
fn python_stdin_piped() {
    let rootfs = require_rootfs("python");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/stdin_echo.py");
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
        output.contains("echo: line two"),
        "expected second line, got: {output:?}"
    );
    assert!(
        output.contains("stdin-done"),
        "expected stdin-done marker, got: {output:?}"
    );
}

#[test]
fn python_stdin_inline_piped() {
    let rootfs = require_rootfs("python");
    use std::io::Write;
    use std::process::{Command, Stdio};

    let bin = env!("CARGO_BIN_EXE_hluk");
    let mut child = Command::new(bin)
        .args([
            "run",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--exec",
            "import sys; data = sys.stdin.read(); print(f'got: {data}')",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn hluk");

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(b"secret data").ok();
    }
    child.stdin.take();

    let output = child.wait_with_output().expect("hluk didn't finish");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("got: secret data"),
        "expected stdin data, got: {stdout:?}",
    );
}

#[test]
fn python_stdin_empty_piped() {
    let rootfs = require_rootfs("python");
    use std::process::{Command, Stdio};

    let bin = env!("CARGO_BIN_EXE_hluk");
    let mut child = Command::new(bin)
        .args([
            "run",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--exec",
            "import sys; data = sys.stdin.read(); print(f'len={len(data)}')",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn hluk");

    child.stdin.take();

    let output = child.wait_with_output().expect("hluk didn't finish");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("len=0"),
        "expected empty stdin (len=0), got: {stdout:?}",
    );
}

#[test]
fn python_fs_ops() {
    let rootfs = require_rootfs("python");
    let mount_dir = std::env::temp_dir().join(format!("hluk-fs-ops-{}", std::process::id()));
    std::fs::create_dir_all(&mount_dir).unwrap();

    let mounts = vec![Mount::rw(&mount_dir, "/mnt/host")];
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/fs_ops.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Cleanup done"),
        "expected fs_ops.py to print 'Cleanup done', got: {output:?}",
    );

    let sentinel = mount_dir.join("sentinel.txt");
    let content = std::fs::read_to_string(&sentinel)
        .expect("sentinel.txt should exist on the host after guest write");
    assert_eq!(content, "guest-was-here\n", "sentinel content mismatch");

    let entries: Vec<_> = std::fs::read_dir(&mount_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(
        entries,
        vec!["sentinel.txt"],
        "expected only sentinel.txt, found: {:?}",
        entries
    );

    let _ = std::fs::remove_dir_all(&mount_dir);
}

#[test]
fn python_fs_large_file() {
    let rootfs = require_rootfs("python");
    let mount_dir = std::env::temp_dir().join(format!("hluk-fs-large-{}", std::process::id()));
    std::fs::create_dir_all(&mount_dir).unwrap();

    let mounts = vec![Mount::rw(&mount_dir, "/mnt/host")];
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/fs_large.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("large file round-trip OK"),
        "expected fs_large.py to print 'large file round-trip OK', got: {output:?}",
    );

    let entries: Vec<_> = std::fs::read_dir(&mount_dir).unwrap().collect();
    assert!(
        entries.is_empty(),
        "fs_large.py should clean up, but {} entries remain",
        entries.len()
    );

    let _ = std::fs::remove_dir_all(&mount_dir);
}

#[test]
fn python_guest_fs_ops() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/guest_fs.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Guest filesystem tests passed"),
        "expected guest_fs.py to print 'Guest filesystem tests passed', got: {output:?}",
    );
}

#[test]
fn python_threading() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/threading_demo.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Threading demo passed"),
        "expected threading_demo.py to print 'Threading demo passed', got: {output:?}",
    );
}

#[test]
fn python_subprocess() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/subprocess_demo.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Subprocess demo passed"),
        "expected subprocess_demo.py to print 'Subprocess demo passed', got: {output:?}",
    );
}

#[test]
fn python_tcp_echo() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/tcp_echo.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("TCP echo test passed"),
        "expected tcp_echo.py to print 'TCP echo test passed', got: {output:?}",
    );
}

#[test]
fn python_tcp_bidir() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/tcp_bidir.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Bidirectional TCP test passed"),
        "expected tcp_bidir.py to print 'Bidirectional TCP test passed', got: {output:?}",
    );
}

#[test]
fn python_http_server_client() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/http_server_client.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("HTTP server+client test passed"),
        "expected http_server_client.py to print 'HTTP server+client test passed', got: {output:?}",
    );
}

#[test]
fn python_http_get() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/http_get.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Status: 200"),
        "expected http_get.py to get Status: 200, got: {output:?}",
    );
}

#[test]
fn python_threaded_select() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/python/threaded_select.py");
    run(&mut sandbox, Exec::File(script)).unwrap();
    let output = cfg.drain_output();
    assert!(
        output.contains("Threaded select() test passed"),
        "expected threaded_select.py to pass, got: {output:?}",
    );
}
