# Agent Examples

The agent rootfs ships Python 3.12, BusyBox (hush), data-science packages, and SSL support for runtime `pip install` via `--net`.

## Prerequisites

```bash
just build-rootfs agent         # full agent (~400MB, numpy/pandas/scipy/sklearn/matplotlib/seaborn)
just build-rootfs python-shell    # slim agent (~50MB, no data-science packages)
```

## Run examples

```bash
# Hello — prints Python version and available packages
just run agent examples/agent/hello.py

# Data science — numpy, pandas, scipy, sklearn
just run agent examples/agent/data_science.py

# Shell commands via subprocess (BusyBox hush)
just run agent examples/agent/shell_commands.py

# pip install at runtime (requires --net)
just run agent --net examples/agent/pip_install.py
```

Slim examples (same scripts, smaller rootfs):

```bash
just run python-shell examples/agent/hello.py
just run python-shell examples/agent/shell_commands.py
```

Or with `hluk` directly:

```bash
hluk run --initrd build-elfloader/agent-rootfs.cpio \
    --scratch-mb 1536 examples/agent/hello.py

hluk run --initrd build-elfloader/python-shell-rootfs.cpio \
    --scratch-mb 256 examples/agent/hello.py
```

## Snapshot round-trip

```bash
# Save
hluk snapshot save --initrd build-elfloader/agent-rootfs.cpio \
    --scratch-mb 1536 --output ../../.snapshots/agent

# Run from snapshot (file)
hluk snapshot run ../../.snapshots/agent examples/agent/hello.py

# Run from snapshot (inline)
hluk snapshot run ../../.snapshots/agent --exec "import numpy; print(numpy.__version__)"
```

## pip install over `--net`

The agent rootfs includes SSL libraries and CA certificates, so pip can connect to PyPI over HTTPS when the guest has network access.

```bash
just run agent --net examples/agent/pip_install.py
```

This is safe because Hyperlight uses hardware VM isolation — the guest runs in its own hardware-enforced address space, not a shared-kernel sandbox.

## Custom rootfs

The `custom/` directory shows how to extend `python-shell` with additional packages. It installs Pydantic and PyYAML via pip:

```bash
# Build the custom rootfs example
just build-rootfs agent-custom examples/agent/custom/Dockerfile

# Run the custom example
just run agent-custom examples/agent/custom/custom_packages.py
```

To build your own custom rootfs, create a Dockerfile and use:

```bash
just build-rootfs my-custom path/to/my/Dockerfile
just run my-custom my_script.py
```

A typical Dockerfile extends `python-shell`:

```dockerfile
FROM hluk-python-shell-rootfs:latest AS base

FROM python:3.12-slim-bookworm AS installer
COPY --from=base / /rootfs/
RUN pip install --no-cache-dir --target /rootfs/usr/local/lib/python3.12/site-packages \
        your-package-here

FROM scratch
COPY --from=installer /rootfs/ /
```
