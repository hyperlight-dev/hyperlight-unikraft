# Guest Runtime Support Tiers

This document defines the support tiers for guest runtimes in hyperlight-unikraft. The tier model is inspired by [Rust's platform support policy](https://doc.rust-lang.org/nightly/rustc/target-tier-policy.html) and [CPython's PEP 11](https://peps.python.org/pep-0011/), adapted for guest runtimes running inside Unikraft unikernels on Hyperlight.

## Tiers at a Glance

| | Tier 1 | Tier 2 | Tier 3 |
|---|---|---|---|
| **Guarantee** | Works and is measured | Works | Builds and smoke-tested |
| **Integration tests** | ✅ Capability coverage | ✅ Capability coverage (where applicable) | ✅ Basic smoke (boot/run/snapshot) |
| **Conformance tests** | ✅ Full upstream suite | — | — |
| **Benchmarks** | ✅ Tracked on CI | — | — |
| **Failures block release** | Yes | Yes | Yes (smoke tests run in CI) |
| **Driver + Dockerfile** | ✅ | ✅ | ✅ |
| **Known-failures manifest** | ✅ Maintained | — | — |
| **Snapshot support** | ✅ Verified | ✅ Verified | ✅ Smoke test |

## Tier 1 — Works and is measured

A tier 1 runtime has full conformance testing against the upstream test suite, a maintained known-failures manifest documenting every non-passing test with a root cause, and benchmarks tracked across commits for regression detection.

**Requirements:**

- **Driver and Dockerfile** that produce a working rootfs CPIO.
- **Integration tests** covering boot, inline execution, file execution, snapshot round-trip, and multi-call dispatch. Failures block release.
- **Conformance tests** running the runtime's upstream test suite (e.g., CPython's `Lib/test/`). Each test module runs in an isolated guest via snapshot restore.
- **Known-failures manifest** (`conformance/<runtime>/known_failures.toml`) categorizing every non-passing module as `unsupported`, `may-fix`, or `fixable`, with a reason and TODO for each.
- **Benchmarks** with workload scripts covering dispatch overhead, CPU-bound, and stdlib-heavy profiles. Results published to gh-pages for trend tracking; regressions flagged on PRs.

**Current tier 1 runtimes:** Python

## Tier 2 — Works

A tier 2 runtime boots, runs guest code, and has integration tests covering the capabilities that apply to it. It does not have conformance testing against the upstream test suite or benchmark tracking.

**Requirements:**

- **Driver and Dockerfile** that produce a working rootfs CPIO.
- **Integration tests** covering inline and/or file execution, a snapshot round-trip, and the applicable capability surface — filesystem, networking, concurrency, stdin, environment variables — for the capabilities that apply to the runtime. (A shell runtime has no networking or threads; the Python-based runtimes inherit the tier-1 Python suite.) Failures block release.
- **Example script** in `examples/<runtime>/`.

**Current tier 2 runtimes:** Node.js, .NET (JIT and AOT), Bash (BusyBox), Agent, Python-Shell

## Tier 3 — Builds and smoke-tested

A tier 3 runtime has a Dockerfile that produces a rootfs and a small **smoke test** (it boots, runs a hello, and survives a snapshot round-trip), but no broad capability coverage (filesystem, networking, concurrency, stdin, …) and no conformance or benchmark tracking. It is the more experimental entry point for runtimes: the smoke tests run in CI and block release, but a tier 3 runtime may still hit unexercised edges.

**Requirements:**

- **Driver and Dockerfile** in `drivers/<runtime>/` that produce a valid rootfs CPIO via `just build-rootfs <runtime>`.
- **Smoke test** covering boot + execution and a snapshot round-trip (e.g. `tests/compiled.rs`, `tests/powershell.rs`).
- **Example script** in `examples/<runtime>/` (recommended but not required).

**Current tier 3 runtimes:** C, Rust, Go, PowerShell

## Promotion and Demotion

**Tier 3 → Tier 2:** Broaden the integration tests from a boot/run/snapshot smoke test to full capability coverage (filesystem, networking, stdin, concurrency, and any runtime-specific surface) in `tests/<runtime>.rs`.

**Tier 2 → Tier 1:** Add conformance test infrastructure (`conformance/<runtime>/`), run the full upstream test suite, create a known-failures manifest, and add benchmark workloads with CI tracking.

**Demotion:** A runtime is demoted if its tier requirements are no longer met — e.g., integration tests are disabled or consistently skipped, conformance results degrade without manifest updates, or the driver Dockerfile no longer builds.
