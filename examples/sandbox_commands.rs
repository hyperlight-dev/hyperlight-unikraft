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

use futures::io::{AsyncBufReadExt, AsyncRead, BufReader};
use hyperlight_unikraft::{
    Mount, SandboxBuilder,
    command::{Child, Stdio},
};

async fn report(name: &str, mut child: Child) -> Result<(), Box<dyn Error>> {
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let (status, (), ()) = futures::try_join!(
        child.wait(),
        async {
            report_lines(name, stdout, &mut io::stdout())
                .await
                .map_err(hyperlight_unikraft::Error::Wait)
        },
        async {
            report_lines(name, stderr, &mut io::stderr())
                .await
                .map_err(hyperlight_unikraft::Error::Wait)
        },
    )?;
    match status.code() {
        Some(code) => {
            let message = if status.success() {
                "success"
            } else {
                "non-zero exit status"
            };
            println!("[{name} exit code {code}: {message}]");
        }
        None => println!(
            "[{name} exit code unavailable: terminated by signal {}]",
            status.signal().expect("exit status has a signal")
        ),
    }
    if !status.success() {
        return Err(io::Error::other(format!("{name} failed: {status:?}")).into());
    }
    Ok(())
}

async fn report_lines(
    name: &str,
    reader: Option<impl AsyncRead + Unpin>,
    writer: &mut impl Write,
) -> io::Result<()> {
    let Some(reader) = reader else {
        return Ok(());
    };
    let mut reader = BufReader::new(reader);
    let mut line = Vec::new();
    while reader.read_until(b'\n', &mut line).await? != 0 {
        write!(writer, "[{name}] ")?;
        writer.write_all(&line)?;
        if !line.ends_with(b"\n") {
            writeln!(writer)?;
        }
        writer.flush()?;
        line.clear();
    }
    Ok(())
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
    let stdout = tree.stdout.take().expect("stdout was configured as piped");
    // This fixture's root deliberately exits with 7 before its descendant.
    let status = tree.wait().await?;
    println!("tree root: exit code {:?}", status.code());

    // Reading the taken pipe continues driving descendants after root exit.
    report_lines("tree", Some(stdout), &mut io::stdout()).await?;
    tree.wait_tree().await?;
    println!("whole tree finished");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{FutureExt, StreamExt, TryStreamExt};

    #[tokio::test]
    async fn prefixes_each_line_and_terminates_partial_lines() {
        let mut output = Vec::new();
        report_lines("worker", Some(&b"first\n\nlast\xff"[..]), &mut output)
            .await
            .unwrap();
        assert_eq!(output, b"[worker] first\n[worker] \n[worker] last\xff\n");
    }

    #[tokio::test]
    async fn empty_output_has_no_prefixed_line() {
        let mut output = Vec::new();
        report_lines("worker", Some(&b""[..]), &mut output)
            .await
            .unwrap();
        assert!(output.is_empty());
    }

    #[test]
    fn reports_complete_lines_before_eof() {
        let reader = futures::stream::iter([Ok::<_, io::Error>(b"early\npartial".to_vec())])
            .chain(futures::stream::pending())
            .into_async_read();
        let mut output = Vec::new();
        assert!(
            report_lines("worker", Some(reader), &mut output)
                .now_or_never()
                .is_none()
        );
        assert_eq!(output, b"[worker] early\n");
    }
}
