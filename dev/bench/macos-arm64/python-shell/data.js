window.BENCHMARK_DATA = {
  "lastUpdate": 1792609628149,
  "repoUrl": "https://github.com/hyperlight-dev/hyperlight-unikraft",
  "entries": {
    "python-shell benchmarks": [
      {
        "commit": {
          "author": {
            "email": "danilochiarlone@gmail.com",
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "290ab8c812127f57f11306401f31436b609ff36f",
          "message": "Merge pull request #174 from hyperlight-dev/arm64-parity\n\narm64: CI parity with x86 on Linux, macOS and Windows",
          "timestamp": "2026-10-21T11:37:21-07:00",
          "tree_id": "e226581d7a42a090dbb0789dfdd0f8502b6cd919",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/290ab8c812127f57f11306401f31436b609ff36f"
        },
        "date": 1792609626670,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 352.94,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 354.219,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 348.982,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 335.472,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 6.302,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 2.679,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 3.446,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 12.808,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 3.47,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 0.436,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 1.273,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 9.739,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.385,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 3.618,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.264,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.357,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.947,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.057,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.698,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.766,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.892,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.238,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 31.874,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 28.351,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 105.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 105.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 105.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 105.7,
            "unit": "MiB"
          }
        ]
      }
    ]
  }
}