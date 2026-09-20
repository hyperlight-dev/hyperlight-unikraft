window.BENCHMARK_DATA = {
  "lastUpdate": 1789945371624,
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
        "date": 1789239693107,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 288.358,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 285.735,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 402.325,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.996,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.893,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 145.526,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.683,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.211,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 125.384,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.36,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.35,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.568,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.457,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.159,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.225,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.539,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.56,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 217.607,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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
          "id": "cb6b6e1eb13f0ad346a93e6ca28a0f1dd042df4c",
          "message": "kernel: source the Unikraft platform from upstream unikraft/unikraft@plat-hyperlight-v2\n\nThe kernel/unikraft submodule now points at the canonical\nunikraft/unikraft plat-hyperlight-v2 branch instead of the danbugs fork.\nSame commit (9eca1f0b), so the embedded kernel binary is unchanged.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T19:42:24Z",
          "tree_id": "c408dbd54aff44e0e30a89eeb097bf40aadf0f80",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/cb6b6e1eb13f0ad346a93e6ca28a0f1dd042df4c"
        },
        "date": 1789242328260,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 225.136,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 223.03,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 314.374,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 11.478,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.485,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 112.248,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.118,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.224,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 96.85,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.295,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.287,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.452,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.986,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.068,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.073,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.736,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 4.15,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 169.173,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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
          "id": "9502e7de06b5dd96cd87e944ede85dbafb2f5c7c",
          "message": "demos/urunc: build the python rootfs base before staging the urunc image\n\npublish-urunc's rootfs.Dockerfile is FROM hluk-python-rootfs:latest — a\nlocal build stage, not a registry image. The publish job never built it, so\nDocker tried to pull it from Docker Hub and failed. Build it first in stage.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T20:15:17Z",
          "tree_id": "62cee22f1881f7340cf5f2bdb2d37966edd7c7aa",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/9502e7de06b5dd96cd87e944ede85dbafb2f5c7c"
        },
        "date": 1789244242637,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 275.906,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 274.496,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 389.291,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.947,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.1,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 143.317,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.439,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.817,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 123.405,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.347,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.327,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.476,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.514,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.095,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.04,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.417,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.073,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 216.141,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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
          "id": "d41b90a58bb73653a019f8bfa3357ad0ef93ea15",
          "message": "demos/urunc: call _push from the repo root\n\nThe publish-urunc recipe cd'd into demos/urunc and then called `just _push`,\nwhich resolved to demos/urunc/Justfile (no _push there). Run the staging +\ndocker build in a subshell so _push runs against the root justfile.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T20:23:33Z",
          "tree_id": "4b667d4b38a0253453f3cf9cafc6f4477fce31ea",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d41b90a58bb73653a019f8bfa3357ad0ef93ea15"
        },
        "date": 1789244731268,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 279.3,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 276.338,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 394.976,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.659,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.951,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 144.615,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.907,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.14,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 125.513,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.431,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.372,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.559,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.524,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.157,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.168,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.66,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.198,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 218.19,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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
          "id": "b36bb983867a5f5c9a8ca351f1e3ca339d9978ef",
          "message": "ci: pin crates-io-auth-action to a SHA (v1.0.5); drop stale git-deps note\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T21:02:16Z",
          "tree_id": "792a30b74831261ef4269f4aecbb46dcd993d811",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/b36bb983867a5f5c9a8ca351f1e3ca339d9978ef"
        },
        "date": 1789247066474,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 283.025,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 280.898,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 397.828,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.875,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.924,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 146.938,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.048,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.394,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 126.562,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.443,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.411,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.562,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.49,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.16,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.223,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.459,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.343,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 220.296,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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
          "id": "15a18a414bf59270698af375e24e5f942c365216",
          "message": "cargo: add required crates.io metadata (description, license, repository, readme)\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-12T21:10:44Z",
          "tree_id": "4092aa4c6bedbd45a68c5b62f3aedab79635ab16",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/15a18a414bf59270698af375e24e5f942c365216"
        },
        "date": 1789247579101,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 246.778,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 244.885,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 353.203,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 11.864,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.028,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 133.031,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.058,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.794,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 117.736,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.304,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.29,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.585,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.263,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.095,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 40.209,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.815,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 4.354,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 215.231,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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
          "id": "36b94063b559dc15d8111264903d463d0039ca95",
          "message": "Merge pull request #114 from hyperlight-dev/coop-pause-vm-v0.14.0\n\nv0.14.0: cooperative step model for long-running guests",
          "timestamp": "2026-09-20T15:50:40-07:00",
          "tree_id": "ef82580ea34504b16e6480392c408ffc6e3644bb",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/36b94063b559dc15d8111264903d463d0039ca95"
        },
        "date": 1789945368793,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 278.028,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 275.992,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 391.678,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.388,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.528,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 143.3,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.649,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.156,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 122.044,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.005,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.919,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.633,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.47,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.166,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.112,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.026,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.83,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 213.152,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
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