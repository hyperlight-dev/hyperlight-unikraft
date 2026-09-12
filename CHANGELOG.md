# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Prerelease] - Unreleased

### Added

### Changed

### Removed

### Fixed

## [v0.13.0]

0.13.0 is a ground-up rewrite of the project since 0.12.1: a single `hluk` CLI plus a library, a typed guest-to-host boundary, one reproducible embedded Unikraft kernel, and a formal runtime support-tier policy. It is **not** CLI- or API-compatible with 0.12.x; see **Removed** and the behavioral notes under **Changed**.

### Added

- Single `hluk` CLI with subcommands `run`, `snapshot save` / `snapshot run`, and `bench` (`cold` / `cold-snap` / `warm-restore` / `warm-stateful` / `parallel`). Flags: `--mount HOST:GUEST[:ro]`, `--net` / `--net-allow` / `--net-block` / `--port`, `--exec`, `--guest-exec` (urunc-style, runs a binary baked into the initrd), `--entry`, `--env KEY=VALUE`, and `--scratch-mb`.
- Library API: a `SandboxBuilder` — `from_initrd(rootfs)` / `from_kernel` / `from_snapshot`, then `.boot()` to get a running sandbox — plus the `run` dispatch call, `Mount`, `Exec`, `GuestConfig`, and re-exported `Snapshot` / `OciTag`. `hluk` is built on the same API.
- 11 guest runtimes on a 3-tier support policy (`docs/guest-support-tiers.md`), a shared C driver toolkit, and a CPython conformance suite. `.NET` runs both ahead-of-time (AOT) and in-guest JIT (Roslyn source compilation).
- Typed `fs_*` / `net_*` host functions with new filesystem operations (`rename`, `symlink`, `readlink`, hard `link`, `chmod`), multi-mount support, `net_resolve` (host DNS), real guest stdin (`ReadStdin` + EOF), and host environment-variable passthrough (`--env`, re-applied across snapshot restore).
- `--kernel <path>` (advanced) to boot an external kernel instead of the embedded one, and a reproducible native-kernel test fixture (`just build-native-kernel` / `just verify-native-kernel`).
- Snapshot save/restore as OCI-layout directories; macOS (`hvf`) support; parallel multi-VM benchmarking; on-demand Windows surrogates.

### Changed

- **Guest-to-host boundary**: a single JSON-RPC `__dispatch` function plus a host-side `ToolRegistry` became many small, typed FlatBuffer host functions returning real `-errno` (Linux numbers, with a Win32/Winsock-to-Linux translation table).
- **Boot metadata**: pushed as magic-tagged TLVs prepended to the initrd became pulled via host functions at boot (`GetCmdLine`, `GetInitrdBase/Size`, `GetWallClockNs`, `GetEnvVars`, …), re-answerable after a snapshot restore.
- **Filesystem sandbox**: a hand-rolled path resolver became [`cap-std`](https://docs.rs/cap-std) with kernel-enforced `openat2(RESOLVE_BENEATH)` on Linux (component-by-component resolution on Windows). **Networking**: blocking `socket2` became non-blocking `rustix` with a `net_poll` readiness model.
- **Guest memory**: a no-paging design with a bespoke `lib/cpiovfs` and `lib/ukmmap` became the standard Unikraft paging/vmem/mmap stack with a ramfs initrd.
- **Kernel provisioning**: per-example `kraft` builds became one reproducible, Docker-built Unikraft kernel embedded in the binary via `include_bytes!` and verified in CI (kraftkit dropped).
- **VMM**: `hyperlight-host` 0.16 to 0.17. On Linux the default build is KVM-only (for the fast `MADV_DONTNEED` snapshot-restore path); MSHV support is behind `--features mshv`.
- **Behavioral (breaking)**: in 0.12.x `--port` implied `--net`; now `--port` requires an explicit `--net` (or `--net-allow` / `--net-block`) and errors on its own. Under `--net`, the default `AllowAll` policy permits the host loopback interface (needed for intra-guest server+client patterns); `--net-allow` / `--net-block` still block loopback.
- **PowerShell** runtime moved from a 7.6.x tarball to the 7.4 (LTS) Alpine image.
- The Unikraft kernel is now embedded by default (0.12.x took a positional kernel path); use `--kernel` to override it.

### Removed

- Wasm custom host tools (`--tool`, `--tool-wasi-*`), the public `ToolRegistry` / `SandboxBuilder::tool()` extension API, and the WASIp1 host-function sandbox.
- The `pyhl` Python tool (image pull from GHCR, warm-then-snapshot, `--deterministic`) and the `multifn-test` / `pydriver-run` dev binaries; the snapshot workflow folded into `hluk snapshot`.
- The `--memory` / `--stack` knobs (replaced by `--scratch-mb`), the reserved-mountpoint rejection list (`/`, `/bin`, `/dev`, `/proc`, `/sys`, `/usr`), and the implicit default `/host` mount path (mounts now require an explicit guest path).
- Kernel-internal: the bespoke `lib/cpiovfs`, the `/dev/hcall` userspace device, the per-syscall TSC profiler, and the trace ports. File-backed `mmap` and demand paging move from the custom `lib/ukmmap`/`cow.c` to the standard `ukvmem` fault handler. (`mmap`'s `mremap` is currently `ENOSYS`; glibc tolerates it.)
- The `app-elfloader` and `kraftkit` forks; the platform builds against upstream `app-elfloader` and a single `unikraft` kernel fork.
- Docs: the old `host_functions.md` (dispatch wire format / attack surface) and `python-packages.md`.

### Fixed

- Host-mount path-escape resolution is now OS-enforced (`openat2` `RESOLVE_BENEATH` / `RESOLVE_NO_MAGICLINKS` on Linux; component-by-component on Windows), with defense-in-depth `:ro` enforcement on both the VFS and host sides.
- Guest output is captured via a `HostPrint` host function (one VM exit per buffer instead of per byte) and exposed programmatically through `GuestConfig::drain_output`.
