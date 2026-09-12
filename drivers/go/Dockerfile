# Go exec runtime for Hyperlight/Unikraft.
#
# Produces a minimal rootfs with the shared exec driver and the musl
# dynamic linker.  The user mounts a host directory containing the
# compiled binary into the guest and dispatches its guest path; the
# driver runs it via vfork+execve.
#
# Go PIE binaries built on glibc hosts reference /lib64/ld-linux-x86-64.so.2
# as the program interpreter.  A compatibility symlink maps that path
# to musl's linker so the same binary works in this Alpine-based rootfs.
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

# Compat symlink: glibc INTERP → musl linker
RUN mkdir -p /lib64 && ln -s /lib/ld-musl-x86_64.so.1 /lib64/ld-linux-x86-64.so.2

RUN rm -rf /tmp/* /tmp/.[!.]*

FROM scratch
COPY --from=build-driver /tmp /tmp
# musl dynamic linker
COPY --from=build-driver /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
# glibc compat symlink (Go PIE on glibc hosts sets INTERP to this path)
COPY --from=build-driver /lib64 /lib64
COPY --from=build-driver /hl_godriver /usr/local/bin/hl_godriver
