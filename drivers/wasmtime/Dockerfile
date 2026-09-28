# Wasmtime runtime for Hyperlight/Unikraft.
#
# The driver, hl_wasmtimedriver (drivers/wasmtime/driver), embeds
# Wasmtime and runs WebAssembly modules and components under WASI: the
# WebAssembly text a call carries, or a .wasm/.cwasm file in the guest
# filesystem or a mount.  On x86_64 it is a Rust static-PIE binary, so the
# rootfs is the driver and nothing else.  Rust 1.98 links no static PIE
# for arm64 (1.99 does), so there it is a dynamic PIE and the rootfs
# carries the glibc it links.
#
# Build context: repository root.

# The same Rust as rust-toolchain.toml.
FROM rust:1.98.0-bookworm AS build

COPY drivers/wasmtime/driver /src
WORKDIR /src
# Position-independent, as the elfloader wants (see examples/rust/README.md),
# and static where rustc can make that.  The getrandom crate calls the
# getrandom syscall directly: its default probe falls back to poll()ing
# /dev/random, which never reports ready in the guest.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    set -eu; \
    arch=$(uname -m); target=$arch-unknown-linux-gnu; \
    RUSTFLAGS='--cfg getrandom_backend="linux_getrandom"'; \
    [ "$arch" != x86_64 ] || \
        RUSTFLAGS="-C target-feature=+crt-static -C relocation-model=pie $RUSTFLAGS"; \
    export RUSTFLAGS; \
    cargo build --release --locked --target $target && \
    cp target/$target/release/hl_wasmtimedriver /hl_wasmtimedriver

# The rootfs: the driver, and for a dynamic one the libraries it links.
RUN set -eu; \
    mkdir -p /rootfs/tmp /rootfs/usr/local/bin; \
    cp /hl_wasmtimedriver /rootfs/usr/local/bin/; \
    if ldd /hl_wasmtimedriver > /dev/null 2>&1; then \
        for lib in $(ldd /hl_wasmtimedriver | grep -o '/[^ ]*'); do \
            mkdir -p "/rootfs$(dirname "$lib")"; cp -L "$lib" "/rootfs$lib"; \
        done; \
    fi

FROM scratch
COPY --from=build /rootfs/ /
