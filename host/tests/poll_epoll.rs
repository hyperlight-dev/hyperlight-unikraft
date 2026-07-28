//! Live cooperative-poll run exercising **epoll over two host-proxied sockets**
//! from a Tokio runtime.
//!
//! This is the multi-socket sibling of `poll_recv_async.rs`. Where that test
//! parks a single blocking `recv()`, here the guest
//! (`examples/poll-epoll-c/poll.c`) multiplexes TWO listening sockets with one
//! epoll set and a single thread: it `epoll_wait()`s to accept a connection on
//! each port, then `epoll_wait()`s again to `recv()` a short ASCII integer from
//! each. Under the cooperative poll model `epoll_wait()` must yield the vCPU
//! back to the host (parking on the Unikraft posix-poll waitq) rather than issue
//! a blocking host call.
//!
//! The two clients **send at staggered times** (port A gets its byte early, port
//! B much later). This is the interesting part: the guest's phase-2
//! `epoll_wait()` returns readiness for socket A, the guest consumes it and
//! parks again, and only later does the second `epoll_wait()` return readiness
//! for socket B. So a single epoll set demonstrably delivers readiness for two
//! independent sockets across multiple cooperative park/resume cycles — the
//! host-side building block for the kernel-level "wait on many, wake on any"
//! (epoll-with-timeout) model.
//!
//! The guest exits with the **sum** of the two received integers (40 + 2 = 42),
//! which we assert as the exit code — proving both sockets' payloads reached the
//! guest intact through the multiplexed epoll path.
//!
//! Kernel reuse: the guest binary is loaded from the initrd at `/bin/poll`, so
//! the kernel is an app-agnostic ELF loader. `poll-epoll-c` needs exactly the
//! same kernel config as `poll-recv-c` (posix-poll — which provides epoll — plus
//! hostsock and the cooperative poll hooks). We therefore discover a dedicated
//! `poll-epoll-c` kernel if one has been built, and otherwise fall back to the
//! already-built `poll-recv-c` kernel. Only the initrd (the epoll app) is
//! specific to this test.
//!
//! Like the other live tests, this needs a hypervisor and built artifacts; it
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

/// The two ports the guest (`examples/poll-epoll-c/poll.c`) binds and listens
/// on. Baked into the guest, so the host necessarily proxies these two ports.
/// Distinct from `poll_recv`'s single port (34567) so the two test binaries can
/// run concurrently without fighting over a host port.
const PORT_A: u16 = 34568;
const PORT_B: u16 = 34569;

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

/// Find the app-agnostic ELF-loader kernel: prefer a dedicated `poll-epoll-c`
/// build, else fall back to the identical `poll-recv-c` kernel (same config;
/// the app lives in the initrd, not the kernel).
fn find_kernel() -> Option<PathBuf> {
    let examples = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples");
    for dir in ["poll-epoll-c", "poll-recv-c"] {
        let build = examples.join(dir).join(".unikraft/build");
        if !build.is_dir() {
            continue;
        }
        let kernel = std::fs::read_dir(&build).ok().and_then(|rd| {
            rd.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.ends_with("_hyperlight-x86_64") && !n.ends_with(".dbg"))
                    .unwrap_or(false)
            })
        });
        if let Some(k) = kernel {
            return Some(k);
        }
    }
    None
}

fn find_initrd() -> Option<PathBuf> {
    let example_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/poll-epoll-c");
    let initrd = example_dir.join("poll-epoll-initrd.cpio");
    initrd.is_file().then_some(initrd)
}

/// Skips the test body with a diagnostic if prerequisites aren't met.
/// Returns None → the test body should early-return.
fn setup() -> Option<(PathBuf, PathBuf)> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available (no /dev/kvm)");
        return None;
    }
    let Some(kernel) = find_kernel() else {
        eprintln!(
            "SKIP: no ELF-loader kernel found — build poll-epoll-c (or \
             poll-recv-c) under examples/*/.unikraft/build/ via \
             `kraft-hyperlight build --plat hyperlight --arch x86_64`"
        );
        return None;
    };
    let Some(initrd) = find_initrd() else {
        eprintln!(
            "SKIP: poll-epoll initrd missing — run `just rootfs` in \
             examples/poll-epoll-c/ to build poll-epoll-initrd.cpio"
        );
        return None;
    };
    Some((kernel, initrd))
}

