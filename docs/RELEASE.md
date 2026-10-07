# Releasing

Releases are driven by the `version` in the root `Cargo.toml` and cut with three workflows in `.github/workflows/`:

| Workflow | Trigger | What it does |
|---|---|---|
| `release.yml` | manual (`workflow_dispatch`) | Tags the version, creates a GitHub Release (notes from the matching `CHANGELOG.md` section) with the `hluk` binaries, and triggers the GHCR publish. |
| `publish-images.yml` | git tag `v*`, or manual | Pushes each runtime as one package (`:latest` base + `:initrd` cpio), plus busybox, the kernel, and the urunc "hello" image to GHCR. |
| `cargo-publish.yml` | manual | Publishes the `hyperlight-unikraft` crate to crates.io. Dry-run by default. |
| `dev.yml` | `main`'s CI passing on a push, or manual (from `main`) | Replaces the `dev` pre-release and publishes the commit's `:dev-<sha7>` images (see [Dev builds](#dev-builds)). |

User-facing changes are recorded in [`CHANGELOG.md`](../CHANGELOG.md) (Keep a Changelog format): changes accumulate under `## [Prerelease] - Unreleased`, then that heading is renamed to the version at release time.

## Cutting a release

1. In [`CHANGELOG.md`](../CHANGELOG.md), rename the top `## [Prerelease] - Unreleased` heading to the version being cut (`## [v<version>]`), confirm it captures what shipped, and add a fresh empty `## [Prerelease] - Unreleased` above it for the next cycle. (`just changelog-notes v<version>` prints exactly what the release notes will be.)
2. If the host side of the guest contract changed since the last release (a host function's meaning, the virtqueue pool sizes, the layout, the guest MSRs (system registers on arm64), or the hyperlight-host dependency), bump `SNAPSHOT_CONTRACT` in [`build.rs`](../build.rs) so saved snapshots are refused rather than misread. A kernel change rolls the snapshot key by itself, and a release that changes neither keeps every saved snapshot loadable.
3. Bump `version` in `Cargo.toml` on `main` (semver, e.g. `0.2.0`), commit, and push (the `CHANGELOG.md` edit can ride in the same commit).
4. Actions → **Create release** → **Run workflow**. It:
   - validates the version and that the tag doesn't exist,
   - builds `hluk` for Linux (x86_64, arm64), macOS (Apple silicon, signed with the hypervisor entitlement) and Windows (x86_64), each on a native runner,
   - creates tag `v<version>` and a GitHub Release whose notes are the matching `CHANGELOG.md` section plus GitHub's auto-generated PR list (via `just changelog-notes v<version>`; the run fails if that section is missing), with the binaries attached (`hluk-v<version>-<target>.tar.gz`/`.zip`) and a `SHA256SUMS` over them; the release stays a draft until the last of these is attached,
   - triggers `publish-images.yml` for that tag.

Nothing is pushed to crates.io as part of this — that is deliberate.

`install.sh` (Linux, macOS) and `install.ps1` (Windows) at the repository root fetch the binary for the host from the latest release (or `HLUK_VERSION`); when a release carries a `SHA256SUMS`, the download must be listed in it and match; only a release from before `SHA256SUMS` existed installs unchecked.

## What's published

- **GitHub Release**: `hluk` binaries for `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `aarch64-apple-darwin` and `x86_64-pc-windows-msvc`, their `SHA256SUMS`, and release notes extracted from the matching [`CHANGELOG.md`](../CHANGELOG.md) section.
- **GHCR** (`ghcr.io/<owner>/<repo>/…`):
  - `<runtime>` — one package per runtime (`python`, `node`, `agent`, `python-shell`, `bash`, `c`, `go`, `rust`, `dotnet-aot`, `dotnet-jit`, `java`, `powershell`, `quickjs`, `wasmtime`), with two tags, each a multi-platform index (linux/amd64 and linux/arm64):
    - `:latest` (+ `:v<version>`) — the rootfs filesystem image; build a custom guest `FROM <registry>/<runtime>`.
    - `:initrd` (+ `:initrd-v<version>`) — the runnable CPIO; `just pull-rootfs <runtime> <registry>` fetches it into `build-elfloader/` to `hluk run` — no local build.
  - `busybox` — the shared BusyBox base at `:latest` (bash/agent/python-shell build on it), for both platforms,
  - `kernel` — the Unikraft elfloader kernel at `/kernel`, the x86_64 or arm64 one per platform,
  - `hello-urunc` — a urunc-runnable OCI image (see `demos/urunc/`).

## Dev builds

Each push to `main` whose CI passes replaces the `dev` channel, so the latest `main` can be installed and run without a release:

| What | Where |
|---|---|
| `hluk` for the four release targets, and a `SHA256SUMS` | the `dev` pre-release; the `dev` tag moves to the commit |
| every runtime, busybox, the kernel and the urunc image | tagged with the commit: `:dev-<sha7>`, and `:initrd-dev-<sha7>` for the CPIO, multi-platform |

A dev `hluk` is built with `HLUK_CHANNEL=dev-<sha7>`, so it pulls the images of its own commit, and `hluk --version` prints its build (`0.17.0+dev.44ee170`). A newer dev build asks for newer tags, so no cache or Docker build hands it an older build's images. `HLUK_VERSION=dev` makes `install.sh` and `install.ps1` install it, and they refuse it while its `SHA256SUMS` is being replaced.

The images go up before the binaries, so a dev binary never names images that are not there yet. Once the binaries are up, the dev images of every build but this one and the one before are deleted, with the untagged platform manifests only they list: the channel keeps only what its installed binaries pull. A run whose commit is older than the one `dev` already points at (a CI re-run) publishes nothing. Builds run side by side, and only the release step is serialized. GitHub keeps one pending run per group, so a run that arrives while another waits replaces it; if an older commit's run replaces a newer one, `dev` is one commit behind until the next push (or re-run the cancelled one). When two builds overlap, the newer one's cleanup can delete the older one's images while it is still joining them, and that run fails; it would have published nothing anyway. `:latest`, `:initrd` and the release tags are left to releases, and GitHub does not count a pre-release as the latest release, so a plain install still gets the last release.

## crates.io

`hyperlight-unikraft` and `hluk` are the same crate (one `[[bin]]`), so this is a single `cargo publish`. Run **Publish to crates.io** manually; it's a dry run unless you set `dry_run = false`.
