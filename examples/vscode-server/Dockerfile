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

# Patch VS Code Server JS for single-vCPU cooperative scheduling:
# 1. Gallery manifest barrier bypass: fall back to product.json config
#    instead of blocking forever on browser IPC that can't arrive during
#    extension host init.
RUN sed -i 's|async getExtensionGalleryManifest(){return await this.barrier.wait(),this._extensionGalleryManifest??null}|async getExtensionGalleryManifest(){if(!this.barrier.isOpen()){let _f=await super.getExtensionGalleryManifest();if(_f){this._extensionGalleryManifest=_f;this.barrier.open()}}return this._extensionGalleryManifest??null}|g' \
        /rootfs/opt/vscode-server/out/server-main.js
# 2. WebSocket heartbeat timeout 20s -> 120s (server-side)
RUN sed -i \
        -e 's|e>=2e4\&\&n>=2e4\&\&o>=2e4\&\&!this._loadEstimator|e>=12e4\&\&n>=12e4\&\&o>=12e4\&\&!this._loadEstimator|g' \
        -e 's|e>=2e4\&\&n>=2e4\&\&!this._loadEstimator|e>=12e4\&\&n>=12e4\&\&!this._loadEstimator|g' \
        -e 's|Math.max(2e4-e,2e4-n,2e4-o,500)|Math.max(12e4-e,12e4-n,12e4-o,500)|g' \
        /rootfs/opt/vscode-server/out/server-main.js
# 3. WebSocket heartbeat timeout 20s -> 120s (browser-side)
RUN sed -i \
        -e 's|e>=2e4\&\&t>=2e4\&\&i>=2e4\&\&!this._loadEstimator|e>=12e4\&\&t>=12e4\&\&i>=12e4\&\&!this._loadEstimator|g' \
        -e 's|e>=2e4\&\&t>=2e4\&\&!this._loadEstimator|e>=12e4\&\&t>=12e4\&\&!this._loadEstimator|g' \
        -e 's|Math.max(2e4-e,2e4-t,2e4-i,500)|Math.max(12e4-e,12e4-t,12e4-i,500)|g' \
        /rootfs/opt/vscode-server/out/vs/code/browser/workbench/workbench.js

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
