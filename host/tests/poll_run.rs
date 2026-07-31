//! Live cooperative-poll run under a real hypervisor (`/dev/kvm` on Linux).
//!
//! Unlike a single guest entry, which runs one pump iteration on one vCPU
//! entry, the cooperative *poll* model runs the unikernel scheduler only up
//! to the point it would go idle, then yields the vCPU back to the host
//! (a HALT) reporting the nanoseconds until its next scheduled wakeup. The
//! host re-enters ([`Sandbox::poll`]) to drive the guest forward. Guest memory —
//! including all scheduler/thread state — persists across the HALT/re-entry
//! boundary, so no `restore()` happens between steps.
//!
//! This test boots the dedicated `poll-c` guest, which sleeps a few short
//! intervals (each drives the scheduler idle with a pending timer) and then
//! returns from `main` (a normal `exit_group` → SYSHALT). Driving it with
//! [`Sandbox::poll`] should therefore:
//!   - observe one or more `Poll::Pending` steps with a pending timer
//!     ([`Sandbox::next_wakeup`] returns `Some`), i.e. the sleeps, then
//!   - observe `Poll::Ready(())` once the app exits,
//!
//! all within a bounded number of steps.
//!
//! Like `snapshot_roundtrip.rs`, this needs a hypervisor and the built
//! artifacts. It self-skips (with a diagnostic) when either is missing so
//! `cargo test` still passes on runners without KVM or a built poll kernel.
//!
//! Artifacts live in `examples/poll-c/`: the poll-enabled kernel at
//! `.unikraft/build/*_hyperlight-x86_64` and the initrd as `*-initrd.cpio`.
//! The kernel must be built with `CONFIG_HYPERLIGHT_POLL=y` (the example's
//! `kraft.yaml` sets it). Populate with `just rootfs` and a
//! `kraft-hyperlight build` in that directory.

use core::task::Poll;
use hyperlight_unikraft::Sandbox;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Environment probe
// ---------------------------------------------------------------------------

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

fn poll_artifacts() -> Option<(PathBuf, PathBuf)> {
    let example_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("examples/poll-c");
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
    let Some((kernel, initrd)) = poll_artifacts() else {
        eprintln!(
            "SKIP: poll-c artifacts missing under examples/poll-c/.unikraft/build/ \
             — run `just rootfs` then `kraft-hyperlight build --plat hyperlight \
             --arch x86_64` in examples/poll-c/ (kraft.yaml enables \
             CONFIG_HYPERLIGHT_POLL) to populate them"
        );
        return None;
    };
    Some((kernel, initrd))
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

/// Drive the poll-c guest to completion cooperatively and assert the poll
/// state machine behaves: it makes forward progress, reports at least one
/// pending-timer wait (the guest's sleeps), and terminates with `Done`
/// within a bounded number of steps — without a `restore()` between steps.
#[test]
fn poll_run_reaches_done_under_kvm() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .build()
        .expect("build sandbox");

    // Reset guest state once before the first step; the poll loop then runs
    // without further restores so scheduler/thread state persists across the
    // HALT/re-entry boundary.
    sbox.restore().expect("restore before first poll");

    // Bound the loop so a hang can't wedge the suite. The guest sleeps 3×10ms
    // then exits, so this is generous.
    const MAX_STEPS: usize = 200;
    const MAX_WALL: Duration = Duration::from_secs(30);
    // Cap how long we actually wait between steps so a bogus deadline can't
    // stall the test; the guest's TSC advances with wall-clock time either
    // way, so a shorter wait just means an extra (harmless) poll.
    const WAIT_CAP: Duration = Duration::from_millis(50);

    let start = Instant::now();
    let mut steps = 0usize;
    let mut saw_wait = false;
    let mut done = false;

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        steps += 1;
        match sbox.poll().expect("poll") {
            Poll::Ready(()) => {
                done = true;
                break;
            }
            Poll::Pending => match sbox.next_wakeup() {
                Some(d) => {
                    // The guest is parked on a timer (a sleep).
                    saw_wait = true;
                    std::thread::sleep(d.min(WAIT_CAP));
                }
                None => {
                    // No pending timer. In this closed test there is no external
                    // input source, so just yield briefly and re-poll.
                    std::thread::sleep(Duration::from_millis(1));
                }
            },
        }
    }

    assert!(
        done,
        "poll run did not reach Done within {steps} steps / {:?} \
         (saw_wait={saw_wait})",
        start.elapsed()
    );
    assert!(
        saw_wait,
        "expected at least one timer-parked poll (next_wakeup() == Some) from \
         the guest's nanosleep calls before it exited"
    );
    // A well-behaved cooperative run takes more than a single step: the guest
    // yields on each sleep and is re-entered. If it finished in one step the
    // sleeps weren't routed through the scheduler idle path.
    assert!(
        steps > 1,
        "expected multiple poll steps (guest should yield on each sleep), \
         got {steps}"
    );

    // Note: a plain C app exits via exit_group → SYSHALT and does *not* send
    // `__hl_exit`, so `last_exit_code()` isn't driven here (it stays at its
    // default). Termination is proven by reaching `Done` above, not by the
    // exit-code atomic — so we don't assert on it.
    let _ = sbox.last_exit_code();
}
