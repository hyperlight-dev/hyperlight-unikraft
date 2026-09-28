# PowerShell runtime for Hyperlight/Unikraft.
#
# Installs PowerShell 7 (which ships .NET CoreCLR) and bundles it
# with a dedicated driver that writes PS1 code to a temp file and
# runs it via vfork+execve of pwsh.
#
# The rootfs is large (~180 MB) due to the full .NET runtime.
#
# Microsoft publishes a musl (Alpine) PowerShell for x86_64 only, so each
# architecture has its own stage: amd64 takes the Alpine image, arm64 the
# glibc build (linux-arm64) of the same release on Debian bookworm (glibc
# 2.36, as the python image).  Each stages its rootfs under /rootfs.

ARG TARGETARCH

# Pinned to the 7.4.6 image (no per-patch Alpine tag exists); bump it together
# with PWSH_VERSION below so both architectures ship the same release.
FROM mcr.microsoft.com/powershell:7.4-alpine-3.20@sha256:c2bee73acbaa53e9209daab8075b5d234f0f04e6824e7b6f7bf7dde1ba5b772b AS pwsh-amd64

# Links named here are followed, links inside a copied directory kept, as
# COPY does.
RUN set -eu; \
    stage() { mkdir -p "/rootfs$(dirname "$2")"; cp -rpH "$1" "/rootfs$2"; }; \
    # musl dynamic linker (pwsh's native libs need it)
    stage /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1; \
    # Core system libraries pwsh depends on
    for f in /usr/lib/libstdc++.so* /usr/lib/libgcc_s.so* /lib/libz.so*; do stage "$f" "$f"; done; \
    # PowerShell + .NET runtime
    stage /opt/microsoft/powershell/7 /opt/microsoft/powershell/7; \
    # Minimal /etc
    stage /etc/passwd /etc/passwd; \
    stage /etc/group /etc/group

FROM debian:bookworm-slim AS pwsh-arm64

ARG PWSH_VERSION=7.4.6
# The release's own hashes.sha256 lists this checksum.
ADD --checksum=sha256:c0159b03e85f44ae1e7697818a011558da6c813d0aae848bf5ac13bf435d8624 \
    https://github.com/PowerShell/PowerShell/releases/download/v${PWSH_VERSION}/powershell-${PWSH_VERSION}-linux-arm64.tar.gz /pwsh.tar.gz

# PowerShell + .NET runtime, and the glibc libraries pwsh and its native
# libraries link (ldd over all of them).
RUN set -eu; \
    mkdir -p /rootfs/opt/microsoft/powershell/7; \
    tar -xzf /pwsh.tar.gz -C /rootfs/opt/microsoft/powershell/7; \
    chmod +x /rootfs/opt/microsoft/powershell/7/pwsh; \
    # A library missing here would only fail when pwsh starts in the guest.
    # LTTng is the exception: CoreCLR's tracing provider loads it if present.
    missing=$(for bin in /rootfs/opt/microsoft/powershell/7/pwsh /rootfs/opt/microsoft/powershell/7/*.so; do \
        ldd "$bin" 2>/dev/null | grep 'not found' | grep -v liblttng-ust || true; done); \
    [ -z "$missing" ] || { echo "missing libraries: $missing" >&2; exit 1; }; \
    for bin in /rootfs/opt/microsoft/powershell/7/pwsh /rootfs/opt/microsoft/powershell/7/*.so; do \
        ldd "$bin" 2>/dev/null | grep -o '/[^ ]*' || true; \
    done | sort -u | while read -r lib; do \
        case $lib in /rootfs/*) continue ;; esac; \
        [ -e "$lib" ] || continue; \
        mkdir -p "/rootfs$(dirname "$lib")"; cp -L "$lib" "/rootfs$lib"; \
    done; \
    # Minimal /etc
    mkdir -p /rootfs/etc; cp /etc/passwd /etc/group /rootfs/etc/; \
    # glibc finds the main thread's stack in /proc/self/maps, which the
    # kernel serves as /dev/maps; CoreCLR fails to start without it.
    mkdir -p /rootfs/proc/self; ln -s /dev/maps /rootfs/proc/self/maps

FROM pwsh-${TARGETARCH} AS pwsh-base

# Create dirs that FROM scratch doesn't have
RUN mkdir -p /empty-tmp /stub-proc/self && \
    printf 'pwsh\0-NoProfile\0-File\0/tmp/hl_dispatch.ps1\0' > /stub-proc/self/cmdline

# ── Compile the driver ──────────────────────────────────────────
FROM alpine:3.20 AS build-driver

RUN apk add --no-cache gcc musl-dev

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/powershell/hl_pwshdriver.c /src/drivers/powershell/hl_pwshdriver.c
RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_pwshdriver /src/drivers/powershell/hl_pwshdriver.c

# ── Rootfs ──────────────────────────────────────────────────────
FROM scratch

COPY --from=pwsh-base /rootfs/ /

# /tmp + a /proc/self/cmdline for pwsh to read
COPY --from=pwsh-base /empty-tmp/ /tmp/
COPY --from=pwsh-base /stub-proc/ /proc/

# Driver
COPY --from=build-driver /hl_pwshdriver /usr/local/bin/hl_pwshdriver
