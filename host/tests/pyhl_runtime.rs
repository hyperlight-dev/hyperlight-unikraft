//! Integration tests for the `pyhl::Runtime` API.
//!
//! These tests exercise the programmatic interface that library consumers
//! use: `Runtime::new()` → `run_code()` with various configurations
//! (filesystem mounts, network policies, exit codes, hermetic rewinds).
//!
//! All tests self-skip if the pyhl image is not installed (no snapshot at
//! `.pyhl/snapshot.hls`) or if no hypervisor is available. Run `pyhl setup`
//! to populate the image before running these tests.

use hyperlight_unikraft::pyhl::Runtime;
use hyperlight_unikraft::{AllowList, NetworkPolicy, Preopen};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Environment probe
// ---------------------------------------------------------------------------

fn hypervisor_available() -> bool {
    #[cfg(unix)]
    {
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/kvm")
            .is_ok()
    }
    #[cfg(windows)]
    {
        true
    }
}

fn pyhl_home() -> Option<PathBuf> {
    // Check .pyhl/ in the workspace root (two levels up from host/)
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".pyhl");
    if workspace.join("snapshot.hls").is_file() {
        return Some(workspace);
    }
    // Check user-level install
    if let Some(home) = dirs_or_default() {
        if home.join("snapshot.hls").is_file() {
            return Some(home);
        }
    }
    None
}

fn dirs_or_default() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else {
        Some(
            std::env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    let home = std::env::var_os("HOME").unwrap_or_default();
                    PathBuf::from(home).join(".local/share")
                }),
        )
    };
    base.map(|b| b.join("pyhl"))
}

fn setup() -> Option<(PathBuf, Runtime)> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available");
        return None;
    }
    let home = pyhl_home()?;
    let rt = Runtime::new(&home, &[], None, None).ok()?;
    Some((home, rt))
}

fn setup_with_net(policy: NetworkPolicy) -> Option<Runtime> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available");
        return None;
    }
    let home = pyhl_home()?;
    Runtime::new(&home, &[], Some(&policy), None).ok()
}

fn setup_with_mount(preopen: Preopen) -> Option<Runtime> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available");
        return None;
    }
    let home = pyhl_home()?;
    Runtime::new(&home, &[preopen], None, None).ok()
}

// ---------------------------------------------------------------------------
// Basic execution
// ---------------------------------------------------------------------------

#[test]
fn runtime_hello_world() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    let timing = rt.run_code("print('hello from runtime test')").unwrap();
    assert_eq!(timing.exit_code, 0);
}

#[test]
fn runtime_pandas_import() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    let timing = rt
        .run_code("import pandas as pd; print(pd.DataFrame({'x':[1,2,3]}).sum().to_dict())")
        .unwrap();
    assert_eq!(timing.exit_code, 0);
}

// ---------------------------------------------------------------------------
// Exit code propagation
// ---------------------------------------------------------------------------

#[test]
fn runtime_exit_code_zero() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    let timing = rt.run_code("import sys; sys.exit(0)").unwrap();
    assert_eq!(timing.exit_code, 0);
}

#[test]
fn runtime_exit_code_nonzero() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    let timing = rt.run_code("import sys; sys.exit(42)").unwrap();
    assert_eq!(timing.exit_code, 42);
}

#[test]
fn runtime_exit_code_from_exception() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    let timing = rt.run_code("raise ValueError('boom')").unwrap();
    assert_ne!(timing.exit_code, 0);
}

// ---------------------------------------------------------------------------
// Hermetic execution (state isolation between calls)
// ---------------------------------------------------------------------------

#[test]
fn runtime_hermetic_no_state_leak() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    // First call sets a global
    let t1 = rt
        .run_code("MARKER = 'set_by_first_call'; print('first')")
        .unwrap();
    assert_eq!(t1.exit_code, 0);

    // Second call should NOT see the global from the first
    let t2 = rt
        .run_code(
            "import sys\ntry:\n    print(MARKER)\n    sys.exit(1)\nexcept NameError:\n    print('clean')\n    sys.exit(0)",
        )
        .unwrap();
    assert_eq!(t2.exit_code, 0, "state leaked between hermetic calls");
}

#[test]
fn runtime_repeated_runs_stable() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };
    for i in 0..5 {
        let timing = rt
            .run_code(&format!("import time; print('iter {}', time.time())", i))
            .unwrap();
        assert_eq!(timing.exit_code, 0, "iteration {i} failed");
    }
}

// ---------------------------------------------------------------------------
// Filesystem access via Preopen
// ---------------------------------------------------------------------------

