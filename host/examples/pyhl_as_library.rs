//! Drive the python-agent-driver image from Rust without shelling out to
//! the `pyhl` binary.
//!
//! Usage: `cargo run --release --example pyhl_as_library -- <code>`
//!
//! Assumes `pyhl setup` has already run and `.pyhl/snapshot.hls` exists
//! in the current directory (or override with `PYHL_HOME`).

use hyperlight_unikraft::{pyhl, Preopen};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let code = std::env::args()
        .nth(1)
        .unwrap_or_else(|| r#"print("hello from the pyhl library API")"#.to_string());

    let home = std::env::var("PYHL_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Path::new(".pyhl").to_path_buf());

    // Default: no mounts. Add `Preopen::new(host, guest)` entries to
    // expose host directories via the guest's hostfs.
    let mounts: &[Preopen] = &[];

    let mut rt = pyhl::Runtime::new(&home, mounts, None, None)?;

    eprintln!("-- first run (hermetic from loaded snapshot) --");
    let t1 = rt.run_code(&code)?;
    eprintln!("restore={:.1}ms call={:.1}ms", t1.restore_ms, t1.call_ms);

    eprintln!("-- second run (restores to the same snapshot) --");
    let t2 = rt.run_code(&code)?;
    eprintln!("restore={:.1}ms call={:.1}ms", t2.restore_ms, t2.call_ms);

    Ok(())
}
