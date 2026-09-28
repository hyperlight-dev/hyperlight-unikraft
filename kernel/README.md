# Kernel

The embedded kernel is a Unikraft app-elfloader built against `unikraft/unikraft`'s `plat-hyperlight-v2` branch, whose `plat/hyperlight` carries the cooperative step model (`step.c`, `/dev/hlcall`) that lets the host drive long-running guests and snapshot them mid-run. There is one binary per architecture:

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
| `unikraft` | [unikraft/unikraft](https://github.com/unikraft/unikraft) | `plat-hyperlight-v2` |
| `app-elfloader` | [danbugs/app-elfloader](https://github.com/danbugs/app-elfloader), `staging` plus the arm64 `AT_HWCAP` fix, until [unikraft/app-elfloader#105](https://github.com/unikraft/app-elfloader/pull/105) merges | `arm64-at-hwcap` |
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
