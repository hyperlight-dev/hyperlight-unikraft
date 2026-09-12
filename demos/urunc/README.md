# urunc packaging for a hyperlight-unikraft guest

[urunc](https://github.com/urunc-dev/urunc) is a container runtime for unikernels. It has a native VMM for **hyperlight-unikraft** guests, run through containerd, so `docker run` / `nerdctl run` boots the micro-VM. (The `com.urunc.unikernel.hypervisor` value is `hyperlight-unikraft`, matching the upstream packaging; an older urunc build spelled the VMM `hyperlight`.)

That VMM support targets the `hyperlight-unikraft` monitor and is being **upstreamed** from this project (initial support: urunc issue #140; follow-up: [urunc-dev/urunc#632](https://github.com/urunc-dev/urunc/pull/632)). Until a urunc build with it is installed on your host, this demo **prepares** the OCI image so it is ready to run the moment that support lands.

## What the image contains

A `FROM scratch` OCI image carrying the kernel and a rootfs (the stock `python` rootfs with a demo workload baked in at `/entrypoint.py`) plus urunc's annotations:

| Annotation | Value | Meaning |
|---|---|---|
| `com.urunc.unikernel.binary` | `/unikernel/kernel` | the Unikraft elfloader kernel |
| `com.urunc.unikernel.initrd` | `/unikernel/initrd.cpio` | the rootfs CPIO |
| `com.urunc.unikernel.unikernelType` | `unikraft` | unikernel family |
| `com.urunc.unikernel.hypervisor` | `hyperlight-unikraft` | urunc's hyperlight VMM |

The bunny frontend also writes these values into a `urunc.json` inside the image, so runtimes (Docker) that don't forward OCI annotations still find them.

## Files

| File | Purpose |
|---|---|
| `Containerfile` | bunny-syntax package: copies the kernel + rootfs CPIO and sets the annotations. |
| `rootfs.Dockerfile` | The stock python rootfs + the demo workload at `/entrypoint.py`. |
| `hello.py` | The demo workload (prints a line). Baked in as `/entrypoint.py`. |
| `Justfile` | `stage` / `image` / `inspect` / `clean`. |

## Prepare the image

```bash
cd demos/urunc
just image     # stages the kernel + python rootfs, builds the OCI image
just inspect   # shows the com.urunc.unikernel.* annotations
```

## Run it (once urunc's hyperlight VMM is on your host)

```bash
docker run --rm --runtime io.containerd.urunc.v2 hluk/hello-urunc:demo
# or: sudo nerdctl run --rm --runtime io.containerd.urunc.v2 hluk/hello-urunc:demo
```

Prerequisites for running (not for preparing the image): Linux with KVM, containerd + `containerd-shim-urunc-v2` + a `urunc` build that includes the hyperlight VMM, and the `hyperlight-unikraft` monitor on `$PATH`.

## How a Python workload runs

The kernel boots the guest, which runs the python driver (`hl_pydriver`); the driver waits for a command. urunc's monitor supplies it two ways, both of which end at the same `hluk` guest-command path:

- **No cmdline** → the monitor runs the guest with an empty command, so the driver runs the rootfs's conventional `/entrypoint.py` (the baked `hello.py`).
- **A cmdline** (`com.urunc.unikernel.cmdline` / container args) → the monitor passes it as the guest command, e.g. `cmdline = "/app/report.py --fast"` runs that script with those args.

So the app lives in the initrd, and the image (its `/entrypoint.py` or its `cmdline`) says what to run — exactly like any other urunc unikernel. You can try the same thing locally today:

```bash
just stage   # builds build-elfloader/urunc-hello-rootfs.cpio
hluk run --initrd ../../build-elfloader/urunc-hello-rootfs.cpio --scratch-mb 256
#   -> hello from Python 3.12 on Unikraft, via urunc   (empty command → /entrypoint.py)
hluk run --initrd ../../build-elfloader/urunc-hello-rootfs.cpio --scratch-mb 256 \
    --guest-exec "/entrypoint.py"                       # the cmdline path
```

## Notes

- The kernel here is this project's Unikraft elfloader; the rootfs is the stock `python` rootfs. `hluk` consumes a **guest command** the same way every urunc VMM does: `hluk run --initrd <cpio> --guest-exec "<command>"` runs a file baked into the initrd with args, which maps directly to urunc's `Command` (`com.urunc.unikernel.cmdline` / the container args). With an empty command the guest runs its conventional entrypoint (`/entrypoint.py`, `/entrypoint`, …). See `examples/autonomous/`. What remains for end-to-end urunc is the runtime-side adaptation: the hyperlight `BuildExecCmd` emitting `hluk run --initrd … --scratch-mb … --guest-exec "<Command>"` (mapping `--memory`→`--scratch-mb`, ignoring the embedded kernel). The packaging here (kernel + rootfs + annotations) is the stable, reusable artifact.
- `docker build` uses bunny as a buildkit frontend via the `#syntax=` line; no separate install is needed.
