# Autonomous run

An appliance whose **entire workload lives in the initrd** and runs with **no host input**. `hluk run --initrd <rootfs>` (no `--exec`, no script) boots the guest and runs its conventional entrypoint. Every driver supports this, each with its language's natural entrypoint:

| Runtime | Driver | Entrypoint |
|---|---|---|
| Python / agent | `hl_pydriver` / `hl_pywarmdriver` | `/entrypoint.py` |
| Node | `hl_nodedriver` | `/entrypoint.js` |
| Bash | `hl_bashdriver` | `/entrypoint.sh` |
| .NET (JIT) | `hl_dotnetdriver` | `/entrypoint.cs` |
| PowerShell | `hl_pwshdriver` | `/entrypoint.ps1` |
| C / Rust / Go / .NET AOT | `hl_execdriver` | `/entrypoint` (an executable) |

This is the deployment model a container runtime like [urunc](../../demos/urunc/) uses: the image carries the kernel + rootfs, and the guest runs itself — no mounts, no network, nothing passed from the host.

## Run a specific file, with args

The general form is `--guest-exec "<path> [args...]"`, which runs a file that already lives in the guest fs with that argv — this is the mode a container runtime (urunc) drives, and it takes arguments:

```bash
hluk run --initrd app.cpio --guest-exec "/app/server --port 8080"
```

An empty command (i.e. no `--guest-exec` and no script) is the autonomous case above: the driver runs its conventional entrypoint from the table.

## How it works

`hluk` sends the command to the guest under a distinct dispatch name, `GuestExec` (as opposed to `Exec` for inline code — see `src/lib.rs` `Exec::Guest` and `drivers/hl_fc.h` `fc_name_is`). Each driver runs the named guest file with argv: the exec driver `execv`s it; the interpreters run it as a script with `sys.argv`/`process.argv` set (see the per-driver dispatch in `drivers/`). An empty command maps to the driver's conventional entrypoint. Because it dispatches at the same point as any other run, it behaves identically on a fresh boot or a restored snapshot.

## Files

| File | Purpose |
|---|---|
| `entrypoint.py` | The appliance workload, baked into the rootfs at `/entrypoint.py`. |
| `Dockerfile` | Stock `python` rootfs + the entrypoint. |

## Run

```bash
just build-rootfs autonomous examples/autonomous/Dockerfile

# No workload argument — the guest runs its baked-in /entrypoint.py:
hluk run --initrd build-elfloader/autonomous-rootfs.cpio --scratch-mb 256
```

Output:

```
== autonomous appliance ==
python 3.12.14 on Unikraft
primes below 100000: 9592 (largest 99991) in 5.2 ms
done
```

## Fast start with a snapshot

Snapshot the booted-but-not-yet-run guest once, then restore-and-run repeatedly. The restore is milliseconds; the autonomous entrypoint runs on restore just as it does on a fresh boot:

```bash
hluk snapshot save --initrd build-elfloader/autonomous-rootfs.cpio \
    --scratch-mb 256 --output .snapshots/autonomous
hluk snapshot run .snapshots/autonomous          # restore + run /entrypoint.py
```

## Any runtime

Bake the entrypoint at the path for your runtime (see the table above) and `hluk run --initrd <rootfs>` runs it — Python, Node, Bash, .NET, PowerShell, or a compiled executable. If a rootfs has no entrypoint, the guest prints a short "nothing to run" notice and exits cleanly.

## Relation to urunc

This is exactly what urunc's hyperlight VMM invokes: the monitor is given only a kernel and an initrd and boots the guest, which runs its own entrypoint. See `demos/urunc/` for packaging this shape as an OCI image. The remaining piece for end-to-end urunc is the runtime-side CLI adaptation (mostly in urunc); the guest-autonomy side lives here.
