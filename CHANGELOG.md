# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Prerelease] - Unreleased

### Fixed

- `connect(2)` with `AF_UNSPEC` dissolves a datagram socket's association, as on Linux, instead of failing. glibc's `getaddrinfo` relies on it to probe every candidate of a dual-stack answer through one IPv6 socket, so a passive lookup -- `socket.getaddrinfo(None, port, AF_UNSPEC, SOCK_STREAM, 0, AI_PASSIVE)`, what Python's `http.server` does to bind -- no longer aborts the guest with a glibc assertion on the source address. Kernel: `hostsock` forwards it as the new `net_disconnect` host function.

## [v0.14.0]

### Added

- **Cooperative step model** for long-running guests. The kernel hands the vCPU back to the host when the guest goes idle, reporting when its next timer is due, instead of spinning in the VM; the host waits on that timer and on the guest's sockets, then re-enters. A guest with nothing left to wake it is reported deadlocked instead of hung forever, and a guest can be snapshotted at any boundary -- even mid-call -- and resumed later, in the same process or from disk. Library: `SandboxBuilder::boot` returns an `AppSandbox`; `run` dispatches a call and waits, `submit` dispatches without waiting, `step` advances the guest one boundary, `join` drives an entry-point workload to exit, and `snapshot` / `from_snapshot` (or `restore` in place) checkpoint and resume it, sockets included. `Yield` says why a step returned.
- A plain Linux binary can be the guest's entry point (`--entry "/bin/server --flag"`) with no runtime driver; `hluk run` drives it until it exits, and the library exposes it as `AppSandbox::join`. This is how a container runtime or an actor host runs a server in the guest.
- `asyncio` now works in the Python guest, whose event loop needs the Unix-domain sockets the kernel now enables (`examples/python/asyncio_demo.py`).
- The host learns how a call and the guest ended: a failed call carries a status (`Yield::CallFailed { status }` -- the program's own exit code, 1 for an uncaught exception, -1 when the driver could not run it), and a process exit carries the guest's status (`Yield::Exited { status }`, returned by `join`; `hluk run` exits with it). A guest that halts without reporting is an error, not an exit of unknown status.
- A restored guest is re-seeded and reconnected: the kernel reseeds its CSPRNG on restore (two clones no longer draw the same `os.urandom`, UUIDs or TLS nonces) and re-establishes its sockets (listeners rebind; connections whose peers died read as closed), so a checkpointed server keeps serving after a restore with nothing saved beside the snapshot.
- Runtime drivers receive calls through `/dev/hlcall`, each on its own thread. The device's `HLCALL_IOC_MAXLEN` ioctl reports the largest call the host can send, and `HLCALL_IOC_GETENV` hands a driver the host's current environment to refresh before each call.
- `AppSandbox::snapshot_to(dir)`, `SandboxBuilder::from_snapshot_dir(dir)` and `AppSandbox::restore_from(dir)` move a snapshot through disk. It is named by the crate version inside the directory; a load by another version fails and names the versions present.
- New docs: `docs/execution.md` (the step model and sandbox lifecycle), `docs/driver.md` (the `/dev/hlcall` driver contract and how to write a driver), `docs/clock.md` and `docs/random.md` (where the guest's clocks and random bytes come from, and what a restore does to them).

### Changed

- Network-policy hostnames are enforced at the DNS question and the destination. Under an allow list, only listed names may be looked up and only listed, resolved-at-build, or DNS-learned addresses may be reached -- the host no longer resolves names for the guest, so an address it was never given is refused. Under a block list, a blocked name is refused and re-checked at each connect (250 ms deadline; refused if the lookup fails or runs late). Malformed or non-query traffic to port 53 is refused, and the allow list's DNS exemption is UDP-only. Before, a name was enforced only through addresses the host re-resolved at every connect, so a blocked name that moved was reachable until the resolver caught up, and an allow-listed guest could query any name.
- `hluk run` and `hluk snapshot run` exit with the guest's status when the guest is what failed: `sys.exit(3)` is exit code 3 with nothing added, as running the script directly would be. A malformed `--env` (no `=`) or `--mount` (no `:`) is now an error instead of being silently ignored, and `snapshot run` drives a restored entry-point guest to its exit like `run`.
- The crate has its own error type, `hyperlight_unikraft::Error`, and every public function returns `hyperlight_unikraft::Result`, with a variant per condition (`CallFailed`, `GuestExited`, `Deadlocked`, `NoDriver`, `CallInFlight`, and so on) plus `Hyperlight` for the hypervisor layer, so an embedder matches a failed call or a deadlock instead of parsing text. `set_env_vars`, which cannot fail, no longer returns a `Result`.
- A program the guest runs no longer inherits a driver's call device or pipes (both are close-on-exec).
- The .NET JIT driver sets the GC hard limit as a share of the guest's memory (`DOTNET_GCHeapHardLimitPercent`) instead of a fixed 768 MB that never engaged, so an allocation the guest cannot serve is now a catchable `OutOfMemoryException` instead of a SIGSEGV.
- `SandboxBuilder::boot` returns an `AppSandbox` instead of a `(MultiUseSandbox, GuestConfig)` tuple; `GuestConfig` is no longer public and the free `run` is now `AppSandbox::run`. `run` fails if the guest exits before the call returns, `join` refuses a driver image with no call in flight, and `boot` refuses `kernel`/`initrd`/`entry`/`scratch_mb` on a `from_snapshot` builder or a mount path the kernel's `vfs.fstab` cannot carry (whitespace, `:`, brackets, or a relative path), instead of ignoring them.
- **Breaking, guest side**: the driver protocol changed (see Removed), so rootfs images built for 0.13.0 must be rebuilt with `just build-rootfs`.
- Kernel: guest sockets are more robust -- a `recv`/`send` on a connection whose peer died or whose host socket is gone (after a restore, or a reset while parked) returns instead of hanging, a `select`/`accept` loop no longer livelocks, and a server polling more than 64 sockets no longer stops waking.
- Kernel: transfer buffers are sized from the host's I/O stacks instead of hard-coded literals, so a socket send carries the full 64 KiB, a directory listing or symlink target is no longer cut at 8 KiB / 1 KiB, and an environment over 4 KiB no longer vanishes.
- Kernel: the periodic CSPRNG reseed timer is off, so an idle guest no longer wakes the host every 300 s, and a guest with nothing to wake it is reported deadlocked instead of waited on forever. The CSPRNG is still seeded at boot and on every restore.
- `AppSandbox::set_env_vars` no longer rejects keys starting with `HL_`; the kernel reserves no keys now.
- `AllowList::from_hosts` and `BlockList::from_hosts` fail with a `ResolveError` (naming the entry and the resolver error) instead of a `String`; several host-side policy internals are no longer public.

### Removed

- The `net_resolve` host function (unused; it ran a blocking, unfiltered resolver lookup on the vCPU thread).
- The `host_nanosleep` host function (unused; it stalled the embedder's thread up to 30 s). `net_poll` now refuses a non-zero timeout: waiting is the host's job between entries.
- The callback-and-halt driver protocol: drivers no longer write a callback pointer and halt the VM themselves, and the `HL_*` dispatch and env addresses are gone from the guest environment. `HLCALL_IOC_GETENV` replaces the raw env-refresh function pointer, and `hl_driver_init` no longer takes `envp`.
- `SNAPSHOT_TAG` and the `OciTag` re-export: embedders no longer name snapshots themselves.

### Fixed

- Interactive programs no longer echo every character twice: the serial terminal now honors the `ECHO` flag and stores the `termios` a program sets, so a shell (`hluk run --entry /bin/sh`) can turn echo off, while a program that leaves `ECHO` on still has its input echoed once.
- The Python drivers set `PATH=/usr/local/bin:/usr/bin:/bin` at startup, so `subprocess.run(["python3", ...])` and other bare-name lookups find the interpreter (a host `--env PATH` still overrides it).
- AWS's IPv6 instance-metadata address (`fd00:ec2::254`) is refused under every network policy, like the link-local metadata addresses already were.
- Snapshotting a guest whose process had already exited produced an unresumable image; `AppSandbox::snapshot` (and `snapshot_to`, `hluk snapshot save`) now refuses with `Error::GuestExited`.
- Listing a mounted directory too large for one host call (about 3,000 entries) poisoned the sandbox; the host now refuses that one listing with `EOVERFLOW` and the guest goes on. Other listing errors now reach the guest with the right errno instead of `EIO`.
- The driver FunctionCall reader (`hl_fc.h`) bounds-checks every offset, so a malformed call fails instead of reading out of bounds.
- Kernel: signal-handling fixes (`sigaltstack`) for a runtime that manages its own alternate signal stacks, so the .NET workers no longer crash the kernel when the guest runs out of memory.
- The exec driver (C, C++, Rust, Go, .NET AOT) and the PowerShell driver report the program's exit status: a non-zero exit, a signal, a C++ `std::terminate` or `exit 3` now fails the call. Before, they took the closed exit pipe for success whatever the program did.
- Kernel: a program that returns from `main()` while another of its threads sleeps no longer hangs the guest.
- The Node driver reads the next call asynchronously, so its event loop keeps turning: an `unref()`ed timer or handle left behind makes progress between calls, and the child no longer exits (and deadlocks the next call) after leaving only unref'd work.
- Kernel: a guest kernel crash now ends the call with a `GuestAborted` error and the crash dump in the output, instead of the vCPU running on and the guest spinning at 100% CPU.
- An exit inside a call ends that call with its status and leaves the runtime ready for the next, in every runtime: Python catches `SystemExit`; Node turns `process.exit()`, `process.exitCode` and uncaught errors into the status and stops the timers, servers and sockets the call left; the .NET JIT and bash drivers take a child's exit status and respawn for the next call. `sys.exit(0)` / `process.exit(0)` succeed; a non-zero code fails the call. Before, an exit could take down the whole guest or deadlock the next call.
- The Python drivers release the interpreter lock while parked between calls, so a thread or server a call left behind makes progress between calls instead of starving and waking the guest every 5 ms.
- A host environment value with a quote, backslash, newline or shell syntax no longer breaks or alters the call: the Python drivers set `os.environ` through the C API, and the bash, Node and .NET drivers quote values as literals of their language. The 4 KiB cap that silently dropped variables is gone.
- A `connect()` to a peer that never answers no longer freezes the vCPU for the OS's roughly two-minute SYN timeout: the host returns `EINPROGRESS`, the guest parks on writability, and a guest-side timeout (`socket.settimeout`) is honored.
- The network policy read an IPv4-mapped IPv6 address (`::ffff:169.254.169.254`) as an unrelated address, so a dual-stack socket could bypass the loopback, link-local and `BlockList` rules. Addresses are now checked in their IPv4 form.
- Data arriving on a socket no guest thread was reading made the host spin a full core per idle guest; the host now watches a socket for readability only while the guest is parked on it.
- The guest clock no longer runs 12% fast: the kernel asks the host for the TSC frequency (`GetTscHz`) instead of assuming 2.5 GHz.
- A guest restored from a snapshot reports the current time: the kernel re-anchors its wall clock on the host's during restore.
- `SandboxBuilder::env` variables now reach a program that is the guest's entry point, not only driver-served calls.
- Kernel: `clock_nanosleep` is implemented for the monotonic and realtime clocks (relative and `TIMER_ABSTIME`), so CPython's `time.sleep()` no longer returns immediately.
- A dispatched call is no longer capped at 16 KiB ("FunctionCall too large"); the call buffer is sized from the host's I/O buffer.

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
