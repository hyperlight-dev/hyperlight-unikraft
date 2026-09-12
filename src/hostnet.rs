// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Host networking — `net_*` host functions backed by [`rustix`].
//!
//! The guest driver (`hostsock.c`) speaks the POSIX socket API, one host
//! call per syscall.  rustix is a safe, cross-platform (Linux; Windows
//! via WinSock and `WSAPoll`) wrapper over exactly that API, so every
//! host function here is one library call plus wire encoding.  Policy —
//! [`NetworkPolicy`] for outbound, [`ListenPorts`] for bind — is checked
//! before the corresponding syscall.
//!
//! A host function call is a synchronous VM exit: the guest is paused
//! until it returns, so no host call may block on a peer that lives in
//! the same guest.  Host sockets are therefore non-blocking: the guest
//! driver checks readiness with `net_poll(timeout=0)` first, and a call
//! that would block anyway returns `-EAGAIN`, which the guest's socket
//! layer already treats as "yield and retry".  The one exception is
//! `connect`, whose peer is never the guest itself: it waits for the
//! handshake to finish, as a blocking `connect` would.
//!
//! ## Return conventions
//!
//! - **`i32` returns**: non-negative on success (value is op-specific),
//!   negative = `-errno` on error.
//! - **`Vec<u8>` returns**: first 4 bytes are `i32` status, followed by
//!   packed data on success.
//!
//! Errno values are Linux errnos (see [`crate::errno`]) — the guest is a
//! Linux-ABI unikernel whatever the host OS is.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use hyperlight_host::func::Registerable;
use rustix::event::{PollFd, PollFlags, Timespec};
use rustix::fd::OwnedFd;
use rustix::io::Errno;
use rustix::net::{
    self, AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown, SocketFlags, SocketType, sockopt,
};

use crate::HOST_CALL_MAX;
use crate::errno::{self, from_rustix};
use crate::net_policy::{self, ListenPorts, NetworkPolicy};

const MAX_SOCKETS: usize = 1024;

/// Linux ABI constants the guest speaks.
mod abi {
    pub const AF_INET: i32 = 2;
    pub const AF_INET6: i32 = 10;
    pub const SOCK_STREAM: i32 = 1;
    pub const SOCK_DGRAM: i32 = 2;

    pub const SHUT_RD: i32 = 0;
    pub const SHUT_WR: i32 = 1;

    pub const POLLIN: i16 = 0x001;
    pub const POLLPRI: i16 = 0x002;
    pub const POLLOUT: i16 = 0x004;
    pub const POLLERR: i16 = 0x008;
    pub const POLLHUP: i16 = 0x010;
    pub const POLLNVAL: i16 = 0x020;

    pub const SOL_SOCKET: i32 = 1;
    pub const SO_REUSEADDR: i32 = 2;
    pub const SO_TYPE: i32 = 3;
    pub const SO_ERROR: i32 = 4;
    pub const SO_BROADCAST: i32 = 6;
    pub const SO_SNDBUF: i32 = 7;
    pub const SO_RCVBUF: i32 = 8;
    pub const SO_KEEPALIVE: i32 = 9;
    pub const SO_OOBINLINE: i32 = 10;
    pub const SO_LINGER: i32 = 13;
    #[cfg(not(windows))]
    pub const SO_REUSEPORT: i32 = 15;
    pub const SO_RCVTIMEO: i32 = 20;
    pub const SO_SNDTIMEO: i32 = 21;
    pub const SO_ACCEPTCONN: i32 = 30;
    #[cfg(target_os = "linux")]
    pub const SO_PROTOCOL: i32 = 38;
    #[cfg(target_os = "linux")]
    pub const SO_DOMAIN: i32 = 39;

    pub const IPPROTO_IP: i32 = 0;
    #[cfg(not(windows))]
    pub const IP_TOS: i32 = 1;
    pub const IP_TTL: i32 = 2;
    pub const IP_RECVERR: i32 = 11;
    pub const IP_MULTICAST_TTL: i32 = 33;
    pub const IP_MULTICAST_LOOP: i32 = 34;

    pub const IPPROTO_TCP: i32 = 6;
    pub const TCP_NODELAY: i32 = 1;
    pub const TCP_KEEPIDLE: i32 = 4;
    pub const TCP_KEEPINTVL: i32 = 5;
    pub const TCP_KEEPCNT: i32 = 6;

    pub const IPPROTO_IPV6: i32 = 41;
    pub const IPV6_UNICAST_HOPS: i32 = 16;
    pub const IPV6_MULTICAST_LOOP: i32 = 19;
    pub const IPV6_RECVERR: i32 = 25;
    pub const IPV6_V6ONLY: i32 = 26;
}
use abi::*;

/// `Err` carries the Linux errno number; it is negated on the wire.
type Res<T> = Result<T, i32>;

/// Flags for new and accepted sockets.  Close-on-exec keeps guest
/// sockets out of any child the host spawns; Windows has no such flag.
fn socket_flags() -> SocketFlags {
    #[cfg(windows)]
    {
        SocketFlags::empty()
    }
    #[cfg(not(windows))]
    {
        SocketFlags::CLOEXEC
    }
}

/// Never let a dead peer raise SIGPIPE in the host process.
fn send_flags() -> SendFlags {
    #[cfg(target_os = "linux")]
    {
        SendFlags::NOSIGNAL
    }
    #[cfg(not(target_os = "linux"))]
    {
        SendFlags::empty()
    }
}

// ── SocketTable ─────────────────────────────────────────────────────

/// Guest fd → host socket.  Dropping the fd closes the socket.
struct SocketTable {
    sockets: HashMap<i32, OwnedFd>,
    next_fd: i32,
}

impl SocketTable {
    fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            next_fd: 3, // skip stdin/stdout/stderr
        }
    }

    fn insert(&mut self, sock: OwnedFd) -> Res<i32> {
        if self.sockets.len() >= MAX_SOCKETS {
            return Err(errno::EMFILE);
        }
        let fd = self.next_fd;
        self.next_fd = self.next_fd.wrapping_add(1);
        self.sockets.insert(fd, sock);
        Ok(fd)
    }

    fn get(&self, fd: i32) -> Res<&OwnedFd> {
        self.sockets.get(&fd).ok_or(errno::EBADF)
    }

    fn remove(&mut self, fd: i32) -> Res<OwnedFd> {
        self.sockets.remove(&fd).ok_or(errno::EBADF)
    }
}

