// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! `AppSandbox::dirty_bytes`: what the guest wrote since the sandbox was
//! last restored or snapshotted.

mod common;

use std::sync::Arc;

use common::require_rootfs;
use hyperlight_unikraft::{AppSandbox, SandboxBuilder, Snapshot};

const SCRIPT: &str = "def handler(request):
    return {'ok': True}

def write(request):
    b = bytearray(int(request['mb']) << 20)
    for i in range(0, len(b), 4096):
        b[i] = 1
    return {'len': len(b)}
";

const MIB: u64 = 1 << 20;

/// What a small call may write, the interpreter's own bookkeeping and the
/// kernel's frame-allocator chunks included (about 0.5 MiB on x86_64 KVM).
const SMALL: u64 = 4 * MIB;

async fn write_mb(sandbox: &mut AppSandbox, mb: u64) {
    let out = sandbox
        .call("write", &format!(r#"{{"mb":{mb}}}"#))
        .await
        .unwrap();
    assert!(out.contains(&(mb * MIB).to_string()), "{out}");
}

fn dirty(sandbox: &AppSandbox) -> u64 {
    sandbox.dirty_bytes().unwrap()
}

/// A python sandbox with [`SCRIPT`] loaded, and its snapshot.
async fn warm_snapshot() -> (AppSandbox, Arc<Snapshot>) {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(SCRIPT).await.unwrap();
    let warm = sandbox.snapshot().unwrap();
    (sandbox, warm)
}

#[tokio::test]
async fn dirty_bytes_counts_what_the_guest_wrote_since_the_restore() {
    let (mut sandbox, warm) = warm_snapshot().await;
    assert_eq!(dirty(&sandbox), 0, "right after the snapshot");

    // The resume entry's own writes are not the guest's.
    sandbox.restore(warm.clone()).unwrap();
    assert_eq!(dirty(&sandbox), 0, "right after the restore");
    sandbox.call("handler", "{}").await.unwrap();
    let small = dirty(&sandbox);
    assert!(small < SMALL, "a small call: {small} bytes counted");

    write_mb(&mut sandbox, 32).await;
    let written = dirty(&sandbox);
    assert!(
        (32 * MIB..32 * MIB + SMALL).contains(&written),
        "32 MiB written, {written} bytes counted"
    );

    // A second restore starts the count again.
    sandbox.restore(warm.clone()).unwrap();
    assert_eq!(dirty(&sandbox), 0, "right after the second restore");
    sandbox.call("handler", "{}").await.unwrap();
    let again = dirty(&sandbox);
    assert!(again < SMALL, "a small call: {again} bytes counted");

    // So does a snapshot.
    write_mb(&mut sandbox, 16).await;
    assert!(dirty(&sandbox) >= 16 * MIB);
    sandbox.snapshot().unwrap();
    assert_eq!(dirty(&sandbox), 0, "right after the second snapshot");
}

#[tokio::test]
async fn sandboxes_from_one_snapshot_count_apart() {
    let (sandbox, warm) = warm_snapshot().await;
    drop(sandbox);

    let mut a = SandboxBuilder::from_snapshot(warm.clone()).boot().unwrap();
    let mut b = SandboxBuilder::from_snapshot(warm).boot().unwrap();
    // A boot from a snapshot is a restore: its resume entry is not counted.
    assert_eq!(dirty(&a), 0);
    assert_eq!(dirty(&b), 0);

    write_mb(&mut a, 32).await;
    assert_eq!(dirty(&b), 0, "b has not run");
    write_mb(&mut b, 8).await;

    let (a, b) = (dirty(&a), dirty(&b));
    assert!(
        (32 * MIB..32 * MIB + SMALL).contains(&a),
        "a wrote 32 MiB: {a}"
    );
    assert!(
        (8 * MIB..8 * MIB + SMALL).contains(&b),
        "b wrote 8 MiB: {b}"
    );
}
