# Profiling

`hluk run --profile` (or `HLUK_PROFILE=1`, or `SandboxBuilder::profile(true)` from the library) records where a sandbox's time goes, seen from the host, and prints it when the run ends:

```sh
hluk run --cold --profile --initrd build-elfloader/quickjs-rootfs.cpio examples/quickjs/handler.js \
  --call handler --input '{"name":"x"}'
```

```
hluk profile (host side)                    count    total ms    mean µs     max µs    exits
entry Exec                                      1       0.205      205.4      205.4      2.0
  guest                                         1       0.205      204.9      204.9
  host functions                                1       0.001        0.6        0.6
entry Call                                      1       0.096       95.8       95.8      1.0
  guest                                         1       0.095       95.5       95.5
  host functions                                1       0.000        0.3        0.3
boot: evolve (cold boot)                        1      42.215    42214.8    42214.8
host GetTscHz                                   1      20.070    20070.0    20070.0
host Yield                                      3       0.001        0.3        0.3
...
```

| Row | What it measures |
|---|---|
| `entry NAME` | One VM entry (`step`, `resume`, `Exec`, `Call`, …), wall time. `exits` is the mean number of host function calls it made, one VM exit each. |
| `guest` | The entry's time outside host functions: the guest running, plus the VM exits and entries themselves. |
| `host functions` | The entry's time inside host functions. |
| `host NAME` | Each host function hluk registers, across all entries. |
| `restore: hyperlight` | Hyperlight's own restore, before the `resume` entry. |
| `boot: …` | A cold boot (`evolve`), or a sandbox started from a snapshot. |

From the library, `AppSandbox::profile()` gives the report (`report()`) and a way to start over after a warm-up (`reset()`). Off, the profiler costs one atomic load per entry and host function.

The profile is host-side: it tells guest time from host time, and counts exits, but not what the guest spent its time on. For that, Unikraft's tracepoints (`UK_TRACEPOINT`, `CONFIG_LIBUKDEBUG_TRACEPOINTS`) or sampling the guest with `perf kvm --guest` against `kernel/.build/elfloader_hyperlight-<arch>.dbg` are the tools.
