window.BENCHMARK_DATA = {
  "lastUpdate": 1789242329881,
  "repoUrl": "https://github.com/hyperlight-dev/hyperlight-unikraft",
  "entries": {
    "python benchmarks": [
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
        "date": 1789239693107,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 288.358,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 285.735,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 402.325,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.996,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.893,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 145.526,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.683,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.211,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 125.384,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.36,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.35,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.568,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.457,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.159,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.225,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.539,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.56,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 217.607,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 78.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 78.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 78.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 22,
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
        "date": 1789242328260,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 225.136,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 223.03,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 314.374,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 11.478,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.485,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 112.248,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.118,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.224,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 96.85,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.295,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.287,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.452,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.986,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.068,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.073,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.736,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 4.15,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 169.173,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 78.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 78.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 78.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 22,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}