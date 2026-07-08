#!/bin/bash
set -e

if [ ! -d "$PYHL_HOME/snapshot" ]; then
    staging=$(mktemp -d)
    mkdir -p "$staging/.unikraft/build"
    ln -s "$PYHL_HOME/kernel" "$staging/.unikraft/build/driver_hyperlight-x86_64"
    ln -s "$PYHL_HOME/initrd.cpio" "$staging/driver-initrd.cpio"
    pyhl setup --dest "$PYHL_HOME" --from "$staging" --net --force
    rm -rf "$staging"
fi

exec pyhl run --dest "$PYHL_HOME" --net "$@"
