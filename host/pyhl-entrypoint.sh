#!/bin/sh
set -e

if [ ! -f "$PYHL_HOME/snapshot/index.json" ]; then
    staging=$(mktemp -d)
    trap 'rm -rf "$staging"' EXIT
    mkdir -p "$staging/.unikraft/build"
    ln -s "$PYHL_HOME/kernel" "$staging/.unikraft/build/driver_hyperlight-x86_64"
    ln -s "$PYHL_HOME/initrd.cpio" "$staging/driver-initrd.cpio"
    pyhl setup --dest "$PYHL_HOME" --from "$staging" --net --force
fi

if [ "$1" = "--setup-only" ]; then
    exit 0
fi

exec pyhl run --dest "$PYHL_HOME" --net "$@"
