// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The cooperative step model, end to end, on a hypervisor.
//!
//! A guest built with the step model (the embedded kernel) runs its
//! scheduler only until it would go idle, then yields the vCPU back to
//! the host with the time until its next timer; the host waits and
//! re-enters with [`AppSandbox::step`].  Because every parked thread's
//! state lives in guest memory, a checkpoint taken between two steps can
//! be resumed -- in the same process or from disk -- and pick up exactly
//! where it left off, even in the middle of a dispatched call.
//!
//! These tests drive the Python rootfs (`just build-rootfs python`); the
//! entry-point test needs the Go rootfs and `just build-test-bins`, and
//! the server tests need networking on the host.  Like the other
//! hypervisor tests they need `/dev/kvm`.

use std::io::{Read as _, Write as _};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use hyperlight_unikraft::{
    AppSandbox, Error, ListenPorts, Mount, NetworkPolicy, SandboxBuilder, Yield,
};

mod common;

/// Log steps when `RUST_LOG` asks for them (e.g. `hyperlight_unikraft=debug`).
fn init_logs() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .with_writer(std::io::stderr)
        .try_init();
}

/// Bound on steps and wall time for the short runs below.
const MAX_STEPS: usize = 500;
const MAX_WALL: Duration = Duration::from_secs(60);

/// A script that prints a line, sleeps `ticks` × `ms`, and finishes.
fn tick_script(ticks: u32, ms: u32) -> String {
    format!(
        "import time
print('start', flush=True)
for i in range({ticks}):
    time.sleep({ms} / 1000)
    print(f'tick {{i}}', flush=True)
print('done', flush=True)
"
    )
}

/// Step the sandbox until the in-flight call completes or the guest exits,
/// or until `stop(output_so_far)` says to pause at the current boundary.
/// Returns the accumulated output, the number of steps that ended on a
/// guest timer, and whether the call finished.
fn drive(sandbox: &mut AppSandbox, mut stop: impl FnMut(&str) -> bool) -> (String, usize, bool) {
    let start = Instant::now();
    let mut out = String::new();
    let mut timers = 0;
    for _ in 0..MAX_STEPS {
        assert!(
            start.elapsed() < MAX_WALL,
            "step run did not finish in time; output so far: {out:?}"
        );
        let y = sandbox.step(Duration::from_millis(50)).expect("step");
        out.push_str(&sandbox.drain_output());
        match y {
            Yield::CallDone | Yield::CallFailed { .. } | Yield::Exited { .. } => {
                return (out, timers, true);
            }
            Yield::Blocked { until: Some(_) } => timers += 1,
            Yield::Blocked { until: None } => {}
        }
        if stop(&out) {
            return (out, timers, false);
        }
    }
    panic!("step run exceeded {MAX_STEPS} steps; output so far: {out:?}");
}

/// Save `sandbox`'s snapshot under `dir` and drop the sandbox, as a
/// checkpoint into another process would.
fn checkpoint_to(mut sandbox: AppSandbox, dir: impl AsRef<std::path::Path>) {
    sandbox.snapshot_to(dir).expect("snapshot");
    drop(sandbox);
}

fn restore_from(dir: impl AsRef<std::path::Path>) -> SandboxBuilder {
    SandboxBuilder::from_snapshot_dir(dir).expect("load")
}

