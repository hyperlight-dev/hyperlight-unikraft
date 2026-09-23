window.BENCHMARK_DATA = {
  "lastUpdate": 1790137910718,
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
      }
    ]
  }
}