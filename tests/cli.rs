// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use common::{require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, SandboxBuilder};

#[tokio::test]
async fn exec_file_not_found() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let result = sandbox
        .run(Exec::File("/nonexistent/script.py".into()))
        .await;
    assert!(result.is_err());
}

#[test]
fn boot_missing_initrd() {
    let result = SandboxBuilder::from_initrd("/nonexistent/rootfs.cpio").boot();
    assert!(result.is_err());
}

#[test]
fn cli_env_flag() {
    let rootfs = require_rootfs("python");
    let bin = env!("CARGO_BIN_EXE_hluk");
    let output = std::process::Command::new(bin)
        .args([
            "run",
            "--initrd", rootfs.to_str().unwrap(),
            "--scratch-mb", "256",
            "--env", "MY_VAR=cli_test",
            "--env", "NUM=42",
            "--exec", "import os; print(f\"MY_VAR={os.environ.get('MY_VAR','?')}\"); print(f\"NUM={os.environ.get('NUM','?')}\")",
        ])
        .output()
        .expect("failed to run hluk");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("MY_VAR=cli_test"),
        "expected MY_VAR=cli_test, got: {stdout:?}",
    );
    assert!(
        stdout.contains("NUM=42"),
        "expected NUM=42, got: {stdout:?}",
    );
}

/// `hluk run` writes the guest's standard error to its own stderr, not
/// into its stdout.
#[test]
fn cli_stderr_apart_from_stdout() {
    let rootfs = require_rootfs("python");
    let bin = env!("CARGO_BIN_EXE_hluk");
    let output = std::process::Command::new(bin)
        .args([
            "run",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--scratch-mb",
            "256",
            "--exec",
            "import sys; print('to stdout'); print('to stderr', file=sys.stderr)",
        ])
        .output()
        .expect("failed to run hluk");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "to stdout\n");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "to stderr\n");
}

/// `--call` runs the workload, then calls a function it defined and
/// prints the result; with no workload, only the call runs, and a failed
/// call fails the run.
#[test]
fn cli_call_flag() {
    let rootfs = require_rootfs("quickjs");
    let bin = env!("CARGO_BIN_EXE_hluk");
    let handler = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/quickjs/handler.js");
    let run = |args: &[&str]| {
        std::process::Command::new(bin)
            .args(["run", "--cold", "--initrd", rootfs.to_str().unwrap()])
            .args(args)
            .output()
            .expect("failed to run hluk")
    };
    let output = run(&[handler, "--call", "handler", "--input", r#"{"name":"cli"}"#]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"greeting\":\"Hello, cli!\",\"calls\":1}\n"
    );
    let output = run(&["--call", "handler"]);
    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("no global function handler"),
        "{output:?}"
    );
}

#[test]
fn cli_snapshot_and_benchmarks_drive_guest_timers() {
    let rootfs = require_rootfs("python");
    let root = temp_dir("cli-async-driving");
    let snapshot = root.path().join("snapshot");
    let script = root.path().join("timer.py");
    std::fs::write(&script, "import time; time.sleep(0.01)\n").unwrap();
    let run = |args: &[&str]| {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_hluk"))
            .args(args)
            .current_dir(root.path())
            .env("HLUK_CACHE_DIR", root.path().join("cache"))
            .output()
            .expect("failed to run hluk");
        assert!(output.status.success(), "{args:?}: {output:?}");
        String::from_utf8(output.stdout).unwrap()
    };
    run(&[
        "snapshot",
        "save",
        "--initrd",
        rootfs.to_str().unwrap(),
        "--scratch-mb",
        "256",
        "--warm-exec",
        "import time\ntime.sleep(0.01)\ndef twice(value):\n    time.sleep(0.01)\n    return value * 2",
        "--output",
        snapshot.to_str().unwrap(),
    ]);
    let output = run(&[
        "snapshot",
        "run",
        snapshot.to_str().unwrap(),
        "--exec",
        "time.sleep(0.01)",
        "--call",
        "twice",
        "--input",
        "21",
    ]);
    assert_eq!(output, "42\n");
    let output = run(&[
        "bench",
        "cold",
        "--initrd",
        rootfs.to_str().unwrap(),
        "--scratch-mb",
        "256",
        script.to_str().unwrap(),
        "--samples",
        "2",
    ]);
    assert!(output.contains("samples=2"), "{output}");
    for mode in ["cold-snap", "warm-restore", "warm-stateful"] {
        let output = run(&[
            "bench",
            mode,
            snapshot.to_str().unwrap(),
            script.to_str().unwrap(),
            "--samples",
            "2",
        ]);
        assert!(output.contains("samples=2"), "{mode}: {output}");
    }
    let output = run(&[
        "bench",
        "parallel",
        snapshot.to_str().unwrap(),
        script.to_str().unwrap(),
        "--vms",
        "2",
        "--iterations",
        "2",
    ]);
    assert!(output.contains("total_calls=4"), "{output}");
    assert!(output.contains("samples=4"), "{output}");
}

#[test]
fn cli_entry_waits_and_preserves_exit_status() {
    let rootfs = require_rootfs("python");
    let root = temp_dir("cli-async-join");
    std::fs::write(
        root.path().join("exit.py"),
        "import time; time.sleep(0.01); raise SystemExit(7)\n",
    )
    .unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hluk"))
        .args([
            "run",
            "--cold",
            "--initrd",
            rootfs.to_str().unwrap(),
            "--scratch-mb",
            "256",
            "--mount",
            &format!("{}:/mnt/app:ro", root.path().display()),
            "--entry",
            "/usr/local/bin/python3 /mnt/app/exit.py",
        ])
        .current_dir(root.path())
        .env("HLUK_CACHE_DIR", root.path().join("cache"))
        .output()
        .expect("failed to run hluk");
    assert_eq!(output.status.code(), Some(7), "{output:?}");
}

#[test]
fn cli_snapshots_capture_pending_network_readiness() {
    let rootfs = require_rootfs("python");
    let root = temp_dir("cli-network-snapshot");
    let snapshot = root.path().join("snapshot");
    let warm = "import socket; listener = socket.socket(); listener.bind(('127.0.0.1', 0)); listener.listen(1); print('warming')";
    let exec = "assert listener.getsockname()[1] > 0; listener.close(); print('ready')";
    let run = |args: &[&str]| {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_hluk"))
            .args(args)
            .current_dir(root.path())
            .env("HLUK_CACHE_DIR", root.path().join("cache"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{args:?}: {output:?}");
        String::from_utf8(output.stdout).unwrap()
    };
    run(&[
        "snapshot",
        "save",
        "--initrd",
        rootfs.to_str().unwrap(),
        "--scratch-mb",
        "256",
        "--net",
        "--warm-exec",
        warm,
        "--output",
        snapshot.to_str().unwrap(),
    ]);
    assert_eq!(
        run(&[
            "snapshot",
            "run",
            snapshot.to_str().unwrap(),
            "--net",
            "--exec",
            exec
        ]),
        "ready\n"
    );
    let args = [
        "run",
        "--warm",
        "--initrd",
        rootfs.to_str().unwrap(),
        "--scratch-mb",
        "256",
        "--net",
        "--warm-exec",
        warm,
        "--exec",
        exec,
    ];
    assert_eq!(run(&args), "warming\nready\n");
    assert_eq!(run(&args), "ready\n");
}
