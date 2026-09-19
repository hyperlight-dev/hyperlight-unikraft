// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use common::{hluk_with_stdin_scratch, require_rootfs, temp_dir};
use hyperlight_unikraft::{Exec, Mount, NetworkPolicy, SandboxBuilder};

#[test]
fn dotnet_jit_inline_code() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox
        .run("Console.WriteLine(\"hluk-dotnet-ok\");")
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("hluk-dotnet-ok"),
        "expected guest to print 'hluk-dotnet-ok', got: {output:?}",
    );
}

#[test]
fn dotnet_jit_exec_file() {
    let rootfs = require_rootfs("dotnet-jit");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/Hello.cs");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("Hello"),
        "expected Hello.cs to produce output containing 'Hello', got: {output:?}",
    );
}

#[test]
fn dotnet_jit_snapshot_round_trip() {
    let rootfs = require_rootfs("dotnet-jit");
    let snap_dir = temp_dir("dotnet-jit-snap");

    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox
        .run("Console.WriteLine(\"restored-dotnet-ok\");")
        .unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("restored-dotnet-ok"),
        "expected restored guest to print 'restored-dotnet-ok', got: {output:?}",
    );
}

#[test]
fn dotnet_jit_multiple_runs() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();

    sandbox.run("Console.WriteLine(\"run1-ok\");").unwrap();
    let output1 = sandbox.drain_output();
    assert!(
        output1.contains("run1-ok"),
        "expected 'run1-ok' from first dispatch, got: {output1:?}"
    );

    sandbox.run("Console.WriteLine($\"x={1 + 1}\");").unwrap();
    let output2 = sandbox.drain_output();
    assert!(
        output2.contains("x=2"),
        "expected 'x=2' from second dispatch, got: {output2:?}"
    );
}

#[test]
fn dotnet_jit_math() {
    let rootfs = require_rootfs("dotnet-jit");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/Math.cs");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("=== .NET Math Demo ==="),
        "expected Math.cs header, got: {output:?}"
    );
    assert!(
        output.contains("Math demo done"),
        "expected Math.cs to complete, got: {output:?}"
    );
}

#[test]
fn dotnet_jit_stdin_piped() {
    let rootfs = require_rootfs("dotnet-jit");
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/StdinEcho.cs");
    let output = hluk_with_stdin_scratch(&rootfs, &script, b"hello from host\nline two\n", 768);
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
fn dotnet_jit_env_vars() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox.set_env_vars(&[
        ("MY_VAR", "hello_world"),
        ("DEBUG", "1"),
        ("GREETING", "hi there"),
    ]);
    sandbox
        .run(Exec::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/EnvVars.cs"),
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

#[test]
fn dotnet_jit_fs() {
    let rootfs = require_rootfs("dotnet-jit");
    let mount_dir = temp_dir("dotnet-jit-fs");

    let mounts = vec![Mount::rw(mount_dir.path(), "/mnt/host")];
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .mounts(mounts)
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/FsOps.cs");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();

    assert!(
        output.contains("read-back: hello from dotnet"),
        "expected FsOps.cs to read back its write, got: {output:?}",
    );
    assert!(
        output.contains("line-count: 3") && output.contains("fs-ops-done"),
        "expected FsOps.cs to finish, got: {output:?}",
    );
    // The write landed on the host side of the mount.
    assert!(mount_dir.path().join("dotnet_fs.txt").exists());
}

#[test]
fn dotnet_jit_threading() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/Threads.cs");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();

    assert!(
        output.contains("sum-of-squares: 140"),
        "expected Task fan-out result 140, got: {output:?}",
    );
    assert!(
        output.contains("thread-counter: 4") && output.contains("threads-done"),
        "expected 4 threads to increment the counter, got: {output:?}",
    );
}

#[test]
fn dotnet_jit_http_get() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/dotnet-jit/HttpGet.cs");
    sandbox.run(Exec::File(script)).unwrap();
    let output = sandbox.drain_output();

    assert!(
        output.contains("Status: 200"),
        "expected HttpGet.cs to get Status: 200, got: {output:?}",
    );
}

