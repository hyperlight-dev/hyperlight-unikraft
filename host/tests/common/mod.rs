//! Shared fixtures for the live cooperative-poll integration tests.
//!
//! Every `poll_*.rs` test needs the same two things before it can run: a
//! hypervisor, and a built kernel + initrd for the example guest it drives.
//! Both are environment-dependent, so the tests self-skip (with a diagnostic)
//! rather than fail when either is missing — `cargo test` still passes on a
//! runner without KVM or without built artifacts.
//!
//! Each integration test is its own crate, so a helper only used by some of
//! them looks dead to the rest; hence the blanket allow.
#![allow(dead_code)]

use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Whether a usable hypervisor is present.
///
/// On Linux the KVM device must be openable read-write — present but unusable
/// (permissions, virtualization disabled) is the common CI case, and a plain
/// existence check would not catch it. Windows uses WHP, which Hyperlight
/// probes at runtime, so there is nothing cheap to check up front.
pub fn hypervisor_available() -> bool {
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

fn example_dir(example: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples")
        .join(example)
}

/// The first file in `dir` whose name matches `pred`.
fn find_file(dir: &Path, pred: impl Fn(&str) -> bool) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(&pred))
}

/// Kernel image and initrd built for `example`, if both are present.
pub fn guest_artifacts(example: &str) -> Option<(PathBuf, PathBuf)> {
    let dir = example_dir(example);
    // The stripped image, not the `.dbg` companion kraft leaves beside it.
    let kernel = find_file(&dir.join(".unikraft/build"), |n| {
        n.ends_with("_hyperlight-x86_64") && !n.ends_with(".dbg")
    })?;
    let initrd = find_file(&dir, |n| n.ends_with("-initrd.cpio"))?;
    Some((kernel, initrd))
}

/// Kernel + initrd for `example`, or `None` (having explained why on stderr)
/// when the test cannot run here.
pub fn setup(example: &str) -> Option<(PathBuf, PathBuf)> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available (no /dev/kvm)");
        return None;
    }
    let artifacts = guest_artifacts(example);
    if artifacts.is_none() {
        eprintln!(
            "SKIP: {example} artifacts missing — run `just rootfs` then \
             `kraft-hyperlight build --plat hyperlight --arch x86_64` in \
             examples/{example}/ to populate them"
        );
    }
    artifacts
}

/// Connect to a guest listener on `port` and send `payload`.
///
/// Retries until `deadline` because the guest binds its socket somewhere in
/// the middle of its own boot, so the first attempts race it. `send_delay`
/// holds the connection open with no data first, which is how a test drives
/// the guest into a readiness wait that has nothing to read yet.
///
/// Returns whether the payload was delivered.
pub fn client_connect_and_send(
    port: u16,
    payload: &[u8],
    send_delay: Duration,
    deadline: Instant,
) -> bool {
    let addr = format!("127.0.0.1:{port}");
    while Instant::now() < deadline {
        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                std::thread::sleep(send_delay);
                if stream.write_all(payload).is_ok() {
                    let _ = stream.flush();
                    // Keep the stream open briefly so the guest's recv sees the
                    // data before a close/RST could race it.
                    std::thread::sleep(Duration::from_millis(100));
                    return true;
                }
                return false;
            }
            Err(_) => std::thread::sleep(Duration::from_millis(10)),
        }
    }
    false
}
