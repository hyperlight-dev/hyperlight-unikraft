// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;

use common::{require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, SandboxBuilder};

#[test]
fn powershell_hello() {
    let rootfs = require_rootfs("powershell");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/powershell/hello.ps1");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1024)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello from PowerShell on Hyperlight"),
        "expected PowerShell hello output, got: {output:?}",
    );
}

#[test]
fn powershell_snapshot_round_trip() {
    let rootfs = require_rootfs("powershell");
    let snap_dir = temp_dir("powershell-snap");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs.clone())
        .scratch_mb(1024)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox
        .run("[Console]::WriteLine('restored-pwsh-ok')")
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("restored-pwsh-ok"),
        "expected restored PowerShell output, got: {output:?}",
    );
}

#[test]
fn powershell_env_vars() {
    let rootfs = require_rootfs("powershell");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1024)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox
        .run(Exec::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/powershell/env_vars.ps1"),
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

/// A script's exit code is the call's status, as it is for an exec'd
/// program: `exit 3` fails the call, and the next call runs as usual.
#[test]
fn powershell_exit_status_fails_the_call() {
    let rootfs = require_rootfs("powershell");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(1024)
        .boot()
        .unwrap();
    assert!(
        sandbox
            .run("[Console]::WriteLine('leaving'); exit 3")
            .is_err(),
        "a non-zero exit code did not fail the call"
    );
    assert!(sandbox.drain_output().contains("leaving"));
    sandbox.run("[Console]::WriteLine('still here')").unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}
