#!/usr/bin/env bash
# Measure how much host memory N concurrent Python sandboxes take, for the
# site's density section.
#
#   site/tools/density.sh [VMS] [ITERATIONS]      # default: 1000 20
#
# Saves a python snapshot with the hluk on PATH, runs `hluk bench parallel`
# (VMS threads, each restoring the snapshot and running a script ITERATIONS
# times) and samples the process's private memory (RssAnon) until it exits.
set -euo pipefail

vms=${1:-1000}
iterations=${2:-20}
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cd "$work"

echo 'print("ok")' >hello.py
hluk snapshot save --runtime python --output snap >/dev/null
hluk bench parallel --vms "$vms" --iterations "$iterations" snap hello.py >bench.log 2>&1 &
pid=$!
peak=0
while kill -0 "$pid" 2>/dev/null; do
  kb=$(awk '/^RssAnon:/ { print $2 }' "/proc/$pid/status" 2>/dev/null || echo 0)
  [ "${kb:-0}" -gt "$peak" ] && peak=$kb
  sleep 0.05
done
wait "$pid" || { cat bench.log >&2; exit 1; }

grep 'parallel summary' bench.log
echo "snapshot: $(du -sh snap | cut -f1)"
awk -v kb="$peak" -v n="$vms" 'BEGIN {
  printf "peak private memory: %.2f GiB for %d sandboxes, %.2f MiB each\n", kb / 1048576, n, kb / 1024 / n
}'
