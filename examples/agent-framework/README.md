# Agent Framework on Hyperlight

Two [Microsoft Agent Framework](https://github.com/microsoft/agent-framework) agents running inside a Hyperlight micro-VM: one doing **fully offline** inference, one calling a **remote** hosted model.

Both are real `agent_framework` Agents with a custom `BaseChatClient`. Both step the coroutine directly (`run_sync`) instead of `asyncio.run()`, because the unikernel has no `socket.socketpair()` for an event loop's self-pipe and the inference call never suspends.

## Remote (GitHub Models API)

`remote.py` calls the GitHub Models inference API (OpenAI-compatible) over the network with a small synchronous `urllib` client. The token is passed at run time and never baked into the image or written to disk.

```bash
just build-rootfs agent-fw-remote examples/agent-framework/remote.Dockerfile

export GITHUB_TOKEN=...   # a token with the models:read scope
hluk run --initrd build-elfloader/agent-fw-remote-rootfs.cpio --scratch-mb 512 \
    --net --env GITHUB_TOKEN="$GITHUB_TOKEN" \
    examples/agent-framework/remote.py
```

The model defaults to `openai/gpt-4o-mini`; override with `--env GITHUB_MODELS_MODEL=...`.

## Local (offline llama.cpp)

`local.py` runs a small `Qwen2.5-0.5B-Instruct` GGUF with `llama-cpp-python`, baked into the rootfs. Inference is entirely in-guest — no network, no keys.

```bash
just build-rootfs agent-fw-local examples/agent-framework/local.Dockerfile

hluk run --initrd build-elfloader/agent-fw-local-rootfs.cpio --scratch-mb 1536 \
    examples/agent-framework/local.py
```

Example output (generation is a few tok/s: single vCPU, SSE-only):

```
User:  In one sentence, what is a micro-VM?
Agent: A micro-VM is a lightweight virtual machine that boots in milliseconds
       and runs a single workload in a hardware-isolated address space.
```

Why the build looks the way it does:

- **SSE-only llama.cpp.** The guest CPU exposes SSE/SSE2/SSE4 but **not AVX/AVX2**. The prebuilt `llama-cpp-python` wheels assume AVX2 and crash with an illegal instruction, so the `Dockerfile` compiles llama.cpp from source with `-DGGML_AVX=OFF -DGGML_AVX2=OFF -DGGML_FMA=OFF -DGGML_F16C=OFF`.
- **OpenMP off** (`-DGGML_OPENMP=OFF`) to avoid a libgomp runtime dependency; ggml falls back to pthreads.
- **Small buffers, no mmap.** `n_ctx=512`, `n_batch=64`, and `use_mmap=False` keep the compute buffers inside the identity-mapped guest memory.

The rootfs is ~540 MB (it bakes in the ~380 MB model), and the whole cpio is extracted into the guest ramfs on boot; `use_mmap=False` then reads the model into a second buffer. So the guest needs ~1.5 GB of scratch — `--scratch-mb 1536` is the floor here (1024/1280 fail to even extract the rootfs); more is fine.

Generation is a few tokens/sec because the guest is a **single vCPU** and the model runs **SSE-only** (the guest CPU exposes no AVX/AVX2, so llama.cpp is compiled without them) with OpenMP off. It's scalar inference on one core; that's a guest-capability limit, not a config to tune.
- **stdlib stubs.** `local.py` registers no-op `multiprocessing`/`sqlite3` stubs that `llama-cpp-python` imports transitively but this example never uses; they are ignored when the real modules are present.

Both rootfses extend `python-shell` (build it first with `just build-rootfs python-shell`). The local build compiles llama.cpp and downloads a ~400 MB model, so it takes several minutes.
