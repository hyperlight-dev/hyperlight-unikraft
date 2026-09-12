#!/bin/bash
# Interactive shell — reads commands from stdin and executes them.
# Use with: hluk run --initrd ... examples/bash/shell.sh
# Then type commands interactively, one per line.
while true; do
    printf '# '
    IFS= read -r cmd || break
    eval "$cmd"
done
