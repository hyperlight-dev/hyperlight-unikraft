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

# Compat symlinks: glibc library names → musl (in /lib so musl finds them)
# AOT binaries cross-compiled on glibc hosts reference ld-linux-x86-64.so.2
# and libc.so.6 even when targeting linux-musl-x64.
RUN ln -s /lib/ld-musl-x86_64.so.1 /lib/ld-linux-x86-64.so.2 && \
    ln -s /lib/ld-musl-x86_64.so.1 /lib/libc.so.6

RUN rm -rf /tmp/* /tmp/.[!.]*

FROM scratch
COPY --from=build-driver /tmp /tmp
# musl dynamic linker
COPY --from=build-driver /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
# glibc compat symlinks (AOT cross-compilation on glibc hosts)
COPY --from=build-driver /lib/ld-linux-x86-64.so.2 /lib/ld-linux-x86-64.so.2
COPY --from=build-driver /lib/libc.so.6 /lib/libc.so.6
COPY --from=build-driver /hl_dotnetaotdriver /usr/local/bin/hl_dotnetaotdriver
