# Projects and the manifest

A project is a directory with a `hluk.toml` in it. `hluk init` writes one from a template, `hluk run` (with no `--initrd`) runs what it describes, `hluk build` compiles it or builds its rootfs, and `hluk pull` refreshes the rootfs it names. The manifest is `hluk run`'s flags written down: every key is a flag, a flag given on the command line wins (`--mount` and `--env` add to the manifest's; any network flag replaces its `[run.net]` table as a whole), and a guest can be run with no manifest at all, from a CPIO (`hluk run --initrd rootfs.cpio main.py`) or a published image by name (`hluk run --runtime python main.py`, pulled into the cache when missing).

## The workflow

```bash
hluk templates                          # what you can start from
hluk init hello --template python       # writes hello/hluk.toml, hello/main.py; pulls the python rootfs
cd hello
hluk run                                 # boots the guest and runs main.py
hluk run                                 # restores the warm snapshot the first run saved: milliseconds
hluk cache                               # what has been pulled and snapshotted, and where
```

Without a project: `hluk run --runtime python main.py` runs the published python image, warm by default (`--cold` boots fresh), and `hluk run --initrd build-elfloader/python-rootfs.cpio main.py` runs a CPIO you built, cold unless `--warm`. `--runtime` also takes a full image reference. `hluk snapshot save --runtime python --warm-exec "import json" -o warm` saves such a guest by hand, for `hluk snapshot run`, `hluk bench` or an embedder to restore.

`hluk init` with no arguments asks for the template and a project name. A template is a starter for one runtime: the interpreted ones (`python`, `python-shell`, `agent`, `node`, `quickjs`, `bash`, `dotnet`, `powershell`) run a script from the host, and `bash-repl` runs a read-eval loop on `hluk run`'s stdin; the compiled ones (`go`, `rust`, `c`, `dotnet-aot`, and `wasmtime`, a WASI component) carry a `[build]` command and mount its output into the guest; the `http-*` ones serve on port 8080 from a rootfs their Dockerfile extends. `hluk templates` lists them with the [support tier](guest-support-tiers.md) of their runtime, and `--template` also takes your own, from a directory or GitHub ([templates.md](templates.md)).

Where the rootfs comes from decides the next step:

| `[rootfs]` | How it gets there | Then |
|---|---|---|
| `image` | Pulled from the registry into the cache, by `init` and by the first `run`; `hluk pull` refreshes it | `hluk run` |
| `dockerfile` | `hluk build` runs Docker and exports the image as `.hluk/rootfs.cpio` | `hluk build`, then `hluk run` (or `hluk run --build`) |
| `path` | A CPIO you built, e.g. with `just build-rootfs` | `hluk run` |

## The published images and the cache

Every release publishes each runtime's rootfs to `ghcr.io/hyperlight-dev/hyperlight-unikraft/<runtime>` in two forms (see [RELEASE.md](RELEASE.md)): `:initrd-v<version>` is the runnable CPIO, `:v<version>` the filesystem image a Dockerfile builds `FROM`. A rootfs is tied to the release of `hluk` it was built with, because the two share the driver protocol, so `hluk init` pins both tags to its own version: a project made by `hluk` 0.14.1 names `python:initrd-v0.14.1` and `python-shell:v0.14.1`, and keeps working when a newer `hluk` is installed until you choose to move it. A build of an unreleased `hluk` has no published rootfs; the error says so and points at `just build-rootfs` and `[rootfs] path`.

`hluk` pulls the image itself: the anonymous token, the manifest and the one layer, checked against its digest and unpacked, with no Docker on the host. So a project runs wherever `hluk` does, Windows and macOS included. Pulled images land in a cache shared by every project, `$HLUK_CACHE_DIR` if set, else `$XDG_CACHE_HOME/hluk` or `~/.cache/hluk` on Unix and `%LOCALAPPDATA%\hluk\cache` on Windows, laid out by reference: `rootfs/ghcr.io/hyperlight-dev/hyperlight-unikraft/python/initrd-v0.14.1.cpio`. `hluk pull` asks the registry again and re-downloads a tag whose layer changed; `hluk run` takes what is cached.

