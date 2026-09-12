// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use common::require_rootfs;
use hyperlight_unikraft::{Exec, SandboxBuilder, run};

#[test]
fn exec_file_not_found() {
    let rootfs = require_rootfs("python");
    let (mut sandbox, _cfg) = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    let result = run(&mut sandbox, Exec::File("/nonexistent/script.py".into()));
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
