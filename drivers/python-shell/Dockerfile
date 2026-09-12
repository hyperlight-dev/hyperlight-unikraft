# python-shell rootfs: Python 3.12 + BusyBox (hush) + SSL for pip install.
#
# Lightweight rootfs (~50MB CPIO) with no pre-installed data science
# packages — the general Python + shell base other images build on.  SSL
# support (libssl + CA certs) enables runtime package installation via
# `pip install` when the guest runs with --net.
#
# Pin to bookworm (glibc 2.36) — newer Debian ships glibc 2.41 which
# uses syscalls Unikraft doesn't support yet.
#
# Build context: repository root (not drivers/).
#
# The BusyBox userland comes from the shared hluk-busybox base image
# (drivers/busybox.Dockerfile), pulled in with COPY --from=hluk-busybox below.

# ── Build: Python driver ────────────────────────────────────────

FROM python:3.12-slim-bookworm AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev python3-dev \
        libssl-dev libsqlite3-dev libffi-dev libbz2-dev liblzma-dev \
        libstdc++6 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Headers live alongside the driver in /src/drivers/ so #include "hl_fc.h" resolves
COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_py.h /src/drivers/hl_py.h
COPY drivers/hl_pywarmdriver.c /src/drivers/hl_pywarmdriver.c

RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        $(python3-config --cflags --embed) \
        -o /hl_pywarmdriver /src/drivers/hl_pywarmdriver.c \
        $(python3-config --ldflags --embed)

# No-op lsb_release stub — pip's distro detection execs this; without
# a valid ELF binary Unikraft's execve crashes (no graceful failure).
COPY drivers/lsb_release_stub.c /src/drivers/lsb_release_stub.c
RUN gcc -O2 -pie -fPIE -o /usr/bin/lsb_release /src/drivers/lsb_release_stub.c

# ── Stage 3: Minimal rootfs ─────────────────────────────────────

FROM scratch

# glibc + dynamic linker
COPY --from=build /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=build /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=build /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=build /usr/lib/x86_64-linux-gnu/libz.so.1 /usr/lib/x86_64-linux-gnu/libz.so.1

# NSS + resolver — needed for DNS resolution (glibc loads these dynamically)
COPY --from=build /usr/lib/x86_64-linux-gnu/libnss_dns.so.2  /usr/lib/x86_64-linux-gnu/libnss_dns.so.2
COPY --from=build /usr/lib/x86_64-linux-gnu/libnss_files.so.2 /usr/lib/x86_64-linux-gnu/libnss_files.so.2
COPY --from=build /usr/lib/x86_64-linux-gnu/libresolv.so.2   /usr/lib/x86_64-linux-gnu/libresolv.so.2

# SSL libraries — enables pip install over --net
COPY --from=build /lib/x86_64-linux-gnu/libssl.so.3 /lib/x86_64-linux-gnu/libssl.so.3
COPY --from=build /lib/x86_64-linux-gnu/libcrypto.so.3 /lib/x86_64-linux-gnu/libcrypto.so.3

# CA certificates — needed for HTTPS (pip, requests, etc.)
COPY --from=build /etc/ssl/certs /etc/ssl/certs
COPY --from=build /etc/ssl/openssl.cnf /etc/ssl/openssl.cnf

# C++ runtime + glibc compat stubs — needed by pip-installed packages
# with C extensions (numpy, scipy, etc.).  Modern glibc merges pthread/dl/rt
# into libc but many .so files still reference them by name.
COPY --from=build /usr/lib/x86_64-linux-gnu/libstdc++.so.6  /usr/lib/x86_64-linux-gnu/libstdc++.so.6
COPY --from=build /usr/lib/x86_64-linux-gnu/libgcc_s.so.1   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=build /lib/x86_64-linux-gnu/libpthread.so.0     /lib/x86_64-linux-gnu/libpthread.so.0
COPY --from=build /lib/x86_64-linux-gnu/libdl.so.2          /lib/x86_64-linux-gnu/libdl.so.2
COPY --from=build /lib/x86_64-linux-gnu/librt.so.1          /lib/x86_64-linux-gnu/librt.so.1

# Shared libs for Python stdlib modules
COPY --from=build /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 /usr/lib/x86_64-linux-gnu/libsqlite3.so.0
COPY --from=build /usr/lib/x86_64-linux-gnu/libffi.so.8 /usr/lib/x86_64-linux-gnu/libffi.so.8
COPY --from=build /lib/x86_64-linux-gnu/libbz2.so.1.0 /lib/x86_64-linux-gnu/libbz2.so.1.0
COPY --from=build /lib/x86_64-linux-gnu/liblzma.so.5 /lib/x86_64-linux-gnu/liblzma.so.5

# Python interpreter binary (keep at /usr/local/bin so RPATH ../lib resolves)
COPY --from=build /usr/local/bin/python3.12 /usr/local/bin/python3

# Python shared library + soname symlinks
COPY --from=build /usr/local/lib/libpython3.12.so.1.0 /usr/local/lib/libpython3.12.so.1.0
COPY --from=build /usr/local/lib/libpython3.12.so /usr/local/lib/libpython3.12.so
COPY --from=build /usr/local/lib/libpython3.so /usr/local/lib/libpython3.so

# Python standard library (includes pip)
COPY --from=build /usr/local/lib/python3.12 /usr/local/lib/python3.12

# ld.so config so the driver can find libpython in /usr/local/lib
COPY --from=build /etc/ld.so.conf /etc/ld.so.conf
COPY --from=build /etc/ld.so.conf.d /etc/ld.so.conf.d
COPY --from=build /etc/ld.so.cache /etc/ld.so.cache

# /tmp needed for guest filesystem writes (ramfs, in-memory)
COPY --from=build /tmp /tmp

# Fix sys.executable for subprocess support
COPY drivers/python/sitecustomize.py /usr/local/lib/python3.12/sitecustomize.py

# BusyBox with all applet symlinks + musl dynamic linker
COPY --from=hluk-busybox /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
COPY --from=hluk-busybox /busybox-root/ /

# Pre-warm driver + lsb_release stub (must be in /bin — guest PATH is /bin)
COPY --from=build /hl_pywarmdriver /usr/local/bin/hl_pywarmdriver
COPY --from=build /usr/bin/lsb_release /bin/lsb_release
