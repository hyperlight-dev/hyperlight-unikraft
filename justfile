# hyperlight-unikraft (hluk)
#
# Cross-platform build recipes.
#
# Usage:
#   just build-rootfs python
#   just run python examples/python/hello.py
#   just clean
#
# Rootfs images are built with Docker on Linux (see build-rootfs); on Windows,
# copy the CPIOs into build-elfloader/ and use run/test/conformance — or pull a
# published image with `just pull-rootfs <runtime> <registry>` (no local build).

# Windows: every recipe runs under PowerShell 7 (pwsh, https://aka.ms/pwsh),
# which must be on PATH; recipes that need Docker or a Linux toolchain say so.
set windows-shell := ["pwsh", "-NoLogo", "-NoProfile", "-Command"]

# Directories
root_dir        := justfile_directory()
drivers_dir     := root_dir / "drivers"
build_dir       := root_dir / "build-elfloader"
snapshot_dir    := root_dir / ".snapshots"
examples_dir    := root_dir / "examples"
conformance_dir := root_dir / "conformance"
benchmarks_dir  := root_dir / "benchmarks"

# Per-runtime scratch memory (MiB). Must cover rootfs extraction +
# runtime startup.
scratch_c          := "64"
scratch_rust       := "64"
scratch_go         := "128"
scratch_bash       := "256"
scratch_python     := "256"
scratch_dotnet_aot := "256"
scratch_node       := "512"
scratch_dotnet_jit := "768"
scratch_powershell := "1024"
scratch_agent        := "1536"
scratch_python_shell   := "256"
scratch_agent_custom := "256"

# Internal: resolve per-runtime scratch MiB (single source of truth).
# Used by run, snapshot-save, bench, and conformance recipes.
# Pure `just` expression — no shell — so it works on every platform.
[private]
_scratch-mb runtime:
    @echo {{ if runtime == "c" { scratch_c } \
        else if runtime == "rust" { scratch_rust } \
        else if runtime == "go" { scratch_go } \
        else if runtime == "bash" { scratch_bash } \
        else if runtime == "python" { scratch_python } \
        else if runtime == "dotnet-aot" { scratch_dotnet_aot } \
        else if runtime == "node" { scratch_node } \
        else if runtime == "dotnet-jit" { scratch_dotnet_jit } \
        else if runtime == "powershell" { scratch_powershell } \
        else if runtime == "agent" { scratch_agent } \
        else if runtime == "python-shell" { scratch_python_shell } \
        else if runtime == "agent-custom" { scratch_agent_custom } \
        else { "256" } }}

# ── Build ────────────────────────────────────────────────────────

# Build the hluk CLI binary (release by default, pass `--debug` for debug)
[unix]
build *flags:
    #!/usr/bin/env bash
    set -euo pipefail
    if echo "{{flags}}" | grep -q -- '--debug'; then
        cargo build --manifest-path "{{root_dir}}/Cargo.toml"
    else
        cargo build --release --manifest-path "{{root_dir}}/Cargo.toml"
    fi

[windows]
build *flags:
    if ("{{flags}}" -match '--debug') { cargo build --manifest-path "{{root_dir}}/Cargo.toml" } else { cargo build --release --manifest-path "{{root_dir}}/Cargo.toml" }

# ── Kernel ───────────────────────────────────────────────────────

kernel_dir    := root_dir / "kernel"
kernel_bin    := kernel_dir / "elfloader_hyperlight-x86_64"
kernel_build  := kernel_dir / ".build"

# Native test-fixture kernel (a C main() compiled into the kernel — no
# elfloader/initrd), used by tests/native_kernel.rs.
native_kernel_dir := root_dir / "tests" / "fixtures" / "native-kernel"
native_kernel_bin := native_kernel_dir / "helloworld-native_hyperlight-x86_64"

# Build the Unikraft elfloader kernel from submodule sources.
# Uses Docker for reproducible builds — the same binary on every machine.
# Requires: Docker, and the kernel submodules (git submodule update --init).
[unix]
build-kernel:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f "{{kernel_dir}}/unikraft/Makefile" ]; then
        echo "error: kernel submodules not initialised" >&2
        echo "run: git submodule update --init --recursive" >&2
        exit 1
    fi

    echo "==> Building kernel builder image..."
    docker build -q -t hluk-kernel-builder \
        -f "{{kernel_dir}}/Dockerfile.build" "{{kernel_dir}}/"

    echo "==> Building kernel inside Docker (reproducible toolchain)..."
    docker run --rm \
        -v "{{kernel_dir}}:/kernel" \
        -v "{{root_dir}}/defconfig-elfloader:/defconfig-elfloader:ro" \
        -e HOST_UID="$(id -u)" -e HOST_GID="$(id -g)" \
        hluk-kernel-builder bash -c '
            set -euo pipefail
            rm -rf .build
            mkdir -p .build app-elfloader/workdir/libs
            ln -sfn /kernel/unikraft app-elfloader/workdir/unikraft
            ln -sfn /kernel/libs/libelf app-elfloader/workdir/libs/libelf
            ln -sfn /kernel/.build app-elfloader/workdir/build
            cp /defconfig-elfloader app-elfloader/.config
            cd app-elfloader
            yes "" 2>/dev/null | make WITH_LWIP=n olddefconfig || true
            make WITH_LWIP=n -j$(nproc)
            cd ..
            cp .build/elfloader_hyperlight-x86_64 elfloader_hyperlight-x86_64
            chown "$HOST_UID:$HOST_GID" elfloader_hyperlight-x86_64
            chown -R "$HOST_UID:$HOST_GID" .build
            rm -f app-elfloader/.config app-elfloader/.config.old
            rm -f app-elfloader/workdir/unikraft app-elfloader/workdir/libs/libelf app-elfloader/workdir/build
            rmdir app-elfloader/workdir/libs app-elfloader/workdir 2>/dev/null || true
        '

    echo "==> Kernel built: {{kernel_bin}}"
    echo "    sha256: $(sha256sum "{{kernel_bin}}" | cut -d' ' -f1)"

[windows]
build-kernel:
    @Write-Error "build-kernel needs Docker on Linux. Build there (just build-kernel) and commit kernel/elfloader_hyperlight-x86_64."; exit 1

# Verify the committed kernel binary matches a fresh build.
# Returns exit 0 if they match, exit 1 if they differ.
[unix]
verify-kernel:
    #!/usr/bin/env bash
    set -euo pipefail
    committed="$(sha256sum "{{kernel_bin}}" | cut -d' ' -f1)"

    if [ ! -f "{{kernel_dir}}/unikraft/Makefile" ]; then
        echo "error: kernel submodules not initialised" >&2
        echo "run: git submodule update --init --recursive" >&2
        exit 1
    fi

    docker build -q -t hluk-kernel-builder \
        -f "{{kernel_dir}}/Dockerfile.build" "{{kernel_dir}}/"

    docker run --rm \
        -v "{{kernel_dir}}:/kernel" \
        -v "{{root_dir}}/defconfig-elfloader:/defconfig-elfloader:ro" \
        -e HOST_UID="$(id -u)" -e HOST_GID="$(id -g)" \
        hluk-kernel-builder bash -c '
            set -euo pipefail
            rm -rf .build
            mkdir -p .build app-elfloader/workdir/libs
            ln -sfn /kernel/unikraft app-elfloader/workdir/unikraft
            ln -sfn /kernel/libs/libelf app-elfloader/workdir/libs/libelf
            ln -sfn /kernel/.build app-elfloader/workdir/build
            cp /defconfig-elfloader app-elfloader/.config
            cd app-elfloader
            yes "" 2>/dev/null | make WITH_LWIP=n olddefconfig > /dev/null 2>&1 || true
            make WITH_LWIP=n -j$(nproc) > /dev/null 2>&1
            cd ..
            chown -R "$HOST_UID:$HOST_GID" .build
            rm -f app-elfloader/.config app-elfloader/.config.old
            rm -f app-elfloader/workdir/unikraft app-elfloader/workdir/libs/libelf app-elfloader/workdir/build
            rmdir app-elfloader/workdir/libs app-elfloader/workdir 2>/dev/null || true
        '

    fresh="$(sha256sum "{{kernel_build}}/elfloader_hyperlight-x86_64" | cut -d' ' -f1)"
    if [ "$committed" = "$fresh" ]; then
        echo "✓ Kernel binary matches source (sha256: $committed)"
    else
        echo "✗ Kernel binary does NOT match source" >&2
        echo "  committed: $committed" >&2
        echo "  fresh:     $fresh" >&2
        echo "  Run 'just build-kernel' to rebuild." >&2
        exit 1
    fi

