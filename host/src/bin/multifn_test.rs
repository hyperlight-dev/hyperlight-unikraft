//! multifn-test — end-to-end smoke check for the multi-function
//! dispatch plumbing.
//!
//! Drives the multifn-c guest (examples/multifn-c) through the new
//! Sandbox::call_named_async + snapshot_now APIs, and eyeballs the console
//! output. Expected sequence on stderr:
//!
//!   INIT                   <- from call_named_async("init", ())
//!   (snapshot_now here)
//!   RUN: hello             <- from call_named_async("run", "hello"), post-restore
//!   RUN: world             <- from call_named_async("run", "world"), post-restore
//!
//! Because `run` is called AFTER snapshot_now, the guest always sees
//! `already_initialized=true` (state preserved via snapshot), so the
//! "(uninitialized!)" branch must never fire.

use anyhow::Result;
use hyperlight_unikraft::Sandbox;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <kernel> <initrd.cpio>", args[0]);
        std::process::exit(2);
    }
    let kernel = PathBuf::from(&args[1]);
    let initrd = PathBuf::from(&args[2]);

    eprintln!("=== building sandbox ===");
    let mut sandbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(256 * 1024 * 1024)
        .build()?;

    // The guest's main() runs once here (post-evolve, with whatever
    // FunctionCall the builder synthesizes via its cmdline path — which
    // for multifn-c is a no-op since main reads env + fc slot), so the
    // snapshot the builder took is pre-"init". Explicitly call init
    // below to drive the dispatch path we actually want to exercise.

    eprintln!("=== restore + call_named_async(\"init\", ()) ===");
    sandbox.restore()?;
    sandbox.call_named_async("init", ()).await?;

    eprintln!("=== snapshot_now (capture post-init state) ===");
    sandbox.snapshot_now()?;

    for arg in ["hello", "world"] {
        eprintln!("=== restore + call_named_async(\"run\", {:?}) ===", arg);
        sandbox.restore()?;
        sandbox.call_named_async("run", arg.to_string()).await?;
    }

    eprintln!("=== done ===");
    Ok(())
}
