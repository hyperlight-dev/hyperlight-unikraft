// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use common::require_rootfs;
use hyperlight_unikraft::{Exec, SandboxBuilder};

#[test]
fn exec_file_not_found() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let result = sandbox.run(Exec::File("/nonexistent/script.py".into()));
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
        String::from_utf8_lossy(&output.stdout).contains("no global function handler"),
        "{output:?}"
    );
}