// ── Net ─────────────────────────────────────────────────────────────

/// Per-sandbox networking state shared by all `net_*` host functions.
struct Net {
    /// The `net_*` closures share this state through an `Arc`, which
    /// needs it to be `Sync` (Hyperlight wants each closure `Send +
    /// 'static`).  Hence a mutex, even though a sandbox has one vCPU
    /// and never makes two host calls at once: the lock is uncontended
    /// and is held across the (non-blocking) syscalls without harm.
    table: Mutex<SocketTable>,
    policy: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
}

impl Net {
    fn new(policy: Option<NetworkPolicy>, listen_ports: Option<ListenPorts>) -> Self {
        // WinSock must be initialised before any socket call; std does
        // this lazily for its own types but rustix calls it directly.
        #[cfg(windows)]
        {
            static WSA: std::sync::Once = std::sync::Once::new();
            WSA.call_once(|| {
                let _ = net::wsa_startup();
            });
        }
        Self {
            table: Mutex::new(SocketTable::new()),
            policy,
            listen_ports,
        }
    }

    fn table(&self) -> MutexGuard<'_, SocketTable> {
        self.table.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Outbound policy (connect, sendto).
    fn allow_outbound(&self, addr: &SocketAddr) -> Res<()> {
        match &self.policy {
            Some(p) if p.check(addr).is_err() => Err(errno::EACCES),
            _ => Ok(()),
        }
    }

    /// Inbound policy (bind).  Ephemeral binds (port 0) are always allowed.
    fn allow_bind(&self, addr: &SocketAddr) -> Res<()> {
        match &self.listen_ports {
            Some(lp) if addr.port() != 0 && lp.check(addr.port()).is_err() => Err(errno::EACCES),
            _ => Ok(()),
        }
    }

    fn socket(&self, family: i32, ty: i32, proto: i32) -> Res<i32> {
        let af = match family {
            AF_INET => AddressFamily::INET,
            AF_INET6 => AddressFamily::INET6,
            _ => return Err(errno::EAFNOSUPPORT),
        };
        // The low byte is the type; the upper bits (SOCK_NONBLOCK,
        // SOCK_CLOEXEC) are guest-side concerns.
        let ty = match ty & 0xff {
            SOCK_STREAM => SocketType::STREAM,
            SOCK_DGRAM => SocketType::DGRAM,
            _ => return Err(errno::EPROTONOSUPPORT),
        };
        // Forward the protocol so the OS validates the (type, protocol)
        // pair and e.g. ICMP datagram sockets stay ICMP.
        let proto = match u32::try_from(proto) {
            Ok(0) => None,
            Ok(p) => NonZeroU32::new(p).map(Protocol::from_raw),
            Err(_) => return Err(errno::EINVAL),
        };
        let sock = net::socket_with(af, ty, socket_flags(), proto).map_err(from_rustix)?;
        rustix::io::ioctl_fionbio(&sock, true).map_err(from_rustix)?;
        // Linux AF_INET6 sockets are dual-stack by default; Windows's are
        // v6-only.  Give the guest the Linux default (it can still opt in).
        #[cfg(windows)]
        if af == AddressFamily::INET6 {
            let _ = sockopt::set_ipv6_v6only(&sock, false);
        }
        self.table().insert(sock)
    }

    fn bind(&self, fd: i32, addr: SocketAddr) -> Res<()> {
        self.allow_bind(&addr)?;
        net::bind(self.table().get(fd)?, &addr).map_err(from_rustix)
    }

    fn listen(&self, fd: i32, backlog: i32) -> Res<()> {
        net::listen(self.table().get(fd)?, backlog).map_err(from_rustix)
    }

    fn accept(&self, fd: i32) -> Res<(i32, Option<SocketAddr>)> {
        let mut tbl = self.table();
        let (conn, peer) =
            net::acceptfrom_with(tbl.get(fd)?, socket_flags()).map_err(from_rustix)?;
        rustix::io::ioctl_fionbio(&conn, true).map_err(from_rustix)?;
        let peer = peer.and_then(|a| SocketAddr::try_from(a).ok());
        Ok((tbl.insert(conn)?, peer))
    }

    /// Completes the handshake before returning, so the vCPU is frozen
    /// for as long as the peer takes — up to the OS's SYN retry timeout
    /// for an unreachable one (about 2 minutes on Linux, 20 seconds on
    /// Windows).  The guest driver has no non-blocking connect protocol
    /// that would let us do better.
    fn connect(&self, fd: i32, addr: SocketAddr) -> Res<()> {
        self.allow_outbound(&addr)?;
        let tbl = self.table();
        let fd = tbl.get(fd)?;
        match net::connect(fd, &addr) {
            Ok(()) => Ok(()),
            // Handshake in progress: wait for it like a blocking connect
            // would (the OS still times out SYN retries), then report
            // the outcome the socket recorded.
            Err(Errno::INPROGRESS) | Err(Errno::WOULDBLOCK) => {
                let mut pfd = [PollFd::new(fd, PollFlags::OUT)];
                rustix::event::poll(&mut pfd, None).map_err(from_rustix)?;
                sockopt::socket_error(fd)
                    .map_err(from_rustix)?
                    .map_err(from_rustix)?;
                // Linux keeps a non-blocking socket "connecting" until a
                // later connect() observes the established state.  Make
                // that observation here so a guest that connects again
                // gets EISCONN, exactly as after a blocking connect.
                match net::connect(fd, &addr) {
                    Ok(()) | Err(Errno::ISCONN) => Ok(()),
                    Err(e) => Err(from_rustix(e)),
                }
            }
            Err(e) => Err(from_rustix(e)),
        }
    }

    fn send(&self, fd: i32, data: &[u8]) -> Res<usize> {
        net::send(self.table().get(fd)?, data, send_flags()).map_err(from_rustix)
    }

    fn sendto(&self, fd: i32, data: &[u8], addr: SocketAddr) -> Res<usize> {
        self.allow_outbound(&addr)?;
        net::sendto(self.table().get(fd)?, data, send_flags(), &addr).map_err(from_rustix)
    }

