# agent-framework-local on Hyperlight

Run a [Microsoft Agent Framework](https://github.com/microsoft/agent-framework)
agent doing **real, fully offline inference** inside a
[Hyperlight](https://github.com/hyperlight-dev/hyperlight) micro-VM.

`agent.py` is a real `agent_framework` `Agent` backed by a custom `BaseChatClient`
that runs a small GGUF model with **llama-cpp-python**. The model is baked into the
image, so inference happens entirely in-guest — **no network, no API keys**.

- Model: `Qwen2.5-0.5B-Instruct` (Q4_K_M, ~397 MB), swappable via the `MODEL_URL`
  build arg in the `Dockerfile`.

## Run

```sh
just build      # fetch the prebuilt Python kernel from GHCR
just rootfs     # build the initrd CPIO (compiles llama.cpp + bakes in the model)
just run        # run the agent inside the micro-VM
```

Example output (generation is ~4–5 tok/s: single vCPU, SSE-only):

```
User:  In one sentence, what is a micro-VM?
Agent: A micro-VM refers to a lightweight virtual machine that is small enough to run on a single hardware device, typically a server or cloud instance.
```

## Why the build looks the way it does

Getting a native inference stack to run on the trimmed `python-base` +
identity-mapped unikernel needed a few specific choices, all in the `Dockerfile`
and `agent.py`:

- **SSE-only llama.cpp.** The guest CPU exposes SSE/SSE2/SSE4 but **not AVX/AVX2**
  (numpy reports `AVX: False`). The prebuilt `llama-cpp-python` CPU wheels assume
  AVX2 and crash with an illegal instruction, so we compile llama.cpp from source
  with `-DGGML_AVX=OFF -DGGML_AVX2=OFF -DGGML_FMA=OFF -DGGML_F16C=OFF`.
- **Matching glibc + OpenMP off.** `python-base` ships Ubuntu glibc 2.35, so the
  Justfile builds `local-python-base-dev` from the `python-dev` stage in
  `runtimes/python.Dockerfile` and compiles native deps against that same ABI.
  We disable OpenMP
  (`-DGGML_OPENMP=OFF`) to avoid a libgomp runtime dependency.
- **Small buffers, no mmap.** `n_ctx=512`, `n_batch=64`, and `use_mmap=False` keep
  the compute buffers within the identity-mapped guest memory (larger values crash
  it). Run with `--memory 3Gi`.
- **stdlib stubs.** `python-base` trims `multiprocessing` and `sqlite3`, which
  llama-cpp-python imports transitively; `agent.py` registers minimal stubs since
  neither is actually used here.
- **`run_sync` instead of `asyncio.run()`.** The unikernel has no
  `socket.socketpair()`, which asyncio's event loop requires; the synchronous
  inference call never suspends, so we step the coroutine directly.

The kernel is generic (it runs `/usr/local/bin/python3 <cmdline>`), so this example
reuses the published `python-agent-kernel` instead of building its own.