[windows]
verify-kernel:
    @Write-Error "verify-kernel needs Docker on Linux; run it there."; exit 1

# Build the native test-fixture kernel from source, reproducibly, in the same
# Docker toolchain as the elfloader kernel.  Sources live in the fixture dir;
# the app is built at fixed container paths so the binary is deterministic.
[unix]
build-native-kernel:
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f "{{kernel_dir}}/unikraft/Makefile" ]; then
        echo "error: kernel submodules not initialised" >&2
        echo "run: git submodule update --init --recursive" >&2
        exit 1
    fi

    echo "==> Building kernel builder image..."
    docker build -q -t hluk-kernel-builder \
        -f "{{kernel_dir}}/Dockerfile.build" "{{kernel_dir}}/"

    echo "==> Building native kernel inside Docker (reproducible toolchain)..."
    docker run --rm \
        -v "{{kernel_dir}}:/kernel" \
        -v "{{native_kernel_dir}}:/napp" \
        -e HOST_UID="$(id -u)" -e HOST_GID="$(id -g)" \
        hluk-kernel-builder bash -c '
            set -euo pipefail
            rm -rf /build
            mkdir -p /build/app
            cp /napp/helloworld.c /napp/Makefile.uk /build/app/
            cp /napp/defconfig /build/app/.config
            yes "" 2>/dev/null | make -C /kernel/unikraft A=/build/app olddefconfig || true
            make -C /kernel/unikraft A=/build/app -j$(nproc)
            cp /build/app/build/helloworld-native_hyperlight-x86_64 /napp/helloworld-native_hyperlight-x86_64
            chown "$HOST_UID:$HOST_GID" /napp/helloworld-native_hyperlight-x86_64
        '

    echo "==> Native kernel built: {{native_kernel_bin}}"
    echo "    sha256: $(sha256sum "{{native_kernel_bin}}" | cut -d' ' -f1)"

[windows]
build-native-kernel:
    @Write-Error "build-native-kernel needs Docker on Linux. Build there and commit the fixture binary."; exit 1

# Verify the committed native fixture kernel matches a fresh reproducible build.
[unix]
verify-native-kernel:
    #!/usr/bin/env bash
    set -euo pipefail
    committed="$(sha256sum "{{native_kernel_bin}}" | cut -d' ' -f1)"

    if [ ! -f "{{kernel_dir}}/unikraft/Makefile" ]; then
        echo "error: kernel submodules not initialised" >&2
        echo "run: git submodule update --init --recursive" >&2
        exit 1
    fi

    docker build -q -t hluk-kernel-builder \
        -f "{{kernel_dir}}/Dockerfile.build" "{{kernel_dir}}/"

    docker run --rm \
        -v "{{kernel_dir}}:/kernel" \
        -v "{{native_kernel_dir}}:/napp" \
        -e HOST_UID="$(id -u)" -e HOST_GID="$(id -g)" \
        hluk-kernel-builder bash -c '
            set -euo pipefail
            rm -rf /build
            mkdir -p /build/app
            cp /napp/helloworld.c /napp/Makefile.uk /build/app/
            cp /napp/defconfig /build/app/.config
            yes "" 2>/dev/null | make -C /kernel/unikraft A=/build/app olddefconfig > /dev/null 2>&1 || true
            make -C /kernel/unikraft A=/build/app -j$(nproc) > /dev/null 2>&1
            cp /build/app/build/helloworld-native_hyperlight-x86_64 /napp/.native-verify.bin
            chown "$HOST_UID:$HOST_GID" /napp/.native-verify.bin
        '

    fresh="$(sha256sum "{{native_kernel_dir}}/.native-verify.bin" | cut -d' ' -f1)"
    rm -f "{{native_kernel_dir}}/.native-verify.bin"
    if [ "$committed" = "$fresh" ]; then
        echo "✓ Native kernel binary matches source (sha256: $committed)"
    else
        echo "✗ Native kernel binary does NOT match source" >&2
        echo "  committed: $committed" >&2
        echo "  fresh:     $fresh" >&2
        echo "  Run 'just build-native-kernel' to rebuild." >&2
        exit 1
    fi

[windows]
verify-native-kernel:
    @Write-Error "verify-native-kernel needs Docker on Linux; run it there."; exit 1

# Clean kernel build artifacts (does not touch the committed binary).
[unix]
clean-kernel:
    rm -rf "{{kernel_build}}"

[windows]
clean-kernel:
    if (Test-Path "{{kernel_build}}") { Remove-Item -Recurse -Force "{{kernel_build}}" }

# ── Rootfs ───────────────────────────────────────────────────────

# Build the shared BusyBox base image (hluk-busybox), the NOMMU/PIE userland
# used by the bash, agent and python-shell rootfs.  Built automatically by
# build-rootfs for those runtimes; run directly to refresh it.
[unix]
build-busybox:
    docker build -t hluk-busybox -f "{{drivers_dir}}/busybox.Dockerfile" "{{root_dir}}/"

[windows]
build-busybox:
    @Write-Error "build-busybox needs Docker on Linux."; exit 1

# Build a rootfs CPIO from a driver Dockerfile.
#
# Standard runtimes:   just build-rootfs python
# Custom Dockerfile:   just build-rootfs agent-custom path/to/Dockerfile
[unix]
build-rootfs runtime dockerfile="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "{{dockerfile}}" ]; then
        # Custom Dockerfile path provided
        df="{{dockerfile}}"
        if [ ! -f "$df" ]; then
            echo "error: $df not found" >&2
            exit 1
        fi
    else
        # Standard driver lookup
        df="{{drivers_dir}}/{{runtime}}/Dockerfile"
        if [ ! -f "$df" ]; then
            echo "error: $df not found" >&2
            echo "available runtimes:" >&2
            ls -1 "{{drivers_dir}}" | while read f; do \
                [ -d "{{drivers_dir}}/$f" ] && echo "  $f"; \
            done >&2
            exit 1
        fi
    fi
    image="hluk-{{runtime}}-rootfs"
    output="{{build_dir}}/{{runtime}}-rootfs.cpio"
    mkdir -p "{{build_dir}}"
    # Dependency: build the shared BusyBox base image first if this
    # Dockerfile pulls from it (COPY --from=hluk-busybox).
    if grep -q 'from=hluk-busybox' "$df"; then
        just build-busybox
    fi
    echo "==> Building image $image from $df"
    docker build -t "$image" -f "$df" "{{root_dir}}/"
    just _export-cpio "{{runtime}}" "$image"

[windows]
build-rootfs runtime dockerfile="":
    @Write-Error "build-rootfs needs Docker + cpio on Linux. Build there (just build-rootfs {{runtime}}) and copy build-elfloader/{{runtime}}-rootfs.cpio here."; exit 1

# Export a rootfs docker image to build-elfloader/<runtime>-rootfs.cpio.
# Used by build-rootfs after the docker build.
[private]
[unix]
_export-cpio runtime image:
    #!/usr/bin/env bash
    set -euo pipefail
    output="{{build_dir}}/{{runtime}}-rootfs.cpio"
    mkdir -p "{{build_dir}}"
    echo "==> Exporting {{image}} to $output (newc CPIO)"
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT
    cid=$(docker create --entrypoint=/ "{{image}}" 2>/dev/null || docker create "{{image}}")
    docker export "$cid" | tar -C "$tmpdir" -xf -
    docker rm "$cid" > /dev/null
    # docker export replaces /etc/hosts, /etc/resolv.conf with empty
    # virtual mounts — restore minimal versions.
    # 'unikraft' = Unikraft's default hostname (gethostname()).
    printf '127.0.0.1 localhost unikraft\n::1 localhost unikraft\n' > "$tmpdir/etc/hosts"
    # nsswitch.conf: files first, then DNS for external resolution
    printf 'hosts: files dns\n' > "$tmpdir/etc/nsswitch.conf"
    # resolv.conf: public DNS + single-request (serializes A/AAAA queries;
    # glibc's parallel A+AAAA mode doesn't work correctly through hostsock)
    printf 'nameserver 8.8.8.8\nnameserver 1.1.1.1\noptions single-request\n' > "$tmpdir/etc/resolv.conf"
    (cd "$tmpdir" && find . | cpio -o -H newc --quiet > "$output")
    echo "==> Done: $output ($(du -h "$output" | cut -f1))"

