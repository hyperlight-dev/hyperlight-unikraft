window.BENCHMARK_DATA = {
  "lastUpdate": 1793162061219,
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
        "date": 1789240120180,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7406.462,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7387.907,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7372.973,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 29.964,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.309,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 47.087,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 10.19,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.537,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.759,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.509,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.4,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.475,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.108,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.213,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.384,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.676,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.753,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 875.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 875.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 875.9,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
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
        "date": 1789242767701,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7566.762,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7596.381,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7515.693,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 30.967,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.315,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 49.748,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 10.632,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.297,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 22.188,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.562,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.44,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.524,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.56,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.081,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.049,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.313,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.151,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.883,
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
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
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
        "date": 1789245072146,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5998.596,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5912.052,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5904.696,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 24.379,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.397,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.893,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.406,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.548,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.526,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.244,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.219,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.359,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.992,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.068,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.529,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.453,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 6.516,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.115,
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
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
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
        "date": 1789247429635,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6389.251,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6522.324,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6413.899,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 22.573,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.843,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.081,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.939,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 3.733,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.879,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 0.963,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.955,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.001,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.609,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.1,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.455,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.484,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 5.265,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 24.776,
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
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
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
        "date": 1789248008780,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7390.659,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7387.427,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7417.653,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 29.204,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.217,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 46.656,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 9.725,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 4.295,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.754,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.372,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.378,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.419,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.529,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.098,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.334,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 17.041,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 7.001,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.701,
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
            "value": 28,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 30,
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
        "date": 1789945811029,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7638.86,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7674.388,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7515.244,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 30.458,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.931,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 46.52,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.745,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.961,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.799,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.919,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 3.626,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.752,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.614,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.178,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.214,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.987,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.229,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.462,
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
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
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
          "id": "f54d18ca2245c5c776660a0b0b44196b67d0f437",
          "message": "Merge pull request #115 from hyperlight-dev/ci-bench-hardening\n\nci, bench: report a hung benchmark as itself, and show how far it got",
          "timestamp": "2026-09-22T10:58:32-07:00",
          "tree_id": "7698416ca212eb8142169d86ac58bfe176aa601f",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/f54d18ca2245c5c776660a0b0b44196b67d0f437"
        },
        "date": 1790100294352,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4944.02,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4959.026,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4944.929,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 22.685,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.849,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.936,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.294,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.464,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.821,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.683,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.569,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.675,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.493,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.097,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.793,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.049,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.14,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.468,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
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
          "id": "fb62e312716e874ca205079533bb0caad699cf33",
          "message": "Merge pull request #116 from hyperlight-dev/v0.14.1-urunc-fixes\n\nFixes and additions from the urunc port: AF_UNSPEC disconnect, --port all, --resolv-conf",
          "timestamp": "2026-09-22T16:00:42-07:00",
          "tree_id": "f5acc639381de1da38f8591283ce7421427365e3",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/fb62e312716e874ca205079533bb0caad699cf33"
        },
        "date": 1790118478568,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5957.751,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5857.06,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5909.949,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 24.695,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.694,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 30.896,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.414,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.694,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.026,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.908,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.958,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.005,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.022,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.162,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.603,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.095,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.831,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 20.43,
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
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
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
        "date": 1790121909218,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7611.73,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7571.696,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7550.012,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 31.208,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.796,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 50.176,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.035,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.02,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 20.1,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.7,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 3.475,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.841,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.701,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.234,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.14,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.745,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.029,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.181,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
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
          "id": "2b4790d1626e2645dd1f00469311424be8cb2f94",
          "message": "Merge pull request #117 from hyperlight-dev/hostfs-mounts-on-resume\n\nv0.14.1: Restore a snapshot with any mounts",
          "timestamp": "2026-09-22T21:30:13-07:00",
          "tree_id": "9e7c5f5de22880c141cdcdd15811ec44d0aee070",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/2b4790d1626e2645dd1f00469311424be8cb2f94"
        },
        "date": 1790138493792,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7468.021,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7441.945,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7473.997,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7472.87,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 30.069,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.956,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 18.049,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.34,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.638,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.985,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.912,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.947,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.784,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 3.66,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.403,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.915,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.635,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.249,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.915,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.311,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.823,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.311,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.651,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.759,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 27,
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
        "date": 1790236463169,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5904.542,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5894.789,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 5949.296,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5943.214,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 24.537,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.417,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.52,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 38.556,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.207,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.539,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.062,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.276,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.975,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.792,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.621,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.952,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.064,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.148,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.673,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.537,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.702,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.434,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.871,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.236,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 873.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 873.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 873.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 873.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 29,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 27,
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
        "date": 1790367493429,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 7781.318,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 7718.351,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 7897.396,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 7385.003,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 30.761,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 12.986,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 17.702,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 47.013,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.712,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.084,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.725,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.303,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.206,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.062,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.465,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.739,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.579,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.264,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.916,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.312,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.399,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.259,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.203,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 26.468,
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
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 27,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 28,
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
        "date": 1790459610069,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4950.409,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4936.788,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4922.668,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4966.936,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 21.373,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.303,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.155,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.029,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.496,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.383,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.883,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.652,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.064,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.979,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.455,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.173,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.052,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.156,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.652,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.568,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.142,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.096,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.48,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 20.38,
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
            "value": 10,
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
        "date": 1790664878605,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6650.009,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6524.356,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6587.697,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6459.984,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 28.972,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.712,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.668,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 46.131,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 8.246,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.913,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.814,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.5,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.699,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.45,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.017,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.689,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.585,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.213,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.922,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.602,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 13.153,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.081,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.17,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.304,
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
            "value": 10,
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
        "date": 1790954417169,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6078.133,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6097.186,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6080.845,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6100.276,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.634,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.819,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.519,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.337,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.602,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.809,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.58,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.721,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.329,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.316,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.88,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.428,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.569,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.195,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.935,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.344,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.483,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.905,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.965,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.513,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1790955861543,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5002.28,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5107.419,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 5076.772,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5040.926,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.719,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.011,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.833,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 29.501,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.965,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.672,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.217,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.492,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.331,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.103,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.737,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.412,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.094,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.144,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.746,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.706,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.993,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.622,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.676,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 20.762,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1790961983515,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6364.704,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6285.256,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6294.515,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6259.558,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.613,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.06,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.942,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.332,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.056,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.864,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.663,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.172,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.658,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.417,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.969,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.634,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.591,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.151,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.871,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.338,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.642,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.811,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.892,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.673,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1790984080511,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4264.183,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4123.537,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4082.972,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4148.369,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.489,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 6.943,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.896,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 27.637,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.479,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.472,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.051,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.789,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.285,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.006,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.626,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.382,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.482,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.076,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.49,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.766,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.661,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 1.984,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.222,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.552,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 822.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1790985067848,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6068.052,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6060.471,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6112.558,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6132.39,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.531,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.93,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.858,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.363,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.553,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.912,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.703,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.519,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.391,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.268,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.992,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.397,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.625,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.18,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.955,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.24,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.184,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.998,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.597,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.713,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792006898428,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6534.336,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6491.959,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6443.509,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6418.835,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.08,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.348,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.121,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 30.986,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.494,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.896,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.685,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.658,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.648,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.404,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.21,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.621,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.351,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.129,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.811,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.351,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.006,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.668,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.023,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.257,
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
            "value": 10,
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
        "date": 1792012934443,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6196.346,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6265.899,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6122.252,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6167.798,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.209,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.81,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.393,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 42.999,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.512,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.702,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.617,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.273,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.689,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.483,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.264,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.648,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.605,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.15,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.879,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.265,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.995,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.956,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.823,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.437,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792014341696,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4907.921,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4938.405,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4915.7,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4846.036,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.448,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.847,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.473,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 36.943,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.604,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.514,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.034,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.465,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.309,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.279,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.712,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.364,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.992,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.1,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.585,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.507,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.823,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.395,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.471,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.208,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1792085739824,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6167.652,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6222.922,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6232.147,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6216.394,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.935,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.456,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 16.047,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 36.882,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.982,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.907,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 4.099,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.583,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.726,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.596,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.338,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.785,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.604,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.185,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.833,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.288,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.127,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.138,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.418,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 26.831,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1792087617217,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6424.144,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6481.405,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6428.348,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6425.5,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.98,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.543,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.253,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.045,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.027,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.478,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.281,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.123,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.121,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.766,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.391,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.775,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.365,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.158,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.735,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.606,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.19,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.34,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.834,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 28.815,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1792091528790,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4144.729,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4194.456,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4187.308,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4173.552,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.246,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.995,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.711,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.321,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.237,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.367,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.902,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.617,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.352,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.166,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.793,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.38,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.586,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.087,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.534,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.908,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.559,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.029,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.095,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.571,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792094269380,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4973.589,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4852.854,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4883.532,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4925.34,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.811,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.273,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.743,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 27.054,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.404,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.538,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.989,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.747,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.388,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.275,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.832,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.434,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.978,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.104,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.643,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.571,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.481,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.491,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.681,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.928,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792106298037,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4834.133,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4784.062,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4813.613,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4826.576,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.123,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.066,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.488,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.493,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.43,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.493,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.89,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.092,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.354,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.301,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.814,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.439,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.993,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.15,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.666,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.541,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.72,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.32,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.518,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.662,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792108277865,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4889.802,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4818.055,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4916.401,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4911.259,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.03,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.725,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.309,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 26.897,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.167,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.538,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 2.936,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.353,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.335,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.329,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.818,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.67,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.99,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.169,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.683,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.552,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.455,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.444,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.644,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.315,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792109212334,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6385.974,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6373.022,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6347.199,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6411.944,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.939,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.876,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.965,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 41.476,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.631,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.681,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.609,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.875,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.854,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.703,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.535,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.977,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.712,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.228,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.894,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.304,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.006,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.831,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.983,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.884,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792111560547,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4289.692,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4333.122,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4423.347,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4281.337,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 13.672,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.279,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.516,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 23.969,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.026,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.53,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.171,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 11.551,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.297,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.258,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.021,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.233,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.603,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.095,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.663,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.521,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.348,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.211,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.425,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 16.722,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792112706990,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6183.897,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6284.041,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6190.942,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6676.547,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.127,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.708,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.911,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.044,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.51,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.742,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.615,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.266,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.799,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.617,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.408,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.081,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.655,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.213,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.955,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.479,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.013,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.929,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.856,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 24.876,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792396040855,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6343.315,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6358.791,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6438.187,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6374.73,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.464,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 10.74,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.871,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.585,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.482,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.013,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.741,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.141,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.698,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.098,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.324,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.842,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.624,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.217,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.9,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.256,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.163,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.066,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.113,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.101,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792416206085,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6258.512,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6184.828,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6183.47,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6206.281,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.996,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.222,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.204,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.217,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.795,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.743,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.601,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.512,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.803,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.586,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.24,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.777,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.637,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.228,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.952,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.2,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.245,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.103,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.075,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.982,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792480620614,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4812.534,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4826.588,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 5047.08,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5069.278,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.099,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.045,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.581,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 35.492,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.268,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.507,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.14,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.869,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.333,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.307,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.147,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.701,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.047,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.122,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.666,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.63,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.466,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.29,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.781,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 20.02,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792485735486,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6468.267,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6513.202,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6552.495,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6605.808,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.522,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.715,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.003,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 37.107,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.264,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.737,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.748,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.418,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.785,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.643,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.484,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.164,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.323,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.146,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.753,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.411,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 9.535,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.707,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.003,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.154,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 823,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792555680867,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4607.457,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4379.741,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4381.947,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4323.838,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.5,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.567,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.302,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.172,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.971,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.899,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.446,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.404,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.744,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.586,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.254,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.728,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.646,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.11,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.667,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.98,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.372,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.547,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.697,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.777,
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
        "date": 1792556838690,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6172.917,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6224.548,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6170.329,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6272.839,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 18.121,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.959,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 16.452,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.22,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.769,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.908,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 4.06,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.362,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.658,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.484,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.387,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.764,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.711,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.247,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.945,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.477,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.473,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.17,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.363,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.209,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1792563179304,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6175.047,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6154.997,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6241.833,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6241.378,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.095,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.394,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.193,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 42.342,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.329,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.693,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.566,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.197,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.586,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.664,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.269,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.919,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.699,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.247,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.937,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.243,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.029,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.633,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.746,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 25.036,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792564129580,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6227.366,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6213.506,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6245.992,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6236.316,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.694,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.948,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.809,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.367,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.731,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.966,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.94,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.666,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.478,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.338,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.126,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.751,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.673,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.227,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.961,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.393,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.521,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.193,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.103,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 24.794,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 824.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 824.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 824.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 824.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 11,
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
        "date": 1792608298352,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4459.933,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4516.735,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4508.311,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4619.247,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.176,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.966,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.933,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.237,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.71,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.801,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.637,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.489,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.662,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.63,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.351,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.042,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.546,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.112,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.613,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.099,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.232,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.49,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.94,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 18.799,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 807.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 807.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 807.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 807.1,
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
        "date": 1792610509777,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5213.002,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5020.017,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4917.614,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4825.402,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 14.702,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.705,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 13.351,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.266,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.631,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.518,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.064,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 14.694,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.55,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.298,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.866,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.588,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.035,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.141,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.681,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.614,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.667,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.372,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.75,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.834,
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
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 7,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 9,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 9,
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
        "date": 1792624291027,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6384.492,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6386.478,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6482.494,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6336.296,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 19.434,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 12.181,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 18.6,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 45.971,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.339,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.32,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 4.527,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.543,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.429,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 3.673,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.342,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.742,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.703,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.237,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.95,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.309,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.294,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.481,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 7.048,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 26.966,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 824.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 824.9,
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
        "date": 1792625083210,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6482.678,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6526.011,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6619.204,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6566.093,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 20.743,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 11.822,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 18.963,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 37.715,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 7.848,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.36,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 4.478,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.931,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.804,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 3.49,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.449,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.925,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.675,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.246,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.999,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.205,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.8,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.472,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.924,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.134,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 825,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 825,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 825,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 825,
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
        "date": 1792627615386,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4435.961,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4458.055,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4490.095,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5366.987,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.299,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.822,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 16.2,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 33.547,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.56,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.744,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.709,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 15.432,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.68,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.628,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.709,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.393,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.521,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.093,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.674,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.454,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.2,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.436,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.159,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 21.251,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 828.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 828.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 828.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 828.2,
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
          "id": "b11c33e8d180dd06ce5087a575a77100c029fef4",
          "message": "arm64-host: a job-scoped hluk cache\n\nPoint HLUK_CACHE_DIR at the job's temp directory on the self-hosted\nhosts, which the runner empties when the job ends, cancelled or not, so\nthe images a job pulls and the warm snapshots it saves go with it,\ninstead of staying in the runner user's home for later jobs, and\nhyperlight's, to find the disk full.\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-22T00:42:05Z",
          "tree_id": "fb01edfe62fe98b17eeca80462299a1a01013e18",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/b11c33e8d180dd06ce5087a575a77100c029fef4"
        },
        "date": 1792631312750,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6148.34,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6124.998,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6132.963,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6148.386,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.284,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.677,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.74,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 34.481,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.614,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 2.025,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.67,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 18.227,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.245,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.21,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.004,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.343,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.59,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.188,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.896,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.012,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.595,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.959,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.609,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 24.908,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 828.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 828.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 828.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 828.3,
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
            "value": 10,
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
        "date": 1792685760632,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6084.785,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6115.416,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6105.671,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6115.618,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 27.113,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.748,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 16.037,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.766,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.818,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.948,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.805,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.744,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.339,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.338,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.008,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.418,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.623,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.207,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.845,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.28,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.898,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.081,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.441,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.977,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834.6,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834.6,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
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
            "value": 10,
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
        "date": 1792693125041,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6029.641,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6022.005,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6026.508,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6039.508,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 16.993,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.285,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.59,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 31.085,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.663,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.928,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.82,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.134,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.291,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.255,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.954,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.292,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.651,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.241,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.838,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.24,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 11.621,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.151,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.246,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.456,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792701788912,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6075.479,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6093.942,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6086.595,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6081.991,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 26.834,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.593,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.542,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.206,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.995,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.874,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.783,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 16.852,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.309,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.269,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.98,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.331,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.651,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.208,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.855,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.268,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 12.129,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.258,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 6.402,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 24.663,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792704722595,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4204.469,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4276.665,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4216.004,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4332.446,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 22.456,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.927,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.415,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 26.453,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.533,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.033,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.84,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.016,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.941,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.558,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.071,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.584,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.079,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.558,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.115,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.93,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.236,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.331,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.412,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792718939691,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4934.685,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4927.801,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4843.487,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4936.374,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 15.806,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 8.983,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 14.074,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 28.295,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.606,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.734,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.286,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 13.789,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.389,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.317,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.855,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.437,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.345,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.106,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.675,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.326,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 8.607,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.621,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.795,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 19.837,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792723579535,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6171.606,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6166.584,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6163.389,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6169.043,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 17.269,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 9.875,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 15.787,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 32.806,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 6.724,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.914,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.816,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 17.392,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.313,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.238,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.968,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.37,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.587,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.197,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.864,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.073,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 10.84,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 3.023,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 5.821,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 23.863,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834.1,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792725855737,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 5486.652,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 5427.07,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 5453.072,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 5417.24,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 21.199,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 7.935,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 12.101,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 25.035,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 5.811,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 1.62,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 3.208,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 12.768,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.454,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.363,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.95,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.522,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.73,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.116,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.659,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 7.779,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 7.946,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 2.209,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 4.393,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 17.978,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 834,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 834,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 834,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 834,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 8,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 10,
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
        "date": 1792790660975,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6273.357,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6286.165,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6258.997,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6300.171,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 54.119,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 37.325,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 42.379,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 57.196,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.273,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 12.698,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.992,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 26.736,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.559,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.623,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.162,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.739,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.61,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.19,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.892,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.351,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.383,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 20.157,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.985,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 38.795,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 840.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 840.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 840.4,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 840.4,
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
            "value": 12,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 14,
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
        "date": 1792791492563,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6232.016,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6199.67,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6236.69,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6238.825,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 42.934,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.268,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 41.905,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 55.968,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.4,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 12.164,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.875,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 26.083,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.788,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.741,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.443,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.892,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.68,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.24,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.055,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.247,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 26.998,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 19.145,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.993,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 38.55,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 839.9,
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
            "name": "rss/mount",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 13,
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
        "date": 1792799747869,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 3943.56,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 3945.849,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 3930.583,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 3937.491,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 33.013,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 28.249,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 33.483,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 42.579,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.204,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 9.143,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 10.272,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.165,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.022,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 0.962,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.527,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.039,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.531,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.091,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.522,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.564,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.129,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 13.946,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 15.628,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.618,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 840.2,
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
            "value": 14,
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
        "date": 1792800949556,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4933.102,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4951.274,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4996.295,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4984.862,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 44.023,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 30.849,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 35.26,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 46.608,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.601,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 10.501,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.01,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 21.968,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.575,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.501,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.072,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.53,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.039,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.164,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.675,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.547,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 22.219,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.811,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.652,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.674,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 839.9,
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
            "name": "rss/mount",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 13,
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
        "date": 1792801681481,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6075.478,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6108.732,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6099.419,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6092.402,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 51.779,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 35.617,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 40.925,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 55.369,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 16.695,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 11.787,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 13.314,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 25.753,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.446,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.422,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.109,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.5,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.691,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.241,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.895,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.311,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.381,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 19.357,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.244,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 38.182,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 840.2,
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
            "value": 14,
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
        "date": 1792802442637,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6166.874,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6188.7,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6185.878,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6190.436,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 52.485,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 36.132,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 41.312,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 55.726,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 17.019,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 12.038,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 14.119,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 26.199,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.536,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.473,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.27,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.603,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.698,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.233,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.861,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.171,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 27.414,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 19.189,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 21.854,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 38.491,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 840.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 840.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 840.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 840.1,
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
            "value": 14,
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
        "date": 1792803240947,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 4815.264,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 4822.3,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4823.2,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4824.993,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 35.457,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 29.962,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 34.096,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 46.013,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 13.578,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 10.038,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.318,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 21.642,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.168,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.125,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.725,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.233,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.114,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.163,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.644,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 8.571,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 21.097,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.538,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.482,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 30.665,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 840.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 840.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 840.1,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 840.1,
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
            "value": 14,
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
        "date": 1792995607312,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 3947.205,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 3960.592,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 4047.051,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 4120.26,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 35.696,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 30.527,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 34.817,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 44.245,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 12.513,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 9.544,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.321,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 19.693,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 1.206,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 1.162,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 1.859,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 1.361,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 1.488,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.066,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.529,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 6.632,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 18.975,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 14.885,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 16.25,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 27.729,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 839.9,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 839.9,
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
            "name": "rss/mount",
            "value": 13,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 13,
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
        "date": 1792996929121,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6368.989,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6335.626,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6312.877,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6366.79,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 39.261,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 27.86,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 31.172,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 45.615,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.536,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 10.473,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 11.301,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 24.165,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 2.4,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.151,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 2.73,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 2.392,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.513,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.191,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.759,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 11.709,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 24.595,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 15.587,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.85,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 37.651,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 840.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 840.2,
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
            "value": 14,
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
        "date": 1793125578511,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6339.768,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6314.296,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6309.901,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6337.481,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 36.867,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 30.66,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 36.089,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 48.727,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 15.044,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 11.104,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.905,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 24.483,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.327,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.863,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.849,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.324,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.393,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.152,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.805,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.231,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 23.15,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 16.252,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 18.242,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 34.985,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
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
        "date": 1793126706408,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6560.564,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6560.541,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6489.736,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6465.703,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 48.805,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 42.119,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 47.09,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 61.987,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 19.753,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 15.25,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 16.826,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.711,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.052,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.058,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.747,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 4.138,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.688,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.246,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.916,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.332,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 31.209,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 23.507,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 26.189,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 43.427,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
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
        "date": 1793140044749,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6687.802,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6706.377,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6749.169,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6673.416,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 49.513,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 42.531,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 48.484,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 62.647,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.395,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 15.74,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 17.716,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.971,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.332,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.253,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 5.133,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 4.321,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.675,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.242,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.023,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.345,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.21,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 24.192,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 27.029,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.28,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
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
        "date": 1793142415488,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6707.97,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6637.674,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6576.63,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6570.904,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 50.425,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 42.569,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 47.162,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 62.562,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.951,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 15.447,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 17.14,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.012,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.328,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.066,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.796,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 4.133,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.711,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.224,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.018,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.674,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 31.878,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 24.06,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 26.817,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.195,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
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
        "date": 1793146665993,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6662.142,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6660.052,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6667.589,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6676.103,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 51.508,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 43.529,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 49.448,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 64.642,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.282,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 15.759,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 17.397,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 30.706,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.206,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.242,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 4.946,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 4.494,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.706,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.229,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 1.01,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.347,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.054,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 24.29,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 26.812,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.825,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
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
        "date": 1793148918448,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6216.776,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6178.459,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 5785.042,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6212.437,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 36.691,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 28.075,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 34.792,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 49.352,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 14.751,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 10.01,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 12.089,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 24.953,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 3.306,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 2.549,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 3.499,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 3.743,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.33,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.154,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.75,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 9.982,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 22.883,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 14.948,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 17.557,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 36.716,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.2,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
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
          "id": "6f310ffe652c5f8a3cd926709a653afb1adca613",
          "message": "docs(changelog): one-line cpio entry with the feature and /etc files\n\nSigned-off-by: danbugs <danilochiarlone@gmail.com>",
          "timestamp": "2026-10-28T02:03:45Z",
          "tree_id": "0ba36047e3c580e1ed9e8c3d3467855d31f69d62",
          "url": "https://github.com/hyperlight-dev/hyperlight-unikraft/commit/6f310ffe652c5f8a3cd926709a653afb1adca613"
        },
        "date": 1793162060034,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "cold/compute",
            "value": 6602.864,
            "unit": "ms"
          },
          {
            "name": "cold/hello",
            "value": 6508.224,
            "unit": "ms"
          },
          {
            "name": "cold/mount",
            "value": 6608.532,
            "unit": "ms"
          },
          {
            "name": "cold/stdlib",
            "value": 6624.241,
            "unit": "ms"
          },
          {
            "name": "cold-snap/compute",
            "value": 50.242,
            "unit": "ms"
          },
          {
            "name": "cold-snap/hello",
            "value": 42.897,
            "unit": "ms"
          },
          {
            "name": "cold-snap/mount",
            "value": 48.754,
            "unit": "ms"
          },
          {
            "name": "cold-snap/stdlib",
            "value": 62.491,
            "unit": "ms"
          },
          {
            "name": "warm-restore/compute",
            "value": 20.23,
            "unit": "ms"
          },
          {
            "name": "warm-restore/hello",
            "value": 15.447,
            "unit": "ms"
          },
          {
            "name": "warm-restore/mount",
            "value": 17.544,
            "unit": "ms"
          },
          {
            "name": "warm-restore/stdlib",
            "value": 29.492,
            "unit": "ms"
          },
          {
            "name": "restore-cost/compute",
            "value": 4.128,
            "unit": "ms"
          },
          {
            "name": "restore-cost/hello",
            "value": 4.1,
            "unit": "ms"
          },
          {
            "name": "restore-cost/mount",
            "value": 5.014,
            "unit": "ms"
          },
          {
            "name": "restore-cost/stdlib",
            "value": 4.097,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/compute",
            "value": 2.736,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/hello",
            "value": 0.241,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/mount",
            "value": 0.963,
            "unit": "ms"
          },
          {
            "name": "warm-stateful/stdlib",
            "value": 10.265,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/compute",
            "value": 32.677,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/hello",
            "value": 23.868,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/mount",
            "value": 27.1,
            "unit": "ms"
          },
          {
            "name": "parallel-exec/stdlib",
            "value": 44.133,
            "unit": "ms"
          },
          {
            "name": "snapshot-size/compute",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/hello",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/mount",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "snapshot-size/stdlib",
            "value": 841.3,
            "unit": "MiB"
          },
          {
            "name": "rss/compute",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/hello",
            "value": 10,
            "unit": "MB"
          },
          {
            "name": "rss/mount",
            "value": 11,
            "unit": "MB"
          },
          {
            "name": "rss/stdlib",
            "value": 12,
            "unit": "MB"
          }
        ]
      }
    ]
  }
}