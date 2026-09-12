# HTTP server in the guest

Three tiny HTTP servers that run **inside** a Hyperlight micro-VM and are reachable from the host on `http://127.0.0.1:8080`:

| Server | Runtime | Rootfs |
|--------|---------|--------|
| Flask   | Python 3.12 | extends `python-shell` with `flask` |
| Express | Node 21     | extends the `node` rootfs with `express` |
| Kestrel | .NET 9 (ASP.NET Core) | self-contained musl publish on the `dotnet-aot` driver |

## How it works

`hluk` backs the guest's POSIX sockets with host sockets (one syscall per host call). When the guest binds `0.0.0.0:8080` and accepts, the accept happens on the host socket, so a `curl http://127.0.0.1:8080` on the host reaches the server running in the VM. Two flags are needed:

- `--port 8080` — allows the guest to bind that port (the listen-port allowlist).
- `--net` — registers the socket layer (`AllowAll`, which also permits loopback so host↔guest works). Without a network policy the socket host functions are not registered.

The server runs until you stop it (Ctrl-C), like any server.

## Flask (Python)

```bash
just build-rootfs http-flask examples/http-server/flask/Dockerfile

hluk run --initrd build-elfloader/http-flask-rootfs.cpio --scratch-mb 256 \
    --net --port 8080 examples/http-server/flask/server.py
# in another terminal:
curl http://127.0.0.1:8080/         # Hello from Flask on Hyperlight!
curl http://127.0.0.1:8080/health   # {"status":"ok","server":"flask",...}
```

## Express (Node)

```bash
just build-rootfs http-express examples/http-server/express/Dockerfile

hluk run --initrd build-elfloader/http-express-rootfs.cpio --scratch-mb 512 \
    --net --port 8080 examples/http-server/express/server.js
curl http://127.0.0.1:8080/         # Hello from Express on Hyperlight!
```

## Kestrel (.NET)

Kestrel is a compiled binary, so it's dispatched by guest path with `--exec` (there is no host-side script to read):

```bash
just build-rootfs http-kestrel examples/http-server/kestrel/Dockerfile

hluk run --initrd build-elfloader/http-kestrel-rootfs.cpio --scratch-mb 256 \
    --net --port 8080 --exec /app/KestrelHyperlight
curl http://127.0.0.1:8080/         # Hello from Kestrel on Hyperlight!
```
