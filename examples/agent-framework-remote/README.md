# agent-framework-remote on Hyperlight

Run a [Microsoft Agent Framework](https://github.com/microsoft/agent-framework)
agent that calls a **remote hosted model** — [GitHub Models](https://github.com/marketplace/models)
(OpenAI-compatible) — over the network from inside a
[Hyperlight](https://github.com/hyperlight-dev/hyperlight) micro-VM.

This example uses the **python-agent-driver / `pyhl` stack**: a warmed CPython
interpreter that is snapshotted once and then restored per run (~2–3 s/run, no
kernel boot). The shipped driver kernel already includes **host-proxied
networking + hostfs**, so there is no custom kernel to build. The rootfs is kept
small (~64 MB) by taking only the shipped `hl_pydriver` interpreter shim plus
`agent-framework-core` — not the ~1 GB preloaded data-science stack the
general-purpose pyhl image carries.

`agent.py` is a real `agent_framework` `Agent` backed by a custom `BaseChatClient`
that POSTs to the GitHub Models API. The token is provided at run time through
`pyhl run --env GITHUB_TOKEN` — never baked into the image or written to disk.

## Prerequisites

- Rust/Cargo, so the Justfile can run the in-repo **`pyhl`** with `cargo run`:

  ```sh
  cargo build --manifest-path ../../host/Cargo.toml --bin pyhl
  ```

  To use an installed `pyhl` instead, point the Justfile at it:

  ```sh
  export PYHL=pyhl
  ```

  (The published GHCR driver image is version-matched to released `pyhl` builds.)
- A token valid for GitHub Models in `$GITHUB_TOKEN`, or an authenticated
  [`gh`](https://cli.github.com/) CLI so `just run` can call `gh auth token`.
  To set the token explicitly:

  ```sh
  export GITHUB_TOKEN="$(gh auth token)"
  ```

## Run

```sh
just build      # fetch the shipped driver kernel from GHCR
just rootfs     # build a ~64 MB initrd: shipped hl_pydriver + agent-framework-core
just setup      # one-time: warm up + persist a Python snapshot (~24 s)
just run        # run the agent (restores the snapshot, ~2-3 s)
```

Example output:

```
User:  In one short sentence, what is Hyperlight?
Agent: Hyperlight is a technology that enables the transmission of data ...
```

To use a different model, set `GITHUB_MODELS_MODEL` (default `openai/gpt-4o-mini`).
For example:

```sh
GITHUB_MODELS_MODEL=openai/gpt-5 just run
```

The example uses `max_completion_tokens` for compatibility with GPT-5 models.
For `openai/gpt-5*`, it defaults to `GITHUB_MODELS_MAX_COMPLETION_TOKENS=1024`
and `GITHUB_MODELS_REASONING_EFFORT=minimal`; set either variable before
`just run` to override those defaults.

## How it works

- **Shipped driver stack, minimal rootfs.** The `python-agent-driver` kernel
  published to GHCR enables networking (`CONFIG_LIBPOSIX_SOCKET` +
  `CONFIG_LIBHOSTSOCK`) and hostfs. `just build` pulls the kernel; `just rootfs`
  extracts just the shipped `hl_pydriver` (so it stays version-matched to that
  kernel) and lays it onto `python-base` with `agent-framework-core` added
  (pydantic, an agent-framework dependency, is already in `python-base`). This
  keeps the initrd ~64 MB instead of the ~1 GB general-purpose pyhl image.
- **`--net` + egress.** The guest has no network unless `--net` is passed. The
  shipped rootfs already ships CA certificates + `/etc/resolv.conf` for outbound
  TLS. The Justfile uses `--net-allow models.github.ai` so the guest can only
  reach the GitHub Models endpoint.
- **Token via `--env`.** `just run` passes `$GITHUB_TOKEN` into the guest Python
  environment with `pyhl run --env GITHUB_TOKEN`. If `GITHUB_MODELS_MODEL` is set,
  the Justfile passes that through the same way.
- **Synchronous client.** Agent Framework provides `OpenAIChatCompletionClient`
  for normal Python apps, but it uses the async OpenAI SDK. This example keeps a
  tiny blocking `urllib` client and steps the coroutine directly instead of
  `asyncio.run()`, keeping the guest free of an event loop (whose self-pipe needs
  `socket.socketpair()`).

## Note on the plain-`hyperlight-unikraft` alternative

The GHCR kernels for the *non-driver* examples (e.g. `python-agent-kernel`) are
built **without** networking, so a plain `hyperlight-unikraft ... -- /script.py`
launch can't open sockets there. Networking requires either this driver stack or a
kernel built with the socket libraries (as in the `networking-py` example).
