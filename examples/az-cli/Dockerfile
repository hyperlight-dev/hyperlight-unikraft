# az-cli rootfs: a BusyBox hush shell whose only tool beyond the BusyBox
# applets is the Azure CLI.  Built on the python-shell rootfs (Python 3.12 +
# hush + SSL), with the bash runtime's hush driver as the entry point, so
# the guest is a shell and `az` runs Python as a child process.
#
# Build:  just build-rootfs az-cli examples/az-cli/Dockerfile
ARG ROOTFS_SUFFIX=

# Same Python as python-shell, so the wheels match its interpreter.
FROM python:3.12-slim-bookworm@sha256:34386ef0cb081344d7ec1c103ba398e6e9f64e9ab3a1509accc92a4e24a07258 AS build
# Bytecode compiled here: without it every az call compiles the modules it
# imports, in a guest whose filesystem is gone when it exits.
# psutil goes: it reads /proc/<pid>, which the guest lacks, and az raises
# on its NoSuchProcess (when it colours output for a terminal) where it
# copes with psutil missing.
RUN pip install --no-cache-dir --no-compile --target /site azure-cli && \
    rm -rf /site/psutil /site/psutil-*.dist-info && \
    python3 -m compileall -q -j0 /site > /dev/null

# pip's az is a #! script, which the guest cannot exec: an ELF launcher.
FROM alpine:3.20@sha256:d9e853e87e55526f6b2917df91a2115c36dd7c696a35be12163d44e6e2a4b6bc AS launcher
RUN apk add --no-cache gcc musl-dev
COPY examples/az-cli/az.c /src/az.c
RUN gcc -O2 -Wall -Wextra -static-pie -fPIE -o /az /src/az.c

FROM hluk-bash-rootfs${ROOTFS_SUFFIX}:latest AS bash

FROM hluk-python-shell-rootfs${ROOTFS_SUFFIX}:latest
COPY --from=build /site/ /usr/local/lib/python3.12/site-packages/
COPY --from=bash /usr/local/bin/hl_bashdriver /usr/local/bin/hl_bashdriver
COPY --from=launcher /az /bin/az
