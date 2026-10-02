# The same `hluk run` across runtimes. The projects were created and warmed
# by record.sh before recording, so each run restores a snapshot.
. "$(dirname "${BASH_SOURCE[0]}")/common.sh"

pe "ls"
beat
for lang in python node dotnet go rust quickjs; do
  pe "time hluk run -f hello-$lang"
  beat 0.8
done
beat 2
