//! shared-vm-demo — two guest VMs sharing state through a host directory.
//!
//! An "installer VM" downloads a Python wheel from PyPI and extracts it
//! into a shared host directory. An "executor VM" then mounts that same
//! directory read-only and imports the installed package.

use anyhow::{Context, Result};
use hyperlight_unikraft::{pyhl, NetworkPolicy, Preopen};
use std::path::{Path, PathBuf};

/// Resolve the pyhl image home (same XDG logic as the `pyhl` binary).
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

    // Ensure the shared directory exists before creating preopens
    // (Preopen::new canonicalises the host path, so it must exist).
    std::fs::create_dir_all(shared_dir)
        .with_context(|| format!("create shared dir {}", shared_dir.display()))?;

    // -- Installer VM: download + extract a wheel into the shared dir --------
    eprintln!("[shared-vm-demo] starting installer VM …");
    {
        let preopen = Preopen::new(shared_dir, "/host/packages")?;
        let mut rt = pyhl::Runtime::new(
            &home,
            &[preopen],
            Some(&NetworkPolicy::AllowAll),
            None,
            None,
        )?;

        let timing = rt.run_code(
            "\
import urllib.request, zipfile, io
url = 'https://files.pythonhosted.org/packages/b7/ce/149a00dd41f10bc29e5921b496af8b574d8413afcd5e30f4c4e3cd1f2942/humanize-4.12.3-py3-none-any.whl'
data = urllib.request.urlopen(url).read()
zf = zipfile.ZipFile(io.BytesIO(data))
zf.extractall('/host/packages')
print('installed humanize to /host/packages')
",
        )?;
        eprintln!(
            "[shared-vm-demo] installer done (restore={:.1}ms, call={:.1}ms)",
            timing.restore_ms, timing.call_ms
        );
    }

    // -- Executor VM: mount read-only and import the package -----------------
    eprintln!("[shared-vm-demo] starting executor VM …");
    {
        let preopen = Preopen::new(shared_dir, "/host/packages")?.read_only();
        let mut rt = pyhl::Runtime::new(&home, &[preopen], None, None, None)?;

        let timing = rt.run_code(
            "\
import sys
sys.path.insert(0, '/host/packages')
import humanize
print(f'humanize version: {humanize.__version__}')
",
        )?;
        eprintln!(
            "[shared-vm-demo] executor done (restore={:.1}ms, call={:.1}ms)",
            timing.restore_ms, timing.call_ms
        );
    }

    Ok(())
}