#[test]
fn runtime_filesystem_write_and_read() {
    let tmp = tempdir("hl-rt-fs");
    let preopen = match Preopen::new(&tmp, "/host") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("SKIP: cannot create preopen: {e}");
            cleanup(&tmp);
            return;
        }
    };
    let Some(mut rt) = setup_with_mount(preopen) else {
        cleanup(&tmp);
        return;
    };

    // Write a file from guest
    let timing = rt
        .run_code("with open('/host/test.txt', 'w') as f: f.write('hello from guest\\n')")
        .unwrap();
    assert_eq!(timing.exit_code, 0);

    // Verify on host side
    let content = std::fs::read_to_string(tmp.join("test.txt")).unwrap();
    assert_eq!(content, "hello from guest\n");

    // Read it back from guest
    let timing = rt
        .run_code("with open('/host/test.txt') as f: print(f.read().strip())")
        .unwrap();
    assert_eq!(timing.exit_code, 0);

    cleanup(&tmp);
}

#[test]
fn runtime_filesystem_listdir() {
    let tmp = tempdir("hl-rt-ls");
    std::fs::write(tmp.join("a.txt"), "aaa").unwrap();
    std::fs::write(tmp.join("b.txt"), "bbb").unwrap();
    let preopen = match Preopen::new(&tmp, "/host") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("SKIP: cannot create preopen: {e}");
            cleanup(&tmp);
            return;
        }
    };
    let Some(mut rt) = setup_with_mount(preopen) else {
        cleanup(&tmp);
        return;
    };

    let timing = rt
        .run_code("import os; files = sorted(os.listdir('/host')); print(files); assert 'a.txt' in files and 'b.txt' in files")
        .unwrap();
    assert_eq!(timing.exit_code, 0);

    cleanup(&tmp);
}

#[test]
fn runtime_filesystem_mkdir_and_stat() {
    let tmp = tempdir("hl-rt-mkdir");
    let preopen = match Preopen::new(&tmp, "/host") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("SKIP: cannot create preopen: {e}");
            cleanup(&tmp);
            return;
        }
    };
    let Some(mut rt) = setup_with_mount(preopen) else {
        cleanup(&tmp);
        return;
    };

    let timing = rt
        .run_code("import os\nos.makedirs('/host/sub/dir', exist_ok=True)\nwith open('/host/sub/dir/file.txt', 'w') as f: f.write('nested')\nst = os.stat('/host/sub/dir/file.txt')\nprint(f'size={st.st_size}')")
        .unwrap();
    assert_eq!(timing.exit_code, 0);

    assert!(tmp.join("sub/dir/file.txt").is_file());
    assert_eq!(
        std::fs::read_to_string(tmp.join("sub/dir/file.txt")).unwrap(),
        "nested"
    );

    cleanup(&tmp);
}

// ---------------------------------------------------------------------------
// Network policy enforcement
// ---------------------------------------------------------------------------

#[test]
fn runtime_network_allowlist_permits_allowed_host() {
    let al = match AllowList::from_hosts(&["example.com"]) {
        Ok(al) => al,
        Err(e) => {
            eprintln!("SKIP: DNS resolution failed: {e}");
            return;
        }
    };
    let Some(mut rt) = setup_with_net(NetworkPolicy::AllowList(al)) else {
        return;
    };

    let timing = rt
        .run_code("import urllib.request; r = urllib.request.urlopen('http://example.com/', timeout=10); print(r.status)")
        .unwrap();
    assert_eq!(timing.exit_code, 0);
}

#[test]
fn runtime_network_allowlist_blocks_unlisted_host() {
    let al = match AllowList::from_hosts(&["example.com"]) {
        Ok(al) => al,
        Err(e) => {
            eprintln!("SKIP: DNS resolution failed: {e}");
            return;
        }
    };
    let Some(mut rt) = setup_with_net(NetworkPolicy::AllowList(al)) else {
        return;
    };

    // httpbin.org is NOT in the allowlist — should be blocked
    let timing = rt
        .run_code("import urllib.request, sys\ntry:\n    urllib.request.urlopen('http://httpbin.org/', timeout=5)\n    sys.exit(1)\nexcept Exception:\n    sys.exit(0)")
        .unwrap();
    assert_eq!(timing.exit_code, 0, "unlisted host should be blocked");
}

#[test]
fn runtime_network_disabled_by_default() {
    let Some((_home, mut rt)) = setup() else {
        return;
    };

    // Without network policy, socket operations should fail
    let timing = rt
        .run_code("import urllib.request, sys\ntry:\n    urllib.request.urlopen('http://example.com/', timeout=5)\n    sys.exit(1)\nexcept Exception:\n    sys.exit(0)")
        .unwrap();
    assert_eq!(
        timing.exit_code, 0,
        "networking should be disabled by default"
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn tempdir(prefix: &str) -> PathBuf {
    let base = std::env::temp_dir();
    let pid = std::process::id();
    let uniq = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = base.join(format!("{prefix}-{pid}-{uniq}"));
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}

fn cleanup(path: &Path) {
    let _ = std::fs::remove_dir_all(path);
}
