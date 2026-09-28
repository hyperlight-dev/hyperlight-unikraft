# .NET Native AOT runtime for Hyperlight/Unikraft.
#
# Produces a minimal rootfs with the shared exec driver and the musl
# dynamic linker (AOT binaries on Alpine link against musl).  The user
# mounts a host directory containing the compiled binary into the
# guest and dispatches its guest path; the driver runs it via
# vfork+execve.
#
# See examples/dotnet-aot/README.md for publishing and running
# .NET AOT applications.

FROM alpine:3.20 AS build-driver

RUN apk add --no-cache gcc musl-dev

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_execdriver.c /src/drivers/hl_execdriver.c
RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_dotnetaotdriver /src/drivers/hl_execdriver.c

# Rootfs staged under /rootfs: the musl linker, also under the glibc
# library names (in /lib so musl finds them), all named for the
# architecture.  AOT binaries cross-compiled on glibc hosts reference
# ld-linux-<arch>.so and libc.so.6 even when targeting linux-musl-<arch>.
RUN set -eu; \
    musl=/lib/ld-musl-$(uname -m).so.1; \
    case $(uname -m) in \
        x86_64)  glibc_ldso=ld-linux-x86-64.so.2 ;; \
        aarch64) glibc_ldso=ld-linux-aarch64.so.1 ;; \
        *) echo "unsupported arch $(uname -m)" >&2; exit 1 ;; \
    esac; \
    mkdir -p /rootfs/lib /rootfs/usr/local/bin; \
    mkdir -p /rootfs/tmp; \
    cp $musl /rootfs/lib/; \
    cp $musl /rootfs/lib/$glibc_ldso; \
    cp $musl /rootfs/lib/libc.so.6; \
    cp /hl_dotnetaotdriver /rootfs/usr/local/bin/hl_dotnetaotdriver

FROM scratch
COPY --from=build-driver /rootfs/ /
