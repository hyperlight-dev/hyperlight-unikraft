window.BENCHMARK_DATA = {
  "lastUpdate": 1790138080420,
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
      }
    ]
  }
}