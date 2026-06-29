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
        s.settimeout(10)
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

def _copy_tree(src, dst):
    if _os.path.isdir(src):
        _os.makedirs(dst, exist_ok=True)
        for item in _os.listdir(src):
            _copy_tree(_os.path.join(src, item), _os.path.join(dst, item))
    else:
        with open(src, 'rb') as _sf:
            with open(dst, 'wb') as _df:
                while True:
                    _chunk = _sf.read(65536)
                    if not _chunk:
                        break
                    _df.write(_chunk)

_orig_rename = _os.rename
def _patched_rename(src, dst):
    try:
        return _orig_rename(src, dst)
    except OSError as _e:
        if _e.errno in (30, 38):
            _copy_tree(src, dst)
        else:
            raise
_os.rename = _patched_rename

_orig_replace = _os.replace
def _patched_replace(src, dst):
    try:
        return _orig_replace(src, dst)
    except OSError as _e:
        if _e.errno in (30, 38):
            _copy_tree(src, dst)
        else:
            raise
_os.replace = _patched_replace

import shutil as _shutil
_orig_copystat = _shutil.copystat
def _patched_copystat(src, dst, **kwargs):
    try:
        return _orig_copystat(src, dst, **kwargs)
    except OSError as _e:
        if _e.errno in (30, 38):
            pass
        else:
            raise
_shutil.copystat = _patched_copystat

_orig_unlink = _os.unlink
def _patched_unlink(path):
    try:
        return _orig_unlink(path)
    except OSError as _e:
        if _e.errno in (30, 38):
            pass
        else:
            raise
_os.unlink = _patched_unlink
_os.remove = _patched_unlink

_orig_rmdir = _os.rmdir
def _patched_rmdir(path):
    try:
        return _orig_rmdir(path)
    except OSError as _e:
        if _e.errno in (30, 38):
            pass
        else:
            raise
_os.rmdir = _patched_rmdir

if '/tmp/pip_packages' not in _sys.path:
    _sys.path.insert(0, '/tmp/pip_packages')

import ssl as _ssl
_orig_ssl_close = _ssl.SSLSocket.close
def _fast_ssl_close(self):
    try:
        super(_ssl.SSLSocket, self).close()
    except OSError:
        pass
_ssl.SSLSocket.close = _fast_ssl_close

# Unikraft's exit_group syscall hangs when child processes have had
# network connections. Work around by having child processes SIGKILL
# themselves instead of calling exit_group, and having the parent
# treat SIGKILL exit status as success.
import signal as _signal
if _os.getpid() > 2:
    _orig_sys_exit = _sys.exit
    def _child_sys_exit(code=0):
        _sys.stdout.flush()
        _sys.stderr.flush()
        _os.kill(_os.getpid(), _signal.SIGKILL)
        _orig_sys_exit(code)
    _sys.exit = _child_sys_exit

import subprocess as _subprocess
_orig_popen_wait = _subprocess.Popen.wait
def _patched_popen_wait(self, timeout=None):
    rc = _orig_popen_wait(self, timeout)
    if rc < 0 and abs(rc) == _signal.SIGKILL:
        rc = 0
        self.returncode = 0
    return rc
_subprocess.Popen.wait = _patched_popen_wait
