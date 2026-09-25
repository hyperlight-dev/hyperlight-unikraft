window.BENCHMARK_DATA = {
  "lastUpdate": 1790367055107,
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
        "date": 1789239864322,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 972.155,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 974.247,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1127.088,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 49.414,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 32.794,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 258.05,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.483,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.899,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 181.725,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.612,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.524,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.976,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.767,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.254,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 43.422,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.049,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 14.244,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 302.463,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1789242462225,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1033.565,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1045.143,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1216.559,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 51.9,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.978,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 282.108,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.337,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 10.086,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 194.231,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.809,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.809,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.898,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.783,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.327,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.136,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.447,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.406,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 324.259,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1789244386650,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1080.855,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1132.887,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1296.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 51.906,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 41.715,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 272.403,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.495,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 10.15,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 190.58,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.814,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.793,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.885,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.707,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.377,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.077,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.961,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.938,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 331.868,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1789244848817,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1080.214,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1053.175,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1197.159,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 54.16,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.919,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 278.972,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.218,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 9.864,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 196.163,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.774,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.687,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 11.617,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.732,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.351,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.14,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 38.041,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.044,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 442.791,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1789247185168,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1043.49,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1040.474,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1197.764,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 51.693,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.956,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 273.287,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.314,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 9.701,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 188.559,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.798,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.686,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.725,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.712,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.331,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.011,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.265,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.665,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 327.656,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1789247664174,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 754.913,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 754.013,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 875.617,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 37.532,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 25.621,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 198.891,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.873,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.273,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 140.576,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.401,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.281,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.52,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.197,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.24,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.558,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.812,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.737,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 232.108,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1789945505430,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1023.817,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1025.284,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1230.116,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 51.314,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.065,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 306.4,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.139,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.406,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 205.54,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 14.606,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 14.728,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.631,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.772,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.468,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 51.166,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.253,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.73,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 452.528,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 25,
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
        "date": 1790100128878,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 981.541,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 981.95,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1136.891,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 49.683,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.331,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 258.643,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.389,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.198,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 177.293,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 14.445,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 14.179,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 15.698,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.84,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.512,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 43.782,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 16.916,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.044,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 294.872,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1790118253947,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1054.437,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1053.337,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1238.826,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 53.119,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.605,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 283.194,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.028,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.755,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 189.738,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.246,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.243,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 16.35,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.877,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.705,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.022,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.855,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 329.094,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1790121618375,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1471.16,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1476.393,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1659.323,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 70.719,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 50.055,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 339.83,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 15.046,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.11,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 219.991,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 16.37,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 16.231,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 17.939,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 4.635,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.597,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 50.987,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 21.226,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.562,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 347.419,
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
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 26,
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
        "date": 1790138034970,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1026.354,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1024.83,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1027.651,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1210.203,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 53.066,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.429,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.138,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 278.117,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.722,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.489,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.308,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 186.929,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.31,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.003,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 16.516,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 16.406,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.748,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.511,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.782,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.428,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.758,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.652,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.518,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 319.776,
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
            "name": "snapshot-size/mount",
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
            "value": 14,
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
            "value": 26,
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
        "date": 1790236186360,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1032.046,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1033.302,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1049.383,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1215.276,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 53.358,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.666,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 53.042,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 277.417,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.901,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.749,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.03,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 190.699,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.602,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.524,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 17.209,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 16.659,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.02,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.551,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.772,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.318,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.975,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.771,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.297,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 320.397,
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
            "name": "snapshot-size/mount",
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
            "value": 14,
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
            "value": 26,
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
        "date": 1790367052504,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 1180.466,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 1199.067,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 1182.673,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 1263.435,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.266,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 41.237,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.485,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 167.015,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.064,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.049,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.251,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.426,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 15.629,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 15.489,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 17.137,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 16.055,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.87,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.575,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.506,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.521,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.074,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.023,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.596,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 143.734,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 83.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 83.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 83.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 83.5,
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
            "value": 21,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}