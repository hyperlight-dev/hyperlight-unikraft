window.BENCHMARK_DATA = {
  "lastUpdate": 1789245073800,
  "repoUrl": "https://github.com/hyperlight-dev/hyperlight-unikraft",
  "entries": {
    "agent benchmarks": [
      {
        "commit": {
          "author": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "committer": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "4cdb5dbeff65ff2884edcb481a447af4f2cff762",
          "message": "Replace with hyperlight-unikraft 0.13.0 (from hl-uk-mini); prior 0.12.x history preserved",
          "timestamp": "2026-09-12T18:48:12Z",
          "tree_id": "9902f340cc8a8a76ee16123c29e3ec3c981c5ded",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/4cdb5dbeff65ff2884edcb481a447af4f2cff762"
        },
        "date": 1789240120180,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7406.462,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7387.907,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7372.973,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 29.964,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.309,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 47.087,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 10.19,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.537,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.759,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.509,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.4,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.475,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.108,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.213,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.384,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.676,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.753,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.9,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
            "unit": "MB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "committer": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "cb6b6e1eb13f0ad346a93e6ca28a0f1dd042df4c",
          "message": "kernel: source the Unikraft platform from upstream unikraft/unikraft@plat-hyperlight-v2\n\nThe kernel/unikraft submodule now points at the canonical\nunikraft/unikraft plat-hyperlight-v2 branch instead of the danbugs fork.\nSame commit (9eca1f0b), so the embedded kernel binary is unchanged.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T19:42:24Z",
          "tree_id": "c408dbd54aff44e0e30a89eeb097bf40aadf0f80",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/cb6b6e1eb13f0ad346a93e6ca28a0f1dd042df4c"
        },
        "date": 1789242767701,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7566.762,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7596.381,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7515.693,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 30.967,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.315,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 49.748,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 10.632,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.297,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.188,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.562,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.44,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.524,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.56,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.081,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.049,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.313,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.151,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.883,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
            "unit": "MB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "committer": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "d41b90a58bb73653a019f8bfa3357ad0ef93ea15",
          "message": "demos/urunc: call _push from the repo root\n\nThe publish-urunc recipe cd'd into demos/urunc and then called `just _push`,\nwhich resolved to demos/urunc/Justfile (no _push there). Run the staging +\ndocker build in a subshell so _push runs against the root justfile.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T20:23:33Z",
          "tree_id": "4b667d4b38a0253453f3cf9cafc6f4477fce31ea",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d41b90a58bb73653a019f8bfa3357ad0ef93ea15"
        },
        "date": 1789245072146,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5998.596,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5912.052,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5904.696,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 24.379,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.397,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.893,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.406,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.548,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.526,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.244,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.219,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.359,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.992,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.068,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.529,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.453,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.516,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.115,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}