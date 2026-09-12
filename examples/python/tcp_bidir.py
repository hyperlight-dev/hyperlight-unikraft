"""Bidirectional TCP — stress test for send() POLLOUT guard.

Server and client exchange large payloads simultaneously.
"""

import socket
import threading
import time

PORT = 9998
SIZE = 128 * 1024  # 128 KB — larger than typical socket buffer

SERVER_DATA = b"S" * SIZE
CLIENT_DATA = b"C" * SIZE


def server():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as srv:
        srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        srv.bind(("127.0.0.1", PORT))
        srv.listen(1)
        conn, _ = srv.accept()
        with conn:
            # Read everything from client
            received = b""
            while len(received) < SIZE:
                chunk = conn.recv(4096)
                if not chunk:
                    break
                received += chunk

            # Send response
            conn.sendall(SERVER_DATA)

            assert received == CLIENT_DATA, (
                f"server got {len(received)} bytes, expected {SIZE}"
            )


t = threading.Thread(target=server, daemon=True)
t.start()
time.sleep(0.1)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(("127.0.0.1", PORT))
    s.sendall(CLIENT_DATA)
    s.shutdown(socket.SHUT_WR)

    received = b""
    while True:
        chunk = s.recv(4096)
        if not chunk:
            break
        received += chunk

assert received == SERVER_DATA, (
    f"client got {len(received)} bytes, expected {SIZE}"
)
print(f"Exchanged {SIZE} bytes in each direction.")
print("Bidirectional TCP test passed.")
