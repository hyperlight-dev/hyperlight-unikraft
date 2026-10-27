window.BENCHMARK_DATA = {
  "lastUpdate": 1793125177218,
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
        "date": 1790459320223,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 543.235,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 533.299,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 537.727,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 591.005,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 55.791,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.744,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.864,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 162.633,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.224,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.03,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.214,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.186,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.293,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.129,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.652,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.94,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.001,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.424,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.46,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.375,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.979,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.095,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.003,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 142.646,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
        "date": 1790664518140,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 652.816,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 655.795,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 663.571,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 725.737,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 68.113,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 42.133,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 62.957,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 190.123,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.569,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.578,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.636,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 85.733,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 11.415,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 11.119,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 12.941,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.25,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.845,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.393,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.431,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.867,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 23.922,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.946,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.397,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 157.607,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "distinct": false,
          "id": "44ee170f171ed387cbeb530775edb913bbf687aa",
          "message": "examples, tests: check musl condition variables and futex requeues\n\nTwo C programs, built with musl in Alpine (glibc's condition variables\ndo not requeue): condvar broadcasts to four waiters fifty times and\nexpects every one to wake, and futex_requeue checks FUTEX_REQUEUE and\nFUTEX_CMP_REQUEUE against Linux's semantics, which it passes on Linux\ntoo.  Every wait has a deadline, so a kernel that loses a waiter fails\nthe test rather than hanging it, and the tests print which check failed.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T14:23:19Z",
          "tree_id": "c75fe4aa7f972e6f548fa3f3b9fb7a0e6026d448",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/44ee170f171ed387cbeb530775edb913bbf687aa"
        },
        "date": 1790954032818,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 526.506,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 521.258,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 524.614,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 574.588,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 54.962,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.355,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.262,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 158.147,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.026,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.945,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.964,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 77.557,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.45,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.263,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.616,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.794,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.813,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.45,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.494,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.366,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 24.383,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.988,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.012,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 143.465,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "57a491cd8c38c154e46fb7c78713a4bd9cf31a0c",
          "message": "ci: publish main's builds as the dev pre-release\n\nEach push to main whose CI passes publishes that commit's images, builds hluk for the four release targets pinned to them, moves the dev tag and replaces the dev pre-release, then deletes the dev images of all but this build and the one before.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T15:35:52Z",
          "tree_id": "9225836f73e02859f996924c72fa772985e99809",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/57a491cd8c38c154e46fb7c78713a4bd9cf31a0c"
        },
        "date": 1790955604819,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 645.29,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 643.999,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 648.245,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 694.493,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 66.657,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 40.914,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 60.805,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 186.774,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.228,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.468,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.203,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.992,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 11.321,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 11.163,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 12.769,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 11.785,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.662,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.441,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.23,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 41.557,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 23.531,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.426,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 16.564,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 143.313,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "66d6f70039271bdfb443f9273dc0f285a341563c",
          "message": "README: say how hluk is pronounced\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T17:16:20Z",
          "tree_id": "989ffe030b778bd802adae626678a846769c2384",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/66d6f70039271bdfb443f9273dc0f285a341563c"
        },
        "date": 1790961622955,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 546.898,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 537.884,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 543.256,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 597.944,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 57.166,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.778,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.325,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 162.025,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.637,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.765,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.629,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.617,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.185,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.019,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.809,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.024,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.42,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.474,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.091,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.857,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.134,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.508,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 151.394,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "146c71d599ce601df254149340486f5f532bb563",
          "message": "site: match hyperlight.org\n\nThe site now reads as part of hyperlight.org: its colors (Starlight's\nblue-gray neutrals and blue accent, in light and dark), a header like\nits nav bar that links back to it, and the Hyperlight logo for the mark\nand the favicon in place of a project-specific one. hyperlight.org takes\nthis site's fonts in turn, so the two match both ways.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T23:27:04Z",
          "tree_id": "03011ad5ad3b3f629476eda7584e89ce79c692a2",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/146c71d599ce601df254149340486f5f532bb563"
        },
        "date": 1790983858634,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 535.903,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 533.129,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 528.611,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 586.963,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 56.296,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.199,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.571,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 159.76,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.501,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.014,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.887,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.343,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.39,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.138,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.483,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.546,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.006,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.424,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.42,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.085,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 25.548,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.173,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.598,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 141.696,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "3df47f64f99229e3cebef07b22ba948c69e1398c",
          "message": "site: stop narrow phones from scrolling sideways\n\nAt 375px the template picker made the page 4px wider than the screen,\nand at 320px the platform table did, by 11px. The template grid's one\ncolumn can now shrink below its content, and below 340px the table\nbreaks \"Hypervisor.framework\" rather than widening the page.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T23:41:32Z",
          "tree_id": "07515b9b26e279dec1882c4b82f82d6b214986af",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/3df47f64f99229e3cebef07b22ba948c69e1398c"
        },
        "date": 1790984688321,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 524.097,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 519.875,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 529.178,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 577.359,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 54.892,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.743,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 50.242,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 158.311,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 15.904,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.076,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.104,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 78.694,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.295,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.272,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.615,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.809,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.872,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.415,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.519,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.552,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 24.801,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.935,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.209,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 140.429,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
        "date": 1792006500164,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 371.772,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 368.955,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 376.254,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 405.117,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 40.518,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 25.311,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 36.577,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 109.742,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.292,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.372,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.311,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 49.912,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.495,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.54,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.387,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.39,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.742,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.223,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.33,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 24.386,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 15.497,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.585,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 11.324,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 76.587,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
        "date": 1792012580408,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 737.176,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 740.53,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 732.711,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 788.18,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 75.1,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 47.747,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 76.012,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 209.347,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.479,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.351,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 18.652,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 96.956,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.618,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.561,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 16.72,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.931,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.356,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.42,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.391,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 51.7,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 30.153,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.221,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 36.849,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 162.368,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "distinct": false,
          "id": "cf458a07ed847f06d78284998a5beb13dc9fb426",
          "message": "hluk: keep no guest output in the CLI\n\nhluk run, hluk snapshot and hluk bench echo the guest's output as it is\nwritten and never drain it, yet kept all of it: a long-running guest\nthat kept printing grew the process without end (200 MiB of output\npeaked at 288 MB RSS, 80 MB now).  Every CLI boot now goes through one\nhelper that sets output_limit(0); the output still reaches the terminal.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-14T20:59:52Z",
          "tree_id": "1d4e4ff87b2a7f7b79fefdd62bc0c1efd249c12b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/cf458a07ed847f06d78284998a5beb13dc9fb426"
        },
        "date": 1792014013024,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 357.017,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 357.68,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 353.435,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 379.736,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 37.809,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 22.736,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 34.097,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 98.95,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.162,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.063,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 7.813,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 44.904,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.278,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.023,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.128,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.365,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.746,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.181,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.336,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.703,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 14.953,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.334,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 10.583,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 70.168,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "5624423bf19e44e31acd2787a9627cfff0f8e72d",
          "message": "README: a shell script and a shell REPL in python-shell\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T16:58:11Z",
          "tree_id": "9a8ca7da9cfb0ed6263af8129450f9cb85ab968a",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/5624423bf19e44e31acd2787a9627cfff0f8e72d"
        },
        "date": 1792083915585,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 524.509,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 519.697,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 528.314,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 596.395,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 55.527,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 34.228,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 51.446,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 165.304,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.072,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.957,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.02,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 86.65,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.314,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.186,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 13.772,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 14.105,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.974,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.359,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.335,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 50.55,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 24.605,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.257,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.299,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 245.031,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "50a2e4593f3a9d2062fc0da091896f88a713f292",
          "message": "ci: enable Dependabot version updates\n\nMonthly grouped updates for cargo, GitHub Actions and the Dockerfiles'\nbase images. Docker gets digest refreshes and patch releases only; rust\nand powershell images, Hyperlight minors and majors, and the local hluk-*\nrootfs images are left to be bumped by hand. Majors outside the wasmtime\ngroup and typos come as their own pull requests.\n\nFixes #135\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T17:09:56Z",
          "tree_id": "f5920b16c41cd2490c0da38cd837c961e3430f19",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/50a2e4593f3a9d2062fc0da091896f88a713f292"
        },
        "date": 1792085383509,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 532.626,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 530.948,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 535.077,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 589.981,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 58.119,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.745,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 53.726,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 165.154,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.997,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.027,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.67,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.674,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.551,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.438,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.491,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.228,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.064,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.408,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.413,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.381,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.52,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.148,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.449,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 144.476,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "e5aaa2fbf54c8bad953ab97ecc36e5dcdd51e27a",
          "message": "CHANGELOG: Rust 1.99, the static arm64 wasmtime image\n\nFixes #129\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T17:38:20Z",
          "tree_id": "d00bd1550d12b4b9f20b74cf8c28c62671bec5d8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/e5aaa2fbf54c8bad953ab97ecc36e5dcdd51e27a"
        },
        "date": 1792087319617,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 523.601,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 515.391,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 529.039,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 576.115,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 57.253,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.541,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 52.289,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 158.23,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.893,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.185,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.392,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 77.87,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.75,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.639,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.272,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.976,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.004,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.435,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.574,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.245,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.127,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.142,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.578,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 142.327,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "731b525bd222d982bcd1493505df2ec31454e188",
          "message": "Merge pull request #148 from hyperlight-dev/dependabot/github_actions/actions-7c9a00d418\n\nbuild(deps): bump the actions group across 1 directory with 3 updates",
          "timestamp": "2026-10-15T11:20:20-07:00",
          "tree_id": "f1833ee302c2466c2e3c932c0d76937b9173dac6",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/731b525bd222d982bcd1493505df2ec31454e188"
        },
        "date": 1792089566946,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 380.936,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 396.519,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 377.368,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 396.813,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 42.982,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 24.982,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 37.378,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 104.251,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.81,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.418,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.831,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 46.994,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.8,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.617,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.761,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 8.182,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.841,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.19,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.325,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 23.595,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.975,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.587,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.102,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 72.589,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
          "id": "39a850213d054eff389dbc1fae173accf8c86021",
          "message": "Merge pull request #149 from hyperlight-dev/dependabot/github_actions/dtolnay/rust-toolchain-e2a55d2ffb04f378e9626c28d38b36d230d1e12f\n\nbuild(deps): bump dtolnay/rust-toolchain from 02cb101ec7c40f2c49e1d9714d64511d8e1b74de to e2a55d2ffb04f378e9626c28d38b36d230d1e12f",
          "timestamp": "2026-10-15T11:48:52-07:00",
          "tree_id": "bde23d89676bc7c70bbba15443e128be8ada7032",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/39a850213d054eff389dbc1fae173accf8c86021"
        },
        "date": 1792091310764,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 382.269,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 377.885,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 382.073,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 419.136,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 43.013,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 25.799,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 37.986,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 115.746,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.2,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.32,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.681,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 57.072,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.286,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.129,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.336,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.564,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.125,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.286,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.664,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.689,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.028,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.215,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 11.924,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.579,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 73.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 73.6,
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
        "date": 1792093964741,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 401.786,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 395.649,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 403.329,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 453.077,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.352,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.467,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 39.958,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 121.967,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.478,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.504,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.398,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 60.635,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.551,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.293,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.455,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.148,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.202,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.291,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.79,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 34.629,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.484,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.422,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.239,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 120.252,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "0414e5a04fdc200302c3031977cf48903e620507",
          "message": "conformance: skip test_gc until a sleeping thread wakes on time (#157)\n\ntest_trashcan_threads sleeps 1 s in its main thread while two threads allocate in a loop of short sleeps.  The main thread wakes late or not at all, and the guest runs out of memory: intermittent on main (Windows), every time with this branch's kernel, whose change shifts the timing without touching sleeping or scheduling.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T22:19:47Z",
          "tree_id": "22a21e9b722c6abe7e20755744c3a51e7f634f2b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/0414e5a04fdc200302c3031977cf48903e620507"
        },
        "date": 1792106042798,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 415.878,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 412.639,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 405.45,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 443.627,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 45.633,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 28.027,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.583,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 120.135,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.818,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.64,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.487,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 60.107,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.862,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.397,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.497,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.054,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.808,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.298,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.976,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 34.066,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 20.07,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.409,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.072,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 108.402,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "7ed4eb137c6285f8c478ff87d55441969a797490",
          "message": "tests: retry the HTTP GET examples when the server fails\n\ndotnet_jit_http_get, java_http_get, node_http_get and python_http_get\nfetch httpbin.org, which answers 502 often enough to fail CI on pull\nrequests that have nothing to do with networking.  A 5xx is the\nservice's failure, not the guest's, so the example now runs again, up\nto three times, 2 s then 5 s apart, each time in a fresh sandbox (a\nreused one keeps the failed attempt's sockets).  A run that fails any\nother way, or gets any other status, still fails the test at once: a\nDNS or connection error exercises the guest's resolver and network\nstack, which an outside request is there to test.  The output keeps\nevery attempt, and a retry is a CI warning, so retries that become the\nrule are seen.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:33:56Z",
          "tree_id": "f8e915338b960b068abc60350203c611e711ceec",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/7ed4eb137c6285f8c478ff87d55441969a797490"
        },
        "date": 1792108001168,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 557.549,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 555.982,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 555.666,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 600.736,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.078,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.588,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.543,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 164.312,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.273,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.307,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.165,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.048,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.852,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.635,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.268,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.26,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.826,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.45,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.568,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.592,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.046,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.165,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.94,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 148.047,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "78e0b8e182040f5ba7eaa51342ce0e2508d424e1",
          "message": "CHANGELOG: Dependabot version updates, ci-passed waiting for arm64\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:43:36Z",
          "tree_id": "1a20c20e1116abff820839863e1572fac775de9c",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/78e0b8e182040f5ba7eaa51342ce0e2508d424e1"
        },
        "date": 1792108819821,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 556.279,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 557.914,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 558.085,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 623.828,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.824,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.902,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.224,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 166.761,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.714,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.532,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.775,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 83.003,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.033,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.888,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.611,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.566,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.036,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.455,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.722,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 46.071,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.879,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.282,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.62,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 154.27,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792110093035,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 364.208,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 354.119,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 365.613,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 389.83,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 40.603,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 24.767,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 35.878,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 101.242,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.16,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.089,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.602,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 45.352,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 7.513,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 6.997,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.386,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.788,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.667,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.253,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.353,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.244,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 16.078,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.519,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.752,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 71.872,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792111335704,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 549.763,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 546.026,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 553.35,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 603.415,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.74,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.955,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.881,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 163.138,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.057,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.282,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.869,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.887,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.805,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.67,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.095,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.247,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.957,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.452,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.579,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.476,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.88,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.462,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.052,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 147.974,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "a2ad7ab464bbd46b49798452281b848e47f3cea5",
          "message": "release: v0.18.0\n\nBump the crate to 0.18.0 and add the changelog's entries under\n[v0.18.0], keeping an empty Unreleased section.  0.18.0 rather than\n0.17.1: the host transport moves to Hyperlight 0.18's virtqueues, the\nhost functions changed (HostWrite, HostWriteErr), and snapshots and\nnative kernels built for 0.17 are refused.  The snapshot contract was\nalready bumped with those changes; the kernel moved too, which rolls\nthe snapshot key by itself.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-16T00:22:52Z",
          "tree_id": "3a351f900d4e88ce267f6dd00a34cd41f2385049",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a2ad7ab464bbd46b49798452281b848e47f3cea5"
        },
        "date": 1792112314861,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 544.325,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 544.775,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 547.161,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 616.553,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.144,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.788,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.535,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 162.957,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.702,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.307,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.128,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.848,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.615,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.557,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.21,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.08,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.383,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.419,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.958,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.255,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.158,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.347,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.467,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 144.071,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792395630882,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 398.675,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 397.578,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 400.639,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 440.646,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.664,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 26.972,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.596,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 117.212,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.953,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.556,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.454,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 59.076,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.301,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.242,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.474,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.843,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.166,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.281,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.775,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.692,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.175,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.407,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 16.584,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 112.133,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "id": "e1aa358ed7a3ddf787b46d9b676277000c75fcf1",
          "message": "integrations/smolagents: release 0.1.1\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-19T13:13:38Z",
          "tree_id": "bce20460e72585f23122a2282bddaca743187a36",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/e1aa358ed7a3ddf787b46d9b676277000c75fcf1"
        },
        "date": 1792415836190,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 557.295,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 554.921,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 567.197,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 620.893,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 62.576,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.781,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.335,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 167.697,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.43,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.531,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.949,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 83.007,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.111,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.863,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.733,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.448,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.136,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.431,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.594,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.362,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.618,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.221,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.821,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 143.72,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597",
          "message": "README, CHANGELOG, docs/RELEASE.md: mcp-server-hyperlight\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T06:56:43Z",
          "tree_id": "d76923ae19bdf0b4c4b4bd38aec58c5340d50e04",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597"
        },
        "date": 1792480342440,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 544.73,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 539.172,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 546.082,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 608.071,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.522,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.885,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.996,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 163.052,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.837,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.076,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.49,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.797,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.999,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.545,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.481,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.276,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.084,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.463,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.586,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.809,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.46,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.082,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 24.407,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 146.465,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "dcb8b5010494132b990c75525c5dd4c6a29bbe01",
          "message": "CHANGELOG: Windows arm64 CI\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T08:13:51Z",
          "tree_id": "4c13bd809aac1b0ac04baedecd9575615c8eb1eb",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/dcb8b5010494132b990c75525c5dd4c6a29bbe01"
        },
        "date": 1792485330578,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 549.502,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 545.474,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 554.112,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 607.144,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.14,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.903,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.492,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 166.24,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.684,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.397,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.918,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.246,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.685,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.636,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.168,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.074,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.938,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.425,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.76,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.282,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.604,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.405,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.18,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 144.554,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792555617621,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 412.924,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 400.678,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 405.177,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 438.289,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 45.407,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.894,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.028,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 117.847,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.512,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.75,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.694,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 58.976,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.8,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.531,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.605,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.003,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.213,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.298,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.81,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 34.136,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.595,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.611,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 13.359,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 111.917,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792556455089,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 549.573,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 537.387,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 548.344,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 621.745,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.756,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.618,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.092,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 161.94,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.051,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.193,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.894,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.084,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.703,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.486,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.098,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.188,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.894,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.404,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.536,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 46.389,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.896,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.23,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.168,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 151.335,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792562807123,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 576.298,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 573.345,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 578.981,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 634.97,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.38,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 40.159,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.141,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 171.413,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.014,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.886,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.552,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 83.793,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.297,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13.222,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.889,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.617,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.127,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.47,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.808,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.616,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.18,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.505,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 24.862,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 180.667,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
          "distinct": false,
          "id": "f5bcafc73642f096bec5468a9c73f1298de89926",
          "message": "CHANGELOG, docs/RELEASE.md: MCP Registry and Desktop Extension releases\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-21T06:06:16Z",
          "tree_id": "63d8957dc45c01ebfdc0e04feb8dec8ab2489ed8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f5bcafc73642f096bec5468a9c73f1298de89926"
        },
        "date": 1792563749448,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 570.41,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 554.531,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 566.746,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 618.201,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.828,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.88,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.973,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 168.258,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.989,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.627,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.427,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 83.761,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.276,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.872,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.203,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.694,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.146,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.445,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.632,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.601,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.599,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.581,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.401,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 146.108,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792608047633,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 398.93,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 396.008,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 397.519,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 438.87,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.946,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.363,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 39.738,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 116.8,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.955,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.543,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.398,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 58.578,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.365,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.212,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.31,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 9.763,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.169,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.295,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.76,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.634,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.258,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.034,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.58,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 109.362,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792610217319,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 556.172,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 556.56,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 563.598,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 618.02,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.378,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.83,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.074,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 167.766,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.192,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.561,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.994,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.914,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.022,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.665,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.463,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.012,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.466,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.58,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.955,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.376,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.293,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.733,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 149.313,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792624023697,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 536.876,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 539.054,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 545.673,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 603.143,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.47,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.06,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 54.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 163.822,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.922,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.182,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.859,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.551,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.816,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.512,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.009,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.245,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.986,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.421,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.755,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.006,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.819,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.259,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.83,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 142.112,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792624658337,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 552.951,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 551.798,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 557.27,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 609.374,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.935,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.716,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.338,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 166.421,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.737,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.539,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.153,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.325,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.682,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.705,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.11,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.264,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.022,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.452,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.575,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.624,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.489,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.619,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.509,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 142.408,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
            "value": 22,
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
        "date": 1792627368499,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 574.883,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 540.697,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 546.184,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 604.185,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 67.009,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.687,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 53.781,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 163.691,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.589,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.368,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.866,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.474,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.071,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.611,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.193,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.187,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.109,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.454,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.491,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.392,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.511,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.08,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.554,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 145.439,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
        "date": 1792630932059,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 417.987,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 421.605,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 425.156,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 452.006,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 46.015,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 29.117,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 42.332,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 121.193,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.438,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.024,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.399,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 59.89,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 9.908,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 9.785,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 10.972,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 10.269,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.259,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.306,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.845,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 34.118,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.1,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.719,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.583,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 106.744,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.1,
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
        "date": 1792685297460,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 558.675,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 555.247,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 559.299,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 629.259,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.491,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.137,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 57.45,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 169.61,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.687,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.473,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.363,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 83.969,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.975,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.639,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.401,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.533,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.099,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.477,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.397,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 50.696,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.262,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.603,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.462,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 197.774,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792692780109,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 556.287,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 549.144,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 549.522,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 609.222,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.418,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.439,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.074,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 164.014,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.376,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.37,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.305,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.685,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.921,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.532,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.361,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.417,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.008,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.446,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.493,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.422,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.666,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.143,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.977,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 144.391,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792701422147,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 546.609,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 547.275,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 559.176,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 598.784,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.348,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.347,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.915,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 162.393,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.301,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.431,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.304,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.244,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.875,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.51,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.263,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.14,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.051,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.445,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.481,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.044,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.69,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.354,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 24.697,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 149.833,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792704096353,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 553.81,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 551.451,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 554.46,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 611.709,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.769,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.533,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.177,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 168.916,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.762,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.594,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.444,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 82.811,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.085,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 13.009,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.473,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.521,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.143,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.47,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.632,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.756,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.672,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.191,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.936,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 143.358,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
        "date": 1792704503435,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 549.82,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 551.277,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 551.82,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 604.657,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.973,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.086,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.637,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 160.334,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.196,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.145,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.342,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.853,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.642,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.505,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.341,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.059,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.03,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.417,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.535,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.922,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.836,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.238,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.9,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 145.507,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
        "date": 1792718668971,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 557.396,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 559.124,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 563.275,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 613.834,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.081,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 38.293,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.518,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 165.045,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.199,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.339,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.066,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.268,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.766,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.674,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.218,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.293,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.026,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.477,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.522,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.275,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.784,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.508,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.03,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 142.446,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792723194374,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 554.798,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 561.976,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 557.136,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 604.542,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.7,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.679,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.901,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 164.388,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.827,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.476,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.319,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.904,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.058,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.799,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.329,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.352,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.302,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.491,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.524,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.411,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.403,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.359,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.461,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 147.459,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
        "date": 1792725532978,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 550.896,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 557.961,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 586.502,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 619.945,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 58.834,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.897,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.16,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 162.694,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.172,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.283,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.181,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.802,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.726,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.596,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.25,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.996,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.034,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.431,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.053,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.141,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.311,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.707,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.334,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 145.699,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792790258857,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 547.048,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 542.807,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 544.864,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 592.543,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 60.483,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.633,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 55.114,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 163.221,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.488,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.802,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.106,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 79.885,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.548,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.523,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.109,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 12.851,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.535,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.435,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.32,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.874,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.718,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.507,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 141.14,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792799546019,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 579.957,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 554.593,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 547.427,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 596.528,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 96.477,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.7,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.041,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 164.573,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.635,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.772,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.311,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.564,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.965,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.545,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.13,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.049,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.126,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.45,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.486,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.039,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.2,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.849,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.514,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 146.669,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792801310606,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 548.735,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 546.383,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 557.561,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 604.815,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.777,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.72,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.696,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 169.347,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.766,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.152,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 82.399,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.733,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.896,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.694,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.452,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.939,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.493,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.613,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.959,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.265,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.049,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.573,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 148.469,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792802004015,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 393.212,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 379.05,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 373.114,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 399.394,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 43.34,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 26.606,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 37.268,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 104.62,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.716,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.647,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.87,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 46.162,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 8.039,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 7.279,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 8.515,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 7.652,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.894,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.232,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.319,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.912,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.384,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.31,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.477,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 71.14,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1792995427687,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 551.645,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 548.634,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 551.092,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 597.052,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.898,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 39.942,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 56.092,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 163.202,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.934,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.95,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.471,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 80.81,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 12.825,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.598,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.221,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.148,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.992,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.488,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.455,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.179,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.097,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.949,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.698,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 142.434,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
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
        "date": 1793125173202,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 567.215,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 561.29,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 567.256,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 624.601,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 63.186,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 40.684,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 58.547,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 169.423,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.01,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.349,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.166,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 81.429,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 13.136,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 12.97,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 14.499,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 13.408,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.978,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.44,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.653,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 45.514,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.299,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.682,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.942,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 150.342,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 74.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 74.2,
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
            "value": 22,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}