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
4. The host's **hostnet** module looks up the virtual fd in a `SocketTable`, checks policy, and makes the matching [`rustix`](https://docs.rs/rustix) call — `rustix::net::connect`, `rustix::net::sendto`, `rustix::event::poll`, … — on the real OS socket.  rustix is a safe wrapper over the POSIX/WinSock socket API, so the host side is the same code on Linux, Windows and macOS.

### Host portability

The host side is the same code on Linux, Windows and macOS.  The guest is a Linux-ABI unikernel, so the host always speaks Linux numbers to it: errno values, `POLL*` bits and `SOL_*`/`SO_*` option numbers are translated at the boundary (`src/errno.rs`, `src/hostnet.rs`), and socket options are served from an allow-list of the int-valued options runtimes use (unknown ones return `-ENOPROTOOPT`).

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

When `None` (the default) and no listen ports are set, the `net_*` host functions refuse every `socket()` with `EACCES`, so guest socket calls fail; with listen ports, see [ListenPorts](#listenports).  They are registered either way, so a snapshot saved under a policy restores without one (its sockets die on resume, as their peers would have) and a snapshot saved without a policy restores under one.

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
- **Transfer size:** 69,620 bytes per `write` / `sendmsg` call, the most one host call carries, and 69,593 per `recvfrom` (its reply also carries the source address); a longer write returns a short count and the caller's loop continues. The guest takes the figure from the virtqueue pool sizes the host chose.
- **Protocols:** `AF_INET`/`AF_INET6` with `SOCK_STREAM`/`SOCK_DGRAM` reach the host. No raw sockets. `AF_UNIX` exists only inside the guest (`socketpair`, which asyncio's event loop needs), served by the kernel itself.

## Network policy

`NetworkPolicy` controls which outbound destinations a guest can reach.  It is a destination filter on the host's own sockets: no network namespace, no inbound filtering, no rate limit; isolation beyond that is the embedder's to provide.

| Variant | Behaviour |
|---------|-----------|
| `AllowAll` | Everything, except the metadata addresses. |
| `AllowList(AllowList)` | Only the listed names and addresses. |
| `BlockList(BlockList)` | Everything except the listed names and addresses. |
| `Filter(Filter)` | What the embedder's `NetworkFilter` allows, by name and port, asked at every question, TCP connect and UDP send. |

Under every variant the cloud metadata addresses are refused: the link-local ranges (`169.254.0.0/16`, `fe80::/10`) and AWS's IPv6 endpoint `fd00:ec2::254`.  The two lists and a filter also refuse loopback, and the unspecified address (`0.0.0.0`, `::`), which a connect sends to loopback, since a guest socket is a host socket and host services trust `127.0.0.1`; `AllowAll` permits it, so a server and a client can meet inside one guest.  An IPv4 address written as IPv4-mapped IPv6 (`::ffff:a.b.c.d`) is judged as `a.b.c.d`.

### Where it is checked

The guest resolves names itself, so a destination reaches the host as an address; the name is seen only in the DNS question.  Hence:

| Host call | Check |
|-----------|-------|
| `net_connect(fd, address)` | The destination, by address, for TCP.  UDP only selects a peer and source address; it sends nothing. |
| `net_sendto(fd, data, address)` | The destination; and, if the address is port 53, the DNS question in `data`, by name. |
| `net_send(fd, data)` | The destination of every connected UDP send; and the DNS question in `data`, by name, on a socket the guest connected to port 53.  Other TCP sends use the destination checked at connect. |
| `net_recvfrom(fd)` | Nothing is refused; under an allow list, an answer from a resolver the guest may ask has its addresses recorded. |
| `net_bind(fd, address)` | The inbound rule, [`ListenPorts`](#listenports). |

Nothing else is inspected.

UDP `connect()` is also how libc probes configured address families and source
addresses (`AI_ADDRCONFIG` in musl, address sorting in glibc).  These probes use
the real host socket and retain its family and routing errors, but transmit no
datagram.  Selecting even a loopback peer therefore succeeds when the host
supports it; every actual UDP send still checks its destination, including
after a reconnect or an `AF_UNSPEC` disconnect.  This does not grant access to
host-loopback services or to any unlisted destination.

**The question.**  Under an allow list the guest may ask only about listed names; under a block list, not about a blocked one; anything to port 53 that is not a well-formed query is refused.  An allow list lets questions reach the well-known resolvers (`8.8.8.8`, `8.8.4.4`, `1.1.1.1`, `1.0.0.1`) and the host's own from `/etc/resolv.conf`, on UDP only, and records only their answers.

**The destination.**  An allow list allows an address that was listed, resolved when the list was built, or recorded from an answer, and nothing else: the host resolves nothing on the guest's behalf, so an address the guest was never given is refused.  A block list refuses an address that was listed or resolved at build time, and whatever a blocked name resolves to now, looked up at the TCP connect or UDP send with a 250 ms deadline; a lookup that fails or runs late counts as blocked.  So every TCP connect and UDP send under a block list pays a bounded lookup per blocked name, and a blocked name that stops resolving refuses each until it resolves again or is removed.

**What that promises.**  An allow list refuses everything it was not told about.  A block list refuses the name in every question and every address the name is known to have; it cannot refuse an address the guest obtained some other way that the name is not currently shown to have, which no name-based rule can.  An embedder who needs a guarantee lists what is allowed.

Both lists take names and addresses, `AllowList::from_hosts(&["example.com", "10.0.0.5"])?`; a name that does not resolve when the list is built is a `ResolveError`.

### Filter

An embedder with rules of its own, finer than a list of names, or that change while the guest runs, implements `NetworkFilter` and passes `NetworkPolicy::Filter(Filter::new(filter))`, or `Filter::from_arc` to keep a handle on it:

```rust,no_run
use std::sync::{Arc, RwLock};
use hyperlight_unikraft::{Destination, Filter, NetworkFilter, NetworkPolicy, SandboxBuilder};

/// HTTPS to the listed names, which the embedder edits as the guest runs.
struct Https(RwLock<Vec<String>>);

impl NetworkFilter for Https {
    fn allows_name(&self, name: &str) -> bool {
        self.0.read().unwrap().iter().any(|n| n == name)
    }
    fn allows(&self, d: &Destination<'_>) -> bool {
        d.addr.port() == 443 && d.names.iter().any(|n| self.allows_name(n))
    }
}

let https = Arc::new(Https(RwLock::new(vec!["pypi.org".into()])));
let sandbox = SandboxBuilder::from_initrd("rootfs/python.cpio")
    .network(NetworkPolicy::Filter(Filter::from_arc(https.clone())))
    .boot()?;
https.0.write().unwrap().push("files.pythonhosted.org".into());
# Ok::<(), hyperlight_unikraft::Error>(())
```

It is asked the two things the host can tell: `allows_name(name)` for each name in a DNS question, in lower case without the trailing dot, and `allows(&Destination)` for each TCP connect and UDP send (connected or not), with the address, whether the socket is UDP, and `names`, the names whose answers gave the guest that address (recorded as under an allow list: only answers from a resolver the guest may ask, only for names `allows_name` allows; empty for an address the guest did not get from DNS).  Both are asked every time, so a change to what the filter allows applies from the next question, TCP connect or UDP send, without booting again.  Before the filter is asked, loopback, the unspecified address and the metadata addresses are refused and the resolvers an allow list exempts take questions on UDP port 53, as under `AllowList`.  The host sees no further than the address, the port and the names DNS gave the address: the scheme, path and method of what goes over the socket are not visible to a filter, nor which of the sites sharing an address (a CDN, a shared host) the guest talks to.  It runs while the guest is stopped in a host call, so it should answer from memory and must not call into the sandbox; a panic in it counts as a refusal.  `allows_name` is asked again when an answer arrives, before its addresses are credited to the name.  Answers over TCP are not read, so an address the guest learns only that way is not credited to its name.  What a `Filter` records is shared by its clones: give each sandbox its own, made with `Filter::from_arc` from the one filter to share the rules.

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

Listen ports need no outbound policy: without `.network(...)`, the guest may serve on them and reach nothing, as under an allow list that lists nothing — what a TCP server that only answers requests needs (its replies go over the accepted connections).  A UDP service cannot answer so: each reply is a `sendto()` to the client, an outbound destination like any other, so it needs a policy that allows its clients.

Without `ListenPorts`, every bind but an ephemeral one is refused with `EACCES`, a bind that only picks a source port for an outbound connection (`source_address`, `Dialer.LocalAddr`) included: a guest given a policy and no ports can serve only on a port the host picks.  Ephemeral binds (port 0 — "assign any port") are always allowed.  `ListenPorts::all()` permits every port, and `with_range(8000..=8010)` a range; on the command line these are `--port all` and `--port 8000-8010`, next to `--port 80`.  `all` is for an embedder whose own boundary already scopes what the guest exposes, such as a container runtime, where the guest binds inside the container's network namespace the way any container process does.

### Resolver configuration

The rootfs images ship an `/etc/resolv.conf` naming public resolvers, which is right on a laptop and wrong in a pod, where names resolve through the cluster's resolver and a search list that includes the pod's namespace.  That file belongs to the place the guest runs, so it is handed over at run time: `hluk run --resolv-conf FILE`, or `SandboxBuilder::resolv_conf(content)` in the library.  The kernel fetches it through the `GetResolvConf` host function once the rootfs is mounted and writes it as `/etc/resolv.conf`, and does so again on every restore, so a snapshot taken on one machine resolves names where it is resumed.  Without it the rootfs's file stands.  `options single-request` is added unless the file has it: glibc's parallel A and AAAA queries do not work through the socket layer.  Under an allow list, the file's nameservers are exempt on port 53 like the host's own, so a guest that may only reach listed names can still ask the resolver it was given.

A guest restored from a snapshot binds its listeners again on its `resume` entry, so their ports must be in the restoring sandbox's `ListenPorts` too.

### CLI usage

```sh
# AllowAll — full access:
hluk run --net ...

# AllowList — only reach these hosts (implies --net):
hluk run --net-allow example.com --net-allow 10.0.0.5 ...

# BlockList — block these hosts (implies --net):
hluk run --net-block evil.com --net-block 1.2.3.4 ...

# Let the guest serve on ports 80 and 443:
hluk run --net --port 80 --port 443 ...

# Serve on a port, reach nothing:
hluk run --port 8080 ...
```

## Examples

- [`examples/python/tcp_echo.py`](../examples/python/tcp_echo.py) — TCP echo server and client running in two threads inside the guest (requires `NetworkPolicy::AllowAll` — uses loopback).
- [`examples/python/net_policy_probe.py`](../examples/python/net_policy_probe.py) — TCP connect and UDP sendto probe for integration-testing policy enforcement.