    /// Receive up to `len` bytes plus the source address when the
    /// socket has one (datagrams).  `len` is capped at what one host
    /// call can carry back; the guest asks for at most that anyway.
    fn recvfrom(&self, fd: i32, len: usize) -> Res<(Vec<u8>, Option<SocketAddr>)> {
        let mut buf = vec![0u8; len.min(HOST_CALL_MAX)];
        let tbl = self.table();
        let fd = tbl.get(fd)?;
        let (n, _, from) = match net::recvfrom(fd, &mut buf[..], RecvFlags::empty()) {
            Ok(r) => r,
            // Windows surfaces an earlier ICMP "port unreachable" on a UDP
            // socket as a reset.  Linux reports that as ECONNREFUSED on a
            // connected socket and not at all on an unconnected one.
            #[cfg(windows)]
            Err(Errno::CONNRESET)
                if sockopt::socket_type(fd).is_ok_and(|t| t == SocketType::DGRAM) =>
            {
                return Err(match net::getpeername(fd) {
                    Ok(Some(_)) => errno::ECONNREFUSED,
                    _ => errno::EAGAIN,
                });
            }
            Err(e) => return Err(from_rustix(e)),
        };
        buf.truncate(n);
        let from = from.and_then(|a| SocketAddr::try_from(a).ok());
        // Under an allow-list, learn the IPs in DNS answers so the guest
        // can reach what it just resolved.
        if let (Some(NetworkPolicy::AllowList(al)), Some(src)) = (&self.policy, from)
            && src.port() == 53
        {
            net_policy::learn_ips_from_dns_response(&buf, al);
        }
        Ok((buf, from))
    }

    fn shutdown(&self, fd: i32, how: i32) -> Res<()> {
        let how = match how {
            SHUT_RD => Shutdown::Read,
            SHUT_WR => Shutdown::Write,
            _ => Shutdown::Both,
        };
        net::shutdown(self.table().get(fd)?, how).map_err(from_rustix)
    }

    fn close(&self, fd: i32) -> Res<()> {
        self.table().remove(fd).map(drop)
    }

    fn peer_addr(&self, fd: i32) -> Res<SocketAddr> {
        net::getpeername(self.table().get(fd)?)
            .map_err(from_rustix)?
            .and_then(|a| SocketAddr::try_from(a).ok())
            .ok_or(errno::ENOTCONN)
    }

    fn local_addr(&self, fd: i32) -> Res<SocketAddr> {
        let addr = net::getsockname(self.table().get(fd)?).map_err(from_rustix)?;
        SocketAddr::try_from(addr).map_err(|_| errno::EAFNOSUPPORT)
    }

    /// `getsockopt` for the int-valued options guests actually use.
    /// Anything else is `ENOPROTOOPT`.
    fn getsockopt(&self, fd: i32, level: i32, name: i32) -> Res<i32> {
        let tbl = self.table();
        let fd = tbl.get(fd)?;
        let as_i32 = |v: usize| i32::try_from(v).unwrap_or(i32::MAX);
        let secs = |d: Duration| i32::try_from(d.as_secs()).unwrap_or(i32::MAX);
        let r: Result<i32, Errno> = match (level, name) {
            (SOL_SOCKET, SO_REUSEADDR) => sockopt::socket_reuseaddr(fd).map(i32::from),
            (SOL_SOCKET, SO_TYPE) => sockopt::socket_type(fd).map(|t| {
                if t == SocketType::STREAM {
                    SOCK_STREAM
                } else {
                    SOCK_DGRAM
                }
            }),
            (SOL_SOCKET, SO_ERROR) => {
                sockopt::socket_error(fd).map(|r| r.err().map_or(0, from_rustix))
            }
            (SOL_SOCKET, SO_BROADCAST) => sockopt::socket_broadcast(fd).map(i32::from),
            (SOL_SOCKET, SO_SNDBUF) => sockopt::socket_send_buffer_size(fd).map(as_i32),
            (SOL_SOCKET, SO_RCVBUF) => sockopt::socket_recv_buffer_size(fd).map(as_i32),
            (SOL_SOCKET, SO_KEEPALIVE) => sockopt::socket_keepalive(fd).map(i32::from),
            (SOL_SOCKET, SO_OOBINLINE) => sockopt::socket_oobinline(fd).map(i32::from),
            (SOL_SOCKET, SO_ACCEPTCONN) => sockopt::socket_acceptconn(fd).map(i32::from),
            #[cfg(not(windows))]
            (SOL_SOCKET, SO_REUSEPORT) => sockopt::socket_reuseport(fd).map(i32::from),
            #[cfg(target_os = "linux")]
            (SOL_SOCKET, SO_DOMAIN) => sockopt::socket_domain(fd).map(|d| {
                if d == AddressFamily::INET6 {
                    AF_INET6
                } else {
                    AF_INET
                }
            }),
            #[cfg(target_os = "linux")]
            (SOL_SOCKET, SO_PROTOCOL) => sockopt::socket_protocol(fd)
                .map(|p| p.map_or(0, |p| i32::try_from(p.as_raw().get()).unwrap_or(0))),
            #[cfg(not(windows))]
            (IPPROTO_IP, IP_TOS) => sockopt::ip_tos(fd).map(i32::from),
            (IPPROTO_IP, IP_TTL) => sockopt::ip_ttl(fd).map(|v| v as i32),
            (IPPROTO_IP, IP_MULTICAST_TTL) => sockopt::ip_multicast_ttl(fd).map(|v| v as i32),
            (IPPROTO_IP, IP_MULTICAST_LOOP) => sockopt::ip_multicast_loop(fd).map(i32::from),
            (IPPROTO_IP, IP_RECVERR) | (IPPROTO_IPV6, IPV6_RECVERR) => Ok(0),
            (IPPROTO_TCP, TCP_NODELAY) => sockopt::tcp_nodelay(fd).map(i32::from),
            (IPPROTO_TCP, TCP_KEEPIDLE) => sockopt::tcp_keepidle(fd).map(secs),
            (IPPROTO_TCP, TCP_KEEPINTVL) => sockopt::tcp_keepintvl(fd).map(secs),
            (IPPROTO_TCP, TCP_KEEPCNT) => sockopt::tcp_keepcnt(fd).map(|v| v as i32),
            (IPPROTO_IPV6, IPV6_V6ONLY) => sockopt::ipv6_v6only(fd).map(i32::from),
            (IPPROTO_IPV6, IPV6_UNICAST_HOPS) => sockopt::ipv6_unicast_hops(fd).map(i32::from),
            // IPV6_MULTICAST_HOPS is deliberately absent: rustix 1.1 issues
            // it at level IPPROTO_IP, which on Linux is IP_PASSSEC.
            (IPPROTO_IPV6, IPV6_MULTICAST_LOOP) => sockopt::ipv6_multicast_loop(fd).map(i32::from),
            _ => return Err(errno::ENOPROTOOPT),
        };
        r.map_err(from_rustix)
    }