fn boot_python() -> AppSandbox {
    init_logs();
    SandboxBuilder::from_initrd(common::require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .expect("boot")
}

/// The whole yield / re-enter / complete cycle for a dispatched call: the
/// guest parks on each wait instead of spinning, the host sees timer
/// yields, and the call is reported done once the script returns.  Wall
/// time covers the waits, so the vCPU really was handed back.
#[test]
fn python_call_yields_on_timers() {
    let mut sandbox = boot_python();
    assert!(sandbox.has_driver());
    let t = Instant::now();
    sandbox.submit(tick_script(5, 20)).expect("submit");
    let mut out = sandbox.drain_output();
    let (rest, timers, done) = drive(&mut sandbox, |_| false);
    out.push_str(&rest);
    assert!(done, "{out:?}");
    assert!(
        timers >= 1,
        "expected timer yields from the script's waits, got {timers}"
    );
    assert!(
        t.elapsed() >= Duration::from_millis(100),
        "returned before the waits elapsed"
    );
    for line in ["start", "tick 0", "tick 4", "done"] {
        assert!(out.contains(line), "missing {line:?} in {out:?}");
    }
    // The guest is alive and takes the next call.
    sandbox.run("print('again')").expect("second run");
    assert!(sandbox.drain_output().contains("again"));
}

/// A call is only limited by the host's I/O buffer, which the kernel reads
/// from the PEB and the driver learns through the device: a script far
/// larger than any guest-side constant still runs.  (The previous kernel
/// rejected anything over 16 KiB.)
#[test]
fn python_large_call_payload() {
    let mut sandbox = boot_python();
    let mut script = String::new();
    for i in 0..2500 {
        script.push_str(&format!("x{i:05} = {i}\n"));
    }
    script.push_str("print('big script ran', x02499)\n");
    assert!(script.len() > 32 * 1024, "{} bytes", script.len());
    sandbox.run(script).expect("run");
    let out = sandbox.drain_output();
    assert!(out.contains("big script ran 2499"), "{out:?}");
}

/// `join` on a driver image with nothing submitted can never return, so
/// it refuses up front; `run` on a guest that dies mid-call reports it.
#[test]
fn python_join_and_exit_are_reported() {
    let mut sandbox = boot_python();
    let err = sandbox
        .join()
        .expect_err("join with an idle driver must fail");
    assert!(matches!(err, Error::NothingToJoin), "{err}");

    // The driver process exits inside the call: the guest is gone.
    let err = sandbox
        .run("import os; print('bye', flush=True); os._exit(0)")
        .expect_err("run must report the guest exiting");
    assert!(matches!(err, Error::GuestExited { status: 0 }), "{err}");
    assert!(sandbox.drain_output().contains("bye"));
    assert_eq!(
        sandbox.step(Duration::ZERO).expect("step"),
        Yield::Exited { status: 0 }
    );
    assert!(sandbox.run("print('never')").is_err());
    // An exited guest joins at once, with the status it exited with.
    assert_eq!(sandbox.join().expect("join after exit"), 0);
}

/// The kernel serves one call at a time: while one is in flight, `submit`
/// and `run` are refused up front, and the guest is untouched.
#[test]
fn python_second_call_is_refused_while_one_is_in_flight() {
    let mut sandbox = boot_python();
    sandbox.submit(tick_script(2, 100)).expect("submit");
    assert!(matches!(
        sandbox.submit("print('second')"),
        Err(Error::CallInFlight)
    ));
    assert!(matches!(
        sandbox.run("print('second')"),
        Err(Error::CallInFlight)
    ));
    let (out, _, done) = drive(&mut sandbox, |_| false);
    assert!(done, "{out:?}");
    assert!(out.contains("done") && !out.contains("second"), "{out:?}");
    sandbox.run("print('second')").expect("run after the call");
    assert!(sandbox.drain_output().contains("second"));
}

/// `time.sleep` must park the guest: the host waits with the VM halted and
/// the call takes the requested time.  (CPython sleeps with
/// `clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME)`, which the kernel's
/// posix-time now implements.)
#[test]
fn python_time_sleep_parks_the_guest() {
    let mut sandbox = boot_python();
    let t = Instant::now();
    sandbox
        .submit(
            "import time; t = time.monotonic(); time.sleep(2); \
             print('slept', round(time.monotonic() - t, 1), flush=True)",
        )
        .expect("submit");
    let (out, timers, done) = drive(&mut sandbox, |_| false);
    assert!(done, "{out:?}");
    assert!(timers >= 1, "the sleep never yielded a timer: {out:?}");
    assert!(
        t.elapsed() >= Duration::from_millis(1800),
        "sleep returned after {:?}: {out:?}",
        t.elapsed()
    );
    assert!(out.contains("slept 2."), "{out:?}");
}

/// A driver process that really exits (`os._exit(3)`, which skips the
/// interpreter's own exit handling) ends the guest: the call fails and
/// the host learns the status.  (`sys.exit()` is not that: it ends the
/// call with its code and leaves the interpreter up; see tests/python.rs.)
#[test]
fn python_os_exit_ends_the_guest() {
    let mut sandbox = boot_python();
    let err = sandbox
        .run("import os; print('leaving', flush=True); os._exit(3)")
        .expect_err("a process exit must fail the call");
    assert!(matches!(err, Error::GuestExited { status: 3 }), "{err}");
    // Nothing is left to snapshot.
    assert!(
        matches!(sandbox.snapshot(), Err(Error::GuestExited { status: 3 })),
        "a snapshot of an exited guest was taken"
    );
    assert!(sandbox.drain_output().contains("leaving"));
    assert_eq!(
        sandbox.step(Duration::ZERO).expect("step"),
        Yield::Exited { status: 3 }
    );
}

/// An uncaught exception must fail the call too, and leave the guest
/// alive for the next one.
#[test]
fn python_uncaught_exception_fails_the_call() {
    let mut sandbox = boot_python();
    let err = sandbox
        .run("raise RuntimeError('boom')")
        .expect_err("an uncaught exception must fail the call");
    assert!(matches!(err, Error::CallFailed { status: 1 }), "{err}");
    assert!(sandbox.drain_output().contains("RuntimeError"));
    // The same through submit/step: the report says failed, not done.
    sandbox.submit("1/0").expect("submit");
    let mut saw_failed = false;
    let out = loop {
        match sandbox.step(Duration::from_millis(50)).expect("step") {
            Yield::CallFailed { status } => {
                assert_eq!(
                    status, 1,
                    "an uncaught exception is status 1, as the script's exit"
                );
                saw_failed = true;
                break sandbox.drain_output();
            }
            Yield::CallDone => break sandbox.drain_output(),
            Yield::Exited { .. } => panic!("guest exited: {:?}", sandbox.drain_output()),
            Yield::Blocked { .. } => {}
        }
    };
    assert!(saw_failed, "expected CallFailed; output: {out:?}");
    assert!(out.contains("ZeroDivisionError"), "{out:?}");
    sandbox.run("print('still here')").expect("next call");
    assert!(sandbox.drain_output().contains("still here"));
}

/// A guest whose every thread is blocked with no timer and no host socket
/// can never make progress; `run` must say so rather than wait forever.
#[test]
fn python_deadlock_is_reported() {
    let mut sandbox = boot_python();
    let t = Instant::now();
    let err = sandbox
        .run("import threading; print('parking', flush=True); threading.Event().wait()")
        .expect_err("a deadlocked guest must be reported");
    assert!(matches!(err, Error::Deadlocked), "{err}");
    assert!(
        t.elapsed() < Duration::from_secs(5),
        "took {:?}",
        t.elapsed()
    );
    assert!(sandbox.drain_output().contains("parking"));
}

/// Data arriving on a socket no guest thread is reading must not turn the
/// step loop into a busy spin: the host watches a socket for readability
/// only while the guest is parked on it, so a server that sleeps with an
/// unread request costs one step per timer, not thousands.
#[test]
fn python_unread_socket_data_does_not_spin() {
    init_logs();
    let rootfs = common::require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(&rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([18085]))
        .boot()
        .expect("boot");
    sandbox
        .submit(
            "import socket, time\n\
             s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)\n\
             s.bind(('0.0.0.0', 18085)); s.listen(1)\n\
             c, _ = s.accept(); print('accepted', flush=True)\n\
             time.sleep(2)\n\
             print('read', len(c.recv(1024)), flush=True)",
        )
        .expect("submit");
    use std::io::Write as _;
    let mut client = std::net::TcpStream::connect("127.0.0.1:18085").expect("connect");
    client.write_all(b"hello").expect("send");
    let t = Instant::now();
    let mut steps = 0u32;
    loop {
        match sandbox.step(Duration::from_millis(200)).expect("step") {
            Yield::Blocked { .. } => {}
            other => {
                assert_eq!(other, Yield::CallDone);
                break;
            }
        }
        steps += 1;
        assert!(
            t.elapsed() < Duration::from_secs(10),
            "server never finished"
        );
    }
    assert!(sandbox.drain_output().contains("read 5"));
    eprintln!("{steps} steps for the 2 s sleep");
    assert!(
        steps < 40,
        "{steps} steps for a 2 s sleep: the host is spinning on the unread socket"
    );
}

/// A connect to a peer that never answers must not hold the host: the
/// handshake runs in the background, the guest thread parks, and the
/// guest's own timeout decides.  192.0.2.1 (TEST-NET-1) is unroutable or
/// refused depending on the network; either way the host returns fast.
#[test]
fn python_connect_to_unreachable_peer_does_not_hold_the_host() {
    init_logs();
    let rootfs = common::require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(&rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .expect("boot");
    let t = Instant::now();
    sandbox
        .run(concat!(
            "import socket, time\n",
            "t = time.time()\n",
            "try:\n",
            "    socket.create_connection(('192.0.2.1', 80), timeout=1)\n",
            "except OSError as e:\n",
            "    print('connect:', type(e).__name__, round(time.time() - t, 1), 's', flush=True)\n",
        ))
        .expect("run");
    let out = sandbox.drain_output();
    assert!(out.contains("connect:"), "{out:?}");
    assert!(
        t.elapsed() < Duration::from_secs(8),
        "host held for {:?}: {out:?}",
        t.elapsed()
    );
}

/// A call ends when its main code returns.  Threads it started, daemon
/// or not, do not hold it: they are left behind and make progress on the
/// steps that follow.  (Unlike `python file.py`, which joins non-daemon
/// threads at exit -- after running exit callbacks that stop thread pools,
/// which a live interpreter cannot do.)
#[test]
fn python_threads_do_not_extend_the_call() {
    let mut sandbox = boot_python();
    let t = Instant::now();
    sandbox
        .run(concat!(
            "import threading, time\n",
            "def late():\n",
            "    time.sleep(0.5)\n",
            "    print('thread done', flush=True)\n",
            "threading.Thread(target=late).start()\n",
            "print('main done', flush=True)\n",
        ))
        .expect("run");
    assert!(
        t.elapsed() < Duration::from_millis(400),
        "call waited for the thread"
    );
    assert_eq!(sandbox.drain_output(), "main done\r\n");
    while t.elapsed() < Duration::from_millis(1200) {
        sandbox.step(Duration::from_millis(500)).expect("step");
    }
    assert_eq!(sandbox.drain_output(), "thread done\r\n");
}

/// A module-level thread pool keeps idle non-daemon workers alive.  The
/// call must still return: a driver that joined non-daemon threads would
/// wait for them forever, and the host would report a deadlock.
#[test]
fn python_thread_pool_does_not_hold_the_call() {
    let mut sandbox = boot_python();
    sandbox
        .run(concat!(
            "from concurrent.futures import ThreadPoolExecutor\n",
            "ex = ThreadPoolExecutor()\n",
            "print('result', ex.submit(lambda: 21 * 2).result(), flush=True)\n",
        ))
        .expect("run");
    assert_eq!(sandbox.drain_output(), "result 42\r\n");
    sandbox
        .run("print('again', ex.submit(lambda: 1).result(), flush=True)")
        .expect("run");
    assert_eq!(sandbox.drain_output(), "again 1\r\n");
}

/// A Python thread a call leaves behind keeps running between calls: the
/// driver parks with the interpreter lock released.  Before, the thread
/// starved until the next call and woke the guest every 5 ms asking for
/// the lock, so the steps here would number in the hundreds.
#[test]
fn python_thread_left_behind_runs_between_calls() {
    let mut sandbox = boot_python();
    sandbox
        .run(concat!(
            "import threading, time\n",
            "def tick():\n",
            "    for i in range(4):\n",
            "        print('tick', i, flush=True)\n",
            "        time.sleep(0.2)\n",
            "threading.Thread(target=tick, daemon=True).start()\n",
        ))
        .expect("run");
    let t = Instant::now();
    let mut steps = 0;
    while t.elapsed() < Duration::from_millis(1500) {
        sandbox.step(Duration::from_millis(500)).expect("step");
        steps += 1;
    }
    let out = sandbox.drain_output();
    assert!(
        out.contains("tick 3"),
        "thread did not run while parked: {out:?}"
    );
    assert!(steps < 20, "guest woke {steps} times in 1.5 s: {out:?}");
}

/// A thread a call left behind is guest state like any other: a snapshot
/// taken while it sleeps carries it, and it goes on ticking in the
/// restored guest.
#[test]
fn python_thread_left_behind_survives_restore() {
    let mut sandbox = boot_python();
    sandbox
        .run(concat!(
            "import threading, time\n",
            "def tick():\n",
            "    for i in range(6):\n",
            "        print('tick', i, flush=True)\n",
            "        time.sleep(0.3)\n",
            "threading.Thread(target=tick, daemon=True).start()\n",
        ))
        .expect("run");
    let t = Instant::now();
    while t.elapsed() < Duration::from_millis(500) {
        sandbox.step(Duration::from_millis(500)).expect("step");
    }
    let before = sandbox.drain_output();
    assert!(before.contains("tick 1"), "{before:?}");
    let dir = common::temp_dir("step-thread-restore");
    checkpoint_to(sandbox, &dir);

    let mut sandbox = restore_from(&dir).boot().expect("restore");
    let t = Instant::now();
    while t.elapsed() < Duration::from_millis(1800) {
        sandbox.step(Duration::from_millis(500)).expect("step");
    }
    let after = sandbox.drain_output();
    assert!(
        after.contains("tick 5"),
        "thread did not go on after the restore: {after:?}"
    );
}

/// A guest restored from disk must report the current time, not the time
/// it was checkpointed at: the kernel asks the host for the wall clock
/// again on its resume entry.
#[test]
fn python_restored_guest_clock_is_current() {
    let sandbox = boot_python();
    let dir = common::temp_dir("step-clock");
    checkpoint_to(sandbox, &dir);
    std::thread::sleep(Duration::from_secs(3));

    let mut sandbox = restore_from(&dir).boot().expect("restore");
    sandbox
        .run("import time; print('T', time.time())")
        .expect("run");
    let out = sandbox.drain_output();
    let guest: f64 = out
        .lines()
        .find_map(|l| l.strip_prefix("T "))
        .and_then(|t| t.trim().parse().ok())
        .unwrap_or_else(|| panic!("no timestamp in {out:?}"));
    let host = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    assert!(
        (host - guest).abs() < 1.5,
        "guest clock off by {:.2} s after a restore (guest {guest}, host {host})",
        host - guest
    );
}

/// Two guests restored from one checkpoint start from the same CSPRNG
/// state, so unless the kernel reseeds on restore they draw the same
/// "random" bytes.  The host announces a restore with a `resume` entry in
/// place of the first `step`, and the kernel reseeds there.
#[test]
fn python_restored_clones_draw_different_random_bytes() {
    let mut sandbox = boot_python();
    sandbox.run("import os").expect("warm up");
    let dir = common::temp_dir("step-random");
    checkpoint_to(sandbox, &dir);

    let draw = |label: &str| -> String {
        let mut clone = restore_from(&dir).boot().expect("restore");
        clone
            .run("import os; print('rand', os.urandom(16).hex(), flush=True)")
            .expect("run");
        let out = clone.drain_output();
        out.lines()
            .find_map(|l| l.strip_prefix("rand "))
            .map(|h| h.trim().to_string())
            .unwrap_or_else(|| panic!("{label}: no random line in {out:?}"))
    };
    let first = draw("first clone");
    let second = draw("second clone");
    assert_ne!(
        first, second,
        "two guests restored from the same checkpoint drew identical random bytes"
    );
}

/// A checkpoint taken at a step boundary *in the middle of a call* resumes
/// that call: the restored guest's parked interpreter wakes, prints the
/// ticks it had left, and reports the call done.
#[test]
fn python_checkpoint_resumes_mid_call() {
    let mut sandbox = boot_python();
    sandbox.submit(tick_script(3, 30)).expect("submit");
    sandbox.drain_output();

    // Run until the first tick has printed, then stop at that boundary.
    let (head, _, done) = drive(&mut sandbox, |out| out.contains("tick 0"));
    assert!(!done, "call finished before we could checkpoint: {head:?}");
    assert!(!head.contains("tick 1"), "ran past the boundary: {head:?}");
    let snapshot = sandbox.snapshot().expect("snapshot");

    // The original finishes on its own...
    let (tail, _, done) = drive(&mut sandbox, |_| false);
    assert!(done);
    assert!(
        tail.contains("tick 1") && tail.contains("tick 2") && tail.contains("done"),
        "{tail:?}"
    );

    // ...and so does a fresh sandbox restored from the mid-call checkpoint,
    // picking up exactly where the first one was: no repeat of the ticks
    // that already happened, and the call completes there too.
    let mut restored = SandboxBuilder::from_snapshot(snapshot)
        .boot()
        .expect("restore");
    let (rest, timers, done) = drive(&mut restored, |_| false);
    assert!(done, "restored guest never completed the call: {rest:?}");
    assert!(
        timers >= 1,
        "restored guest never parked on its remaining waits: {rest:?}"
    );
    assert!(
        !rest.contains("start") && !rest.contains("tick 0"),
        "restored guest restarted: {rest:?}"
    );
    assert!(
        rest.contains("tick 1") && rest.contains("tick 2") && rest.contains("done"),
        "{rest:?}"
    );
    restored.run("print('again')").expect("run after restore");
    assert!(restored.drain_output().contains("again"));
}

/// The same, through the on-disk layout: what a checkpoint/restore across
/// two host processes does.
#[test]
fn python_checkpoint_round_trips_through_disk_mid_call() {
    let mut sandbox = boot_python();
    sandbox.submit(tick_script(3, 30)).expect("submit");
    sandbox.drain_output();
    let (_, _, done) = drive(&mut sandbox, |out| out.contains("tick 1"));
    assert!(!done);
    let dir = common::temp_dir("step-python");
    checkpoint_to(sandbox, &dir);

    let mut restored = restore_from(&dir).boot().expect("restore");
    let (rest, _, done) = drive(&mut restored, |_| false);
    assert!(done, "restored guest never completed the call: {rest:?}");
    assert!(
        !rest.contains("tick 1"),
        "restored guest repeated work: {rest:?}"
    );
    assert!(rest.contains("tick 2") && rest.contains("done"), "{rest:?}");
}

// ── Long-running servers ────────────────────────────────────────────

/// One HTTP GET against the host-proxied listener; the response body.
fn http_get(port: u16, path: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    write!(s, "GET {path} HTTP/1.0\r\nHost: localhost\r\n\r\n").unwrap();
    let mut buf = String::new();
    let _ = s.read_to_string(&mut buf);
    buf.split("\r\n\r\n").nth(1).unwrap_or("").to_string()
}

/// Step `sandbox` on this thread while `f` talks to it from another; `f`'s
/// result is returned once it finishes.  The guest is left parked at a
/// step boundary.
fn while_serving<T: Send + 'static>(
    sandbox: &mut AppSandbox,
    f: impl FnOnce() -> T + Send + 'static,
) -> T {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    let start = Instant::now();
    loop {
        if let Ok(v) = rx.try_recv() {
            return v;
        }
        assert!(
            start.elapsed() < MAX_WALL,
            "client did not finish; output: {:?}",
            sandbox.drain_output()
        );
        match sandbox.step(Duration::from_millis(20)).expect("step") {
            Yield::Exited { status } => {
                panic!(
                    "server exited with {status}; output: {:?}",
                    sandbox.drain_output()
                )
            }
            Yield::CallFailed { status } => {
                panic!(
                    "server failed with {status}; output: {:?}",
                    sandbox.drain_output()
                )
            }
            Yield::CallDone | Yield::Blocked { .. } => {}
        }
    }
}

/// A Python HTTP server counting requests in memory, dispatched with
/// `Exec` and therefore still *inside* that call when the guest is
/// checkpointed: the request count must carry over to a guest restored
/// from disk in a fresh sandbox, which binds its listener again itself.
#[test]
fn python_http_server_survives_checkpoint_restore() {
    init_logs();
    let rootfs = common::require_rootfs("python");
    const PORT: u16 = 18089;
    let boot = || {
        SandboxBuilder::from_initrd(&rootfs)
            .scratch_mb(256)
            .network(NetworkPolicy::AllowAll)
            .listen_ports(ListenPorts::from_ports([PORT]))
    };
    let mut sandbox = boot().boot().expect("boot");
    assert!(
        sandbox.has_driver(),
        "python rootfs should report a driver at boot"
    );

    let server = format!(
        r#"
import http.server, socketserver
class H(http.server.BaseHTTPRequestHandler):
    n = 0
    def do_GET(self):
        H.n += 1
        body = f"count={{H.n}}".encode()
        self.send_response(200); self.send_header("Content-Length", str(len(body))); self.end_headers()
        self.wfile.write(body)
    def log_message(self, *a): pass
socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(("0.0.0.0", {PORT}), H) as s:
    print("serving", flush=True)
    s.serve_forever()
"#
    );
    // The server never returns, so run() would never come back: submit it
    // and keep stepping by hand.
    sandbox.submit(server).expect("submit");

    let body = while_serving(&mut sandbox, || http_get(PORT, "/"));
    assert_eq!(
        body,
        "count=1",
        "first request; output: {:?}",
        sandbox.drain_output()
    );
    let body = while_serving(&mut sandbox, || http_get(PORT, "/"));
    assert_eq!(body, "count=2");

    // Snapshot to disk: guest memory as an OCI layout, nothing else.  The
    // host listener lives in the sandbox's socket table, so the sandbox
    // must go before the port can be bound again (in a real checkpoint the
    // whole process is gone).
    let dir = common::temp_dir("step-http");
    checkpoint_to(sandbox, &dir);

    // Restore in a fresh sandbox: the guest binds its listener again on
    // its resume entry, and the third request continues the count.
    let mut restored = restore_from(&dir)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT]))
        .boot()
        .expect("restore");
    let body = while_serving(&mut restored, || http_get(PORT, "/"));
    assert_eq!(
        body,
        "count=3",
        "restored guest lost its state; output: {:?}",
        restored.drain_output()
    );
}

