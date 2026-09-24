# The whole guest session: read a line, run it, print a prompt, until stdin
# closes. `eval` runs each line in this shell, so `cd` and variables persist
# from one command to the next.
echo "{{name}}: a shell inside a Hyperlight micro-VM. Ctrl-D to leave."
while true; do
    printf '%s' "${PS1:-# }"
    if ! IFS= read -r cmd; then
        # End of input.  A last line with no newline has still filled cmd:
        # run it, and stop here rather than read past the end.
        [ -n "$cmd" ] && eval "$cmd"
        break
    fi
    eval "$cmd"
done
echo
