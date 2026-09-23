// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Derives the snapshot key from what a snapshot depends on, so that only
//! a change to those refuses a saved snapshot; see `SNAPSHOT_KEY` in the
//! library.

use sha2::{Digest, Sha256};

/// The host side of the guest contract: the host functions and their
/// meaning, the I/O stack sizes, the heap and scratch layout, the guest
/// MSR list, and the hyperlight-host release that reads a snapshot.  Bump
/// it when any of those changes in a way a snapshot would notice; a
/// kernel change rolls the key by itself.
const SNAPSHOT_CONTRACT: u32 = 1;

/// The embedded kernel, whose code and host-call protocol a snapshot
/// carries in its memory.
const KERNEL: &str = "kernel/elfloader_hyperlight-x86_64";

fn main() {
    println!("cargo:rerun-if-changed={KERNEL}");
    println!("cargo:rerun-if-changed=build.rs");
    let kernel = std::fs::read(KERNEL).expect("the embedded kernel is in the tree");
    let hash: String = Sha256::digest(&kernel)
        .iter()
        .take(8)
        .map(|b| format!("{b:02x}"))
        .collect();
    println!("cargo:rustc-env=HLUK_SNAPSHOT_KEY=k{hash}-c{SNAPSHOT_CONTRACT}");
}
