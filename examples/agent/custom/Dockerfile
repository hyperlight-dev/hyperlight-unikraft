# Custom agent rootfs — extends python-shell with additional packages.
#
# Build:
#   docker build -t hluk-agent-custom-rootfs -f examples/agent/custom/Dockerfile .
#
# Extract CPIO:
#   id=$(docker create hluk-agent-custom-rootfs)
#   docker export "$id" | cpio -o -H newc > build-elfloader/agent-custom-rootfs.cpio
#   docker rm "$id"
#
# Run:
#   hluk run --initrd build-elfloader/agent-custom-rootfs.cpio \
#       --scratch-mb 256 examples/agent/custom/custom_packages.py

FROM hluk-python-shell-rootfs:latest AS base

# Install packages into the rootfs.
# Use a multi-stage build so pip runs in a real Linux environment
# (not scratch), then copy the installed packages into the rootfs.
FROM python:3.12-slim-bookworm AS installer
COPY --from=base / /rootfs/
RUN pip install --no-cache-dir --target /rootfs/usr/local/lib/python3.12/site-packages \
        pydantic pyyaml

FROM scratch
COPY --from=installer /rootfs/ /
