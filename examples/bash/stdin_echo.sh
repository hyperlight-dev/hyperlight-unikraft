#!/bin/bash
# Read from stdin and echo each line.
lines=0
while IFS= read -r line; do
    lines=$((lines + 1))
    echo "echo: $line"
done
echo "lines=$lines"
echo "stdin-done"
