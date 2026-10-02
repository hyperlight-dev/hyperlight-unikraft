window.BENCHMARK_DATA = {
  "lastUpdate": 1790985068938,
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
      }
    ]
  }
}