# Shared setup for the scenes: demo-magic, typed at a readable speed, with
# no keypress waits so a scene records unattended.
. "$(dirname "${BASH_SOURCE[0]}")/demo-magic.sh" -n

TYPE_SPEED=28
DEMO_PROMPT="${GREEN}\W${COLOR_RESET} \$ "
DEMO_COMMENT_COLOR=$GREY
TIMEFORMAT=$'\e[0;33mtook %Rs\e[0m'

# A beat between commands so the output can be read before the next prompt.
beat() { sleep "${1:-1.2}"; }

# demo-magic's run_cmd returns 0 whatever the command did. This one notes a
# failure in $SCENE_FAILED, so record.sh keeps the committed cast instead of
# replacing it with a recording of the error.
run_cmd() {
  eval "$@" || echo "$*: exit $?" >>"${SCENE_FAILED:?}"
}
