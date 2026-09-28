# Autonomous appliance rootfs — the stock python rootfs with the workload baked
# in at /entrypoint.py. The guest runs it on launch; the host supplies no app
# code, only the fixed launcher (see README / launch.py).
#
# Build:  just build-rootfs autonomous examples/autonomous/Dockerfile
# -arm64 when `just build-rootfs` builds for another platform
# (HLUK_ROOTFS_PLATFORM), so the base is that platform's image too.
ARG ROOTFS_SUFFIX=
FROM hluk-python-rootfs${ROOTFS_SUFFIX}:latest
COPY examples/autonomous/entrypoint.py /entrypoint.py