    /// `setsockopt` counterpart of [`Self::getsockopt`].
    fn setsockopt(&self, fd: i32, level: i32, name: i32, value: i32) -> Res<()> {
        let tbl = self.table();
        let fd = tbl.get(fd)?;
        let on = value != 0;
        let size = usize::try_from(value).unwrap_or(0);
        let secs = Duration::from_secs(u64::try_from(value).unwrap_or(0));
        let r = match (level, name) {
            #[cfg(not(windows))]
            (SOL_SOCKET, SO_REUSEADDR) => sockopt::set_socket_reuseaddr(fd, on),
            // On Windows SO_REUSEADDR means "may bind a port another socket
            // is actively using" (port hijacking), while rebinding a
            // TIME_WAIT port — what Linux callers want — is already allowed.
            #[cfg(windows)]
            (SOL_SOCKET, SO_REUSEADDR) => Ok(()),
            (SOL_SOCKET, SO_BROADCAST) => sockopt::set_socket_broadcast(fd, on),
            (SOL_SOCKET, SO_SNDBUF) => sockopt::set_socket_send_buffer_size(fd, size),
            (SOL_SOCKET, SO_RCVBUF) => sockopt::set_socket_recv_buffer_size(fd, size),
            (SOL_SOCKET, SO_KEEPALIVE) => sockopt::set_socket_keepalive(fd, on),
            (SOL_SOCKET, SO_OOBINLINE) => sockopt::set_socket_oobinline(fd, on),
            #[cfg(not(windows))]
            (SOL_SOCKET, SO_REUSEPORT) => sockopt::set_socket_reuseport(fd, on),
            // Struct-valued options — the guest only forwards the first int.
            (SOL_SOCKET, SO_LINGER | SO_RCVTIMEO | SO_SNDTIMEO) => return Err(errno::EINVAL),
            #[cfg(not(windows))]
            (IPPROTO_IP, IP_TOS) => sockopt::set_ip_tos(fd, value as u8),
            (IPPROTO_IP, IP_TTL) => sockopt::set_ip_ttl(fd, value as u32),
            (IPPROTO_IP, IP_MULTICAST_TTL) => sockopt::set_ip_multicast_ttl(fd, value as u32),
            (IPPROTO_IP, IP_MULTICAST_LOOP) => sockopt::set_ip_multicast_loop(fd, on),
            // Linux-only extended ICMP error queueing.  glibc's resolver
            // enables it and treats failure as fatal, so accept it; the
            // errors it would queue are already surfaced through the
            // normal return values on connected sockets.
            (IPPROTO_IP, IP_RECVERR) | (IPPROTO_IPV6, IPV6_RECVERR) => Ok(()),
            (IPPROTO_TCP, TCP_NODELAY) => sockopt::set_tcp_nodelay(fd, on),
            (IPPROTO_TCP, TCP_KEEPIDLE) => sockopt::set_tcp_keepidle(fd, secs),
            (IPPROTO_TCP, TCP_KEEPINTVL) => sockopt::set_tcp_keepintvl(fd, secs),
            (IPPROTO_TCP, TCP_KEEPCNT) => sockopt::set_tcp_keepcnt(fd, value as u32),
            (IPPROTO_IPV6, IPV6_V6ONLY) => sockopt::set_ipv6_v6only(fd, on),
            (IPPROTO_IPV6, IPV6_UNICAST_HOPS) => {
                sockopt::set_ipv6_unicast_hops(fd, u8::try_from(value).ok())
            }
            (IPPROTO_IPV6, IPV6_MULTICAST_LOOP) => sockopt::set_ipv6_multicast_loop(fd, on),
            _ => return Err(errno::ENOPROTOOPT),
        };
        r.map_err(from_rustix)
    }

    /// `poll(2)` over guest fds.  Returns the ready count (or `-errno`)
    /// and one Linux-encoded `revents` per entry.
    fn poll(&self, entries: &[(i32, i16)], timeout_ms: i32) -> (i32, Vec<i16>) {
        let tbl = self.table();
        let mut revents = vec![0i16; entries.len()];
        let mut fds = Vec::with_capacity(entries.len());
        let mut index = Vec::with_capacity(entries.len());
        let mut invalid = 0i32;
        for (i, &(fd, events)) in entries.iter().enumerate() {
            match tbl.get(fd) {
                Ok(s) => {
                    fds.push(PollFd::new(s, poll_flags(events)));
                    index.push(i);
                }
                Err(_) => {
                    revents[i] = POLLNVAL;
                    invalid += 1;
                }
            }
        }
        // Negative timeout = block indefinitely.  An invalid entry counts
        // as ready, so with one present poll must return at once.
        let timeout_ms = if invalid > 0 {
            Some(0)
        } else {
            u64::try_from(timeout_ms).ok()
        };
        let timeout = timeout_ms.map(|ms| Timespec {
            tv_sec: (ms / 1000) as _,
            tv_nsec: ((ms % 1000) * 1_000_000) as _,
        });
        if fds.is_empty() {
            // Nothing to poll: sleep for the timeout, as Linux does.
            match timeout_ms {
                Some(ms) => std::thread::sleep(Duration::from_millis(ms)),
                None => loop {
                    std::thread::sleep(Duration::from_secs(3600));
                },
            }
            return (invalid, revents);
        }
        match rustix::event::poll(&mut fds, timeout.as_ref()) {
            Ok(ready) => {
                for (pfd, &i) in fds.iter().zip(&index) {
                    revents[i] = linux_revents(pfd.revents());
                }
                (ready as i32 + invalid, revents)
            }
            Err(e) => (-from_rustix(e), revents),
        }
    }
}

