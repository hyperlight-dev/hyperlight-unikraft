#!/bin/sh
# Install hluk from a GitHub release, no Rust toolchain needed:
#
#   curl -fsSL https://raw.githubusercontent.com/hyperlight-dev/hyperlight-unikraft/main/install.sh | sh
#
#   HLUK_VERSION=v0.14.1   a release instead of the latest
#   HLUK_INSTALL_DIR=DIR   where the binary goes (default ~/.local/bin)
#
# Releases carry x86_64 Linux and Windows binaries (docs/RELEASE.md). On
# Windows take the .zip from the release page; elsewhere, `cargo install
# hyperlight-unikraft` builds from source.
set -eu

REPO=hyperlight-dev/hyperlight-unikraft
VERSION=${HLUK_VERSION:-}
DIR=${HLUK_INSTALL_DIR:-$HOME/.local/bin}

case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
    *)
        echo "no prebuilt hluk for $(uname -s)/$(uname -m): use the Windows .zip from" >&2
        echo "https://github.com/$REPO/releases, or \`cargo install hyperlight-unikraft\`" >&2
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
# Verify against the release's checksum file when it has one.
if curl -fsSL "https://github.com/$REPO/releases/download/$VERSION/SHA256SUMS" -o "$tmp/SHA256SUMS" 2>/dev/null \
    && grep -q " $asset\$" "$tmp/SHA256SUMS"; then
    (cd "$tmp" && grep " $asset\$" SHA256SUMS | sha256sum -c - >/dev/null) || {
        echo "checksum mismatch for $asset" >&2; exit 1; }
fi
tar -xzf "$tmp/$asset" -C "$tmp"

mkdir -p "$DIR"
install -m 755 "$tmp/hluk" "$DIR/hluk"
echo "installed $("$DIR/hluk" --version) to $DIR/hluk"
case ":$PATH:" in
    *":$DIR:"*) ;;
    *) echo "add it to PATH: export PATH=\"$DIR:\$PATH\"" ;;
esac
if [ -e /dev/kvm ] && [ ! -w /dev/kvm ]; then
    echo "note: /dev/kvm is not writable by you; on most distributions: sudo usermod -aG kvm \$USER, then log in again"
fi
echo "next: hluk init"
