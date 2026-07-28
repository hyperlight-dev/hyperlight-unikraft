//! Live cooperative-poll socket-recv run driven from a **Tokio runtime**,
//! exercising [`Sandbox::drive_host_functions`] (the async inter-step drive).
//!
//! This is the async twin of `poll_recv.rs`. The cooperative model splits into
//! two halves:
//!
//! - **CPU half** — [`Sandbox::poll`] runs the vCPU via a blocking
//!   `KVM_RUN`. It stays synchronous by design; here we simply call it inline
//!   on the runtime thread (the client runs on its own OS thread, so blocking
//!   the single-threaded test runtime for the duration of a step is fine).
//! - **I/O half** — [`Sandbox::drive_host_functions`] `await`s host-socket
//!   readiness on the Tokio reactor (via `AsyncFd`) instead of blocking. When
//!   the client below connects and later sends, the awaited drive resolves and
//!   the loop re-enters `poll`, whose guest pump rescans socket readiness
//!   (`hostsock_rescan_events`) and wakes the parked `accept`/`recv` thread.
//!
//! The client deliberately pauses between `connect()` and its first byte to
//! reproduce the data-less window: during it the guest is parked on `POLLIN`
//! while the socket is already writable. `drive_host_functions` watches
//! read-readiness only, so the await blocks on the
//! reactor for the whole pause instead of spinning — the run reaches `Done` in
//! a small number of steps.
//!
//! Like `poll_recv.rs`, this needs a hypervisor and built artifacts; it
//! self-skips (with a diagnostic) when either is missing so `cargo test`
//! still passes on runners without KVM or a built kernel.

use core::task::Poll;
use hyperlight_unikraft::{ListenPorts, NetworkPolicy, Sandbox};
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Environment probe
// ---------------------------------------------------------------------------

/// The port the guest (`examples/poll-recv-c/poll.c`) binds and listens on.
/// It is baked into the guest, so every test in this file necessarily proxies
/// the *same* host port.
const GUEST_PORT: u16 = 34567;

/// Serializes the two `#[tokio::test]` functions below.
///
/// libtest runs test functions on parallel threads, and both tests here drive
/// a guest whose listener is proxied onto the single fixed host port
/// [`GUEST_PORT`] (hardcoded in `examples/poll-recv-c/poll.c`). Two concurrent
/// runs would fight over that port: the second host listener fails to bind, its
/// client can never connect, and the loser spins on `Idle` until `client.join()`
/// blocks for the full wall deadline. Holding this lock for the duration of each
/// test makes them run one at a time. Poison is ignored so a panic in one test
/// (which is exactly when we still want the other to run) doesn't wedge it.
static PORT_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn hypervisor_available() -> bool {
    #[cfg(unix)]
    {
        std::fs::metadata("/dev/kvm")
            .map(|_| {
                std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/kvm")
                    .is_ok()
            })
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        true
    }
}

fn poll_recv_artifacts() -> Option<(PathBuf, PathBuf)> {
    let example_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/poll-recv-c");
    let build = example_dir.join(".unikraft/build");
    if !build.is_dir() {
        return None;
    }
    let kernel = std::fs::read_dir(&build)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("_hyperlight-x86_64") && !n.ends_with(".dbg"))
                .unwrap_or(false)
        })?;
    let initrd = std::fs::read_dir(&example_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("-initrd.cpio"))
                .unwrap_or(false)
        })?;
    Some((kernel, initrd))
}

/// Skips the test body with a diagnostic if prerequisites aren't met.
/// Returns None → the test body should early-return.
fn setup() -> Option<(PathBuf, PathBuf)> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available (no /dev/kvm)");
        return None;
    }
    let Some((kernel, initrd)) = poll_recv_artifacts() else {
        eprintln!(
            "SKIP: poll-recv-c artifacts missing under \
             examples/poll-recv-c/.unikraft/build/ — run `just rootfs` then \
             `kraft-hyperlight build --plat hyperlight --arch x86_64` in \
             examples/poll-recv-c/ (kraft.yaml enables CONFIG_HYPERLIGHT_POLL \
             and CONFIG_LIBHOSTSOCK) to populate them"
        );
        return None;
    };
    Some((kernel, initrd))
}

