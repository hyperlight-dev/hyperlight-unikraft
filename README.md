<div align="center">
    <h1>Hyperlight</h1>
    <img src="https://raw.githubusercontent.com/hyperlight-dev/hyperlight/refs/heads/main/docs/assets/hyperlight-logo.png" width="150px" alt="hyperlight logo"/>
    <p><strong>Hyperlight is a lightweight Virtual Machine Manager (VMM) designed to be embedded within applications. It enables safe execution of untrusted code within <i>micro virtual machines</i> with very low latency and minimal overhead.</strong> <br> We are a <a href="https://cncf.io/">Cloud Native Computing Foundation</a> sandbox project. </p>
</div>

# hyperlight-unikraft

[![crates.io](https://img.shields.io/crates/v/hyperlight-unikraft.svg)](https://crates.io/crates/hyperlight-unikraft)

Run ordinary Linux programs (e.g., Python, Node.js, .NET, Go, Rust, C, Bash) inside [Hyperlight](https://github.com/hyperlight-dev/hyperlight) micro-VMs, using a [Unikraft](https://github.com/unikraft/unikraft) unikernel as the guest kernel.

hyperlight-unikraft ships as both a Rust library and a CLI. `hluk` (the CLI) boots the kernel, mounts a rootfs, runs your workload behind a **default-deny** host boundary (no filesystem or network unless you opt in), and can snapshot a warmed guest to skip startup on later runs. The same building blocks (`SandboxBuilder`, `run`, snapshot save/restore) are exposed as a library, so you can embed guest execution in your own application (`hluk` itself is a good reference consumer of it).

## Quick start

```bash
# 1. Install the CLI (the Unikraft kernel is baked into the binary)
curl -fsSL https://raw.githubusercontent.com/hyperlight-dev/hyperlight-unikraft/main/install.sh | sh
#    or, with a Rust toolchain: cargo install hyperlight-unikraft

# 2. Start a project from a template; the matching rootfs is pulled from GHCR
hluk init hello --template python          # or plain `hluk init` to be asked
cd hello

# 3. Run it inside a micro-VM
hluk run                                    # boots the guest and runs main.py
hluk run                                    # restores the warm snapshot the first run saved: milliseconds

# Or skip the project: a published image by name, warm by default
hluk run --runtime python examples/python/hello.py
```

`hluk templates` lists what `init` can start from: a script for Python, Node.js, Bash, C# (compiled in the guest by Roslyn) and PowerShell; a compiled program for Go, Rust, C and .NET Native AOT (`hluk build` runs the compiler, `hluk run` mounts the result); an HTTP server on a rootfs you extend with pip, npm or the .NET SDK through a Dockerfile (`hluk build` builds it with Docker); and the `agent` image with numpy, pandas, scipy and scikit-learn preloaded. The project's `hluk.toml` says what runs and with which capabilities; [`docs/manifest.md`](docs/manifest.md) is the reference.

Without a project, `hluk run` takes the same settings as flags (`--initrd`, `--mount`, `--net`, `--net-allow`, `--net-block`, `--port`, `--exec`, `--scratch-mb`, …), and `hluk snapshot save` / `hluk snapshot run` save and restore a guest by hand; `hluk --help` has the full set. To build a rootfs from this repository's recipes instead of pulling one, `just build-rootfs python` (needs Docker and `just`) writes `build-elfloader/python-rootfs.cpio`, which `hluk run --initrd` or a manifest's `[rootfs] path` takes. The `just` recipes (`just run <runtime> <script>`, `just snapshot-save`, `just bench`) wrap a locally built `./target/release/hluk`, which is handy when developing on the repo.

## Runtimes and platforms

hyperlight-unikraft runs on **Linux** (KVM, or MSHV with `cargo build --features mshv`), **Windows** (WHP), and **macOS** (HVF).

Guest runtimes include **Python**, **Node.js**, **.NET** (JIT and AOT), **Bash**, **Go**, **Rust**, **C**, and **PowerShell**. Two are Python variants with a BusyBox shell: **python-shell** (Python + a shell; a general base other images build on) and **agent** (python-shell plus preloaded data-science packages like numpy, pandas, scipy, scikit-learn, which are commonly used for agent code-executors). Any Python guest with `--net` can also `pip install` packages on demand from inside the guest (see [`examples/agent/pip_install.py`](examples/agent/pip_install.py)). See [support tiers](docs/guest-support-tiers.md) for the guarantee behind each; `just list-runtimes` prints the current set.

## Examples

Guest scripts under `examples/` run with `hluk run` (or `just run <runtime> <script>`):

- **HTTP servers** — `examples/http-server/{flask,express,kestrel}/` also show how to build a custom guest image on top of hluk's base runtime rootfs images (Python, Node, .NET AOT); see e.g., [`examples/http-server/flask/Dockerfile`](examples/http-server/flask/Dockerfile). The `http-python`, `http-node` and `http-dotnet` templates of `hluk init` are the same three as projects.
- **Concurrency** — `examples/python/threading_demo.py` and `examples/python/subprocess_demo.py` exercise guest threads and subprocesses.
- **Agents** — `examples/agent-framework/` runs a Microsoft Agent Framework agent with fully offline llama.cpp inference (`local.py`) or a remote model call (`remote.py`).

Larger, self-contained demos live under `demos/`, each with its own `Justfile` and `README.md` (Linux):

- `demos/supply-chain/` — a typosquat supply-chain attack, contained by the VM.
- `demos/pptx-gen/` — LLM-written `python-pptx` code run in a sandbox to build a `.pptx`.
- `demos/urunc/` — deploy a guest through [urunc](https://github.com/urunc-dev/urunc) and containerd.

## How it works

1. **Load** — `hluk` embeds the Unikraft elfloader kernel and maps a rootfs CPIO (the guest userland) as the initrd.
2. **Boot** — Hyperlight starts a micro-VM and jumps to the kernel, which mounts the rootfs and runs your program through a small per-runtime driver.
3. **Sandbox** — by default the guest reaches nothing on the host. Capabilities are opt-in: `--mount` preopens a host directory, `--net` (with optional `--net-allow`/`--net-block` lists) enables networking, and `--port` lets the guest listen.
4. **Snapshot** — once a guest is warmed (interpreter initialised, imports loaded), its state can be saved and restored, so later invocations skip startup.

More details are in [`docs/`](docs/): projects, the manifest and the published images ([`manifest.md`](docs/manifest.md)), how a guest is driven and the ways to run one ([`execution.md`](docs/execution.md)), the contract between a runtime driver and the kernel ([`driver.md`](docs/driver.md)), the host filesystem sandbox ([`fs.md`](docs/fs.md)), guest networking ([`net.md`](docs/net.md)), guest concurrency and snapshot restore ([`concurrency.md`](docs/concurrency.md)), the guest's clocks ([`clock.md`](docs/clock.md)) and its random source ([`random.md`](docs/random.md)).

## Development

```bash
just build-all-rootfs      # build every guest rootfs
just test                  # unit + integration tests (need the rootfs CPIOs)
just conformance python    # run a tier-1 runtime's upstream test suite
just ci                    # lint, license headers, tests, and demo smoke tests
```

## Benchmarks

Cold start, snapshot-restore latency, and throughput are measured on every merge to `main` for tier 1 runtimes, with trends [published to GitHub Pages](https://hyperlight-dev.github.io/hyperlight-unikraft).

## Join our community

Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for how to contribute.

We hold **weekly community meetings**, open to everyone.

- **When**: Mondays at 09:00 (PST/PDT). Convert to your local time [here](https://dateful.com/convert/pst-pdt-pacific-time?t=09).
- **Where**: See the [Hyperlight community meeting notes](https://hackmd.io/blCrncfOSEuqSbRVT9KYkg#Agenda) for the agenda and join link.

## Chat with us on the CNCF Slack

The Hyperlight project Slack lives in the CNCF Slack `#hyperlight` channel. [Join the CNCF Slack](https://www.cncf.io/membership-faq/#how-do-i-join-cncfs-slack), then join `#hyperlight`.

## More information

For the broader project, see the main [Hyperlight](https://github.com/hyperlight-dev/hyperlight) repository.

## Code of Conduct

See the [CNCF Code of Conduct](https://github.com/cncf/foundation/blob/main/code-of-conduct.md).

## License

Licensed under the [Apache License, Version 2.0](./LICENSE).
