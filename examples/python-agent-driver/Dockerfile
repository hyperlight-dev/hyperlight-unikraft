# python-agent-driver — Python runtime driver for Hyperlight.
#
# Same deps as python-agent (numpy + pandas + the usual stack) PLUS
# a small C program `hl_pydriver` that embeds libpython3.12, does
# Py_Initialize + warm-up imports on its first invocation, and then
# installs an FC-aware dispatch callback that handles every
# subsequent host->guest call without re-running main().

ARG BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/python-base:latest

# Stage 1: Python deps (same as python-agent).
FROM python:3.12-slim AS deps
RUN pip install --target=/deps --no-cache-dir \
    tqdm pyyaml jinja2 beautifulsoup4 tabulate click tenacity \
    python-dotenv pypdf openpyxl markdown-it-py pydantic pillow \
    lxml cryptography python-dateutil numpy pandas
RUN set -eux; \
    find /deps -maxdepth 1 -type d -name '*.dist-info' -exec rm -rf {} + 2>/dev/null || true; \
    find /deps \( -type d -name tests -o -type d -name test \) -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type f -name '*.pyc' -delete 2>/dev/null || true; \
    find /deps -name '*.so*' -exec strip --strip-unneeded {} + 2>/dev/null || true

# Stage 2: build hl_pydriver against the SAME Python 3.12-from-source
# that produces local-python-base, so the glibc/libpython3.12.so ABI
# matches the runtime.
FROM local-python-base-dev AS driver-build
COPY hl_pydriver.c /src/hl_pydriver.c
RUN PY_INC=$(python3.12 -c 'import sysconfig; print(sysconfig.get_path("include"))') && \
    PY_LIBDIR=$(python3.12 -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))') && \
    gcc -O2 -fPIE -pie -Wl,-E \
        -fno-stack-protector \
        -I$PY_INC \
        -o /src/hl_pydriver /src/hl_pydriver.c \
        -L$PY_LIBDIR \
        -Wl,-rpath,/usr/local/lib \
        -Wl,-rpath,/lib/x86_64-linux-gnu \
        -lpython3.12 -ldl -lpthread -lm -lutil

# Stage 3: assemble rootfs.
FROM ${BASE} AS rootfs
COPY --from=deps         /deps            /usr/local/lib/python3.12/site-packages
COPY --from=driver-build /src/hl_pydriver /bin/hl_pydriver

# Stage 4: pack CPIO.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils ca-certificates
COPY --from=rootfs / /rootfs/

# DNS configuration for glibc's getaddrinfo
RUN mkdir -p /rootfs/etc && \
    echo "nameserver 8.8.8.8" > /rootfs/etc/resolv.conf && \
    echo "nameserver 8.8.4.4" >> /rootfs/etc/resolv.conf && \
    echo "hosts: files dns" > /rootfs/etc/nsswitch.conf && \
    echo "127.0.0.1 localhost" > /rootfs/etc/hosts

# CA certificates for TLS
RUN mkdir -p /rootfs/etc/ssl/certs /rootfs/usr/lib/ssl && \
    cp /etc/ssl/certs/ca-certificates.crt /rootfs/etc/ssl/certs/ && \
    ln -sf /etc/ssl/certs/ca-certificates.crt /rootfs/usr/lib/ssl/cert.pem

RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