# Pull a published <runtime>:initrd image and drop its CPIO into build-elfloader/
# — run a guest without building locally:
#   just pull-rootfs python <registry>
#   hluk run --initrd build-elfloader/python-rootfs.cpio ...
[unix]
pull-rootfs runtime registry:
    #!/usr/bin/env bash
    set -euo pipefail
    image="{{registry}}/{{runtime}}:initrd"
    output="{{build_dir}}/{{runtime}}-rootfs.cpio"
    mkdir -p "{{build_dir}}"
    docker pull "$image"
    cid=$(docker create --entrypoint=/ "$image" 2>/dev/null || docker create "$image" /)
    docker cp "$cid:/initrd.cpio" "$output"
    docker rm "$cid" >/dev/null
    echo "==> Pulled $output ($(du -h "$output" | cut -f1))"

[windows]
pull-rootfs runtime registry:
    @Write-Error "pull-rootfs needs Docker on Linux; pull there and copy the CPIO."; exit 1

# ── Release ──────────────────────────────────────────────────────

# Print release notes for a version: the curated CHANGELOG.md section, plus
# GitHub's auto-generated PR list (best-effort — needs `gh` and a version tag,
# as in CI).  Mirrors hyperlight's `create-release-notes` (which does the same
# via dev/extract-changelog.sh), kept in the justfile.  Accepts `v0.13.0`,
# `0.13.0`, or `Prerelease`; release.yml pipes it into `gh release create
# --notes-file`.
[unix]
changelog-notes version:
    #!/usr/bin/env bash
    set -euo pipefail
    # Curated summary: the CHANGELOG.md section for this version.
    awk -v ver="{{version}}" '
        BEGIN { h1 = "## [" ver "]"; sub(/^v/, "", ver); h2 = "## [v" ver "]"; h3 = "## [" ver "]" }
        index($0, h1)==1 || index($0, h2)==1 || index($0, h3)==1 { grab=1; next }
        grab && /^## \[/ { exit }
        grab { print }
    ' "{{root_dir}}/CHANGELOG.md" | sed -e '/./,$!d' | sed -e :strip -e '/^\n*$/{$d;N;bstrip}'
    # Plus GitHub's auto-generated PR list, stripped of its own heading.
    if [ "{{version}}" != "Prerelease" ] && command -v gh >/dev/null 2>&1; then
        prs="$(gh api "repos/{owner}/{repo}/releases/generate-notes" \
            -f tag_name="{{version}}" --jq '.body' 2>/dev/null | sed '1,/^## /d')" || true
        if [ -n "${prs:-}" ]; then
            printf '\n## Pull requests\n\n%s\n' "$prs"
        fi
    fi

[windows]
changelog-notes version:
    @Write-Error "changelog-notes uses a POSIX shell; run it on Linux."; exit 1

# ── Publish (GHCR) ───────────────────────────────────────────────
#
# Push images to <registry> (the workflow passes ghcr.io/<owner>/<repo>).  Each
# runtime is ONE package with two tags — the same rootfs in two forms, so the
# registry isn't cluttered with separate packages:
#   <runtime>:latest  — the rootfs filesystem image; build a custom guest
#                       `FROM <registry>/<runtime>`.  (+ :<version> on a tag.)
#   <runtime>:initrd  — the runnable CPIO; `just pull-rootfs` fetches it to
#                       `hluk run`, no local build.  (+ :initrd-<version>.)
# Same scheme for busybox (:latest is its base; no initrd — it's build-only),
# and the kernel / urunc "hello" images.  publish-images.yml logs in and passes
# the version; run locally after `docker login`.  Linux-only (Docker builds).

# Tag a local image into <target>:latest (+ :<version> when given) and push.
[private]
[unix]
_push local target version="":
    #!/usr/bin/env bash
    set -euo pipefail
    docker tag "{{local}}:latest" "{{target}}:latest"
    docker push "{{target}}:latest"
    if [ -n "{{version}}" ]; then
        docker tag "{{local}}:latest" "{{target}}:{{version}}"
        docker push "{{target}}:{{version}}"
    fi

# Build a runtime's rootfs and publish it as one package, two tags:
# <registry>/<runtime>:latest (filesystem base, to build FROM) and
# <registry>/<runtime>:initrd (the runnable CPIO, for pull-rootfs).
[unix]
publish runtime registry version="":
    #!/usr/bin/env bash
    set -euo pipefail
    just build-rootfs "{{runtime}}"
    repo="{{registry}}/{{runtime}}"
    # :latest (+ :<version>) — the rootfs filesystem image, to build FROM.
    just _push "hluk-{{runtime}}-rootfs" "$repo" "{{version}}"
    # :initrd (+ :initrd-<version>) — the runnable CPIO wrapped in a scratch
    # image.  build-elfloader is in .dockerignore, so wrap from a temp context.
    tmpctx=$(mktemp -d); trap 'rm -rf "$tmpctx"' EXIT
    cp "{{build_dir}}/{{runtime}}-rootfs.cpio" "$tmpctx/initrd.cpio"
    printf 'FROM scratch\nCOPY initrd.cpio /initrd.cpio\n' \
        | docker build -q -f - -t "hluk-{{runtime}}-initrd" "$tmpctx"
    docker tag "hluk-{{runtime}}-initrd:latest" "$repo:initrd"
    docker push "$repo:initrd"
    if [ -n "{{version}}" ]; then
        docker tag "hluk-{{runtime}}-initrd:latest" "$repo:initrd-{{version}}"
        docker push "$repo:initrd-{{version}}"
    fi

# Publish the shared BusyBox base as <registry>/busybox:latest.  busybox has no
# driver and isn't a runnable guest, so it's base-only (no :initrd) — it's the
# userland bash/agent/python-shell build on.  Kept separate from `publish` (which
# is for runtimes), alongside publish-kernel / publish-urunc.
[unix]
publish-busybox registry version="":
    just build-busybox
    just _push hluk-busybox "{{registry}}/busybox" "{{version}}"

# Publish the committed kernel as <registry>/kernel (scratch image at /kernel).
[unix]
publish-kernel registry version="":
    #!/usr/bin/env bash
    set -euo pipefail
    printf 'FROM scratch\nCOPY kernel/elfloader_hyperlight-x86_64 /kernel\n' \
        | docker build -f - -t hluk-kernel "{{root_dir}}/"
    just _push hluk-kernel "{{registry}}/kernel" "{{version}}"

# Publish the urunc "hello" OCI image (see demos/urunc).
[unix]
publish-urunc registry version="":
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{root_dir}}/demos/urunc"
    just stage
    docker build -f Containerfile -t hluk-hello-urunc .
    just _push hluk-hello-urunc "{{registry}}/hello-urunc" "{{version}}"

# Clean rebuild of a rootfs — pulls fresh base images, no Docker cache.
# Also nukes stale snapshots. Use when base images or drivers change.
[unix]
rebuild-rootfs runtime:
    #!/usr/bin/env bash
    set -euo pipefail
    dockerfile="{{drivers_dir}}/{{runtime}}/Dockerfile"
    if [ ! -f "$dockerfile" ]; then
        echo "error: $dockerfile not found" >&2
        exit 1
    fi
    # Pull fresh base images (skip local build stages like "hluk-python-rootfs")
    for img in $(grep '^FROM ' "$dockerfile" | awk '{print $2}' | sort -u); do
        if docker pull "$img" 2>/dev/null; then
            echo "==> Pulled $img"
        fi
    done
    image="hluk-{{runtime}}-rootfs"
    echo "==> Rebuilding $image (--no-cache)"
    docker build --no-cache -t "$image" -f "$dockerfile" "{{root_dir}}/"
    just build-rootfs "{{runtime}}"
    # Rebuild the conformance rootfs too (it inherits the base image)
    conformance_dockerfile="{{conformance_dir}}/{{runtime}}/Dockerfile"
    if [ -f "$conformance_dockerfile" ]; then
        echo "==> Rebuilding conformance image (inherits base)..."
        just build-conformance "{{runtime}}"
    fi
    # Invalidate snapshots built from the old rootfs
    rm -rf "{{snapshot_dir}}/{{runtime}}" "{{snapshot_dir}}/{{runtime}}-conformance"
    echo "==> Stale snapshots removed"

[windows]
rebuild-rootfs runtime:
    @Write-Error "rebuild-rootfs needs Docker + cpio on Linux. Rebuild there (just rebuild-rootfs {{runtime}}) and copy build-elfloader/{{runtime}}-rootfs.cpio here."; exit 1

# List available runtimes
[unix]
list-runtimes:
    @ls -1 "{{drivers_dir}}" | while read f; do \
        [ -d "{{drivers_dir}}/$f" ] && echo "$f"; \
    done

[windows]
list-runtimes:
    @Get-ChildItem -Directory "{{drivers_dir}}" | ForEach-Object { $_.Name }

