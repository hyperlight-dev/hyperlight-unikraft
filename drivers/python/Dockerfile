# Pin to bookworm (glibc 2.36) — newer Debian ships glibc 2.41 which
# uses syscalls Unikraft doesn't support yet.
FROM python:3.12-slim-bookworm AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
        gcc libc6-dev python3-dev && \
    rm -rf /var/lib/apt/lists/*

# Preserve directory layout so #include "../hl_fc.h" resolves
COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_py.h /src/drivers/hl_py.h
COPY drivers/python/hl_pydriver.c /src/drivers/python/hl_pydriver.c

RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        $(python3-config --cflags --embed) \
        -o /hl_pydriver /src/drivers/python/hl_pydriver.c \
        $(python3-config --ldflags --embed)

# Minimal rootfs: only Python runtime + driver + glibc.  Staged under
# /rootfs here, where the shell can ask for the architecture's library
# directory (x86_64-linux-gnu, aarch64-linux-gnu) and dynamic linker; the
# final image is that tree.  Links are followed, as COPY would.
RUN set -eu; \
    lib=/lib/$(gcc -dumpmachine); \
    usrlib=/usr$lib; \
    ldso=$(readelf -l /usr/local/bin/python3.12 | sed -n 's/.*interpreter: \(.*\)]/\1/p'); \
    stage() { mkdir -p "/rootfs$(dirname "$2")"; cp -rL "$1" "/rootfs$2"; }; \
    # glibc + dynamic linker
    stage $lib/libc.so.6 $lib/libc.so.6; \
    stage $lib/libm.so.6 $lib/libm.so.6; \
    stage $ldso $ldso; \
    stage $usrlib/libz.so.1 $usrlib/libz.so.1; \
    # NSS + resolver — needed for DNS resolution (glibc loads these dynamically)
    stage $usrlib/libnss_dns.so.2 $usrlib/libnss_dns.so.2; \
    stage $usrlib/libnss_files.so.2 $usrlib/libnss_files.so.2; \
    stage $usrlib/libresolv.so.2 $usrlib/libresolv.so.2; \
    # Python interpreter binary (keep at /usr/local/bin so RPATH ../lib resolves)
    stage /usr/local/bin/python3.12 /usr/local/bin/python3; \
    # Python shared library + soname symlinks
    stage /usr/local/lib/libpython3.12.so.1.0 /usr/local/lib/libpython3.12.so.1.0; \
    stage /usr/local/lib/libpython3.12.so /usr/local/lib/libpython3.12.so; \
    stage /usr/local/lib/libpython3.so /usr/local/lib/libpython3.so; \
    # Python standard library
    stage /usr/local/lib/python3.12 /usr/local/lib/python3.12; \
    # ld.so config so the driver can find libpython in /usr/local/lib
    stage /etc/ld.so.conf /etc/ld.so.conf; \
    stage /etc/ld.so.conf.d /etc/ld.so.conf.d; \
    stage /etc/ld.so.cache /etc/ld.so.cache; \
    # /tmp needed for guest filesystem writes (ramfs, in-memory)
    mkdir -p /rootfs/tmp; \
    # Hyperlight driver
    stage /hl_pydriver /usr/local/bin/hl_pydriver

FROM scratch
COPY --from=build /rootfs/ /

# Fix sys.executable for subprocess support (see sitecustomize.py)
COPY drivers/python/sitecustomize.py /usr/local/lib/python3.12/sitecustomize.py
