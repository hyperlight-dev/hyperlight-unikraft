"""TCP echo — runs a server and client inside the guest.

A background thread starts a TCP echo server on 127.0.0.1:9999.
The main thread connects, sends a message, reads the echo, and
verifies the content.

This exercises intra-guest networking: the hostsock driver's
check_ready pattern returns EAGAIN on blocking calls (accept, recv)
when the socket isn't ready, letting Unikraft's cooperative scheduler
yield to the other thread.
"""

import socket
import threading

PORT = 9999
PAYLOAD = b"Hello from guest!"


def echo_server():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as srv:
        srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        srv.bind(("127.0.0.1", PORT))
        srv.listen(1)
        conn, _addr = srv.accept()
        with conn:
            data = b""
            while True:
                chunk = conn.recv(4096)
                if not chunk:
                    break
                data += chunk
            conn.sendall(data)


t = threading.Thread(target=echo_server, daemon=True)
t.start()

# Give the server thread a chance to call listen() before we connect.
import time
time.sleep(0.1)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect(("127.0.0.1", PORT))
    s.sendall(PAYLOAD)
    s.shutdown(socket.SHUT_WR)
    reply = b""
    while True:
        chunk = s.recv(4096)
        if not chunk:
            break
        reply += chunk

assert reply == PAYLOAD, f"echo mismatch: sent {PAYLOAD!r}, got {reply!r}"
print(f"Echo reply: {reply.decode()}")
print("TCP echo test passed.")
