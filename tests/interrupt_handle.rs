// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! `SandboxBuilder::on_interrupt_handle`: the interrupt handle before
//! `boot` returns, so a guest that spins in its `resume` entry can be
//! stopped.
//!
//! The spinning guest is a python call parked on `recv` from a host
//! connection when it was snapshotted.  The connection dies with the old
//! host, so on the restored guest's `resume` entry `recv` returns EOF and
//! the call spins, whatever the clocks say.

mod common;

use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use common::require_rootfs;
use hyperlight_unikraft::hyperlight_host::HyperlightError;
use hyperlight_unikraft::hyperlight_host::hypervisor::InterruptHandle;
use hyperlight_unikraft::{
    AppSandbox, Error, ListenPorts, NetworkPolicy, SandboxBuilder, Snapshot, Yield,
};

type Handle = Arc<dyn InterruptHandle>;

fn python() -> SandboxBuilder {
    SandboxBuilder::from_initrd(require_rootfs("python")).scratch_mb(256)
}

/// A sandbox booted from `builder`, and a snapshot of it idle.
fn warm(builder: SandboxBuilder) -> (AppSandbox, Arc<Snapshot>) {
    let mut sandbox = builder.boot().unwrap();
    sandbox.run("print('warm')").unwrap();
    let idle = sandbox.snapshot().unwrap();
    (sandbox, idle)
}

/// [`warm`] with networking, and a snapshot of the sandbox in a call that
/// spins once the connection it waits on reads EOF.
fn snapshots(port: u16) -> (AppSandbox, Arc<Snapshot>, Arc<Snapshot>) {
    let (mut sandbox, idle) = warm(
        python()
            .network(NetworkPolicy::AllowAll)
            .listen_ports(ListenPorts::from_ports([port])),
    );

    sandbox
        .submit(format!(
            "import socket
s = socket.socket()
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(('0.0.0.0', {port}))
s.listen(1)
c, _ = s.accept()
s.close()
print('waiting', flush=True)
c.recv(1)
while True:
    pass
"
        ))
        .unwrap();
    let client = TcpStream::connect(("127.0.0.1", port)).unwrap();
    let started = Instant::now();
    let mut out = String::new();
    while !out.contains("waiting") {
        assert!(started.elapsed() < Duration::from_secs(30), "{out:?}");
        let y = sandbox.step(Duration::from_millis(50)).unwrap();
        assert!(matches!(y, Yield::Blocked { .. }), "{y:?}: {out:?}");
        out.push_str(&sandbox.drain_output());
    }
    // Parked on recv, with the client still connected.
    let spinning = sandbox.snapshot().unwrap();
    drop(client);
    (sandbox, idle, spinning)
}

/// How long an entry killed below may take to return.
const LIMIT: Duration = Duration::from_secs(60);

/// Run `entry` on its own thread while another kills through the handle
/// `handles` delivers, and return what `entry` returned, once the killer
/// has stopped.  A kill counts only once Hyperlight has started the guest
/// call, and one made while the vCPU is not running returns `false` yet
/// cancels it, so the killer keeps at it until one lands or `entry`
/// returns, and `entry`'s result says whether it worked.  An entry that
/// is not back within [`LIMIT`] fails the test rather than hold it.
fn while_killing<T: Send + 'static>(
    handles: mpsc::Receiver<Handle>,
    entry: impl FnOnce() -> T + Send + 'static,
) -> T {
    let done = Arc::new(AtomicBool::new(false));
    let killer = {
        let done = done.clone();
        thread::spawn(move || {
            // Wait for the handle only until `done`: an entry that never
            // delivers one (it failed first, or hangs before handing it
            // over) must not leave the join below waiting forever.
            let handle = loop {
                match handles.recv_timeout(Duration::from_millis(5)) {
                    Ok(handle) => break handle,
                    Err(mpsc::RecvTimeoutError::Timeout) if !done.load(Ordering::Acquire) => {}
                    Err(_) => return None,
                }
            };
            while !done.load(Ordering::Acquire) && !handle.kill() {
                thread::sleep(Duration::from_millis(5));
            }
            Some(handle)
        })
    };
    let (tx, rx) = mpsc::channel();
    let entry = thread::spawn(move || {
        let _ = tx.send(entry());
    });
    let result = rx.recv_timeout(LIMIT);
    // The killer stops whatever the entry did, so a failed test leaves no
    // thread killing behind it.
    done.store(true, Ordering::Release);
    let handle = killer.join().unwrap();
    match result {
        Ok(result) => result,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            // Kill for a while longer, so the entry left behind does not
            // spin on through the tests after this one.
            if let Some(handle) = handle {
                let started = Instant::now();
                while !handle.kill() && started.elapsed() < Duration::from_secs(1) {
                    thread::sleep(Duration::from_millis(5));
                }
            }
            panic!("not cancelled within {LIMIT:?}")
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            std::panic::resume_unwind(entry.join().unwrap_err())
        }
    }
}

fn is_cancelled(e: &Error) -> bool {
    matches!(
        e,
        Error::Hyperlight(HyperlightError::ExecutionCanceledByHost())
    )
}