`--registry` (or `HLUK_REGISTRY`) points `init` at another registry laid out the same way, such as a fork's; the image references it writes are complete, so `run` and `pull` need no flag. `--image-version` (or `HLUK_IMAGE_VERSION`) pins another release's images instead of this build's, which is how a build between releases, with no published images of its own, starts a project: `hluk init --image-version 0.14.1` works for as long as the driver protocol has not changed since that release.

## Extending a rootfs

The published images are minimal on purpose: the interpreter, its standard library, the driver. A project that needs packages says so in a Dockerfile that starts `FROM` the published base and adds them; `hluk build` builds it with Docker, exports the image's filesystem and converts it to the CPIO the kernel takes, with the `/etc` files `docker export` empties (`hosts`, `nsswitch.conf`, `resolv.conf`) put back. The `http-python` template is the pattern for pip:

```dockerfile
FROM ghcr.io/hyperlight-dev/hyperlight-unikraft/python-shell:v0.14.1 AS base

# pip runs in a real Linux stage; the rootfs is FROM scratch and has no shell.
FROM python:3.12-slim-bookworm AS installer
COPY --from=base / /rootfs/
COPY requirements.txt /tmp/requirements.txt
RUN pip install --no-cache-dir -r /tmp/requirements.txt \
        --target /rootfs/usr/local/lib/python3.12/site-packages

FROM scratch
COPY --from=installer /rootfs/ /
```

`http-node` does the same with `npm install` into `/node_modules`, and `http-dotnet` publishes an ASP.NET Core app with the .NET SDK onto the `dotnet-aot` base. Anything a Dockerfile can put in a filesystem can go in: a compiled program at `/app/server` run with `exec = "/app/server"`, data files, certificates. What must hold is what the [drivers](driver.md) need: the binaries are position-independent (PIE) and do not `fork()`, and the driver the base image carries stays where it is.

## The warm snapshot

With `warm = true` (or `--warm`, or `--runtime`), the first `hluk run` boots the guest, and once the runtime is up and the driver parked (see [execution.md](execution.md)), snapshots it before running the workload. Every later `hluk run` restores that instead of booting: the interpreter is initialised, the `agent` image's numpy and pandas are imported, .NET's Roslyn is loaded. A restore is milliseconds where the boot was hundreds of them or seconds. `--warm` and `--cold` override the manifest for one run.

The snapshot lives in the cache, under `snapshots/`, not in the project: it is named by what it depends on (below), so every project on the same rootfs with the same memory, entry point and `warm_exec` shares one, and the `agent` image's gigabyte is stored once rather than per directory. `hluk cache ls` lists them with what they were taken from (`--snapshots` or `--rootfs` for one kind, paths relative to the cache directory its first line names), `hluk cache clean --snapshots` removes them. Different settings for the same rootfs (memory, entry point, `warm_exec`) are different guests and each keeps a snapshot; a rebuilt rootfs, the same path with a new file, replaces every snapshot of the old one.

What the snapshot has loaded is the project's to choose. The drivers do their part at boot: the python-shell and agent images import numpy, pandas, scipy, scikit-learn and matplotlib when they are installed. `warm_exec` adds the rest: code the runtime driver runs once, after boot and before the snapshot, exactly as a call would, so whatever it leaves behind (modules in the interpreter's cache, code the JIT compiled) is in the snapshot. The `http-python` template sets `warm_exec = "import flask"`, `http-node` `warm_exec = "require('express')"`; a project that pip-installs pandas and sets `warm_exec = "import flask, pandas"` restores in a few milliseconds where importing the two natively takes hundreds.

The snapshot is taken again when it would no longer match: `hluk` stamps it with what it depends on, the rootfs file (path, size, mtime), `scratch_mb`, `entry`, `warm_exec` and the build's [snapshot key](execution.md#snapshot-and-restore), and a run whose stamp differs boots fresh and saves anew. Mounts, the network policy and the environment are supplied on restore, so editing them in the manifest does not cost a boot. A program the exec driver runs from a mount is checked before the boot: one that is not position-independent is refused with the build flag that makes it one.

## Reference

