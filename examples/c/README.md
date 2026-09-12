# C on Hyperlight

Compile a static-PIE binary with musl:

```bash
gcc -O2 -Wall -static-pie -fPIE -o hello hello.c
```

The binary must be position-independent (`-fPIE -static-pie`) for the elfloader.  Static linking (`-static-pie`) avoids runtime library dependencies in the rootfs.

Run it by mounting the directory containing the binary:

```bash
hluk run --initrd ../../build-elfloader/c-rootfs.cpio --scratch-mb 64 \
         --mount ./:/mnt/bin --exec /mnt/bin/hello
```

Or with a snapshot (save once, run many):

```bash
# Save a snapshot with the mount point configured
hluk snapshot save --initrd ../../build-elfloader/c-rootfs.cpio --scratch-mb 64 \
--mount ./:/mnt/bin -o ../../.snapshots/c

# Run from snapshot — mount the directory with the binary
hluk snapshot run ../../.snapshots/c --mount ./:/mnt/bin --exec /mnt/bin/hello
```
