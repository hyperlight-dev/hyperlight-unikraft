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

# Minimal rootfs: only Python runtime + driver + glibc
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

# Python interpreter binary (keep at /usr/local/bin so RPATH ../lib resolves)
COPY --from=build /usr/local/bin/python3.12 /usr/local/bin/python3

# Python shared library + soname symlinks
COPY --from=build /usr/local/lib/libpython3.12.so.1.0 /usr/local/lib/libpython3.12.so.1.0
COPY --from=build /usr/local/lib/libpython3.12.so /usr/local/lib/libpython3.12.so
COPY --from=build /usr/local/lib/libpython3.so /usr/local/lib/libpython3.so

# Python standard library
COPY --from=build /usr/local/lib/python3.12 /usr/local/lib/python3.12

# ld.so config so the driver can find libpython in /usr/local/lib
COPY --from=build /etc/ld.so.conf /etc/ld.so.conf
COPY --from=build /etc/ld.so.conf.d /etc/ld.so.conf.d
COPY --from=build /etc/ld.so.cache /etc/ld.so.cache

# /tmp needed for guest filesystem writes (ramfs, in-memory)
COPY --from=build /tmp /tmp

# Fix sys.executable for subprocess support (see sitecustomize.py)
COPY drivers/python/sitecustomize.py /usr/local/lib/python3.12/sitecustomize.py

# Hyperlight driver
COPY --from=build /hl_pydriver /usr/local/bin/hl_pydriver
