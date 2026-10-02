window.BENCHMARK_DATA = {
  "lastUpdate": 1790984482390,
  "repoUrl": "https://github.com/hyperlight-dev/hyperlight-unikraft",
  "entries": {
    "agent benchmarks": [
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
        "date": 1789241165870,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 21682.845,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 21665.259,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 21601.671,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 183.583,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 87.689,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 246.042,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 40.136,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 19.644,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 67.935,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 50.408,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 49.607,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 50.815,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.693,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.371,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 12.029,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 48.555,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 23.099,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 86.367,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 876,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 876,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 876,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1789243062285,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10496.686,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10601.972,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10543.128,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 91.886,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 49.256,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 122.439,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.48,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.813,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.477,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 29.994,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 28.567,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 29.572,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.598,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.172,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.046,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.533,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 14.57,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 47.242,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1789245731387,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 15208.387,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 15445.548,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 15559.128,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 136.159,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 76.001,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 197.59,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 27.613,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 12.708,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 48.655,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 54.147,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 53.991,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 55.214,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.773,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.289,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.649,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 39.02,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 17.762,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 93.738,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.7,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.7,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1789248649899,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 16064.46,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16170.061,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16588.859,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 139.909,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 78.165,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 193.284,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 26.961,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 13.412,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 47.588,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 55.447,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 56.165,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 57.103,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.93,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.322,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.659,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 42.108,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 19.814,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 101.288,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.8,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1790098072232,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 16106.458,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16213.965,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16516.383,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 145.558,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 79.408,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 156.561,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.72,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.979,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.767,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 62.198,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 61.799,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 62.64,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.922,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.456,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.969,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 34.598,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.257,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 93.789,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
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
        "date": 1790101086823,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 16100.488,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16411.595,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16885.148,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 145.366,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 80.269,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 205.608,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.761,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.448,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 44.428,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 61.769,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 62.503,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 62.203,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.988,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.594,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.924,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 33.966,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.898,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 95.132,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1790119187178,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 15848.86,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16068.039,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16489.949,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 143.34,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 78.419,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 157.841,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.403,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.79,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 40.524,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 61.529,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 61.963,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 62.217,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.855,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.669,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.087,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.345,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.831,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 86.922,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
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
        "date": 1790122562118,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 16321.96,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16438.633,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16743.232,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 153.293,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 80.695,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 156.645,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 24.358,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.782,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.774,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 62.874,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 61.858,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 63.163,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.406,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.537,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.14,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 37.599,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.359,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 63.145,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
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
        "date": 1790139426821,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 18480.351,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 17621.941,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 17413.285,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 17250.464,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 164.121,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 71.609,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 100.337,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 152.999,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 28.826,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.037,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 18.025,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 43.973,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 48.597,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 43.578,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 45.466,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 42.764,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.877,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.484,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.681,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.246,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 50.079,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.256,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.489,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 55.232,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 29,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 25,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 29,
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
        "date": 1790237469608,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 16103.626,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16308.016,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 16601.315,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16734.437,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 143.839,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 101.31,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 104.708,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 198.897,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.07,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.112,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.824,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 43.266,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 61.989,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 63.176,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 64.176,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 63.136,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.973,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.578,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.464,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.925,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 34.944,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.748,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.334,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 89.944,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 873.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 25,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1790368333026,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 16227.689,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 16591.428,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 16691.836,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 16902.253,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 147.371,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 81.164,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 105.155,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 194.753,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.746,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.825,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.75,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.598,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 63.058,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 62.86,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 65.675,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 63.869,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.878,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.545,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.215,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.332,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 31.599,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.023,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.921,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 87.817,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 873.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 25,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 32,
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
        "date": 1790460255835,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 11252.9,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 11209.595,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 11442.185,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 11652.459,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 144.692,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 75.763,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 104.261,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 198.605,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 22.456,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.164,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 15.599,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 43.023,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 60.846,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 59.95,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 62.151,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.985,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.855,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.386,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.456,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.258,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 34.53,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.754,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 24.608,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 62.475,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 24,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 33,
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
        "date": 1790665321202,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10413.471,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10559.782,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10555.769,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10743.974,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 134.663,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 78.988,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 96.446,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 179.973,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.683,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.326,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.955,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 39.88,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 58.81,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 65.625,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 60.641,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 60.03,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.695,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.382,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.404,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.062,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.578,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.227,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.494,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 69.723,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.4,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 24,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 31,
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
        "date": 1790955063102,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 12193.943,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 12566.602,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 12762.687,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 12728.198,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 109.995,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 79.196,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 105.853,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 159.224,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 23.121,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.505,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 16.82,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 44.964,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 61.082,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 61.786,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 64.126,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 61.173,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.426,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.423,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 4.181,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.512,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 34.341,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.244,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 27.014,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 65.821,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 25,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
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
        "date": 1790956125722,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7234.3,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7420.095,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7184.937,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7091.254,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 65.072,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 45.286,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 78.813,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 96.373,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.879,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.941,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.98,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 28.006,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 33.124,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 31.952,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 33.914,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 33.022,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.766,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.221,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.031,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.414,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.697,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.476,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.002,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 38.344,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 24,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
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
        "date": 1790962183451,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7829.01,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7817.33,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7798.609,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7819.316,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 71.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 54.127,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 70.329,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 109.33,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.537,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.904,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.541,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 31.448,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 43.145,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 42.918,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 44.517,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 43.394,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.234,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.258,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.455,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.886,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 21.214,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.234,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.796,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 43.035,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 24,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
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
        "date": 1790984478957,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7922.566,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7849.224,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7880.472,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7806.529,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 73.619,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 53.639,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 71.827,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 108.11,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.907,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.86,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.556,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.377,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 43.165,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 42.855,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 44.302,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 43.299,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.397,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.248,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.233,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.78,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 21.651,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.452,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.928,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 41.991,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.5,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 20,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 24,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 28,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}