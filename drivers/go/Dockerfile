# Go exec runtime for Hyperlight/Unikraft.
#
# Produces a minimal rootfs with the shared exec driver and the musl
# dynamic linker.  The user mounts a host directory containing the
# compiled binary into the guest and dispatches its guest path; the
# driver runs it via vfork+execve.
#
# Go PIE binaries built on glibc hosts reference glibc's loader
# (/lib64/ld-linux-x86-64.so.2, /lib/ld-linux-aarch64.so.1) as the program
# interpreter.  A compatibility symlink maps that path to musl's linker so
# the same binary works in this Alpine-based rootfs.
#
# See examples/go/README.md for compiling and running Go programs.

FROM alpine:3.20 AS build-driver

RUN apk add --no-cache gcc musl-dev

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_execdriver.c /src/drivers/hl_execdriver.c
RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_godriver /src/drivers/hl_execdriver.c

# Rootfs staged under /rootfs: the musl linker plus the glibc-INTERP compat
# symlink, both named for the architecture.
RUN set -eu; \
    musl=/lib/ld-musl-$(uname -m).so.1; \
    mkdir -p /rootfs/lib /rootfs/usr/local/bin; \
    mkdir -p /rootfs/tmp; \
    cp $musl /rootfs/lib/; \
    case $(uname -m) in \
        x86_64)  mkdir -p /rootfs/lib64; \
                 ln -s $musl /rootfs/lib64/ld-linux-x86-64.so.2 ;; \
        aarch64) ln -s $musl /rootfs/lib/ld-linux-aarch64.so.1 ;; \
        *) echo "unsupported arch $(uname -m)" >&2; exit 1 ;; \
    esac; \
    cp /hl_godriver /rootfs/usr/local/bin/hl_godriver

FROM scratch
COPY --from=build-driver /rootfs/ /
