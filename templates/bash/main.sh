# BusyBox runs in NOMMU mode here (the kernel has vfork, not fork): every
# applet is its own process, and a pipe or a $(...) substitution between
# processes is not supported yet -- so one command per line, and shell
# builtins (echo, printf, read, test, arithmetic) wherever possible.
echo "Hello from {{name}}!"
printf 'Running on '
uname -srm
echo "inside a Hyperlight micro-VM"
