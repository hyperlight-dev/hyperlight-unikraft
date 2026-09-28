"""TCP echo — runs a server and client inside the guest.

A background thread serves TCP echo on a free port of 127.0.0.1.
The main thread connects, sends a message, reads the echo, and
verifies the content.

This exercises intra-guest networking: the hostsock driver's
check_ready pattern returns EAGAIN on blocking calls (accept, recv)
when the socket isn't ready, letting Unikraft's cooperative scheduler
yield to the other thread.
"""

import socket
import threading

PAYLOAD = b"Hello from guest!"

# Bound and listening before the thread starts, on a port the host picks,
# so the client never races the server or meets another program's port.
srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
srv.bind(("127.0.0.1", 0))
srv.listen(1)
PORT = srv.getsockname()[1]


def echo_server():
    with srv:
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