/// The connections a server had open at checkpoint time do not survive
/// the process, and after a restore the host has no socket for them.  A
/// single-threaded server parked in `recv` on such a connection must see
/// it closed and get back to `accept`, or it never serves another request.
/// This keeps an HTTP/1.1 keep-alive connection open across the checkpoint
/// and asserts the restored server still answers a new one.
#[test]
fn python_http_server_recovers_from_dead_keepalive_connection() {
    let rootfs = common::require_rootfs("python");
    init_logs();
    const PORT: u16 = 18092;
    let boot = || {
        SandboxBuilder::from_initrd(&rootfs)
            .scratch_mb(256)
            .network(NetworkPolicy::AllowAll)
            .listen_ports(ListenPorts::from_ports([PORT]))
    };
    let mut sandbox = boot().boot().expect("boot");
    let server = format!(
        r#"
import http.server, socketserver
class H(http.server.BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"
    n = 0
    def do_GET(self):
        H.n += 1
        body = f"count={{H.n}}".encode()
        self.send_response(200); self.send_header("Content-Length", str(len(body))); self.end_headers()
        self.wfile.write(body)
    def log_message(self, *a): pass
socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(("0.0.0.0", {PORT}), H) as s:
    print("serving", flush=True)
    s.serve_forever()
"#
    );
    sandbox.submit(server).expect("submit");

    // One keep-alive request; the connection stays open and the server's
    // only thread parks in recv() waiting for the next request on it.
    let keepalive = while_serving(&mut sandbox, || {
        let mut s = TcpStream::connect(("127.0.0.1", PORT)).expect("connect");
        s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        write!(s, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        // Headers and body may arrive in separate segments; read until the
        // body is in (the connection stays open, so no EOF to wait for).
        let mut text = String::new();
        let mut buf = [0u8; 4096];
        while !text.ends_with("count=1") {
            let n = s.read(&mut buf).expect("read");
            assert!(n > 0, "server closed the keep-alive connection: {text:?}");
            text.push_str(&String::from_utf8_lossy(&buf[..n]));
        }
        s
    });

    let snapshot = sandbox.snapshot().expect("snapshot");
    drop(sandbox);
    drop(keepalive);

    // The accepted connection's host end is gone; the restored guest must
    // see it as closed and get back to accept().
    let mut restored = SandboxBuilder::from_snapshot(snapshot)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT]))
        .boot()
        .expect("restore");
    let body = while_serving(&mut restored, || http_get(PORT, "/"));
    assert_eq!(
        body,
        "count=2",
        "restored server did not recover; output: {:?}",
        restored.drain_output()
    );
}

