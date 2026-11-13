// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Concurrent commands and descendant output in one sandbox.
//!
//! From the repository root on Linux, with Docker, just, GCC, Rust, and KVM:
//!
//! ```sh
//! just build-rootfs c
//! mkdir -p build-elfloader/bins/c
//! gcc -O2 -Wall -static-pie -fPIE -o build-elfloader/bins/c/command examples/c/command.c
//! cargo run --example sandbox_commands
//! ```
//!
//! This reuses the C command fixture; `just build-test-bins` also builds it.
//! Each wait drives its own process tree. There is no sandbox-wide run loop.

use std::{error::Error, io, io::Write, path::Path};

use futures::io::AsyncReadExt;
use hyperlight_unikraft::{
    Mount, SandboxBuilder,
    command::{Child, Stdio},
};

async fn report(name: &str, child: Child) -> Result<(), Box<dyn Error>> {
    let output = child.wait_with_output().await?;
    report_lines(name, &output.stdout, &mut io::stdout())?;
    report_lines(name, &output.stderr, &mut io::stderr())?;
    match output.status.code() {
        Some(code) => {
            let message = if output.status.success() {
                "success"
            } else {
                "non-zero exit status"
            };
            println!("[{name} exit code {code}: {message}]");
        }
        None => println!(
            "[{name} exit code unavailable: terminated by signal {}]",
            output.status.signal().expect("exit status has a signal")
        ),
    }
    if !output.status.success() {
        return Err(io::Error::other(format!("{name} failed: {:?}", output.status)).into());
    }
    Ok(())
}

fn report_lines(name: &str, output: &[u8], writer: &mut impl Write) -> io::Result<()> {
    for line in output.split_inclusive(|byte| *byte == b'\n') {
        write!(writer, "[{name}] ")?;
        writer.write_all(line)?;
        if !line.ends_with(b"\n") {
            writeln!(writer)?;
        }
    }
    writer.flush()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sandbox = SandboxBuilder::from_initrd(root.join("build-elfloader/c-rootfs.cpio"))
        .mount(Mount::ro(root.join("build-elfloader/bins/c"), "/mnt/bin"))
        .scratch_mb(128)
        .echo_output(false)
        .boot()?
        .into_commands()?;

    let first = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "150", "hello from the first command"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let second = sandbox
        .command("/mnt/bin/command")
        .args(["sleep", "25", "hello from the second command"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Both waits are polled concurrently, including their output drains.
    futures::try_join!(report("first", first), report("second", second))?;

    let mut tree = sandbox
        .command("/mnt/bin/command")
        .arg("tree")
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdout = tree.stdout.take().expect("stdout was configured as piped");
    // This fixture's root deliberately exits with 7 before its descendant.
    let status = tree.wait().await?;
    println!("tree root: exit code {:?}", status.code());

    // Reading the taken pipe continues driving descendants after root exit.
    let mut output = Vec::new();
    stdout.read_to_end(&mut output).await?;
    io::stdout().write_all(&output)?;
    tree.wait_tree().await?;
    println!("whole tree finished");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_each_line_and_terminates_partial_lines() {
        let mut output = Vec::new();
        report_lines("worker", b"first\n\nlast\xff", &mut output).unwrap();
        assert_eq!(output, b"[worker] first\n[worker] \n[worker] last\xff\n");
    }

    #[test]
    fn empty_output_has_no_prefixed_line() {
        let mut output = Vec::new();
        report_lines("worker", b"", &mut output).unwrap();
        assert!(output.is_empty());
    }
}
