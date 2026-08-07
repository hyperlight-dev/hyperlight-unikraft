# pandoc-native — Real pandoc binary (Arch PIE build) for Hyperlight benchmarks.
#
# Uses Arch Linux's pandoc-cli package which is compiled as PIE (ET_DYN),
# making it compatible with Unikraft's ELF loader. The binary is dynamically
# linked against ~260 Haskell shared libraries (~291 MB total).
#
# Build:
#   docker build --platform linux/amd64 --target cpio -t pandoc-native-cpio .
#   docker create --name pandoc-native-tmp pandoc-native-cpio /bin/true
#   docker cp pandoc-native-tmp:/output.cpio ./pandoc-native-initrd.cpio
#   docker rm -f pandoc-native-tmp

# Stage 1: Install pandoc on Arch (the only distro that builds it as PIE)
FROM archlinux:latest AS pandoc-source
RUN pacman -Sy --noconfirm pandoc-cli

# Collect pandoc + all shared library dependencies into /staging
RUN mkdir -p /staging/usr/bin /staging/lib64 /staging/usr/share && \
    cp /usr/bin/pandoc /staging/usr/bin/ && \
    cp /lib64/ld-linux-x86-64.so.2 /staging/lib64/ && \
    ldd /usr/bin/pandoc | awk '{print $3}' | grep -v '^$' | while read lib; do \
        dir=$(dirname "$lib"); \
        mkdir -p "/staging$dir"; \
        cp "$lib" "/staging$dir/"; \
    done; \
    cp -r /usr/share/haskell-pandoc /staging/usr/share/haskell-pandoc

# Stage 2: Assemble rootfs and pack CPIO
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils

# Copy pandoc + all its shared libraries
COPY --from=pandoc-source /staging/ /rootfs/

# Busybox for shell support (static-PIE build, no glibc dependency)
COPY --from=ghcr.io/hyperlight-dev/hyperlight-unikraft/shell-base:latest \
     /bin/busybox /rootfs/bin/busybox
RUN for cmd in sh echo ls cat; do \
        ln -sf busybox /rootfs/bin/$cmd; \
    done

# Input markdown for the benchmark
COPY input.md /rootfs/data/input.md

# /tmp for output files
RUN mkdir -p /rootfs/tmp

# Wrapper script: run pandoc and emit success marker
RUN printf '#!/bin/sh\n/usr/bin/pandoc /data/input.md -o /tmp/output.docx\necho "PANDOC_DOCX_OK"\n' \
    > /rootfs/bin/run-pandoc.sh && chmod +x /rootfs/bin/run-pandoc.sh

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
