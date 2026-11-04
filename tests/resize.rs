// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Snapshots that restore with a different scratch size than the guest
//! booted with.

mod common;

use common::{require_rootfs, temp_dir};
use hyperlight_host::HyperlightError::{
    GuestAborted, MemoryRequestTooBig, MemoryRequestTooSmall, PoisonedSandbox,
};
use hyperlight_unikraft::{Error, SandboxBuilder, save_snapshot, snapshot_scratch_mb};

const SCRIPT: &str = "def handler(request):
    request['uri'] = '/redirected.html'
    return request

def big(request):
    b = bytearray(int(request['mb']) << 20)
    for i in range(0, len(b), 4096):
        b[i] = 1
    return {'len': len(b)}
";

/// A guest function call that outgrew the scratch either has its process
/// killed, when the kernel's memory budget runs out first, or aborts the
/// guest, when scratch itself does.
fn outgrew(e: &Error) -> bool {
    matches!(
        e,
        Error::GuestExited { status: 137 } | Error::Hyperlight(GuestAborted(..))
    )
}

const REQUEST: &str = r#"{"uri":"/index.html"}"#;
const BIG: &str = r#"{"mb":32}"#;
// More than the 256 MiB the sandbox boots with.
const HUGE: &str = r#"{"mb":320}"#;

/// The sandbox that takes a resized snapshot continues on its size: its
/// guest function calls get that scratch, and its later plain snapshots
/// record it.
#[test]
fn the_sandbox_continues_on_its_resized_snapshots_scratch() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(SCRIPT).unwrap();

    let small = sandbox.snapshot_with_scratch_mb(8).unwrap();
    assert_eq!(snapshot_scratch_mb(&small), Some(8));
    let plain = sandbox.snapshot().unwrap();
    assert_eq!(snapshot_scratch_mb(&plain), Some(8));
    let e = sandbox
        .call("big", BIG)
        .expect_err("32 MiB must not fit 8 MiB");
    assert!(outgrew(&e), "{e}");
    // A plain snapshot of it is an 8 MiB snapshot, in a new sandbox too.
    let mut fresh = SandboxBuilder::from_snapshot(plain).boot().unwrap();
    let e = fresh
        .call("big", BIG)
        .expect_err("32 MiB must not fit a fresh 8 MiB sandbox");
    assert!(outgrew(&e), "{e}");

    // Growing it gives this sandbox the room, with no restore.
    sandbox.restore(small).unwrap();
    let grown = sandbox.snapshot_with_scratch_mb(512).unwrap();
    assert_eq!(snapshot_scratch_mb(&grown), Some(512));
    let out = sandbox.call("big", HUGE).unwrap();
    assert!(out.contains("335544320"), "{out}");
    assert_eq!(snapshot_scratch_mb(&sandbox.snapshot().unwrap()), Some(512));
}

