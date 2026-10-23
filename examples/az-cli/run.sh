#!/usr/bin/env bash
# Host side: boot the az-cli rootfs as a shell.  Egress is limited to the
# hosts the CLI's sign-in and ARM calls use; the CLI's state (tokens,
# config) lives in $AZ_HOME on the host, so a sign-in outlives the guest.
#
#   examples/az-cli/run.sh                 interactive
#   examples/az-cli/run.sh 'az group list' one command
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
hluk=${HLUK:-$root/target/release/hluk}
az_home=${AZ_HOME:-$HOME/.hluk-az}
# The token cache lands here, as plain JSON: keep it private.
mkdir -p -m 700 "$az_home"
args=(
    --initrd "$root/build-elfloader/az-cli-rootfs.cpio"
    --entry /usr/local/bin/hl_bashdriver
    --scratch-mb 2048 --warm
    --mount "$az_home:/root/.azure"
    --net-allow login.microsoftonline.com
    --net-allow management.azure.com
    --net-allow graph.microsoft.com
    --env HOME=/root
    # az starts a telemetry uploader after each command; in a long-lived
    # guest they pile up and each command runs slower than the last.
    --env AZURE_CORE_COLLECT_TELEMETRY=false
)
if [ $# -gt 0 ]; then
    exec "$hluk" run "${args[@]}" --exec "$*"
fi
exec "$hluk" run "${args[@]}" "$here/shell.sh"
