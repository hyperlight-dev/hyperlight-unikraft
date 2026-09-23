window.BENCHMARK_DATA = {
  "lastUpdate": 1790122565299,
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
      }
    ]
  }
}