/// A worker thread that waits.  The threads in `Threads.cs` finish in
/// microseconds; one that sleeps past the first second is around when the
/// runtime first signals its threads, which exercises the alternate signal
/// stacks the runtime installs per thread.
#[test]
fn dotnet_jit_worker_that_waits() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "var t = new System.Threading.Thread(() => { System.Threading.Thread.Sleep(1500); Console.WriteLine(\"slept\"); });\n",
            "t.Start(); t.Join(); Console.WriteLine(\"joined\"); Console.Out.Flush();\n",
        ))
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("slept") && out.contains("joined"), "{out:?}");
}

/// A call ends when its top-level statements return; a thread it started
/// is left behind and makes progress on the steps that follow, and the
/// next call finds the runtime intact.
#[test]
fn dotnet_jit_thread_runs_between_calls() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    let t = Instant::now();
    sandbox
        .run(concat!(
            "var t = new System.Threading.Thread(() => { for (int i = 1; i <= 5; i++) { System.Threading.Thread.Sleep(300); Console.WriteLine($\"tick {i}\"); Console.Out.Flush(); } });\n",
            "t.Start(); Console.WriteLine(\"started\"); Console.Out.Flush();\n",
        ))
        .unwrap();
    assert!(
        t.elapsed() < Duration::from_millis(800),
        "the call waited for the thread"
    );
    assert!(sandbox.drain_output().contains("started"));
    while t.elapsed() < Duration::from_millis(2200) {
        sandbox.step(Duration::from_millis(500)).unwrap();
    }
    assert!(
        sandbox.drain_output().contains("tick 5"),
        "thread did not run while parked"
    );
    sandbox
        .run("Console.WriteLine(\"again\"); Console.Out.Flush();")
        .unwrap();
    assert!(sandbox.drain_output().contains("again"));
}

/// `Environment.Exit()` ends the .NET process, which is the runtime: the
/// call ends with that status, and the next call starts a fresh one.
/// The fresh runtime needs the room for a second one, 1024 MB here,
/// until the kernel returns an exited process's memory.
#[test]
fn dotnet_jit_environment_exit_ends_the_call() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs.clone())
        .scratch_mb(1024)
        .boot()
        .unwrap();
    sandbox
        .run("Console.WriteLine(\"leaving\"); Environment.Exit(0); Console.WriteLine(\"not reached\");")
        .unwrap();
    assert_eq!(sandbox.drain_output(), "leaving\r\n");
    sandbox.run("Console.WriteLine(\"still here\");").unwrap();
    assert!(sandbox.drain_output().contains("still here"));

    // A non-zero code fails the call.
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    assert!(
        sandbox.run("Environment.Exit(3);").is_err(),
        "Environment.Exit(3) did not fail the call"
    );
}

/// The GC's hard limit is a share of the memory the kernel reports, so an
/// allocation that cannot be served is an OutOfMemoryException the code
/// can catch, not a page fault the kernel cannot serve.
#[test]
fn dotnet_jit_out_of_memory_is_an_exception() {
    let rootfs = require_rootfs("dotnet-jit");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(768)
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "GC.Collect();\n",
            "Console.WriteLine($\"limit={GC.GetGCMemoryInfo().TotalAvailableMemoryBytes >> 20}\");\n",
            "var keep = new System.Collections.Generic.List<byte[]>();\n",
            "try { while (true) keep.Add(new byte[1 << 20]); }\n",
            "catch (OutOfMemoryException) { Console.WriteLine($\"oom after {keep.Count}\"); }\n",
            "keep = null; GC.Collect();\n",
        ))
        .unwrap();
    let out = sandbox.drain_output();
    let limit_mb: u64 = out
        .lines()
        .find_map(|l| l.trim().strip_prefix("limit="))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no limit line in {out:?}"));
    assert!(
        (100..500).contains(&limit_mb),
        "the GC limit should be a share of the guest's memory, got {limit_mb} MiB: {out:?}"
    );
    assert!(
        out.contains("oom after"),
        "no OutOfMemoryException: {out:?}"
    );
    sandbox.run("Console.WriteLine(\"still here\");").unwrap();
    assert!(sandbox.drain_output().contains("still here"));
}
