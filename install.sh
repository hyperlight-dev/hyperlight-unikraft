#!/bin/sh
# Install hluk from a GitHub release, no Rust toolchain needed:
#
#   curl -fsSL https://raw.githubusercontent.com/hyperlight-dev/hyperlight-unikraft/main/install.sh | sh
#
#   HLUK_VERSION=v0.14.1   a release instead of the latest
#   HLUK_INSTALL_DIR=DIR   where the binary goes (default ~/.local/bin)
#
# Releases carry binaries for Linux (x86_64, arm64), macOS (Apple silicon)
# and Windows (docs/RELEASE.md). On Windows use install.ps1 instead:
#
#   irm https://raw.githubusercontent.com/hyperlight-dev/hyperlight-unikraft/main/install.ps1 | iex
#
# Elsewhere, `cargo install hyperlight-unikraft` builds from source.
set -eu

REPO=hyperlight-dev/hyperlight-unikraft
VERSION=${HLUK_VERSION:-}
DIR=${HLUK_INSTALL_DIR:-$HOME/.local/bin}

case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
    Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-gnu ;;
    Darwin-arm64) target=aarch64-apple-darwin ;;
    MINGW* | MSYS* | CYGWIN*)
        echo "on Windows, install with PowerShell:" >&2
        echo "  irm https://raw.githubusercontent.com/$REPO/main/install.ps1 | iex" >&2
        exit 1
        ;;
    *)
        echo "no prebuilt hluk for $(uname -s)/$(uname -m):" >&2
        echo "\`cargo install hyperlight-unikraft\` builds it from source" >&2
        exit 1
        ;;
esac

if [ -z "$VERSION" ]; then
    VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -1)
    [ -n "$VERSION" ] || { echo "cannot find the latest release of $REPO" >&2; exit 1; }
fi
case "$VERSION" in v*) ;; *) VERSION=v$VERSION ;; esac

asset="hluk-$VERSION-$target.tar.gz"
url="https://github.com/$REPO/releases/download/$VERSION/$asset"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

echo "downloading $url"
curl -fsSL "$url" -o "$tmp/$asset"
# Verify against the release's checksum file when it has one.  Only its
# absence (404) skips the check; any other failure stops the install.
code=$(curl -sSL -w '%{http_code}' -o "$tmp/SHA256SUMS" \
    "https://github.com/$REPO/releases/download/$VERSION/SHA256SUMS") || {
    echo "cannot fetch SHA256SUMS" >&2; exit 1; }
case $code in
    200) ;;
    404) rm -f "$tmp/SHA256SUMS" ;;
    *) echo "cannot fetch SHA256SUMS (HTTP $code)" >&2; exit 1 ;;
esac
if [ -f "$tmp/SHA256SUMS" ]; then
    grep -q " $asset\$" "$tmp/SHA256SUMS" || { echo "$asset is not in SHA256SUMS" >&2; exit 1; }
    # macOS has shasum rather than sha256sum.
    if command -v sha256sum >/dev/null; then sum="sha256sum"; else sum="shasum -a 256"; fi
    (cd "$tmp" && grep " $asset\$" SHA256SUMS | $sum -c - >/dev/null) || {
        echo "checksum mismatch for $asset" >&2; exit 1; }
fi
tar -xzf "$tmp/$asset" -C "$tmp"

mkdir -p "$DIR"
install -m 755 "$tmp/hluk" "$DIR/hluk"
if [ "$(uname -s)" = Darwin ]; then
    # Hypervisor.framework serves only a binary signed with this
    # entitlement; sign it here, where it is installed.
    cat > "$tmp/entitlements.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict><key>com.apple.security.hypervisor</key><true/></dict></plist>
PLIST
    codesign -f -s - --entitlements "$tmp/entitlements.plist" "$DIR/hluk" 2>/dev/null || {
        echo "error: codesign failed; unsigned, hluk cannot use Hypervisor.framework" >&2
        exit 1
    }
fi
echo "installed $("$DIR/hluk" --version) to $DIR/hluk"
case ":$PATH:" in
    *":$DIR:"*) ;;
    *) echo "add it to PATH: export PATH=\"$DIR:\$PATH\"" ;;
esac
if [ -e /dev/kvm ] && [ ! -w /dev/kvm ]; then
    echo "note: /dev/kvm is not writable by you; on most distributions: sudo usermod -aG kvm \$USER, then log in again"
fi
echo "next: hluk init"
