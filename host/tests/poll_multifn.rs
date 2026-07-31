//! Live cooperative-poll run exercising **named guest calls** — the poll-model
//! `Sandbox::call_named_async` — under a real hypervisor (`/dev/kvm`).
//!
//! A single guest entry runs one pump iteration, so a call that parks mid-way
//! would be abandoned. `Sandbox::call_named_step` / `call_named_async` carry
//! the function name and arguments on the *first* cooperative step and then
//! drive the pump like any other poll loop, so the call can park and resume
//! freely.
//!
//! The guest binary is `examples/poll-multifn-c/`. Its `main()` installs an
//! FC-aware dispatch callback and halts; every later named call reaches that
//! callback, which the kernel runs on a scheduler thread. The handler
//! deliberately sleeps in the middle of each call and keeps a running total in
//! guest memory, reporting the new total through a `report` host function.
//!
//! What each assertion pins down:
//!   - **first call** — reaches the guest through `main()` and ends in
//!     [`PollOutcome::Exited`]; the reported total proves the argument arrived;
//!   - **later calls** — end in [`PollOutcome::CallComplete`], i.e. the guest
//!     told the host the named call returned *without* the process exiting.
//!     This is the behaviour that a persistent runtime (`hl_pydriver`) needs
//!     and that the blocking path cannot express;
//!   - **parking works** — each later call takes more than one poll step, so
//!     the handler really did yield the vCPU mid-call and resume;
//!   - **routing is real** — the handler is the registered callback, not a
//!     re-run of `main()`: a re-run would reset the total instead of adding to
//!     the snapshot's;
//!   - **state is snapshot-relative** — two calls made after the same
//!     `snapshot_now()` each start from that snapshot's total.
//!
//! Needs a hypervisor and the built artifacts; self-skips (with a diagnostic)
//! when either is missing.

use hyperlight_unikraft::{PollOutcome, Sandbox};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
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
        .join("examples/poll-multifn-c");
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

fn setup() -> Option<(PathBuf, PathBuf)> {
    if !hypervisor_available() {
        eprintln!("SKIP: no hypervisor available (no /dev/kvm)");
        return None;
    }
    let Some((kernel, initrd)) = poll_artifacts() else {
        eprintln!(
            "SKIP: poll-multifn-c artifacts missing under \
             examples/poll-multifn-c/.unikraft/build/ — run `just rootfs` then \
             `kraft-hyperlight build --plat hyperlight --arch x86_64` in \
             examples/poll-multifn-c/ to populate them"
        );
        return None;
    };
    Some((kernel, initrd))
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

/// What the guest reported through the `report` host function.
type Reports = Arc<Mutex<Vec<(String, i64)>>>;

/// Run one named call to completion, returning `(outcome, steps)`.
async fn run_named_call(sbox: &mut Sandbox, name: &str, arg: &str) -> (PollOutcome, usize) {
    const MAX_STEPS: usize = 500;
    const MAX_WALL: Duration = Duration::from_secs(30);

    let start = Instant::now();
    let mut steps = 1usize;
    let mut outcome = sbox
        .call_named_step(name, arg.to_string())
        .expect("named call entry step");

    while steps < MAX_STEPS && start.elapsed() < MAX_WALL {
        match outcome {
            PollOutcome::CallComplete | PollOutcome::Exited => return (outcome, steps),
            _ => sbox.drive_host_functions().await,
        }
        steps += 1;
        outcome = sbox.poll_outcome().expect("poll");
    }
    panic!(
        "named call {name}({arg}) did not finish within {steps} steps / {:?}",
        start.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn poll_multifn_named_calls_under_kvm() {
    let Some((kernel, initrd)) = setup() else {
        return;
    };

    let reports: Reports = Arc::new(Mutex::new(Vec::new()));
    let report_tool = {
        let reports = Arc::clone(&reports);
        move |args: serde_json::Value| {
            let obj = args
                .as_object()
                .ok_or_else(|| anyhow::anyhow!("report: expected an object"))?;
            let (key, value) = obj
                .iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("report: expected one field"))?;
            let value = value
                .as_i64()
                .ok_or_else(|| anyhow::anyhow!("report: value must be an integer"))?;
            reports.lock().unwrap().push((key.clone(), value));
            Ok(serde_json::json!({}))
        }
    };

    let mut sbox = Sandbox::builder(&kernel)
        .initrd_file(&initrd)
        .heap_size(32 * 1024 * 1024)
        .tool("report", report_tool)
        .build()
        .expect("build sandbox");

    // ---- First call: enters through main(), which registers the callback ----
    sbox.restore().expect("restore before first call");
    let (outcome, steps) = run_named_call(&mut sbox, "run", "40").await;
    assert_eq!(
        outcome,
        PollOutcome::Exited,
        "the first call runs main(), which halts the guest when it returns"
    );
    assert!(
        steps > 1,
        "the handler sleeps mid-call, so even the first call must park and \
         resume; got {steps} step(s)"
    );
    assert_eq!(
        reports.lock().unwrap().as_slice(),
        [("total".to_string(), 40)],
        "the first call's argument must reach the handler"
    );

    // Capture the warmed guest: callback registered, running total at 40.
    sbox.snapshot_now().expect("snapshot after warm-up call");

    // ---- Later calls: routed to the registered callback ----
    for (arg, expected_total) in [("2", 42), ("100", 140)] {
        reports.lock().unwrap().clear();
        sbox.restore().expect("restore before named call");
        let (outcome, steps) = run_named_call(&mut sbox, "run", arg).await;

        assert_eq!(
            outcome,
            PollOutcome::CallComplete,
            "a call served by the registered callback completes without the \
             guest process exiting (arg={arg})"
        );
        assert!(
            steps > 1,
            "the handler sleeps mid-call, so the call must park and resume; \
             got {steps} step(s) for arg={arg}"
        );
        assert_eq!(
            reports.lock().unwrap().as_slice(),
            [("total".to_string(), expected_total)],
            "the call must add to the snapshot's total ({}), proving it ran the \
             registered callback rather than re-running main() (arg={arg})",
            expected_total - arg.parse::<i64>().unwrap()
        );
    }
}