/// Connect to `127.0.0.1:port`, retrying until the guest's host-proxied listener
/// is up, then (after `send_delay`) send `payload`. Runs on its own OS thread so
/// it can race the poll loop. The `send_delay` staggering between the two
/// clients is what forces the guest's epoll to return for one socket, park, and
/// later return for the other.
fn client_connect_and_send(
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

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

/// Drive the poll-epoll guest to completion, multiplexing two host-proxied
/// sockets through a single guest epoll set. Two clients connect on the two
/// ports and send `40` and `2` at staggered times; the guest sums them and
/// exits `42`. Asserts the run reaches `Done`, that it yielded cooperatively
/// (Wait/Idle) while parked in `epoll_wait`, and that the summed exit code is
/// correct — proving epoll delivered readiness for both sockets across the
/// cooperative park/resume boundary.
#[tokio::test]
async fn poll_epoll_two_sockets_reaches_done_under_kvm() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT_A, PORT_B]))
        .build()
        .expect("build sandbox");

    // Reset guest state once before the first step; the poll loop then runs
    // without further restores so scheduler/thread/socket state persists across
    // the HALT/re-entry boundary.
    sbox.restore().expect("restore before first poll");

    const MAX_STEPS: usize = 4000;
    const MAX_WALL: Duration = Duration::from_secs(30);

    let start = Instant::now();
    let deadline = start + MAX_WALL;

    // Two clients race the poll loop on their own OS threads. Port A sends its
    // byte early; port B waits noticeably longer, so the guest's phase-2
    // epoll_wait returns for A, parks, and only later returns for B.
    let client_a = std::thread::spawn(move || {
        client_connect_and_send(PORT_A, b"40", Duration::from_millis(150), deadline)
    });
    let client_b = std::thread::spawn(move || {
        client_connect_and_send(PORT_B, b"2", Duration::from_millis(600), deadline)
    });

    let mut steps = 0usize;
    let mut saw_wait_or_idle = false;
    let mut done = false;

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        steps += 1;
        // CPU half: blocking KVM_RUN, called inline as intended.
        match sbox.poll().expect("poll") {
            Poll::Ready(()) => {
                println!(
                    "poll_epoll reached Done after {steps} steps / {:?}",
                    start.elapsed()
                );
                done = true;
                break;
            }
            Poll::Pending => {
                saw_wait_or_idle = true;
                // Drive host-side async work (the parked accept/recv futures and
                // the inter-step socket-readiness wait) off the vCPU thread.
                // Bound the whole drive so a missed wakeup can't hang the run.
                let _ =
                    tokio::time::timeout(Duration::from_secs(5), sbox.drive_host_functions()).await;
            }
        }
    }

    let a_ok = client_a.join().unwrap_or(false);
    let b_ok = client_b.join().unwrap_or(false);

    assert!(
        a_ok,
        "client A failed to connect/send to the guest listener at 127.0.0.1:{PORT_A}"
    );
    assert!(
        b_ok,
        "client B failed to connect/send to the guest listener at 127.0.0.1:{PORT_B}"
    );
    assert!(
        saw_wait_or_idle,
        "expected the guest to yield cooperatively (Wait/Idle) while parked in \
         epoll_wait — a blocking host call would never have returned control to \
         poll"
    );
    assert!(
        done,
        "poll-epoll run did not reach Done within {steps} steps / {:?}; the \
         guest should have accepted both connections, recv'd both payloads via \
         epoll, and exited",
        start.elapsed()
    );
    // 40 + 2 = 42: both sockets' payloads reached the guest intact through the
    // single multiplexed epoll set.
    assert_eq!(
        sbox.last_exit_code(),
        42,
        "guest should exit with the sum of the two socket payloads (40 + 2); \
         got {} — a payload was lost or misrouted across the two epoll'd sockets",
        sbox.last_exit_code()
    );
}
