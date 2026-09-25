// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The wasmtime image: WebAssembly text through `Exec`, and a WASI
//! program (`examples/wasmtime/hello`, a core module for wasm32-wasip1 and
//! a component for wasm32-wasip2, built by `just build-test-bins`) through
//! `GuestExec` from a mount.

mod common;

use common::{BIN_MOUNT, require_bin, require_bins, require_rootfs, temp_dir};
use hyperlight_unikraft::{Error, Exec, Mount, SandboxBuilder};

const SCRATCH_MB: usize = 256;

/// A WASI preview 1 command that prints `text` with `fd_write`.
fn hello_wat(text: &str) -> String {
    let len = text.len() + 1;
    format!(
        r#"(module
  (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 16) "{text}\n")
  (func (export "_start")
    (i32.store (i32.const 0) (i32.const 16))
    (i32.store (i32.const 4) (i32.const {len}))
    (drop (call $fd_write (i32.const 1) (i32.const 0) (i32.const 1) (i32.const 8)))))"#
    )
}

#[test]
fn wasmtime_runs_webassembly_text() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .boot()
        .unwrap();
    sandbox.run(hello_wat("Hello from Wasmtime")).unwrap();
    assert_eq!(sandbox.drain_output().trim(), "Hello from Wasmtime");
    // The engine serves the next call too.
    sandbox.run(hello_wat("again")).unwrap();
    assert_eq!(sandbox.drain_output().trim(), "again");
}

#[test]
fn wasmtime_exit_code_is_the_call_status() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .boot()
        .unwrap();
    let exit = r#"(module
  (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
  (memory (export "memory") 1)
  (func (export "_start") (call $exit (i32.const 7))))"#;
    match sandbox.run(exit) {
        Err(Error::CallFailed { status }) => assert_eq!(status, 7),
        other => panic!("expected the call to fail with 7, got {other:?}"),
    }
    // A trap fails the call; the engine is still there.
    let trap = r#"(module (func (export "_start") unreachable))"#;
    assert!(matches!(
        sandbox.run(trap),
        Err(Error::CallFailed { status: 1 })
    ));
    assert!(sandbox.drain_output().contains("unreachable"));
    sandbox.run(hello_wat("still here")).unwrap();
}

/// The WASI program as a module and as a component: arguments, the host's
/// environment, and a file read and written through a mount (which needs
/// the kernel to resolve a path relative to the preopened `/` across the
/// mount point).
#[test]
fn wasmtime_runs_wasi_modules_and_components_from_a_mount() {
    let data = temp_dir("wasmtime-data");
    std::fs::write(data.path().join("input.txt"), "one two three\n").unwrap();
    for program in ["hello-p1.wasm", "hello-p2.wasm"] {
        let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
            .scratch_mb(SCRATCH_MB)
            .mounts(vec![
                Mount::ro(require_bins("wasmtime"), BIN_MOUNT),
                Mount::rw(data.path(), "/mnt/data"),
            ])
            .boot()
            .unwrap();
        sandbox.set_env_vars(&[("GREETING", "hi there")]);
        sandbox
            .run(Exec::Guest(format!("{BIN_MOUNT}/{program} world")))
            .unwrap();
        let output = sandbox.drain_output();
        assert!(
            output.contains("Hello, world, from WebAssembly"),
            "{program}: {output}"
        );
        assert!(output.contains("GREETING=hi there"), "{program}: {output}");
        assert!(
            output.contains("input.txt has 3 words"),
            "{program}: {output}"
        );
        let written = std::fs::read_to_string(data.path().join("output.txt")).unwrap();
        assert_eq!(written.trim(), "3", "{program}");
        std::fs::remove_file(data.path().join("output.txt")).unwrap();

        // A non-zero exit fails the call: the module's own code, the
        // component's 1 (WASI 0.2's exit carries only success or failure).
        let want = if program.ends_with("p1.wasm") { 3 } else { 1 };
        match sandbox.run(Exec::Guest(format!("{BIN_MOUNT}/{program} fail"))) {
            Err(Error::CallFailed { status }) => assert_eq!(status, want, "{program}"),
            other => panic!("{program}: expected a failed call, got {other:?}"),
        }
    }
}

#[test]
fn wasmtime_snapshot_round_trip() {
    let snap_dir = temp_dir("wasmtime-snap");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.run(hello_wat("restored-wasmtime-ok")).unwrap();
    assert_eq!(sandbox.drain_output().trim(), "restored-wasmtime-ok");
}

