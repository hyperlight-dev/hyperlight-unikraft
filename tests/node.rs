// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;

use common::{hluk_with_stdin_scratch, require_rootfs, temp_dir};
use std::time::{Duration, Instant};

use hyperlight_unikraft::{Exec, Mount, NetworkPolicy, SandboxBuilder, Yield};

#[test]
fn node_exec_file() {
    let rootfs = require_rootfs("node");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/hello.js");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello"),
        "expected hello.js to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn node_inline_code() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox.run("console.log('hluk-node-ok')").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("hluk-node-ok"),
        "expected guest to print 'hluk-node-ok', got: {output:?}",
    );
}

#[test]
fn node_snapshot_round_trip() {
    let rootfs = require_rootfs("node");
    let snap_dir = temp_dir("node-snap");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs.clone())
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.run("console.log('restored-node-ok')").unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("restored-node-ok"),
        "expected restored node output, got: {output:?}",
    );
}

#[test]
fn node_stdin_piped() {
    let rootfs = require_rootfs("node");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/stdin_echo.js");
    let output = hluk_with_stdin_scratch(&rootfs, &script, b"hello from host\nline two\n", 512);
    assert!(
        output.contains("lines=2"),
        "expected 2 lines, got: {output:?}"
    );
    assert!(
        output.contains("echo: hello from host"),
        "expected first line, got: {output:?}"
    );
    assert!(
        output.contains("stdin-done"),
        "expected stdin-done marker, got: {output:?}"
    );
}

#[test]
fn node_async_timers() {
    let rootfs = require_rootfs("node");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/async_timers.js");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    let code = std::fs::read_to_string(&script).unwrap();
    sandbox.run(&*code).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("results: timer-1, timer-2"),
        "expected chained timer results, got: {output:?}",
    );
    assert!(
        output.contains("async-done"),
        "expected async-done marker, got: {output:?}",
    );
}

#[test]
fn node_fs_ops() {
    let rootfs = require_rootfs("node");
    let mount_dir = temp_dir("node-fs-ops");

    let mounts = vec![Mount::rw(mount_dir.path(), "/mnt/host")];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .mounts(mounts)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/fs_ops.js");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Cleanup done"),
        "expected fs_ops.js to print 'Cleanup done', got: {output:?}",
    );
}

#[test]
fn node_http_get() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();

    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/http_get.js");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Status: 200"),
        "expected http_get.js to get Status: 200, got: {output:?}",
    );
}

#[test]
fn node_env_vars() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox
        .run(Exec::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/node/env_vars.js"),
        ))
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("MY_VAR=hello_world"),
        "expected MY_VAR=hello_world, got: {output:?}"
    );
    assert!(
        output.contains("DEBUG=1"),
        "expected DEBUG=1, got: {output:?}"
    );
    assert!(
        output.contains("GREETING=hi there"),
        "expected GREETING=hi there, got: {output:?}"
    );
}

/// A call that awaits a timer parks the guest: the host waits the second
/// out without entering, then the call goes on and returns.
#[test]
fn node_await_sleep_parks_the_guest() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    let t = Instant::now();
    sandbox
        .run(
            "(async () => { await new Promise(r => setTimeout(r, 1000)); console.log('woke'); })()",
        )
        .unwrap();
    assert!(t.elapsed() >= Duration::from_millis(900), "returned early");
    assert!(sandbox.drain_output().contains("woke"));
}

/// A live interval keeps the call in flight, as it keeps `node script.js`
/// alive: the steps report `Blocked`, the ticks arrive, and a second call
/// is refused until the first ends.
#[test]
fn node_interval_keeps_the_call_in_flight() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox
        .submit("globalThis.n = 0; setInterval(() => { globalThis.n += 1; console.log('tick', globalThis.n); }, 200);")
        .unwrap();
    let t = Instant::now();
    while t.elapsed() < Duration::from_millis(1100) {
        let y = sandbox.step(Duration::from_millis(500)).unwrap();
        assert!(matches!(y, Yield::Blocked { .. }), "the call ended: {y:?}");
    }
    assert!(sandbox.drain_output().contains("tick 4"));
    assert!(
        sandbox.submit("1").is_err(),
        "a second call was accepted while one is in flight"
    );
}

/// An `unref()`ed interval ends the call, like a Node script exits with
/// one pending, and then keeps running on the steps that follow: the
/// child's event loop stays alive while it waits for the next call, and
/// that next call finds the child there.
#[test]
fn node_unref_interval_runs_between_calls() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    let t = Instant::now();
    sandbox
        .run("globalThis.n = 0; setInterval(() => { globalThis.n += 1; }, 200).unref(); console.log('started');")
        .unwrap();
    assert!(
        t.elapsed() < Duration::from_millis(500),
        "the unref'd interval held the call"
    );
    assert_eq!(sandbox.drain_output(), "started\r\n");
    while t.elapsed() < Duration::from_millis(1300) {
        sandbox.step(Duration::from_millis(500)).unwrap();
    }
    sandbox.run("console.log('n =', globalThis.n)").unwrap();
    let out = sandbox.drain_output();
    let n: u32 = out
        .trim()
        .trim_start_matches("n =")
        .trim()
        .parse()
        .unwrap_or(0);
    assert!(n >= 3, "interval did not run while parked: {out:?}");
}

/// An exit inside a call ends the call, not the child: `process.exit(3)`
/// fails the call with that status, `process.exitCode` counts when the
/// loop drains, an uncaught error is status 1, and what the call left
/// running stops with it, as it would have with the process.  Globals
/// survive, and the next call runs as usual.
#[test]
fn node_exit_ends_the_call() {
    let rootfs = require_rootfs("node");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(512)
        .boot()
        .unwrap();
    sandbox
        .run("globalThis.kept = 'yes'; console.log('leaving'); process.exit(0); console.log('not reached')")
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\r\n");
    assert!(
        sandbox.run("process.exit(3)").is_err(),
        "process.exit(3) did not fail the call"
    );
    assert!(
        sandbox.run("process.exitCode = 2").is_err(),
        "process.exitCode = 2 did not fail the call"
    );
    assert!(
        sandbox
            .run("setTimeout(() => { throw new Error('boom') }, 10)")
            .is_err(),
        "an uncaught error in a timer did not fail the call"
    );
    assert!(sandbox.drain_output().contains("boom"));
    // An exit from inside an interval stops the interval: the next call
    // is not held by it.
    let t = Instant::now();
    sandbox
        .run("let n = 0; setInterval(() => { if (++n === 3) { console.log('third tick'); process.exit(0); } }, 50)")
        .unwrap();
    assert!(sandbox.drain_output().contains("third tick"));
    sandbox
        .run("console.log('kept =', globalThis.kept)")
        .unwrap();
    assert!(
        t.elapsed() < Duration::from_secs(2),
        "the interval held the next call"
    );
    assert!(sandbox.drain_output().contains("kept = yes"));
    // An exit stops what its own call started; what an earlier call left
    // running in the background goes on.
    sandbox
        .run("globalThis.bg = 0; setInterval(() => { globalThis.bg++; }, 10).unref();")
        .unwrap();
    sandbox
        .run("setInterval(() => {}, 10); process.exit(0)")
        .unwrap();
    sandbox
        .run("(async () => { const b = globalThis.bg; await new Promise(r => setTimeout(r, 50)); console.log('bg ticking:', globalThis.bg > b); })()")
        .unwrap();
    assert!(
        sandbox.drain_output().contains("bg ticking: true"),
        "the exit stopped an earlier call's background work"
    );
}
