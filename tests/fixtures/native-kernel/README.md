# Native-kernel test fixture

A **native** Unikraft image for the Hyperlight platform: `helloworld.c`'s `main()` is compiled *directly into the kernel* (no ELF loader, no initrd) — the opposite of the runtime model hluk normally uses (the embedded elfloader kernel + a rootfs CPIO). It exists to prove `SandboxBuilder::from_kernel(...)` / `hluk run --kernel <path>` can boot a kernel other than the embedded one.

`tests/native_kernel.rs` boots this via `init()` and asserts the greeting was captured. Note the different execution model: a native app runs its workload at *boot* (during `init()`/evolve), not on a later `run(Exec)` dispatch, so the test never calls `run()`.

## Two platform fixes that make native kernels work

Both are on the `plat-hyperlight-cleanup` branch of `unikraft` (the pinned submodule):

1. **`select LIBPOSIX_ENVIRON`** in `plat/hyperlight/Config.uk`. `dispatch.c` injects the `HL_*` pointers into the process environment with `putenv`/`setenv` (from `lib/posix-environ`). The elfloader pulled that in transitively; a native app doesn't, so the platform must declare it — else the kernel fails to link with `undefined reference to putenv`.

2. **Term-pointer fix** in `plat/hyperlight/dispatch.c`. A native app *returns* from `main()`, running the kernel's shutdown path (`uk_pm_shutdown` → term functions → `hyperlight_shutdown`, a clean port-108 halt). This exposed a latent bug: `hyperlight_dispatch_inject_host_env` was registered as `uk_late_initcall(fn, 0x1)`, but that macro's second argument is a **term-function pointer**, not a priority (`0x0` = "no term function"). The `0x1` was a bogus term pointer the shutdown term loop called → `rip: 0x1` crash. The elfloader never hit it (it runs on *dispatch* and never returns from `main`). Fixed to `0x0`.

## Build

Reproducibly, in the same Docker toolchain as the elfloader kernel:

```bash
just build-native-kernel    # rebuilds helloworld-native_hyperlight-x86_64 here
just verify-native-kernel   # rebuilds and checks the committed binary matches (CI gate)
```

The build invokes `make -C kernel/unikraft A=<app>` directly (no per-app `Makefile` — only `Makefile.uk`, which registers the source; the uk libs come from the pinned `kernel/unikraft` tree). The binary is deterministic (fixed container build paths + `CONFIG_LIBUKLIBID_INFO_COMPILEDATE=n`), so `verify-native-kernel` — run in the GitHub `kernel-verify` job — guards it against source drift, exactly like `verify-kernel` does for the elfloader kernel.

## Files

- `helloworld.c` — the native app (`main()` prints a greeting)
- `Makefile.uk` — Unikraft app build rules (registers `helloworld.c`)
- `defconfig` — native Hyperlight config (paging + vmem + posix-environ, no elfloader app)
- `helloworld-native_hyperlight-x86_64` — the prebuilt kernel the test loads
