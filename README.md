<div align="center">
    <h1>Hyperlight</h1>
    <img src="https://raw.githubusercontent.com/hyperlight-dev/hyperlight/refs/heads/main/docs/assets/hyperlight-logo.png" width="150px" alt="hyperlight logo"/>
    <p><strong>Hyperlight is a lightweight Virtual Machine Manager (VMM) designed to be embedded within applications. It enables safe execution of untrusted code within <i>micro virtual machines</i> with very low latency and minimal overhead.</strong> <br> We are a <a href="https://cncf.io/">Cloud Native Computing Foundation</a> sandbox project. </p>
</div>

# hyperlight-unikraft

Run [Unikraft](https://unikraft.org/) unikernels on [Hyperlight](https://github.com/hyperlight-dev/hyperlight), a lightweight Virtual Machine Manager (VMM) designed for embedded use within applications.

## Overview

This project enables running Linux applications (Python, Node.js, Go, Rust, C/C++) on Hyperlight micro-VMs using Unikraft as the guest kernel. It provides:

1. **hyperlight-unikraft** - A CLI host that loads and runs Unikraft kernels on Hyperlight
2. **Example configurations** - Ready-to-use kraft configs for building various applications

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  Your Application (Python, Node.js, Go, Rust, C/C++)         │
│  (runs as ELF binary inside the VM)                          │
├──────────────────────────────────────────────────────────────┤
│  Unikraft Kernel (ELF loader + VFS + POSIX)                  │
│  - Mounts initrd as ramfs                                    │
│  - Loads and executes application ELF                        │
├──────────────────────────────────────────────────────────────┤
│  hyperlight-unikraft (embedded Hyperlight host)              │
│  - Loads kernel ELF + initrd                                 │
│  - Passes arguments via magic header in initrd               │
├──────────────────────────────────────────────────────────────┤
│  Hyperlight VMM (hypervisor interface)                       │
│  - Creates micro-VM with identity-mapped page tables         │
│  - Provides PEB structure with memory regions                │
├──────────────────────────────────────────────────────────────┤
│  KVM (Linux) / MSHV (Windows)                                │
└──────────────────────────────────────────────────────────────┘
```

### How It Works

1. **Host loads kernel and initrd**: `hyperlight-unikraft` reads the Unikraft kernel ELF and optional initrd (CPIO archive)
2. **Arguments embedded in initrd**: Application arguments are prepended to the initrd with a magic header (`HLCMDLN\0`)
3. **VM starts**: Hyperlight creates a micro-VM with identity-mapped memory and jumps to the kernel entry point
4. **Kernel extracts initrd**: Unikraft mounts the initrd as a RAM filesystem, extracts the embedded cmdline
5. **Application runs**: The ELF loader loads and executes the application binary (e.g., `/usr/bin/python3`)
6. **Output via console**: Application output goes through `outb` to port 0xE9, which Hyperlight captures

### Key Features

- **Thin, opt-in host surface** — by default, the guest has no access to the host filesystem, network, or any host functions. When you enable features like `--mount`, `--net`, or `--enable-tools`, a single `__dispatch` JSON-RPC bridge is registered as the only guest→host channel. See [HOST_FUNCTIONS.md](HOST_FUNCTIONS.md) for the full list of dispatchable operations
- **Identity-mapped memory** - Simplified memory layout (vaddr == paddr)
- **Generic cmdline mechanism** - Pass arguments to any application via `-- arg1 arg2 ...`
- **Fast cold start** - Hyperlight's lightweight design enables millisecond startup times

### Benchmarks

Performance and density benchmarks run on every push to `main` and on PRs. Historical trends are available at:

- [Linux benchmarks](https://hyperlight-dev.github.io/hyperlight-unikraft/dev/bench/linux/)
- [Windows benchmarks](https://hyperlight-dev.github.io/hyperlight-unikraft/dev/bench/windows/)

## Prerequisites

Common on both Linux and Windows:

- [Rust](https://rustup.rs/) 1.89+
- [Docker](https://www.docker.com/) (builds the rootfs CPIO archives)
- [`just`](https://github.com/casey/just) (build runner — replaces Make)

Linux-only (needed to build Unikraft kernels locally):

- KVM (`/dev/kvm` readable/writable)
- Go 1.25+ (builds `kraft-hyperlight`)

Windows-only:

- Windows Hypervisor Platform (`Enable-WindowsOptionalFeature -Online -FeatureName HypervisorPlatform`; reboot)
- Developer Mode enabled (Settings → For developers → Developer Mode)
- Kernels are pulled pre-built from GHCR; `kraft-hyperlight` is not required.

## Setup

### Linux — from scratch

```bash
# 1. Toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install just

# 2. kraft-hyperlight (builds Unikraft kernels)
git clone --branch hyperlight-platform https://github.com/danbugs/kraftkit.git
cd kraftkit && go build -o kraft-hyperlight ./cmd/kraft
sudo mv kraft-hyperlight /usr/local/bin/ && cd ..

# 3. This repo + host CLI
git clone https://github.com/hyperlight-dev/hyperlight-unikraft.git
cd hyperlight-unikraft/host
cargo build --release
sudo cp target/release/hyperlight-unikraft /usr/local/bin/
cd ..

# 4. Run any example
cd examples/helloworld-c
just build      # build the Unikraft kernel with kraft-hyperlight
just rootfs     # build the rootfs CPIO via Docker
just run
```

### Python on Hyperlight with `pyhl`

For Python workloads specifically, the `pyhl` binary wraps the
`python-agent-driver` image (kernel + CPIO with numpy/pandas/pydantic/
yaml/jinja2/bs4/tabulate/click/tenacity/tqdm/openpyxl/pypdf/markdown-it-py/
pillow/lxml/cryptography/dateutil/dotenv preloaded) behind a simple
`setup` / `run` workflow:

```bash
# One-time: build the driver image (kernel + CPIO)
cd examples/python-agent-driver
just rootfs && just build
cd ../..

# Install pyhl
cargo install --git https://github.com/hyperlight-dev/hyperlight-unikraft \
    hyperlight-unikraft-host --bin pyhl

# Point pyhl at the image you just built — creates ./.pyhl/ in cwd
pyhl setup --from examples/python-agent-driver

# Run Python
pyhl run -c 'import pandas as pd; print(pd.DataFrame({"x":[1,2,3]}).sum().to_dict())'
pyhl run my_script.py
pyhl run my_script.py --repeat 4      # 5 hermetic invocations
```

Each `pyhl run` process pays a ~10s cold start (kernel boot + Py_Initialize
+ preloaded imports) once, then every user invocation (including the
first) runs hermetic at ~100ms — the driver snapshots the post-warmup
state and restores between calls, so `__main__` globals and `sys.modules`
don't leak between runs.

`pyhl setup` is idempotent — re-running reports the existing install and
exits 0; pass `--force` to overwrite. Artifacts are found via
`--dest`/`$PYHL_HOME` / `./.pyhl/` / `~/.local/share/pyhl/`, in that order.

### Windows — from scratch

```powershell
# 1. Toolchain
# Install Rust via https://www.rust-lang.org/tools/install
cargo install just

# 2. This repo + host CLI
git clone https://github.com/hyperlight-dev/hyperlight-unikraft.git
cd hyperlight-unikraft\host
cargo build --release
Copy-Item target\release\hyperlight-unikraft.exe $env:USERPROFILE\.cargo\bin\ -Force
cd ..

# 3. Run any example (kernel pulled from GHCR)
cd examples\helloworld-c
just build      # docker pull ghcr.io/hyperlight-dev/hyperlight-unikraft/helloworld-c-kernel
just rootfs     # docker build + extract CPIO
just run
```

### What each recipe does

| Recipe | Linux | Windows |
|--------|-------|---------|
| `just build` | `kraft-hyperlight build` | `docker pull` the pre-built kernel from GHCR |
| `just rootfs` | `docker build --target cpio` + extract the CPIO | same |
| `just run` | `hyperlight-unikraft <kernel> --initrd ...` | same |
| `just clean` | remove `.unikraft/` and the CPIO | same |

## Examples

| Example | Binary | Notes |
|---------|--------|-------|
| `helloworld-c` | Static PIE C binary | Compiled with `musl-gcc` |
| `rust` | Static PIE Rust binary | Compiled with `rustc --target x86_64-unknown-linux-musl` |
| `python` | CPython 3.12 | Rootfs from Docker, script passed via cmdline |
| `go` | Static PIE Go binary | Compiled with musl via Docker for CGO support |
| `nodejs` | Node.js 21 | Rootfs from Alpine, script passed via cmdline |
| `hostfs-posix-c` | C + unmodified POSIX | `open`/`read`/`write`/`mkdir` against `/host`, forwarded by `lib/hostfs` |
| `hostfs-posix-py` | Python + stdlib | Same as `hostfs-posix-c` using `open()`/`os.mkdir`/`os.stat` |

### Host filesystem sandbox

`--mount HOST_DIR[:GUEST_PATH]` preopens a host directory for the guest:

```bash
# Default: guest-visible at /host
hyperlight-unikraft kernel --initrd app.cpio --mount ./work

# Custom guest mount point
hyperlight-unikraft kernel --initrd app.cpio --mount ./work:/data
```

`lib/hostfs` in the guest auto-mounts `HOST_DIR` at `GUEST_PATH` (default
`/host`); unmodified POSIX calls (`open`, `read`, `write`, `stat`,
`mkdir`, `truncate`, …) are forwarded by the VFS driver to the host's
`FsSandbox` tool handlers. The guest mount point is advertised runtime
via an `HLHSMNT` TLV in init_data, so one kernel build can serve
different mount points. Reserved kernel dirs (`/`, `/bin`, `/dev`,
`/proc`, `/sys`, `/usr`) are refused to avoid shadowing the initrd.

Every path the guest sends is resolved relative to `HOST_DIR` and any
escape (via `..` or symlinks) is rejected host-side.

Known limitation: `opendir`/`readdir` don't work yet (see
[lib/hostfs/README.md](https://github.com/unikraft/unikraft/blob/plat-hyperlight/lib/hostfs/README.md)). Stat and enumerate known paths instead.

### Running ad-hoc code (no initrd rebuild)

`--exec CODE` / `-e CODE` feeds a snippet to the guest interpreter as
`-c CODE`. The host handles all the argparse-escape quoting internally,
so you can pass arbitrary whitespace, quotes, and newlines without
wrapping:

```bash
hyperlight-unikraft python-kernel --initrd python.cpio --memory 96Mi \
    --exec 'for i in range(3): print(i * i)'
```

Works for any interpreter that treats `-c` as "run the next arg as
code" — CPython, `sh`, etc. `node -e` works identically with `-e`.

`examples/hostfs-posix-py` wraps it in two Justfile recipes:

```bash
just exec "print('hi'); print(2 + 2)"
just run-file path/to/myscript.py   # file's contents → --exec
```

No `--mount` involved. No `/host/…` path contract. The host just passes
argv.

#### Passing extra script arguments

`--exec` and positional `-- args` are mutually exclusive (clap enforces
it at parse time) — they both populate argv, so letting both through
would silently lose one. If you need inline code *plus* extra `sys.argv`
arguments, drop back to the raw `--` form and do the quoting yourself:

```bash
hyperlight-unikraft python-kernel --initrd python.cpio --memory 96Mi \
    -- -c '"import sys; print(sys.argv[1:])"' alpha beta gamma
# => ['alpha', 'beta', 'gamma']
```

The inner `-c` payload is wrapped in outer double-quotes so `uk_argparse`
preserves whitespace, with internal quotes backslash-escaped. Anything
after is plain argv.

### Running with Arguments

For interpreted languages, pass the script path after `--`:

```bash
# Python
hyperlight-unikraft kernel --initrd python.cpio --memory 256Mi -- /script.py arg1 arg2

# Node.js
hyperlight-unikraft kernel --initrd node.cpio --memory 512Mi -- /app/server.js --port 8080
```

## CLI Options

```
hyperlight-unikraft [OPTIONS] <KERNEL> [-- <APP_ARGS>...]

Arguments:
  <KERNEL>              Path to the Unikraft kernel binary
  <APP_ARGS>...         Arguments passed to the application (after --)

Options:
      --initrd <CPIO>          Path to initrd/rootfs CPIO archive
  -m, --memory <MEMORY>        Memory allocation (e.g., 256Mi, 512Mi, 1Gi) [default: 512Mi]
      --stack <STACK>          Stack size (e.g., 8Mi) [default: 8Mi]
  -q, --quiet                  Quiet mode — suppress host-side status messages
      --enable-tools           Enable tool dispatch via __dispatch host function
      --mount <HOST[:GUEST]>   Preopen a host directory for the guest's sandboxed filesystem
                               (repeatable; default guest path: /host)
      --net                    Enable guest networking (off by default)
      --net-allow <HOST_OR_IP> Restrict networking to listed hosts/IPs (implies --net; repeatable;
                               conflicts with --net-block)
      --net-block <HOST_OR_IP> Block listed hosts/IPs, allow everything else (implies --net;
                               repeatable; conflicts with --net-allow)
      --port <PORT>            Allow guest to bind (listen) on this port (implies --net; repeatable)
      --repeat <N>             Run the application N additional times via snapshot/restore [default: 0]
  -e, --exec <CODE>            Inline code snippet — interpreter invoked with -c <CODE>
                               (conflicts with positional -- <APP_ARGS>)
  -h, --help                   Print help
  -V, --version                Print version
```

## Project Structure

```
hyperlight-unikraft/
├── host/                    # Rust host (hyperlight-unikraft CLI + pyhl)
├── examples/                # Ready-to-use kraft configs
│   ├── helloworld-c/       # C (musl-gcc)
│   ├── rust/               # Rust (musl)
│   ├── python/             # CPython 3.12
│   ├── python-agent-driver/# Python with pre-loaded packages (pyhl)
│   ├── go/                 # Go (musl via Docker)
│   ├── nodejs/             # Node.js 21
│   ├── hostfs-posix-c/     # Host filesystem sandbox (C)
│   ├── hostfs-posix-py/    # Host filesystem sandbox (Python)
│   ├── dotnet/             # .NET
│   ├── powershell/         # PowerShell
│   ├── shell/              # Shell
│   └── ...
├── runtimes/                # Dockerfiles for runtime images
└── demos/                   # Demo materials
```

## Dependencies

This project builds on the following upstream repositories:

| Repository | Description |
|------------|-------------|
| [hyperlight-dev/hyperlight](https://github.com/hyperlight-dev/hyperlight) | Hyperlight VMM |
| [unikraft/unikraft](https://github.com/unikraft/unikraft) | Unikraft core with Hyperlight platform support |
| [unikraft/app-elfloader](https://github.com/unikraft/app-elfloader) | ELF loader application |
| [unikraft/kraftkit](https://github.com/unikraft/kraftkit) | Kraft build tool with Hyperlight machine driver |

## Join our Community

Please review the [CONTRIBUTING.md](./CONTRIBUTING.md) file for more information on how to contribute to
Hyperlight.

This project holds fortnightly community meetings to discuss the project's progress, roadmap, and any other topics of interest. The meetings are open to everyone, and we encourage you to join us.

- **When**: Alternating Wednesdays at 09:00 and 07:00 (PST/PDT) [Convert to your local time](https://dateful.com/convert/pst-pdt-pacific-time?t=09)
- **Where**: Zoom! - Agenda and information on how to join can be found in the [Hyperlight Community Meeting Notes](https://hackmd.io/blCrncfOSEuqSbRVT9KYkg#Agenda). Please log into hackmd to edit!

## Chat with us on the CNCF Slack

The Hyperlight project Slack is hosted in the CNCF Slack #hyperlight. To join the Slack, [join the CNCF Slack](https://www.cncf.io/membership-faq/#how-do-i-join-cncfs-slack), and join the #hyperlight channel.

## More Information

For more information, please refer to the main [Hyperlight project](https://github.com/hyperlight-dev/hyperlight).

## Code of Conduct

See the [CNCF Code of Conduct](https://github.com/cncf/foundation/blob/main/code-of-conduct.md).
