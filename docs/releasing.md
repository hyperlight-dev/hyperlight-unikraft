# Releasing hyperlight-unikraft

All examples, kernels, and GHCR images share a single version derived
from `host/Cargo.toml`.

## Steps

1. **Bump the version** — open a PR that updates `version` in
   `host/Cargo.toml` (e.g., `"0.1.0"` → `"0.2.0"`). Merge it to main.

2. **Run the release workflow** — go to
   **Actions → Create release → Run workflow**. No input needed — the
   workflow reads the version from `Cargo.toml`.

3. **What happens automatically:**
   - A `v<version>` git tag is created on main.
   - A GitHub Release is created with auto-generated notes.
   - The publish workflow is triggered, pushing all GHCR images with
     both `:latest` and `:v<version>` tags.

## Versioning policy

- New examples or language support → minor bump (0.1.0 → 0.2.0).
- Bug fixes or documentation → patch bump (0.1.0 → 0.1.1).
- Breaking API changes → major bump (0.x → 1.0.0, once stable).

All images are republished on every release, even if unchanged. GHCR
tags are idempotent, so this is harmless.

## crates.io

Publishing to crates.io is blocked until `hyperlight-host` with the
`disk_snapshot_copy` APIs is available on crates.io (see [#27]).
Once unblocked, a `cargo publish` step should be added to the release
workflow.

[#27]: https://github.com/hyperlight-dev/hyperlight-unikraft/issues/27
