# Supply chain attack demo rootfs
#
# Bundles the typosquatted `reqeusts` package and victim app into a
# Python rootfs for execution inside a Hyperlight micro-VM.
#
# Build:
#   docker build --platform linux/amd64 --target cpio -t supply-chain-cpio -f Dockerfile ..

ARG BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/python-base:latest
FROM ${BASE} AS rootfs

# Install the typosquatted package into site-packages.
# The base image is a shell-less rootfs so RUN is not available; COPY only.
COPY reqeusts/ /usr/local/lib/python3.12/site-packages/reqeusts/

# Install the victim application
COPY victim_app.py /victim_app.py

# --- CPIO rootfs builder ---
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils
COPY --from=rootfs / /rootfs/
RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
