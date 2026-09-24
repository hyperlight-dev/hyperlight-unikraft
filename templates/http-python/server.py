"""A Flask server inside a Hyperlight micro-VM.

The guest binds 0.0.0.0:8080; hluk backs guest sockets with host sockets,
so the listener is reachable from the host at http://127.0.0.1:8080. The
manifest's [run.net] enables networking and allows the port.
"""
from flask import Flask, jsonify

app = Flask(__name__)


@app.get("/")
def index():
    return "Hello from {{name}} on Flask, inside a Hyperlight micro-VM!\n"


@app.get("/health")
def health():
    return jsonify(status="ok", app="{{name}}", isolation="hyperlight-micro-vm")


if __name__ == "__main__":
    print("Listening on http://127.0.0.1:8080", flush=True)
    app.run(host="0.0.0.0", port=8080, threaded=True)
