# Networking

Unikraft guests on Hyperlight can access host networking through a **hostsock** driver that forwards POSIX socket calls to the host via hypercalls.  The host manages real sockets; the guest only holds a virtual file descriptor.

## Architecture

```
Guest (Unikraft)              Host (Rust)
─────────────────             ──────────────
Python/Node app               hostnet.rs
      │                            │
  libc socket()               SocketTable
      │                       (fd → host socket, kind, peer)
  hostsock.c                       │
  (POSIX socket driver)       rustix::net / rustix::event
      │                       (real OS socket, one call per syscall)
  hl_hcall_int / vecbytes ──→ net_* host functions
```

### How it works

1. The guest calls `socket()`, `connect()`, `send()`, etc. through the standard POSIX socket API.
2. Unikraft's socket layer dispatches to **hostsock** — a kernel driver registered for `AF_INET` and `AF_INET6`.
3. hostsock serialises the call into `hl_param` structs and makes a synchronous host call (`hl_hcall_int` for integer results, `hl_hcall_vecbytes` for variable-length data).
4. The host's **hostnet** module looks up the virtual fd in a `SocketTable`, checks policy, and makes the matching [`rustix`](https://docs.rs/rustix) call — `rustix::net::connect`, `rustix::net::sendto`, `rustix::event::poll`, … — on the real OS socket.  rustix is a safe wrapper over the POSIX/WinSock socket API, so the host side is the same code on Linux and Windows.

### Host portability

The host side is the same code on Linux and Windows.  The guest is a Linux-ABI unikernel, so the host always speaks Linux numbers to it: errno values, `POLL*` bits and `SOL_*`/`SO_*` option numbers are translated at the boundary (`src/errno.rs`, `src/hostnet.rs`), and socket options are served from an allow-list of the int-valued options runtimes use (unknown ones return `-ENOPROTOOPT`).

### Blocking model and intra-guest networking

A host function call is a synchronous VM exit — the guest vCPU is fully paused until the call returns.  If a host call (e.g. `accept()`, `recv()`, a large `send()`) were allowed to block on a peer that lives in the same guest, the entire VM would freeze and that peer could never run.

Two things prevent this.  hostsock uses a **check-ready pattern**: it polls with `net_poll(timeout=0)` first and returns `EAGAIN` instead of calling the host when the socket isn't ready.  And host sockets are **non-blocking**, so a call that would block anyway (a send larger than the free buffer space) also comes back as `-EAGAIN`.  Either way, Unikraft's POSIX socket layer then calls `uk_file_poll()`, which blocks *the current thread* (not the vCPU) and yields to the cooperative scheduler, so other guest threads can run.  `connect()` starts the handshake and returns `EINPROGRESS`; the socket layer parks the thread on writability and reads the outcome through `SO_ERROR`, the same way as on Linux.

When all threads are blocked, the kernel yields the vCPU to the host, which parks in `poll(2)` on the real sockets the guest is waiting on (readability, and writability where a send was refused) and re-enters when one is ready; on every entry the kernel rescans them and wakes the waiting threads.  See [execution.md](execution.md).

This enables intra-guest networking — for example, a server and client can run in two threads inside the same guest (see [`examples/python/tcp_echo.py`](../examples/python/tcp_echo.py)).

## Enabling networking

Pass a `NetworkPolicy` when creating a sandbox:

```rust
use hyperlight_unikraft::{NetworkPolicy, AllowList, BlockList, ListenPorts, SandboxBuilder};

// Full network access (all outbound destinations permitted):
let mut guest = SandboxBuilder::from_initrd(rootfs)
    .scratch_mb(256)
    .network(NetworkPolicy::AllowAll)
    .boot()?;
```

When `None` (the default), no `net_*` host functions are registered and guest socket calls fail.

## Host functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `net_socket` | `(family, type, proto) → fd` | Create a socket (IPv4/IPv6, TCP/UDP only) |
| `net_bind` | `(fd, family, addr, port) → 0` | Bind to an address |
| `net_listen` | `(fd, backlog) → 0` | Start listening |
| `net_accept` | `(fd) → [new_fd, addr]` | Accept a connection |
| `net_connect` | `(fd, family, addr, port) → 0` | Connect to a remote address |
| `net_disconnect` | `(fd) → 0` | Dissolve a datagram socket's association (`connect` with `AF_UNSPEC`) |
| `net_send` | `(fd, data) → bytes_sent` | Send data |
| `net_sendto` | `(fd, data, family, addr, port) → bytes_sent` | Send to a specific address |
| `net_recvfrom` | `(fd, len) → [bytes, addr, data]` | Receive data |
| `net_shutdown` | `(fd, how) → 0` | Shut down part of a connection |
| `net_close` | `(fd) → 0` | Close a socket |
| `net_getpeername` | `(fd) → addr` | Get peer address |
| `net_getsockname` | `(fd) → addr` | Get local address |
| `net_getsockopt` | `(fd, level, optname) → value` | Get socket option |
| `net_setsockopt` | `(fd, level, optname, value) → 0` | Set socket option |
| `net_poll` | `(pollfds, timeout_ms) → [count, revents]` | Poll for I/O events |