/// A guest blocked on a *write* must wake when the peer drains the socket.
/// The server answers with far more than the host socket buffers hold, to
/// a client that starts reading only after a delay, so the server's only
/// thread parks on writability with no timer and no readable socket in
/// sight.  Only the host watching writability can get it going again.
#[test]
fn python_server_large_response_to_slow_reader() {
    let rootfs = common::require_rootfs("python");
    init_logs();
    const PORT: u16 = 18093;
    const BODY: usize = 16 << 20;
    let mut sandbox = SandboxBuilder::from_initrd(&rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT]))
        .boot()
        .expect("boot");
    let server = [
        "import socket",
        "s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)",
        &format!("s.bind(('0.0.0.0', {PORT})); s.listen(1)"),
        "print('serving', flush=True)",
        "c, _ = s.accept()",
        "c.recv(64)",
        &format!("c.sendall(b'x' * {BODY})"),
        "c.close(); s.close()",
        "print('sent', flush=True)",
    ]
    .join("\n");
    sandbox.submit(server).expect("submit");

    let received = while_serving(&mut sandbox, || {
        let mut s = TcpStream::connect(("127.0.0.1", PORT)).expect("connect");
        s.set_read_timeout(Some(Duration::from_secs(20))).unwrap();
        s.write_all(b"go\n").unwrap();
        // Let the server fill every buffer between us and park on
        // writability before we read a single byte.
        std::thread::sleep(Duration::from_secs(1));
        let mut buf = Vec::new();
        let _ = s.read_to_end(&mut buf);
        buf.len()
    });
    assert_eq!(received, BODY, "output: {:?}", sandbox.drain_output());
    // The server finished and closed while the client was still reading,
    // so its call completed inside `while_serving`; the guest is idle and
    // takes the next call.
    assert!(sandbox.drain_output().contains("sent"));
    sandbox.run("print('again')").expect("next call");
    assert!(sandbox.drain_output().contains("again"));
}

