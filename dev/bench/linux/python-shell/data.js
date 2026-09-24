window.BENCHMARK_DATA = {
  "lastUpdate": 1790236034007,
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
      }
    ]
  }
}