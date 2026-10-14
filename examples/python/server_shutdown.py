"""Server shutdown demo: select() on host sockets says what is there.

A listener is readable only with a connection pending, and a connection only
with data unread: select() saying otherwise sends the caller into an
accept() or recv() that blocks for good.  socketserver's shutdown() is the
usual victim: its loop never gets back to see the request.
"""
import http.server
import select
import socket
import threading
import urllib.request

# 1. A listener, idle, then with a connection pending, then idle again.
srv = socket.socket()
srv.bind(("127.0.0.1", 0))
srv.listen()
assert select.select([srv], [], [], 0.2)[0] == [], "idle listener readable"
cli = socket.create_connection(srv.getsockname())
assert select.select([srv], [], [], 5)[0] == [srv]
conn, _ = srv.accept()
assert select.select([srv], [], [], 0.2)[0] == [], "listener readable after accept"
print("1. listener readiness")

# 2. A connection, readable while data is unread and not after.
cli.sendall(b"ping")
assert select.select([conn], [], [], 5)[0] == [conn]
assert conn.recv(4) == b"ping"
assert select.select([conn], [], [], 0.2)[0] == [], "connection readable after recv"
for s in (cli, conn, srv):
    s.close()
print("2. connection readiness")


# 3. A server thread answers, then shutdown() returns.
class Handler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-Length", "2")
        self.end_headers()
        self.wfile.write(b"ok")

    def log_message(self, *args):
        pass


server = http.server.HTTPServer(("127.0.0.1", 0), Handler)
thread = threading.Thread(target=server.serve_forever, kwargs={"poll_interval": 0.1})
thread.start()
url = "http://127.0.0.1:%d/" % server.server_address[1]
assert urllib.request.urlopen(url).read() == b"ok"
server.shutdown()
thread.join()
server.server_close()
print("3. serve_forever and shutdown")

# 4. A blocking connect() to a port nobody listens on is refused.
probe = socket.socket()
probe.bind(("127.0.0.1", 0))
port = probe.getsockname()[1]
probe.close()
try:
    socket.create_connection(("127.0.0.1", port), timeout=5)
    raise AssertionError("connected to nothing")
except ConnectionRefusedError:
    pass
print("4. connection refused")

print("Server shutdown demo passed")
