"""Network policy probe — tests TCP connect and UDP sendto enforcement.

Usage from the guest:
  exec("HOST = '10.0.0.5'; PORT = 19999")  # set target
  exec(open('/path/to/net_policy_probe.py').read())

Tests both code paths where the host enforces network policy:
  - reg_connect (TCP connect)
  - reg_sendto  (UDP sendto)

Output lines:
  TCP_OK        — TCP connect succeeded
  TCP_BLOCKED   — TCP connect denied by policy (EACCES)
  TCP_REFUSED   — TCP connect got ECONNREFUSED (policy allowed it,
                  but nothing was listening — this still proves the
                  policy check passed)
  TCP_FAIL:<e>  — TCP connect failed for another reason

  UDP_OK        — UDP sendto succeeded
  UDP_BLOCKED   — UDP sendto denied by policy (EACCES)
  UDP_FAIL:<e>  — UDP sendto failed for another reason
"""
import socket
import errno

# ── TCP connect (exercises reg_connect) ──
try:
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(5)
    s.connect((HOST, PORT))
    s.close()
    print("TCP_OK")
except OSError as e:
    if e.errno == errno.EACCES:
        print("TCP_BLOCKED")
    elif e.errno == errno.ECONNREFUSED:
        print("TCP_REFUSED")
    else:
        print(f"TCP_FAIL:{e.errno}")

# ── UDP sendto (exercises reg_sendto) ──
try:
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.sendto(b"probe", (HOST, PORT))
    s.close()
    print("UDP_OK")
except OSError as e:
    if e.errno == errno.EACCES:
        print("UDP_BLOCKED")
    else:
        print(f"UDP_FAIL:{e.errno}")
