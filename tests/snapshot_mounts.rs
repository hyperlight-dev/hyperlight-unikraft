// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! A restored guest gets the mounts the restore names: on `resume` the
//! kernel makes its mount table match what the host serves.  A warm
//! snapshot saved without mounts serves any mount set, so an embedder
//! runs a script over host directories from the warm image.
mod common;

use std::path::Path;
use std::time::Duration;

use common::{require_rootfs, temp_dir};
use hyperlight_unikraft::{Mount, SandboxBuilder, Yield};

/// A python guest saved to `dir` with the given mounts (none for a warm image).
fn save_snapshot(dir: &Path, mounts: Vec<Mount>) {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .mounts(mounts)
        .boot()
        .unwrap();
    sandbox.snapshot_to(dir).unwrap();
}

#[test]
fn restore_adds_a_mount_the_snapshot_lacked() {
    let tmp = temp_dir("mount-added");
    let snap = tmp.path().join("snap");
    let share = tmp.path().join("share");
    std::fs::create_dir(&share).unwrap();
    std::fs::write(share.join("in.txt"), "from the host\n").unwrap();
    save_snapshot(&snap, Vec::new());

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .mount(Mount::rw(&share, "/mnt/share"))
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
with open('/mnt/share/in.txt') as f:
    print('read:', f.read().strip())
with open('/mnt/share/out.txt', 'w') as f:
    f.write('from the guest\n')
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("read: from the host"),
        "expected the mount added at restore to be readable, got: {output:?}",
    );
    assert_eq!(
        std::fs::read_to_string(share.join("out.txt")).unwrap(),
        "from the guest\n",
        "expected the guest's write to land in the host directory",
    );
}

#[test]
fn restore_adds_several_mounts_and_keeps_them_apart() {
    let tmp = temp_dir("mounts-added");
    let snap = tmp.path().join("snap");
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");
    std::fs::create_dir(&a).unwrap();
    std::fs::create_dir(&b).unwrap();
    std::fs::write(a.join("who"), "a").unwrap();
    std::fs::write(b.join("who"), "b").unwrap();
    save_snapshot(&snap, Vec::new());

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .mount(Mount::ro(&a, "/mnt/a"))
        .mount(Mount::rw(&b, "/mnt/b"))
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
print('a:', open('/mnt/a/who').read(), 'b:', open('/mnt/b/who').read())
try:
    open('/mnt/a/x', 'w')
    print('a: writable')
except OSError as e:
    print('a: errno', e.errno)
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("a: a b: b"),
        "expected each mount to show its own directory, got: {output:?}"
    );
    // EROFS: the kernel's own read-only flag, from the mount table it built on resume.
    assert!(
        output.contains("a: errno 30"),
        "expected the read-only mount to refuse a write, got: {output:?}"
    );
}

#[test]
fn restore_drops_a_mount_the_host_no_longer_serves() {
    let tmp = temp_dir("mount-dropped");
    let snap = tmp.path().join("snap");
    let share = tmp.path().join("share");
    std::fs::create_dir(&share).unwrap();
    std::fs::write(share.join("in.txt"), "x").unwrap();
    save_snapshot(&snap, vec![Mount::rw(&share, "/mnt/share")]);

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .boot()
        .unwrap();
    sandbox
        .run("import os; print('present:', os.path.exists('/mnt/share/in.txt'))")
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("present: False"),
        "expected the dropped mount's files to be gone, got: {output:?}",
    );
}

#[test]
fn restore_changes_a_mount_to_read_only() {
    let tmp = temp_dir("mount-ro");
    let snap = tmp.path().join("snap");
    let share = tmp.path().join("share");
    std::fs::create_dir(&share).unwrap();
    save_snapshot(&snap, vec![Mount::rw(&share, "/mnt/share")]);

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .mount(Mount::ro(&share, "/mnt/share"))
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
try:
    open('/mnt/share/out.txt', 'w').write('x')
    print('write: allowed')
except OSError as e:
    print('write: errno', e.errno)
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("write: errno 30"),
        "expected EROFS on the mount made read-only at restore, got: {output:?}",
    );
    assert!(
        !share.join("out.txt").exists(),
        "the write must not reach the host"
    );
}