/// CPU time consumed so far by the calling thread, from procfs.
#[cfg(target_os = "linux")]
fn thread_cpu_time() -> Duration {
    let stat = std::fs::read_to_string("/proc/thread-self/stat").expect("procfs");
    // Fields after the parenthesised command name; utime and stime are the
    // 14th and 15th fields overall, in USER_HZ (always 100) ticks.
    let rest = &stat[stat.rfind(')').expect("stat format") + 2..];
    let f: Vec<&str> = rest.split_whitespace().collect();
    let ticks: u64 = f[11].parse::<u64>().unwrap() + f[12].parse::<u64>().unwrap();
    Duration::from_millis(ticks * 10)
}

/// The point of the model: a server with nothing to do costs the host
/// nothing.  Driving an idle HTTP server for a second must take only a
/// handful of steps and almost no CPU on the driving thread, because the
/// host parks in `poll(2)` on the guest's timer and sockets instead of the
/// guest spinning in its idle loop.
#[cfg(target_os = "linux")]
#[test]
fn idle_server_costs_no_cpu() {
    let rootfs = common::require_rootfs("python");
    init_logs();
    const PORT: u16 = 18091;
    let mut sandbox = SandboxBuilder::from_initrd(&rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT]))
        .boot()
        .expect("boot");
    // Built line by line: a `\`-continued string literal would strip the
    // indentation Python needs for the `with` body.
    let server = [
        "import socketserver, http.server",
        "socketserver.TCPServer.allow_reuse_address = True",
        &format!("with socketserver.TCPServer(('0.0.0.0', {PORT}), http.server.BaseHTTPRequestHandler) as s:"),
        "    s.serve_forever(poll_interval=0.5)",
    ]
    .join("\n");
    sandbox.submit(server).expect("submit");

    let wall = Duration::from_secs(1);
    let cpu_before = thread_cpu_time();
    let start = Instant::now();
    let mut steps = 0;
    while start.elapsed() < wall {
        let y = sandbox
            .step(wall.saturating_sub(start.elapsed()))
            .expect("step");
        assert!(
            matches!(y, Yield::Blocked { .. }),
            "server stopped: {y:?}, output {:?}",
            sandbox.drain_output()
        );
        steps += 1;
    }
    let cpu = thread_cpu_time() - cpu_before;

    // serve_forever wakes every 0.5 s, so a second of idling is two or
    // three steps; each is microseconds of host work.
    assert!(
        steps <= 6,
        "idle server was stepped {steps} times in {wall:?}"
    );
    assert!(
        cpu < Duration::from_millis(200),
        "idle server cost {cpu:?} of CPU in {wall:?}"
    );
}

