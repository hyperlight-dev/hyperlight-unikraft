"""Threaded select() — server blocks in select() with infinite timeout.

Validates that the cooperative scheduler's idle thread polls hostsock
even when no thread has a timed wakeup (wake_up_time == 0).  Without
the halt_irq fix, this hangs forever because the idle thread never
calls hostsock_rescan_events().
"""
import socket
import threading
import select

result = []

def server(srv):
    readable, _, _ = select.select([srv], [], [])
    if readable:
        conn, _ = srv.accept()
        data = conn.recv(64)
        result.append(data.decode())
        conn.sendall(b"pong")
        conn.close()

srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
srv.bind(("127.0.0.1", 9123))
srv.listen(1)

t = threading.Thread(target=server, args=(srv,))
t.start()

import time
time.sleep(0.05)

cli = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
cli.connect(("127.0.0.1", 9123))
cli.sendall(b"ping")
resp = cli.recv(64)
cli.close()
srv.close()

t.join(timeout=3)
assert not t.is_alive(), "server thread still alive — select() never woke"
assert result == ["ping"], f"expected ['ping'], got {result}"
assert resp == b"pong", f"expected b'pong', got {resp}"
print("Threaded select() test passed")
