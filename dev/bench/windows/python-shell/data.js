window.BENCHMARK_DATA = {
  "lastUpdate": 1793142016297,
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
          "id": "a2ad7ab464bbd46b49798452281b848e47f3cea5",
          "message": "release: v0.18.0\n\nBump the crate to 0.18.0 and add the changelog's entries under\n[v0.18.0], keeping an empty Unreleased section.  0.18.0 rather than\n0.17.1: the host transport moves to Hyperlight 0.18's virtqueues, the\nhost functions changed (HostWrite, HostWriteErr), and snapshots and\nnative kernels built for 0.17 are refused.  The snapshot contract was\nalready bumped with those changes; the kernel moved too, which rolls\nthe snapshot key by itself.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-16T00:22:52Z",
          "tree_id": "3a351f900d4e88ce267f6dd00a34cd41f2385049",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a2ad7ab464bbd46b49798452281b848e47f3cea5"
        },
        "date": 1792112309396,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 696.822,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 717.507,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 741.995,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 701.831,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.719,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 28.006,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 41.54,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 62.31,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.377,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.725,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.763,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.86,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.325,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.338,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.609,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.289,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.151,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.291,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.801,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.856,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.75,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.584,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.552,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 31.698,
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
          "id": "d8f5a055c47b9c60ec3a50eeb83fc836e8217d08",
          "message": "Merge pull request #163 from hyperlight-dev/smolagents-integration\n\nAdd smolagents-hyperlight, a smolagents executor in a micro-VM",
          "timestamp": "2026-10-19T00:37:27-07:00",
          "tree_id": "3bc95c35b2f14ffb9bb7f2329897ba862f85be4a",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d8f5a055c47b9c60ec3a50eeb83fc836e8217d08"
        },
        "date": 1792395670070,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 946.52,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 953.375,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 962.339,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 973.399,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.398,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.381,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.36,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 88.574,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.906,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.393,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.517,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.53,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.669,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.411,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.304,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.766,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.987,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.408,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.665,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.935,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.767,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.733,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.273,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 45.242,
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
          "distinct": true,
          "id": "e1aa358ed7a3ddf787b46d9b676277000c75fcf1",
          "message": "integrations/smolagents: release 0.1.1\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-19T13:13:38Z",
          "tree_id": "bce20460e72585f23122a2282bddaca743187a36",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/e1aa358ed7a3ddf787b46d9b676277000c75fcf1"
        },
        "date": 1792415845157,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 958.225,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 987.053,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1009.282,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 983.827,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.527,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.879,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.697,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 89.64,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.711,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.937,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.294,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.066,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.636,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.712,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.261,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.75,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.967,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.417,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.659,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.584,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.656,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.019,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.771,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 46.384,
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
          "id": "c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597",
          "message": "README, CHANGELOG, docs/RELEASE.md: mcp-server-hyperlight\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T06:56:43Z",
          "tree_id": "d76923ae19bdf0b4c4b4bd38aec58c5340d50e04",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597"
        },
        "date": 1792480359488,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 938.406,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 939.203,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 945.083,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 946.32,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.231,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.598,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.962,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 87.379,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.117,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.161,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.848,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.03,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.645,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.337,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.076,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.616,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.007,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.383,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.551,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.381,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.052,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.641,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.931,
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
          "id": "dcb8b5010494132b990c75525c5dd4c6a29bbe01",
          "message": "CHANGELOG: Windows arm64 CI\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T08:13:51Z",
          "tree_id": "4c13bd809aac1b0ac04baedecd9575615c8eb1eb",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/dcb8b5010494132b990c75525c5dd4c6a29bbe01"
        },
        "date": 1792485319720,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 716.688,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 715.343,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 721.176,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 731.522,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 46.393,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 28.442,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 43.175,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 66.173,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.191,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.03,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.988,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 23.866,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.675,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.439,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.623,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.588,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.227,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.325,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.931,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.874,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 20.299,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.795,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.667,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 32.739,
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
          "id": "69c1d8ccb57090793642dd64bcb5d6456c76aff2",
          "message": "Merge pull request #168 from hyperlight-dev/fix-py-flush\n\ndrivers: flush Python's stdout and stderr when a call ends",
          "timestamp": "2026-10-20T20:47:31-07:00",
          "tree_id": "fdf5c27204ba52f7bc2ffa80733cdd37b2fa8eb8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/69c1d8ccb57090793642dd64bcb5d6456c76aff2"
        },
        "date": 1792555517968,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 699.003,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 690.177,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 696.949,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 706.515,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.789,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.623,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.574,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 62.867,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.61,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.609,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.979,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.5,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 8.14,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.728,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 9.014,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.154,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.861,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.315,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.517,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.588,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.523,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.24,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 13.339,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 31.046,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "a7fb4a0d785960dcd128e3d3d0fd69599fce5293",
          "message": "Merge pull request #167 from hyperlight-dev/clickhouse-udf\n\nintegrations/clickhouse: untrusted Python UDFs for ClickHouse in micro-VMs",
          "timestamp": "2026-10-20T21:16:07-07:00",
          "tree_id": "06b079ed25f8283740ae2c14d6f7717373c5159d",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a7fb4a0d785960dcd128e3d3d0fd69599fce5293"
        },
        "date": 1792556503697,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 968.38,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1051.064,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1042.605,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 959.462,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 69.172,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 43.439,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.967,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 90.278,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.942,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.604,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.46,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 35.135,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.743,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 14.323,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.101,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 14.983,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 4.38,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.596,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.851,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.01,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 45.41,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.824,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.015,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 46.101,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "5efba5f4f34cfb410a130c3e3e0e8aea7334bfaa",
          "message": "Merge pull request #169 from hyperlight-dev/windows-arm64-guests\n\nWindows arm64 guests: shareable arm64 pages, initrd clear of WHP's GIC",
          "timestamp": "2026-10-20T23:03:22-07:00",
          "tree_id": "4389426a93fa5c7703812621980a540796254f93",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/5efba5f4f34cfb410a130c3e3e0e8aea7334bfaa"
        },
        "date": 1792562782407,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 708.225,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 704.124,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 707.404,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 735.885,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 45.376,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.709,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 41.615,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 63.507,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.68,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.822,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.287,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.863,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 8.127,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 8.029,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 9.138,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.248,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.879,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.258,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.69,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.791,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.78,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.247,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 16.354,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.828,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "f5bcafc73642f096bec5468a9c73f1298de89926",
          "message": "CHANGELOG, docs/RELEASE.md: MCP Registry and Desktop Extension releases\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-21T06:06:16Z",
          "tree_id": "63d8957dc45c01ebfdc0e04feb8dec8ab2489ed8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f5bcafc73642f096bec5468a9c73f1298de89926"
        },
        "date": 1792563773143,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 946.172,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 946.423,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 960.862,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1011.248,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.489,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.767,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 61.229,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 86.615,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.777,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.279,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.717,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.274,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.511,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.468,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.222,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.683,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.046,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.446,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.99,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.535,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.317,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.296,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.381,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 45.084,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "290ab8c812127f57f11306401f31436b609ff36f",
          "message": "Merge pull request #174 from hyperlight-dev/arm64-parity\n\narm64: CI parity with x86 on Linux, macOS and Windows",
          "timestamp": "2026-10-21T11:37:21-07:00",
          "tree_id": "e226581d7a42a090dbb0789dfdd0f8502b6cd919",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/290ab8c812127f57f11306401f31436b609ff36f"
        },
        "date": 1792608092611,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 910.417,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 904.615,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 917.919,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 923.124,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 58.539,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.912,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 53.33,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 82.416,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.793,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.861,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.894,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.341,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.193,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.027,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.547,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.344,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.028,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.368,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.381,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.335,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.47,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.197,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.815,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 41.519,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "ba9e757928837e5a6b6373b7c9f5e234e65da8d2",
          "message": "Merge pull request #176 from hyperlight-dev/dependabot-composite-actions\n\ndependabot.yml: update the composite actions' pins too",
          "timestamp": "2026-10-21T12:13:35-07:00",
          "tree_id": "83d2dfeb82e93e1f29f0cda955a23a4a33cc9a37",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/ba9e757928837e5a6b6373b7c9f5e234e65da8d2"
        },
        "date": 1792610253167,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1130.818,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1107.404,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1117.251,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1117.827,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 76.565,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 46.457,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 70.836,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 105.942,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 22.653,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.742,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 16.112,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 35.161,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 10.738,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 10.407,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 12.53,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.905,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.759,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.396,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.405,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.589,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.515,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.827,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.608,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 42.652,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "7ac0437956f8b3d15a94189d3db510db7ab1b535",
          "message": "Merge pull request #179 from hyperlight-dev/net-filter\n\nnet_policy: NetworkPolicy::Filter, a destination filter the embedder decides with",
          "timestamp": "2026-10-21T15:45:26-07:00",
          "tree_id": "e99e2cd943d345907926f62ec45bde996ed8f2e1",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/7ac0437956f8b3d15a94189d3db510db7ab1b535"
        },
        "date": 1792623877014,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 692.493,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 691.054,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 698.998,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 703.84,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.482,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.038,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.677,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 64.479,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.438,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.398,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.777,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 23.343,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.19,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.075,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.283,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.33,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.281,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.32,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.832,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.021,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.75,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.267,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.03,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 32.462,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "a7facb316caae77e6a277a03873cad6481c9035e",
          "message": "Merge pull request #177 from hyperlight-dev/dependabot/github_actions/dot-github/actions/arm64-host/actions-a055b02c66\n\nbuild(deps): bump astral-sh/setup-uv from 10.2.0 to 10.3.0 in /.github/actions/arm64-host in the actions group across 1 directory",
          "timestamp": "2026-10-21T16:13:59-07:00",
          "tree_id": "3b7d6b6c8ee5c3e379bb2272d1dc7f9e43412178",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a7facb316caae77e6a277a03873cad6481c9035e"
        },
        "date": 1792624690763,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 958.885,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 971.007,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 979.827,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 984.307,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 64.315,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.471,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.002,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 89.969,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.063,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.489,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.998,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.698,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.757,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.609,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.463,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.125,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.062,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.443,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.562,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.134,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.341,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.349,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.567,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 46.583,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
          "id": "47bc9e250df9f76c9294639421c0f37533c0817d",
          "message": "Merge pull request #181 from hyperlight-dev/fs-backend\n\nhostfs: FsBackend, mounts served by the embedder",
          "timestamp": "2026-10-21T16:42:55-07:00",
          "tree_id": "f2a5ebd44efd1940857eac717abaa36be33cc8a1",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/47bc9e250df9f76c9294639421c0f37533c0817d"
        },
        "date": 1792627377625,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 935.865,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 949.721,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 969.463,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 952.834,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.778,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.23,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.734,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 84.916,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.247,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.438,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.831,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.586,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.608,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.531,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.21,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.789,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.093,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.508,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.621,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.594,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.043,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.693,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.353,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.838,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
            "name": "danbugs",
            "username": "danbugs"
          },
          "committer": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "distinct": false,
          "id": "b11c33e8d180dd06ce5087a575a77100c029fef4",
          "message": "arm64-host: a job-scoped hluk cache\n\nPoint HLUK_CACHE_DIR at the job's temp directory on the self-hosted\nhosts, which the runner empties when the job ends, cancelled or not, so\nthe images a job pulls and the warm snapshots it saves go with it,\ninstead of staying in the runner user's home for later jobs, and\nhyperlight's, to find the disk full.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-22T00:42:05Z",
          "tree_id": "fb01edfe62fe98b17eeca80462299a1a01013e18",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/b11c33e8d180dd06ce5087a575a77100c029fef4"
        },
        "date": 1792630953086,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 705.384,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 720.001,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 748.558,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 719.346,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 45.69,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 29.49,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 48.279,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 64.999,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.819,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.87,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.119,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 24.79,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.567,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.417,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.824,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.744,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.277,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.304,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.064,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.958,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.57,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.909,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 13.198,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47.341,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.7,
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
            "name": "danbugs",
            "username": "danbugs"
          },
          "committer": {
            "email": "danilochiarlone@gmail.com",
            "name": "danbugs",
            "username": "danbugs"
          },
          "distinct": false,
          "id": "51a085f8446713530991febc02ef2f5113fa5d69",
          "message": "CHANGELOG: guest file, console, call and kernel log fixes\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-22T00:59:05Z",
          "tree_id": "58b3c77c5413a3c798b4f42a9f0ba0be2e5b44f6",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/51a085f8446713530991febc02ef2f5113fa5d69"
        },
        "date": 1792685329569,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 684.834,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 680.822,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 677.539,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 689.801,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 45.722,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.737,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 41.477,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 63.52,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.405,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.684,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.699,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.461,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 8.1,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.89,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 9.113,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.017,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.774,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.343,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.569,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.603,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.05,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.217,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 13.104,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.472,
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
          "id": "bc59be94cc731a588e0f7fee30c9334e7a31e0a6",
          "message": "Merge pull request #183 from hyperlight-dev/ci-flakes\n\nCI flakes: re-measure benchmark breaches, MCP and smolagents test races, Dependabot majors",
          "timestamp": "2026-10-22T11:09:15-07:00",
          "tree_id": "f18d079483036e4b2c4cdc57f2d5654cda3ca55a",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/bc59be94cc731a588e0f7fee30c9334e7a31e0a6"
        },
        "date": 1792692783964,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 931.939,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 945.347,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 947.392,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 952.17,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.903,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.863,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.652,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 87.817,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.245,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.442,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.95,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.431,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.732,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.62,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.042,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.744,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.828,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.609,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.671,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.871,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.762,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.687,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.429,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 45.47,
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
          "id": "342cd051800ee07412ea58e00857a9fcff7e065c",
          "message": "Merge pull request #184 from hyperlight-dev/bind-gate\n\nhostnet: refuse a bind on a port no one listed",
          "timestamp": "2026-10-22T13:16:36-07:00",
          "tree_id": "4cfb31f78c5ccfd710065f136e9d5c1fdab1fce5",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/342cd051800ee07412ea58e00857a9fcff7e065c"
        },
        "date": 1792701390509,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 641.369,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 632.162,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 629.642,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 643.744,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 43.168,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 25.245,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 37.577,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 58.953,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.743,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.282,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.592,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 21.512,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.616,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.29,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.303,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.736,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.795,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.335,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.35,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.327,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 16.677,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.666,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.054,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 28.711,
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
          "id": "d462556852c86c1d66a54548767daeb642e1d0d1",
          "message": "Merge pull request #186 from hyperlight-dev/dependabot/docker/drivers/dotnet-jit/images-0ccb09127f\n\nbuild(deps): bump the images group across 3 directories with 1 update",
          "timestamp": "2026-10-22T14:02:35-07:00",
          "tree_id": "c440f18c19dbcd0d50bcfbfec1b7477a7581194a",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d462556852c86c1d66a54548767daeb642e1d0d1"
        },
        "date": 1792704178485,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1182.038,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1220.006,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1221.702,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1181.044,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 80.365,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 48.484,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 71.329,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 109.896,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 23.376,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.974,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 16.526,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 38.053,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.555,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13.402,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.282,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.702,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.823,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.771,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.399,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.582,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.565,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.058,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.107,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 56.784,
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
          "id": "ee0ef9d18bee96b4f91905cf5387b19fcaf71dbc",
          "message": "Merge pull request #188 from hyperlight-dev/verl-integration\n\nintegrations/verl: verl-hyperlight, run verl's code-reward test cases in micro-VMs",
          "timestamp": "2026-10-22T14:24:10-07:00",
          "tree_id": "72ff363b43e77a5a6056925cf393df9100c8a357",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/ee0ef9d18bee96b4f91905cf5387b19fcaf71dbc"
        },
        "date": 1792704484503,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 679.35,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 669.217,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 673.059,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 676.238,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.982,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 26.711,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.464,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 62.808,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.567,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.52,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.6,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.704,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 8.2,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.619,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 9.031,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.236,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.814,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.342,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.668,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.516,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.515,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.983,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 13.202,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 29.417,
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
          "id": "02fd27911eeeed87db7ef9ac768500f7ef4d796b",
          "message": "Merge pull request #190 from hyperlight-dev/release-v0.19.0\n\nRelease v0.19.0",
          "timestamp": "2026-10-22T18:21:05-07:00",
          "tree_id": "abdbcdf3b28152a092bda7d3a8e947b57c46c352",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/02fd27911eeeed87db7ef9ac768500f7ef4d796b"
        },
        "date": 1792718697670,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1081.239,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1066.201,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1068.817,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1086.064,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 72.664,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 42.443,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 64.5,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 102.03,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.891,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.354,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.867,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.221,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 10.768,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 10.407,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 12.597,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 11.231,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.479,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.544,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.156,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.74,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.945,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.555,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.933,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 41.945,
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
          "id": "f4a8234ed2a775ef8647f1302614f46356c9e611",
          "message": "Merge pull request #191 from hyperlight-dev/ci-tiers\n\nCI tiers: path-filtered x86 on every PR, macOS/Windows arm64 on ci/arm64, Linux arm64 nightly",
          "timestamp": "2026-10-22T19:24:43-07:00",
          "tree_id": "1c30f9ba4d00180e4b21f515555227d33d1a8948",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f4a8234ed2a775ef8647f1302614f46356c9e611"
        },
        "date": 1792723188283,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 700.064,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 698.942,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 705.935,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 713.441,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.324,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.975,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.284,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 64.115,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.387,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.672,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.572,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 23.423,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.456,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.412,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.53,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.653,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.317,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.426,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.961,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.036,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.898,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.514,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.894,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 31.766,
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
          "id": "31fde20972f3ee4e0d8f7d34a842f5f096957c40",
          "message": "Merge pull request #192 from hyperlight-dev/conformance-skip-race\n\njustfile: a module the conformance skip list names is always skipped",
          "timestamp": "2026-10-22T20:15:29-07:00",
          "tree_id": "e1015fa35b945892524db27d1c6a012d38cba9ad",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/31fde20972f3ee4e0d8f7d34a842f5f096957c40"
        },
        "date": 1792725562276,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 971.131,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 960.481,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 974.846,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 987.335,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.703,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.819,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.466,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 89.499,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.87,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.625,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.776,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.316,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.977,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.831,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.452,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.182,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.037,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.588,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.751,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.922,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.315,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.702,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.783,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 45.728,
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
          "id": "8354ed2d64885db20725d8ab06f5f13897120acb",
          "message": "Merge pull request #195 from hyperlight-dev/snapshot-fixes\n\nReseed userspace RNGs after snapshot restore, plus kernel fixes",
          "timestamp": "2026-10-23T14:00:32-07:00",
          "tree_id": "fb91fc70b42848eb9e3c92df43999f6b9693fa96",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/8354ed2d64885db20725d8ab06f5f13897120acb"
        },
        "date": 1792790293986,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 952.604,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 947.652,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 956.093,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 973.397,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 92.059,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 73.355,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 90.049,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 114.716,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 29.701,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 20.95,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 26.991,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 43.593,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.038,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.913,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.607,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.256,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.959,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.437,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.5,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.859,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 46.337,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 35.861,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 44.27,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 64.428,
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
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 21,
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
          "id": "236f429c3bb2348569f4b87aa9e2e3d277a54540",
          "message": "Merge pull request #197 from hyperlight-dev/php-driver\n\nAdd a PHP runtime (tier 2)",
          "timestamp": "2026-10-23T23:29:31Z",
          "tree_id": "82e7a3a457a996889fa73cbe6b4ff968475c432d",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/236f429c3bb2348569f4b87aa9e2e3d277a54540"
        },
        "date": 1792799527777,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 650.108,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 638.084,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 635.368,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 631.721,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 64.666,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 49.858,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 61.589,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 76.521,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.768,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 14.449,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 18.088,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 28.93,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.952,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.701,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.82,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.84,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.765,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.186,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.379,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.167,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.799,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 23.868,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 26.356,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 37.483,
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
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 21,
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
          "id": "74541cf8063692a558a6b9029ec147d913871bd1",
          "message": "Merge pull request #199 from hyperlight-dev/php2shell-demo\n\nAdd a PHP RCE containment demo",
          "timestamp": "2026-10-24T00:17:17Z",
          "tree_id": "9153126bad07e53801c9251cf216abbd034a4bb2",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/74541cf8063692a558a6b9029ec147d913871bd1"
        },
        "date": 1792801410052,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1273.923,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1285.677,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1277.622,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1317.512,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 128.277,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 105.766,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 132.579,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 165.111,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 42.281,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 31.234,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 38.068,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 57.161,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.947,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13.566,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 15.882,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.862,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.05,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.481,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.67,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 18.507,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 54.069,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 41.805,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 65.653,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 108.71,
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
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 21,
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
          "id": "5f14d1e73039f2cfb4928766586cfe75a87301ad",
          "message": "Merge pull request #201 from hyperlight-dev/release-v0.19.1\n\nRelease v0.19.1",
          "timestamp": "2026-10-23T17:30:03-07:00",
          "tree_id": "762317564015fd9b5c785d8dc374ab629146a7d0",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/5f14d1e73039f2cfb4928766586cfe75a87301ad"
        },
        "date": 1792802108171,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1158.811,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1161.108,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1165.034,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1174.774,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 116.739,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 94.206,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 113.924,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 141.206,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 37.046,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 26.894,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 33.627,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 51.355,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.629,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.546,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.454,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.711,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.766,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.35,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.312,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.441,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 45.243,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 35.741,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 43.859,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 62.292,
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
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 21,
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
          "id": "194db1ca2e5272de24fd1aed9d6ba82edfd1f9e3",
          "message": "Merge pull request #208 from hyperlight-dev/nightly-arm64-fixes\n\nClear the arm64 nightly failures from v0.19.1",
          "timestamp": "2026-10-25T23:13:01-07:00",
          "tree_id": "8a03322175db8a9bde0819822d787074476aac14",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/194db1ca2e5272de24fd1aed9d6ba82edfd1f9e3"
        },
        "date": 1792995422789,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 964.48,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 964.037,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 975.037,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 971.264,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 92.155,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 76.878,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 92.678,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 115.92,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 29.811,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 21.271,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 27.389,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 42.481,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.068,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13.049,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.661,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.198,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.928,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.437,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.665,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.984,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 46.188,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 36.772,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 44.636,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 63.106,
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
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 17,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 21,
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
          "id": "9d4480c22178f65feac93fe9767b7737303f686f",
          "message": "Merge pull request #209 from hyperlight-dev/memory-lifetimes\n\nRelease a process's memory when it ends, and support mremap()",
          "timestamp": "2026-10-27T10:54:11-07:00",
          "tree_id": "67c1141503748d135cf7f95906ba35fe5865f18b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/9d4480c22178f65feac93fe9767b7737303f686f"
        },
        "date": 1793125205052,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 729.958,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 725.065,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 729.81,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 732.461,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 68.002,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 54.675,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 66.852,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 86.044,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 22.101,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 15.147,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 19.879,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.139,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 10.186,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 10.019,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 11.202,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.289,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.299,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.271,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.795,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.096,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.744,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 22.03,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 29.455,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 45.032,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.9,
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
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 20,
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
          "id": "01084329ca87b71485bac6406347b1476099fea4",
          "message": "docs: list openai-agents-hyperlight\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-27T21:57:23Z",
          "tree_id": "9911ec08023672e5c453e90b3b8903923870cfe4",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/01084329ca87b71485bac6406347b1476099fea4"
        },
        "date": 1793139129405,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 975.698,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 995.987,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 957.779,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 958.104,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 97.601,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 73.112,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 90.595,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 113.377,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 29.245,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 19.884,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 26.185,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 42.218,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.949,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.795,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.444,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.973,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.032,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.442,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.459,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.549,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 50.478,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 33.416,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 46.117,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 63.911,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.9,
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
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 21,
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
          "id": "3edd6887f972a30599f0e456b866764b5732b473",
          "message": "benchmarks: raise the linux agent cold-snap ceilings for slower runners\n\nMain's agent bench measured cold-snap/hello at 42 ms on one runner and\n35 ms on another, with the same kernel. Raise cold-snap/hello 30 -> 50,\ncold-snap/mount 38 -> 55 and restore-cost/hello 4 -> 5.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-27T22:13:45Z",
          "tree_id": "19370f35ea086a7a30d9dfd7d5e92837e00ebc21",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/3edd6887f972a30599f0e456b866764b5732b473"
        },
        "date": 1793139572648,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 986.266,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 981.972,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1018.448,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 995.559,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 93.593,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 84.574,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 92.383,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 120.882,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 29.988,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 21.96,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 27.142,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 44.455,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.406,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13.833,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 15.353,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.934,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.144,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.48,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.84,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.907,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 46.009,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 42.045,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 45.303,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 65.833,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.9,
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
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 20,
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
          "id": "8daaaf0c9c53313eac54b2c677f5c3365138655f",
          "message": "dependabot.yml: widen the Python integrations' ranges with auto\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-27T22:29:28Z",
          "tree_id": "29eb2f7c622d9020504f3bffbf72361f3cbc13bd",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/8daaaf0c9c53313eac54b2c677f5c3365138655f"
        },
        "date": 1793142012919,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 944.371,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 945.009,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 960.909,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 943.227,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 91.205,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 73.205,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 93.297,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 113.552,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 28.955,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 20.368,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 26.887,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 42.768,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.938,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.92,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.707,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.272,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.878,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.387,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.863,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.642,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 45.652,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 33.073,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 43.435,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 65.031,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 102.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 102.9,
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
            "name": "rss/mount",
            "value": 19,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 20,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}