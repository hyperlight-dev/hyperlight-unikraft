window.BENCHMARK_DATA = {
  "lastUpdate": 1793148433260,
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
        "date": 1790100023706,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 614.143,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 611.81,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 621.079,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.552,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.851,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.785,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.134,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.474,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.917,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.527,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.325,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.685,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.617,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.254,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.259,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.254,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.439,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.641,
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
        "date": 1790118147591,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 640.446,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 637.043,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 650.076,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 18.737,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.207,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.754,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.689,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.504,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.109,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.927,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.564,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.955,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.724,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.201,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.988,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.299,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.512,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.993,
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
        "date": 1790121472087,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 487.419,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 488.172,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 495.587,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.735,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.878,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 26.628,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.649,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.308,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.7,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.191,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.097,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.642,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.639,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.171,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.079,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.214,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.715,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.117,
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
        "date": 1790137924320,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 608.867,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 607.137,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 608.901,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 621.141,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.877,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.134,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.012,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.709,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.173,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.526,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.153,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.933,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.582,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.544,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.294,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.818,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.667,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.243,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.791,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.198,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.417,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.358,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.856,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.879,
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
        "date": 1790236032850,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 610.326,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 607.645,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 609.032,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 618.281,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.409,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.353,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.433,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.689,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.178,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.523,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.124,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.904,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.745,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.557,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.933,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.877,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.556,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.273,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.776,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.125,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.199,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.379,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.67,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.515,
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
        "date": 1790366915540,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 616.135,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 614.259,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 614.936,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 629.252,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 18.093,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.43,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.194,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.355,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.108,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.532,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.195,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.975,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.868,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.68,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.19,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.074,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.588,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.253,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.772,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.332,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.356,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.491,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.96,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.501,
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
        "date": 1790459242915,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 349.319,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 346.591,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 348.195,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 358.281,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.701,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.887,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 9.968,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.428,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.994,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.291,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.567,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.253,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.799,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.764,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.104,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.83,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.018,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.101,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.508,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.566,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.847,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.84,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.597,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.723,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1790664376592,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 350.575,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 345.765,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 345.037,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 355.359,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.81,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.501,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.065,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.37,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.011,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.333,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.48,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.32,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.781,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.75,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.051,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.838,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.998,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.124,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.535,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.525,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.777,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.944,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.61,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.078,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1790953939352,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 347.957,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 346.32,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 346.902,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 356.24,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.868,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.879,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.298,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.268,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.067,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.296,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.5,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.241,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.78,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.756,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.049,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.839,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.091,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.123,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.48,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.558,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.844,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.901,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.602,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.073,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1790955461056,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 333.457,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 322.219,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 329.612,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 333.871,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.319,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.307,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.382,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.001,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.794,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.318,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.644,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.065,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.795,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.764,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.122,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.864,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.645,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.084,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.462,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.966,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.317,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.863,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.677,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.386,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1790961496306,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 358.169,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 356.771,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 358.015,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 368.798,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 11.795,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.262,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 9.177,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.597,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.796,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.362,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.562,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.206,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.791,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.751,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.078,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.872,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.002,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.114,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.487,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.947,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.247,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.734,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.479,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 20.223,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1790983777428,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 432.698,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 431.06,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 430.79,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 443.121,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.447,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.121,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.384,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.035,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.1,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.642,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.085,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.168,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.929,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.907,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.257,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.027,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.602,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.172,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.696,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.25,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.281,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.464,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.877,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.75,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1790984603000,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 445.432,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 443.205,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 442.695,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 454.055,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.053,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.657,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.746,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.334,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.583,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.646,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.129,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.169,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.012,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.997,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.355,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.05,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.702,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.133,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.667,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.109,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.042,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.366,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.614,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.188,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792006385354,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 353.366,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 349.14,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 348.987,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 362.969,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.917,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.887,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.628,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.792,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.115,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.332,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.539,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.427,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.807,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.764,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.095,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.871,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.088,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.13,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.494,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.569,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.9,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.864,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.806,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.2,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792012458774,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 447.772,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 447.613,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 445.345,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 462.855,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.209,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.647,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.175,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.46,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.627,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.273,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.319,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.094,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.961,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.39,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.118,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.677,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.167,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.749,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.372,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.417,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.381,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.962,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.801,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792013967253,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 437.141,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 438.322,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 440.876,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 448.469,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.85,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.983,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.519,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.04,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.274,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.648,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.157,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.925,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.999,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.942,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.298,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.014,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.637,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.17,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.704,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.166,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.527,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.515,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.879,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.606,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792083797242,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 442.956,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 444.668,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 441.831,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 453.302,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.422,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.564,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.961,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.33,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.577,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.702,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.183,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.094,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.063,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.001,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.376,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.058,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.599,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.103,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.659,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.99,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.164,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.361,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.694,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.867,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792085274955,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 440.727,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 442.021,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 441.337,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 455.661,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.5,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.475,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.65,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.335,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.325,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.631,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.348,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.035,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.116,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.015,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.49,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.136,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.614,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.187,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.703,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.181,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.568,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.643,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.241,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.847,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792087090273,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 316.157,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 312.622,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 317.299,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 334.099,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.144,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.525,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.664,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.748,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.712,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.373,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.627,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.05,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.853,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.832,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.204,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.98,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.586,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.071,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.416,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.096,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.794,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.162,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.702,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.361,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792089558141,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 450.621,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 448.239,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 450.998,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 456.356,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.804,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.255,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.995,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.168,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.65,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.778,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.483,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.574,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.189,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.081,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.545,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.227,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.618,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.205,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.767,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.47,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.835,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.658,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.283,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.05,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792091217550,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 434.949,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 433.828,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 437.365,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 447.618,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.099,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.344,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.057,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.271,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.302,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.657,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.325,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.927,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.05,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.032,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.456,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.099,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.609,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.191,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.724,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.14,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.637,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.565,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.136,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 22.575,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792093905120,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 444.721,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 441.593,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 442.113,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 451.98,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.36,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.543,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.112,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.357,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.254,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.592,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.279,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.207,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.097,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.073,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.516,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.141,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.666,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.141,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.786,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.218,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.556,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.52,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.191,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.436,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792105939205,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 456.193,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 453.642,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 455.624,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 465.97,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.352,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 30.164,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.568,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.701,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.385,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.274,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.213,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.164,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.616,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.265,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.522,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.2,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.734,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.937,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.353,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.512,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.052,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.821,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792107899960,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 342.332,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 333.969,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 327.214,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 340.945,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.824,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.676,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.522,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.248,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.747,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.356,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.708,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.761,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.894,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.904,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.281,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.06,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.634,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.102,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.508,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.053,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.924,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.892,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.982,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.371,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792108731259,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 407.12,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 407.789,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 405.683,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 416.164,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.69,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.808,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 9.962,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 24.639,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.276,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.325,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.74,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.334,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.943,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.898,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.259,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.181,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.432,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.184,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.632,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.42,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.278,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.142,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.88,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 24.561,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792110033101,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 350.857,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 350.504,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 350.026,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 359.178,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.1,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.247,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.758,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.348,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.036,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.314,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.574,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.535,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.924,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.898,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.251,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.957,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.947,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.1,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.538,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.487,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.962,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.904,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.793,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.867,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792111238026,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 444.495,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 442.15,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 445.089,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 451.944,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.323,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.668,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.07,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.021,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.248,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.635,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.226,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.162,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.108,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.094,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.486,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.159,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.654,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.213,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.76,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.205,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.59,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.65,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.957,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.285,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792112216487,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 444.827,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 443.804,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 444.298,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 452.64,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.871,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.975,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.218,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.814,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.563,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.626,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.375,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.38,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.082,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.075,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.528,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.114,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.644,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.207,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.77,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.336,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.484,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.581,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.121,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.433,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792395544943,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 357.094,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 351.56,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 324.035,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 329.767,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.413,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.619,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.077,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 22.593,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.096,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.351,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.541,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.347,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.013,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.907,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.223,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.963,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.696,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.092,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.447,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.911,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.482,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.873,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.677,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.294,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792415739225,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 513.316,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 526.907,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 527.436,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 535.304,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 20.698,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.963,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 16.846,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.944,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.755,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.008,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 4.266,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.434,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.363,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.257,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.958,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.573,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.104,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.222,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.948,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.954,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.735,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.023,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.163,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.446,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792480322133,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 471.524,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 469.981,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 471.52,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 480.621,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.475,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.15,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.215,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 30.002,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.272,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.941,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.898,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.634,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.432,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.327,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.854,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.529,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.661,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.208,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.75,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.354,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.03,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.881,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.053,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.491,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792485225868,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 430.241,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 424.011,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 427.046,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 424.843,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.644,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.149,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.986,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.392,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.655,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.533,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.142,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.589,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.044,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.067,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.541,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.148,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.349,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.149,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.675,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.134,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.389,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.198,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.495,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.285,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792555356065,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 457.446,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 454.29,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 452.539,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 455.507,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.199,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.042,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.798,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.743,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.633,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.679,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.436,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.265,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.2,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.147,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.479,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.124,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.713,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.218,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.804,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.225,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.771,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.528,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.368,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.442,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792556359049,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 329.18,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 325.786,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 337.655,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 333.763,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.545,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.271,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.181,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.101,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.854,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.345,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.018,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.313,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.89,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.859,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.245,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.894,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.607,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.094,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.245,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.126,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.051,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.173,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.641,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 15.984,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792562696245,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 321.641,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 312.802,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 320.789,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 318.472,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.626,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.239,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.576,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 22.218,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.993,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.207,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.682,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.036,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.931,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.826,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.18,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.932,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.653,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.096,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.508,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.945,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.968,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.702,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.777,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 14.554,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792563648895,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 357.129,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 362.592,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 363.022,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 390.455,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.4,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.998,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.032,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.247,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.815,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.496,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.275,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.851,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.239,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.057,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.577,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.382,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.592,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.099,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.492,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.346,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.295,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.168,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.515,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.692,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792607950733,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 400.604,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 401.444,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 401.455,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 409.43,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.896,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.867,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.512,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 26.132,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.042,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.701,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.426,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.991,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.138,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.068,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.553,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.236,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.952,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.162,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.664,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.714,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.344,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.224,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.566,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.984,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792610137349,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 434.353,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 435.039,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 435.253,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 444.27,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.214,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.848,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.743,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.973,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.145,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.653,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.255,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.171,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.191,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.057,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.475,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.268,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.345,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.146,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.656,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.393,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.526,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.363,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.733,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.78,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792623783944,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 338.697,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 337.677,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 339.57,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 345.134,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.229,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.72,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.832,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.596,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.486,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.607,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.082,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.236,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.178,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.121,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.524,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.244,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.635,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.107,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.46,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.039,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.781,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.082,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.156,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.034,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792624584550,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 445.368,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 441.337,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 443.704,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 452.154,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.798,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.655,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.859,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.749,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.356,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.61,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.399,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.483,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.058,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.042,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.436,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.211,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.691,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.223,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.786,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.211,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.574,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.499,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.387,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.945,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792627266606,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 376.706,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 373.676,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 377.516,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 386.744,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.508,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.629,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.389,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 22.307,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.334,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.512,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.97,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.896,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.991,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.963,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.319,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.026,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.08,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.139,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.588,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.628,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.187,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.079,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.248,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.225,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792630850425,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 449.65,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 448.929,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 450.03,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 456.857,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.453,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.043,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.768,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.907,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.811,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.685,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.398,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.381,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.153,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.05,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.46,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.175,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.719,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.239,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.868,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.238,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.927,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.554,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.4,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.536,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792685280473,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 330.774,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 327.902,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 332.465,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 332.592,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.878,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.72,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.113,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 22.576,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.803,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.285,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.599,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 10.946,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.778,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.708,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.077,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.811,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.551,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.14,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.42,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.618,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.49,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.943,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.921,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 15.963,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792692657716,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 332.603,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 328.719,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 339.254,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 337.833,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.174,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.515,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.222,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.475,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.01,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.381,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.801,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.579,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.894,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.779,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.2,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.901,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.633,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.137,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.219,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.428,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.913,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.199,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.265,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792701321659,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 353.927,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 353.817,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 354.815,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 363.716,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.523,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.169,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.475,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.368,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.072,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.325,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.618,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.583,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.829,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.77,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.084,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.815,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.028,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.197,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.499,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.569,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.349,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.914,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.922,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.863,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792703989676,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 351.984,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 349.771,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 350.976,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 359.863,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.971,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.086,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.55,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.04,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.088,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.276,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.605,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.549,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.786,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.762,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.078,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.807,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.026,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.195,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.581,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.564,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.11,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.92,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.897,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.88,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792704398191,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 312.54,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 309.702,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 311.367,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 314.73,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.651,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.009,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.747,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 22.359,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.609,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.254,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.5,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.089,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.758,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.733,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.053,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.805,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.596,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.129,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.419,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.979,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.289,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.847,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.52,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 15.813,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792718572515,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 392.657,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 386.247,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 393.173,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 389.382,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.3,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.329,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.76,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 24.43,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.335,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.464,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.583,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.868,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.08,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.954,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.09,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.038,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.721,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.149,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.795,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.498,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.949,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.054,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.569,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.21,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792723108116,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 439.051,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 436.912,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 436.674,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 446.756,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.995,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.664,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.123,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.132,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.219,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.634,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.234,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.968,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.957,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.348,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.979,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.686,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.258,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.727,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.29,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.832,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.609,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.225,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.488,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792725428402,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 331.365,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 330.677,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 335.402,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 340.514,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.015,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.889,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.655,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.902,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.351,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.666,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.576,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.828,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.802,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.153,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.969,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.668,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.16,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.495,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.103,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.635,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.921,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.909,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.603,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792790180255,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 442.409,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 439.735,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 443.302,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 451.079,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 24.209,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 18.056,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 21.906,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.463,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.494,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.338,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.76,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.407,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.003,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.992,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.385,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.033,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.654,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.209,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.771,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.223,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 15.625,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.052,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 11.003,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 26.688,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792799526658,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 633.119,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 537.732,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 500.23,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 636.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 248.983,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 217.819,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 245.8,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 261.032,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.147,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.366,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.597,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.956,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.281,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.171,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.517,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.26,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.082,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.178,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.648,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.922,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.586,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.148,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 8.491,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.594,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1792800547210,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 440.471,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 438.25,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 441.102,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 450.229,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 23.932,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 18.245,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 22.18,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.557,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.361,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.442,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.956,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.437,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.004,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.998,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.389,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.056,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.694,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.201,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.682,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.163,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 15.823,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.969,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 11.189,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 26.78,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1792801235654,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 452.344,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 446.882,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 444.768,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 453.957,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 24.066,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 18.028,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 21.897,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.351,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.385,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.484,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.844,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.23,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.059,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.01,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.402,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.084,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.608,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.249,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.803,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.227,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 15.734,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.906,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 10.873,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 26.731,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792801952502,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 452.944,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 455.679,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 467.795,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 473.906,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 25.092,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 19.684,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 24.144,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.422,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.663,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.042,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 7.634,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.942,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.054,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.144,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.618,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.2,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.618,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.229,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.805,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.236,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 16.039,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.509,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 11.743,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.406,
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
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 7,
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
        "date": 1792995309385,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 375.37,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 369.027,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 368.284,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 374.845,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 19.906,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 15.821,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 18.154,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 29.047,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.904,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.265,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.04,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.157,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.003,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.033,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.332,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.002,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.933,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.139,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.588,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.306,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.473,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.695,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 7.787,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.28,
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
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 8,
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
        "date": 1793125085243,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 380.95,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 379.785,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 380.48,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 394.156,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 21.173,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 15.838,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 19.248,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.987,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.837,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.239,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.557,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.271,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.309,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.234,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.644,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.505,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.077,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.145,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.54,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.586,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.254,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.011,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 9.602,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.295,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1793139022427,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 443.242,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 437.009,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 442.455,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 452.027,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 21.013,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 15.8,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 19.111,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.857,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.937,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.26,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.621,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.873,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.29,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.177,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.659,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.396,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.442,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.164,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.622,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.409,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.492,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.667,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 9.556,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 28.198,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1793139513046,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 379.882,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 387.346,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 378.995,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 385.386,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 21.887,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 16.006,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 19.47,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.901,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.387,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.197,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.655,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.79,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.467,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.234,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.66,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.339,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.07,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.129,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.522,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.483,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.984,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.657,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 9.928,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.052,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
        "date": 1793141891126,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 483.855,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 482.179,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 483.234,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 492.968,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 27.005,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 20.205,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 24.491,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 39.694,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.087,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.621,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.298,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 21.437,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.653,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.586,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.116,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.739,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.657,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.185,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.69,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.009,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.029,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.756,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.217,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 29.364,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
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
          "id": "98acb6ce56757acadfa01ad4aac4efccb36bce54",
          "message": "benchmarks: raise the linux python-shell cold-snap ceiling to 25 ms\n\nMain's python-shell bench measured cold-snap/hello at 20.1 ms on one\nrunner, just over the 20 ms ceiling; probes on other runners measured\n15.5 to 23 ms with the same kernel.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-27T23:40:23Z",
          "tree_id": "d1083d19293b0683075b2a81380f307901d9881d",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/98acb6ce56757acadfa01ad4aac4efccb36bce54"
        },
        "date": 1793144549816,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 476.145,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 478.475,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 475.422,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 482.784,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 26.188,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 19.943,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 24.63,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 39.496,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 11.111,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.458,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.124,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.628,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.626,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.511,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.009,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.689,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.695,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.204,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.794,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.197,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.088,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.383,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 13.098,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 29.792,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
            "unit": "MB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b6ff697b57a2bec969925344a51148fedee3917c",
          "message": "build(deps): bump the images group across 3 directories with 1 update (#214)\n\n> [!WARNING]\n> Cooldown could not be applied because no publication date was\navailable from the registry.\n>\n\nBumps the images group with 1 update in the /drivers/dotnet-jit\ndirectory: dotnet/sdk.\nBumps the images group with 1 update in the\n/examples/http-server/kestrel directory: dotnet/sdk.\nBumps the images group with 1 update in the /templates/http-dotnet\ndirectory: dotnet/sdk.\n\nUpdates `dotnet/sdk` from `a4ef339` to `5c27d49`\n\nUpdates `dotnet/sdk` from `a4ef339` to `5c27d49`\n\nUpdates `dotnet/sdk` from `a4ef339` to `5c27d49`\n\n\nDependabot will resolve any conflicts with this PR as long as you don't\nalter it yourself. You can also trigger a rebase manually by commenting\n`@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits\nthat have been made to it\n- `@dependabot show <dependency name> ignore conditions` will show all\nof the ignore conditions of the specified dependency\n- `@dependabot ignore <dependency name> major version` will close this\ngroup update PR and stop Dependabot creating any more for the specific\ndependency's major version (unless you unignore this specific\ndependency's major version or upgrade to it yourself)\n- `@dependabot ignore <dependency name> minor version` will close this\ngroup update PR and stop Dependabot creating any more for the specific\ndependency's minor version (unless you unignore this specific\ndependency's minor version or upgrade to it yourself)\n- `@dependabot ignore <dependency name>` will close this group update PR\nand stop Dependabot creating any more for the specific dependency\n(unless you unignore this specific dependency or upgrade to it yourself)\n- `@dependabot unignore <dependency name>` will remove all of the ignore\nconditions of the specified dependency\n- `@dependabot unignore <dependency name> <ignore condition>` will\nremove the ignore condition of the specified dependency and ignore\nconditions\n\n\n</details>\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-27T16:42:46-07:00",
          "tree_id": "cc1e73d1e7153a5c702021d86b977a4d69323ae1",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/b6ff697b57a2bec969925344a51148fedee3917c"
        },
        "date": 1793146149130,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 468.673,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 468.748,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 469.116,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 478.126,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 25.733,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 19.096,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 23.418,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 37.76,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 10.725,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.276,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 8.049,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.171,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.545,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.492,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.991,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.594,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.618,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.206,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.744,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.225,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.578,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.102,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.821,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 29.21,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
            "unit": "MB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ea3600ab2b2e52b593f6333b6da831fc1b9a4deb",
          "message": "build(deps): bump the actions group across 1 directory with 2 updates (#216)\n\nBumps the actions group with 2 updates in the / directory:\n[Swatinem/rust-cache](https://github.com/swatinem/rust-cache) and\n[pypa/gh-action-pypi-publish](https://github.com/pypa/gh-action-pypi-publish).\n\nUpdates `Swatinem/rust-cache` from 2.9.2 to 2.9.3\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/swatinem/rust-cache/releases\">Swatinem/rust-cache's\nreleases</a>.</em></p>\n<blockquote>\n<h2>v2.9.3</h2>\n<h2>What's Changed</h2>\n<ul>\n<li>Issue-375: stop cleanup timestamp pruning after the first entry by\n<a\nhref=\"https://github.com/TethysSvensson\"><code>@​TethysSvensson</code></a>\nin <a\nhref=\"https://redirect.github.com/Swatinem/rust-cache/pull/377\">Swatinem/rust-cache#377</a></li>\n<li>fix: log rustup toolchain list failure as info instead of warning by\n<a href=\"https://github.com/SDAChess\"><code>@​SDAChess</code></a> in <a\nhref=\"https://redirect.github.com/Swatinem/rust-cache/pull/394\">Swatinem/rust-cache#394</a></li>\n</ul>\n<h2>New Contributors</h2>\n<ul>\n<li><a\nhref=\"https://github.com/TethysSvensson\"><code>@​TethysSvensson</code></a>\nmade their first contribution in <a\nhref=\"https://redirect.github.com/Swatinem/rust-cache/pull/377\">Swatinem/rust-cache#377</a></li>\n<li><a href=\"https://github.com/SDAChess\"><code>@​SDAChess</code></a>\nmade their first contribution in <a\nhref=\"https://redirect.github.com/Swatinem/rust-cache/pull/394\">Swatinem/rust-cache#394</a></li>\n</ul>\n<p><strong>Full Changelog</strong>: <a\nhref=\"https://github.com/Swatinem/rust-cache/compare/v2.9.2...v2.9.3\">https://github.com/Swatinem/rust-cache/compare/v2.9.2...v2.9.3</a></p>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/Swatinem/rust-cache/blob/master/CHANGELOG.md\">Swatinem/rust-cache's\nchangelog</a>.</em></p>\n<blockquote>\n<h1>Changelog</h1>\n<h2>2.9.3</h2>\n<ul>\n<li>Fix timestamp based pruning</li>\n<li>Add missing <code>await</code>s</li>\n<li>Do not emit warning on missing <code>rustup</code></li>\n</ul>\n<h2>2.9.2</h2>\n<ul>\n<li>Fix <code>credentials.toml</code> cleanup</li>\n<li>Improvements to cleanup, preserving more valid targets</li>\n<li>Improvements to <code>cargo install</code> handling</li>\n<li>Correctly sort/dedupe Rust versions</li>\n</ul>\n<h2>2.9.1</h2>\n<ul>\n<li>Fix regression in hash calculation</li>\n</ul>\n<h2>2.9.0</h2>\n<ul>\n<li>Update to <code>node24</code></li>\n<li>Support running from within a <code>nix</code> shell</li>\n<li>Consider all installed toolchains for cache key</li>\n<li>Use case-insensitive comparison to determine exact cache hit</li>\n</ul>\n<h2>2.8.2</h2>\n<ul>\n<li>Don't overwrite env for cargo-metadata call</li>\n</ul>\n<h2>2.8.1</h2>\n<ul>\n<li>Set empty <code>CARGO_ENCODED_RUSTFLAGS</code> when retrieving\nmetadata</li>\n<li>Various dependency updates</li>\n</ul>\n<h2>2.8.0</h2>\n<ul>\n<li>Add support for <code>warpbuild</code> cache provider</li>\n<li>Add new <code>cache-workspace-crates</code> feature</li>\n</ul>\n<h2>2.7.8</h2>\n<ul>\n<li>Include CPU arch in the cache key</li>\n</ul>\n<h2>2.7.7</h2>\n<ul>\n<li>Also cache <code>cargo install</code> metadata</li>\n</ul>\n<h2>2.7.6</h2>\n<!-- raw HTML omitted -->\n</blockquote>\n<p>... (truncated)</p>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/d1527c8f67ef9af357290475d4af4b0fab8f12e0\"><code>d1527c8</code></a>\n2.9.3</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/f0fca6fc1c91b0c4158e7b92bdd2582341575c8e\"><code>f0fca6f</code></a>\nupdate dependencies and rebuild</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/b49b17f98024bf3c4d14fb465de5bad92750344b\"><code>b49b17f</code></a>\nadd missing awaits</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/6a7bc3ebce750e0a16a430dd57485acfdb38b05f\"><code>6a7bc3e</code></a>\nBump the dev-minor group across 1 directory with 2 updates (<a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/383\">#383</a>)</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/4ece996238f932eddbc3099e93131a4089950e4f\"><code>4ece996</code></a>\nBump the actions group across 1 directory with 6 updates (<a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/392\">#392</a>)</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/8103aff431aa12c1132e226b21f6a655ea749803\"><code>8103aff</code></a>\nBump the prd-minor group across 1 directory with 2 updates (<a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/389\">#389</a>)</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/0e361f3528cccdd0ac5d0e0dda2bde9bcfc77a1f\"><code>0e361f3</code></a>\nfix: log rustup toolchain list failure as info instead of warning (<a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/393\">#393</a>)\n(<a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/394\">#394</a>)</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/f0d9c3887740aee45f6153b24b3a6b815192ec16\"><code>f0d9c38</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/380\">#380</a>\nfrom Swatinem/dependabot/github_actions/actions-8ff58...</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/ccd9742ce3913201b7e871cc4b4f95ee3930f15b\"><code>ccd9742</code></a>\nBump the actions group with 2 updates</li>\n<li><a\nhref=\"https://github.com/Swatinem/rust-cache/commit/258712b0b7b1ddf8bddc9fc3b0faca682b2736c3\"><code>258712b</code></a>\nfix: stop cleanup timestamp pruning after the first entry (<a\nhref=\"https://redirect.github.com/swatinem/rust-cache/issues/377\">#377</a>)</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/swatinem/rust-cache/compare/6323deb102c322ba6fcbdcafc7e3dddab59af2b6...d1527c8f67ef9af357290475d4af4b0fab8f12e0\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\nUpdates `pypa/gh-action-pypi-publish` from 1.14.2 to 1.14.3\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/releases\">pypa/gh-action-pypi-publish's\nreleases</a>.</em></p>\n<blockquote>\n<h2>v1.14.3</h2>\n<!-- raw HTML omitted -->\n<blockquote>\n<p>[!important]\n🚨 This release includes fixes for <a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/security/advisories/GHSA-5xjx-3w67-w9x9\">GHSA-5xjx-3w67-w9x9</a>\npatched up by <a\nhref=\"https://github.com/sethmlarson\"><code>@​sethmlarson</code></a><a\nhref=\"https://github.com/sponsors/sethmlarson\">💰</a> in <a\nhref=\"https://redirect.github.com/pypa/gh-action-pypi-publish/issues/427\">#427</a>\nand in other places across the ecosystem.</p>\n<p>Additional ecosystem and related mitigations beyond you might want to\nwatch out for:</p>\n<ul>\n<li><a\nhref=\"https://discuss.python.org/t/information-for-python-users-regarding-emudhra-emsign-ca-certificate-security-incident/109576\">https://discuss.python.org/t/information-for-python-users-regarding-emudhra-emsign-ca-certificate-security-incident/109576</a></li>\n<li><a\nhref=\"https://emudhra.we-have-a-problem.com\">https://emudhra.we-have-a-problem.com</a></li>\n</ul>\n</blockquote>\n<h2>🛠️ Security Tightening</h2>\n<p><a href=\"https://github.com/webknjaz\"><code>@​webknjaz</code></a><a\nhref=\"https://github.com/sponsors/webknjaz\">💰</a> made sure the\ndependency tree within the action runtime uses hash-checking from now on\nvia <a\nhref=\"https://redirect.github.com/pypa/gh-action-pypi-publish/pull/420\">pypa/gh-action-pypi-publish#420</a>.</p>\n<p><strong>🪞 Full Diff</strong>: <a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/compare/v1.14.2...v1.14.3\">https://github.com/pypa/gh-action-pypi-publish/compare/v1.14.2...v1.14.3</a></p>\n<p><strong>🧔‍♂️ Release Manager:</strong> <a\nhref=\"https://github.com/sponsors/webknjaz\"><code>@​webknjaz</code></a>\n<a href=\"https://stand-with-ukraine.pp.ua\">🇺🇦</a></p>\n<p><strong>💬 Discuss</strong> <a\nhref=\"https://bsky.app/profile/webknjaz.me/post/3mydcjweyak2f\">on\nBluesky 🦋</a>, <a\nhref=\"https://mastodon.social/@webknjaz/117474743338290807\">on Mastodon\n🐘</a> and <a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/discussions/429\">on\nGitHub</a>.</p>\n<p><a href=\"https://github.com/sponsors/webknjaz\"><img\nsrc=\"https://img.shields.io/badge/%40webknjaz-transparent?logo=githubsponsors&amp;logoColor=%23EA4AAA&amp;label=Sponsor&amp;color=2a313c\"\nalt=\"GH Sponsors badge\" /></a></p>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/7245906fb61fa33a13554f9d5fb856fe61758086\"><code>7245906</code></a>\nMerge pull request <a\nhref=\"https://redirect.github.com/pypa/gh-action-pypi-publish/issues/427\">#427</a>\nfrom sethmlarson/certifi-sigstore</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/331eccb0e13a5c1124ed9f414a079ef2bef1095f\"><code>331eccb</code></a>\nUpgrade certifi and sigstore</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/f68fee44f9faa979857cda34894b750df5f571ca\"><code>f68fee4</code></a>\n💰 Sync the FUNDING config</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/c0d6f8e4af2ad8247a7da213b199ea78dac8395e\"><code>c0d6f8e</code></a>\nMerge branch 'integrity/runtime/deptree' into unstable/v1</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/909009990f9cf7e7c6a64e602638cf9e91af47f8\"><code>9090099</code></a>\n🧪 Make smoke test deps hash-pinned in CI</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/34e40c24acd7b4a9150956d646c8c7ba667a14f3\"><code>34e40c2</code></a>\n📌 Bump pip-tools managed transitive deptrees</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/e800ed547ac29cbc9324311809e2f36fe82148f9\"><code>e800ed5</code></a>\n👷 Enable hash-checking in the runtime deptree 🔗</li>\n<li><a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/commit/dd8c5f4f1f65ddf01533969008446145facc7b02\"><code>dd8c5f4</code></a>\n🔧 Make pip-tools config available in container</li>\n<li>See full diff in <a\nhref=\"https://github.com/pypa/gh-action-pypi-publish/compare/dc37677b2e1c63e2034f94d8a5b11f265b73ba33...7245906fb61fa33a13554f9d5fb856fe61758086\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-27T17:20:49-07:00",
          "tree_id": "5d58ea19fb69157d9aaf7e7614351a66cbf43c41",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/ea3600ab2b2e52b593f6333b6da831fc1b9a4deb"
        },
        "date": 1793147021029,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 470.139,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 476.578,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 468.397,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 478.958,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 26.139,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 19.071,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 23.538,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.444,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 10.865,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.235,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 7.975,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.021,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.544,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.481,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.982,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.6,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.646,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.204,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.758,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.238,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.594,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.141,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 12.71,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 29.458,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
            "unit": "MB"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "67f960bfa87ce259e4e9ac5921260082109c77e2",
          "message": "build(deps): bump libc from 0.2.190 to 0.2.191 in the cargo group across 1 directory (#215)\n\nBumps the cargo group with 1 update in the /drivers/wasmtime/driver\ndirectory: [libc](https://github.com/rust-lang/libc).\n\nUpdates `libc` from 0.2.190 to 0.2.191\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/rust-lang/libc/releases\">libc's\nreleases</a>.</em></p>\n<blockquote>\n<h2>0.2.191</h2>\n<h3>Support</h3>\n<ul>\n<li>Emscripten: Support v4.0.12 to v6.0.11 (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5610\">#5610</a>)</li>\n</ul>\n<h3>Added</h3>\n<ul>\n<li>Apple: Add <code>F_SETNOSIGPIPE</code> and\n<code>F_GETNOSIGPIPE</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5609\">#5609</a>)</li>\n<li>Apple: add <code>ifaliasreq</code> and <code>in6_aliasreq</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5557\">#5557</a>)</li>\n<li>FreeBSD: Add <code>PT_*</code> constants (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5625\">#5625</a>)</li>\n<li>Haiku: Add <code>AI_*</code> and <code>NI_*</code> constants from\nnetdb.h (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5585\">#5585</a>)</li>\n<li>Hurd: Add <code>POSIX_SPAWN_*</code> flags (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5578\">#5578</a>)</li>\n<li>NuttX: Add <code>AT_SYMLINK_NOFOLLOW</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5614\">#5614</a>)</li>\n<li>OpenBSD: Add <code>sensordev</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5534\">#5534</a>)</li>\n<li>QuRT: Add <code>isatty</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5618\">#5618</a>)</li>\n<li>WASI: Add <code>pthread_condattr_{get,set}clock</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5626\">#5626</a>)</li>\n</ul>\n<h3>Fixed</h3>\n<ul>\n<li>FreeBSD-aarch64: Fix <code>gpregs</code> and <code>fpregs</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5603\">#5603</a>)</li>\n</ul>\n<h3>Changed</h3>\n<ul>\n<li>AIX: Restore 'c_ulong' as the type of 'statvfs.f_fsid'. (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5616\">#5616</a>)</li>\n<li>Android: Make <code>prop_info</code> an extern type (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5561\">#5561</a>)</li>\n<li>Apple: Mach API is now available on non-MacOS targets again. This\nshould resolve some issues building ecosystem crates. (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pulls/5612\">#5612</a>),\n(<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5636\">#5636</a>)</li>\n<li>FreeBSD: Add back <code>NOTE_PCTRLMASK</code> with a deprecation\nwarning (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5600\">#5600</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a\nhref=\"https://github.com/rust-lang/libc/blob/0.2.191/CHANGELOG.md\">libc's\nchangelog</a>.</em></p>\n<blockquote>\n<h2><a\nhref=\"https://github.com/rust-lang/libc/compare/0.2.190...0.2.191\">0.2.191</a>\n- 2026-10-16</h2>\n<h3>Support</h3>\n<ul>\n<li>Emscripten: Support v4.0.12 to v6.0.11 (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5610\">#5610</a>)</li>\n</ul>\n<h3>Added</h3>\n<ul>\n<li>Apple: Add <code>F_SETNOSIGPIPE</code> and\n<code>F_GETNOSIGPIPE</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5609\">#5609</a>)</li>\n<li>Apple: add <code>ifaliasreq</code> and <code>in6_aliasreq</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5557\">#5557</a>)</li>\n<li>FreeBSD: Add <code>PT_*</code> constants (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5625\">#5625</a>)</li>\n<li>Haiku: Add <code>AI_*</code> and <code>NI_*</code> constants from\nnetdb.h (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5585\">#5585</a>)</li>\n<li>Hurd: Add <code>POSIX_SPAWN_*</code> flags (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5578\">#5578</a>)</li>\n<li>NuttX: Add <code>AT_SYMLINK_NOFOLLOW</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5614\">#5614</a>)</li>\n<li>OpenBSD: Add <code>sensordev</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5534\">#5534</a>)</li>\n<li>QuRT: Add <code>isatty</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5618\">#5618</a>)</li>\n<li>WASI: Add <code>pthread_condattr_{get,set}clock</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5626\">#5626</a>)</li>\n</ul>\n<h3>Fixed</h3>\n<ul>\n<li>FreeBSD-aarch64: Fix <code>gpregs</code> and <code>fpregs</code> (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5603\">#5603</a>)</li>\n</ul>\n<h3>Changed</h3>\n<ul>\n<li>AIX: Restore 'c_ulong' as the type of 'statvfs.f_fsid'. (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5616\">#5616</a>)</li>\n<li>Android: Make <code>prop_info</code> an extern type (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5561\">#5561</a>)</li>\n<li>Apple: Mach API is now available on non-MacOS targets again. This\nshould resolve some issues building ecosystem crates. (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pulls/5612\">#5612</a>),\n(<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5636\">#5636</a>)</li>\n<li>FreeBSD: Add back <code>NOTE_PCTRLMASK</code> with a deprecation\nwarning (<a\nhref=\"https://redirect.github.com/rust-lang/libc/pull/5600\">#5600</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/ccc2ff4aaab962f7fde7b8edda7379c045b1ca12\"><code>ccc2ff4</code></a>\nlibc: Release 0.2.191</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/bce6efbe0a9d77770b579b933af5cbbe0b5a916d\"><code>bce6efb</code></a>\napple: Add <code>mach_task_self</code> and <code>mach_task_self_</code>\nto semver checks</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/ecf4fd4d0febb45e5199d8f82b956ab45817423f\"><code>ecf4fd4</code></a>\napple: Re-restrict <code>mach_vm_map</code> to MacOS-only</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/b0e042335159940bff94be6693107f76d43dc4ea\"><code>b0e0423</code></a>\nRevert &quot;apple: Disable <code>mach.h</code> API on Apple mobile\nOSs&quot;</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/9c7f568d7a00751d7d32cb0014f4edbff68bff0b\"><code>9c7f568</code></a>\napple: add <code>ifaliasreq</code> and <code>in6_aliasreq</code></li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/018c7e9d0d1f84601180ea83d9240c5812909570\"><code>018c7e9</code></a>\nwasi: Add pthread_condattr_{get,set}clock</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/2c7718527e806c3b40dff13268f8119276100672\"><code>2c77185</code></a>\nopenbsd: add sensordev structure</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/e10813cff2a5cfb6ea654f9ead821572de2f278f\"><code>e10813c</code></a>\nhurd: Add missing <code>POSIX_SPAWN_*</code> flags</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/2ea4bc1d8d8ee12b8b12237ee9fdc64bcdb4acac\"><code>2ea4bc1</code></a>\nFix FreeBSD/aarch64</li>\n<li><a\nhref=\"https://github.com/rust-lang/libc/commit/6d666fdfdee785cec793de163fdcbd0ad83f5567\"><code>6d666fd</code></a>\ndragonfly: Parse version from headers rather than CLI</li>\n<li>Additional commits viewable in <a\nhref=\"https://github.com/rust-lang/libc/compare/0.2.190...0.2.191\">compare\nview</a></li>\n</ul>\n</details>\n<br />\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2026-10-27T17:24:09-07:00",
          "tree_id": "e6fb6acf48a86fb881d9c41e4114848c2db00a58",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/67f960bfa87ce259e4e9ac5921260082109c77e2"
        },
        "date": 1793148432248,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 336.296,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 332.354,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 333.856,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 338.884,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 20.48,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 15.742,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 19.107,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 29.526,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.047,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.089,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 6.358,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.998,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.238,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.2,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.612,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.296,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.586,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.1,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.438,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.927,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.552,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.954,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 8.824,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 20.692,
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
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 6,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 6,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}