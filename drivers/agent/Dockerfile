# Agent rootfs for Hyperlight/Unikraft.
#
# Full-featured agent environment: Python 3.12 + BusyBox (hush) +
# data-science packages + SSL/TLS + pip (runtime install via --net).
#
# Build stages:
#   build    — Python driver + pip packages + shared libs (Debian bookworm)
#   scratch  — minimal rootfs assembly
# The BusyBox userland comes from the shared hluk-busybox base image
# (drivers/busybox.Dockerfile), pulled in with COPY --from=hluk-busybox below.
#
# Pin to bookworm (glibc 2.36) — newer Debian ships glibc 2.41 which
# uses syscalls Unikraft doesn't support yet.

# ── Build: Python driver + pip packages ─────────────────────────

FROM python:3.12-slim-bookworm AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev python3-dev \
        libssl-dev libsqlite3-dev libffi-dev libbz2-dev liblzma-dev \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# ── Install pip packages ────────────────────────────────────────
RUN pip install --no-cache-dir \
        numpy pandas scipy scikit-learn \
        matplotlib seaborn sympy \
        pillow openpyxl python-docx python-pptx \
        pypdf pdfplumber reportlab \
        requests httpx beautifulsoup4 \
        python-dateutil pytz tqdm jinja2 \
        lxml pydantic tabulate chardet pyyaml && \
    # Strip test suites and unnecessary files to reduce rootfs size
    find /usr/local/lib/python3.12 -type d -name tests -exec rm -rf {} + 2>/dev/null; \
    find /usr/local/lib/python3.12 -type d -name test -exec rm -rf {} + 2>/dev/null; \
    find /usr/local/lib/python3.12 -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null; \
    find /usr/local/lib/python3.12 -name '*.pyc' -delete 2>/dev/null; \
    rm -rf /usr/local/lib/python3.12/test 2>/dev/null; \
    rm -rf /usr/local/lib/python3.12/idlelib 2>/dev/null; \
    rm -rf /usr/local/lib/python3.12/tkinter 2>/dev/null; \
    rm -rf /usr/local/lib/python3.12/turtle*.py 2>/dev/null; \
    rm -rf /usr/local/lib/python3.12/ensurepip/_bundled 2>/dev/null; \
    true

# ── Compile agent driver ────────────────────────────────────────
# Headers live alongside the driver in /src/drivers/ so #include "hl_fc.h" resolves
COPY drivers/hl_fc.h           /src/drivers/hl_fc.h
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
COPY --from=build /lib/x86_64-linux-gnu/libc.so.6     /lib/x86_64-linux-gnu/libc.so.6
COPY --from=build /lib/x86_64-linux-gnu/libm.so.6     /lib/x86_64-linux-gnu/libm.so.6
COPY --from=build /lib64/ld-linux-x86-64.so.2          /lib64/ld-linux-x86-64.so.2
COPY --from=build /usr/lib/x86_64-linux-gnu/libz.so.1  /usr/lib/x86_64-linux-gnu/libz.so.1

# NSS + resolver — needed for DNS resolution (glibc loads these dynamically)
COPY --from=build /usr/lib/x86_64-linux-gnu/libnss_dns.so.2  /usr/lib/x86_64-linux-gnu/libnss_dns.so.2
COPY --from=build /usr/lib/x86_64-linux-gnu/libnss_files.so.2 /usr/lib/x86_64-linux-gnu/libnss_files.so.2
COPY --from=build /usr/lib/x86_64-linux-gnu/libresolv.so.2   /usr/lib/x86_64-linux-gnu/libresolv.so.2

# SSL/TLS — unlocks pip install over --net and https from Python
COPY --from=build /lib/x86_64-linux-gnu/libssl.so.3    /lib/x86_64-linux-gnu/libssl.so.3
COPY --from=build /lib/x86_64-linux-gnu/libcrypto.so.3 /lib/x86_64-linux-gnu/libcrypto.so.3
COPY --from=build /etc/ssl/certs                        /etc/ssl/certs
COPY --from=build /etc/ssl/openssl.cnf                   /etc/ssl/openssl.cnf

# C++ runtime (needed by numpy, scipy, sklearn C extensions)
COPY --from=build /usr/lib/x86_64-linux-gnu/libstdc++.so.6  /usr/lib/x86_64-linux-gnu/libstdc++.so.6
COPY --from=build /usr/lib/x86_64-linux-gnu/libgcc_s.so.1   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1

# glibc compat stubs (modern glibc merges these into libc, but
# numpy/scipy .so files still reference them by name)
COPY --from=build /lib/x86_64-linux-gnu/libpthread.so.0     /lib/x86_64-linux-gnu/libpthread.so.0
COPY --from=build /lib/x86_64-linux-gnu/libdl.so.2          /lib/x86_64-linux-gnu/libdl.so.2
COPY --from=build /lib/x86_64-linux-gnu/librt.so.1          /lib/x86_64-linux-gnu/librt.so.1

# Additional shared libs for stdlib modules (_sqlite3, _ctypes, _bz2, _lzma)
COPY --from=build /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 /usr/lib/x86_64-linux-gnu/libsqlite3.so.0
COPY --from=build /usr/lib/x86_64-linux-gnu/libffi.so.8     /usr/lib/x86_64-linux-gnu/libffi.so.8
COPY --from=build /lib/x86_64-linux-gnu/libbz2.so.1.0       /lib/x86_64-linux-gnu/libbz2.so.1.0
COPY --from=build /usr/lib/x86_64-linux-gnu/liblzma.so.5    /usr/lib/x86_64-linux-gnu/liblzma.so.5

# Python interpreter binary (keep at /usr/local/bin so RPATH ../lib resolves)
COPY --from=build /usr/local/bin/python3.12 /usr/local/bin/python3

# Python shared library + soname symlinks
COPY --from=build /usr/local/lib/libpython3.12.so.1.0 /usr/local/lib/libpython3.12.so.1.0
COPY --from=build /usr/local/lib/libpython3.12.so     /usr/local/lib/libpython3.12.so
COPY --from=build /usr/local/lib/libpython3.so        /usr/local/lib/libpython3.so

# Python standard library (includes pip)
COPY --from=build /usr/local/lib/python3.12 /usr/local/lib/python3.12

# ld.so config so the driver can find libpython in /usr/local/lib
COPY --from=build /etc/ld.so.conf   /etc/ld.so.conf
COPY --from=build /etc/ld.so.conf.d /etc/ld.so.conf.d
COPY --from=build /etc/ld.so.cache  /etc/ld.so.cache

# /tmp needed for guest filesystem writes (ramfs, in-memory)
COPY --from=build /tmp /tmp

# Fix sys.executable for subprocess support
COPY drivers/python/sitecustomize.py /usr/local/lib/python3.12/sitecustomize.py

# BusyBox with all applet symlinks + musl linker (BusyBox is musl-linked)
COPY --from=hluk-busybox /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
COPY --from=hluk-busybox /busybox-root/ /

# Pre-warm driver + lsb_release stub (must be in /bin — guest PATH is /bin)
COPY --from=build /hl_pywarmdriver /usr/local/bin/hl_pywarmdriver
COPY --from=build /usr/bin/lsb_release /bin/lsb_release