# ── Run ──────────────────────────────────────────────────────────

# Build + run a script with a given runtime (e.g. just run python examples/python/hello.py)
[unix]
run runtime script *args:
    #!/usr/bin/env bash
    set -euo pipefail
    rootfs="{{build_dir}}/{{runtime}}-rootfs.cpio"
    if [ ! -f "$rootfs" ]; then
        echo "==> rootfs not found, building first..."
        if [ "{{runtime}}" = "agent-custom" ]; then
            just build-rootfs agent-custom examples/agent/custom/Dockerfile
        else
            just build-rootfs "{{runtime}}"
        fi
    fi
    scratch=$(just _scratch-mb "{{runtime}}")
    just build
    "{{root_dir}}/target/release/hluk" run \
        --initrd "$rootfs" \
        --scratch-mb "$scratch" \
        {{script}} {{args}}

[windows]
run runtime script *args:
    #!pwsh
    $ErrorActionPreference = 'Stop'
    $rootfs = "{{build_dir}}/{{runtime}}-rootfs.cpio"
    if (-not (Test-Path $rootfs)) { Write-Error "rootfs not found: $rootfs (build it on Linux and copy it here)" }
    $scratch = just _scratch-mb "{{runtime}}"
    just build
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & "{{root_dir}}/target/release/hluk.exe" run --initrd $rootfs --scratch-mb $scratch {{script}} {{args}}
    exit $LASTEXITCODE

# Run from a pre-built snapshot (e.g. just snapshot-run .snapshots/python hello.py)
[unix]
snapshot-run snapshot script *args: build
    "{{root_dir}}/target/release/hluk" snapshot run {{snapshot}} {{script}} {{args}}

[windows]
snapshot-run snapshot script *args: build
    & "{{root_dir}}/target/release/hluk.exe" snapshot run {{snapshot}} {{script}} {{args}}; exit $LASTEXITCODE

# ── Snapshot ─────────────────────────────────────────────────────

# Save a post-evolve snapshot (e.g. just snapshot-save python)
[unix]
snapshot-save runtime *args:
    #!/usr/bin/env bash
    set -euo pipefail
    rootfs="{{build_dir}}/{{runtime}}-rootfs.cpio"
    if [ ! -f "$rootfs" ]; then
        echo "==> rootfs not found, building first..."
        if [ "{{runtime}}" = "agent-custom" ]; then
            just build-rootfs agent-custom examples/agent/custom/Dockerfile
        else
            just build-rootfs "{{runtime}}"
        fi
    fi
    scratch=$(just _scratch-mb "{{runtime}}")
    just build
    mkdir -p "{{snapshot_dir}}"
    "{{root_dir}}/target/release/hluk" snapshot save \
        --initrd "$rootfs" \
        --scratch-mb "$scratch" \
        --output "{{snapshot_dir}}/{{runtime}}" {{args}}

[windows]
snapshot-save runtime *args:
    #!pwsh
    $ErrorActionPreference = 'Stop'
    $rootfs = "{{build_dir}}/{{runtime}}-rootfs.cpio"
    if (-not (Test-Path $rootfs)) { Write-Error "rootfs not found: $rootfs (build it on Linux and copy it here)" }
    $scratch = just _scratch-mb "{{runtime}}"
    just build
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    New-Item -ItemType Directory -Force "{{snapshot_dir}}" | Out-Null
    & "{{root_dir}}/target/release/hluk.exe" snapshot save --initrd $rootfs --scratch-mb $scratch --output "{{snapshot_dir}}/{{runtime}}" {{args}}
    exit $LASTEXITCODE

# ── Examples ─────────────────────────────────────────────────────

# Run the Python hello world example
example-python: (run "python" (examples_dir / "python" / "hello.py"))

# Run the Node.js hello world example
example-node: (run "node" (examples_dir / "node" / "hello.js"))

# Run the Bash hello world example
example-bash: (run "bash" (examples_dir / "bash" / "hello.sh"))

# Run the .NET JIT hello world example
example-dotnet-jit: (run "dotnet-jit" (examples_dir / "dotnet-jit" / "Hello.cs"))

# Run the PowerShell hello world example
example-powershell: (run "powershell" (examples_dir / "powershell" / "hello.ps1"))

# Compiled runtime examples (C, Rust, Go, dotnet-aot) require compiling
# on the host first.  See examples/<runtime>/README.md for instructions,
# then: just run <runtime> ./hello

# ── Test binaries ────────────────────────────────────────────────

