window.BENCHMARK_DATA = {
  "lastUpdate": 1792111363344,
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
        "date": 1789239880236,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1965.631,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1948.835,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1959.873,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 69.508,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 41.531,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 117.474,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 26.403,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 13.114,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 44.181,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.044,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 6.79,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.16,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.574,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.416,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.332,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 31.509,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.849,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 61.201,
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
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
        "date": 1789242480558,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1674.65,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1677.863,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1674.857,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.684,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.229,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 99.999,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 22.731,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 11.005,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 39.93,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.786,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.696,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.842,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.845,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.383,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.677,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 36.213,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 16.788,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 67.473,
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
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
        "date": 1789244391411,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1997.914,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 2005.416,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 2037.906,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 70.59,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 43.981,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 121.491,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 26.83,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 13.361,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 46.295,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.319,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.089,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.623,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.415,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.382,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.691,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.296,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.988,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 60.111,
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
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
        "date": 1789244855555,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1244.364,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1258.182,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1272.299,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.781,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 28.034,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 75.547,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.25,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.078,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.025,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.641,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.403,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.884,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.144,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.259,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.003,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.114,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 12.077,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47.851,
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
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
          "id": "b36bb983867a5f5c9a8ca351f1e3ca339d9978ef",
          "message": "ci: pin crates-io-auth-action to a SHA (v1.0.5); drop stale git-deps note\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T21:02:16Z",
          "tree_id": "792a30b74831261ef4269f4aecbb46dcd993d811",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/b36bb983867a5f5c9a8ca351f1e3ca339d9978ef"
        },
        "date": 1789247202427,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1729.24,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1707.697,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1728.675,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.347,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.208,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 104.002,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 23.778,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 11.532,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.115,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.976,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.748,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.096,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.916,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.403,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.779,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 37.424,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 17.443,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 68.258,
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
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
          "id": "15a18a414bf59270698af375e24e5f942c365216",
          "message": "cargo: add required crates.io metadata (description, license, repository, readme)\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T21:10:44Z",
          "tree_id": "4092aa4c6bedbd45a68c5b62f3aedab79635ab16",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/15a18a414bf59270698af375e24e5f942c365216"
        },
        "date": 1789247711898,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1242.108,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1236.793,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1247.51,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 45.241,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.9,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.427,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.292,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.293,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.97,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.615,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.295,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.607,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.17,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.255,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.985,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.868,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.923,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 46.151,
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
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "36b94063b559dc15d8111264903d463d0039ca95",
          "message": "Merge pull request #114 from hyperlight-dev/coop-pause-vm-v0.14.0\n\nv0.14.0: cooperative step model for long-running guests",
          "timestamp": "2026-09-20T15:50:40-07:00",
          "tree_id": "ef82580ea34504b16e6480392c408ffc6e3644bb",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/36b94063b559dc15d8111264903d463d0039ca95"
        },
        "date": 1789945522620,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1708.354,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1866.129,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1719.762,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 65.344,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.51,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 100.88,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 23.96,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.801,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.705,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 18.23,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 14.797,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.096,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.902,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.487,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.624,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 38.575,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.95,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 56.514,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
            "unit": "MB"
          }
        ]
      },
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
          "id": "f54d18ca2245c5c776660a0b0b44196b67d0f437",
          "message": "Merge pull request #115 from hyperlight-dev/ci-bench-hardening\n\nci, bench: report a hung benchmark as itself, and show how far it got",
          "timestamp": "2026-09-22T10:58:32-07:00",
          "tree_id": "7698416ca212eb8142169d86ac58bfe176aa601f",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f54d18ca2245c5c776660a0b0b44196b67d0f437"
        },
        "date": 1790100164742,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1677.862,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1666.482,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1692.315,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.768,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.704,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 97.954,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.63,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.802,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.011,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.258,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 14.924,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.399,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.154,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.513,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.731,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.42,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.928,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 50.291,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
            "unit": "MB"
          }
        ]
      },
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
          "id": "fb62e312716e874ca205079533bb0caad699cf33",
          "message": "Merge pull request #116 from hyperlight-dev/v0.14.1-urunc-fixes\n\nFixes and additions from the urunc port: AF_UNSPEC disconnect, --port all, --resolv-conf",
          "timestamp": "2026-09-22T16:00:42-07:00",
          "tree_id": "f5acc639381de1da38f8591283ce7421427365e3",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/fb62e312716e874ca205079533bb0caad699cf33"
        },
        "date": 1790118277689,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1675.826,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1664.102,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1677.639,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.991,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.735,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 100.003,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.534,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.883,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.526,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.112,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.076,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.435,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.939,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.52,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.695,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.203,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.662,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 54.437,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
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
          "id": "d67e68d6cf0b9852ba5e48f4d5d3981bb4f9bf98",
          "message": "demos: declare the urunc demo image's command\n\nAn OCI image says what it runs; Docker refuses to start one that says\nnothing, and the urunc demo image said nothing, so the README's own\n`docker run` line never worked. It ran everywhere else only because the\ndriver fell back to /entrypoint.py, a path that is the driver's business\nand should not be part of an image's contract.\n\nBake the workload at /app/hello.py and declare it as the image's CMD,\nwhich urunc passes to hluk as the guest command, and name Hyperlight in\nthe greeting. Refresh the README, which still described the urunc side\nas pending.\n\nVerified with the rebuilt image through hluk directly, `docker run` and\n`ctr run` with nothing after the image name.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-22T23:56:16Z",
          "tree_id": "685261fd60cbc7a91c107d588c6e04ffebbeb35b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d67e68d6cf0b9852ba5e48f4d5d3981bb4f9bf98"
        },
        "date": 1790121606016,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1663.657,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1653.71,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1669.147,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.924,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.426,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 100.294,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.428,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.723,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.814,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.01,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 14.859,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.229,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.899,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.541,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.722,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.429,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.74,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 53.679,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
            "unit": "MB"
          }
        ]
      },
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
          "id": "2b4790d1626e2645dd1f00469311424be8cb2f94",
          "message": "Merge pull request #117 from hyperlight-dev/hostfs-mounts-on-resume\n\nv0.14.1: Restore a snapshot with any mounts",
          "timestamp": "2026-09-22T21:30:13-07:00",
          "tree_id": "9e7c5f5de22880c141cdcdd15811ec44d0aee070",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/2b4790d1626e2645dd1f00469311424be8cb2f94"
        },
        "date": 1790138078045,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1648.726,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1658.694,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1681.222,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1699.038,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.882,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.327,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.368,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 97.756,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.557,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.777,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.721,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.339,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.374,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.087,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 16.68,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.53,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.974,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.559,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.701,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.688,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.845,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.752,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.168,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 48.913,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "93430ffb18831a6090b6c66d9c795aef2022da55",
          "message": "Merge pull request #118 from hyperlight-dev/hluk-init\n\nv0.14.2: projects, templates, a registry-pulled rootfs and warm snapshots",
          "timestamp": "2026-09-24T00:44:06-07:00",
          "tree_id": "1a8de55a9d9aad839d67227138cfc319403a8419",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/93430ffb18831a6090b6c66d9c795aef2022da55"
        },
        "date": 1790236200733,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1657.866,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1666.871,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1685.03,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1703.317,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.363,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.37,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.186,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 102.898,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.438,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.839,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.455,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.511,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.339,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.158,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 17.056,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.63,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.924,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.541,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.167,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.838,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.542,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.51,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.193,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 50.01,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "a69e8856a5c1c315f5c392ae00e305010bf2fb63",
          "message": "release: v0.15.0\n\nBump the crate to 0.15.0 and move the changelog's Unreleased entries under [v0.15.0], keeping an empty Unreleased section. 0.15.0 rather than 0.14.3: Exec gains a variant.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-25T12:51:08-07:00",
          "tree_id": "34e87bd3c3bf6077f93c657ae90623f00de82235",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a69e8856a5c1c315f5c392ae00e305010bf2fb63"
        },
        "date": 1790367105619,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1769.952,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1759.158,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1765.326,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1780.774,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 64.411,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 41.301,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 59.217,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 104.456,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.893,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.192,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.529,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.468,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.813,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.686,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 17.386,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 16.197,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.981,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.57,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.54,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.803,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.975,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.389,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.973,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 50.469,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 111.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 111.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 111.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 111.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "273ca74eda3ec0c3fad29b174768d1dcc9144b77",
          "message": "release: v0.16.0\n\nBump the crate to 0.16.0 and add the changelog's entries under\n[v0.16.0], keeping an empty Unreleased section.  0.16.0 rather than\n0.15.1: snapshots saved by 0.15 are refused, the host functions changed.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-26T14:45:37-07:00",
          "tree_id": "34908bd2c306a6bda57ad916877458945fd2fa55",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/273ca74eda3ec0c3fad29b174768d1dcc9144b77"
        },
        "date": 1790459362966,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 934.854,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 944.709,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 942.846,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 955.101,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 57.675,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.9,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.203,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 94.764,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.923,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.269,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.999,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.813,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.578,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.394,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.948,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.786,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.765,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.434,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.827,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.065,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.534,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.418,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.647,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 49.36,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "8f636e00cdf29e6c33ba0f578482c595d7827cf6",
          "message": "release: v0.17.0\n\nBump the crate to 0.17.0 and add the changelog's entries under\n[v0.17.0], keeping an empty Unreleased section.  0.17.0 rather than\n0.16.1: arm64 and macOS hosts are new, and guests see new behavior\n(Linux's siginfo for CPU-raised signals, SIGILL for SVE and SME).\nBump the snapshot contract for the new GetRandomBytes host function.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-28T23:29:16-07:00",
          "tree_id": "d3290c971a7d6c5468d1f0b4eb5961f43b18942b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/8f636e00cdf29e6c33ba0f578482c595d7827cf6"
        },
        "date": 1790664486694,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 898.827,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 896.172,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 894.325,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 903.203,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 55.303,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 33.546,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 50.121,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 90.551,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.008,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.874,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.725,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.848,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 11.944,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 11.789,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.143,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.114,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.973,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.359,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.293,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.457,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 24.563,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.952,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.923,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 45.678,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "distinct": false,
          "id": "44ee170f171ed387cbeb530775edb913bbf687aa",
          "message": "examples, tests: check musl condition variables and futex requeues\n\nTwo C programs, built with musl in Alpine (glibc's condition variables\ndo not requeue): condvar broadcasts to four waiters fifty times and\nexpects every one to wake, and futex_requeue checks FUTEX_REQUEUE and\nFUTEX_CMP_REQUEUE against Linux's semantics, which it passes on Linux\ntoo.  Every wait has a deadline, so a kernel that loses a waiter fails\nthe test rather than hanging it, and the tests print which check failed.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T14:23:19Z",
          "tree_id": "c75fe4aa7f972e6f548fa3f3b9fb7a0e6026d448",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/44ee170f171ed387cbeb530775edb913bbf687aa"
        },
        "date": 1790954059136,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 928.817,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 923.006,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 925.879,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 934.101,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 57.571,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.086,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 52.517,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 95.798,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.571,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.217,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.483,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.131,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.225,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.139,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.607,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.302,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.698,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.439,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.545,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.544,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.506,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.185,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.27,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 49.459,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "id": "57a491cd8c38c154e46fb7c78713a4bd9cf31a0c",
          "message": "ci: publish main's builds as the dev pre-release\n\nEach push to main whose CI passes publishes that commit's images, builds hluk for the four release targets pinned to them, moves the dev tag and replaces the dev pre-release, then deletes the dev images of all but this build and the one before.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T15:35:52Z",
          "tree_id": "9225836f73e02859f996924c72fa772985e99809",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/57a491cd8c38c154e46fb7c78713a4bd9cf31a0c"
        },
        "date": 1790955583877,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 945.93,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 935.309,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 940.398,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 954.839,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 58.478,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.33,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 53.456,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 97.493,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.821,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.176,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.537,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.52,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.382,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.222,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.587,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.587,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.841,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.42,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.604,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.818,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.116,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.208,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.309,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 50.054,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "id": "66d6f70039271bdfb443f9273dc0f285a341563c",
          "message": "README: say how hluk is pronounced\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T17:16:20Z",
          "tree_id": "989ffe030b778bd802adae626678a846769c2384",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/66d6f70039271bdfb443f9273dc0f285a341563c"
        },
        "date": 1790961629240,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 928.255,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 927.66,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 944.196,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 950.178,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 57.328,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.363,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 52.691,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 96.34,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.729,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.27,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.31,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.788,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.367,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.216,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.529,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.57,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.926,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.51,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.527,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.931,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.563,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.278,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.797,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 50.183,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "id": "146c71d599ce601df254149340486f5f532bb563",
          "message": "site: match hyperlight.org\n\nThe site now reads as part of hyperlight.org: its colors (Starlight's\nblue-gray neutrals and blue accent, in light and dark), a header like\nits nav bar that links back to it, and the Hyperlight logo for the mark\nand the favicon in place of a project-specific one. hyperlight.org takes\nthis site's fonts in turn, so the two match both ways.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T23:27:04Z",
          "tree_id": "03011ad5ad3b3f629476eda7584e89ce79c692a2",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/146c71d599ce601df254149340486f5f532bb563"
        },
        "date": 1790983833131,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 588.704,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 594.741,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 598.584,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 604.179,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 39.185,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 22.445,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 33.536,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 61.985,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.242,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.058,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.062,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 21.643,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 6.722,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 6.776,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 7.788,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.016,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.641,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.207,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.29,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.695,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 15.354,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.431,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 10.658,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 28.527,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "id": "3df47f64f99229e3cebef07b22ba948c69e1398c",
          "message": "site: stop narrow phones from scrolling sideways\n\nAt 375px the template picker made the page 4px wider than the screen,\nand at 320px the platform table did, by 11px. The template grid's one\ncolumn can now shrink below its content, and below 340px the table\nbreaks \"Hypervisor.framework\" rather than widening the page.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T23:41:32Z",
          "tree_id": "07515b9b26e279dec1882c4b82f82d6b214986af",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/3df47f64f99229e3cebef07b22ba948c69e1398c"
        },
        "date": 1790984722499,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 945.301,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 936.086,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 949.445,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 966.934,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.02,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.892,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.129,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 98.373,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.948,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.347,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.91,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.545,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.509,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.453,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.907,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.76,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.881,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.391,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.514,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.941,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.302,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.314,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.918,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 50.072,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "94942a874aef08d441b8a738e5d2a47f97f72cb4",
          "message": "docs: static Linux release binaries\n\nRELEASE.md names the musl targets, and the changelog records the change\nand why, for the release notes.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-14T12:30:04-07:00",
          "tree_id": "ad5bb11a764997b5e38ec138f1dea2671988a457",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/94942a874aef08d441b8a738e5d2a47f97f72cb4"
        },
        "date": 1792006432329,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 918.974,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 912.143,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 924.457,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 933.214,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 58.137,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.5,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.615,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 94.416,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.326,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.146,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.22,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.117,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.156,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.048,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.493,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.289,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.87,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.384,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.337,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.531,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.47,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.304,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.22,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 48.903,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "a85c89b2ac6119a3449948cd44db95de40222b22",
          "message": "CHANGELOG: the review fixes for guest output and vfork\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-14T13:57:10-07:00",
          "tree_id": "71dcbfc8af1e5a883b501a7afe0de9b6f1def86c",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a85c89b2ac6119a3449948cd44db95de40222b22"
        },
        "date": 1792012587365,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 921.01,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 912.412,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 920.198,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 949.338,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 58.055,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 33.819,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.541,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 96.269,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.128,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.735,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.42,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.215,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.181,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.04,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.452,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.467,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.901,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.294,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.415,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.492,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.033,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.658,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.931,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 49.61,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "distinct": false,
          "id": "cf458a07ed847f06d78284998a5beb13dc9fb426",
          "message": "hluk: keep no guest output in the CLI\n\nhluk run, hluk snapshot and hluk bench echo the guest's output as it is\nwritten and never drain it, yet kept all of it: a long-running guest\nthat kept printing grew the process without end (200 MiB of output\npeaked at 288 MB RSS, 80 MB now).  Every CLI boot now goes through one\nhelper that sets output_limit(0); the output still reaches the terminal.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-14T20:59:52Z",
          "tree_id": "1d4e4ff87b2a7f7b79fefdd62bc0c1efd249c12b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/cf458a07ed847f06d78284998a5beb13dc9fb426"
        },
        "date": 1792014058317,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 628.252,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 624.999,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 674.557,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 652.486,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 40.566,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 24.494,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 38.561,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 65.159,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.867,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.182,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.655,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 23.389,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.197,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.349,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.597,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.582,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.614,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.135,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.404,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.937,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 15.688,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.419,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.665,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.77,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "id": "5624423bf19e44e31acd2787a9627cfff0f8e72d",
          "message": "README: a shell script and a shell REPL in python-shell\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T16:58:11Z",
          "tree_id": "9a8ca7da9cfb0ed6263af8129450f9cb85ab968a",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/5624423bf19e44e31acd2787a9627cfff0f8e72d"
        },
        "date": 1792083945364,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 924.242,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 890.199,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 887.218,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 896.496,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 55.604,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.473,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 49.318,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 90.511,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.988,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.96,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.549,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.683,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 11.857,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 11.715,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 12.917,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 11.891,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.772,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.297,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.137,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.223,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.268,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.377,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 16.054,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.843,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 14,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "id": "50a2e4593f3a9d2062fc0da091896f88a713f292",
          "message": "ci: enable Dependabot version updates\n\nMonthly grouped updates for cargo, GitHub Actions and the Dockerfiles'\nbase images. Docker gets digest refreshes and patch releases only; rust\nand powershell images, Hyperlight minors and majors, and the local hluk-*\nrootfs images are left to be bumped by hand. Majors outside the wasmtime\ngroup and typos come as their own pull requests.\n\nFixes #135\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T17:09:56Z",
          "tree_id": "f5920b16c41cd2490c0da38cd837c961e3430f19",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/50a2e4593f3a9d2062fc0da091896f88a713f292"
        },
        "date": 1792085392974,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 957.791,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 949.449,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 948.163,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 951.055,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.823,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.165,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.785,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 97.544,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.282,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.43,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.999,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.184,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.739,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.51,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.216,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.89,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.966,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.388,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.535,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.71,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.401,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.464,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.27,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 49.483,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
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
          "id": "e5aaa2fbf54c8bad953ab97ecc36e5dcdd51e27a",
          "message": "CHANGELOG: Rust 1.99, the static arm64 wasmtime image\n\nFixes #129\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T17:38:20Z",
          "tree_id": "d00bd1550d12b4b9f20b74cf8c28c62671bec5d8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/e5aaa2fbf54c8bad953ab97ecc36e5dcdd51e27a"
        },
        "date": 1792087221910,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 640.317,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 624.331,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 655.036,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 666.44,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 41.188,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 24.671,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 39.421,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 71.718,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.143,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.399,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.058,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 24.411,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.218,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.221,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.696,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.001,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.67,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.176,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.393,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.564,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.284,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.875,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.227,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 33.403,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "39a850213d054eff389dbc1fae173accf8c86021",
          "message": "Merge pull request #149 from hyperlight-dev/dependabot/github_actions/dtolnay/rust-toolchain-e2a55d2ffb04f378e9626c28d38b36d230d1e12f\n\nbuild(deps): bump dtolnay/rust-toolchain from 02cb101ec7c40f2c49e1d9714d64511d8e1b74de to e2a55d2ffb04f378e9626c28d38b36d230d1e12f",
          "timestamp": "2026-10-15T11:48:52-07:00",
          "tree_id": "bde23d89676bc7c70bbba15443e128be8ada7032",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/39a850213d054eff389dbc1fae173accf8c86021"
        },
        "date": 1792091321260,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 945.342,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 950.206,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 955.541,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 972.441,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 66.421,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.824,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 67.069,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 102.362,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.955,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.939,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 15.592,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.365,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.514,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.316,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 12.945,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.784,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.969,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.319,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.62,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.875,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 22.708,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.292,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 16.742,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 39.951,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
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
            "name": "Dan Chiarlone",
            "username": "danbugs"
          },
          "distinct": true,
          "id": "f74bf3de6eff6a68244957dc244e19cc06a28bdb",
          "message": "kernel: fix starting, waiting for and stopping processes\n\nBump the kernel to the unikraft process and signal fixes\n(plat-hyperlight-v2 3c363e9d -> 51d0bd8a), each reproduced in a guest\nand, the generic ones, on stock QEMU/KVM:\n\n- wait() for a still-running child never woke unless it waited for any\n  child, and the guest deadlocked; with several waiting threads, or a\n  child gone some other way, waiters could sleep for good.  A parent\n  paused in sigsuspend() never woke for a child's SIGCHLD, so a shell's\n  `cmd & wait` hung; sigsuspend() never ran the handler of the signal\n  it unblocked, and pause() and sigsuspend() returned for signals they\n  do not take.\n- A killed process lived until its threads next left a system call: a\n  timeout's kill waited out a sleep and never ended a reader of a pipe.\n  Its blocked calls are now interrupted and unwind, releasing what they\n  hold.  A vfork parent resumed under its running child when another\n  thread or child of its exited, and a process could end under one.\n- One process group and session: setsid() failed, and killpg() and\n  kill(0) signalled every process, the caller's supervisor and init\n  included.  execve kept the old image's signal handlers.\n- execve of a missing file failed with EINVAL, not ENOENT; kill() of a\n  pid past the table and pthread_kill(t, 0) crashed the kernel; a full\n  signal queue refused SIGKILL and SIGTERM, and crashed the kernel on a\n  child's exit.\n- A program out of memory crashed the kernel rather than being killed;\n  it is killed now, though a child out of memory with others running\n  still crashes the kernel as its memory is freed.\n- select() called a new listener, a drained listener and a drained\n  connection readable, so a socketserver's shutdown() never returned.\n\nprocess_demo.py (20 steps), server_shutdown.py, the sigsuspend and\nvfork_wake C checks, and the tests that run them and an out-of-memory\nguest, crash, hang or fail on the old kernel.  Five CPython conformance\nmodules that crashed or hung pass now (test_docxmlrpc, test_dtrace,\ntest_httpservers, test_imaplib, test_robotparser) and leave the known\nfailures, with nine that pass on the old kernel too; test_fcntl and\ntest_wait3 fail rather than crash.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T12:35:58-07:00",
          "tree_id": "d934322a1fc8d822ddeab6dc2554bc43bcb06e02",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f74bf3de6eff6a68244957dc244e19cc06a28bdb"
        },
        "date": 1792094046951,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1210.05,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1211.942,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1217.939,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1227.36,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 82.279,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 48.405,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 73.645,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 114.739,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 23.935,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.476,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 16.729,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 39.516,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.221,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 11.986,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.367,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.527,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.842,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.344,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.721,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.983,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.654,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.089,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.366,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47.502,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "distinct": false,
          "id": "0414e5a04fdc200302c3031977cf48903e620507",
          "message": "conformance: skip test_gc until a sleeping thread wakes on time (#157)\n\ntest_trashcan_threads sleeps 1 s in its main thread while two threads allocate in a loop of short sleeps.  The main thread wakes late or not at all, and the guest runs out of memory: intermittent on main (Windows), every time with this branch's kernel, whose change shifts the timing without touching sleeping or scheduling.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T22:19:47Z",
          "tree_id": "22a21e9b722c6abe7e20755744c3a51e7f634f2b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/0414e5a04fdc200302c3031977cf48903e620507"
        },
        "date": 1792106060863,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 967.798,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 969.639,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 966.824,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 988.654,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.599,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.336,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.754,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 91.193,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.85,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.463,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.644,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.967,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.773,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.511,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.365,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.89,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.386,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.454,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.631,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.601,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.236,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.579,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.877,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 46.504,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "distinct": false,
          "id": "7ed4eb137c6285f8c478ff87d55441969a797490",
          "message": "tests: retry the HTTP GET examples when the server fails\n\ndotnet_jit_http_get, java_http_get, node_http_get and python_http_get\nfetch httpbin.org, which answers 502 often enough to fail CI on pull\nrequests that have nothing to do with networking.  A 5xx is the\nservice's failure, not the guest's, so the example now runs again, up\nto three times, 2 s then 5 s apart, each time in a fresh sandbox (a\nreused one keeps the failed attempt's sockets).  A run that fails any\nother way, or gets any other status, still fails the test at once: a\nDNS or connection error exercises the guest's resolver and network\nstack, which an outside request is there to test.  The output keeps\nevery attempt, and a retry is a CI warning, so retries that become the\nrule are seen.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:33:56Z",
          "tree_id": "f8e915338b960b068abc60350203c611e711ceec",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/7ed4eb137c6285f8c478ff87d55441969a797490"
        },
        "date": 1792108036792,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 970.736,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 974.663,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 986.722,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 987.873,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.689,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.301,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.028,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 89.464,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.661,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.444,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.205,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.441,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.674,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.587,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.265,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.863,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.097,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.41,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.575,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.765,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 30.075,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.767,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.633,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47.308,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
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
          "distinct": false,
          "id": "78e0b8e182040f5ba7eaa51342ce0e2508d424e1",
          "message": "CHANGELOG: Dependabot version updates, ci-passed waiting for arm64\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:43:36Z",
          "tree_id": "1a20c20e1116abff820839863e1572fac775de9c",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/78e0b8e182040f5ba7eaa51342ce0e2508d424e1"
        },
        "date": 1792108849645,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 964.491,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 963.697,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 956.182,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 998.198,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.434,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.829,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.924,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 88.967,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.906,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.543,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.08,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.413,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.878,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.758,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.65,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.05,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.098,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.398,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.679,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.037,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.542,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.303,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.695,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
            "unit": "MB"
          }
        ]
      },
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
          "id": "965ab21cce2cd6b964bd1baf2693aaafbfcddd6a",
          "message": "Merge pull request #160 from hyperlight-dev/dependabot/cargo/demos/pptx-gen/cargo-8681492501\n\nbuild(deps): bump rustls from 0.23.43 to 0.23.46 in /demos/pptx-gen in the cargo group across 1 directory",
          "timestamp": "2026-10-15T17:18:46-07:00",
          "tree_id": "d67a478738c1505c7a8e7d4ea172fd156040381e",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/965ab21cce2cd6b964bd1baf2693aaafbfcddd6a"
        },
        "date": 1792110153686,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 954.873,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 957.018,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 966.476,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1010.459,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.32,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.155,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.805,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 92.818,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.833,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.293,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.805,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.932,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.851,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.407,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.785,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.157,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.183,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.392,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.61,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.506,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.11,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.467,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 24.289,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47.031,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 17,
            "unit": "MB"
          }
        ]
      },
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
          "id": "75f065e006970a3a21e49a0576e0e6134abdcaea",
          "message": "Merge pull request #161 from hyperlight-dev/dependabot/cargo/drivers/wasmtime/driver/wasmtime-e909ad153e\n\nbuild(deps): bump wasmparser from 0.259.0 to 0.261.0 in /drivers/wasmtime/driver in the wasmtime group across 1 directory",
          "timestamp": "2026-10-15T17:22:38-07:00",
          "tree_id": "cfcc3acee37fd110882e74bbe37b8fca3db55260",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/75f065e006970a3a21e49a0576e0e6134abdcaea"
        },
        "date": 1792111360750,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 950.536,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 946.558,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 975.988,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1005.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.63,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.983,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.976,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 88.442,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.407,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.585,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.554,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.693,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.809,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.745,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.912,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.831,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.89,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.399,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.136,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.799,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.297,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.695,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 24.682,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 46.326,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 15,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 18,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}