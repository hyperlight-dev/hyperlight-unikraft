# pandoc-bench — Stripped Python for Hyperlight document-conversion benchmark.
#
# Pure Python DOCX generation using only the standard library
# (zipfile + xml.etree.ElementTree). No external binaries needed.
#
# Build:
#   docker build --platform linux/amd64 --target cpio -t pandoc-bench-cpio .
#   docker create --name pandoc-bench-tmp pandoc-bench-cpio /bin/true
#   docker cp pandoc-bench-tmp:/output.cpio ./pandoc-bench-initrd.cpio
#   docker rm -f pandoc-bench-tmp

ARG PYTHON_BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/python-base:latest

# Stage 1: Assemble rootfs — just python-base + hl_pydriver.
# No external packages needed — workload uses only stdlib.
FROM ${PYTHON_BASE} AS rootfs
COPY hl_pydriver /bin/hl_pydriver

# Stage 2: Pack CPIO.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils ca-certificates
COPY --from=rootfs / /rootfs/

# DNS configuration for glibc's getaddrinfo
RUN mkdir -p /rootfs/etc && \
    echo "nameserver 8.8.8.8" > /rootfs/etc/resolv.conf && \
    echo "hosts: files dns" > /rootfs/etc/nsswitch.conf && \
    echo "127.0.0.1 localhost" > /rootfs/etc/hosts

# /tmp for output files
RUN mkdir -p /rootfs/tmp

# Busybox for shell support (static-PIE build)
COPY --from=ghcr.io/hyperlight-dev/hyperlight-unikraft/shell-base:latest \
     /bin/busybox /rootfs/bin/busybox
RUN for cmd in sh echo ls cat cp mv rm mkdir; do \
        ln -sf busybox /rootfs/bin/$cmd; \
    done

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
