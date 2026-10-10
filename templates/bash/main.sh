# BusyBox runs in NOMMU mode here (the kernel has vfork, not fork): every
# applet is its own process, started with vfork + execve, so pipes and
# $(...) substitutions work, and shell builtins (echo, printf, read, test,
# arithmetic) save a process wherever they do the job.
echo "Hello from {{name}}!"
printf 'Running on '
uname -srm
echo "inside a Hyperlight micro-VM"