/// A library module, loaded once and called many times, its state kept
/// between calls; an import outside WASI is the embedder's function.
#[test]
fn wasmtime_calls_a_loaded_module_and_the_host() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .host_function("math.mul", |args| {
            let [a, b]: [i64; 2] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok((a * b).to_string())
        })
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"(module
  (import "math" "mul" (func $mul (param i32 i32) (result i32)))
  (global $calls (mut i32) (i32.const 0))
  (func (export "add") (param i32 i32) (result i32)
    (global.set $calls (i32.add (global.get $calls) (i32.const 1)))
    (i32.add (local.get 0) (local.get 1)))
  (func (export "mul_plus_one") (param i32 i32) (result i32)
    (i32.add (call $mul (local.get 0) (local.get 1)) (i32.const 1)))
  (func (export "calls") (result i32) (global.get $calls))
  (func (export "half") (param f64) (result f64) (f64.mul (local.get 0) (f64.const 0.5))))"#,
        )
        .unwrap();
    assert_eq!(sandbox.call("add", "[2, 3]").unwrap(), "5");
    assert_eq!(sandbox.call("add", "[40, 2]").unwrap(), "42");
    assert_eq!(sandbox.call("calls", "").unwrap(), "2");
    assert_eq!(sandbox.call("mul_plus_one", "[6, 7]").unwrap(), "43");
    assert_eq!(sandbox.call("half", "[3]").unwrap(), "1.5");
    // An i32 takes a signed or an unsigned 32-bit number, never a wider
    // one cut down to fit.
    assert_eq!(sandbox.call("add", "[4294967295, 1]").unwrap(), "0");
    assert!(sandbox.call("add", "[4294967297, 1]").is_err());
    assert!(sandbox.call("add", "[1e20, 1]").is_err());
    // Wrong arguments, a missing export: the call fails, the module stays.
    assert!(sandbox.call("add", "[1]").is_err());
    assert!(sandbox.call("nope", "[]").is_err());
    // Three adds ran; the refused calls never reached the module.
    assert_eq!(sandbox.call("calls", "").unwrap(), "3");
}

/// A component: a function it exports, lifted from core code that calls
/// a function of an imported interface, which the embedder provides as
/// `math.mul`.
#[test]
fn wasmtime_calls_a_component_and_its_host_interface() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .host_function("math.mul", |args| {
            let [a, b]: [u32; 2] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok((a * b).to_string())
        })
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"(component
  (import "example:host/math@0.1.0" (instance $math
    (export "mul" (func (param "a" u32) (param "b" u32) (result u32)))))
  (core func $mul (canon lower (func $math "mul")))
  (core module $m
    (import "host" "mul" (func $mul (param i32 i32) (result i32)))
    (func (export "mul-plus-one") (param i32 i32) (result i32)
      (i32.add (call $mul (local.get 0) (local.get 1)) (i32.const 1)))
    (func (export "is-even") (param i32) (result i32)
      (i32.eqz (i32.rem_u (local.get 0) (i32.const 2)))))
  (core instance $i (instantiate $m (with "host" (instance (export "mul" (func $mul))))))
  (func (export "mul-plus-one") (param "a" u32) (param "b" u32) (result u32)
    (canon lift (core func $i "mul-plus-one")))
  (func (export "is-even") (param "n" u32) (result bool)
    (canon lift (core func $i "is-even"))))"#,
        )
        .unwrap();
    assert_eq!(sandbox.call("mul-plus-one", "[6, 7]").unwrap(), "43");
    assert_eq!(sandbox.call("is-even", "[4]").unwrap(), "true");
    assert_eq!(sandbox.call("is-even", "[5]").unwrap(), "false");
    // A u32 cannot be negative: the argument is refused, not wrapped.
    assert!(sandbox.call("is-even", "[-1]").is_err());
}

/// Two clones of one warm snapshot, with a library loaded in it, draw
/// different WASI random bytes: the generator is the kernel's, which
/// reseeds on every restore, not one captured with the library.
#[test]
fn wasmtime_snapshot_clones_draw_their_own_random_bytes() {
    let snap_dir = temp_dir("wasmtime-random-snap");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"(module
  (import "wasi_snapshot_preview1" "random_get" (func $random_get (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "draw") (result i64)
    (drop (call $random_get (i32.const 0) (i32.const 8)))
    (i64.load (i32.const 0))))"#,
        )
        .unwrap();
    sandbox.snapshot_to(&snap_dir).unwrap();
    let draws: Vec<String> = (0..2)
        .map(|_| {
            let mut clone = SandboxBuilder::from_snapshot_dir(&snap_dir)
                .unwrap()
                .boot()
                .unwrap();
            clone.call("draw", "").unwrap()
        })
        .collect();
    assert_ne!(draws[0], draws[1], "two clones drew the same bytes");
}

