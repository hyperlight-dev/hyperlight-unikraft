# agent-framework-local — a Microsoft Agent Framework agent doing REAL, fully
# offline inference inside a Hyperlight micro-VM.
#
# Bundles llama-cpp-python + a small Qwen2.5-0.5B-Instruct GGUF into the CPIO,
# so the model runs entirely in-guest — no network, no API keys.

ARG BASE=ghcr.io/hyperlight-dev/hyperlight-unikraft/python-base:latest
ARG DEPS_BASE=local-python-base-dev:latest

# Stage 1: python deps — agent-framework-core + llama-cpp-python. Two constraints
# drive the build: (1) the guest only exposes SSE (no AVX), and (2) native deps
# should be built against the same Ubuntu 22.04 / glibc 2.35 ABI as python-base.
# The Justfile materializes DEPS_BASE from the python-dev stage in
# ../../runtimes/python.Dockerfile.
FROM ${DEPS_BASE} AS deps
RUN apt-get update && apt-get install -y --no-install-recommends git \
    && rm -rf /var/lib/apt/lists/*
RUN python3.12 -m pip install --no-cache-dir cmake
RUN python3.12 -m pip install --target=/deps --no-cache-dir --prefer-binary agent-framework-core
# SSE-only, no AVX/AVX2/AVX512/FMA/F16C (all VEX-encoded and unsupported here),
# and no OpenMP (avoids a libgomp runtime dependency; ggml falls back to pthreads).
ENV CMAKE_ARGS="-DGGML_NATIVE=OFF -DGGML_AVX=OFF -DGGML_AVX2=OFF -DGGML_AVX512=OFF -DGGML_FMA=OFF -DGGML_F16C=OFF -DGGML_OPENMP=OFF"
RUN python3.12 -m pip install --target=/deps --no-cache-dir --no-binary llama-cpp-python llama-cpp-python
RUN set -eux; \
    find /deps -maxdepth 1 -type d -name '*.dist-info' -exec rm -rf {} + 2>/dev/null || true; \
    find /deps \( -type d -name tests -o -type d -name test \) -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true; \
    find /deps -type f -name '*.pyc' -delete 2>/dev/null || true

# Stage 2: download the GGUF model (kept in its own stage for layer caching).
FROM debian:bookworm-slim AS model
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*
ARG MODEL_URL=https://huggingface.co/bartowski/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf
RUN curl -fL --retry 3 -o /model.gguf "${MODEL_URL}"

# Stage 3: assemble rootfs.
FROM ${BASE} AS rootfs
COPY --from=deps  /deps        /usr/local/lib/python3.12/site-packages
COPY --from=model /model.gguf  /model.gguf
COPY agent.py /agent.py

# Stage 4: pack as CPIO.
FROM alpine:3.20 AS cpio
RUN apk add --no-cache cpio findutils
COPY --from=rootfs / /rootfs/
RUN cd /rootfs && find . | cpio -o -H newc > /output.cpio 2>/dev/null
