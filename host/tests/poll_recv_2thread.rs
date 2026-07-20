//! Live cooperative-poll run exercising **two guest threads each blocking in
//! `recv()` on its own host-proxied socket**, from a Tokio runtime.
//!
//! This is the multi-threaded sibling of `poll_recv.rs` / `poll_recv_async.rs`.
//! Where those park a single blocking `recv()` on one thread, here the guest
//! (`examples/poll-recv-2thread-c/poll.c`) spawns TWO worker threads; each owns
//! its own listening socket on its own port, accepts a connection, and blocks in
//! `recv()`. So at the interesting moment BOTH threads are simultaneously parked
//! in a blocking `recv()` on two different sockets.
//!
//! Why this matters for the async host: two concurrently-parked guest recvs mean
//! the host has TWO in-flight async net futures on the Tokio reactor at once,
//! exercising the `JoinSet` multi-future path in
//! [`Sandbox::drive_host_functions`] under real concurrency — not just the
//! single-future fast path that `poll_recv` covers. It also exercises the
//! cooperative scheduler waking two independent guest threads as their sockets
//! become readable at staggered times (vs. `poll_epoll`, which multiplexes two
//! sockets from a *single* thread via one epoll set).
//!
//! The two clients **send at staggered times** (port A gets its byte early, port
//! B much later), so worker A completes and its thread exits while worker B is
//! still parked — proving the two per-thread recvs are driven independently
//! across cooperative park/resume cycles. The guest sums the two received
//! integers (40 + 2 = 42) and exits with the sum, which we assert as the exit
//! code — proving each socket's payload reached its own thread intact.
//!
//! Kernel: unlike `poll_epoll`, this test needs its OWN kernel and does not fall
//! back to the `poll-recv-c` kernel. The guest spawns pthreads, so the kernel
//! must be built with `CONFIG_LIBPOSIX_PROCESS_MULTITHREADING=y` *and*
//! `CONFIG_LIBPOSIX_FUTEX=y` (musl's `pthread_create`/`pthread_join` coordinate
//! via the futex syscall) — options the `poll-recv-c` kernel does not enable.
//!
//! Like the other live tests, this needs a hypervisor and built artifacts; it
//! self-skips (with a diagnostic) when either is missing so `cargo test` still
//! passes on runners without KVM or a built kernel.

use core::task::Poll;
use hyperlight_unikraft::{ListenPorts, NetworkPolicy, Sandbox};
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Environment probe
// ---------------------------------------------------------------------------

/// The two ports the guest (`examples/poll-recv-2thread-c/poll.c`) binds and
/// listens on, one per worker thread. Baked into the guest, so the host
/// necessarily proxies these two ports. Distinct from `poll_recv` (34567) and
/// `poll_epoll` (34568/34569) so all the live net tests can run concurrently
/// without fighting over host ports.
const PORT_A: u16 = 34570;
const PORT_B: u16 = 34571;

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

/// Find the dedicated `poll-recv-2thread-c` ELF-loader kernel. Unlike
/// `poll_epoll`, we do NOT fall back to the `poll-recv-c` kernel: the guest
/// spawns pthreads, which requires a kernel built with multithreading + futex
/// support that `poll-recv-c` does not enable.
fn find_kernel() -> Option<PathBuf> {
    let build = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/poll-recv-2thread-c/.unikraft/build");
    if !build.is_dir() {
        return None;
    }
    std::fs::read_dir(&build).ok().and_then(|rd| {
        rd.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("_hyperlight-x86_64") && !n.ends_with(".dbg"))
                .unwrap_or(false)
        })
    })
}

fn find_initrd() -> Option<PathBuf> {
    let example_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/poll-recv-2thread-c");
    let initrd = example_dir.join("poll-recv-2thread-initrd.cpio");
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
            "SKIP: no poll-recv-2thread-c kernel found — build it under \
             examples/poll-recv-2thread-c/.unikraft/build/ via \
             `kraft-hyperlight build --plat hyperlight --arch x86_64` (needs \
             multithreading + futex; the poll-recv-c kernel is NOT a valid \
             fallback here)"
        );
        return None;
    };
    let Some(initrd) = find_initrd() else {
        eprintln!(
            "SKIP: poll-recv-2thread initrd missing — run `just rootfs` in \
             examples/poll-recv-2thread-c/ to build poll-recv-2thread-initrd.cpio"
        );
        return None;
    };
    Some((kernel, initrd))
}

/// Connect to `127.0.0.1:port`, retrying until the guest's host-proxied listener
/// is up, then (after `send_delay`) send `payload`. Runs on its own OS thread so
/// it can race the poll loop. The `send_delay` staggering between the two
/// clients is what forces worker A to complete and its thread to exit while
/// worker B is still parked in `recv()`.
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

/// Drive the poll-recv-2thread guest to completion. Two clients connect on the
/// two ports and send `40` and `2` at staggered times; two guest worker threads
/// each `recv()` their own socket, and main sums them and exits `42`. Asserts
/// the run reaches `Done`, that it yielded cooperatively (Wait/Idle) while both
/// threads were parked in `recv()`, and that the summed exit code is correct —
/// proving the host drove two concurrent async net futures and delivered each
/// payload to its own thread.
#[tokio::test]
async fn poll_recv_two_threads_reaches_done_under_kvm() {
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
    sbox.reset_poll_deadline();
    sbox.restore().expect("restore before first poll");

    const MAX_STEPS: usize = 4000;
    const MAX_WALL: Duration = Duration::from_secs(30);

    let start = Instant::now();
    let deadline = start + MAX_WALL;

    // Two clients race the poll loop on their own OS threads. Port A sends its
    // byte early; port B waits noticeably longer, so worker A finishes and its
    // thread exits while worker B is still parked in recv().
    let client_a = std::thread::spawn(move || {
        client_connect_and_send(PORT_A, b"40", Duration::from_millis(150), deadline)
    });
    let client_b = std::thread::spawn(move || {
        client_connect_and_send(PORT_B, b"2", Duration::from_millis(500), deadline)
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
                    "poll_recv_2thread reached Done after {steps} steps / {:?}",
                    start.elapsed()
                );
                done = true;
                break;
            }
            Poll::Pending => {
                saw_wait_or_idle = true;
                // Drive host-side async work (the two parked recv futures and the
                // inter-step socket-readiness wait) off the vCPU thread. Bound
                // the whole drive so a missed wakeup can't hang the run.
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
        "expected the guest to yield cooperatively (Wait/Idle) while its two \
         worker threads were parked in recv() — a blocking host call would never \
         have returned control to poll"
    );
    assert!(
        done,
        "poll-recv-2thread run did not reach Done within {steps} steps / {:?}; \
         both worker threads should have accepted a connection, recv'd their \
         payload, and main should have summed them and exited",
        start.elapsed()
    );
    // 40 + 2 = 42: each socket's payload reached its own worker thread intact.
    assert_eq!(
        sbox.last_exit_code(),
        42,
        "guest should exit with the sum of the two per-thread socket payloads \
         (40 + 2); got {} — a payload was lost or a worker thread failed (see \
         poll.c: 101=socket 102=bind 103=listen 104=accept 105=recv \
         106/107=pthread_create)",
        sbox.last_exit_code()
    );
}
