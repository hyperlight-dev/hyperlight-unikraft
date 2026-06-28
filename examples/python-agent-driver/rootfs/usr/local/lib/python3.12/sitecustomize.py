import socket as _socket
import struct as _struct
import sys as _sys

_orig_getaddrinfo = _socket.getaddrinfo

def _tcp_dns_resolve(host, port):
    query = _struct.pack('>HHHHHH', 0xABCD, 0x0100, 1, 0, 0, 0)
    for label in host.encode().split(b'.'):
        query += bytes([len(label)]) + label
    query += b'\x00\x00\x01\x00\x01'

    results = []
    s = _socket.socket(_socket.AF_INET, _socket.SOCK_STREAM)
    try:
        s.connect(("8.8.8.8", 53))
        s.send(_struct.pack('>H', len(query)) + query)
        resp = b''
        while len(resp) < 2:
            resp += s.recv(512)
        rlen = _struct.unpack('>H', resp[:2])[0]
        while len(resp) < rlen + 2:
            resp += s.recv(512)
        data = resp[2:]
        if len(data) > 12:
            ancount = _struct.unpack('>H', data[6:8])[0]
            pos = 12
            for _ in range(_struct.unpack('>H', data[4:6])[0]):
                while pos < len(data) and data[pos]:
                    pos += data[pos] + 1
                pos += 5
            for _ in range(ancount):
                if pos + 2 > len(data):
                    break
                if data[pos] & 0xC0 == 0xC0:
                    pos += 2
                else:
                    while pos < len(data) and data[pos]:
                        pos += data[pos] + 1
                    pos += 1
                if pos + 10 > len(data):
                    break
                rtype, rclass, ttl, rdlen = _struct.unpack('>HHIH', data[pos:pos+10])
                pos += 10
                if rtype == 1 and rdlen == 4 and pos + 4 <= len(data):
                    ip = '.'.join(str(b) for b in data[pos:pos+4])
                    results.append((_socket.AF_INET, _socket.SOCK_STREAM, 6, '',
                                    (ip, port if port else 0)))
                    results.append((_socket.AF_INET, _socket.SOCK_DGRAM, 17, '',
                                    (ip, port if port else 0)))
                pos += rdlen
    except Exception:
        pass
    finally:
        s.close()
    return results


def _patched_getaddrinfo(host, port, family=0, type=0, proto=0, flags=0):
    if isinstance(host, str) and not host.replace('.', '').isdigit():
        r = _tcp_dns_resolve(host, port if isinstance(port, int) else 0)
        if r:
            if type:
                r = [x for x in r if x[1] == type] or r
            return r
    return _orig_getaddrinfo(host, port, family, type, proto, flags)

_socket.getaddrinfo = _patched_getaddrinfo

import os as _os
_orig_scandir = _os.scandir
def _patched_scandir(path='.'):
    if path == '':
        path = '.'
    return _orig_scandir(path)
_os.scandir = _patched_scandir

if '/tmp/pip_packages' not in _sys.path:
    _sys.path.insert(0, '/tmp/pip_packages')
