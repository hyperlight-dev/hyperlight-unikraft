//! pydriver-run — run a Python script through hl_pydriver.
//!
//! Boots the sandbox once, calls "run" with the script's contents.
//! First call pays the Py_Initialize + warm-up import cost (~2 s);
//! the driver registers an FC-aware callback during that call, so
//! every subsequent call (via --repeat N or snapshot_now + loop) is
//! just the user's code + dispatch overhead.
//!
//! Usage:
//!   pydriver-run <kernel> <initrd.cpio> <script.py> [--repeat N]
//!
//! With --repeat, after the first (warmup + code) call we
//! `snapshot_now()` to capture the post-warmup state, then restore +
//! call_named for each remaining iteration so we can actually
//! *measure* the warm-path cost on runs 2..N.

use anyhow::{anyhow, Result};
use hyperlight_unikraft::Sandbox;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let kernel = args
        .next()
        .ok_or_else(|| anyhow!("usage: pydriver-run <kernel> <initrd> <script> [--repeat N]"))?;
    let initrd = args.next().ok_or_else(|| anyhow!("missing <initrd>"))?;
    let script_path = args.next().ok_or_else(|| anyhow!("missing <script>"))?;
    let mut repeat: u32 = 0;
    while let Some(a) = args.next() {
        if a == "--repeat" {
            repeat = args
                .next()
                .ok_or_else(|| anyhow!("--repeat requires a value"))?
                .parse()?;
        } else {
            return Err(anyhow!("unexpected arg: {}", a));
        }
    }
    let kernel = PathBuf::from(kernel);
    let initrd = PathBuf::from(initrd);
    let script = std::fs::read_to_string(&script_path)?;

    let t_evolve = Instant::now();
    let mut sandbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(5 * 512 * 1024 * 1024)
        .build()?;
    eprintln!(
        "[timing] evolve={:.1}ms",
        t_evolve.elapsed().as_secs_f64() * 1000.0
    );

    let t0 = Instant::now();
    sandbox.restore()?;
    let restore0_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let t1 = Instant::now();
    let _: () = sandbox.call_named("run", script.clone())?;
    let call0_ms = t1.elapsed().as_secs_f64() * 1000.0;
    eprintln!(
        "[run 1/{}] restore={:.1}ms call={:.1}ms (includes Py_Initialize + warmup)",
        repeat + 1,
        restore0_ms,
        call0_ms,
    );

    /* Stateful loop — no snapshot/restore between runs. Each call just
     * resumes from the previous call's halt, so Python state, stack,
     * and TLS are fully continuous across dispatches. The warm-path
     * measurement we care about. */
    for i in 2..=repeat + 1 {
        let tc = Instant::now();
        let _: () = sandbox.call_named("run", script.clone())?;
        let call_ms = tc.elapsed().as_secs_f64() * 1000.0;
        eprintln!("[run {}/{}] call={:.1}ms (warm)", i, repeat + 1, call_ms,);
    }

    Ok(())
}
