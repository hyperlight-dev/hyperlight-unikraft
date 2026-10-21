window.BENCHMARK_DATA = {
  "lastUpdate": 1792609011316,
  "repoUrl": "https://github.com/hyperlight-dev/hyperlight-unikraft",
  "entries": {
    "python benchmarks": [
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
        "date": 1792609009834,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 135.861,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 140.198,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 117.017,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 169.115,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 6.238,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 2.386,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 3.93,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 53.966,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.241,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 0.378,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 1.525,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 49.92,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.094,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.426,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.518,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 4.659,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.051,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.061,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.229,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 47.344,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 5.907,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.329,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 10.856,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 109.681,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 75.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 75.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 75.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 75.8,
            "unit": "MiB"
          }
        ]
      }
    ]
  }
}