```toml
manifest_version = 1                # the format; this hluk reads 1

[app]
name = "hello"                      # letters, digits, `-`, `_`

[rootfs]                            # exactly one of:
image = "ghcr.io/hyperlight-dev/hyperlight-unikraft/python:initrd-v0.14.1"
# dockerfile = "Dockerfile"         # built by `hluk build` into .hluk/rootfs.cpio
# path = "build-elfloader/python-rootfs.cpio"

[build]                             # optional; run by `hluk build` in the project directory
command = "cargo build --release"   # through `sh -c` (Unix) or `cmd /C` (Windows)

[run]
script = "main.py"                  # at most one of: script (a host file the driver runs),
# exec = "/mnt/app/app"             #   exec (code, or a command line for the exec driver),
# guest_exec = "/app/hello.py"      #   guest_exec (a file already in the rootfs, plus args);
# entry = "/app/server"             # the guest's entry point when the rootfs has no driver
# scratch_mb = 512                  # guest memory in MiB; default: see Memory below
mounts = ["./data:/mnt/data:ro"]    # HOST:GUEST[:ro]; a relative host path is relative to the manifest
env = { GREETING = "hi" }
warm = true                         # snapshot the booted guest; restore it on later runs
warm_exec = "import flask"          # run by the driver before that snapshot, so what it loads is in it
# resolv_conf = "resolv.conf"       # installed as the guest's /etc/resolv.conf

[run.net]                           # omit the table for no networking at all; then at most one of:
all = true                          #   every destination (AllowAll)
# allow = ["pypi.org"]              #   an allow list (hosts or addresses)
# block = ["169.254.169.254"]       #   a block list
ports = [8080, "9000-9010"]         # ports the guest may listen on: a port, "LOW-HIGH" or "all"
```

### Memory

A guest's memory is `scratch_mb` when the manifest sets it. Otherwise it is the size the runtime's image is tested and benchmarked with, which `hluk` carries as a table (the justfile's `scratch_*`): c, rust and quickjs 64 MiB, go 128, bash, python, python-shell, dotnet-aot and wasmtime 256, node 512, dotnet-jit 768, powershell 1024, agent 1536. The runtime is read from the image's name when `[rootfs] image` is a published one (`…/agent:initrd-v0.14.1`), and from the driver found in the rootfs otherwise (a Dockerfile rootfs `FROM` the python-shell base carries `hl_pywarmdriver`, so it gets python-shell's 256); an image with no driver `hluk` knows gets 256. `hluk run` without `--scratch-mb` does the same by driver, and so does a `SandboxBuilder` that never called `scratch_mb`; `default_scratch_mb`, `runtime_scratch_mb` and `RUNTIME_SCRATCH_MB` are the library's. The rootfs is unpacked into that memory, so a rootfs that packages have grown past it fails to unpack: `hluk build` and `hluk run` say so, with a size to set, whenever the rootfs takes more than half of the memory it would get.

Every `[run]` key is a `hluk run` flag of the same name (`--mount`, `--env`, `--net`, `--net-allow`, `--net-block`, `--port`, `--resolv-conf`, `--scratch-mb`, `--exec`, `--guest-exec`, `--entry`), with the same meaning; [fs.md](fs.md) and [net.md](net.md) describe what they do. Unknown keys are errors, so a typo is reported rather than ignored.

`hluk run` exits with the guest's status when the workload is what failed.

## Templates

The templates live in [`templates/`](../templates/), one directory each, and are compiled into `hluk` so `init` works offline and the templates match the binary's release. `template.toml` names the template's runtime, tier and description; every other file is copied into the project with `{{name}}`, `{{version}}`, `{{registry}}`, `{{image}}` (the `:initrd-v<version>` image) and `{{base}}` (the `:v<version>` image) filled in, and a `.tmpl` suffix dropped (`Cargo.toml.tmpl` becomes `Cargo.toml`; a real one would make `cargo package` take the directory for a nested crate and leave it out). Adding a template is adding a directory; the unit tests check that each one's manifest validates. `--template` also takes a template directory on disk or on GitHub (`github.com/OWNER/REPO[/PATH][@REF]`); [templates.md](templates.md) is the guide to writing one.
