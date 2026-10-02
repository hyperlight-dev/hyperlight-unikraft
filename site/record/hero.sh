# A Python project, cold and then warm: the first run boots the guest and
# saves a snapshot of it; the second restores the snapshot.
. "$(dirname "${BASH_SOURCE[0]}")/common.sh"

pe "hluk init hello --template python"
beat
pe "cd hello"
pe "cat main.py"
beat 1.5
p "# first run: boot the micro-VM, run main.py, save a snapshot"
pe "time hluk run"
beat 2
p "# every run after restores that snapshot"
pe "time hluk run"
beat 3
