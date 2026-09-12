"""HTTP server+client — exercises threading+socket pattern from CPython tests.

Spawns a simple HTTP server in a background thread, then makes a
request from the main thread.
"""

import http.server
import threading
import urllib.request

PORT = 9997
RESPONSE = b"Hello from the HTTP handler!"


class Handler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(RESPONSE)))
        self.end_headers()
        self.wfile.write(RESPONSE)

    def log_message(self, format, *args):
        pass  # suppress output


server = http.server.HTTPServer(("127.0.0.1", PORT), Handler)
t = threading.Thread(target=server.handle_request, daemon=True)
t.start()

import time
time.sleep(0.1)

url = f"http://127.0.0.1:{PORT}/"
with urllib.request.urlopen(url, timeout=5) as resp:
    body = resp.read()
    assert body == RESPONSE, f"got {body!r}"
    print(f"HTTP response: {body.decode()}")

server.server_close()
print("HTTP server+client test passed.")