# Build the compiled-language example binaries that tests/compiled.rs
# mounts into the guest, into build-elfloader/bins/<runtime>/.  Linux
# ELF output, so like build-rootfs this runs on Linux; copy
# build-elfloader/bins/ to other hosts.  Needs gcc/g++, rustc, go and
# the .NET SDK.
[unix]
build-test-bins:
    #!/usr/bin/env bash
    set -euo pipefail
    bins="{{build_dir}}/bins"
    rm -rf "$bins"
    mkdir -p "$bins/c" "$bins/rust" "$bins/go" "$bins/dotnet-aot"
    echo "==> C / C++"
    for src in hello goodbye env_vars; do
        gcc -O2 -Wall -static-pie -fPIE -o "$bins/c/$src" "{{examples_dir}}/c/$src.c"
    done
    g++ -O2 -Wall -static-pie -fPIE -o "$bins/c/hello_cpp" "{{examples_dir}}/c/hello_cpp.cpp"
    echo "==> Rust"
    for src in hello env_vars; do
        rustc -C opt-level=2 -C target-feature=+crt-static -C relocation-model=pie \
            -o "$bins/rust/$src" "{{examples_dir}}/rust/$src.rs"
    done
    echo "==> Go"
    for src in hello env_vars; do
        CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -buildmode=pie -ldflags='-s -w' \
            -o "$bins/go/$src" "{{examples_dir}}/go/$src.go"
    done
    echo "==> .NET AOT"
    for proj in dotnet-aot dotnet-aot-envvars dotnet-aot-caps; do
        dotnet publish "{{examples_dir}}/$proj" -c Release -r linux-musl-x64 -v q --nologo \
            -o "$bins/dotnet-aot"
    done
    rm -f "$bins/dotnet-aot"/*.dbg
    echo "==> Done:"
    find "$bins" -type f | sort

[windows]
build-test-bins:
    @Write-Error "build-test-bins needs a Linux toolchain. Build there (just build-test-bins) and copy build-elfloader/bins/ here."; exit 1

# ── Test ─────────────────────────────────────────────────────────

# Run all tests (unit + integration; integration tests need rootfs CPIOs)
[unix]
test *args:
    cargo test --all-targets --manifest-path "{{root_dir}}/Cargo.toml" {{args}}

# On Windows, Hyperlight pre-spawns 512 helper processes per test binary
# unless told otherwise; the suite needs a few at a time, so give it a
# small, lazily-spawned pool sized for the thread count.
[windows]
test *args:
    #!pwsh
    $env:HYPERLIGHT_MAX_SURROGATES = '8'
    $env:HYPERLIGHT_INITIAL_SURROGATES = '0'
    $env:RUST_TEST_THREADS = '4'
    cargo test --all-targets --manifest-path "{{root_dir}}/Cargo.toml" {{args}}
    exit $LASTEXITCODE

# Build the demo/example guest rootfs (the ones not covered by build-all-rootfs).
# Their base images (python/python-shell/node/dotnet-aot) must exist first, so run
# `just build-all-rootfs` before this.  supply-chain reuses the python rootfs.
[unix]
build-demos:
    #!/usr/bin/env bash
    set -euo pipefail
    just build-rootfs autonomous   "{{examples_dir}}/autonomous/Dockerfile"
    just build-rootfs http-flask   "{{examples_dir}}/http-server/flask/Dockerfile"
    just build-rootfs http-express "{{examples_dir}}/http-server/express/Dockerfile"
    just build-rootfs http-kestrel "{{examples_dir}}/http-server/kestrel/Dockerfile"
    just build-rootfs pptx         "{{root_dir}}/demos/pptx-gen/Dockerfile.rootfs"
    just build-rootfs agent-fw-local "{{examples_dir}}/agent-framework/local.Dockerfile"

[windows]
build-demos:
    @Write-Error "build-demos needs Docker on Linux; build there and copy the CPIOs."; exit 1

# Run the demos/examples end to end: build each guest (Linux/Docker), then run
# it under hluk and assert the output.  The run+assert half lives in the
# cross-platform ci/run_demo.py, so the `demos` CI job runs the exact same
# checks on Linux and Windows.  With no argument the full set runs; pass names
# to pick, e.g. `just demos http-flask pptx`.
#   Demos: supply-chain autonomous http-flask http-express http-kestrel pptx
#          agent-fw-local
[unix]
demos *which:
    #!/usr/bin/env bash
    set -euo pipefail
    just build
    which="{{which}}"
    [ -n "$which" ] || which="supply-chain autonomous http-flask http-express http-kestrel pptx agent-fw-local"
    for d in $which; do
        echo "════════════════════════════════════════════"
        echo "  demo: $d"
        echo "════════════════════════════════════════════"
        case "$d" in
        supply-chain)   just build-rootfs python ;;
        autonomous)     just build-rootfs python; just build-rootfs autonomous "{{examples_dir}}/autonomous/Dockerfile" ;;
        http-flask)     just build-rootfs python-shell; just build-rootfs http-flask "{{examples_dir}}/http-server/flask/Dockerfile" ;;
        http-express)   just build-rootfs node; just build-rootfs http-express "{{examples_dir}}/http-server/express/Dockerfile" ;;
        http-kestrel)   just build-rootfs dotnet-aot; just build-rootfs http-kestrel "{{examples_dir}}/http-server/kestrel/Dockerfile" ;;
        pptx)           just build-rootfs python-shell; just build-rootfs pptx "{{root_dir}}/demos/pptx-gen/Dockerfile.rootfs" ;;
        agent-fw-local) just build-rootfs python-shell; just build-rootfs agent-fw-local "{{examples_dir}}/agent-framework/local.Dockerfile" ;;
        *) echo "::error::unknown demo '$d'"; exit 1 ;;
        esac
        python3 "{{root_dir}}/ci/run_demo.py" "$d"
        echo "✓ $d"
    done

# On Windows the guest rootfs are prebuilt (restored from the CI cache, or
# copied over from a Linux build) — no Docker — so we just build hluk and run.
[windows]
demos *which:
    #!pwsh
    $ErrorActionPreference = 'Stop'
    just build
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    $which = "{{which}}"
    if (-not $which) { $which = "supply-chain autonomous http-flask http-express http-kestrel pptx agent-fw-local" }
    $py = if (Get-Command py -ErrorAction SilentlyContinue) { "py" } else { "python" }
    foreach ($d in $which.Split(" ", [StringSplitOptions]::RemoveEmptyEntries)) {
        Write-Host "== demo: $d =="
        & $py "{{root_dir}}/ci/run_demo.py" $d
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    }

# Full local gate — mirrors CI: format, lint, build guests, run the whole
# test suite, then smoke-test the demos.  Heavy (builds every rootfs); use it
# before a release or a big change.  `just verify-kernel` (Docker rebuild of
# the elfloader) and `just conformance python` are run separately.
[unix]
ci:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo fmt --all --check
    cargo clippy --all-targets --locked -- -D warnings
    just check-license-headers
    just build-all-rootfs
    just build-test-bins
    just test --locked
    just demos
    echo "✓ ci: all green"

[windows]
ci:
    @Write-Error "ci needs Docker + KVM on Linux; run it there (just ci)."; exit 1

# Check that every first-party Rust file carries the SPDX license header.
check-license-headers:
    ./dev/check-license-headers.sh

# Run benchmarks for a runtime across all workloads and modes.
#
# Usage:
#   just bench python              # all modes, all workloads
#   just bench python cold-snap    # one mode, all workloads
[unix]
bench runtime *mode:
    #!/usr/bin/env bash
    set -euo pipefail
    hluk="{{root_dir}}/target/release/hluk"
    rootfs="{{build_dir}}/{{runtime}}-rootfs.cpio"
    snap_dir="{{snapshot_dir}}/{{runtime}}"
    bench_dir="{{benchmarks_dir}}/{{runtime}}"

    scratch=$(just _scratch-mb "{{runtime}}")

    if [ ! -f "$rootfs" ]; then
        echo "==> rootfs not found, building first..."
        if [ "{{runtime}}" = "agent-custom" ]; then
            just build-rootfs agent-custom examples/agent/custom/Dockerfile
        else
            just build-rootfs "{{runtime}}"
        fi
    fi

    just build

    # Ensure snapshot exists
    if [ ! -d "$snap_dir" ]; then
        echo "==> Snapshot not found, saving first..."
        mkdir -p "$(dirname "$snap_dir")"
        "$hluk" snapshot save \
            --initrd "$rootfs" \
            --scratch-mb "$scratch" \
            --output "$snap_dir"
    fi

    samples=20
    parallel_vms=4
    parallel_iters=10
    modes="{{mode}}"
    if [ -z "$modes" ]; then
        modes="cold cold-snap warm-restore warm-stateful parallel"
    fi

    # Collect all workload scripts
    workloads=()
    for f in "$bench_dir"/*.py "$bench_dir"/*.js; do
        [ -f "$f" ] && workloads+=("$f")
    done
    if [ ${#workloads[@]} -eq 0 ]; then
        echo "error: no benchmark scripts in $bench_dir" >&2
        exit 1
    fi

    # Capture full output for summary extraction
    outfile=$(mktemp)
    trap 'rm -f "$outfile"' EXIT

    for script in "${workloads[@]}"; do
        wname=$(basename "$script" | sed 's/\.\(py\|js\)$//')
        for m in $modes; do
            echo ""
            echo "════════════════════════════════════════════"
            echo "  {{runtime}} / $m / $wname"
            echo "════════════════════════════════════════════"
            case "$m" in
                cold)
                    "$hluk" bench cold \
                        --initrd "$rootfs" --scratch-mb "$scratch" \
                        --samples "$samples" "$script" \
                        2>&1 | grep '^BENCH' \
                        | sed "s/^BENCH /BENCH [${wname}] /" | tee -a "$outfile"
                    ;;
                cold-snap)
                    "$hluk" bench cold-snap \
                        --samples "$samples" "$snap_dir" "$script" \
                        2>&1 | grep '^BENCH' \
                        | sed "s/^BENCH /BENCH [${wname}] /" | tee -a "$outfile"
                    ;;
                warm-restore)
                    "$hluk" bench warm-restore \
                        --samples "$samples" "$snap_dir" "$script" \
                        2>&1 | grep '^BENCH' \
                        | sed "s/^BENCH /BENCH [${wname}] /" | tee -a "$outfile"
                    ;;
                warm-stateful)
                    "$hluk" bench warm-stateful \
                        --samples "$samples" "$snap_dir" "$script" \
                        2>&1 | grep '^BENCH' \
                        | sed "s/^BENCH /BENCH [${wname}] /" | tee -a "$outfile"
                    ;;
                parallel)
                    "$hluk" bench parallel \
                        --vms "$parallel_vms" --iterations "$parallel_iters" \
                        "$snap_dir" "$script" \
                        2>&1 | grep '^BENCH' \
                        | sed "s/^BENCH /BENCH [${wname}] /" | tee -a "$outfile"
                    ;;
                *)
                    echo "error: unknown mode '$m'" >&2
                    exit 1
                    ;;
            esac
        done
    done

    # ── Compact summary table ──────────────────────────────────
    jsonfile="{{root_dir}}/bench-results.json"
    echo ""
    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║  Benchmark Summary — {{runtime}}                            ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    awk -v jsonfile="$jsonfile" '
    BEGIN { nw = 0 }
    /^BENCH \[/ && /median=/ {
        w = $2; gsub(/[][]/, "", w)
        mode = $3; field = $4
        split($5, kv, "="); val = kv[2]
        if (!(w in ws)) { ws[w] = 1; wo[nw++] = w }
        if (mode == "cold" && field == "total_ms") d["cold:" w] = val
        if (mode == "cold-snap" && field == "total_ms") d["snap:" w] = val
        if (mode == "warm-restore" && field == "exec_ms") d["wrest:" w] = val
        if (mode == "warm-restore" && field == "restore_ms") d["rstr:" w] = val
        if (mode == "warm-stateful" && field == "exec_ms") d["wstat:" w] = val
        if (mode == "parallel" && field == "exec_ms") d["pexec:" w] = val
    }
    /^BENCH \[/ && /throughput=/ {
        w = $2; gsub(/[][]/, "", w)
        if (!(w in ws)) { ws[w] = 1; wo[nw++] = w }
        for (i = 1; i <= NF; i++) {
            if ($i ~ /^throughput=/) {
                split($i, kv, "="); sub(/\/s$/, "", kv[2])
                d["pthr:" w] = kv[2]
            }
        }
    }
    /^BENCH \[/ && /snapshot_mib=/ {
        w = $2; gsub(/[][]/, "", w)
        split($4, kv, "="); v = kv[2]
        if (d["snap_sz:" w] == "") d["snap_sz:" w] = v
    }
    /^BENCH \[/ && /rss_mb=/ {
        w = $2; gsub(/[][]/, "", w)
        split($4, kv, "="); mb = kv[2] + 0
        if (d["rss:" w] == "" || mb > d["rss:" w] + 0) d["rss:" w] = mb
    }
    END {
        if (nw == 0) exit
        printf "\n  %-28s", ""
        for (i = 0; i < nw; i++) printf "%12s", wo[i]
        printf "\n  "
        for (i = 0; i < 28 + nw * 12; i++) printf "-"
        printf "\n"
        split("cold total (ms)|snap total (ms)|warm-restore exec (ms)|restore cost (ms)|warm-stateful exec (ms)|parallel throughput (/s)|parallel exec (ms)|snapshot size (MiB)|RSS (MB)", L, "|")
        split("cold|snap|wrest|rstr|wstat|pthr|pexec|snap_sz|rss", K, "|")
        for (r = 1; r <= 9; r++) {
            has = 0
            for (i = 0; i < nw; i++) if (d[K[r] ":" wo[i]] != "") has = 1
            if (!has) continue
            printf "  %-28s", L[r]
            for (i = 0; i < nw; i++) {
                v = d[K[r] ":" wo[i]]
                printf "%12s", (v != "") ? v : "-"
            }
            printf "\n"
        }
        printf "\n"
        if (jsonfile != "") {
            printf "[\n" > jsonfile
            f = 0
            split("cold|snap|wrest|rstr|wstat|pexec|snap_sz|rss", JK, "|")
            split("cold|cold-snap|warm-restore|restore-cost|warm-stateful|parallel-exec|snapshot-size|rss", JN, "|")
            for (r = 1; r <= 8; r++) {
                for (i = 0; i < nw; i++) {
                    v = d[JK[r] ":" wo[i]]
                    if (v == "") continue
                    if (f) printf ",\n" > jsonfile
                    f = 1
                    if (JK[r] == "rss") u = "MB"
                    else if (JK[r] == "snap_sz") u = "MiB"
                    else u = "ms"
                    printf "  {\"name\": \"%s/%s\", \"unit\": \"%s\", \"value\": %s}", JN[r], wo[i], u, v > jsonfile
                }
            }
            printf "\n]\n" > jsonfile
            close(jsonfile)
        }
    }
    ' "$outfile"
    echo ""
    echo "  JSON: $jsonfile"

[windows]
bench runtime *mode:
    #!pwsh
    $ErrorActionPreference = 'Stop'
    [Console]::OutputEncoding = [Text.Encoding]::UTF8
    $hluk = "{{root_dir}}/target/release/hluk.exe"
    $rootfs = "{{build_dir}}/{{runtime}}-rootfs.cpio"
    $snapDir = "{{snapshot_dir}}/{{runtime}}"
    $benchDir = "{{benchmarks_dir}}/{{runtime}}"

    if (-not (Test-Path $rootfs)) { Write-Error "rootfs not found: $rootfs (build it on Linux and copy it here)" }
    $scratch = just _scratch-mb "{{runtime}}"
    just build
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    if (-not (Test-Path $snapDir)) {
        Write-Output "==> Snapshot not found, saving first..."
        New-Item -ItemType Directory -Force (Split-Path $snapDir) | Out-Null
        & $hluk snapshot save --initrd $rootfs --scratch-mb $scratch --output $snapDir
        if ($LASTEXITCODE -ne 0) { Write-Error "snapshot save failed" }
    }

    $samples = 20
    $parallelVms = 4
    $parallelIters = 10
    $modes = "{{mode}}".Trim()
    if (-not $modes) { $modes = "cold cold-snap warm-restore warm-stateful parallel" }

    $workloads = @(Get-ChildItem $benchDir -File | Where-Object { $_.Extension -in '.py', '.js' } | Sort-Object Name)
    if ($workloads.Count -eq 0) { Write-Error "no benchmark scripts in $benchDir" }

    # Every BENCH line, tagged with its workload, for the summary below.
    $lines = @()
    foreach ($script in $workloads) {
        $w = $script.BaseName
        foreach ($m in ($modes -split '\s+')) {
            Write-Output ""
            Write-Output "════════════════════════════════════════════"
            Write-Output "  {{runtime}} / $m / $w"
            Write-Output "════════════════════════════════════════════"
            $benchArgs = switch ($m) {
                'cold'          { @('cold', '--initrd', $rootfs, '--scratch-mb', $scratch, '--samples', $samples, $script.FullName) }
                'cold-snap'     { @('cold-snap', '--samples', $samples, $snapDir, $script.FullName) }
                'warm-restore'  { @('warm-restore', '--samples', $samples, $snapDir, $script.FullName) }
                'warm-stateful' { @('warm-stateful', '--samples', $samples, $snapDir, $script.FullName) }
                'parallel'      { @('parallel', '--vms', $parallelVms, '--iterations', $parallelIters, $snapDir, $script.FullName) }
                default         { Write-Error "unknown mode '$m'" }
            }
            $out = @(& $hluk bench @benchArgs 2>&1 | ForEach-Object { "$_" } | Where-Object { $_ -match '^BENCH ' } | ForEach-Object { $_ -replace '^BENCH ', "BENCH [$w] " })
            if ($LASTEXITCODE -ne 0) { Write-Error "hluk bench $m failed (exit $LASTEXITCODE)" }
            $out | Write-Output
            $lines += $out
        }
    }

    # ── Compact summary table ──────────────────────────────────
    # Same rows, layout and JSON as the bash recipe above.
    $d = @{}
    $wo = @()
    foreach ($l in $lines) {
        $f = $l -split ' '
        $w = $f[1].Trim('[', ']')
        if ($wo -notcontains $w) { $wo += $w }
        if ($l -match 'median=') {
            $key = switch ("$($f[2]) $($f[3])") {
                'cold total_ms'           { 'cold' }
                'cold-snap total_ms'      { 'snap' }
                'warm-restore exec_ms'    { 'wrest' }
                'warm-restore restore_ms' { 'rstr' }
                'warm-stateful exec_ms'   { 'wstat' }
                'parallel exec_ms'        { 'pexec' }
                default                   { $null }
            }
            if ($key) { $d["${key}:$w"] = ($f[4] -split '=')[1] }
        } elseif ($l -match 'throughput=([0-9.]+)') {
            $d["pthr:$w"] = $Matches[1]
        } elseif ($l -match 'snapshot_mib=([0-9.]+)') {
            if (-not $d.ContainsKey("snap_sz:$w")) { $d["snap_sz:$w"] = $Matches[1] }
        } elseif ($l -match 'rss_mb=([0-9.]+)') {
            if (-not $d.ContainsKey("rss:$w") -or [double]$Matches[1] -gt [double]$d["rss:$w"]) { $d["rss:$w"] = $Matches[1] }
        }
    }

    $jsonFile = "{{root_dir}}/bench-results.json"
    Write-Output ""
    Write-Output ""
    Write-Output "╔══════════════════════════════════════════════════════════════╗"
    Write-Output "║  Benchmark Summary — {{runtime}}                            ║"
    Write-Output "╚══════════════════════════════════════════════════════════════╝"
    if ($wo.Count -gt 0) {
        $rows = @(
            @('cold total (ms)', 'cold'), @('snap total (ms)', 'snap'),
            @('warm-restore exec (ms)', 'wrest'), @('restore cost (ms)', 'rstr'),
            @('warm-stateful exec (ms)', 'wstat'), @('parallel throughput (/s)', 'pthr'),
            @('parallel exec (ms)', 'pexec'), @('snapshot size (MiB)', 'snap_sz'), @('RSS (MB)', 'rss'))
        Write-Output ""
        Write-Output ("  " + ("{0,-28}" -f '') + (($wo | ForEach-Object { "{0,12}" -f $_ }) -join ''))
        Write-Output ("  " + ('-' * (28 + 12 * $wo.Count)))
        foreach ($r in $rows) {
            $vals = @($wo | ForEach-Object { if ($d.ContainsKey("$($r[1]):$_")) { $d["$($r[1]):$_"] } else { '-' } })
            if (-not ($vals | Where-Object { $_ -ne '-' })) { continue }
            Write-Output ("  " + ("{0,-28}" -f $r[0]) + (($vals | ForEach-Object { "{0,12}" -f $_ }) -join ''))
        }
        Write-Output ""

        $units = @{ rss = 'MB'; snap_sz = 'MiB' }
        $jrows = @(
            @('cold', 'cold'), @('snap', 'cold-snap'), @('wrest', 'warm-restore'), @('rstr', 'restore-cost'),
            @('wstat', 'warm-stateful'), @('pexec', 'parallel-exec'), @('snap_sz', 'snapshot-size'), @('rss', 'rss'))
        $json = @()
        foreach ($r in $jrows) {
            foreach ($w in $wo) {
                if (-not $d.ContainsKey("$($r[0]):$w")) { continue }
                $u = if ($units.ContainsKey($r[0])) { $units[$r[0]] } else { 'ms' }
                $json += "  {`"name`": `"$($r[1])/$w`", `"unit`": `"$u`", `"value`": $($d["$($r[0]):$w"])}"
            }
        }
        [IO.File]::WriteAllText($jsonFile, "[`n" + ($json -join ",`n") + "`n]`n")
    }
    Write-Output ""
    Write-Output "  JSON: $jsonFile"

# ── Conformance ─────────────────────────────────────────────────

# Build a conformance rootfs (includes upstream test suite).
# Depends on the base driver image — builds it first if missing.
[unix]
build-conformance runtime:
    #!/usr/bin/env bash
    set -euo pipefail
    dockerfile="{{conformance_dir}}/{{runtime}}/Dockerfile"
    if [ ! -f "$dockerfile" ]; then
        echo "error: $dockerfile not found" >&2
        exit 1
    fi
    # Ensure the base driver image exists
    base_image="hluk-{{runtime}}-rootfs"
    if ! docker image inspect "$base_image" >/dev/null 2>&1; then
        echo "==> Base image $base_image not found, building first..."
        just build-rootfs "{{runtime}}"
    fi
    image="hluk-{{runtime}}-conformance"
    output="{{build_dir}}/{{runtime}}-conformance.cpio"
    mkdir -p "{{build_dir}}"
    echo "==> Building conformance image $image"
    docker build -t "$image" -f "$dockerfile" "{{conformance_dir}}/{{runtime}}/"
    echo "==> Exporting to $output (newc CPIO)"
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT
    cid=$(docker create --entrypoint=/ "$image" 2>/dev/null || docker create "$image")
    docker export "$cid" | tar -C "$tmpdir" -xf -
    docker rm "$cid" > /dev/null
    # docker export replaces /etc/hosts, /etc/resolv.conf with empty
    # virtual mounts — restore minimal versions.
    # 'unikraft' = Unikraft's default hostname (gethostname()).
    printf '127.0.0.1 localhost unikraft\n::1 localhost unikraft\n' > "$tmpdir/etc/hosts"
    # nsswitch.conf: files first, then DNS for external resolution
    printf 'hosts: files dns\n' > "$tmpdir/etc/nsswitch.conf"
    # resolv.conf: public DNS + single-request (serializes A/AAAA queries;
    # glibc's parallel A+AAAA mode doesn't work correctly through hostsock)
    printf 'nameserver 8.8.8.8\nnameserver 1.1.1.1\noptions single-request\n' > "$tmpdir/etc/resolv.conf"
    (cd "$tmpdir" && find . | cpio -o -H newc --quiet > "$output")
    echo "==> Done: $output ($(du -h "$output" | cut -f1))"

[windows]
build-conformance runtime:
    @Write-Error "build-conformance needs Docker + cpio on Linux. Build there (just build-conformance {{runtime}}) and copy build-elfloader/{{runtime}}-conformance.cpio here."; exit 1

# Clean rebuild of the conformance rootfs — rebuilds both the base
# driver image and the conformance image, and invalidates stale snapshots.
[unix]
rebuild-conformance runtime:
    #!/usr/bin/env bash
    set -euo pipefail
    just rebuild-rootfs "{{runtime}}"
    just build-conformance "{{runtime}}"
    rm -rf "{{snapshot_dir}}/{{runtime}}-conformance"
    echo "==> Stale conformance snapshot removed"

[windows]
rebuild-conformance runtime:
    @Write-Error "rebuild-conformance needs Docker + cpio on Linux. Rebuild there (just rebuild-conformance {{runtime}}) and copy build-elfloader/{{runtime}}-conformance.cpio here."; exit 1

# Run upstream conformance tests for a runtime.
# Each test module runs in its own guest (snapshot restore) so a crash
# in one doesn't kill the suite and memory resets between tests.
#
# Modules listed in conformance/<runtime>/known_failures.toml are
# skipped — any module NOT in that file is expected to pass.
[unix]
conformance runtime *modules:
    #!/usr/bin/env bash
    set -uo pipefail
    hluk="{{root_dir}}/target/release/hluk"
    rootfs="{{build_dir}}/{{runtime}}-conformance.cpio"
    snap_dir="{{snapshot_dir}}/{{runtime}}-conformance"
    manifest="{{conformance_dir}}/{{runtime}}/known_failures.toml"

    if [ ! -f "$rootfs" ]; then
        echo "==> Conformance rootfs not found, building first..."
        just build-conformance "{{runtime}}"
    fi

    scratch=$(just _scratch-mb "{{runtime}}")

    just build

    # Save a snapshot if one doesn't exist
    if [ ! -d "$snap_dir" ]; then
        echo "==> Saving conformance snapshot..."
        mkdir -p "$(dirname "$snap_dir")"
        "$hluk" snapshot save \
            --initrd "$rootfs" \
            --scratch-mb "$scratch" \
            --net \
            --output "$snap_dir"
    fi

    # Determine which modules to test.  If args given, use those;
    # otherwise discover test_*.py modules from the rootfs via guest.
    if [ -n "{{modules}}" ]; then
        test_modules=({{modules}})
    else
        echo "==> Discovering test modules..."
        # One module per line to avoid output truncation
        mapfile -t test_modules < <("$hluk" snapshot run "$snap_dir" --net --exec \
            "import os; d='/usr/local/lib/python3.12/test'; [print(f[:-3]) for f in sorted(os.listdir(d)) if f.startswith('test_') and f.endswith('.py')]" \
            2>/dev/null | tr -d '\r' | grep "^test_" || true)
    fi

    if [ "${#test_modules[@]}" -eq 0 ]; then
        echo "error: no test modules discovered" >&2
        exit 1
    fi

    # Build skip list from the known-failures manifest
    declare -A SKIP=()
    if [ -f "$manifest" ]; then
        while IFS= read -r mod; do
            SKIP["$mod"]=1
        done < <(grep -oP '^\s+"(test_[^"]+)"' "$manifest" | sed 's/.*"\(test_[^"]*\)".*/\1/')
    fi

    pass=0 fail=0 error=0 skip=0 crash=0 total=0

    echo "==> Running ${#test_modules[@]} modules (${#SKIP[@]} in skip list)"
    echo ""

    for mod in "${test_modules[@]}"; do
        if [[ -n "${SKIP[$mod]+x}" ]]; then
            echo "SKIP $mod"
            ((skip++)) || true
            ((total++)) || true
            continue
        fi

        # Run this module in its own guest with a timeout.
        # --kill-after=5 ensures a SIGKILL follows if the process
        # ignores SIGTERM (some modules spin the hypervisor at 100% CPU).
        inline=$(printf "MODULE='%s'\n%s" "$mod" "$(cat '{{conformance_dir}}/{{runtime}}/run_tests.py')")

        output=$(timeout --kill-after=5 60 "$hluk" snapshot run "$snap_dir" \
            --net --exec "$inline" 2>/dev/null || echo "RESULT $mod status=CRASH tests=0 fail=0 error=0 skip=0 time=0")

        # Extract the RESULT line
        result_line=$(echo "$output" | tr -d '\r' | grep "^RESULT " | tail -1)

        if [ -z "$result_line" ]; then
            result_line="RESULT $mod status=CRASH tests=0 fail=0 error=0 skip=0 time=0"
        fi

        echo "$result_line"

        status=$(echo "$result_line" | grep -oP 'status=\K\w+')
        case "$status" in
            PASS)  ((pass++))  || true ;;
            FAIL)  ((fail++))  || true ;;
            ERROR) ((error++)) || true ;;
            CRASH) ((crash++)) || true ;;
        esac
        ((total++)) || true
    done

    echo ""
    echo "════════════════════════════════════════════"
    echo "SUMMARY total=$total pass=$pass fail=$fail error=$error skip=$skip crash=$crash"
    echo "════════════════════════════════════════════"

    if [ "$fail" -gt 0 ] || [ "$error" -gt 0 ] || [ "$crash" -gt 0 ]; then
        echo ""
        echo "✗ Conformance suite failed (fail=$fail error=$error crash=$crash)" >&2
        exit 1
    fi