All functions return negative `-errno` values on error.

## Limits

- **Max sockets:** 1024 open sockets per sandbox (host-enforced).
- **Transfer size:** 64 KiB per `recvfrom` / `write` / `sendmsg` call, the most one host call carries; a longer write returns a short count and the caller's loop continues. The guest takes the figure from the PEB I/O stack sizes the host chose.
- **Protocols:** `AF_INET`/`AF_INET6` with `SOCK_STREAM`/`SOCK_DGRAM` reach the host. No raw sockets. `AF_UNIX` exists only inside the guest (`socketpair`, which asyncio's event loop needs), served by the kernel itself.

## Network policy

`NetworkPolicy` controls which outbound destinations a guest can reach.  It is a destination filter on the host's own sockets: no network namespace, no inbound filtering, no rate limit; isolation beyond that is the embedder's to provide.

| Variant | Behaviour |
|---------|-----------|
| `AllowAll` | Everything, except the metadata addresses. |
| `AllowList(AllowList)` | Only the listed names and addresses. |
| `BlockList(BlockList)` | Everything except the listed names and addresses. |

Under every variant the cloud metadata addresses are refused: the link-local ranges (`169.254.0.0/16`, `fe80::/10`) and AWS's IPv6 endpoint `fd00:ec2::254`.  The two lists also refuse loopback, since a guest socket is a host socket and host services trust `127.0.0.1`; `AllowAll` permits it, so a server and a client can meet inside one guest.  An IPv4 address written as IPv4-mapped IPv6 (`::ffff:a.b.c.d`) is judged as `a.b.c.d`.

### Where it is checked

The guest resolves names itself, so a destination reaches the host as an address; the name is seen only in the DNS question.  Hence:

| Host call | Check |
|-----------|-------|
| `net_connect(fd, address)` | The destination, by address.  Every connect, TCP or UDP. |
| `net_sendto(fd, data, address)` | The destination; and, if the address is port 53, the DNS question in `data`, by name. |
| `net_send(fd, data)` | The DNS question in `data`, by name, on a socket the guest connected to port 53.  Other sends are not checked: their destination was, at the connect. |
| `net_recvfrom(fd)` | Nothing is refused; under an allow list, an answer from a resolver the guest may ask has its addresses recorded. |
| `net_bind(fd, address)` | The inbound rule, [`ListenPorts`](#listenports). |

Nothing else is inspected.

**The question.**  Under an allow list the guest may ask only about listed names; under a block list, not about a blocked one; anything to port 53 that is not a well-formed query is refused.  An allow list lets questions reach the well-known resolvers (`8.8.8.8`, `8.8.4.4`, `1.1.1.1`, `1.0.0.1`) and the host's own from `/etc/resolv.conf`, on UDP only, and records only their answers.

**The destination.**  An allow list allows an address that was listed, resolved when the list was built, or recorded from an answer, and nothing else: the host resolves nothing on the guest's behalf, so an address the guest was never given is refused.  A block list refuses an address that was listed or resolved at build time, and whatever a blocked name resolves to now, looked up at the connect with a 250 ms deadline; a lookup that fails or runs late counts as blocked.  So every connect under a block list pays a bounded lookup per blocked name, and a blocked name that stops resolving refuses every connect until it resolves again or is removed.

**What that promises.**  An allow list refuses everything it was not told about.  A block list refuses the name in every question and every address the name is known to have; it cannot refuse an address the guest obtained some other way that the name is not currently shown to have, which no name-based rule can.  An embedder who needs a guarantee lists what is allowed.

Both lists take names and addresses, `AllowList::from_hosts(&["example.com", "10.0.0.5"])?`; a name that does not resolve when the list is built is a `ResolveError`.

### ListenPorts

`ListenPorts` is orthogonal to the outbound policy — it controls which ports the guest may `bind()` for inbound connections:

```rust
let ports = ListenPorts::from_ports([80, 443]);
let mut guest = SandboxBuilder::from_initrd(rootfs)
    .scratch_mb(256)
    .network(NetworkPolicy::AllowAll)
    .listen_ports(ports)
    .boot()?;
```

Ephemeral binds (port 0 — "assign any port") are always allowed.

A guest restored from a snapshot binds its listeners again on its `resume` entry, so their ports must be in the restoring sandbox's `ListenPorts` too.

### CLI usage

```sh
# AllowAll — full access:
hluk run --net ...

# AllowList — only reach these hosts (implies --net):
hluk run --net-allow example.com --net-allow 10.0.0.5 ...

# BlockList — block these hosts (implies --net):
hluk run --net-block evil.com --net-block 1.2.3.4 ...

# Restrict inbound listen ports:
hluk run --net --port 80 --port 443 ...
```

## Examples

- [`examples/python/tcp_echo.py`](../examples/python/tcp_echo.py) — TCP echo server and client running in two threads inside the guest (requires `NetworkPolicy::AllowAll` — uses loopback).
- [`examples/python/net_policy_probe.py`](../examples/python/net_policy_probe.py) — TCP connect and UDP sendto probe for integration-testing policy enforcement.
