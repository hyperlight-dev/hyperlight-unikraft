#!/bin/sh
# Guest side: a line-at-a-time shell over the host's stdin.
# Your terminal already echoes what you type; on a terminal the guest
# would echo it a second time.
stty -echo 2>/dev/null
export PATH=/usr/local/bin:/bin
cd /root
while true; do
    printf 'az-shell:%s$ ' "$PWD"
    IFS= read -r cmd || break
    eval "$cmd"
done
