# agent-framework-local rootfs — extends python-shell with agent-framework-core,
# a source-built llama-cpp-python, and a small GGUF model baked in. Inference
# runs entirely in-guest: no network, no API keys.
#
# Two constraints drive the build:
#   1. The guest CPU exposes SSE/SSE2/SSE4 but NOT AVX/AVX2/FMA/F16C, so the
#      prebuilt llama-cpp-python wheels (which assume AVX2) crash with an
#      illegal instruction. We compile from source with those disabled.
#   2. OpenMP is disabled to avoid a libgomp runtime dependency (ggml falls
#      back to pthreads).
#
# Build:  just build-rootfs agent-fw-local examples/agent-framework/local.Dockerfile
#         (compiles llama.cpp + downloads a ~400 MB model; takes a while)
# Run:    hluk run --initrd build-elfloader/agent-fw-local-rootfs.cpio \
#             --scratch-mb 1536 examples/agent-framework/local.py
FROM hluk-python-shell-rootfs:latest AS base

FROM python:3.12-slim-bookworm AS installer
RUN apt-get update && apt-get install -y --no-install-recommends git build-essential cmake \
    && rm -rf /var/lib/apt/lists/*
COPY --from=base / /rootfs/
RUN pip install --no-cache-dir --target /rootfs/usr/local/lib/python3.12/site-packages \
        agent-framework-core
# SSE-only, no AVX/AVX2/AVX512/FMA/F16C (all VEX-encoded and unsupported here),
# and no OpenMP.
ENV CMAKE_ARGS="-DGGML_NATIVE=OFF -DGGML_AVX=OFF -DGGML_AVX2=OFF -DGGML_AVX512=OFF -DGGML_FMA=OFF -DGGML_F16C=OFF -DGGML_OPENMP=OFF"
RUN pip install --no-cache-dir --target /rootfs/usr/local/lib/python3.12/site-packages \
        --no-binary llama-cpp-python llama-cpp-python
RUN find /rootfs/usr/local/lib/python3.12/site-packages \
        \( -type d -name tests -o -type d -name __pycache__ \) -prune -exec rm -rf {} + 2>/dev/null || true

# Download the GGUF model in its own stage (layer caching). Swap MODEL_URL to
# use a different model.
FROM debian:bookworm-slim AS model
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*
ARG MODEL_URL=https://huggingface.co/bartowski/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf
RUN curl -fL --retry 3 -o /model.gguf "${MODEL_URL}"

FROM scratch
COPY --from=installer /rootfs/ /
COPY --from=model /model.gguf /model.gguf
