# Rust compiler (rustc) running inside Hyperlight
#
# Builds a CPIO rootfs containing the Rust compiler toolchain on Alpine Linux.
# The guest runs rustc directly to compile a Rust program inside the micro-VM.
#
# Build the rootfs:
#   docker build --platform linux/amd64 --target cpio -t rustc-cpio .

FROM alpine:3.21 AS toolchain

# Install the Rust compiler and musl development files.
RUN apk add --no-cache rust gcc musl-dev

# Remove files that aren't needed at runtime to shrink the CPIO image.
RUN rm -rf /var/cache/apk/* /var/lib/apk /etc/apk \
    /usr/share/man /usr/share/doc /usr/share/info /usr/share/locale \
    /media /mnt /opt /srv \
    /var/log /var/mail /var/spool

# Add the Rust source file to compile inside the VM.
COPY hello.rs /hello.rs

# --- CPIO rootfs builder ---
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils
COPY --from=toolchain / /rootfs/
RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
