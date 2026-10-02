# Extend a published rootfs with a Dockerfile, build it, and serve from the
# guest.
. "$(dirname "${BASH_SOURCE[0]}")/common.sh"

pe "hluk init api --template http-python"
beat
pe "cd api"
pe "cat requirements.txt Dockerfile"
beat 3
p "# hluk build runs Docker and exports the image as the guest rootfs"
pe "hluk build"
beat
# The server is started outside demo-magic's eval, which loses a background
# job, and its output is shown once it answers.
p "hluk run &"
hluk run >server.log 2>&1 &
server=$!
for _ in $(seq 150); do curl -sf -o /dev/null localhost:8080/health && break; sleep 0.2; done
# Leave out Flask's dev-server warning, the host's private address and the
# readiness probe's request.
grep -v -E 'WARNING|Running on http://(10|172|192)\.|GET /health' server.log
beat 1.5
pe "curl -s localhost:8080/health"
beat 2.5
kill "$server"
# demo-magic defines a wait() that reads a keypress; this is the builtin.
builtin wait 2>/dev/null || true
