# agent-framework-remote (driver/pyhl stack, minimal rootfs)
#
# Builds a SMALL driver rootfs: the shipped hl_pydriver interpreter shim (extracted
# from the published driver image, so it stays version-matched to the GHCR driver
# kernel) + agent-framework-core only — dropping the ~1 GB preloaded data-science
# stack the general-purpose pyhl image carries. Gives the warm-snapshot speed of
# the driver stack with a ~60 MB initrd.

ARG BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/python-base:latest

# Stage 1: agent-framework-core, trimmed.
FROM python:3.12-slim AS deps
RUN pip install --target=/deps --no-cache-dir --prefer-binary agent-framework-core
RUN set -eux; \
    find /deps -maxdepth 1 -type d -name '*.dist-info' -exec rm -rf {} + 2>/dev/null || true; \
    find /deps \( -type d -name tests -o -type d -name test \) -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type f -name '*.pyc' -delete 2>/dev/null || true; \
    find /deps -name '*.so*' -exec strip --strip-unneeded {} + 2>/dev/null || true

# Stage 2: extract just hl_pydriver (+ pydoc stub) from the shipped driver rootfs.
FROM ghcr.io/hyperlight-dev/hyperlight-unikraft/python-agent-driver-initrd:latest AS shipped
FROM alpine:3.20 AS extract
RUN apk add --no-cache cpio
COPY --from=shipped /initrd.cpio /i.cpio
RUN mkdir -p /x && cd /x && cpio -idm 'bin/hl_pydriver' 'usr/local/lib/python3.12/pydoc.py' < /i.cpio 2>/dev/null && ls -l /x/bin/hl_pydriver

# Stage 3: minimal rootfs = python-base + hl_pydriver + agent-framework-core.
FROM ${BASE} AS rootfs
COPY --from=deps    /deps                                   /usr/local/lib/python3.12/site-packages
COPY --from=extract /x/bin/hl_pydriver                      /bin/hl_pydriver
COPY --from=extract /x/usr/local/lib/python3.12/pydoc.py    /usr/local/lib/python3.12/pydoc.py

# Stage 4: pack CPIO + DNS + CA certs for outbound TLS.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils ca-certificates
COPY --from=rootfs / /rootfs/
RUN mkdir -p /rootfs/etc && \
    echo "nameserver 8.8.8.8" > /rootfs/etc/resolv.conf && \
    echo "nameserver 8.8.4.4" >> /rootfs/etc/resolv.conf && \
    echo "hosts: files dns" > /rootfs/etc/nsswitch.conf && \
    echo "127.0.0.1 localhost" > /rootfs/etc/hosts
RUN mkdir -p /rootfs/etc/ssl/certs /rootfs/usr/lib/ssl && \
    cp /etc/ssl/certs/ca-certificates.crt /rootfs/etc/ssl/certs/ && \
    ln -sf /etc/ssl/certs/ca-certificates.crt /rootfs/usr/lib/ssl/cert.pem
RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
