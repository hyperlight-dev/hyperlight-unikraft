# agent-framework-remote rootfs — extends python-shell with agent-framework-core.
#
# Runs a Microsoft Agent Framework agent that calls the GitHub Models API over
# the network. The API token is passed at run time, never baked in.
#
# Build:  just build-rootfs agent-fw-remote examples/agent-framework/remote.Dockerfile
# Run:    hluk run --initrd build-elfloader/agent-fw-remote-rootfs.cpio \
#             --scratch-mb 512 --net --env GITHUB_TOKEN="$GITHUB_TOKEN" \
#             examples/agent-framework/remote.py
FROM hluk-python-shell-rootfs:latest AS base

FROM python:3.12-slim-bookworm AS installer
COPY --from=base / /rootfs/
RUN pip install --no-cache-dir --target /rootfs/usr/local/lib/python3.12/site-packages \
        agent-framework-core \
    && find /rootfs/usr/local/lib/python3.12/site-packages \
        \( -type d -name tests -o -type d -name __pycache__ \) -prune -exec rm -rf {} + 2>/dev/null || true

FROM scratch
COPY --from=installer /rootfs/ /
