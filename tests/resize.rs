// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Snapshots that restore with less scratch than the guest booted with.

mod common;

use common::{require_rootfs, temp_dir};
use hyperlight_host::HyperlightError::{
    GuestAborted, MemoryRequestTooBig, MemoryRequestTooSmall, PoisonedSandbox,
};
use hyperlight_unikraft::{Error, SandboxBuilder, save_snapshot};

const SCRIPT: &str = "def handler(request):
    request['uri'] = '/redirected.html'
    return request

def big(request):
    b = bytearray(int(request['mb']) << 20)
    for i in range(0, len(b), 4096):
        b[i] = 1
    return {'len': len(b)}
";

const REQUEST: &str = r#"{"uri":"/index.html"}"#;
const BIG: &str = r#"{"mb":32}"#;

#[test]
fn a_resized_snapshot_restores_with_less_scratch() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(SCRIPT).unwrap();
    let full = sandbox.snapshot().unwrap();
    // The sandbox's own size is the plain snapshot, already taken.
    let same = sandbox.snapshot_with_scratch_mb(256).unwrap();
    assert!(std::sync::Arc::ptr_eq(&same, &full));
    let small = sandbox.snapshot_with_scratch_mb(8).unwrap();

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
    let e = sandbox
        .snapshot_with_scratch_mb(512)
        .err()
        .expect("growing must be rejected");
    assert!(
        matches!(e, Error::Hyperlight(MemoryRequestTooBig(..))),
        "{e}"
    );
    // Saturates to usize::MAX bytes.
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
    // Restored with 8 MiB, the sandbox cannot take a larger snapshot.
    let e = sandbox
        .snapshot_with_scratch_mb(16)
        .err()
        .expect("growing past the current scratch must be rejected");
    assert!(
        matches!(e, Error::Hyperlight(MemoryRequestTooBig(..))),
        "{e}"
    );

    // 32 MiB does not fit in 8 MiB of scratch, so the resize took effect:
    // the kernel kills the guest process (SIGKILL), and a restore recovers
    // the sandbox.
    sandbox.restore(small.clone()).unwrap();
    let e = sandbox
        .call("big", BIG)
        .expect_err("32 MiB must not fit 8 MiB");
    assert!(matches!(e, Error::GuestExited { status: 137 }), "{e}");

    sandbox.restore(full.clone()).unwrap();
    let out = sandbox.call("big", BIG).unwrap();
    assert!(out.contains("33554432"), "{out}");

    // The smallest whole MiB the host accepts is less than a MiB above its
    // minimum (transport and page tables), while the kernel's frame
    // allocator needs two 512 KiB chunks on top of that, so the restore
    // fails, as does a boot from it, and restoring a larger snapshot
    // recovers the sandbox.
    let tiny = (1..8)
        .find_map(|mb| match sandbox.snapshot_with_scratch_mb(mb) {
            Ok(snapshot) => Some(snapshot),
            Err(Error::Hyperlight(MemoryRequestTooSmall(..))) => None,
            Err(e) => panic!("{mb} MiB: {e}"),
        })
        .expect("a size the host accepts");
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

    // A saved resized snapshot keeps its size, restored in place and in a
    // new sandbox.
    let dir = temp_dir("resize");
    save_snapshot(&small, dir.path().join("small")).unwrap();
    sandbox.restore_from(dir.path().join("small")).unwrap();
    let out = sandbox.call("handler", REQUEST).unwrap();
    assert!(out.contains("redirected"), "{out}");
    let e = sandbox
        .call("big", BIG)
        .expect_err("32 MiB must not fit 8 MiB");
    assert!(matches!(e, Error::GuestExited { status: 137 }), "{e}");

    let mut fresh = SandboxBuilder::from_snapshot_dir(dir.path().join("small"))
        .unwrap()
        .boot()
        .unwrap();
    let out = fresh.call("handler", REQUEST).unwrap();
    assert!(out.contains("redirected"), "{out}");
    let e = fresh
        .call("big", BIG)
        .expect_err("32 MiB must not fit a fresh 8 MiB sandbox");
    assert!(matches!(e, Error::GuestExited { status: 137 }), "{e}");
}
