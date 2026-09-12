# Rust on Hyperlight

Compile a static-PIE musl binary:

```bash
rustc -C opt-level=2 \
      -C target-feature=+crt-static \
      -C relocation-model=pie \
      -o hello hello.rs
```

Or with Cargo (add to `.cargo/config.toml`):

```toml
[build]
rustflags = ["-C", "target-feature=+crt-static", "-C", "relocation-model=pie"]
```

The binary must be position-independent (`relocation-model=pie`) for the elfloader.  Static CRT linking (`target-feature=+crt-static`) avoids runtime library dependencies in the rootfs.

Run it by mounting the directory containing the binary:

```bash
hluk run --initrd ../../build-elfloader/rust-rootfs.cpio --scratch-mb 64 \
         --mount ./:/mnt/bin --exec /mnt/bin/hello
```

Or with a snapshot (save once, run many):

```bash
# Save a snapshot with the mount point configured
hluk snapshot save --initrd ../../build-elfloader/rust-rootfs.cpio --scratch-mb 64 \
                   --mount ./:/mnt/bin -o ../../.snapshots/rust

# Run from snapshot — mount the directory with the binary
hluk snapshot run ../../.snapshots/rust --mount ./:/mnt/bin --exec /mnt/bin/hello
```
