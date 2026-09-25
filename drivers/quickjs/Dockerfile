# QuickJS runtime for Hyperlight/Unikraft.
#
# The driver, hl_quickjsdriver, embeds QuickJS (quickjs-ng) and its std/os
# library and runs JavaScript for each call: the source a call carries, or
# a .js/.mjs file in the guest filesystem or a mount.  Engine and driver
# are one static-PIE binary, so the rootfs is the driver and /tmp.
#
# Build context: repository root.

FROM alpine:3.20 AS build

ARG QUICKJS_VERSION=0.17.0
ARG QUICKJS_SHA256=559bc4c420475e55c7ab4510adbc562f55d7524d75e8e89d79ce4bb02f5687d9

RUN apk add --no-cache gcc musl-dev curl

RUN curl -fsSL -o /quickjs.tar.gz \
        https://github.com/quickjs-ng/quickjs/archive/refs/tags/v${QUICKJS_VERSION}.tar.gz && \
    echo "${QUICKJS_SHA256}  /quickjs.tar.gz" | sha256sum -c - && \
    mkdir /quickjs && tar -xzf /quickjs.tar.gz -C /quickjs --strip-components=1

COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/quickjs/hl_quickjsdriver.c /src/drivers/quickjs/hl_quickjsdriver.c
COPY drivers/quickjs/hl_qjs_await.c /src/drivers/quickjs/hl_qjs_await.c

# The engine and its std/os library compiled straight into the driver, as
# quickjs-ng's own build does with QJS_BUILD_LIBC; quickjs-libc.c through
# hl_qjs_await.c, which includes it unmodified to reach two of its static
# functions for the driver's await.  The engine's sources (and those two
# wrappers) build without warnings shown; the driver's with them.
RUN cd /quickjs && \
    gcc -O2 -D_GNU_SOURCE -DQJS_BUILD_LIBC -fPIE -w -I/quickjs -c \
        quickjs.c libregexp.c libunicode.c dtoa.c \
        /src/drivers/quickjs/hl_qjs_await.c && \
    gcc -O2 -D_GNU_SOURCE -DQJS_BUILD_LIBC -fPIE \
        -Wall -Wextra -Wno-unused-parameter \
        -I/quickjs -I/src/drivers -c \
        /src/drivers/quickjs/hl_quickjsdriver.c && \
    gcc -static-pie -o /hl_quickjsdriver *.o -lm -lpthread

RUN mkdir -p /empty-tmp

FROM scratch
COPY --from=build /empty-tmp /tmp
COPY --from=build /hl_quickjsdriver /usr/local/bin/hl_quickjsdriver