#[test]
fn restore_serves_another_directory_behind_the_same_mount() {
    let tmp = temp_dir("mount-swapped");
    let snap = tmp.path().join("snap");
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");
    std::fs::create_dir(&a).unwrap();
    std::fs::create_dir(&b).unwrap();
    std::fs::write(a.join("who"), "a").unwrap();
    std::fs::write(b.join("who"), "b").unwrap();
    save_snapshot(&snap, vec![Mount::rw(&a, "/mnt/share")]);

    // The guest path and index are unchanged, so the kernel keeps its
    // mount; only the directory the host serves behind it differs.
    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .mount(Mount::rw(&b, "/mnt/share"))
        .boot()
        .unwrap();
    sandbox
        .run("print('who:', open('/mnt/share/who').read())")
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("who: b"),
        "expected the directory given at restore, got: {output:?}"
    );
}

#[test]
fn restore_in_place_keeps_the_mounts_of_the_sandbox() {
    let tmp = temp_dir("mount-in-place");
    let snap = tmp.path().join("snap");
    let share = tmp.path().join("share");
    std::fs::create_dir(&share).unwrap();
    save_snapshot(&snap, Vec::new());

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .mount(Mount::rw(&share, "/mnt/share"))
        .boot()
        .unwrap();
    sandbox
        .run("open('/mnt/share/first.txt', 'w').write('1')")
        .unwrap();
    // An in-place restore of the mount-less image resumes it against this
    // sandbox's mounts, so the mount is back before the next call.
    sandbox.restore_from(&snap).unwrap();
    sandbox
        .run("open('/mnt/share/second.txt', 'w').write('2')")
        .unwrap();
    assert!(share.join("first.txt").exists());
    assert!(share.join("second.txt").exists());
}

#[test]
fn restore_keeps_a_busy_mount_the_host_no_longer_serves() {
    let tmp = temp_dir("mount-busy");
    let share = tmp.path().join("share");
    std::fs::create_dir(&share).unwrap();
    let mut saved = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .mount(Mount::rw(&share, "/mnt/share"))
        .boot()
        .unwrap();
    // A call parked with a file open on the mount.
    saved
        .submit(
            r#"
import time
f = open('/mnt/share/late.txt', 'w')
time.sleep(0.3)
try:
    f.write('late')
    f.close()
    print('write: ok')
except OSError as e:
    print('write: errno', e.errno)
"#,
        )
        .unwrap();
    let snap = saved.snapshot().unwrap();
    drop(saved);

    // Restored without the mount: the open file pins it, so the kernel
    // refuses the unmount and the mount stays, its host calls failing.
    let mut sandbox = SandboxBuilder::from_snapshot(snap).boot().unwrap();
    loop {
        match sandbox.step(Duration::from_secs(5)).unwrap() {
            Yield::CallDone | Yield::CallFailed { .. } => break,
            Yield::Blocked { .. } => {}
            exited @ Yield::Exited { .. } => panic!("guest exited: {exited:?}"),
        }
    }
    let output = sandbox.drain_output();
    // ESTALE: the kernel fails the stale mount's nodes itself.
    assert!(
        output.contains("write: errno 116"),
        "expected the write through the vanished mount to fail, got: {output:?}",
    );
    // The guest goes on.
    sandbox.run("print('alive')").unwrap();
    assert!(sandbox.drain_output().contains("alive"));
}