/// Guest `events` → platform poll flags.
fn poll_flags(events: i16) -> PollFlags {
    let mut f = PollFlags::empty();
    if events & POLLIN != 0 {
        f |= PollFlags::IN;
    }
    if events & POLLOUT != 0 {
        f |= PollFlags::OUT;
    }
    // WSAPoll rejects POLLPRI in `events`.
    #[cfg(not(windows))]
    if events & POLLPRI != 0 {
        f |= PollFlags::PRI;
    }
    f
}

/// Platform `revents` → guest encoding.
///
/// Be conservative here: the guest driver polls every socket for
/// `IN|OUT` right after creating it and caches the answer until an
/// operation returns `EAGAIN`, so readiness that isn't backed by data
/// makes a guest thread spin without yielding.  Changes must pass the
/// intra-guest loopback tests (`tcp_echo`, `tcp_bidir`,
/// `threaded_select`) on both host OSes.
fn linux_revents(f: PollFlags) -> i16 {
    let mut r = 0;
    // `IN`/`OUT` are unions on Windows (RDNORM|RDBAND, WRNORM) and the
    // kernel may set RDNORM/WRNORM alongside — test by intersection.
    if f.intersects(PollFlags::IN | PollFlags::RDNORM) {
        r |= POLLIN;
    }
    if f.intersects(PollFlags::OUT | PollFlags::WRNORM) {
        r |= POLLOUT;
    }
    if f.contains(PollFlags::PRI) {
        r |= POLLPRI;
    }
    // The guest only acts on IN/OUT.  An errored socket is readable and
    // writable (the operations return the error); Linux reports it that
    // way itself, WSAPoll reports ERR alone.
    if f.contains(PollFlags::ERR) {
        r |= POLLERR | POLLIN | POLLOUT;
    }
    // Hang-up: WSAPoll reports a peer's FIN as HUP alone, but a read must
    // be seen as ready so the guest observes EOF.  Linux already sets IN
    // for EOF and uses HUP for *unconnected* sockets, where adding IN
    // would make the guest spin on a socket that has nothing to read.
    if f.contains(PollFlags::HUP) {
        r |= POLLHUP;
        #[cfg(windows)]
        {
            r |= POLLIN;
        }
    }
    if f.contains(PollFlags::NVAL) {
        r |= POLLNVAL;
    }
    r
}

// ── Registration ─────────────────────────────────────────────────────

/// `Ok(v)` → `ok(v)`, `Err(errno)` → `-errno`.
fn ret<T>(r: Res<T>, ok: impl FnOnce(T) -> i32) -> i32 {
    match r {
        Ok(v) => ok(v),
        Err(e) => -e,
    }
}

/// `Ok(body)` → `body` (which starts with its own status), `Err(errno)` → `-errno`.
fn ret_vec(r: Res<Vec<u8>>) -> Vec<u8> {
    r.unwrap_or_else(|e| (-e).to_le_bytes().to_vec())
}

