// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Boots a *native* Unikraft image (a C `main()` compiled directly into the
//! kernel — no ELF loader, no initrd) through the `--kernel` flag / the
//! library's [`SandboxBuilder::from_kernel`].  This proves the host can run a
//! kernel other than the embedded elfloader.
//!
//! The fixture kernel is built reproducibly with `just build-native-kernel`
//! (and checked by `just verify-native-kernel` in CI) against mini's cleanup
//! Unikraft, which needs the platform fix `select LIBPOSIX_ENVIRON` in
//! `plat/hyperlight/Config.uk` (dispatch.c calls putenv/setenv, which a
//! non-elfloader build wouldn't otherwise pull in).  See the fixture README.
//!
//! Native apps use a different execution model from the elfloader runtimes:
//! the workload runs at *boot* (during `init()`/evolve), not on a later
//! `run(Exec)` dispatch.  So this test only exercises `init()`: the native
//! `main()` runs, prints (captured by the host), then the guest shuts down
//! cleanly via `uk_pm_shutdown` → `hyperlight_shutdown` (port 108).  A native
//! app registers no dispatch callback, so `run(Exec)` does not apply.
//!
//! Booting a native kernel this way surfaced (and this branch fixes) a latent
//! bug: `dispatch.c` registered `hyperlight_dispatch_inject_host_env` with a
//! bogus term-function pointer (`0x1`), which the shutdown term loop called —
//! the elfloader never hit it because it never returns from `main`.  See the
//! fixture README.

use std::path::PathBuf;

use hyperlight_unikraft::SandboxBuilder;

fn require_native_kernel() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/native-kernel/helloworld-native_hyperlight-x86_64");
    assert!(
        p.exists(),
        "native kernel fixture not found at {} — build it with `just build-native-kernel`",
        p.display(),
    );
    p
}

#[test]
fn native_kernel_boots_and_prints() {
    let kernel = require_native_kernel();

    // A bare native kernel: no initrd, no mounts, no networking.
    // boot() runs the guest's main() to its halt; capture what it printed.
    let (_sandbox, cfg) = SandboxBuilder::from_kernel(kernel)
        .scratch_mb(64)
        .boot()
        .unwrap();
    let output = cfg.drain_output();

    assert!(
        output.contains("Hello, World from a NATIVE mini kernel"),
        "native kernel boot output missing greeting, got: {output:?}",
    );
}
