# Go on Hyperlight

Compile a PIE binary with CGO disabled:

```bash
CGO_ENABLED=0 go build -buildmode=pie -ldflags='-s -w' -o hello hello.go
```

The binary must be position-independent (`-buildmode=pie`) for the elfloader.  `CGO_ENABLED=0` produces a pure Go binary.  `-ldflags='-s -w'` strips debug info to reduce size.

**Note:** Go PIE binaries on glibc hosts set the program interpreter to `/lib64/ld-linux-x86-64.so.2`.  The Go rootfs includes a compatibility symlink that maps this to musl's linker.

Run it by mounting the directory containing the binary:

```bash
hluk run --initrd ../../build-elfloader/go-rootfs.cpio --scratch-mb 128 \
         --mount ./:/mnt/bin --exec /mnt/bin/hello
```

Or with a snapshot (save once, run many):

```bash
# Save a snapshot with the mount point configured
hluk snapshot save --initrd ../../build-elfloader/go-rootfs.cpio --scratch-mb 128 \
                   --mount ./:/mnt/bin -o ../../.snapshots/go

# Run from snapshot — mount the directory with the binary
hluk snapshot run ../../.snapshots/go --mount ./:/mnt/bin --exec /mnt/bin/hello
```