/// A module may import the same function more than once.
#[test]
fn wasmtime_loads_a_module_importing_a_host_function_twice() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .host_function("math.double", |args| {
            let [a]: [i64; 1] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok((a * 2).to_string())
        })
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"(module
  (import "math" "double" (func $a (param i32) (result i32)))
  (import "math" "double" (func $b (param i32) (result i32)))
  (func (export "quad") (param i32) (result i32) (call $b (call $a (local.get 0)))))"#,
        )
        .unwrap();
    assert_eq!(sandbox.call("quad", "[5]").unwrap(), "20");
}

/// A WASI 0.3 command (`examples/wasmtime/hello-p3`): an async `run`,
/// stdout written as a stream, and two timers awaited at once.
#[test]
fn wasmtime_runs_a_wasi_0_3_command() {
    require_bin("wasmtime", "hello-p3.wasm");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .mount(Mount::ro(require_bins("wasmtime"), BIN_MOUNT))
        .boot()
        .unwrap();
    sandbox
        .run(Exec::Guest(format!("{BIN_MOUNT}/hello-p3.wasm world")))
        .unwrap();
    let output = sandbox.drain_output();
    assert!(output.contains("Hello, world, from WASI 0.3!"), "{output}");
    // Two 20 ms sleeps side by side take one tick, not two.
    let ms: u64 = output
        .split("concurrently in ")
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("{output}"));
    assert!((20..40).contains(&ms), "{output}");
}

/// A WASI 0.3 library (`examples/wasmtime/calculator`): a plain export and
/// an async one, both calling the embedder's function through the WIT
/// interface the component imports.
#[test]
fn wasmtime_calls_an_async_export() {
    require_bin("wasmtime", "calculator.wasm");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .mount(Mount::ro(require_bins("wasmtime"), BIN_MOUNT))
        .host_function("math.add", |args| {
            let [a, b]: [i32; 2] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok((a + b).to_string())
        })
        .boot()
        .unwrap();
    sandbox
        .run(Exec::Guest(format!("{BIN_MOUNT}/calculator.wasm")))
        .unwrap();
    assert_eq!(sandbox.call("sum-of-squares", "[3, 4]").unwrap(), "25");
    assert_eq!(
        sandbox.call("slow-sum-of-squares", "[5, 12, 10]").unwrap(),
        "169"
    );
    // The instance carries on after an async call.
    assert_eq!(sandbox.call("sum-of-squares", "[1, 1]").unwrap(), "2");

    // A snapshot taken with the library loaded and the async runtime
    // started serves the same calls once restored.
    let snap_dir = temp_dir("wasmtime-p3-snap");
    sandbox.snapshot_to(&snap_dir).unwrap();
    drop(sandbox);
    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap_dir)
        .unwrap()
        .host_function("math.add", |args| {
            let [a, b]: [i32; 2] = serde_json::from_str(args).map_err(|e| e.to_string())?;
            Ok((a + b).to_string())
        })
        .boot()
        .unwrap();
    assert_eq!(
        sandbox.call("slow-sum-of-squares", "[3, 4, 10]").unwrap(),
        "25"
    );
}

/// An async export needs no WASI 0.3 import (it uses the component
/// model's own builtins), and is still called asynchronously.
#[test]
fn wasmtime_calls_an_async_export_without_wasi() {
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("wasmtime"))
        .scratch_mb(SCRATCH_MB)
        .boot()
        .unwrap();
    sandbox
        .run(
            r#"(component
  (core module $m
    (import "" "task.return" (func $ret (param i32)))
    (func (export "inc") (param i32) (result i32)
      (call $ret (i32.add (local.get 0) (i32.const 1)))
      (i32.const 0))
    (func (export "cb") (param i32 i32 i32) (result i32) unreachable))
  (core func $ret (canon task.return (result u32)))
  (core instance $i (instantiate $m (with "" (instance (export "task.return" (func $ret))))))
  (func (export "inc") async (param "x" u32) (result u32)
    (canon lift (core func $i "inc") async (callback (core func $i "cb")))))"#,
        )
        .unwrap();
    assert_eq!(sandbox.call("inc", "[41]").unwrap(), "42");
}
