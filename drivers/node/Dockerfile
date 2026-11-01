FROM node:22-alpine@sha256:fe4f6d74e20370acb04ad889077d14d0f64a9c8ea6a42bc60cddf1e844699914 AS build

RUN apk add --no-cache gcc musl-dev

# Preserve directory layout so #include "../hl_fc.h" resolves
COPY drivers/hl_fc.h /src/drivers/hl_fc.h
COPY drivers/hl_env.h /src/drivers/hl_env.h
COPY drivers/hl_driver.h /src/drivers/hl_driver.h
COPY drivers/hl_child.h /src/drivers/hl_child.h
COPY drivers/node/hl_nodedriver.c /src/drivers/node/hl_nodedriver.c

RUN gcc -O2 -Wall -Wextra -Wno-unused-parameter \
        -static-pie -fPIE \
        -o /hl_nodedriver /src/drivers/node/hl_nodedriver.c

# The addon that reseeds Node's OpenSSL after a restore: its symbols are
# the node binary's, resolved when the child loads it.
COPY drivers/node/hl_reseed_addon.c /src/drivers/node/hl_reseed_addon.c
RUN gcc -O2 -Wall -Wextra -shared -fPIC -I/usr/local/include/node \
        -o /hl_reseed.node /src/drivers/node/hl_reseed_addon.c

# musl's dynamic linker is named for the architecture (x86_64, aarch64)
RUN mkdir -p /musl && cp /lib/ld-musl-$(uname -m).so.1 /musl/

# Minimal rootfs: Node binary + musl + driver
FROM scratch

# musl dynamic linker + libs
COPY --from=build /musl/ /lib/
COPY --from=build /usr/lib/libgcc_s.so.1 /usr/lib/libgcc_s.so.1
COPY --from=build /usr/lib/libstdc++.so.6 /usr/lib/libstdc++.so.6

# Node binary
COPY --from=build /usr/local/bin/node /usr/bin/node

# The child's OpenSSL reseed addon (hl_reseed_addon.c)
COPY --from=build /hl_reseed.node /usr/lib/node/hl_reseed.node

# Hyperlight driver
COPY --from=build /hl_nodedriver /usr/local/bin/hl_nodedriver

# /tmp needed for bootstrap JS at runtime
COPY --from=build /tmp /tmp
