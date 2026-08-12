#syntax=harbor.nbfc.io/nubificus/bunny:latest
FROM alpine:3.20 AS build
RUN apk add --no-cache musl-dev gcc cpio findutils
COPY hello.c /src/hello.c
RUN gcc -static-pie -fPIE -fno-stack-protector -o /src/hello /src/hello.c
RUN mkdir -p /rootfs/bin && cp /src/hello /rootfs/bin/hello \
    && cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null

FROM ghcr.io/hyperlight-dev/hyperlight-unikraft/helloworld-c-kernel:latest AS kernel

FROM scratch
COPY --from=build /output.cpio /unikernel/initrd.cpio
COPY --from=kernel /kernel /unikernel/kernel
LABEL "com.urunc.unikernel.unikernelType"="unikraft"
LABEL "com.urunc.unikernel.hypervisor"="hyperlight-unikraft"
LABEL "com.urunc.unikernel.binary"="/unikernel/kernel"
LABEL "com.urunc.unikernel.initrd"="/unikernel/initrd.cpio"