# Windows port of the recipe above — same discovery, skip list, per-module
# timeout, RESULT/SUMMARY lines and exit code.  Keep the two in step.
[windows]
conformance runtime *modules:
    #!pwsh
    $ErrorActionPreference = 'Stop'
    [Console]::OutputEncoding = [Text.Encoding]::UTF8
    $hluk = "{{root_dir}}/target/release/hluk.exe"
    $rootfs = "{{build_dir}}/{{runtime}}-conformance.cpio"
    $snapDir = "{{snapshot_dir}}/{{runtime}}-conformance"
    $manifest = "{{conformance_dir}}/{{runtime}}/known_failures.toml"
    $runner = Get-Content -Raw "{{conformance_dir}}/{{runtime}}/run_tests.py"

    if (-not (Test-Path $rootfs)) { Write-Error "conformance rootfs not found: $rootfs (build it on Linux and copy it here)" }
    $scratch = just _scratch-mb "{{runtime}}"
    just build
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    if (-not (Test-Path $snapDir)) {
        Write-Output "==> Saving conformance snapshot..."
        New-Item -ItemType Directory -Force (Split-Path $snapDir) | Out-Null
        & $hluk snapshot save --initrd $rootfs --scratch-mb $scratch --net --output $snapDir
        if ($LASTEXITCODE -ne 0) { Write-Error "snapshot save failed" }
    }

    # Modules from the command line, else discovered from the rootfs via the guest.
    $modulesArg = "{{modules}}".Trim()
    if ($modulesArg) {
        $testModules = @($modulesArg -split '\s+')
    } else {
        Write-Output "==> Discovering test modules..."
        $discover = "import os; d='/usr/local/lib/python3.12/test'; [print(f[:-3]) for f in sorted(os.listdir(d)) if f.startswith('test_') and f.endswith('.py')]"
        $testModules = @(& $hluk snapshot run $snapDir --net --exec $discover 2>$null | ForEach-Object { $_.Trim() } | Where-Object { $_ -like 'test_*' })
        if ($LASTEXITCODE -ne 0) { Write-Error "module discovery failed (exit $LASTEXITCODE)" }
    }
    if ($testModules.Count -eq 0) { Write-Error "no test modules discovered" }

    # Skip list from the known-failures manifest.
    $skip = @{}
    if (Test-Path $manifest) {
        foreach ($m in [regex]::Matches((Get-Content -Raw $manifest), '(?m)^\s+"(test_[^"]+)"')) { $skip[$m.Groups[1].Value] = $true }
    }

    $pass = 0; $fail = 0; $err = 0; $skipped = 0; $crash = 0; $total = 0
    Write-Output "==> Running $($testModules.Count) modules ($($skip.Count) in skip list)"
    Write-Output ""

    foreach ($mod in $testModules) {
        if ($skip.ContainsKey($mod)) {
            Write-Output "SKIP $mod"; $skipped++; $total++; continue
        }
        # Each module runs in its own guest, killed after 60 s (some spin the hypervisor).
        $line = "RESULT $mod status=CRASH tests=0 fail=0 error=0 skip=0 time=0"
        $psi = [System.Diagnostics.ProcessStartInfo]::new($hluk)
        foreach ($a in @('snapshot', 'run', $snapDir, '--net', '--exec', "MODULE='$mod'`n$runner")) { $psi.ArgumentList.Add($a) }
        $psi.UseShellExecute = $false
        $psi.RedirectStandardOutput = $true
        $psi.RedirectStandardError = $true
        $p = [System.Diagnostics.Process]::Start($psi)
        $stdout = $p.StandardOutput.ReadToEndAsync()
        $null = $p.StandardError.ReadToEndAsync()
        if ($p.WaitForExit(60000)) {
            $p.WaitForExit()  # flush async readers
            if ($p.ExitCode -eq 0) {
                $found = @($stdout.Result -split "`n" | ForEach-Object { $_.Trim() } | Where-Object { $_ -like 'RESULT *' })
                if ($found.Count -gt 0) { $line = $found[-1] }
            }
        } else {
            try { $p.Kill($true) } catch {}
            $p.WaitForExit()
        }
        Write-Output $line
        switch -Regex ($line) {
            'status=PASS'  { $pass++ }
            'status=FAIL'  { $fail++ }
            'status=ERROR' { $err++ }
            'status=CRASH' { $crash++ }
        }
        $total++
    }

    Write-Output ""
    Write-Output "════════════════════════════════════════════"
    Write-Output "SUMMARY total=$total pass=$pass fail=$fail error=$err skip=$skipped crash=$crash"
    Write-Output "════════════════════════════════════════════"
    if ($fail -gt 0 -or $err -gt 0 -or $crash -gt 0) {
        Write-Output ""
        [Console]::Error.WriteLine("✗ Conformance suite failed (fail=$fail error=$err crash=$crash)")
        exit 1
    }

