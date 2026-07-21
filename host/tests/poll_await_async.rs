//! Live cooperative-poll run exercising the **async host-function tools** API
//! (`SandboxBuilder::tool_async` + `Sandbox::poll` + `drive_host_functions`)
//! under a real hypervisor (`/dev/kvm` on Linux).
//!
//! This proves the framework transparently drives a plain Rust `async` handler
//! across the guest's cooperative park/resume:
//!
//!   - the tool is registered with `.tool_async("async_add", |args| async { … })`,
//!   - the guest supplies a numeric request ID; the host echoes it in the yield
//!     sentinel and queues the future while the guest parks,
//!   - the future is driven **off the vCPU thread** by
//!     `sandbox.drive_host_functions().await`,
//!   - once the future resolves, its result is delivered to the guest in the
//!     next `poll` batch (the JSON argument passed to the guest `poll`
//!     function), resuming the parked guest call transparently.
//!
//! The guest binary is `examples/poll-await-c/`: it issues ONE `/dev/hcall`
//! call to `async_add` with `{a:40, b:2}`, then reports the received sum back as
//! its exit code.
//!
//! The target driving loop under test:
//! ```ignore
//! loop {
//!     match sandbox.poll()? {
//!         Poll::Ready(()) => break,
//!         Poll::Pending => sandbox.drive_host_functions().await,
//!     }
//! }
//! ```
//!
//! Assertions (all must hold to prove the async-tools layer end-to-end):
//!   - the run reaches [`Poll::Ready(())`](core::task::Poll::Ready),
//!   - it takes multiple poll steps (the guest parks/resumes across the await),
//!   - `async_add` runs **exactly once** (the non-idempotent future is queued
//!     once; the completion is delivered via a single poll batch), and
//!   - `last_exit_code() == 42`, proving the summed value survived the
//!     yield/await round-trips and reached the guest intact.
//!
//! This needs a hypervisor and the built artifacts. It self-skips (with a
//! diagnostic) when either is missing so `cargo test` still passes on runners
//! without KVM or a built poll-await kernel.

use core::task::Poll;
use hyperlight_unikraft::Sandbox;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
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
        .join("examples/poll-await-c");
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
            "SKIP: poll-await-c artifacts missing under \
             examples/poll-await-c/.unikraft/build/ — run `just rootfs` then \
             `kraft-hyperlight build --plat hyperlight --arch x86_64` in \
             examples/poll-await-c/ (kraft.yaml enables CONFIG_HYPERLIGHT_POLL \
             and CONFIG_HYPERLIGHT_HCALL) to populate them"
        );
        return None;
    };
    Some((kernel, initrd))
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

/// Drive the poll-await guest to completion using the async-tools API and
/// assert the framework's async request-ID handling behaves end-to-end: the
/// `async` handler runs once, off the vCPU thread, and the final value reaches
/// the guest intact.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn poll_await_async_tool_roundtrip_under_kvm() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    // Counts how many times the async handler's future actually runs. The
    // non-idempotent op must be driven exactly once, no matter how many poll
    // steps the guest is parked across.
    let async_add_calls = Arc::new(AtomicU64::new(0));

    let async_add = {
        let async_add_calls = Arc::clone(&async_add_calls);
        move |args: serde_json::Value| {
            let async_add_calls = Arc::clone(&async_add_calls);
            async move {
                async_add_calls.fetch_add(1, Ordering::SeqCst);
                let a = args
                    .get("a")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| anyhow::anyhow!("async_add: missing/invalid 'a'"))?;
                let b = args
                    .get("b")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| anyhow::anyhow!("async_add: missing/invalid 'b'"))?;
                // Simulate real async work (I/O, timer, …) so the guest is
                // provably parked and resumed across several poll iterations.
                tokio::time::sleep(Duration::from_millis(20)).await;
                Ok(json!(a + b))
            }
        }
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .tool_async("async_add", async_add)
        .build()
        .expect("build sandbox");

    // Reset guest state once before the first step; the poll loop then runs
    // without further restores so scheduler/thread state persists across the
    // HALT/re-entry boundary.
    sbox.reset_poll_deadline();
    sbox.restore().expect("restore before first poll");

    // Bound the loop so a hang can't wedge the suite.
    const MAX_STEPS: usize = 500;
    const MAX_WALL: Duration = Duration::from_secs(30);

    let start = Instant::now();
    let mut steps = 0usize;
    let mut pending_steps = 0usize;
    let mut done = false;

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        steps += 1;
        match sbox.poll().expect("poll") {
            Poll::Ready(()) => {
                done = true;
                break;
            }
            Poll::Pending => {
                pending_steps += 1;
                // Let the registered async tool futures make progress off the
                // vCPU thread; blocks until at least one completes or the
                // guest's next timer deadline elapses.
                sbox.drive_host_functions().await;
            }
        }
    }

    let async_add_n = async_add_calls.load(Ordering::SeqCst);

    println!(
        "poll-await async tool run finished: steps={steps}, pending_steps={pending_steps}, \
         async_add_calls={async_add_n}, elapsed={:?}",
        start.elapsed()
    );

    assert!(
        done,
        "async poll-await run did not reach Done within {steps} steps / {:?} \
         (async_add_calls={async_add_n})",
        start.elapsed()
    );
    assert!(
        steps > 1,
        "expected multiple poll steps (guest should park/resume across the \
         async await), got {steps}"
    );
    assert!(
        pending_steps > 0,
        "expected at least one Pending step that drove host functions (the \
         guest awaits its request ID)"
    );
    // The non-idempotent async handler must run exactly once; the completion is
    // delivered to the guest in a single poll batch.
    assert_eq!(
        async_add_n, 1,
        "async_add future must be driven exactly once (no replay); got \
         {async_add_n}"
    );
    // The summed value (40 + 2) must have reached the guest intact and been
    // reported back as the exit code.
    assert_eq!(
        sbox.last_exit_code(),
        42,
        "guest should report the async sum (40+2) as its exit code; the \
         value did not survive the async yield/await round-trips"
    );
}
