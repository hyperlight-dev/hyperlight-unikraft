#!/bin/bash
# Run one example per language to verify the build+run pipeline.
# Languages: C, Rust, Go, .NET, PowerShell, Shell, Python
#
# Usage: ./scripts/test-examples.sh

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
EXAMPLES_DIR="$REPO_ROOT/examples"

FAILURES=0

now_ms() {
  local ns
  ns=$(date +%s%N 2>/dev/null)
  if [ "${#ns}" -gt 10 ]; then
    echo $(( ns / 1000000 ))
  else
    python3 -c 'import time; print(int(time.time()*1000))'
  fi
}

run_example() {
  local name="$1"
  local dir="$EXAMPLES_DIR/$name"

  echo "--- $name ---"

  if [ ! -d "$dir" ]; then
    echo "  SKIP: directory not found"
    return 1
  fi

  cd "$dir"

  # Build
  printf "  build: "
  local t0 t1 rc
  t0=$(now_ms)
  just build < /dev/null > /dev/null 2>&1
  rc=$?
  t1=$(now_ms)
  if [ $rc -eq 0 ]; then
    echo "ok $((t1 - t0))ms"
  else
    echo "FAILED (exit=$rc, $((t1 - t0))ms)"
    FAILURES=$((FAILURES + 1))
    cd "$REPO_ROOT"
    return 1
  fi

  # Rootfs
  printf "  rootfs: "
  t0=$(now_ms)
  just rootfs < /dev/null > /dev/null 2>&1
  rc=$?
  t1=$(now_ms)
  if [ $rc -eq 0 ]; then
    echo "ok $((t1 - t0))ms"
  else
    echo "FAILED (exit=$rc, $((t1 - t0))ms)"
    FAILURES=$((FAILURES + 1))
    cd "$REPO_ROOT"
    return 1
  fi

  # Run
  printf "  run: "
  t0=$(now_ms)
  just run < /dev/null > /dev/null 2>&1
  rc=$?
  t1=$(now_ms)
  if [ $rc -eq 0 ]; then
    echo "ok $((t1 - t0))ms"
  else
    echo "FAILED (exit=$rc, $((t1 - t0))ms)"
    FAILURES=$((FAILURES + 1))
  fi

  cd "$REPO_ROOT"
}

echo "=== test-examples — $(date) ==="
echo ""

run_example "helloworld-c"
run_example "rust"
run_example "go"
run_example "dotnet"
run_example "powershell"
run_example "shell"
run_example "python"

echo ""
if [ "$FAILURES" -gt 0 ]; then
  echo "DONE with $FAILURES failure(s)"
else
  echo "ALL PASSED"
fi

exit "$FAILURES"