#[test]
fn a_busy_mount_never_reaches_the_directory_that_took_its_index() {
    // The stale mount was index 0; the restore's mount is index 0 too.
    // The open file's write must fail, and never land in the new mount.
    let tmp = temp_dir("mount-busy-index");
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");
    std::fs::create_dir(&a).unwrap();
    std::fs::create_dir(&b).unwrap();
    let mut saved = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .mount(Mount::rw(&a, "/mnt/a"))
        .boot()
        .unwrap();
    saved
        .submit(
            r#"
import time
f = open('/mnt/a/late.txt', 'w')
time.sleep(0.3)
try:
    f.write('late')
    f.close()
    print('write: ok')
except OSError as e:
    print('write: errno', e.errno)
"#,
        )
        .unwrap();
    let snap = saved.snapshot().unwrap();
    drop(saved);

    let mut sandbox = SandboxBuilder::from_snapshot(snap)
        .mount(Mount::rw(&b, "/mnt/b"))
        .boot()
        .unwrap();
    loop {
        match sandbox.step(Duration::from_secs(5)).unwrap() {
            Yield::CallDone | Yield::CallFailed { .. } => break,
            Yield::Blocked { .. } => {}
            exited @ Yield::Exited { .. } => panic!("guest exited: {exited:?}"),
        }
    }
    let output = sandbox.drain_output();
    assert!(
        output.contains("write: errno 116"),
        "expected the stale mount's write to fail with ESTALE, got: {output:?}",
    );
    assert!(
        !b.join("late.txt").exists(),
        "the stale mount's write reached the new mount"
    );
    assert!(
        !a.join("late.txt").exists()
            || std::fs::read_to_string(a.join("late.txt"))
                .unwrap()
                .is_empty()
    );
    // The new mount at that index works.
    sandbox
        .run("open('/mnt/b/ok.txt', 'w').write('ok')")
        .unwrap();
    assert!(b.join("ok.txt").exists());
}

#[test]
fn snapshot_of_a_restored_guest_carries_its_mounts_forward() {
    // A guest restored with mounts, snapshotted again and restored with
    // others: the second restore starts from the first restore's table.
    let tmp = temp_dir("mount-chained");
    let first = tmp.path().join("first");
    let second = tmp.path().join("second");
    let a = tmp.path().join("a");
    let b = tmp.path().join("b");
    std::fs::create_dir(&a).unwrap();
    std::fs::create_dir(&b).unwrap();
    std::fs::write(a.join("who"), "a").unwrap();
    std::fs::write(b.join("who"), "b").unwrap();
    save_snapshot(&first, Vec::new());

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&first)
        .unwrap()
        .mount(Mount::rw(&a, "/mnt/a"))
        .boot()
        .unwrap();
    sandbox
        .run("print('first:', open('/mnt/a/who').read())")
        .unwrap();
    assert!(sandbox.drain_output().contains("first: a"));
    sandbox.snapshot_to(&second).unwrap();
    drop(sandbox);

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&second)
        .unwrap()
        .mount(Mount::rw(&b, "/mnt/b"))
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
import os
print('a present:', os.path.exists('/mnt/a/who'))
print('second:', open('/mnt/b/who').read())
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("a present: False") && output.contains("second: b"),
        "expected the first restore's mount gone and the second's present, got: {output:?}",
    );
}

#[test]
fn restore_unmounts_a_mount_nested_under_another() {
    // Unmounting the outer one first would fail: the inner mount point
    // pins it.  The pass repeats until both are gone.
    let tmp = temp_dir("mount-nested");
    let snap = tmp.path().join("snap");
    let outer = tmp.path().join("outer");
    let inner = tmp.path().join("inner");
    std::fs::create_dir(&outer).unwrap();
    std::fs::create_dir(&inner).unwrap();
    std::fs::write(inner.join("who"), "inner").unwrap();
    save_snapshot(
        &snap,
        vec![
            Mount::rw(&outer, "/mnt/outer"),
            Mount::rw(&inner, "/mnt/outer/inner"),
        ],
    );

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"
import os
print('inner present:', os.path.exists('/mnt/outer/inner/who'))
"#,
        )
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("inner present: False"),
        "expected both nested mounts gone, got: {output:?}",
    );
}
