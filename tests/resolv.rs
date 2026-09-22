// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use common::{require_rootfs, temp_dir};
use hyperlight_unikraft::{AppSandbox, SandboxBuilder};

const HERE: &str = "nameserver 192.0.2.53\nsearch example.test\n";
const THERE: &str = "nameserver 198.51.100.53\nsearch other.test\noptions ndots:5 single-request\n";
const SHOW: &str = "print(open('/etc/resolv.conf').read())";

fn boot(resolv_conf: Option<&str>) -> AppSandbox {
    let mut builder = SandboxBuilder::from_initrd(require_rootfs("python")).scratch_mb(256);
    if let Some(rc) = resolv_conf {
        builder = builder.resolv_conf(rc);
    }
    builder.boot().expect("boot")
}

fn show(sandbox: &mut AppSandbox) -> String {
    sandbox.run(SHOW).expect("read /etc/resolv.conf");
    sandbox.drain_output()
}

/// The file handed to the host is what the guest reads, with the socket
/// layer's `single-request` quirk added.
#[test]
fn resolv_conf_is_written_at_boot() {
    let mut sandbox = boot(Some(HERE));
    let out = show(&mut sandbox);
    assert!(
        out.contains("nameserver 192.0.2.53")
            && out.contains("search example.test")
            && out.contains("options single-request"),
        "got {out:?}"
    );
}

/// Without one, the rootfs's own file stands.
#[test]
fn rootfs_resolv_conf_stands_without_one() {
    let mut sandbox = boot(None);
    let out = show(&mut sandbox);
    assert!(
        out.contains("nameserver") && !out.contains("192.0.2.53"),
        "got {out:?}"
    );
}

/// A restored guest resolves names where it now runs: the file is
/// rewritten on resume, and an options line of the caller's is kept as is.
#[test]
fn resolv_conf_is_rewritten_on_restore() {
    let dir = temp_dir("resolv-restore");
    let mut sandbox = boot(Some(HERE));
    assert!(show(&mut sandbox).contains("192.0.2.53"));
    sandbox.snapshot_to(dir.path()).expect("snapshot");
    drop(sandbox);

    let mut restored = SandboxBuilder::from_snapshot_dir(dir.path())
        .expect("load")
        .resolv_conf(THERE)
        .boot()
        .expect("restore");
    let out = show(&mut restored);
    assert!(
        out.contains("nameserver 198.51.100.53")
            && out.contains("search other.test")
            && !out.contains("192.0.2.53"),
        "got {out:?}"
    );
    assert_eq!(out.matches("single-request").count(), 1, "got {out:?}");
}
