window.BENCHMARK_DATA = {
  "lastUpdate": 1789945369665,
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
        "date": 1789247070392,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 626.29,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 638.313,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 657.173,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 18.244,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.458,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.766,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.66,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.979,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.88,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.61,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.603,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.638,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.518,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.178,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.128,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.926,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.906,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.534,
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
          "id": "15a18a414bf59270698af375e24e5f942c365216",
          "message": "cargo: add required crates.io metadata (description, license, repository, readme)\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T21:10:44Z",
          "tree_id": "4092aa4c6bedbd45a68c5b62f3aedab79635ab16",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/15a18a414bf59270698af375e24e5f942c365216"
        },
        "date": 1789247587650,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 614.76,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 613.715,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 625.625,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.537,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.732,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.962,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.121,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.511,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.115,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.595,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.569,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.709,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.537,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.13,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.243,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 14.009,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.971,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.273,
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
        "date": 1789945368463,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 496.779,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 490.794,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 498.65,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.521,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.763,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.294,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.66,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.33,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.051,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.125,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.145,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.136,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.626,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.15,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.503,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.535,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.753,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.648,
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