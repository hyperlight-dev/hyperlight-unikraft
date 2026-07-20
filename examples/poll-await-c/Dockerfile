FROM alpine:3.20 AS build
RUN apk add --no-cache musl-dev gcc
COPY poll.c /src/poll.c
RUN gcc -static-pie -fPIE -fno-stack-protector -o /src/poll /src/poll.c

FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils
RUN mkdir -p /rootfs/bin
COPY --from=build /src/poll /rootfs/bin/poll
RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
