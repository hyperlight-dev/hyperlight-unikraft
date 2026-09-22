# urunc packaging for a hyperlight-unikraft guest

[urunc](https://github.com/urunc-dev/urunc) is a container runtime for unikernels. Its `hyperlight-unikraft` monitor runs `hluk` guests through containerd, so `docker run`, `nerdctl run` and Kubernetes boot the micro-VM from an ordinary OCI image. This demo builds such an image; `just publish-urunc` at the repository root publishes it as `ghcr.io/hyperlight-dev/hyperlight-unikraft/hello-urunc`.

## What the image contains

A `FROM scratch` OCI image carrying the kernel, a rootfs (the stock `python` rootfs with the demo workload at `/app/hello.py`), the command to run, and urunc's annotations:

| Annotation | Value | Meaning |
|---|---|---|
| `com.urunc.unikernel.binary` | `/unikernel/kernel` | the Unikraft elfloader kernel |
| `com.urunc.unikernel.initrd` | `/unikernel/initrd.cpio` | the rootfs CPIO |
| `com.urunc.unikernel.unikernelType` | `unikraft` | unikernel family |
| `com.urunc.unikernel.hypervisor` | `hyperlight-unikraft` | urunc's monitor for hluk |

`CMD ["/app/hello.py"]` is the guest command, as any container image declares what it runs. The bunny frontend also writes the annotations into a `urunc.json` inside the image, so engines that do not forward OCI annotations (Docker) still find them.

## Files

| File | Purpose |
|---|---|
| `Containerfile` | bunny-syntax package: copies the kernel + rootfs CPIO, sets the annotations and the `CMD`. |
| `rootfs.Dockerfile` | The stock python rootfs + the demo workload at `/app/hello.py`. |
| `hello.py` | The demo workload (prints a line). |
| `Justfile` | `stage` / `image` / `inspect` / `clean`. |

## Build the image

```bash
cd demos/urunc
just image     # stages the kernel + python rootfs, builds the OCI image
just inspect   # shows the com.urunc.unikernel.* annotations
```

## Run it

```bash
docker run --rm --runtime io.containerd.urunc.v2 hluk/hello-urunc:demo
# or: sudo nerdctl run --rm --runtime io.containerd.urunc.v2 hluk/hello-urunc:demo
#   -> hello from Python 3.12 on Unikraft in a Hyperlight micro-VM, via urunc
```

Prerequisites: Linux with KVM, containerd with `containerd-shim-urunc-v2` and a urunc whose `hyperlight-unikraft` monitor drives `hluk`, and `hluk` on `$PATH`.

## How the workload runs

The kernel boots the guest, which starts the python driver (`hl_pydriver`); the driver waits for a command. urunc takes the container's command, the image's `CMD` or whatever follows the image name on the command line, and hands it to `hluk run --guest-exec`: a file inside the initrd plus its arguments, which the driver runs. So the app lives in the initrd and the image says what to run, exactly like any other urunc unikernel. The same thing locally:

```bash
just stage   # builds build-elfloader/urunc-hello-rootfs.cpio
hluk run --initrd ../../build-elfloader/urunc-hello-rootfs.cpio --scratch-mb 256 \
    --guest-exec "/app/hello.py"
```

## Notes

- The kernel is this project's Unikraft elfloader, the same one `hluk` embeds; the rootfs is the stock `python` rootfs. Both are tied to the `hluk` release they were built with.
- `docker build` uses bunny as a buildkit frontend via the `#syntax=` line; no separate install is needed.
