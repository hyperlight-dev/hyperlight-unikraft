//! Live cooperative-poll socket-recv run under a real hypervisor
//! (`/dev/kvm` on Linux).
//!
//! This exercises the *poll-friendly* blocking-socket paths: a guest that
//! `accept()`s a connection and then blocks in `recv()` must **not** issue a
//! blocking host call (which would freeze the single-vCPU VM for the whole
//! wait). Instead the guest's `hostsock` driver returns `EAGAIN` after a
//! non-blocking readiness pre-check, and the Unikraft posix-socket layer
//! parks the thread on the poll waitq via `uk_file_poll()` — a cooperative
//! scheduler yield. The scheduler then reaches idle and the poll pump hands
//! the vCPU back to the host, so [`Sandbox::poll`] keeps making forward
//! progress even while the guest is "blocked" in accept/recv.
//!
//! The host drives the guest with [`Sandbox::poll`] +
//! [`Sandbox::drive_host_functions`]. `drive_host_functions` `poll()`s the host-side socket fds
//! during the inter-step wait, so when the client below connects/sends the
//! loop re-enters `poll` promptly; the guest poll pump then rescans socket
//! readiness (`hostsock_rescan_events`) and wakes the parked accept/recv
//! thread.
//!
//! Topology: the guest's `socket`/`bind`/`listen`/`accept`/`recv` calls are
//! host-proxied, so the guest "listener" is a real host socket bound to
//! `127.0.0.1:34567`. A client thread on the host connects to that address
//! and sends a payload; the guest should accept, recv the bytes, and exit
//! (`Done`) within a bounded number of steps.
//!
//! Like `poll_run.rs`, this needs a hypervisor and built artifacts. It
//! self-skips (with a diagnostic) when either is missing so `cargo test`
//! still passes on runners without KVM or a built poll-recv kernel.
//!
//! Artifacts live in `examples/poll-recv-c/`: the poll+hostsock kernel at
//! `.unikraft/build/*_hyperlight-x86_64` and the initrd as
//! `*-initrd.cpio`. Populate with `just rootfs` then a `kraft-hyperlight
//! build` in that directory (its `kraft.yaml` enables CONFIG_HYPERLIGHT_POLL
//! and CONFIG_LIBHOSTSOCK).

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
const GUEST_PORT: u16 = 34567;

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
/// until the guest has bound/listened (or a deadline elapses). Runs on its
/// own thread so it can race the poll loop.
///
/// A deliberate pause is inserted between `connect()` and the first byte of
/// data: this is the scenario that previously busy-looped. While connected
/// but data-less, the guest is parked in `recv()` waiting for POLLIN, but the
/// accepted socket is already writable (POLLOUT), so a host inter-step wait
/// that watched POLLOUT would return instantly on every iteration and spin
/// `poll`. With the wait restricted to readability the host blocks for
/// the whole pause and the loop stays idle.
fn client_connect_and_send(payload: &[u8], deadline: Instant) -> bool {
    let addr = format!("127.0.0.1:{GUEST_PORT}");
    while Instant::now() < deadline {
        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                // Hold the connection open with no data to reproduce the
                // POLLOUT busy-loop scenario before sending.
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

/// Drive the poll-recv guest to completion cooperatively while a client
/// connects and sends. Asserts the poll loop makes forward progress and
/// terminates with `Done` — proving `accept()`/`recv()` yielded the vCPU
/// cooperatively (a blocking host recv would have frozen the VM instead of
/// letting `poll` return).
#[tokio::test]
async fn poll_recv_reaches_done_under_kvm() {
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
    sbox.reset_poll_deadline();
    sbox.restore().expect("restore before first poll");

    const MAX_STEPS: usize = 2000;
    const MAX_WALL: Duration = Duration::from_secs(30);
    // With a busy loop, the ~300ms data-less pause between connect and send
    // would spin `poll` thousands of times (toward MAX_STEPS). With the
    // inter-step wait restricted to readability, the host blocks for the whole
    // pause, so a correct run reaches Done in a small number of steps.
    const MAX_EXPECTED_STEPS: usize = 100;

    let start = Instant::now();

    // Client races the poll loop: it retries connect() until the guest's
    // host-proxied listener is up, then sends the payload.
    let client =
        std::thread::spawn(move || client_connect_and_send(b"hello-poll-recv", start + MAX_WALL));

    let mut steps = 0usize;
    let mut saw_wait_or_idle = false;
    let mut done = false;

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        steps += 1;
        match sbox.poll().expect("poll") {
            Poll::Ready(()) => {
                println!(
                    "poll_recv reached Done after {steps} steps / {:?}",
                    start.elapsed()
                );
                done = true;
                break;
            }
            Poll::Pending => {
                println!("poll_recv step {steps} / {:?}: Pending", start.elapsed());
                saw_wait_or_idle = true;
                // Drive host-side async work (the parked accept/recv future and
                // the inter-step socket-readiness wait) off the vCPU thread, so
                // the guest's parked accept/recv is resolved and re-driven on
                // the next step. Bounded so a missed wakeup can't hang the run.
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
        "poll-recv run did not reach Done within {steps} steps / {:?}; the \
         guest should have accepted, recv'd the payload, and exited",
        start.elapsed()
    );
    assert!(
        steps <= MAX_EXPECTED_STEPS,
        "poll-recv reached Done but took {steps} steps (> {MAX_EXPECTED_STEPS}); \
         the data-less pause between connect and send should have blocked the \
         host inter-step wait, not spun poll — a POLLOUT-driven busy loop \
         has regressed"
    );
    // The guest reports the number of bytes it recv'd via an explicit __hl_exit
    // hcall. "hello-poll-recv" is 15 bytes, so a successful accept+recv yields
    // exit code 15; the guest's failure paths report distinct codes 101-105.
    // This turns a functional regression (e.g. "accept failed") into a hard
    // test failure instead of a silently-passing Done.
    let exit_code = sbox.last_exit_code();
    assert_eq!(
        exit_code,
        b"hello-poll-recv".len() as i32,
        "guest exit code {exit_code} != 15; the async accept()/recv() path did \
         not deliver the payload to the guest (101=socket 102=bind 103=listen \
         104=accept 105=recv failure)"
    );
}
