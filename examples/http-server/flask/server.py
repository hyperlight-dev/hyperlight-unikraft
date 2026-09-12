"""Flask HTTP server running inside a Hyperlight micro-VM.

The guest binds 0.0.0.0:8080; because hluk backs guest sockets with host
sockets, the listener is reachable from the host at http://127.0.0.1:8080.
Run with `--net --port 8080` so the socket layer is registered and the guest
is allowed to bind that port.
"""
from flask import Flask, jsonify

app = Flask(__name__)


@app.get("/")
def index():
    return "Hello from Flask on Hyperlight!\n"


@app.get("/health")
def health():
    return jsonify(status="ok", server="flask", isolation="hyperlight-micro-vm")


if __name__ == "__main__":
    print("Listening on :8080", flush=True)
    # threaded=False keeps it single-request; the demo is illustrative.
    app.run(host="0.0.0.0", port=8080, threaded=True)
