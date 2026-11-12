# Kernel

The embedded kernel is a Unikraft app-elfloader built against `jprendes/unikraft`'s `feat/async-host-calls` branch, based on `unikraft/unikraft`'s `plat-hyperlight-v2` branch. Its `plat/hyperlight` carries the cooperative step model (`step.c`, `/dev/hlcall`) and async host-call protocol that let the host drive long-running guests and snapshot them mid-run. There is one binary per architecture:

| Binary | Hosts |
|--------|-------|
| `elfloader_hyperlight-x86_64` | Linux (KVM, MSHV), Windows (WHP) |
| `elfloader_hyperlight-arm64` | macOS on Apple silicon (HVF), Linux arm64 (KVM) |

Users don't need to build these — `hluk` embeds the one for its host via `include_bytes!`. Only the rootfs (built with `just build-rootfs`) needs to be produced by users.

## Configuration

See `../defconfig-elfloader` for the full kconfig used to build this kernel. The arm64 build adds `../defconfig-elfloader.arm64` on top.

## Rebuilding from source

The kernel is built from three git submodules pinned under `kernel/`:

| Submodule | Source | Branch |
|-----------|--------|--------|
| `unikraft` | [jprendes/unikraft](https://github.com/jprendes/unikraft) | `feat/async-host-calls` |
| `app-elfloader` | [danbugs/app-elfloader](https://github.com/danbugs/app-elfloader): `staging` plus five fixes, one commit each, until they are upstream: arm64 `AT_HWCAP` (upstream has it in another form, which needs a Unikraft change this kernel lacks), `AT_RANDOM` and `AT_EXECFN` on the program's stack, empty argument and environment strings, and freeing the ELF program record after exec | `plat-hyperlight-v2` |
| `libs/libelf` | [unikraft/lib-libelf](https://github.com/unikraft/lib-libelf) | `staging` |

```bash
# Initialise submodules (first time only)
git submodule update --init --recursive

# Build both kernels (or one: just build-kernel arm64)
just build-kernel

# Verify the committed binaries match source (CI uses this)
just verify-kernel
```

Both are built in an amd64 Docker builder (emulated on an arm64 host), so they come out the same on any host; the arm64 one is cross-compiled (`gcc-aarch64-linux-gnu` in `Dockerfile.build`).

The build is reproducible — `CONFIG_LIBUKLIBID_INFO_COMPILEDATE=n` in the defconfig ensures the same source always produces the same binary.

<!-- TODO: upstream kernel changes to kraft so this can be built with
     `kraft build --plat hyperlight --arch x86_64` without manual patching. -->
