window.BENCHMARK_DATA = {
  "lastUpdate": 1789242349137,
  "repoUrl": "https://github.com/hyperlight-dev/hyperlight-unikraft",
  "entries": {
    "python-shell benchmarks": [
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
        "date": 1789239706522,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 562.867,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 556.255,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 565.546,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.001,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.106,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 30.092,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.691,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.572,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.014,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.591,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.569,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.625,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.348,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.104,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.413,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.583,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.416,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 31.681,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 19,
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
        "date": 1789242347603,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 649.866,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 644.845,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 658.283,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 18.356,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.187,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.171,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.78,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.805,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.87,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.752,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.729,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.819,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.571,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.139,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.978,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.617,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.928,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.27,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 19,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}