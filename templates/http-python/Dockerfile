# The guest rootfs for {{name}}: the published python-shell base plus the
# packages in requirements.txt. `hluk build` builds it and exports it as
# .hluk/rootfs.cpio; `hluk run` boots it.
FROM {{base}} AS base

# pip runs in a real Linux stage, then the packages are copied into the
# rootfs, which is FROM scratch and has no shell to run pip in.
FROM python:3.12-slim-bookworm AS installer
COPY --from=base / /rootfs/
COPY requirements.txt /tmp/requirements.txt
RUN pip install --no-cache-dir -r /tmp/requirements.txt \
        --target /rootfs/usr/local/lib/python3.12/site-packages

FROM scratch
COPY --from=installer /rootfs/ /