#[test]
fn a_spinning_resume_is_cancelled_through_the_handle() {
    const PORT: u16 = 18101;
    let (sandbox, _, spinning) = snapshots(PORT);
    drop(sandbox);
    let dir = common::temp_dir("interrupt-resume");
    hyperlight_unikraft::save_snapshot(&spinning, &dir).unwrap();

    let builder = SandboxBuilder::from_snapshot_dir(&dir)
        .unwrap()
        .network(NetworkPolicy::AllowAll)
        .listen_ports(ListenPorts::from_ports([PORT]));
    let (tx, rx) = mpsc::channel();
    let slot: Arc<Mutex<Option<Handle>>> = Arc::default();
    let kept = slot.clone();
    let result = while_killing(rx, move || {
        builder
            .on_interrupt_handle(move |h| {
                *kept.lock().unwrap() = Some(h.clone());
                tx.send(h).unwrap();
            })
            .boot()
    });
    let err = result.err().expect("a killed resume booted");
    assert!(is_cancelled(&err), "{err:?}");
    // The failed boot freed the sandbox.
    let handle = slot.lock().unwrap().take().expect("not called during boot");
    assert!(handle.dropped(), "the cancelled sandbox is still alive");
}

/// `restore` and `restore_from` run their resume entry on a sandbox whose
/// handle the caller already holds; a kill fails them, and a later
/// restore recovers the sandbox.
#[test]
fn restore_is_cancelled_through_the_sandbox_handle() {
    const PORT: u16 = 18102;
    let (sandbox, idle, spinning) = snapshots(PORT);
    let spinning_dir = common::temp_dir("interrupt-restore-spinning");
    hyperlight_unikraft::save_snapshot(&spinning, &spinning_dir).unwrap();
    let idle_dir = common::temp_dir("interrupt-restore-idle");
    hyperlight_unikraft::save_snapshot(&idle, &idle_dir).unwrap();

    let mut sandbox = restore_killed(sandbox, move |s| s.restore(spinning));
    sandbox.restore(idle).unwrap();
    runs_again(&mut sandbox);

    let path = spinning_dir.path().to_path_buf();
    let mut sandbox = restore_killed(sandbox, move |s| s.restore_from(path));
    sandbox.restore_from(&idle_dir).unwrap();
    runs_again(&mut sandbox);
}

/// Run `restore` on `sandbox` while killing through its handle, check it
/// was cancelled, and hand the sandbox back.
fn restore_killed(
    mut sandbox: AppSandbox,
    restore: impl FnOnce(&mut AppSandbox) -> Result<(), Error> + Send + 'static,
) -> AppSandbox {
    let (tx, rx) = mpsc::channel();
    tx.send(sandbox.interrupt_handle()).unwrap();
    let (sandbox, result) = while_killing(rx, move || {
        let result = restore(&mut sandbox);
        (sandbox, result)
    });
    let err = result.expect_err("a killed resume restored");
    assert!(is_cancelled(&err), "{err:?}");
    sandbox
}

fn runs_again(sandbox: &mut AppSandbox) {
    sandbox.drain_output();
    sandbox.run("print('again')").unwrap();
    assert!(sandbox.drain_output().contains("again"));
}

#[test]
fn a_fresh_boot_hands_over_the_handle() {
    let slot: Arc<Mutex<Option<Handle>>> = Arc::default();
    let kept = slot.clone();
    let mut sandbox = python()
        .on_interrupt_handle(move |h| *kept.lock().unwrap() = Some(h))
        .boot()
        .unwrap();
    let handle = slot.lock().unwrap().take().expect("not called during boot");

    // The guest boots as without the option...
    sandbox.drain_output();
    sandbox.run("print('booted')").unwrap();
    assert!(sandbox.drain_output().contains("booted"));

    // ...and the handle is the sandbox's.
    let (tx, rx) = mpsc::channel();
    tx.send(handle).unwrap();
    let result = while_killing(rx, move || sandbox.run("while True:\n    pass\n"));
    let err = result.expect_err("a killed call returned");
    assert!(is_cancelled(&err), "{err:?}");
}

/// A boot from a snapshot with the option set runs as without it; a kill
/// made in `f`, before the `resume` entry's guest call starts, is dropped.
#[test]
fn a_boot_from_a_snapshot_runs_as_without_the_option() {
    let (sandbox, idle) = warm(python());
    drop(sandbox);
    let called = Arc::new(AtomicBool::new(false));
    let seen = called.clone();
    let mut sandbox = SandboxBuilder::from_snapshot(idle)
        .on_interrupt_handle(move |h| {
            // Nothing runs yet, so the kill is dropped.
            assert!(!h.kill(), "the vCPU was running before the resume entry");
            seen.store(true, Ordering::Release);
        })
        .boot()
        .unwrap();
    assert!(called.load(Ordering::Acquire), "not called during boot");
    sandbox.drain_output();
    sandbox.run("print('restored')").unwrap();
    assert!(sandbox.drain_output().contains("restored"));
}
