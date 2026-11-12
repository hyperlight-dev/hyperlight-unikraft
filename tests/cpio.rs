// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

#![cfg(feature = "cpio")]

mod common;

use hyperlight_unikraft::{SandboxBuilder, cpio::from_tar};

#[tokio::test]
async fn the_library_finds_the_driver_in_the_result() {
    let mut tar = tar::Builder::new(Vec::new());
    for (name, kind, data) in [
        ("usr/", tar::EntryType::Directory, &b""[..]),
        ("usr/local/", tar::EntryType::Directory, &b""[..]),
        ("usr/local/bin/", tar::EntryType::Directory, &b""[..]),
        (
            "usr/local/bin/hl_nodedriver",
            tar::EntryType::Regular,
            &b"ELF"[..],
        ),
    ] {
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(kind);
        header.set_mode(if kind.is_dir() { 0o755 } else { 0o644 });
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(1_700_000_000);
        header.set_size(data.len() as u64);
        tar.append_data(&mut header, name, data).unwrap();
    }
    let tar = tar.into_inner().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("rootfs.cpio");
    let mut file = std::fs::File::create(&path).unwrap();
    from_tar(&tar[..], &mut file).unwrap();
    drop(file);
    // Three bytes of ELF cannot run, but the kernel names the file it
    // was told to run and that it opened it: the host's detection read
    // the entry out of our archive, and the kernel found it in the
    // unpacked initrd.  An archive the host could not read would boot
    // nothing and say so differently.
    let (_sandbox, log) = common::guest_log(async {
        // The driver's name would size the guest for node; booting is all
        // this needs.
        SandboxBuilder::from_initrd(&path)
            .scratch_mb(64)
            .guest_log_level(tracing::level_filters::LevelFilter::ERROR)
            .boot()
            .unwrap()
    })
    .await;
    assert!(
        log.iter()
            .any(|l| l.contains("/usr/local/bin/hl_nodedriver: Image format not recognized")),
        "the kernel did not try our driver: {log:#?}"
    );
}
