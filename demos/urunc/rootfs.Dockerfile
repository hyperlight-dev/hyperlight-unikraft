# Rootfs for the urunc demo: the stock python rootfs with the workload baked in
# at /entrypoint.py. urunc's monitor boots the guest and (with no cmdline) the
# python driver runs /entrypoint.py; set com.urunc.unikernel.cmdline to run a
# different script with args instead.
#
# Built by `just stage` into build-elfloader/urunc-hello-rootfs.cpio.
FROM hluk-python-rootfs:latest
COPY demos/urunc/hello.py /entrypoint.py
