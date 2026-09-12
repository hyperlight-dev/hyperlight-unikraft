# Benchmarks

## Modes

| Mode | Description |
|------|-------------|
| `cold` | Fresh boot (evolve) + dispatch, no snapshot. Measures end-to-end startup. |
| `cold-snap` | Load snapshot from disk + restore + dispatch. Measures cold-start from snapshot. |
| `warm-restore` | Load snapshot once, loop (dispatch + restore). Measures steady-state with memory reset. |
| `warm-stateful` | Load snapshot once, loop dispatch only. Measures pure execution overhead. |
| `parallel` | N concurrent VMs from the same snapshot. Measures throughput under contention. |

## Workloads

- **hello.py** — Minimal (`print("ok")`). Measures pure dispatch overhead.
- **compute.py** — CPU-bound (fibonacci + prime sieve). Measures compute performance.
- **stdlib.py** — Stdlib-heavy (json, re, collections, hashlib). Measures real-world library usage.

## Usage

```sh
just bench python              # all modes, all workloads
just bench python cold-snap    # one mode only
```

## Output

Produces a compact summary table at the end, plus a machine-readable `bench-results.json` at the repo root for CI consumption (format: `customSmallerIsBetter` for [github-action-benchmark]).

## CI gate

CI runs every mode on Linux and Windows and charts the results per host OS at <https://hyperlight-dev.github.io/hyperlight-unikraft> (`dev/bench/<os>/<runtime>`), with a current-versus-main table in each job's summary. Ratios against the previous run only warn: the hosted runners vary by 10-30% (Linux) to 2x (Windows) between runs of the same commit. The gate is `limits.json`: a ceiling per runtime, host OS and metric (`ms`, or `MiB` for `snapshot-size`), using the metric names from `bench-results.json`. A job fails when a named metric is over its ceiling; metrics not named are charted but not gated. The initial ceilings are about 2x (Linux) and 3x (Windows) the values seen on the hosted runners.

## Notes

**RSS:** The CLI reports `RssAnon` from `/proc/self/status` on Linux and the private working set (`GetProcessMemoryInfo`) on Windows — private resident memory only. This is the density-relevant metric: it scales linearly with VM count. Guest memory is backed differently on the two (anonymous mmap on Linux, file mapping on Windows), so values are comparable within an OS, not across.

**Snapshot size:** Reported as total bytes on disk (recursive `stat` of the snapshot directory). Same value on all platforms.

## TODO

- [ ] Integrate [pyperformance](https://pyperformance.readthedocs.io/) for standardized Python benchmarks alongside custom workloads.
- [ ] Bench other runtimes.

[github-action-benchmark]: https://github.com/benchmark-action/github-action-benchmark
