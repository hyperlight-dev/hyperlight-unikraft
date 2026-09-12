# Autonomous appliance rootfs — the stock python rootfs with the workload baked
# in at /entrypoint.py. The guest runs it on launch; the host supplies no app
# code, only the fixed launcher (see README / launch.py).
#
# Build:  just build-rootfs autonomous examples/autonomous/Dockerfile
FROM hluk-python-rootfs:latest
COPY examples/autonomous/entrypoint.py /entrypoint.py