/// Register all `net_*` host functions plus `net_resolve` and `host_nanosleep`.
///
/// `policy` controls which outbound destinations are allowed.
/// `listen_ports` controls which ports `net_bind` accepts.
pub(crate) fn register(
    target: &mut impl Registerable,
    policy: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
) -> hyperlight_host::Result<()> {
    let net = Arc::new(Net::new(policy, listen_ports));

    // net_socket(family, type, protocol) -> fd or -errno
    let n = net.clone();
    target.register_host_function(
        "net_socket",
        move |family: i32, ty: i32, proto: i32| -> hyperlight_host::Result<i32> {
            Ok(ret(n.socket(family, ty, proto), |fd| fd))
        },
    )?;

    // net_bind(fd, family, addr, port) -> 0 or -errno
    let n = net.clone();
    target.register_host_function(
        "net_bind",
        move |fd: i32, family: i32, addr: String, port: i32| -> hyperlight_host::Result<i32> {
            let Some(sa) = parse_addr(family, &addr, port) else {
                return Ok(-errno::EINVAL);
            };
            Ok(ret(n.bind(fd, sa), |()| 0))
        },
    )?;

    // net_listen(fd, backlog) -> 0 or -errno
    let n = net.clone();
    target.register_host_function(
        "net_listen",
        move |fd: i32, backlog: i32| -> hyperlight_host::Result<i32> {
            Ok(ret(n.listen(fd, backlog), |()| 0))
        },
    )?;

    // net_accept(fd) -> [i32 new_fd | packed peer addr]
    let n = net.clone();
    target.register_host_function(
        "net_accept",
        move |fd: i32| -> hyperlight_host::Result<Vec<u8>> {
            Ok(ret_vec(n.accept(fd).map(|(new_fd, peer)| {
                let mut buf = Vec::with_capacity(32);
                buf.extend(new_fd.to_le_bytes());
                if let Some(peer) = peer {
                    pack_addr(&mut buf, &peer);
                }
                buf
            })))
        },
    )?;

    // net_connect(fd, family, addr, port) -> 0 or -errno
    let n = net.clone();
    target.register_host_function(
        "net_connect",
        move |fd: i32, family: i32, addr: String, port: i32| -> hyperlight_host::Result<i32> {
            let Some(sa) = parse_addr(family, &addr, port) else {
                return Ok(-errno::EINVAL);
            };
            Ok(ret(n.connect(fd, sa), |()| 0))
        },
    )?;

    // net_send(fd, data) -> bytes_sent or -errno
    let n = net.clone();
    target.register_host_function(
        "net_send",
        move |fd: i32, data: Vec<u8>| -> hyperlight_host::Result<i32> {
            Ok(ret(n.send(fd, &data), |sent| sent as i32))
        },
    )?;

    // net_sendto(fd, data, family, addr, port) -> bytes_sent or -errno
    let n = net.clone();
    target.register_host_function(
        "net_sendto",
        move |fd: i32,
              data: Vec<u8>,
              family: i32,
              addr: String,
              port: i32|
              -> hyperlight_host::Result<i32> {
            let Some(sa) = parse_addr(family, &addr, port) else {
                return Ok(-errno::EINVAL);
            };
            Ok(ret(n.sendto(fd, &data, sa), |sent| sent as i32))
        },
    )?;

    // net_recvfrom(fd, len) -> [i32 len | packed src addr | data]
    let n = net.clone();
    target.register_host_function(
        "net_recvfrom",
        move |fd: i32, len: i32| -> hyperlight_host::Result<Vec<u8>> {
            let len = usize::try_from(len).unwrap_or(0);
            Ok(ret_vec(n.recvfrom(fd, len).map(|(data, from)| {
                let mut buf = Vec::with_capacity(32 + data.len());
                buf.extend((data.len() as i32).to_le_bytes());
                match from {
                    Some(sa) => pack_addr(&mut buf, &sa),
                    None => pack_no_addr(&mut buf),
                }
                buf.extend_from_slice(&data);
                buf
            })))
        },
    )?;

    // net_shutdown(fd, how) -> 0 or -errno
    let n = net.clone();
    target.register_host_function(
        "net_shutdown",
        move |fd: i32, how: i32| -> hyperlight_host::Result<i32> {
            Ok(ret(n.shutdown(fd, how), |()| 0))
        },
    )?;

    // net_close(fd) -> 0 or -errno
    let n = net.clone();
    target.register_host_function(
        "net_close",
        move |fd: i32| -> hyperlight_host::Result<i32> { Ok(ret(n.close(fd), |()| 0)) },
    )?;

    // net_getpeername(fd) -> [i32 status | packed addr]
    let n = net.clone();
    target.register_host_function(
        "net_getpeername",
        move |fd: i32| -> hyperlight_host::Result<Vec<u8>> {
            Ok(ret_vec(n.peer_addr(fd).map(|sa| addr_result(&sa))))
        },
    )?;

    // net_getsockname(fd) -> [i32 status | packed addr]
    let n = net.clone();
    target.register_host_function(
        "net_getsockname",
        move |fd: i32| -> hyperlight_host::Result<Vec<u8>> {
            Ok(ret_vec(n.local_addr(fd).map(|sa| addr_result(&sa))))
        },
    )?;

    // net_getsockopt(fd, level, optname) -> value or -errno
    let n = net.clone();
    target.register_host_function(
        "net_getsockopt",
        move |fd: i32, level: i32, optname: i32| -> hyperlight_host::Result<i32> {
            Ok(ret(n.getsockopt(fd, level, optname), |v| v))
        },
    )?;

    // net_setsockopt(fd, level, optname, value) -> 0 or -errno
    let n = net.clone();
    target.register_host_function(
        "net_setsockopt",
        move |fd: i32, level: i32, optname: i32, value: i32| -> hyperlight_host::Result<i32> {
            Ok(ret(n.setsockopt(fd, level, optname, value), |()| 0))
        },
    )?;

    // net_poll(pollfds, timeout_ms) -> [i32 ready | i16 revents per fd]
    //
    // Input is 8 bytes per fd: i32 fd, i16 events, i16 pad.
    let n = net.clone();
    target.register_host_function(
        "net_poll",
        move |pollfds: Vec<u8>, timeout_ms: i32| -> hyperlight_host::Result<Vec<u8>> {
            let entries: Vec<(i32, i16)> = pollfds
                .as_chunks::<8>()
                .0
                .iter()
                .map(|c| {
                    (
                        i32::from_le_bytes([c[0], c[1], c[2], c[3]]),
                        i16::from_le_bytes([c[4], c[5]]),
                    )
                })
                .collect();
            let (ready, revents) = n.poll(&entries, timeout_ms);
            let mut buf = Vec::with_capacity(4 + revents.len() * 2);
            buf.extend(ready.to_le_bytes());
            for r in revents {
                buf.extend(r.to_le_bytes());
            }
            Ok(buf)
        },
    )?;

    // net_resolve(hostname) -> "ip1,ip2,..." or "error:reason"
    target.register_host_function(
        "net_resolve",
        move |hostname: String| -> hyperlight_host::Result<String> {
            // ToSocketAddrs requires a port — use 0.
            match format!("{hostname}:0").to_socket_addrs() {
                Ok(addrs) => {
                    let ips: Vec<String> = addrs.map(|a| a.ip().to_string()).collect();
                    if ips.is_empty() {
                        Ok("error:ENOENT".to_string())
                    } else {
                        Ok(ips.join(","))
                    }
                }
                Err(e) => Ok(format!("error:{e}")),
            }
        },
    )?;

    // host_nanosleep(ns) -> 0
    target.register_host_function(
        "host_nanosleep",
        move |ns: u64| -> hyperlight_host::Result<i32> {
            std::thread::sleep(Duration::from_nanos(ns.min(30_000_000_000))); // cap 30s
            Ok(0)
        },
    )?;

    Ok(())
}

// ── Wire encoding ────────────────────────────────────────────────────

fn parse_addr(family: i32, addr: &str, port: i32) -> Option<SocketAddr> {
    let port = u16::try_from(port).ok()?;
    let ip: IpAddr = match family {
        AF_INET => addr.parse::<Ipv4Addr>().ok()?.into(),
        AF_INET6 => addr.parse::<Ipv6Addr>().ok()?.into(),
        _ => return None,
    };
    Some(SocketAddr::new(ip, port))
}

/// Pack an address: family(i32) + port(u16) + addr_len(u8) + addr bytes.
fn pack_addr(buf: &mut Vec<u8>, addr: &SocketAddr) {
    match addr {
        SocketAddr::V4(v4) => {
            buf.extend(AF_INET.to_le_bytes());
            buf.extend(v4.port().to_le_bytes());
            buf.push(4);
            buf.extend_from_slice(&v4.ip().octets());
        }
        SocketAddr::V6(v6) => {
            buf.extend(AF_INET6.to_le_bytes());
            buf.extend(v6.port().to_le_bytes());
            buf.push(16);
            buf.extend_from_slice(&v6.ip().octets());
        }
    }
}

/// Pack "no address": family 0, port 0, addr_len 0.
fn pack_no_addr(buf: &mut Vec<u8>) {
    buf.extend(0i32.to_le_bytes());
    buf.extend(0u16.to_le_bytes());
    buf.push(0);
}

