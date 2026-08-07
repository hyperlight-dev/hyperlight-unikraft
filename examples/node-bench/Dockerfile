# node-bench — Node.js + npm for Hyperlight benchmarks.
#
# Includes Node.js runtime and npm for building and testing
# Node.js projects inside a Hyperlight micro-VM.
#
# Build:
#   docker build --platform linux/amd64 --target cpio -t node-bench-cpio .
#   docker create --name node-bench-tmp node-bench-cpio /bin/true
#   docker cp node-bench-tmp:/output.cpio ./node-bench-initrd.cpio
#   docker rm -f node-bench-tmp

ARG NODE_BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/nodejs-base:latest
ARG NODE_VERSION=22

# Stage 1: Get npm from official Node.js Alpine image.
FROM node:${NODE_VERSION}-alpine AS npm-source

# Stage 2: Pack CPIO. Do all assembly here since nodejs-base has no shell.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils

# Copy nodejs-base rootfs
COPY --from=ghcr.io/hyperlight-dev/hyperlight-unikraft/nodejs-base:latest / /rootfs/

# Copy npm and node_modules from Node.js Alpine
COPY --from=npm-source /usr/local/lib/node_modules /rootfs/usr/local/lib/node_modules
RUN mkdir -p /rootfs/usr/local/bin

# npm/npx launcher scripts (node is at /usr/bin/node in nodejs-base)
RUN printf '#!/usr/bin/node\nrequire("/usr/local/lib/node_modules/npm/bin/npm-cli.js")\n' \
    > /rootfs/usr/local/bin/npm && chmod +x /rootfs/usr/local/bin/npm
RUN printf '#!/usr/bin/node\nrequire("/usr/local/lib/node_modules/npm/bin/npx-cli.js")\n' \
    > /rootfs/usr/local/bin/npx && chmod +x /rootfs/usr/local/bin/npx

# /tmp for npm cache and temp files
RUN mkdir -p /rootfs/tmp /rootfs/root/.npm

# Benchmark project — pure Node.js, no external dependencies
COPY project/ /rootfs/app/

# Busybox for shell support (static-PIE build from shell-base)
COPY --from=ghcr.io/hyperlight-dev/hyperlight-unikraft/shell-base:latest \
     /bin/busybox /rootfs/bin/busybox
RUN for cmd in sh echo ls cat grep find wc head tail sort cut sed awk \
               cp mv rm mkdir rmdir ln basename dirname env sleep date \
               hostname id uname whoami which test true tr uniq tee xargs \
               tar gzip gunzip; do \
        ln -sf busybox /rootfs/bin/$cmd; \
    done

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
