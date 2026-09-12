# Shared BusyBox base image (hluk-busybox).
#
# A NOMMU/PIE BusyBox built against Alpine/musl, used as the minimal userland
# for the bash, agent and python-shell rootfs.  Building it here once — rather
# than repeating the ~40-line build in each of those Dockerfiles — keeps the
# BusyBox version, config and applet set in a single place.  Consumers do:
#     COPY --from=hluk-busybox /lib/ld-musl-x86_64.so.1 /lib/ld-musl-x86_64.so.1
#     COPY --from=hluk-busybox /busybox-root/ /
# so this image must be built first (see `just build-busybox`, invoked by
# `just build-rootfs` for those runtimes and by `just build-all-rootfs`).
FROM alpine:3.20

RUN apk add --no-cache gcc musl-dev make perl linux-headers

ARG BUSYBOX_VERSION=1.36.1
ARG BUSYBOX_SHA256=b8cc24c9574d809e7279c3be349795c5d5ceb6fdf19ca709f80cde50e47de314
# busybox.net is a single origin with no CDN and intermittently times out,
# which fails the build on any Docker cache miss.  Fetch from the Buildroot
# source mirror first (CDN-backed, maintained for exactly this), fall back to
# upstream, and verify the checksum so the mirror is trustworthy.
RUN set -eux; \
    f="busybox-${BUSYBOX_VERSION}.tar.bz2"; \
    for url in \
        "https://sources.buildroot.net/busybox/$f" \
        "https://busybox.net/downloads/$f"; do \
        wget -q -T 30 -O "$f" "$url" && break || true; \
    done; \
    [ "$(sha256sum "$f" | cut -d' ' -f1)" = "${BUSYBOX_SHA256}" ] \
        || { echo "busybox checksum mismatch" >&2; exit 1; }; \
    tar xjf "$f" && \
    mv "busybox-${BUSYBOX_VERSION}" /busybox-src

WORKDIR /busybox-src

# NOMMU + standalone/NOFORK shell so applets run without fork/exec; PIE so the
# binary loads at the address Hyperlight maps it to.
RUN make defconfig && \
    sed -i \
        -e 's/# CONFIG_NOMMU is not set/CONFIG_NOMMU=y/' \
        -e 's/# CONFIG_FEATURE_PREFER_APPLETS is not set/CONFIG_FEATURE_PREFER_APPLETS=y/' \
        -e 's/# CONFIG_FEATURE_SH_STANDALONE is not set/CONFIG_FEATURE_SH_STANDALONE=y/' \
        -e 's/# CONFIG_FEATURE_SH_NOFORK is not set/CONFIG_FEATURE_SH_NOFORK=y/' \
        -e 's|CONFIG_BUSYBOX_EXEC_PATH=.*|CONFIG_BUSYBOX_EXEC_PATH="/bin/busybox"|' \
        -e 's/# CONFIG_PIE is not set/CONFIG_PIE=y/' \
        .config 2>/dev/null; \
    yes '' | make oldconfig

RUN make -j$(nproc) EXTRA_CFLAGS="-fPIE" EXTRA_LDFLAGS="-pie" && \
    make install

# Assemble the rootfs at /busybox-root: the binary plus applet symlinks.
RUN mkdir -p /busybox-root/bin /busybox-root/usr/bin \
             /busybox-root/sbin /busybox-root/tmp /busybox-root/etc && \
    cp /busybox-src/busybox /busybox-root/bin/busybox && \
    cd /busybox-root/bin && \
    for applet in \
        sh ash \
        echo printf cat head tail wc sort uniq tr tee \
        ls mkdir rmdir rm cp mv ln chmod chown touch \
        grep egrep sed awk cut paste \
        basename dirname realpath readlink \
        date env sleep true false yes test \
        uname hostname id whoami which \
        dd od hexdump base64 md5sum sha256sum \
        expr seq xargs \
        find stat du df \
    ; do \
        ln -s busybox "$applet"; \
    done

# Minimal /etc for the shell.
RUN printf 'root:x:0:0:root:/root:/bin/sh\n' > /busybox-root/etc/passwd && \
    printf 'root:x:0:\n' > /busybox-root/etc/group
