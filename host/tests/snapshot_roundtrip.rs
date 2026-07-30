//! Snapshot-to-disk roundtrip — the core value prop of `pyhl` and anyone
//! else using `Sandbox::save_snapshot` + `Sandbox::from_snapshot_file`.
//!
//! This was missing coverage until #1 surfaced a Windows-only regression:
//! we had no automated way to verify that post-warmup state survives a
//! save / load / restore cycle and that hermetic rewinds behave.
//!
//! All tests here boot a real Unikraft kernel under Hyperlight, so they
//! need a hypervisor (`/dev/kvm` on Linux, WHP on Windows). They check
//! [`hypervisor_available`] up front and self-skip with a note when
//! absent — lets `cargo test` still pass on runners without a hv.
//!
//! The multifn-c guest is ideal because it:
//!   - has two dispatchable entrypoints (`init`, `run`) that exercise
//!     `call_named` plumbing,
//!   - preserves state via `.data` across snapshot/restore so we can
//!     verify hermeticity.
//!
//! Artifacts live in `examples/multifn-c/`: the kernel at
//! `.unikraft/build/*_hyperlight-x86_64` and initrd as `*-initrd.cpio`.
//! If they're not built yet, the tests self-skip — `just rootfs` and
//! `kraft-hyperlight build` in that dir to populate them.

use hyperlight_unikraft::Sandbox;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Environment probe
// ---------------------------------------------------------------------------

fn hypervisor_available() -> bool {
    #[cfg(unix)]
    {
        std::fs::metadata("/dev/kvm")
            .map(|_| {
                std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/kvm")
                    .is_ok()
            })
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        // Hyperlight probes WHP at runtime; the cheap way from a test is
        // to try and skip gracefully if construction fails. We optimistically
        // return true and let the build() error in that case.
        true
    }
}

fn multifn_artifacts() -> Option<(PathBuf, PathBuf)> {
    let example_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/multifn-c");
    let build = example_dir.join(".unikraft/build");
    if !build.is_dir() {
        return None;
    }
    let kernel = std::fs::read_dir(&build)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("_hyperlight-x86_64") && !n.ends_with(".dbg"))
                .unwrap_or(false)
        })?;
    let initrd = std::fs::read_dir(&example_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("-initrd.cpio"))
                .unwrap_or(false)
        })?;
    Some((kernel, initrd))
}

/// Guard that skips the test body with a diagnostic if prerequisites
/// aren't met. Returns None → test body should early-return.
fn setup() -> Option<(PathBuf, PathBuf)> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available (no /dev/kvm)");
        return None;
    }
    let Some((kernel, initrd)) = multifn_artifacts() else {
        eprintln!(
            "SKIP: multifn-c artifacts missing under examples/multifn-c/.unikraft/build/ \
             — run `just rootfs && kraft-hyperlight build --plat hyperlight --arch x86_64` \
             in that directory to populate them"
        );
        return None;
    };
    Some((kernel, initrd))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Baseline: can we build a sandbox, call into the guest twice (with a
/// restore between), and get clean results? Without this, every snapshot
/// test is moot.
#[test]
fn multifn_init_then_restore_and_run() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .build()
        .expect("build sandbox");

    sbox.restore().expect("restore 1");
    let _: () = sbox.call_named("init", ()).expect("init");

    sbox.snapshot_now().expect("snapshot post-init");

    sbox.restore().expect("restore 2");
    let _: () = sbox
        .call_named("run", "alpha".to_string())
        .expect("run alpha");

    sbox.restore().expect("restore 3");
    let _: () = sbox
        .call_named("run", "beta".to_string())
        .expect("run beta");
}

/// The real roundtrip: warm up the sandbox, persist a snapshot to disk,
/// load it back from file into a *fresh* Sandbox, and verify we can
/// still call into the guest with the recovered state.
#[test]
fn snapshot_save_then_load_roundtrip() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let tmp = tempdir_local("hl-snap");
    // save_snapshot writes an OCI image layout *directory*, not a single
    // file (changed in 1caba5c when we moved to hyperlight-host 0.16).
    let snap_path = tmp.join("roundtrip-snapshot");

    {
        let mut sbox = Sandbox::builder(&kernel)
            .initrd_file(&initrd)
            .heap_size(32 * 1024 * 1024)
            .build()
            .expect("build sandbox");

        sbox.restore().expect("restore");
        let _: () = sbox.call_named("init", ()).expect("init");

        sbox.snapshot_now().expect("snapshot_now");
        sbox.save_snapshot(&snap_path).expect("save_snapshot");
    }
    assert!(
        snap_path.is_dir(),
        "save_snapshot should have created the OCI layout dir {}",
        snap_path.display()
    );

    // Fresh process-space sandbox, loaded from the file on disk.
    let mut sbox = Sandbox::from_snapshot_file(&snap_path).expect("from_snapshot_file");

    // Should be immediately callable without re-doing evolve+init.
    let _: () = sbox
        .call_named("run", "post-load".to_string())
        .expect("run after load");

    // Restore + call again to exercise the hermetic rewind path.
    sbox.restore().expect("restore after load");
    let _: () = sbox
        .call_named("run", "post-restore".to_string())
        .expect("run after restore");

    cleanup_tempdir(&tmp);
}

/// Hermeticity: multiple restore+run calls should each see the same
/// post-init state. We don't have a direct observable for this from the
/// test side (multifn-c's state is internal), but every cycle should
/// succeed — if restore had state leakage it would eventually diverge
/// and fail.
#[test]
fn repeated_restore_run_is_stable() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .build()
        .expect("build sandbox");
    sbox.restore().expect("restore");
    let _: () = sbox.call_named("init", ()).expect("init");
    sbox.snapshot_now().expect("snapshot_now");

    for i in 0..10 {
        sbox.restore().expect("restore in loop");
        let _: () = sbox
            .call_named("run", format!("iter-{i}"))
            .expect("run in loop");
    }
}

// ---------------------------------------------------------------------------
// Small tempdir helper — we avoid pulling in `tempfile` for one test.
// ---------------------------------------------------------------------------

fn tempdir_local(prefix: &str) -> PathBuf {
    let base = std::env::temp_dir();
    let pid = std::process::id();
    let uniq = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = base.join(format!("{prefix}-{pid}-{uniq}"));
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}

fn cleanup_tempdir(path: &Path) {
    let _ = std::fs::remove_dir_all(path);
}
