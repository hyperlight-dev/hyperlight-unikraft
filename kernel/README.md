# Kernel

The embedded kernel binary (`elfloader_hyperlight-x86_64`) is a Unikraft app-elfloader built against the `plat-hyperlight-cleanup` branch.

Users don't need to build this — it's embedded in the `hluk` binary via `include_bytes!`. Only the rootfs (built with `just build-rootfs`) needs to be produced by users.

## Configuration

See `../defconfig-elfloader` for the full kconfig used to build this kernel.

## Rebuilding from source

The kernel is built from three git submodules pinned under `kernel/`:

| Submodule | Source | Branch |
|-----------|--------|--------|
| `unikraft` | [danbugs/unikraft](https://github.com/danbugs/unikraft) | `plat-hyperlight-cleanup` |
| `app-elfloader` | [unikraft/app-elfloader](https://github.com/unikraft/app-elfloader) | `staging` |
| `libs/libelf` | [unikraft/lib-libelf](https://github.com/unikraft/lib-libelf) | `staging` |

```bash
# Initialise submodules (first time only)
git submodule update --init --recursive

# Build the kernel
just build-kernel

# Verify a committed binary matches source (CI uses this)
just verify-kernel
```

The build is reproducible — `CONFIG_LIBUKLIBID_INFO_COMPILEDATE=n` in the defconfig ensures the same source always produces the same binary.

<!-- TODO: upstream kernel changes to kraft so this can be built with
     `kraft build --plat hyperlight --arch x86_64` without manual patching. -->
