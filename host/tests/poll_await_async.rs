//! Live cooperative-poll run exercising the **async host-function tools** API
//! (`SandboxBuilder::tool_async` + `Sandbox::poll_outcome` + `drive_host_functions`)
//! under a real hypervisor (`/dev/kvm` on Linux).
//!
//! This proves the framework transparently drives a plain Rust `async` handler
//! across the guest's cooperative park/resume:
//!
//!   - the tool is registered with `.tool_async("async_add", |args| async { … })`,
//!   - the guest supplies a numeric request ID in a binary control frame; the
//!     host returns a pending frame and queues the future while the guest parks,
//!   - the future is driven **off the vCPU thread** by
//!     `sandbox.drive_host_functions().await`,
//!   - once the future resolves, its result is delivered to the guest in the
//!     next binary `poll` batch, resuming the parked guest call transparently.
//!
//! The guest binary is `examples/poll-await-c/`: it issues ONE `/dev/hcall`
//! call to `async_add` with `{a:40, b:2}`, then reports the received sum back as
//! its exit code.
//!
//! The target driving loop under test:
//! ```ignore
//! loop {
//!     match sandbox.poll_outcome()? {
//!         PollOutcome::Exited => break,
//!         PollOutcome::Idle | PollOutcome::Timer(_)
//!         | PollOutcome::HostCallsPending { .. } => {
//!             sandbox.drive_host_functions().await
//!         }
//!     }
//! }
//! ```
//!
//! Assertions (all must hold to prove the async-tools layer end-to-end):
//!   - the run reaches [`PollOutcome::Exited`],
//!   - at least one step reports [`PollOutcome::HostCallsPending`],
//!   - it takes multiple poll steps (the guest parks/resumes across the await),
//!   - `async_add` runs **exactly once** (the non-idempotent future is queued
//!     once; the completion is delivered via a single poll batch), and
//!   - `last_exit_code() == 42`, proving the summed value survived the
//!     yield/await round-trips and reached the guest intact.
//!
//! This needs a hypervisor and the built artifacts. It self-skips (with a
//! diagnostic) when either is missing so `cargo test` still passes on runners
//! without KVM or a built poll-await kernel.

use hyperlight_unikraft::{PollOutcome, Sandbox};
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

mod common;
use common::setup;

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

/// Drive the poll-await guest to completion using the async-tools API and
/// assert the framework's async request-ID handling behaves end-to-end: the
/// `async` handler runs once, off the vCPU thread, and the final value reaches
/// the guest intact.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn poll_await_async_tool_roundtrip_under_kvm() {
    let Some((kernel, initrd)) = setup("poll-await-c") else {
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
    sbox.restore().expect("restore before first poll");

    // Bound the loop so a hang can't wedge the suite.
    const MAX_STEPS: usize = 500;
    const MAX_WALL: Duration = Duration::from_secs(30);

    let start = Instant::now();
    let mut steps = 0usize;
    let mut pending_steps = 0usize;
    let mut saw_host_calls_pending = false;
    let mut done = false;

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        steps += 1;
        match sbox.poll_outcome().expect("poll") {
            PollOutcome::Exited => {
                done = true;
                break;
            }
            outcome => {
                pending_steps += 1;
                saw_host_calls_pending |= matches!(outcome, PollOutcome::HostCallsPending { .. });
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
    assert!(
        saw_host_calls_pending,
        "typed poll outcome never reported the in-flight async host call"
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
