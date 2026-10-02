# A guest function call: the script defines a handler, the host calls it
# with JSON in and gets JSON out.
. "$(dirname "${BASH_SOURCE[0]}")/common.sh"

pe "cat handler.py"
beat 2
pe "hluk run --runtime python handler.py --call handler --input '{\"name\": \"World\"}'"
beat 3
