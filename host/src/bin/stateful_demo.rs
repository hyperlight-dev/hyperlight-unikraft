use anyhow::{anyhow, Context, Result};
use hyperlight_unikraft::pyhl;
use std::path::PathBuf;
use std::time::Instant;

fn resolve_home() -> Result<PathBuf> {
    if let Ok(h) = std::env::var("PYHL_HOME") {
        return Ok(PathBuf::from(h));
    }
    let cwd = std::env::current_dir().context("read cwd")?.join(".pyhl");
    if cwd.join("snapshot").is_dir() {
        return Ok(cwd);
    }
    #[cfg(unix)]
    {
        let xdg = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .unwrap_or_else(|| {
                let home = std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("/"));
                home.join(".local/share")
            })
            .join("pyhl");
        if xdg.join("snapshot").is_dir() {
            return Ok(xdg);
        }
    }
    #[cfg(windows)]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let win = PathBuf::from(local).join("pyhl");
            if win.join("snapshot").is_dir() {
                return Ok(win);
            }
        }
        let home_dir = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".into());
        let dotpyhl = PathBuf::from(home_dir).join(".pyhl");
        if dotpyhl.join("snapshot").is_dir() {
            return Ok(dotpyhl);
        }
    }
    Err(anyhow!(
        "no pyhl image found. run `pyhl setup` first, or set PYHL_HOME."
    ))
}

fn main() -> Result<()> {
    let home = resolve_home()?;

    println!("Stateful multi-turn execution demo");
    println!("==================================\n");

    let t_init = Instant::now();
    let mut rt = pyhl::Runtime::new(&home, &[], None, None, Some(0))?;
    println!(
        "[init] runtime created in {:.0}ms\n",
        t_init.elapsed().as_secs_f64() * 1000.0
    );

    let turns: &[(&str, &str)] = &[
        (
            "Turn 1: Create variables",
            "x = 42\ny = 'hello from turn 1'\nprint(f'  x = {x}, y = {y!r}')",
        ),
        (
            "Turn 2: Access previous state + compute",
            "z = x * 2\nprint(f'  z = x * 2 = {z}')\nprint(f'  y from turn 1: {y!r}')",
        ),
        (
            "Turn 3: Import library, build on prior state",
            "import pandas as pd\ndf = pd.DataFrame({'val': [x, z, x + z]})\nprint(df.to_string(index=False))",
        ),
        (
            "Turn 4: Use everything from all prior turns",
            "total = df['val'].sum()\nprint(f'  x={x}, z={z}, df_sum={total}')\nprint(f'  All state persisted across {4} turns!')",
        ),
    ];

    for (i, (label, code)) in turns.iter().enumerate() {
        println!("--- {} ---", label);
        let t = rt.run_code_stateful(code)?;
        println!(
            "  [{:.0}ms{}]\n",
            t.call_ms,
            if i == 0 {
                format!(" (includes initial restore: {:.0}ms)", t.restore_ms)
            } else {
                String::new()
            }
        );
    }

    println!("Session complete — sandbox torn down.");
    Ok(())
}
