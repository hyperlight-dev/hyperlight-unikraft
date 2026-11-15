# CI

CI runs in three tiers, by how much of the hosts' time a change can take. These are CI tiers, not the [guest runtime support tiers](guest-support-tiers.md).

| Tier | Hosts | Runs on | Blocks a merge |
|---|---|---|---|
| 1 | Linux x86_64, Windows x86_64 (hosted runners) | every pull request and push to `main` | yes |
| 2 | macOS arm64, Windows arm64 (self-hosted) | pull requests labelled `ci/arm64` | yes, when labelled before `ci-passed` starts |
| 3 | Linux arm64, macOS arm64, Windows arm64 (self-hosted) | nightly on `main` | no |

`ci-passed` is the one required check: it passes when every tier-1 job the pull request calls for passes, and, on a pull request labelled `ci/arm64`, when its tier-2 run passes too. The labels are read when `ci-passed` starts: after adding `ci/arm64` later, re-run it.

## Tier 1: every change

[`ci.yml`](../.github/workflows/ci.yml): lint, spell check, the kernel's build, the guest images, the Rust tests, the static Linux binaries, the demos, the integrations (smolagents, MCP, verl, the OpenAI Agents SDK and ClickHouse, those last three on Linux only), CPython's conformance suite and the benchmarks, on Linux and Windows. It also builds and runs the unit tests on hosted arm64 runners (macOS, Linux, Windows), which can boot no guest.

A pull request runs what its files call for:

| The pull request changes | Jobs |
|---|---|
| documentation alone: markdown, `docs/`, `site/` (not `templates/`, which `hluk` embeds) | spell check |
| `integrations/<name>/` alone, with documentation or not | lint, the Rust tests, and that integration's jobs (smolagents brings MCP's, which runs its guest through it) |
| anything else | everything |

The `ci/integrations` label adds every integration's jobs to any pull request. It is read when the `changes` job starts: after adding it, use Re-run all jobs (re-running the failed ones keeps the `changes` job's result). A push to `main` runs everything.

## Tier 2: on request

[`arm64.yml`](../.github/workflows/arm64.yml) on macOS (Hypervisor.framework) and Windows arm64 (WHP): the Rust tests, the demos, the smolagents and MCP integrations, the conformance suite and the benchmarks, with the guest images built on a hosted arm64 runner.

To request it, a maintainer labels the pull request `ci/arm64`. A pull request without it, or another label added, gets a single skipped `arm64` check: the jobs themselves are in [`arm64-jobs.yml`](../.github/workflows/arm64-jobs.yml), which `arm64.yml`'s one job calls. A pull request from this repository runs again on each push; one from a fork runs when labelled, so remove and re-add the label to test new commits. Dependabot labels the updates that can break an arm64 host or guest (Hyperlight and the other crates of `hluk`, the guest-side wasmtime crates, the Rust toolchain, the images' bases): see [`dependabot.yml`](../.github/dependabot.yml).

## Tier 3: nightly

`arm64.yml` again, at 03:17 UTC on `main`, on all three arm64 hosts. It is the only scheduled run on the self-hosted Linux arm64 host (tier 1's hosted arm64 runners boot no guest), a small VM under nested virtualization, where a guest's first touch of each page is slow. There the tests run with small guests, and Sunday's run adds the slow suites (agent, Java, node, .NET JIT, PowerShell) and the agent benchmarks, which take hours. The benchmarks of each nightly run are charted on the [benchmark pages](https://hyperlight-dev.github.io/hyperlight-unikraft).

A push to `main` runs no arm64 test, as the next nightly run does. The push's arm64 run only refreshes the guest images' cache, on a hosted runner, so that the pull requests after it need not build them. To run it on a branch, start `arm64` by hand (Actions, Run workflow); `weekly` adds Sunday's suites.

## Coverage

[`coverage.yml`](../.github/workflows/coverage.yml) measures how much of the host side, the library and `hluk`, the Rust tests cover. It runs at 05:29 UTC on Mondays on `main`, and on a pull request that changes it; to run it on a branch, start it by hand (Actions, Run workflow). It blocks nothing.

It runs the tests as tier 1's Linux test job does, on a hosted x86_64 runner with the guest images of `ci.yml`'s cache (built on a miss), and measures `hluk` where the integration tests run it as a program too. The guest, the kernel and the drivers, is not measured. The run's page has the summary, per file, and its artifacts the reports: `coverage-html`, per line, and `coverage-lcov`, an LCOV file for an editor or a coverage service.

Locally, on Linux or macOS, with the rootfs CPIOs and test binaries `just test` needs: `just coverage-run` runs the tests instrumented, with cargo-llvm-cov, which it installs when missing, and then `just coverage` prints the summary, `just coverage-html` writes `target/coverage/html/` and `just coverage-lcov` `target/coverage/lcov.info`. It takes cargo test's arguments, as `just test` does: `just coverage-run -- snapshot` runs the tests whose names have `snapshot` in them.

## Failures on main

[`ci-report.yml`](../.github/workflows/ci-report.yml) handles a failed run on `main`: the nightly arm64 run, the weekly coverage run, and the CI and arm64 runs of a push.

1. It re-runs the failed jobs once.
2. Each job that fails again gets an issue with the job's link, the commit and the end of its log. Each later run it fails in comments on that issue while the issue is open.

| Run | Issue title | Label |
|---|---|---|
| nightly arm64 | `arm64 nightly: <job> fails` | `ci/nightly` |
| CI, on a push | `CI on main: <job> fails` | `ci/main` |
| arm64, on a push (the images' cache) | `arm64 on main: <job> fails` | `ci/main` |
| weekly coverage | `weekly coverage: <job> fails` | `ci/main` |

Of a push's runs, only the newest of each workflow is re-run: an older commit's re-run would cancel the newer one's, and its failures may be fixed already. An older run is reported only when it failed again. A CI run that passes on its re-run may start no dev build, as GitHub starts no workflow for most of what a workflow's token does: the next push's run does, or start `dev.yml` by hand. A job that reaches its timeout counts as failed; a run cancelled by hand, or by a newer push, does not. `ci-passed` is reported only when it fails alone. A run that does not start, because GitHub refused the workflow file or an action it uses, gets an issue at once. The reporter runs when such a run ends and every two hours, and reports each failed job once per attempt of a run.
