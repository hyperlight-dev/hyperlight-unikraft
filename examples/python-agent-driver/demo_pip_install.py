import socket, struct, ssl, sys, os, zipfile, io, re, urllib.request


def _tcp_dns_resolve(hostname, dns_server="8.8.8.8"):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.settimeout(10)
    s.connect((dns_server, 53))
    q = struct.pack(">HHHHHH", 0xABCD, 0x0100, 1, 0, 0, 0)
    for label in hostname.encode().split(b"."):
        q += bytes([len(label)]) + label
    q += b"\x00\x00\x01\x00\x01"
    s.send(struct.pack(">H", len(q)) + q)
    r = b""
    while len(r) < 2:
        r += s.recv(512)
    rlen = struct.unpack(">H", r[:2])[0]
    while len(r) < rlen + 2:
        r += s.recv(512)
    s.close()
    d = r[2:]
    ancount = struct.unpack(">H", d[6:8])[0]
    pos = 12
    for _ in range(struct.unpack(">H", d[4:6])[0]):
        while pos < len(d) and d[pos]:
            pos += d[pos] + 1
        pos += 5
    ips = []
    for _ in range(ancount):
        if d[pos] & 0xC0 == 0xC0:
            pos += 2
        else:
            while pos < len(d) and d[pos]:
                pos += d[pos] + 1
            pos += 1
        rtype, _, _, rdlen = struct.unpack(">HHIH", d[pos : pos + 10])
        pos += 10
        if rtype == 1 and rdlen == 4:
            ips.append(".".join(str(b) for b in d[pos : pos + 4]))
        pos += rdlen
    return ips


_orig_getaddrinfo = socket.getaddrinfo


def _patched_getaddrinfo(host, port, family=0, type=0, proto=0, flags=0):
    if host in ("localhost", "127.0.0.1", "::1"):
        return _orig_getaddrinfo(host, port, family, type, proto, flags)
    try:
        ips = _tcp_dns_resolve(host)
        if ips:
            return [
                (socket.AF_INET, socket.SOCK_STREAM, 6, "", (ip, port or 0))
                for ip in ips
            ]
    except Exception:
        pass
    return _orig_getaddrinfo(host, port, family, type, proto, flags)


socket.getaddrinfo = _patched_getaddrinfo


def pip_install(package, target="/tmp/pip_packages"):
    resp = urllib.request.urlopen(
        f"https://pypi.org/simple/{package}/", timeout=30
    )
    html = resp.read().decode()

    links = re.findall(r'href="([^"]*\.whl[^"]*)"', html)
    py_wheels = [l for l in links if "py3-none-any" in l or "py2.py3-none-any" in l]
    if not py_wheels:
        raise RuntimeError(f"No compatible wheel found for {package}")
    wheel_url = py_wheels[-1].split("#")[0]

    resp = urllib.request.urlopen(wheel_url, timeout=30)
    wheel_data = resp.read()

    os.makedirs(target, exist_ok=True)
    zf = zipfile.ZipFile(io.BytesIO(wheel_data))
    zf.extractall(target)

    if target not in sys.path:
        sys.path.insert(0, target)

    return len(zf.namelist())


n = pip_install("six")
print(f"Installed six ({n} files extracted)")

import six

print(f"Imported six {six.__version__}")