/// Connect to the guest's host-proxied listener and send a payload, retrying
/// until the guest has bound/listened (or a deadline elapses). Runs on its own
/// OS thread so it can race the async poll loop.
///
/// The deliberate pause between `connect()` and the first byte reproduces the
/// data-less window that the async wait must sleep through (on the reactor)
/// rather than spin.
fn client_connect_and_send(payload: &[u8], deadline: Instant) -> bool {
    let addr = format!("127.0.0.1:{GUEST_PORT}");
    while Instant::now() < deadline {
        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                std::thread::sleep(Duration::from_millis(300));
                if stream.write_all(payload).is_ok() {
                    let _ = stream.flush();
                    // Keep the stream open briefly so the guest's recv sees the
                    // data before a close/RST could race it.
                    std::thread::sleep(Duration::from_millis(50));
                    return true;
                }
            }
            Err(_) => std::thread::sleep(Duration::from_millis(10)),
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

/// Drive the poll-recv guest to completion from a Tokio runtime, using
/// `poll` (blocking) for the CPU half and `drive_host_functions().await` for
/// the I/O half. Asserts the run reaches `Done`, that it yielded cooperatively
/// (Wait/Idle) while parked in accept()/recv(), and that it did so in a small
/// number of steps (proving the async wait blocked on the reactor during the
/// client's data-less pause instead of busy-looping).
#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn poll_recv_reaches_done_under_kvm_async() {
    // Serialize against the other test: both proxy the same fixed host port.
    let _serialize = PORT_GUARD.lock().unwrap_or_else(|e| e.into_inner());

    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([GUEST_PORT]))
        .build()
        .expect("build sandbox");

    // Reset guest state once before the first step; the poll loop then runs
    // without further restores so scheduler/thread/socket state persists
    // across the HALT/re-entry boundary.
    sbox.restore().expect("restore before first poll");

    const MAX_STEPS: usize = 2000;
    const MAX_WALL: Duration = Duration::from_secs(30);
    // With a busy loop the ~300ms data-less pause would spin `poll`
    // thousands of times; with the async wait blocking on the reactor a
    // correct run reaches Done in a small number of steps.
    const MAX_EXPECTED_STEPS: usize = 100;

    let start = Instant::now();

    // Client races the poll loop on its own OS thread: it retries connect()
    // until the guest's host-proxied listener is up, then (after a pause)
    // sends the payload.
    let client =
        std::thread::spawn(move || client_connect_and_send(b"hello-poll-recv", start + MAX_WALL));

    let mut steps = 0usize;
    let mut saw_wait_or_idle = false;
    let mut done = false;

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        steps += 1;
        // CPU half: blocking KVM_RUN, called inline as intended.
        match sbox.poll().expect("poll") {
            Poll::Ready(()) => {
                println!(
                    "poll_recv_async reached Done after {steps} steps / {:?}",
                    start.elapsed()
                );
                done = true;
                break;
            }
            Poll::Pending => {
                println!(
                    "poll_recv_async step {steps} / {:?}: Pending",
                    start.elapsed()
                );
                saw_wait_or_idle = true;
                // Drive host-side async work (the parked accept/recv future and
                // the inter-step socket-readiness wait) off the vCPU thread.
                // Bound the whole drive so a missed wakeup can't hang the run.
                let _ =
                    tokio::time::timeout(Duration::from_secs(5), sbox.drive_host_functions()).await;
            }
        }
    }

    let client_ok = client.join().unwrap_or(false);

    assert!(
        client_ok,
        "client failed to connect/send to the guest listener at 127.0.0.1:{GUEST_PORT}"
    );
    assert!(
        saw_wait_or_idle,
        "expected the guest to yield cooperatively (Wait/Idle) while parked \
         in accept()/recv() — a blocking host call would never have returned \
         control to poll"
    );
    assert!(
        done,
        "poll-recv-async run did not reach Done within {steps} steps / {:?}; \
         the guest should have accepted, recv'd the payload, and exited",
        start.elapsed()
    );
    assert!(
        steps <= MAX_EXPECTED_STEPS,
        "poll-recv-async reached Done but took {steps} steps (> \
         {MAX_EXPECTED_STEPS}); the data-less pause between connect and send \
         should have blocked the async wait on the reactor, not spun poll"
    );
}

/// Same guest, but driven with the higher-level [`Sandbox::poll_run_async`],
/// which runs the whole cooperative poll loop internally (blocking `poll`
/// + awaited `drive_host_functions`) and returns the guest exit code. Proves the
/// one-call async convenience API drives the VM to completion within the wall
/// deadline.
#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn poll_run_async_drives_poll_recv_under_kvm() {
    // Serialize against the other test: both proxy the same fixed host port.
    let _serialize = PORT_GUARD.lock().unwrap_or_else(|e| e.into_inner());

    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([GUEST_PORT]))
        .build()
        .expect("build sandbox");

    sbox.restore().expect("restore before first poll");

    const MAX_WALL: Duration = Duration::from_secs(30);
    let start = Instant::now();

    let client =
        std::thread::spawn(move || client_connect_and_send(b"hello-poll-recv", start + MAX_WALL));

    // Drive the entire run with the convenience API, bounded so a missed
    // wakeup can't hang the test.
    let exit = tokio::time::timeout(MAX_WALL, sbox.poll_run_async())
        .await
        .expect("poll_run_async did not complete within the wall deadline")
        .expect("poll_run_async");

    let client_ok = client.join().unwrap_or(false);

    assert!(
        client_ok,
        "client failed to connect/send to the guest listener at 127.0.0.1:{GUEST_PORT}"
    );
    println!(
        "poll_run_async reached Done with exit code {exit} in {:?}",
        start.elapsed()
    );
    // The guest reports the number of bytes it recv'd via __hl_exit; the
    // "hello-poll-recv" payload is 15 bytes, so a successful async accept+recv
    // yields exit code 15 (failure paths report 101-105).
    assert_eq!(
        exit,
        b"hello-poll-recv".len() as i32,
        "expected the guest to recv 15 bytes (exit 15), got {exit}"
    );
    assert_eq!(exit, sbox.last_exit_code());
}