#[test]
fn a_resized_snapshot_restores_with_another_scratch_size() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(SCRIPT).unwrap();
    let full = sandbox.snapshot().unwrap();
    assert_eq!(snapshot_scratch_mb(&full), Some(256));
    let same = sandbox.snapshot_with_scratch_mb(256).unwrap();
    assert_eq!(snapshot_scratch_mb(&same), Some(256));
    let small = sandbox.snapshot_with_scratch_mb(8).unwrap();
    assert_eq!(snapshot_scratch_mb(&small), Some(8));
    let grown = sandbox.snapshot_with_scratch_mb(512).unwrap();
    assert_eq!(snapshot_scratch_mb(&grown), Some(512));

    // Sizes it cannot use are rejected before the guest is entered, so
    // the sandbox runs a guest function call right after, with no restore.
    let e = sandbox
        .snapshot_with_scratch_mb(0)
        .err()
        .expect("0 MiB must be rejected");
    assert!(
        matches!(e, Error::Hyperlight(MemoryRequestTooSmall(..))),
        "{e}"
    );
    // Saturates to usize::MAX bytes, above the maximum memory size.
    let e = sandbox
        .snapshot_with_scratch_mb(usize::MAX)
        .err()
        .expect("overflow must be rejected");
    assert!(
        matches!(e, Error::Hyperlight(MemoryRequestTooBig(..))),
        "{e}"
    );
    let out = sandbox.call("handler", REQUEST).unwrap();
    assert!(out.contains("redirected"), "{out}");

    for _ in 0..3 {
        sandbox.restore(small.clone()).unwrap();
        let out = sandbox.call("handler", REQUEST).unwrap();
        assert!(out.contains("redirected"), "{out}");
    }

    // 32 MiB does not fit in 8 MiB of scratch, so the resize took effect,
    // and a restore recovers the sandbox.
    sandbox.restore(small.clone()).unwrap();
    let e = sandbox
        .call("big", BIG)
        .expect_err("32 MiB must not fit 8 MiB");
    assert!(outgrew(&e), "{e}");

    // Restored with 8 MiB, the sandbox can still take a larger snapshot.
    sandbox.restore(small.clone()).unwrap();
    let regrown = sandbox.snapshot_with_scratch_mb(64).unwrap();
    assert_eq!(snapshot_scratch_mb(&regrown), Some(64));
    sandbox.restore(regrown).unwrap();
    let out = sandbox.call("big", BIG).unwrap();
    assert!(out.contains("33554432"), "{out}");

    // 320 MiB does not fit the 256 MiB the sandbox booted with, but fits
    // the grown snapshot.
    sandbox.restore(full.clone()).unwrap();
    let e = sandbox
        .call("big", HUGE)
        .expect_err("320 MiB must not fit 256 MiB");
    assert!(outgrew(&e), "{e}");
    sandbox.restore(grown).unwrap();
    let out = sandbox.call("big", HUGE).unwrap();
    assert!(out.contains("335544320"), "{out}");

    // The smallest whole MiB the host accepts is less than a MiB above its
    // minimum (transport and page tables), while the kernel's frame
    // allocator needs two 512 KiB chunks on top of that, so the sandbox
    // that took it aborts on its next entry, the restore fails, as does a
    // boot from it, and restoring a larger snapshot recovers the sandbox.
    sandbox.restore(full.clone()).unwrap();
    let tiny = (1..8)
        .find_map(|mb| match sandbox.snapshot_with_scratch_mb(mb) {
            Ok(snapshot) => Some(snapshot),
            Err(Error::Hyperlight(MemoryRequestTooSmall(..))) => None,
            Err(e) => panic!("{mb} MiB: {e}"),
        })
        .expect("a size the host accepts");
    let e = sandbox
        .call("handler", REQUEST)
        .expect_err("a guest function call on the tiny scratch must fail");
    assert!(matches!(e, Error::Hyperlight(GuestAborted(..))), "{e}");
    let e = sandbox
        .snapshot()
        .err()
        .expect("a snapshot after the abort must fail");
    assert!(matches!(e, Error::Hyperlight(PoisonedSandbox)), "{e}");
    let e = sandbox
        .restore(tiny.clone())
        .expect_err("the tiny restore must fail");
    assert!(matches!(e, Error::Hyperlight(GuestAborted(..))), "{e}");
    // Until a restore recovers it, the sandbox serves nothing: hluk has
    // forgotten the driver, and hyperlight has poisoned the sandbox.
    let e = sandbox
        .call("handler", REQUEST)
        .expect_err("a guest function call after a failed restore must fail");
    assert!(matches!(e, Error::NoDriver), "{e}");
    let e = sandbox
        .snapshot()
        .err()
        .expect("a snapshot after a failed restore must fail");
    assert!(matches!(e, Error::Hyperlight(PoisonedSandbox)), "{e}");
    let e = SandboxBuilder::from_snapshot(tiny)
        .boot()
        .err()
        .expect("the tiny boot must fail");
    assert!(matches!(e, Error::Hyperlight(GuestAborted(..))), "{e}");
    sandbox.restore(full).unwrap();
    let out = sandbox.call("handler", REQUEST).unwrap();
    assert!(out.contains("redirected"), "{out}");

    // A saved resized snapshot keeps its size and its record, restored in
    // place and in a new sandbox.
    let dir = temp_dir("resize");
    save_snapshot(&small, dir.path().join("small")).unwrap();
    sandbox.restore_from(dir.path().join("small")).unwrap();
    let out = sandbox.call("handler", REQUEST).unwrap();
    assert!(out.contains("redirected"), "{out}");
    let e = sandbox
        .call("big", BIG)
        .expect_err("32 MiB must not fit 8 MiB");
    assert!(outgrew(&e), "{e}");

    let mut fresh = SandboxBuilder::from_snapshot_dir(dir.path().join("small"))
        .unwrap()
        .boot()
        .unwrap();
    let out = fresh.call("handler", REQUEST).unwrap();
    assert!(out.contains("redirected"), "{out}");
    // A plain snapshot of the restored sandbox records the size it took.
    assert_eq!(snapshot_scratch_mb(&fresh.snapshot().unwrap()), Some(8));
    let e = fresh
        .call("big", BIG)
        .expect_err("32 MiB must not fit a fresh 8 MiB sandbox");
    assert!(outgrew(&e), "{e}");
}
