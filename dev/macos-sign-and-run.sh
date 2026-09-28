#!/usr/bin/env bash
# Cargo runner on macOS: Hypervisor.framework only serves a process whose
# binary carries the com.apple.security.hypervisor entitlement, so sign
# (ad hoc) each test or binary before running it, and the hluk binary a
# test runs (target/<profile>/hluk, next to target/<profile>/deps/).
set -Eeuo pipefail

entitlements="$(dirname "$0")/macos-entitlements.plist"
codesign -f -s - --entitlements "$entitlements" "$1"
hluk="$(dirname "$1")/../hluk"
if [ "$(basename "$(dirname "$1")")" = deps ] && [ -f "$hluk" ]; then
    codesign -f -s - --entitlements "$entitlements" "$hluk"
fi
# Hypervisor.framework runs one sandbox of a process at a time, so tests on
# parallel threads gain nothing and one test's guest timers run out while
# another's sandbox holds the hypervisor.  Override with RUST_TEST_THREADS.
export RUST_TEST_THREADS="${RUST_TEST_THREADS:-1}"
exec "$@"