/// Successful address result: status 0 + packed addr.
fn addr_result(sa: &SocketAddr) -> Vec<u8> {
    let mut buf = Vec::with_capacity(24);
    buf.extend(0i32.to_le_bytes());
    pack_addr(&mut buf, sa);
    buf
}

// ── Tests ────────────────────────────────────────────────────────────
//
// These drive the socket layer directly (no hypervisor) so they run on
// every host OS, which is where the portability bugs live.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net_policy::AllowList;

    fn open_net() -> Net {
        Net::new(Some(NetworkPolicy::AllowAll), None)
    }

    fn loopback(port: u16) -> SocketAddr {
        SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port)
    }

    fn tcp(net: &Net) -> i32 {
        net.socket(AF_INET, SOCK_STREAM, 0).unwrap()
    }

    fn udp(net: &Net) -> i32 {
        net.socket(AF_INET, SOCK_DGRAM, 0).unwrap()
    }

    /// Wait until `fd` reports `events` (bounded), returning revents.
    fn wait(net: &Net, fd: i32, events: i16) -> i16 {
        let (ready, rev) = net.poll(&[(fd, events)], 5000);
        assert!(
            ready > 0,
            "poll timed out waiting for {events:#x} on fd {fd}"
        );
        rev[0]
    }

    /// Receive, retrying on EAGAIN until data or EOF arrives.  Like the
    /// guest driver, only proceeds on `POLLIN` — EOF must report it too.
    fn recv(net: &Net, fd: i32) -> Vec<u8> {
        loop {
            assert_ne!(wait(net, fd, POLLIN) & POLLIN, 0, "readable without POLLIN");
            match net.recvfrom(fd, 65536) {
                Ok((data, _)) => return data,
                Err(e) if e == errno::EAGAIN => continue,
                Err(e) => panic!("recvfrom failed: errno {e}"),
            }
        }
    }

    #[test]
    fn tcp_loopback_round_trip() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 4).unwrap();
        let addr = net.local_addr(srv).unwrap();
        assert_eq!(net.getsockopt(srv, SOL_SOCKET, SO_ACCEPTCONN).unwrap(), 1);

        let cli = tcp(&net);
        assert_eq!(net.peer_addr(cli), Err(errno::ENOTCONN));
        net.connect(cli, addr).unwrap();
        assert_eq!(net.connect(cli, addr), Err(errno::EISCONN));
        assert_eq!(net.peer_addr(cli).unwrap(), addr);

        assert_ne!(wait(&net, srv, POLLIN) & POLLIN, 0);
        let (conn, peer) = net.accept(srv).unwrap();
        assert_eq!(peer, Some(net.local_addr(cli).unwrap()));
        // Nothing pending on the listener any more, and an idle stream is
        // writable but not readable.
        let (ready, rev) = net.poll(&[(srv, POLLIN), (cli, POLLIN | POLLOUT)], 0);
        assert_eq!((ready, rev), (1, vec![0, POLLOUT]));
        assert_eq!(net.recvfrom(cli, 64), Err(errno::EAGAIN));

        assert_eq!(net.send(cli, b"ping").unwrap(), 4);
        assert_eq!(recv(&net, conn), b"ping");

        assert_eq!(
            net.getsockopt(cli, SOL_SOCKET, SO_TYPE).unwrap(),
            SOCK_STREAM
        );
        assert_eq!(net.getsockopt(cli, SOL_SOCKET, SO_ERROR).unwrap(), 0);
        net.setsockopt(cli, IPPROTO_TCP, TCP_NODELAY, 1).unwrap();
        assert_eq!(net.getsockopt(cli, IPPROTO_TCP, TCP_NODELAY).unwrap(), 1);
        assert_eq!(
            net.getsockopt(cli, SOL_SOCKET, 9999),
            Err(errno::ENOPROTOOPT)
        );
        assert_eq!(
            net.setsockopt(cli, SOL_SOCKET, SO_LINGER, 1),
            Err(errno::EINVAL)
        );
        // glibc's resolver requires this to succeed.
        net.setsockopt(cli, IPPROTO_IP, IP_RECVERR, 1).unwrap();
        assert_eq!(net.getsockopt(cli, IPPROTO_IP, IP_RECVERR).unwrap(), 0);

        // Half-close → EOF on the other side.
        net.shutdown(cli, SHUT_WR).unwrap();
        assert!(recv(&net, conn).is_empty());

        for fd in [cli, conn, srv] {
            net.close(fd).unwrap();
        }
        assert_eq!(net.close(cli), Err(errno::EBADF));
    }

    /// A send that outruns the peer must never block the host: the guest
    /// driver relies on `-EAGAIN` to yield to the receiving thread.
    #[test]
    fn large_transfer_never_blocks() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 1).unwrap();
        let cli = tcp(&net);
        net.connect(cli, net.local_addr(srv).unwrap()).unwrap();
        wait(&net, srv, POLLIN);
        let (conn, _) = net.accept(srv).unwrap();

        let chunk = vec![0xAAu8; 60 * 1024];
        let total = 4 * 1024 * 1024;
        let (mut sent, mut received) = (0usize, 0usize);
        while sent < total {
            match net.send(cli, &chunk[..chunk.len().min(total - sent)]) {
                Ok(n) => sent += n,
                Err(e) => assert_eq!(e, errno::EAGAIN),
            }
            // Drain whatever has arrived so the sender can make progress.
            while let Ok((data, _)) = net.recvfrom(conn, 65536) {
                received += data.len();
            }
            if sent == received {
                continue;
            }
            wait(&net, conn, POLLIN);
        }
        while received < total {
            received += recv(&net, conn).len();
        }
        assert_eq!(received, total);
    }

    /// The guest driver polls every socket for `POLLIN|POLLOUT` as soon as
    /// it is created and caches the answer: a fresh, unconnected socket
    /// must not claim to be readable, or the guest spins on it.
    #[test]
    fn fresh_socket_is_not_readable() {
        let net = open_net();
        let s = tcp(&net);
        let (_, rev) = net.poll(&[(s, POLLIN | POLLOUT)], 0);
        assert_eq!(rev[0] & POLLIN, 0, "unconnected socket reported readable");
    }

    /// The guest driver polls every socket for `POLLIN|POLLOUT`, listeners
    /// included: an idle listener must report neither, not an error.
    #[test]
    fn poll_listener_for_pollout() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 1).unwrap();
        assert_eq!(net.poll(&[(srv, POLLIN | POLLOUT)], 0), (0, vec![0]));
        let cli = tcp(&net);
        net.connect(cli, net.local_addr(srv).unwrap()).unwrap();
        assert_eq!(
            wait(&net, srv, POLLIN | POLLOUT) & (POLLIN | POLLOUT),
            POLLIN
        );
    }

    #[test]
    fn udp_loopback_sendto_recvfrom() {
        let net = open_net();
        let a = udp(&net);
        net.bind(a, loopback(0)).unwrap();
        let a_addr = net.local_addr(a).unwrap();
        assert_eq!(net.getsockopt(a, SOL_SOCKET, SO_TYPE).unwrap(), SOCK_DGRAM);
        assert_eq!(net.listen(a, 1), Err(errno::EOPNOTSUPP));
        assert_eq!(net.recvfrom(a, 64), Err(errno::EAGAIN));

        let b = udp(&net);
        net.bind(b, loopback(0)).unwrap();
        assert_eq!(net.sendto(b, b"dgram", a_addr).unwrap(), 5);
        assert_ne!(wait(&net, a, POLLIN) & POLLIN, 0);
        let (data, from) = net.recvfrom(a, 64).unwrap();
        assert_eq!(data, b"dgram");
        assert_eq!(from, Some(net.local_addr(b).unwrap()));

        // Connected UDP can use plain send.
        net.connect(b, a_addr).unwrap();
        assert_eq!(net.send(b, b"x").unwrap(), 1);
        assert_eq!(recv(&net, a), b"x");
    }

    #[test]
    fn ipv6_loopback_socket() {
        let net = open_net();
        let addr = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 0);
        let Ok(s) = net.socket(AF_INET6, SOCK_STREAM, 0) else {
            eprintln!("SKIP: no IPv6");
            return;
        };
        if net.bind(s, addr).is_err() {
            eprintln!("SKIP: no IPv6 loopback");
            return;
        }
        assert_eq!(net.local_addr(s).unwrap().ip(), addr.ip());
        assert!(net.getsockopt(s, IPPROTO_IPV6, IPV6_V6ONLY).is_ok());
    }

    #[test]
    fn connection_refused_is_linux_errno() {
        let net = open_net();
        // Bound but not listening: a SYN is answered with RST.
        let closed = tcp(&net);
        net.bind(closed, loopback(0)).unwrap();
        let c = tcp(&net);
        assert_eq!(
            net.connect(c, net.local_addr(closed).unwrap()),
            Err(errno::ECONNREFUSED)
        );
        // The fd survives a failed connect so the guest can close it.
        net.close(c).unwrap();
    }

    #[test]
    fn unsupported_family_type_and_protocol() {
        let net = open_net();
        assert_eq!(net.socket(99, SOCK_STREAM, 0), Err(errno::EAFNOSUPPORT));
        assert_eq!(net.socket(AF_INET, 3, 0), Err(errno::EPROTONOSUPPORT));
        // Explicit matching protocols are accepted, mismatched ones rejected.
        net.close(net.socket(AF_INET, SOCK_STREAM, 6).unwrap())
            .unwrap();
        net.close(net.socket(AF_INET, SOCK_DGRAM, 17).unwrap())
            .unwrap();
        assert!(net.socket(AF_INET, SOCK_STREAM, 17).is_err());
        assert_eq!(net.socket(AF_INET, SOCK_STREAM, -1), Err(errno::EINVAL));
        // SOCK_NONBLOCK/SOCK_CLOEXEC bits are masked off.
        let s = net
            .socket(AF_INET, SOCK_STREAM | 0x800 | 0x80000, 0)
            .unwrap();
        net.close(s).unwrap();
    }

    #[test]
    fn unknown_fd_is_ebadf_and_pollnval() {
        let net = open_net();
        assert_eq!(net.send(42, b"x"), Err(errno::EBADF));
        assert_eq!(net.listen(42, 1), Err(errno::EBADF));
        let (ready, rev) = net.poll(&[(42, POLLIN)], 0);
        assert_eq!(ready, 1);
        assert_eq!(rev, vec![POLLNVAL]);
        assert_eq!(net.poll(&[], 0), (0, vec![]));
        // An invalid entry is ready now, even next to an idle valid one
        // and an indefinite timeout.
        let idle = udp(&net);
        let t = std::time::Instant::now();
        let (ready, rev) = net.poll(&[(idle, POLLIN), (42, POLLIN)], -1);
        assert!(t.elapsed() < Duration::from_secs(1));
        assert_eq!((ready, rev), (1, vec![0, POLLNVAL]));
    }

    #[test]
    fn policy_denies_connect_and_sendto() {
        // An allow-list always blocks loopback.
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let net = Net::new(Some(NetworkPolicy::AllowList(al)), None);
        let t = tcp(&net);
        assert_eq!(net.connect(t, loopback(80)), Err(errno::EACCES));
        let u = udp(&net);
        assert_eq!(net.sendto(u, b"x", loopback(80)), Err(errno::EACCES));
    }

    #[test]
    fn listen_ports_gate_bind() {
        let net = Net::new(
            Some(NetworkPolicy::AllowAll),
            Some(ListenPorts::from_ports([8080])),
        );
        let s = tcp(&net);
        assert_eq!(net.bind(s, loopback(19_999)), Err(errno::EACCES));
        // Ephemeral binds are always allowed.
        net.bind(s, loopback(0)).unwrap();
    }

    #[test]
    fn wire_addr_packing() {
        let mut buf = Vec::new();
        pack_addr(&mut buf, &loopback(0x1234));
        assert_eq!(buf, [2, 0, 0, 0, 0x34, 0x12, 4, 127, 0, 0, 1]);
        assert_eq!(parse_addr(AF_INET, "127.0.0.1", 80), Some(loopback(80)));
        assert_eq!(
            parse_addr(AF_INET6, "0:0:0:0:0:0:0:1", 80).map(|a| a.ip()),
            Some(IpAddr::V6(Ipv6Addr::LOCALHOST))
        );
        assert_eq!(parse_addr(AF_INET, "nope", 80), None);
        assert_eq!(parse_addr(AF_INET, "127.0.0.1", 70000), None);
    }
}
