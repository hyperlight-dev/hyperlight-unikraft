# Agent rootfs for Hyperlight/Unikraft.
#
# Full-featured agent environment: Python 3.12 + BusyBox (hush) +
# data-science packages + SSL/TLS + pip (runtime install via --net).
#
# Build stages:
#   build    — Python driver + pip packages + shared libs (Debian bookworm)
#   scratch  — minimal rootfs assembly
# The BusyBox userland comes from the shared hluk-busybox base image
# (drivers/busybox.Dockerfile), named by BUSYBOX (hluk-busybox-arm64 for
# arm64) and pulled in through the busybox stage, as COPY --from cannot
# expand a build arg.
#
# Pin to bookworm (glibc 2.36) — newer Debian ships glibc 2.41 which
# uses syscalls Unikraft doesn't support yet.

ARG BUSYBOX=hluk-busybox
FROM ${BUSYBOX} AS busybox

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

# Minimal rootfs, staged under /rootfs here, where the shell can ask for
# the architecture's library directory (x86_64-linux-gnu, aarch64-linux-gnu)
# and dynamic linker; the final image is that tree.  As with COPY, a named
# link is followed but links inside a copied directory are kept.
RUN set -eu; \
    lib=/lib/$(gcc -dumpmachine); \
    usrlib=/usr$lib; \
    ldso=$(readelf -l /usr/local/bin/python3.12 | sed -n 's/.*interpreter: \(.*\)]/\1/p'); \
    stage() { mkdir -p "/rootfs$(dirname "$2")"; cp -rH "$1" "/rootfs$2"; }; \
    # glibc + dynamic linker
    stage $lib/libc.so.6 $lib/libc.so.6; \
    stage $lib/libm.so.6 $lib/libm.so.6; \
    stage $ldso $ldso; \
    stage $usrlib/libz.so.1 $usrlib/libz.so.1; \
    # NSS + resolver — needed for DNS resolution (glibc loads these dynamically)
    stage $usrlib/libnss_dns.so.2 $usrlib/libnss_dns.so.2; \
    stage $usrlib/libnss_files.so.2 $usrlib/libnss_files.so.2; \
    stage $usrlib/libresolv.so.2 $usrlib/libresolv.so.2; \
    # SSL/TLS — unlocks pip install over --net and https from Python
    stage $lib/libssl.so.3 $lib/libssl.so.3; \
    stage $lib/libcrypto.so.3 $lib/libcrypto.so.3; \
    stage /etc/ssl/certs /etc/ssl/certs; \
    stage /etc/ssl/openssl.cnf /etc/ssl/openssl.cnf; \
    # C++ runtime (needed by numpy, scipy, sklearn C extensions)
    stage $usrlib/libstdc++.so.6 $usrlib/libstdc++.so.6; \
    stage $usrlib/libgcc_s.so.1 $usrlib/libgcc_s.so.1; \
    # glibc compat stubs (modern glibc merges these into libc, but
    # numpy/scipy .so files still reference them by name)
    stage $lib/libpthread.so.0 $lib/libpthread.so.0; \
    stage $lib/libdl.so.2 $lib/libdl.so.2; \
    stage $lib/librt.so.1 $lib/librt.so.1; \
    # Additional shared libs for stdlib modules (_sqlite3, _ctypes, _bz2, _lzma)
    stage $usrlib/libsqlite3.so.0 $usrlib/libsqlite3.so.0; \
    stage $usrlib/libffi.so.8 $usrlib/libffi.so.8; \
    stage $lib/libbz2.so.1.0 $lib/libbz2.so.1.0; \
    stage $usrlib/liblzma.so.5 $usrlib/liblzma.so.5; \
    # Python interpreter binary (keep at /usr/local/bin so RPATH ../lib resolves)
    stage /usr/local/bin/python3.12 /usr/local/bin/python3; \
    # Python shared library + soname symlinks
    stage /usr/local/lib/libpython3.12.so.1.0 /usr/local/lib/libpython3.12.so.1.0; \
    stage /usr/local/lib/libpython3.12.so /usr/local/lib/libpython3.12.so; \
    stage /usr/local/lib/libpython3.so /usr/local/lib/libpython3.so; \
    # Python standard library (includes pip)
    stage /usr/local/lib/python3.12 /usr/local/lib/python3.12; \
    # ld.so config so the driver can find libpython in /usr/local/lib
    stage /etc/ld.so.conf /etc/ld.so.conf; \
    stage /etc/ld.so.conf.d /etc/ld.so.conf.d; \
    stage /etc/ld.so.cache /etc/ld.so.cache; \
    # /tmp needed for guest filesystem writes (ramfs, in-memory)
    mkdir -p /rootfs/tmp; \
    # Pre-warm driver + lsb_release stub (must be in /bin — guest PATH is /bin)
    stage /hl_pywarmdriver /usr/local/bin/hl_pywarmdriver; \
    stage /usr/bin/lsb_release /bin/lsb_release

# ── Stage 3: Minimal rootfs ─────────────────────────────────────

FROM scratch
COPY --from=build /rootfs/ /

# Fix sys.executable for subprocess support
COPY drivers/python/sitecustomize.py /usr/local/lib/python3.12/sitecustomize.py

# BusyBox with all applet symlinks + musl linker (BusyBox is musl-linked)
COPY --from=busybox /busybox-root/ /
