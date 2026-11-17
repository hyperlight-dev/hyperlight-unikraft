// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! From the repository root, run `cargo run --example basic_command`.
//! Prerequisites: the rootfs and guest binary setup in `sandbox_commands.rs`.
//! A Tokio task streams guest stdout directly to host stdout.

use hyperlight_unikraft::{Mount, SandboxBuilder, command::Stdio};
use tokio::io::AsyncWriteExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = SandboxBuilder::from_initrd("build-elfloader/c-rootfs.cpio")
        .mount(Mount::ro("build-elfloader/bins/c", "/mnt/bin"))
        .echo_output(false)
        .boot_commands()?;

    let mut child = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "25", "Hello from the guest!"])
        .stdout(Stdio::piped())
        .spawn()
        .await?;

    let mut stdout = child.stdout.take().expect("stdout is piped");
    let copy = tokio::spawn(async move {
        let mut host_stdout = tokio::io::stdout();
        tokio::io::copy(&mut stdout, &mut host_stdout).await?;
        host_stdout.flush().await
    });

    // Waiting drives the guest while the spawned task drains its stdout pipe.
    let status = child.wait().await?;
    copy.await??;

    assert!(status.success(), "{status:?}");
    Ok(())
}
