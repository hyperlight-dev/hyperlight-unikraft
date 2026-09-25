# Wasmtime runtime for Hyperlight/Unikraft.
#
# The driver, hl_wasmtimedriver (drivers/wasmtime/driver), embeds
# Wasmtime and runs WebAssembly modules and components under WASI: the
# WebAssembly text a call carries, or a .wasm/.cwasm file in the guest
# filesystem or a mount.  It is a Rust static-PIE binary, so the rootfs is
# the driver and nothing else.
#
# Build context: repository root.

FROM rust:1.98-bookworm AS build

COPY drivers/wasmtime/driver /src
WORKDIR /src
# Static and position-independent, as the elfloader wants (see
# examples/rust/README.md).  The getrandom crate calls the getrandom
# syscall directly: its default probe falls back to poll()ing
# /dev/random, which never reports ready in the guest.
ENV RUSTFLAGS="-C target-feature=+crt-static -C relocation-model=pie --cfg getrandom_backend=\"linux_getrandom\""
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked --target x86_64-unknown-linux-gnu && \
    cp target/x86_64-unknown-linux-gnu/release/hl_wasmtimedriver /hl_wasmtimedriver

RUN mkdir -p /empty-tmp

FROM scratch
COPY --from=build /empty-tmp /tmp
COPY --from=build /hl_wasmtimedriver /usr/local/bin/hl_wasmtimedriver
