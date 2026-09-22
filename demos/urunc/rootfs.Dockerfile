# Rootfs for the urunc demo: the stock python rootfs with the workload baked in
# at /app/hello.py. The image's CMD names it, and urunc passes that command to
# hluk, which runs the file inside the initrd.
#
# Built by `just stage` into build-elfloader/urunc-hello-rootfs.cpio.
FROM hluk-python-rootfs:latest
COPY demos/urunc/hello.py /app/hello.py
