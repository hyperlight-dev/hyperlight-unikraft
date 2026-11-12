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

From the Rust API, inside a Tokio runtime with time and I/O enabled
(parameter order: host path, guest path):

```rust
use hyperlight_unikraft::{Mount, SandboxBuilder};

let mut guest = SandboxBuilder::from_initrd("rootfs.cpio")
    .scratch_mb(256)
    .mount(Mount::rw("/tmp/share", "/mnt/host"))
    .mount(Mount::ro("/data", "/mnt/data"))
    .boot()?;
guest.run("open('/mnt/host/out.txt', 'w').write('hello')").await?;
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

Reads and writes are transferred in chunks: the host's preferred size (`GetHostFsChunkSize`, 64 KiB), or less if one host call cannot carry that much — the guest reads the limit out of the virtqueue pool sizes the host chose.

Paths travel to the host as UTF-8 strings: a name that is not UTF-8 fails with `EILSEQ`, and a host file whose name is not UTF-8 lists with U+FFFD in its place (and cannot be opened by that name). The path below the mount's root is at most 1,023 bytes; a longer one fails with `ENAMETOOLONG`.

Limits of the protocol as it stands.  A directory listing is one host call, so a directory whose names do not fit in it (about 3,000 entries) does not list: `readdir` fails with `EOVERFLOW` and the guest goes on (TODO: page `fs_list`).  `stat` carries the size, the kind, the host's mode bits (on Windows `755`, or `555` for a read-only file) and the access, modification and status change times; a symlink reads as the file it points at (TODO: an `lstat`).

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
| chmod     | `fs_chmod`      | never setuid/setgid; on Windows, the read-only flag |
| utime     | `fs_set_times`  | not before 1970; on Windows, not a read-only file |

### Read-only mounts

Passing `:ro` sets `MNT_RDONLY` on the VFS mount.  The kernel rejects writes at the VFS layer (returns `EROFS`) before they reach hostfs. The host side also enforces read-only as defense-in-depth.

### Backends

A mount need not be a host directory.  `Mount::backend(backend, guest_path, readonly)` serves it with the embedder's `FsBackend`, which is handed each of the operations above as a method call (`stat`, `read`, `write`, `create_dir`, `remove`, `truncate`, `list`, `rename`, and `symlink`, `hard_link`, `chmod` and `set_times`, which by default refuse links and ignore modes and times) and decides it: an embedder that accounts for every byte the guest writes, keeps files somewhere other than a directory, or refuses some names, does it there, while the guest's programs, C extensions and child processes included, use the files as on any mount.

```rust
use std::io;
use hyperlight_unikraft::{DirEntry, FileStat, FsBackend};

struct Quota { /* ... */ }

impl FsBackend for Quota {
    fn write(&self, path: &str, offset: Option<u64>, data: &[u8]) -> io::Result<()> {
        // ...past the quota: the guest's write fails with EDQUOT.
        Err(io::ErrorKind::QuotaExceeded.into())
    }
    // stat, read, create_dir, remove, truncate, list and rename too.
}
```

Every argument is the guest's, and untrusted.  A guest program's paths come through the kernel's VFS, clean: relative to the mount, `/`-separated, the empty path its root, at most 1023 bytes.  But native code in the guest can make the host calls itself, with any string (absolute, `..`, empty components, tens of KiB), and a write's offset can be any `u64`, so confining paths to the backend's own files and bounding offsets is the backend's job.

An error reaches the guest as the errno of its `io::ErrorKind` (`QuotaExceeded` is `EDQUOT`, `FileTooLarge` `EFBIG`, `PermissionDenied` `EACCES`), or as a raw OS error of the host's OS (one of 0 or less is `EIO`).  A panic in a backend method is caught and is `EIO` too.  The protocol is the one hostfs speaks to a directory, so its limits apply:

- No handles: each read or write names its path and offset, at most 64 KiB at a time.
- A directory is read by listing it once per entry and taking the next entry by its position, so `list` must return the same order while the directory is unchanged, and reading a large directory costs as many listings as it has entries.  A name that is empty, `.`, `..`, over 255 bytes, or has a `/` or NUL is left out.
- `stat` gives the size, the type and permission bits, and the times (`FileStat::times`, else the epoch): a file or a directory (`FileStat::file`, `FileStat::dir`), the only nodes the kernel makes on a hostfs mount.  So there are no symbolic links on a backend mount for now, whatever `symlink` does.

On a read-only mount the backend is asked only to read.  A backend runs while the guest is stopped in a host call, and the snapshot rules above hold for its mounts as for any.  A backend mount's `host_path` is empty.

### Snapshots

Mounts belong to the run: give them to `hluk run` or `hluk snapshot run` (or to the `SandboxBuilder` that boots or restores), and save a snapshot without any.  On its `resume` entry the kernel fetches the host's mount list (with `GetResumeState`, which also brings the clock and resolver; `GetMounts` answers the same on its own), the same `vfs.fstab` entries a boot carries, and makes its mount table match: it mounts what the list adds, unmounts what the list lacks, and remounts an entry whose index or read-only flag changed.  Every hostfs node names its mount by index, so the host directory behind a guest path can differ between save and restore.

```sh
hluk snapshot save --initrd rootfs.cpio --output .snapshots/python
hluk snapshot run .snapshots/python --mount /tmp/share:/mnt/host script.py
```

A mount kept busy across the snapshot, by a file left open on it, stays mounted until a restore finds it free; meanwhile its operations fail with `ESTALE`, since the host serves other mounts under its index now.

A mount table holds at most 32 mounts and 3.5 KiB of entries, the kernel's bounds; a boot or restore with more is refused with `Error::MountTable`.

### Security

All host filesystem access is sandboxed by [`cap-std`](https://docs.rs/cap-std) (capability-based filesystem access):

- **Path traversal (`../`)**: blocked.  On Linux 5.6+, `openat2` with `RESOLVE_BENEATH` is kernel-enforced.  On Windows and macOS, cap-std uses component-by-component resolution that rejects escapes.
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
