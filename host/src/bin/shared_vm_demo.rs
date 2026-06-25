//! shared-vm-demo — two guest VMs sharing state through a host directory.
//!
//! VM A ("installer") writes Python packages into a shared host directory.
//! VM B ("executor") mounts that same directory read-only and imports them.
//!
//! Since in-guest downloading is unreliable (DNS/TLS inside the micro-VM),
//! this demo uses host-side pip to populate the shared dir, then VM A
//! verifies the contents and VM B imports the package.

use anyhow::{bail, Context, Result};
use hyperlight_unikraft::{pyhl, Preopen};
use std::path::{Path, PathBuf};

fn resolve_pyhl_home() -> Result<PathBuf> {
    if let Some(v) = std::env::var_os("PYHL_HOME") {
        return Ok(PathBuf::from(v));
    }
    let xdg = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/"));
            home.join(".local/share")
        });
    Ok(xdg.join("pyhl"))
}

fn main() -> Result<()> {
    pyhl::configure_surrogates(None);

    let home = resolve_pyhl_home()?;
    let shared_dir = Path::new("/tmp/shared-vm-packages");

    std::fs::create_dir_all(shared_dir)
        .with_context(|| format!("create shared dir {}", shared_dir.display()))?;

    // Host-side: install the package into the shared directory.
    eprintln!("[shared-vm-demo] installing humanize on host…");
    let status = std::process::Command::new("pip")
        .args(["install", "--target"])
        .arg(shared_dir)
        .args(["humanize", "--no-cache-dir", "--quiet"])
        .status()
        .context("spawn pip")?;
    if !status.success() {
        bail!("pip install failed");
    }

    // -- VM A: verify the package is visible in the shared mount ------------
    eprintln!("[shared-vm-demo] VM A (verifier) …");
    {
        let preopen = Preopen::new(shared_dir, "/host/packages")?;
        let mut rt = pyhl::Runtime::new(&home, &[preopen], None, None, None)?;

        let timing = rt.run_code(
            "\
import os
files = os.listdir('/host/packages')
print(f'VM A sees {len(files)} entries in /host/packages')
for f in sorted(files)[:5]:
    print(f'  {f}')
",
        )?;
        eprintln!(
            "[shared-vm-demo] VM A done (restore={:.1}ms, call={:.1}ms)",
            timing.restore_ms, timing.call_ms
        );
    }

    // -- VM B: mount read-only and import the package -----------------------
    eprintln!("[shared-vm-demo] VM B (executor) …");
    {
        let preopen = Preopen::new(shared_dir, "/host/packages")?.read_only();
        let mut rt = pyhl::Runtime::new(&home, &[preopen], None, None, None)?;

        let timing = rt.run_code(
            "\
import sys
sys.path.insert(0, '/host/packages')
import humanize
print(f'humanize version: {humanize.__version__}')
print(f'humanize demo: {humanize.naturalsize(1_500_000)}')
",
        )?;
        eprintln!(
            "[shared-vm-demo] VM B done (restore={:.1}ms, call={:.1}ms)",
            timing.restore_ms, timing.call_ms
        );
    }

    Ok(())
}