# ── Clean ────────────────────────────────────────────────────────

# Remove build artifacts
[unix]
clean:
    rm -rf "{{build_dir}}" "{{root_dir}}/target"

[windows]
clean:
    if (Test-Path "{{build_dir}}") { Remove-Item -Recurse -Force "{{build_dir}}" }
    if (Test-Path "{{root_dir}}/target") { Remove-Item -Recurse -Force "{{root_dir}}/target" }

# Remove only rootfs build artifacts (keep Rust build cache)
[unix]
clean-rootfs:
    rm -rf "{{build_dir}}"

[windows]
clean-rootfs:
    if (Test-Path "{{build_dir}}") { Remove-Item -Recurse -Force "{{build_dir}}" }

# Remove all snapshots
[unix]
clean-snapshots:
    rm -rf "{{snapshot_dir}}"
    @echo "==> All snapshots removed"

[windows]
clean-snapshots:
    if (Test-Path "{{snapshot_dir}}") { Remove-Item -Recurse -Force "{{snapshot_dir}}" }

# Build all rootfs images (uses Docker cache — fast if nothing changed).
# For a clean rebuild of everything, use `just rebuild-all-rootfs`.
[unix]
build-all-rootfs:
    #!/usr/bin/env bash
    set -euo pipefail
    for d in "{{drivers_dir}}"/*/; do
        runtime=$(basename "$d")
        if [ -f "$d/Dockerfile" ]; then
            echo ""
            echo "════════════════════════════════════════════"
            echo "  Building rootfs: $runtime"
            echo "════════════════════════════════════════════"
            just build-rootfs "$runtime"
        fi
    done
    # agent-custom lives outside drivers/
    if [ -f "{{root_dir}}/examples/agent/custom/Dockerfile" ]; then
        echo ""
        echo "════════════════════════════════════════════"
        echo "  Building rootfs: agent-custom"
        echo "════════════════════════════════════════════"
        just build-rootfs agent-custom examples/agent/custom/Dockerfile
    fi

[windows]
build-all-rootfs:
    @Write-Error "build-all-rootfs needs Docker + cpio on Linux. Build there (just build-all-rootfs) and copy build-elfloader/ here."; exit 1

# Rebuild all rootfs images from scratch (--no-cache, pulls fresh
# base images, invalidates snapshots).
[unix]
rebuild-all-rootfs:
    #!/usr/bin/env bash
    set -euo pipefail
    for d in "{{drivers_dir}}"/*/; do
        runtime=$(basename "$d")
        if [ -f "$d/Dockerfile" ]; then
            echo ""
            echo "════════════════════════════════════════════"
            echo "  Rebuilding rootfs: $runtime"
            echo "════════════════════════════════════════════"
            just rebuild-rootfs "$runtime"
        fi
    done
    # agent-custom lives outside drivers/ — rebuild manually
    if [ -f "{{root_dir}}/examples/agent/custom/Dockerfile" ]; then
        echo ""
        echo "════════════════════════════════════════════"
        echo "  Rebuilding rootfs: agent-custom"
        echo "════════════════════════════════════════════"
        df="{{root_dir}}/examples/agent/custom/Dockerfile"
        for img in $(grep '^FROM ' "$df" | awk '{print $2}' | sort -u); do
            if docker pull "$img" 2>/dev/null; then
                echo "==> Pulled $img"
            fi
        done
        docker build --no-cache -t hluk-agent-custom-rootfs -f "$df" "{{root_dir}}/"
        just build-rootfs agent-custom "$df"
        rm -rf "{{snapshot_dir}}/agent-custom"
    fi

[windows]
rebuild-all-rootfs:
    @Write-Error "rebuild-all-rootfs needs Docker + cpio on Linux. Rebuild there (just rebuild-all-rootfs) and copy build-elfloader/ here."; exit 1
