# PHP runtime for Hyperlight/Unikraft.
#
# The driver, hl_phpdriver, embeds PHP (the embed SAPI) and runs PHP for
# each call: the source a call carries, or a .php file in the guest
# filesystem or a mount.  PHP is built from source into a static libphp,
# with the extensions and the libraries they need linked in, so engine and
# driver are one static-PIE binary: the rootfs is the driver, the CA
# certificates, /etc/passwd and /etc/group, and /tmp.
#
# Build context: repository root.

FROM alpine:3.24@sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a AS build

ARG PHP_VERSION=8.5.12
ARG PHP_SHA256=08bc2eba2a588654ade1357174aa646b761556c862dda0678345ee29ed481209

RUN apk add --no-cache gcc make musl-dev curl xz pkgconf linux-headers \
        ca-certificates \
        openssl-dev openssl-libs-static zlib-dev zlib-static \
        sqlite-dev sqlite-static libxml2-dev libxml2-static xz-static \
        libsodium-dev libsodium-static

RUN curl -fsSL -o /php.tar.xz \
        https://www.php.net/distributions/php-${PHP_VERSION}.tar.xz && \
    echo "${PHP_SHA256}  /php.tar.xz" | sha256sum -c - && \
    mkdir /php && tar -xJf /php.tar.xz -C /php --strip-components=1

# No CLI, CGI or FPM: the embed SAPI as a static library is all the driver
# links.  Off by default and left off: extensions that would need a shared
# library or a large data file in the image (intl's ICU, fileinfo's magic
# database, curl and its protocol libraries), and those for a web server
# (session).  mbstring without its regular expressions, which need
# oniguruma.  php.ini is read from /usr/local/etc/php, as in the official
# images, for an image built FROM this one to add.
RUN cd /php && \
    CFLAGS="-O2 -fPIE" ./configure \
        --prefix=/usr/local \
        --with-config-file-path=/usr/local/etc/php \
        --with-config-file-scan-dir=/usr/local/etc/php/conf.d \
        --disable-all --enable-embed=static \
        --disable-cli --disable-cgi --disable-phpdbg --without-pear \
        --enable-bcmath --enable-calendar --enable-ctype --enable-filter \
        --enable-mbstring --disable-mbregex --enable-phar --enable-posix \
        --enable-sockets --enable-tokenizer \
        --with-openssl --with-sodium --with-zlib \
        --enable-pdo --with-sqlite3 --with-pdo-sqlite \
        --with-libxml --enable-dom --enable-simplexml --enable-xml \
        --enable-xmlreader --enable-xmlwriter && \
    make -j"$(nproc)" && \
    make install-sapi install-headers install-programs

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/php/hl_phpdriver.c /src/drivers/php/hl_phpdriver.c

# The libraries' static archives after libphp's, with what each of them
# needs in turn (libxml2's liblzma and libz, OpenSSL's libcrypto).
RUN gcc -O2 -fPIE -Wall -Wextra -Wno-unused-parameter \
        $(php-config --includes) -I/src/drivers \
        -c -o /hl_phpdriver.o /src/drivers/php/hl_phpdriver.c && \
    gcc -static-pie -s -o /hl_phpdriver /hl_phpdriver.o /usr/local/lib/libphp.a \
        $(pkg-config --static --libs libxml-2.0 libsodium openssl sqlite3 zlib) \
        -lm -lpthread

# The users and groups (root among them), so posix_getpwuid() and
# get_current_user() resolve as on Linux, and the CA bundle at the path
# OpenSSL looks in (OPENSSLDIR is /etc/ssl), so https:// verifies peers.
# And /proc/self/maps, which the kernel serves as /dev/maps: PHP reads the
# main thread's stack from it, and without it takes Linux's 8 MiB for the
# guest's 2 MiB, so deep recursion would run past the stack before PHP's
# "Maximum call stack size" error.
RUN mkdir -p /rootfs/etc/ssl /rootfs/tmp /rootfs/usr/local/bin /rootfs/proc/self && \
    ln -s /dev/maps /rootfs/proc/self/maps && \
    cp /etc/passwd /etc/group /rootfs/etc/ && \
    cp -L /etc/ssl/cert.pem /rootfs/etc/ssl/cert.pem && \
    cp /hl_phpdriver /rootfs/usr/local/bin/hl_phpdriver

FROM scratch
COPY --from=build /rootfs/ /
