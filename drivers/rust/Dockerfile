# Rust exec runtime for Hyperlight/Unikraft.
#
# Produces a minimal rootfs with the shared exec driver.  The user
# mounts a host directory containing the compiled binary into the
# guest and dispatches its guest path; the driver runs it via
# vfork+execve.  Rust static-PIE binaries are fully self-contained
# (no musl needed in the rootfs).
#
# See examples/rust/README.md for compiling and running Rust programs.

FROM alpine:3.20 AS build-driver

RUN apk add --no-cache gcc musl-dev

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_execdriver.c /src/drivers/hl_execdriver.c
RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_rustdriver /src/drivers/hl_execdriver.c

RUN rm -rf /tmp/* /tmp/.[!.]*

FROM scratch
COPY --from=build-driver /tmp /tmp
COPY --from=build-driver /hl_rustdriver /usr/local/bin/hl_rustdriver
