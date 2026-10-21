window.BENCHMARK_DATA = {
  "lastUpdate": 1792564224127,
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
        "date": 1790985561196,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10809.417,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10853.028,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10788.046,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 11130.742,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 100.084,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 74.221,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 95.606,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 143.471,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.91,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.76,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.808,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 39.428,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.228,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 58.86,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.115,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 63.614,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.892,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.363,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.584,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.404,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.072,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.008,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 19.109,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 77.801,
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
            "value": 22,
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
            "value": 27,
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
        "date": 1792007450857,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 11106.867,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10717.075,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10752.011,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10707.904,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 103.655,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 74.039,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 98.566,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 149.559,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.214,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.972,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.621,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.64,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.533,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 59.725,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.792,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.629,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.838,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.36,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.694,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.114,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.242,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.537,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.211,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 60.968,
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
        "date": 1792013422256,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10734.966,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10925.225,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10891.099,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 11182.814,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 101.405,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 83.271,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 98.092,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 148.388,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.726,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 8.994,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.877,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 41.396,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 58.93,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 70.715,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.426,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.648,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.775,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.314,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.215,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.689,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 30.892,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.606,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.51,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 90.943,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
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
          "distinct": false,
          "id": "cf458a07ed847f06d78284998a5beb13dc9fb426",
          "message": "hluk: keep no guest output in the CLI\n\nhluk run, hluk snapshot and hluk bench echo the guest's output as it is\nwritten and never drain it, yet kept all of it: a long-running guest\nthat kept printing grew the process without end (200 MiB of output\npeaked at 288 MB RSS, 80 MB now).  Every CLI boot now goes through one\nhelper that sets output_limit(0); the output still reaches the terminal.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-14T20:59:52Z",
          "tree_id": "1d4e4ff87b2a7f7b79fefdd62bc0c1efd249c12b",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/cf458a07ed847f06d78284998a5beb13dc9fb426"
        },
        "date": 1792014952560,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10833.29,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 11218.468,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 11101.442,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 11186.413,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 101.528,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 73.862,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 99.908,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 145.234,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.973,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.95,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.56,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 38.194,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.488,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 59.633,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.787,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.542,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.881,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.479,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.287,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.126,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 31.911,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.504,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.825,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 75.041,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
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
            "value": 27,
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
        "date": 1792091865772,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7753.503,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7265.937,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7240.187,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7483.978,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 68.757,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 45.9,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 66.022,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 100.668,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 15.498,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.761,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.591,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.508,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 33.439,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 31.742,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 36.023,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 33.518,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.757,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.181,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.066,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.431,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 21.236,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.372,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.734,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 40.735,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
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
        "date": 1792094848704,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10643.61,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10674.767,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10752.637,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10748.24,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 98.202,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 87.138,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 98.64,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 137.623,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.803,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.879,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.928,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 38.268,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.011,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 63.252,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.491,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 60.387,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.962,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.533,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.184,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.193,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.019,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.392,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.154,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 69.25,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
            "value": 27,
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
        "date": 1792106894691,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10630.449,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10746.404,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10773.526,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10898.997,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 96.646,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 73.029,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 94.597,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 171.952,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.235,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 6.584,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.573,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 37.441,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 57.474,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 57.181,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 59.274,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 56.904,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.954,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.375,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.976,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 12.125,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.275,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 8.664,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 20.683,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 54.671,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
          "id": "7ed4eb137c6285f8c478ff87d55441969a797490",
          "message": "tests: retry the HTTP GET examples when the server fails\n\ndotnet_jit_http_get, java_http_get, node_http_get and python_http_get\nfetch httpbin.org, which answers 502 often enough to fail CI on pull\nrequests that have nothing to do with networking.  A 5xx is the\nservice's failure, not the guest's, so the example now runs again, up\nto three times, 2 s then 5 s apart, each time in a fresh sandbox (a\nreused one keeps the failed attempt's sockets).  A run that fails any\nother way, or gets any other status, still fails the test at once: a\nDNS or connection error exercises the guest's resolver and network\nstack, which an outside request is there to test.  The output keeps\nevery attempt, and a retry is a CI warning, so retries that become the\nrule are seen.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:33:56Z",
          "tree_id": "f8e915338b960b068abc60350203c611e711ceec",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/7ed4eb137c6285f8c478ff87d55441969a797490"
        },
        "date": 1792108538941,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7122.555,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6997.071,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7421.003,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7258.756,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 61.671,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 47.195,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 65.086,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 89.268,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.037,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.107,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.788,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 25.443,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 32.065,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 32.029,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 35.354,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 32.632,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.717,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.184,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.79,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.061,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.646,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.665,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.264,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 33.486,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
          "distinct": false,
          "id": "78e0b8e182040f5ba7eaa51342ce0e2508d424e1",
          "message": "CHANGELOG: Dependabot version updates, ci-passed waiting for arm64\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-15T23:43:36Z",
          "tree_id": "1a20c20e1116abff820839863e1572fac775de9c",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/78e0b8e182040f5ba7eaa51342ce0e2508d424e1"
        },
        "date": 1792109690579,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10698.921,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10729.811,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10818.542,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10844.806,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 99.021,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 76.589,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 99.81,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 137.471,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.487,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.23,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 15.456,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 36.759,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.01,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 58.918,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.471,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.444,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.893,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.428,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.994,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.761,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.971,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.471,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.137,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 54.998,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
            "value": 27,
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
        "date": 1792111931879,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7797.042,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7829.676,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7822.446,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7828.437,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 73.833,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 56.073,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 71.538,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 99.587,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.422,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.291,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.775,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 27.581,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 43.22,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 42.951,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 44.902,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 43.35,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.145,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.303,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.223,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.782,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 20.804,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.786,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.017,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 38.008,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
            "value": 27,
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
        "date": 1792112959377,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 8036.347,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 8200.916,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 8337.253,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 8383.498,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 72.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 56.36,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 75.153,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 104.397,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.109,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 5.293,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.853,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 28.43,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 43.887,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 43.672,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 46.194,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 44.26,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.307,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.293,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.79,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.877,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.916,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.051,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.89,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 54.622,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
            "value": 27,
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
        "date": 1792396533898,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10956.403,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 11065.097,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 11118.922,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 11245.363,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 105.78,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 79.854,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 103.266,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 142.901,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 21.013,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.869,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 15.543,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 39.579,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 60.036,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 59.816,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 62.524,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 60.064,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 3.075,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.479,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.499,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.777,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 33.466,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 11.123,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.248,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 58.716,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
            "value": 27,
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
        "date": 1792416672697,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10627.925,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10883.647,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10669.431,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10993.737,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 97.598,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 74.791,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 114.698,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 137.338,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.789,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.218,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.787,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 36.272,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.328,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 58.933,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.458,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.619,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.967,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.448,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.008,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.635,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.265,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.078,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 23.236,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 53.139,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 805.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 805.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 805.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 805.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
          "distinct": false,
          "id": "c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597",
          "message": "README, CHANGELOG, docs/RELEASE.md: mcp-server-hyperlight\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T06:56:43Z",
          "tree_id": "d76923ae19bdf0b4c4b4bd38aec58c5340d50e04",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/c962ba0ae148a858d9f2ee79ec6bbaa4b90d9597"
        },
        "date": 1792481199234,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10692.21,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10806.73,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10868.258,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 11082.297,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 97.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 78.584,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 98.569,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 176.023,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 18.772,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.43,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.776,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 37.712,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 59.664,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 61.256,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.502,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 59.476,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.95,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.445,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.271,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.97,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 28.313,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 9.784,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 22.192,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 75.867,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
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
          "id": "dcb8b5010494132b990c75525c5dd4c6a29bbe01",
          "message": "CHANGELOG: Windows arm64 CI\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-20T08:13:51Z",
          "tree_id": "4c13bd809aac1b0ac04baedecd9575615c8eb1eb",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/dcb8b5010494132b990c75525c5dd4c6a29bbe01"
        },
        "date": 1792485935214,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7852.972,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7860.402,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7898.877,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7994.463,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 72.681,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 55.102,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 72.155,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 102.037,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.598,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.856,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.578,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 28.686,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 43.51,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 43.004,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 44.923,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 43.999,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.138,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.259,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 2.547,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.171,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 19.949,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.472,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 14.77,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 41.534,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 21,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 24,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 27,
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
        "date": 1792557305921,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 10664.809,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 10562.569,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 10872.861,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 10619.538,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 98.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 77.596,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 100.752,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 128.276,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.289,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 7.534,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 15.034,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 33.558,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 58.548,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 59.802,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 61.929,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 58.659,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.98,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.516,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 3.237,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.571,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 29.321,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 10.344,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 25.369,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 48.27,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 807,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 807,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 807,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 807,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
          "distinct": false,
          "id": "f5bcafc73642f096bec5468a9c73f1298de89926",
          "message": "CHANGELOG, docs/RELEASE.md: MCP Registry and Desktop Extension releases\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-21T06:06:16Z",
          "tree_id": "63d8957dc45c01ebfdc0e04feb8dec8ab2489ed8",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f5bcafc73642f096bec5468a9c73f1298de89926"
        },
        "date": 1792564221680,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6590.493,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6613.992,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6604.395,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6740.577,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 59.753,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 44.384,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 59.879,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 83.965,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.05,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.807,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 9.302,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 23.92,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 31.884,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 29.93,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 31.591,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 30.308,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.709,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.276,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.796,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.865,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 16.898,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.545,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.595,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 32.653,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 825.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 825.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 825.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 825.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 23,
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
            "value": 27,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}