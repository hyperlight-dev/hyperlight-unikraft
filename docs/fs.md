# Filesystem

Unikraft guests on Hyperlight have two filesystem layers: a **guest filesystem** (in-memory, from the initrd) and an optional **host filesystem** (pass-through to the host via hypercalls).

## Guest filesystem

At boot the kernel mounts a RAM filesystem (`ramfs`) at `/` and extracts the CPIO initrd over it.  The result is a fully writable in-memory filesystem containing the guest's root — Python stdlib, shared libraries, `/bin`, `/tmp`, etc.

Writes go to ramfs (pure guest memory — no host calls, no disk I/O). Data is ephemeral: it disappears when the VM shuts down or is restored from a snapshot.  The guest filesystem is always read-write; there is no read-only option for it.

### Example

[`examples/python/guest_fs.py`](../examples/python/guest_fs.py) exercises the guest filesystem: writes to `/tmp`, reads back, creates nested directories, renames, lists entries, and cleans up.

## Host filesystem (`hostfs`)

`hostfs` exposes a host directory inside the guest.  It is backed by Hyperlight host functions and sandboxed by [`cap-std`](https://docs.rs/cap-std).

### Usage

Mount a host directory with `--mount`.  The format is `HOST_PATH:GUEST_PATH[:ro]` — the host path comes first, the guest mount point second (same as `docker -v`):

```sh
# Read-write: host /tmp/share appears at /mnt/host in the guest
hluk run --initrd rootfs.cpio --mount /tmp/share:/mnt/host script.py

# Read-only (append :ro)
hluk run --initrd rootfs.cpio --mount /data:/mnt/data:ro script.py

# Multiple mounts
hluk run --initrd rootfs.cpio \
    --mount /tmp/share:/mnt/host \
    --mount /data:/mnt/data:ro \
    script.py
```

From the Rust API (parameter order: host path, guest path):

```rust
use hyperlight_unikraft::{Mount, SandboxBuilder};

let mut guest = SandboxBuilder::from_initrd("rootfs.cpio")
    .scratch_mb(256)
    .mount(Mount::rw("/tmp/share", "/mnt/host"))
    .mount(Mount::ro("/data", "/mnt/data"))
    .boot()?;
guest.run("open('/mnt/host/out.txt', 'w').write('hello')")?;
```

Windows drive-letter paths are supported: `C:\data:/mnt/data:ro`.

### How it works

```
Guest VFS                          Host (Rust)
─────────                          ──────────
open("/mnt/host/foo.txt")
  → hostfs vnode ops
    → hl_hcall("fs_read_bytes")
      → VM exit                    hostfs.rs
                                     cap_std::fs::Dir::open()
                                     → openat2(RESOLVE_BENEATH)
                                     → read()
      ← VM enter (packed bytes)
    → copy to userspace
```

Each mount corresponds to a `cap_std::fs::Dir` on the host side.  The guest kernel injects a mount index into every host call so the host routes operations to the correct directory.

Reads and writes are transferred in chunks: the host's preferred size (`GetHostFsChunkSize`, 32 KiB), or less if one host call cannot carry that much — the guest reads the limit out of the PEB I/O stack sizes the host chose.

Two limits of the protocol as it stands.  A directory listing is one host call, so a directory whose names do not fit in it (about 3,000 entries) does not list: `readdir` fails with `EOVERFLOW` and the guest goes on (TODO: page `fs_list`).  `stat` carries the size and whether the entry is a file or a directory, nothing more: mode reads as `755` (`555` on a read-only file), timestamps as the epoch, and a symlink as the file it points at (TODO: carry the real mode, the times and an `lstat`).

### Supported operations

| Operation | Host function   | Notes                            |
|-----------|-----------------|----------------------------------|
| stat      | `fs_stat`       |                                  |
| read      | `fs_read_bytes` |                                  |
| write     | `fs_write_bytes`| creates file if it doesn't exist |
| mkdir     | `fs_mkdir`      |                                  |
| unlink    | `fs_unlink`     | removes files and directories    |
| truncate  | `fs_truncate`   |                                  |
| readdir   | `fs_list`       |                                  |
| rename    | `fs_rename`     | within the same mount            |
| symlink   | `fs_symlink`    |                                  |
| readlink  | `fs_readlink`   |                                  |
| hard link | `fs_link`       | within the same mount            |
| chmod     | `fs_chmod`      |                                  |

### Read-only mounts

Passing `:ro` sets `MNT_RDONLY` on the VFS mount.  The kernel rejects writes at the VFS layer (returns `EROFS`) before they reach hostfs. The host side also enforces read-only as defense-in-depth.

### Snapshots

Mounts belong to the run: give them to `hluk run` or `hluk snapshot run` (or to the `SandboxBuilder` that boots or restores), and save a snapshot without any.  On its `resume` entry the kernel fetches the host's mount list through the `GetMounts` host function, the same `vfs.fstab` entries a boot carries, and makes its mount table match: it mounts what the list adds, unmounts what the list lacks, and remounts an entry whose index or read-only flag changed.  Every hostfs node names its mount by index, so the host directory behind a guest path can differ between save and restore.

```sh
hluk snapshot save --initrd rootfs.cpio --output .snapshots/python
hluk snapshot run .snapshots/python --mount /tmp/share:/mnt/host script.py
```

A mount kept busy across the snapshot, by a file left open on it, stays mounted until a restore finds it free; meanwhile its operations fail with `ESTALE`, since the host serves other mounts under its index now.

A mount table holds at most 32 mounts and 3.5 KiB of entries, the kernel's bounds; a boot or restore with more is refused with `Error::MountTable`.

### Security

All host filesystem access is sandboxed by [`cap-std`](https://docs.rs/cap-std) (capability-based filesystem access):

- **Path traversal (`../`)**: blocked.  On Linux 5.6+, `openat2` with `RESOLVE_BENEATH` is kernel-enforced.  On Windows, cap-std uses component-by-component resolution that rejects escapes.
- **Symlink escapes**: blocked.  `RESOLVE_NO_MAGICLINKS` prevents `/proc/self/fd/N`-style escapes on Linux.  Absolute symlink targets are rejected on all platforms.
- **No ambient filesystem access**: the guest can only reach files inside the mounted host directory.
- **chmod**: the guest can change permissions on files in the host mount (same as virtio-fs / 9pfs).  Use `:ro` mounts to prevent this.

### Examples

- [`examples/python/fs_ops.py`](../examples/python/fs_ops.py) — write, read, stat, listdir, mkdir, walk, cleanup on a hostfs mount.
- [`examples/python/guest_fs.py`](../examples/python/guest_fs.py) — exercises the guest ramfs: mkdir, write, read, stat, rename, listdir, cleanup.

### Guest vs host filesystem

| Aspect     | Guest (`ramfs`)         | Host (`hostfs`)              |
|------------|-------------------------|------------------------------|
| Source     | initrd CPIO             | host directory               |
| Writable   | always (RAM-backed)     | yes (unless `:ro`)           |
| Persistent | no (lost on VM exit)    | yes (host filesystem)        |
| Isolation  | fully in-guest memory   | sandboxed by `cap-std`       |
