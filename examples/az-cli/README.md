# az-cli shell

A BusyBox hush shell in a Hyperlight micro-VM, with the Azure CLI, Python 3.12 and the BusyBox applets (`ls`, `grep`, `wget`, …). Network access is limited to an allow-list: Entra ID sign-in, Azure Resource Manager and Microsoft Graph. Proof of concept.

## Try it

Needs Linux with KVM, Docker, [just](https://github.com/casey/just) and a Rust toolchain. Builds take a few minutes; the rootfs is 665 MiB. Run everything from the repository root.

```bash
git clone -b az-cli-shell https://github.com/hyperlight-dev/hyperlight-unikraft
cd hyperlight-unikraft

cargo build --release
just build-rootfs bash
just build-rootfs python-shell
just build-rootfs az-cli examples/az-cli/Dockerfile

examples/az-cli/run.sh                    # interactive shell
examples/az-cli/run.sh 'az group list'    # one command
```

The first run boots the guest and saves a warm snapshot (~10 s, once per image build); later runs start from it.

```text
az-shell:/root$ az login --use-device-code --tenant <tenant-id>
az-shell:/root$ az group list -o table
az-shell:/root$ exit
```

Sign in with `--use-device-code`: the guest has no browser. Pass `--tenant` when a tenant requires MFA for Azure Resource Manager; without it, az signs in to the home tenant and then fails to reach the others (`AADSTS50076`).

## In a browser

```bash
pip install websockets
python3 examples/az-cli/web/server.py     # http://localhost:8080
```

Each browser tab gets its own micro-VM and its own `~/.azure`, removed when the tab closes: browser (xterm.js) → websocket → `server.py` → `run.sh` under a pseudo-terminal → micro-VM. The micro-VM has no inbound port; the server owns the socket. The server listens on 127.0.0.1 only.

## The `hluk run` command

`run.sh` runs this:

```bash
target/release/hluk run \
  --initrd build-elfloader/az-cli-rootfs.cpio \
  --entry /usr/local/bin/hl_bashdriver \
  --scratch-mb 2048 --warm \
  --mount ~/.hluk-az:/root/.azure \
  --net-allow login.microsoftonline.com \
  --net-allow management.azure.com \
  --net-allow graph.microsoft.com \
  --env HOME=/root \
  --env AZURE_CORE_COLLECT_TELEMETRY=false \
  examples/az-cli/shell.sh
```

| Option | What it does |
| --- | --- |
| `--initrd` | The guest's root filesystem, unpacked into guest memory at boot. |
| `--entry` | Starts the hush shell driver. Without it, hluk picks the Python driver for this image. |
| `--scratch-mb 2048` | Guest memory in MiB: the unpacked rootfs plus what az needs. |
| `--warm` | Restores the cached snapshot of the booted guest; saves one on the first run. |
| `--mount` | A host directory at `/root/.azure`, where az keeps its config and tokens, so a sign-in outlives the guest. Leave it out for a clean guest every time. |
| `--net-allow` | Networking limited to these names. Everything else, loopback and the cloud metadata address are refused. Exact names only. |
| `--env` | Guest environment variables. Telemetry is off because az starts an uploader after each command, and in one guest they pile up. |
| `shell.sh` | The prompt loop: reads a line, runs it. |

## Numbers

x86_64, KVM, signed out (az exits before any network call), 5 runs after a warm-up.

| | hluk | Host az |
| --- | --- | --- |
| `hluk run` to the first prompt, from snapshot | ~20 ms | |
| Same, without a snapshot | 2.7 s | |
| `az version`, one command per `hluk run` | 1.2 s | 0.20 s |
| `az group list`, one command per `hluk run` | 2.1 s | 0.52 s |

## Limits

- Network: only the three hosts above. Storage data plane (`*.blob.core.windows.net`), Key Vault (`*.vault.azure.net`), `az extension add`, `az upgrade` and `az bicep install` fail with `bad address` until their hosts are added (`--net-allow`, or `--net` for everything).
- No `ssh`, `docker`, `kubectl` or `jq`: `az ssh`, `az acr login` and `az aks` with kubectl do not work. Use `--query` instead of `jq`.
- Line-at-a-time input: no tab completion, history or `az interactive`. The guest always reports a 24×80 terminal.
- Files outside `/root/.azure` are in guest memory and go when the guest exits; `--mount` moves files in and out.

## Files

| File | What it is |
| --- | --- |
| `Dockerfile` | python-shell rootfs + `azure-cli` (precompiled bytecode, no `psutil`) + the bash runtime's hush driver. |
| `az.c` | `/bin/az`, an ELF launcher for `python3 -m azure.cli`: the guest runs ELF executables only, not `#!` scripts. |
| `shell.sh` | Guest side: the prompt loop. |
| `run.sh` | Host side: the `hluk run` command above. `AZ_HOME` and `HLUK` override the defaults. |
| `web/server.py`, `web/index.html` | The browser terminal: a websocket per tab, relayed to `run.sh`. |
