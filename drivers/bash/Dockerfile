# BusyBox hush runtime for Hyperlight/Unikraft.
#
# The userland is the shared NOMMU/PIE BusyBox base image (see
# drivers/busybox.Dockerfile — built as hluk-busybox, and pulled in below with
# COPY --from=hluk-busybox).  BusyBox runs in NOMMU mode so the shell uses
# vfork()+execve() instead of fork() (the Unikraft kernel has no fork); all
# applets (cat, grep, sed, awk, sort, ls, find, …) run as separate processes.
# (NOMMU disables ash; hush is the active shell.)
#
# Limitation: pipes (|) and output redirects from external commands
# (cmd > file) crash because Unikraft's vfork shares the parent's stack with
# the child — a kernel fix (separate child stack) is needed.  Use awk
# single-pass or shell builtins to work around.

# ── Compile the C driver ─────────────────────────────────────────
FROM alpine:3.20 AS build

RUN apk add --no-cache gcc musl-dev

WORKDIR /src

# Preserve directory layout so #include "../hl_fc.h" resolves
COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/bash/hl_bashdriver.c /src/drivers/bash/hl_bashdriver.c

RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_bashdriver /src/drivers/bash/hl_bashdriver.c

# ── Minimal rootfs ───────────────────────────────────────────────
FROM scratch

# BusyBox userland (dynamically linked → needs the musl loader) from the
# shared base image.
COPY --from=hluk-busybox /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
COPY --from=hluk-busybox /busybox-root/ /

# Hyperlight driver
COPY --from=build /hl_bashdriver /usr/local/bin/hl_bashdriver
