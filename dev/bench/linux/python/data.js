window.BENCHMARK_DATA = {
  "lastUpdate": 1792624554250,
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
        "date": 1790100020967,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 275.967,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 273.455,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 389.206,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.574,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.338,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 143.15,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.662,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.155,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 121.728,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.253,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.919,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.571,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.533,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.265,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.083,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.066,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.91,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 214.003,
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
          "id": "fb62e312716e874ca205079533bb0caad699cf33",
          "message": "Merge pull request #116 from hyperlight-dev/v0.14.1-urunc-fixes\n\nFixes and additions from the urunc port: AF_UNSPEC disconnect, --port all, --resolv-conf",
          "timestamp": "2026-09-22T16:00:42-07:00",
          "tree_id": "f5acc639381de1da38f8591283ce7421427365e3",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/fb62e312716e874ca205079533bb0caad699cf33"
        },
        "date": 1790118134323,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 288.763,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 287.35,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 403.914,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.594,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.075,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 144.653,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.73,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.215,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 122.584,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.099,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.087,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.518,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.566,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.229,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 42.686,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.79,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.8,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 214.562,
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
          "id": "d67e68d6cf0b9852ba5e48f4d5d3981bb4f9bf98",
          "message": "demos: declare the urunc demo image's command\n\nAn OCI image says what it runs; Docker refuses to start one that says\nnothing, and the urunc demo image said nothing, so the README's own\n`docker run` line never worked. It ran everywhere else only because the\ndriver fell back to /entrypoint.py, a path that is the driver's business\nand should not be part of an image's contract.\n\nBake the workload at /app/hello.py and declare it as the image's CMD,\nwhich urunc passes to hluk as the guest command, and name Hyperlight in\nthe greeting. Refresh the README, which still described the urunc side\nas pending.\n\nVerified with the rebuilt image through hluk directly, `docker run` and\n`ctr run` with nothing after the image name.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-09-22T23:56:16Z",
          "tree_id": "685261fd60cbc7a91c107d588c6e04ffebbeb35b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d67e68d6cf0b9852ba5e48f4d5d3981bb4f9bf98"
        },
        "date": 1790121464346,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 227.568,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 227.878,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 319.727,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.038,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.38,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 114.091,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 3.829,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 0.933,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 96.973,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.934,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.82,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.149,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.997,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.18,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.185,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.322,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.482,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 169.599,
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
          "id": "2b4790d1626e2645dd1f00469311424be8cb2f94",
          "message": "Merge pull request #117 from hyperlight-dev/hostfs-mounts-on-resume\n\nv0.14.1: Restore a snapshot with any mounts",
          "timestamp": "2026-09-22T21:30:13-07:00",
          "tree_id": "9e7c5f5de22880c141cdcdd15811ec44d0aee070",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/2b4790d1626e2645dd1f00469311424be8cb2f94"
        },
        "date": 1790137909743,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 277.631,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 276.315,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 276.178,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 391.14,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.594,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.264,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.129,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 144.623,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.746,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.207,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.393,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 121.931,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.493,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.084,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.694,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.746,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.541,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.286,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.754,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.112,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.198,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.939,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.822,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 212.898,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
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
          "id": "93430ffb18831a6090b6c66d9c795aef2022da55",
          "message": "Merge pull request #118 from hyperlight-dev/hluk-init\n\nv0.14.2: projects, templates, a registry-pulled rootfs and warm snapshots",
          "timestamp": "2026-09-24T00:44:06-07:00",
          "tree_id": "1a8de55a9d9aad839d67227138cfc319403a8419",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/93430ffb18831a6090b6c66d9c795aef2022da55"
        },
        "date": 1790236014182,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 280.364,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 276.925,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 276.312,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 393.271,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.421,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.327,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.971,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 143.763,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.662,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.217,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.25,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 121.937,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.514,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.234,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.577,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.78,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.56,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.245,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.674,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.143,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.334,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.995,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.764,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 218.739,
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
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 18,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
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
        "date": 1790366896540,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 353.362,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 353.88,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 362.16,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 416.246,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.401,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.531,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.751,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 77.75,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.831,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.639,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.284,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 55.504,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.646,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.57,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.238,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.111,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.648,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.275,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.719,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.414,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.605,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.387,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.006,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.993,
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
            "value": 17,
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
        "date": 1790459229770,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 161.666,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 158.461,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 160.157,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 188.866,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.337,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.513,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.322,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 50.83,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.568,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.293,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.537,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.795,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.063,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.971,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.332,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.235,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.685,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.071,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.464,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 24.095,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.746,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.853,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.592,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 55.375,
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
        "date": 1790664355728,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 152.473,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 152.125,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 152.291,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 183.529,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 10.829,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.036,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 9.152,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 51.284,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.108,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.258,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.301,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 36.848,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.911,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.879,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.171,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.189,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.542,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.117,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.505,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 28.915,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 5.598,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.583,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 2.914,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 66.622,
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
          "id": "44ee170f171ed387cbeb530775edb913bbf687aa",
          "message": "examples, tests: check musl condition variables and futex requeues\n\nTwo C programs, built with musl in Alpine (glibc's condition variables\ndo not requeue): condvar broadcasts to four waiters fifty times and\nexpects every one to wake, and futex_requeue checks FUTEX_REQUEUE and\nFUTEX_CMP_REQUEUE against Linux's semantics, which it passes on Linux\ntoo.  Every wait has a deadline, so a kernel that loses a waiter fails\nthe test rather than hanging it, and the tests print which check failed.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T14:23:19Z",
          "tree_id": "c75fe4aa7f972e6f548fa3f3b9fb7a0e6026d448",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/44ee170f171ed387cbeb530775edb913bbf687aa"
        },
        "date": 1790953925303,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 199.807,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 198.204,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 198.977,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 247.26,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.59,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.258,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.712,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.423,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.842,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.539,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.92,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.972,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.287,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.126,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.492,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.435,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.664,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.201,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.781,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.414,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.792,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.394,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.615,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.766,
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
          "id": "57a491cd8c38c154e46fb7c78713a4bd9cf31a0c",
          "message": "ci: publish main's builds as the dev pre-release\n\nEach push to main whose CI passes publishes that commit's images, builds hluk for the four release targets pinned to them, moves the dev tag and replaces the dev pre-release, then deletes the dev images of all but this build and the one before.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T15:35:52Z",
          "tree_id": "9225836f73e02859f996924c72fa772985e99809",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/57a491cd8c38c154e46fb7c78713a4bd9cf31a0c"
        },
        "date": 1790955457514,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 201.043,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 197.609,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 199.834,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 248.975,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.794,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.783,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.759,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.549,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.806,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.514,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.072,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.804,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.296,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.193,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.673,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.425,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.622,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.213,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.681,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.336,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.87,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.391,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.821,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.939,
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
          "id": "66d6f70039271bdfb443f9273dc0f285a341563c",
          "message": "README: say how hluk is pronounced\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-02T17:16:20Z",
          "tree_id": "989ffe030b778bd802adae626678a846769c2384",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/66d6f70039271bdfb443f9273dc0f285a341563c"
        },
        "date": 1790961475785,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 152.352,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 149.556,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 150.033,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 182.085,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 10.834,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.126,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 8.971,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 50.301,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.703,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.29,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.332,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 36.348,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.032,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.92,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.202,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.944,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.63,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.107,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.523,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 28.842,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 5.76,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.604,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 2.957,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 66.508,
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
        "date": 1790983764889,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 197.525,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 196.142,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 196.668,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 243.937,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.274,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.413,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.57,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.052,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.668,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.487,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.984,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.762,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.176,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.14,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.498,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.419,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.623,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.215,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.682,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.392,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.775,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.48,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.722,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.725,
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
        "date": 1790984582576,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 191.843,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 190.106,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 185.394,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 225.91,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.31,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.736,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.623,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 70.739,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.493,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.446,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.758,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 52.372,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.113,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.098,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.421,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.186,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.567,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.163,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.692,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.105,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.49,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.38,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.341,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 106.018,
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
        "date": 1792006384405,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 199.246,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 197.527,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 198.27,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 245.016,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.613,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.956,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.294,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 73.845,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.9,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.468,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.885,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.942,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.104,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.12,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.452,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.26,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.703,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.226,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.681,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.68,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.804,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.377,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.612,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 108.039,
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
        "date": 1792012435231,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 143.341,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 140.477,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 142.892,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 165.854,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 11.537,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.873,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.183,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 45.04,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 3.987,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.188,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.178,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.413,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.874,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.896,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.189,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.105,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.474,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.063,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.376,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 21.718,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.035,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.595,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 2.998,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 49.204,
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
        "date": 1792013956913,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 194.363,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 193.447,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 194.088,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 242.549,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.843,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.119,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.944,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 73.588,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.646,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.515,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.852,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.501,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.126,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.14,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.489,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.342,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.644,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.167,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.652,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.208,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.644,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.322,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.579,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.333,
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
        "date": 1792083759876,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 144.303,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 143.542,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 140.857,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 166.454,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 11.75,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.89,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 9.837,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 45.606,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.036,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.172,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.173,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.175,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.894,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.882,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.189,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.077,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.478,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.046,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.402,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 21.557,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 5.982,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.567,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.037,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 52.24,
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
          "distinct": true,
          "id": "50a2e4593f3a9d2062fc0da091896f88a713f292",
          "message": "ci: enable Dependabot version updates\n\nMonthly grouped updates for cargo, GitHub Actions and the Dockerfiles'\nbase images. Docker gets digest refreshes and patch releases only; rust\nand powershell images, Hyperlight minors and majors, and the local hluk-*\nrootfs images are left to be bumped by hand. Majors outside the wasmtime\ngroup and typos come as their own pull requests.\n\nFixes #135\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T17:09:56Z",
          "tree_id": "f5920b16c41cd2490c0da38cd837c961e3430f19",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/50a2e4593f3a9d2062fc0da091896f88a713f292"
        },
        "date": 1792085256550,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 160.449,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 160.87,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 163.106,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 197.025,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.298,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.684,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.167,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 58.185,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.894,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.353,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.58,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.637,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.176,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.144,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.523,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.35,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.999,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.128,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.534,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.21,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.708,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.843,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.808,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 87.61,
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
          "distinct": true,
          "id": "e5aaa2fbf54c8bad953ab97ecc36e5dcdd51e27a",
          "message": "CHANGELOG: Rust 1.99, the static arm64 wasmtime image\n\nFixes #129\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T17:38:20Z",
          "tree_id": "d00bd1550d12b4b9f20b74cf8c28c62671bec5d8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/e5aaa2fbf54c8bad953ab97ecc36e5dcdd51e27a"
        },
        "date": 1792087089952,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 157.983,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 155.86,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 156.904,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 192.645,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.003,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.088,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.504,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 57.001,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.77,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.265,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.44,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.414,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.069,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.052,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.417,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.251,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.975,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.119,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.511,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.15,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.587,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.79,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.559,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 87.297,
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
        "date": 1792089518018,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 196.187,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 195.423,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 196.264,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 242.134,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.616,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.487,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.022,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.128,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.826,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.498,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.091,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.632,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.267,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.223,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.69,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.514,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.65,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.195,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.741,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.167,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.942,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.471,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.941,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.329,
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
          "id": "39a850213d054eff389dbc1fae173accf8c86021",
          "message": "Merge pull request #149 from hyperlight-dev/dependabot/github_actions/dtolnay/rust-toolchain-e2a55d2ffb04f378e9626c28d38b36d230d1e12f\n\nbuild(deps): bump dtolnay/rust-toolchain from 02cb101ec7c40f2c49e1d9714d64511d8e1b74de to e2a55d2ffb04f378e9626c28d38b36d230d1e12f",
          "timestamp": "2026-10-15T11:48:52-07:00",
          "tree_id": "bde23d89676bc7c70bbba15443e128be8ada7032",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/39a850213d054eff389dbc1fae173accf8c86021"
        },
        "date": 1792091198283,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 146.575,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 145.21,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 148.334,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 173.499,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.838,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.391,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.068,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 47.416,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.372,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.275,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.424,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.399,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.033,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.372,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.215,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.539,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.091,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.377,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.33,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.513,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.676,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.369,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 51.39,
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
        "date": 1792093884400,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 214.793,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 214.038,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 215.161,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 263.085,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.778,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.134,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.152,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.309,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.653,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.175,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.867,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.923,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.877,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.32,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.064,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.678,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.203,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.784,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.518,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.656,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.426,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.127,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 108.298,
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
        "date": 1792105930720,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 172.592,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 170.487,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 170.756,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 206.181,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.395,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.775,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.193,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 56.992,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.054,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.333,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.496,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.367,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.731,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.7,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.042,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.813,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.064,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.139,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.552,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.195,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.137,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.85,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.722,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 87.607,
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
        "date": 1792107902125,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 214.432,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 212.772,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 213.63,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 260.79,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.063,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.781,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.211,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 73.792,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.126,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.653,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.115,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 54.872,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.885,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.878,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.284,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.054,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.646,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.181,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.766,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.357,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.332,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.41,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.045,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 108.362,
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
          "id": "78e0b8e182040f5ba7eaa51342ce0e2508d424e1",
          "message": "CHANGELOG: Dependabot version updates, ci-passed waiting for arm64\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:43:36Z",
          "tree_id": "1a20c20e1116abff820839863e1572fac775de9c",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/78e0b8e182040f5ba7eaa51342ce0e2508d424e1"
        },
        "date": 1792108719286,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 228.954,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 223.499,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 225.902,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 272.024,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.589,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.295,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.391,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 75.3,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.692,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.836,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.481,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 55.274,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.064,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.994,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.529,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.137,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.759,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.23,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.786,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.615,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.809,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.701,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.394,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 108.674,
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
          "id": "965ab21cce2cd6b964bd1baf2693aaafbfcddd6a",
          "message": "Merge pull request #160 from hyperlight-dev/dependabot/cargo/demos/pptx-gen/cargo-8681492501\n\nbuild(deps): bump rustls from 0.23.43 to 0.23.46 in /demos/pptx-gen in the cargo group across 1 directory",
          "timestamp": "2026-10-15T17:18:46-07:00",
          "tree_id": "d67a478738c1505c7a8e7d4ea172fd156040381e",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/965ab21cce2cd6b964bd1baf2693aaafbfcddd6a"
        },
        "date": 1792110020280,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 217.282,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 216.601,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 217.481,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 269.898,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.583,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.967,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.806,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 75.139,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.295,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.688,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.201,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 55.433,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.93,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.924,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.399,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.128,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.633,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.23,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.764,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.451,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.551,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.414,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.191,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 109.424,
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
          "id": "75f065e006970a3a21e49a0576e0e6134abdcaea",
          "message": "Merge pull request #161 from hyperlight-dev/dependabot/cargo/drivers/wasmtime/driver/wasmtime-e909ad153e\n\nbuild(deps): bump wasmparser from 0.259.0 to 0.261.0 in /drivers/wasmtime/driver in the wasmtime group across 1 directory",
          "timestamp": "2026-10-15T17:22:38-07:00",
          "tree_id": "cfcc3acee37fd110882e74bbe37b8fca3db55260",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/75f065e006970a3a21e49a0576e0e6134abdcaea"
        },
        "date": 1792111233158,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 206.899,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 202.86,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 202.773,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 247.843,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.201,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.498,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.943,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 65.693,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.992,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.552,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.975,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 50.505,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.979,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.852,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.474,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.144,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.363,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.155,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.617,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 41.019,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.675,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.156,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.091,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 109.503,
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
          "distinct": false,
          "id": "a2ad7ab464bbd46b49798452281b848e47f3cea5",
          "message": "release: v0.18.0\n\nBump the crate to 0.18.0 and add the changelog's entries under\n[v0.18.0], keeping an empty Unreleased section.  0.18.0 rather than\n0.17.1: the host transport moves to Hyperlight 0.18's virtqueues, the\nhost functions changed (HostWrite, HostWriteErr), and snapshots and\nnative kernels built for 0.17 are refused.  The snapshot contract was\nalready bumped with those changes; the kernel moved too, which rolls\nthe snapshot key by itself.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-16T00:22:52Z",
          "tree_id": "3a351f900d4e88ce267f6dd00a34cd41f2385049",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a2ad7ab464bbd46b49798452281b848e47f3cea5"
        },
        "date": 1792112204211,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 220.161,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 217.507,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 219.103,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 264.941,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.515,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.728,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.098,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 73.425,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.458,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.708,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.219,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 53.706,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.927,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.908,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.361,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.063,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.57,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.185,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.734,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 42.74,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.224,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.405,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.809,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 113.005,
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
          "id": "d8f5a055c47b9c60ec3a50eeb83fc836e8217d08",
          "message": "Merge pull request #163 from hyperlight-dev/smolagents-integration\n\nAdd smolagents-hyperlight, a smolagents executor in a micro-VM",
          "timestamp": "2026-10-19T00:37:27-07:00",
          "tree_id": "3bc95c35b2f14ffb9bb7f2329897ba862f85be4a",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/d8f5a055c47b9c60ec3a50eeb83fc836e8217d08"
        },
        "date": 1792395536619,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 217.504,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 215.048,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 215.577,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 262.54,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.489,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.665,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.076,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 74.063,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.376,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.695,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.225,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 55.004,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.948,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.925,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.43,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.078,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.646,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.214,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.709,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.327,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.496,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.479,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.023,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 107.839,
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
        "date": 1792415712391,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 155.604,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 158.257,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 158.222,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 180.055,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.854,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.285,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 10.263,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 46.481,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.657,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.339,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.409,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.777,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.715,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.713,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.011,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.733,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.562,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.091,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.399,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.315,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.784,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.766,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.441,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 51.345,
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
          "id": "c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597",
          "message": "README, CHANGELOG, docs/RELEASE.md: mcp-server-hyperlight\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T06:56:43Z",
          "tree_id": "d76923ae19bdf0b4c4b4bd38aec58c5340d50e04",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597"
        },
        "date": 1792480221037,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 149.884,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 148.526,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 148.869,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 172.144,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 12.271,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.525,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 9.779,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 44.138,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 4.436,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.274,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.392,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 28.849,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.671,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.678,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 0.992,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 0.724,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.508,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.077,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.447,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 21.404,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 6.507,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.678,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.35,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 48.961,
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
        "date": 1792485216681,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 187.354,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 186.828,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 187.201,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 223.094,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.366,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.608,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.622,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 59.345,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.671,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.525,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.092,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 43.329,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.228,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.131,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.546,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.547,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.017,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.136,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.535,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 33.224,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.026,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.205,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.411,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 88.093,
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
          "id": "69c1d8ccb57090793642dd64bcb5d6456c76aff2",
          "message": "Merge pull request #168 from hyperlight-dev/fix-py-flush\n\ndrivers: flush Python's stdout and stderr when a call ends",
          "timestamp": "2026-10-20T20:47:31-07:00",
          "tree_id": "fdf5c27204ba52f7bc2ffa80733cdd37b2fa8eb8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/69c1d8ccb57090793642dd64bcb5d6456c76aff2"
        },
        "date": 1792555332195,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 170.939,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 168.881,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 172.13,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 196.14,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.947,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.428,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.519,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 49.674,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.057,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.602,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.884,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.415,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.12,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.127,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.457,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.479,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.615,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.1,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.445,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.971,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.509,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.058,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 3.969,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 53.645,
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
          "id": "a7fb4a0d785960dcd128e3d3d0fd69599fce5293",
          "message": "Merge pull request #167 from hyperlight-dev/clickhouse-udf\n\nintegrations/clickhouse: untrusted Python UDFs for ClickHouse in micro-VMs",
          "timestamp": "2026-10-20T21:16:07-07:00",
          "tree_id": "06b079ed25f8283740ae2c14d6f7717373c5159d",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/a7fb4a0d785960dcd128e3d3d0fd69599fce5293"
        },
        "date": 1792556363704,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 232.836,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 230.453,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 231.952,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 279.63,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.361,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.333,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.983,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 76.896,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.111,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.94,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.908,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 56.944,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.445,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.312,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.862,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.775,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.702,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.229,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.829,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.373,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.955,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.956,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.043,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 111.332,
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
          "id": "5efba5f4f34cfb410a130c3e3e0e8aea7334bfaa",
          "message": "Merge pull request #169 from hyperlight-dev/windows-arm64-guests\n\nWindows arm64 guests: shareable arm64 pages, initrd clear of WHP's GIC",
          "timestamp": "2026-10-20T23:03:22-07:00",
          "tree_id": "4389426a93fa5c7703812621980a540796254f93",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/5efba5f4f34cfb410a130c3e3e0e8aea7334bfaa"
        },
        "date": 1792562694446,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 236.446,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 232.5,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 233.485,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 282.67,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 18.077,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.983,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.519,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 76.958,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.315,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.051,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.898,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 57.257,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.531,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.407,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.877,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.856,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.723,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.215,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.806,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.573,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.957,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.997,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.267,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 111.265,
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
          "id": "f5bcafc73642f096bec5468a9c73f1298de89926",
          "message": "CHANGELOG, docs/RELEASE.md: MCP Registry and Desktop Extension releases\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-21T06:06:16Z",
          "tree_id": "63d8957dc45c01ebfdc0e04feb8dec8ab2489ed8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f5bcafc73642f096bec5468a9c73f1298de89926"
        },
        "date": 1792563637278,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 183.607,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 180.321,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 184.758,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 212.283,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.477,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.848,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.503,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 53.148,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.718,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.694,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.269,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 34.812,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.305,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.203,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.601,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.599,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.707,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.115,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.464,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 23.913,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.279,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.238,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.431,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 57.274,
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
        "date": 1792607954857,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 232.75,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 229.93,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 231.877,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 278.566,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.609,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.354,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.295,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 76.696,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.106,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.951,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.827,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 56.774,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.448,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.341,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.814,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.755,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.73,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.222,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.786,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.323,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.821,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.956,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.92,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 110.939,
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
        "date": 1792610122482,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 231.969,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 230.5,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 231.234,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 279.903,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.205,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.646,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.002,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 77.26,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.036,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.014,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.917,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 57.06,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.417,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.385,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.842,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.877,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.707,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.218,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.768,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.424,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.002,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.072,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.086,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 110.939,
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
        "date": 1792623788636,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 230.974,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 230.959,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 230.284,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 278.248,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.566,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.53,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.931,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 76.192,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.065,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.989,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.802,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 56.481,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.42,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.358,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.814,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.64,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.698,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.205,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.746,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 44.325,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.742,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.021,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.948,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 110.951,
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
        "date": 1792624553306,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 177.844,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 173.523,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 173.059,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 198.888,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.144,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.873,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 11.812,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 49.751,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.459,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.595,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.01,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 32.49,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.222,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.126,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.504,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.376,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.65,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.084,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.409,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 22.851,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.931,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.078,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.042,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 53.242,
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