/// A plain Linux binary as the guest entry point — no driver, no `Exec`:
/// the Go counter server from `examples/go` boots under the pump, answers
/// HTTP, and keeps its in-memory request count across a checkpoint into a
/// fresh sandbox.
#[test]
fn go_counter_entry_survives_checkpoint_restore() {
    init_logs();
    let rootfs = common::require_rootfs("go");
    let bins = common::require_bin("go", "counter");
    const PORT: u16 = 18094;
    let boot = || {
        SandboxBuilder::from_initrd(&rootfs)
            .mount(Mount::ro(&bins, common::BIN_MOUNT))
            .entry(format!("{}/counter -port {PORT}", common::BIN_MOUNT))
            .scratch_mb(256)
            .network(NetworkPolicy::AllowAll)
            .listen_ports(ListenPorts::from_ports([PORT]))
    };
    let mut sandbox = boot().boot().expect("boot");
    assert!(
        !sandbox.has_driver(),
        "a plain entry binary must not count as a driver"
    );
    assert!(
        matches!(sandbox.submit("x"), Err(Error::NoDriver)),
        "a driverless guest must refuse calls"
    );

    // Boot returned at the server's first idle; it should already be up.
    let ready = while_serving(&mut sandbox, || http_get(PORT, "/readyz"));
    assert_eq!(ready.trim(), "ok", "output: {:?}", sandbox.drain_output());
    let body = while_serving(&mut sandbox, || http_get(PORT, "/"));
    assert!(body.contains("count: 1"), "{body:?}");
    let body = while_serving(&mut sandbox, || http_get(PORT, "/"));
    assert!(body.contains("count: 2"), "{body:?}");

    let dir = common::temp_dir("step-counter");
    checkpoint_to(sandbox, &dir);

    let mut restored = restore_from(&dir)
        .mount(Mount::ro(&bins, common::BIN_MOUNT))
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT]))
        .boot()
        .expect("restore");
    let body = while_serving(&mut restored, || http_get(PORT, "/"));
    assert!(
        body.contains("count: 3"),
        "restored guest lost its state: {body:?}; output: {:?}",
        restored.drain_output()
    );
}
