# pyhl container image — runs Python scripts in Hyperlight micro-VMs.
#
# Bundles the pyhl CLI, the python-agent-driver kernel, and its initrd
# into a single runnable container.  Designed for the
# hyperlight-on-kubernetes device plugin: the pod gets /dev/kvm via
# `hyperlight.dev/hypervisor: "1"` and pyhl handles the rest.
#
# Build (from repo root):
#   docker build -f host/pyhl.Dockerfile -t pyhl .
#
# Run:
#   docker run --rm --device /dev/kvm pyhl /path/to/script.py
#   echo 'print("hi")' | docker run --rm -i --device /dev/kvm pyhl -

FROM rust:1.89-bookworm AS builder
COPY rust-toolchain.toml /src/
COPY host/ /src/host/
WORKDIR /src/host
RUN cargo build --release --bin pyhl

FROM ghcr.io/hyperlight-dev/hyperlight-unikraft/python-agent-driver-kernel:latest AS kernel
FROM ghcr.io/hyperlight-dev/hyperlight-unikraft/python-agent-driver-initrd:latest AS initrd

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /src/host/target/release/pyhl /usr/local/bin/
COPY --from=kernel /kernel /opt/pyhl/kernel
COPY --from=initrd /initrd.cpio /opt/pyhl/initrd.cpio
COPY host/pyhl-entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh && chown -R 65534:65534 /opt/pyhl
ENV PYHL_HOME=/opt/pyhl
USER 65534
ENTRYPOINT ["/entrypoint.sh"]
