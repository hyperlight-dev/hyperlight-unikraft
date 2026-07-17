# vscode-server — VS Code Server (REH-Web) for Hyperlight.
#
# Uses Alpine's musl PIE Node.js binary (required by elfloader) with
# the VS Code Server JS files. Native .node modules won't load (glibc
# ABI mismatch) but the server has fallbacks for all of them.

# Stage 1: Download VS Code Server (REH-Web) for Linux x64.
FROM ubuntu:22.04 AS download
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates
RUN curl -L "https://update.code.visualstudio.com/latest/server-linux-x64-web/stable" \
        -o /tmp/vscode-server.tar.gz && \
    mkdir -p /opt/vscode-server && \
    tar -xzf /tmp/vscode-server.tar.gz -C /opt/vscode-server --strip-components=1 && \
    rm /tmp/vscode-server.tar.gz

# Stage 2: Get musl PIE Node.js from Alpine.
FROM node:22-alpine AS node
RUN apk add --no-cache binutils && strip /usr/local/bin/node

# Stage 3: Rootfs assembly.
FROM scratch AS rootfs

# Musl dynamic linker + shared libraries
COPY --from=node /lib/ld-musl-x86_64.so.1    /lib/ld-musl-x86_64.so.1
COPY --from=node /usr/lib/libstdc++.so.6      /usr/lib/libstdc++.so.6
COPY --from=node /usr/lib/libgcc_s.so.1       /usr/lib/libgcc_s.so.1

# VS Code Server (JS files, extensions, resources)
COPY --from=download /opt/vscode-server /opt/vscode-server

# Replace bundled glibc Node.js with musl PIE Node.js
COPY --from=node /usr/local/bin/node /opt/vscode-server/node

# Busybox: use the static-PIE build from the shell-base image.
# Alpine's busybox-static is ET_EXEC (non-PIE) which the elfloader
# rejects; the shell-base image has a static-PIE (ET_DYN) build.
COPY --from=ghcr.io/hyperlight-dev/hyperlight-unikraft/shell-base:latest \
     /bin/busybox /bin/busybox

# Stage 4: Pack CPIO.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils ca-certificates
COPY --from=rootfs / /rootfs/

# DNS configuration
RUN mkdir -p /rootfs/etc && \
    echo "nameserver 8.8.8.8" > /rootfs/etc/resolv.conf && \
    echo "nameserver 8.8.4.4" >> /rootfs/etc/resolv.conf && \
    echo "hosts: files dns" > /rootfs/etc/nsswitch.conf && \
    echo "127.0.0.1 localhost" > /rootfs/etc/hosts

# CA certificates for TLS
RUN mkdir -p /rootfs/etc/ssl/certs /rootfs/usr/lib/ssl && \
    cp /etc/ssl/certs/ca-certificates.crt /rootfs/etc/ssl/certs/ && \
    ln -sf /etc/ssl/certs/ca-certificates.crt /rootfs/usr/lib/ssl/cert.pem

# Busybox applet symlinks
RUN for cmd in sh echo ls cat grep find wc head tail sort cut sed awk \
               cp mv rm mkdir rmdir ln basename dirname env sleep date \
               hostname id uname whoami which test true tr uniq tee xargs; do \
        ln -sf busybox /rootfs/bin/$cmd; \
    done

# VS Code Server expects bin/remote-cli/node for the extension host
# process.  Normally created at runtime, but cpiovfs is read-only.
RUN ln -sf ../../node /rootfs/opt/vscode-server/bin/remote-cli/node

# Writable directories
RUN mkdir -p /rootfs/tmp /rootfs/data

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
