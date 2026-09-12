window.BENCHMARK_DATA = {
  "lastUpdate": 1789244732010,
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
          "id": "9502e7de06b5dd96cd87e944ede85dbafb2f5c7c",
          "message": "demos/urunc: build the python rootfs base before staging the urunc image\n\npublish-urunc's rootfs.Dockerfile is FROM hluk-python-rootfs:latest — a\nlocal build stage, not a registry image. The publish job never built it, so\nDocker tried to pull it from Docker Hub and failed. Build it first in stage.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T20:15:17Z",
          "tree_id": "62cee22f1881f7340cf5f2bdb2d37966edd7c7aa",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/9502e7de06b5dd96cd87e944ede85dbafb2f5c7c"
        },
        "date": 1789244247541,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 624.295,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 619.039,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 633.844,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.516,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.869,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.831,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.686,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.732,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.052,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.65,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.593,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.713,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.569,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.118,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.351,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.748,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.85,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.293,
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
          "id": "d41b90a58bb73653a019f8bfa3357ad0ef93ea15",
          "message": "demos/urunc: call _push from the repo root\n\nThe publish-urunc recipe cd'd into demos/urunc and then called `just _push`,\nwhich resolved to demos/urunc/Justfile (no _push there). Run the staging +\ndocker build in a subshell so _push runs against the root justfile.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T20:23:33Z",
          "tree_id": "4b667d4b38a0253453f3cf9cafc6f4477fce31ea",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d41b90a58bb73653a019f8bfa3357ad0ef93ea15"
        },
        "date": 1789244731322,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 615.114,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 613.925,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 628.305,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.145,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.986,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.711,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.191,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.691,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.131,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.596,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.59,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.686,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.128,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.189,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 14.07,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.607,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.501,
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