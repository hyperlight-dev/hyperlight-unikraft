// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Host networking — Tokio I/O with a [`rustix`] POSIX compatibility layer.
//!
//! The guest driver (`hostsock.c`) speaks the POSIX socket API, one host
//! call per syscall. Async driving uses Tokio for ordinary TCP/UDP transfers
//! and readiness. rustix handles socket setup, options, exact POSIX polling
//! and immediate setup I/O on Linux, Windows and macOS. Policy —
//! [`NetworkPolicy`] for outbound, [`ListenPorts`] for bind — is checked
//! before the corresponding syscall that transmits: TCP connect, and
//! every UDP send.  UDP connect only selects a peer and source address.
//!
//! A host function call is a synchronous VM exit: the guest is paused
//! until it returns, so no host call may block on a peer that lives in
//! the same guest.  Host sockets are therefore non-blocking: the guest
//! driver checks readiness with `net_poll(timeout=0)` first, and a call
//! that would block anyway returns `-EAGAIN`, which the guest's socket
//! layer already treats as "yield and retry". Connect returns EINPROGRESS
//! during the handshake. The kernel's readiness waiter uses an ordinary
//! typed async host call. Network operations use typed replies too; policy
//! checks still complete synchronously before the operation.
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
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::num::NonZeroU32;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use std::task::{Context, Poll, ready};
use std::time::Duration;

use hyperlight_host::func::Registerable;
use rustix::event::{PollFd, PollFlags, Timespec};
use rustix::fd::{AsFd, OwnedFd};
use rustix::io::Errno;
use rustix::net::{
    self, AddressFamily, Protocol, RecvFlags, SendFlags, Shutdown, SocketFlags, SocketType, sockopt,
};

use crate::HOST_CALL_MAX;
use crate::errno::{self, from_rustix};
use crate::net_policy::{ListenPorts, NetworkPolicy};

const MAX_SOCKETS: usize = 1024;

type Accepted = (OwnedFd, Option<SocketAddr>);

enum AsyncSocketIo {
    Stream(tokio::net::TcpStream),
    Datagram(tokio::net::UdpSocket),
    Listener {
        io: tokio::net::TcpListener,
        accepted: Option<std::io::Result<Accepted>>,
    },
}

/// Keep the syscall handle separate from the lazily registered Tokio handle.
/// Duplicating once lets synchronous callers keep using the same socket before
/// and after async driving, without unsafe borrowed ownership conversions.
struct AsyncSocket {
    runtime: tokio::runtime::Id,
    io: AsyncSocketIo,
}

impl AsyncSocket {
    fn new(socket: &OwnedFd, meta: SockMeta, runtime: tokio::runtime::Id) -> std::io::Result<Self> {
        let owned = socket.try_clone()?;
        let io = if meta.listening {
            AsyncSocketIo::Listener {
                io: tokio::net::TcpListener::from_std(owned.into())?,
                accepted: None,
            }
        } else if meta.udp {
            AsyncSocketIo::Datagram(tokio::net::UdpSocket::from_std(owned.into())?)
        } else {
            AsyncSocketIo::Stream(tokio::net::TcpStream::from_std(owned.into())?)
        };
        Ok(Self { runtime, io })
    }

    fn has_accepted(&self) -> bool {
        match &self.io {
            AsyncSocketIo::Listener { accepted, .. } => accepted.is_some(),
            _ => false,
        }
    }

    fn take_accepted(&mut self) -> Option<std::io::Result<Accepted>> {
        match &mut self.io {
            AsyncSocketIo::Listener { accepted, .. } => accepted.take(),
            _ => None,
        }
    }

    fn poll_ready(&mut self, cx: &mut Context<'_>, events: PollFlags) -> Poll<std::io::Result<()>> {
        let interest = if events == PollFlags::IN {
            tokio::io::Interest::READABLE
        } else {
            tokio::io::Interest::WRITABLE
        };
        // Guest syscalls can drain readiness outside Tokio. Only clear its
        // cached readiness after checking the actual OS state for this interest.
        fn check(socket: &impl AsFd, events: PollFlags) -> std::io::Result<()> {
            let mut fds = [PollFd::new(socket, events)];
            rustix::event::poll(
                &mut fds,
                Some(&Timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                }),
            )
            .map_err(std::io::Error::from)?;
            if fds[0].revents().is_empty() {
                Err(std::io::ErrorKind::WouldBlock.into())
            } else {
                Ok(())
            }
        }
        loop {
            let result = match &mut self.io {
                AsyncSocketIo::Stream(io) => {
                    if events == PollFlags::IN {
                        ready!(io.poll_read_ready(cx))?;
                    } else {
                        ready!(io.poll_write_ready(cx))?;
                    }
                    io.try_io(interest, || check(io, events))
                }
                AsyncSocketIo::Datagram(io) => {
                    if events == PollFlags::IN {
                        ready!(io.poll_recv_ready(cx))?;
                    } else {
                        ready!(io.poll_send_ready(cx))?;
                    }
                    io.try_io(interest, || check(io, events))
                }
                AsyncSocketIo::Listener { io, accepted } => {
                    if events != PollFlags::IN {
                        // The guest polls listeners for both directions, but a
                        // listening socket never becomes writable.
                        return Poll::Pending;
                    }
                    if accepted.is_none() {
                        let result = ready!(io.poll_accept(cx))
                            .and_then(|(stream, peer)| Ok((stream.into_std()?.into(), Some(peer))));
                        // Guest accept consumes errors as well as sockets.
                        *accepted = Some(result);
                    }
                    Ok(())
                }
            };
            match result {
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                result => return Poll::Ready(result),
            }
        }
    }
}

struct NetworkWait(Weak<Net>);

impl Drop for NetworkWait {
    fn drop(&mut self) {
        if let Some(net) = self.0.upgrade() {
            net.async_waiting.store(false, Ordering::Relaxed);
        }
    }
}

/// What `net_recvfrom`'s reply carries besides the data: its length (4
/// bytes) and the packed source address (at most 7 + 16).  The kernel
/// keeps the same room (`HOSTSOCK_RECV_HDR` in lib/hostsock).
const RECVFROM_HDR: usize = 4 + 7 + 16;

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
/// sockets out of any child the host spawns; Windows has no such flag,
/// and macOS none at creation (see [`apple_socket_setup`]).
fn socket_flags() -> SocketFlags {
    #[cfg(any(windows, target_vendor = "apple"))]
    {
        SocketFlags::empty()
    }
    #[cfg(not(any(windows, target_vendor = "apple")))]
    {
        SocketFlags::CLOEXEC
    }
}

/// macOS has neither `SOCK_CLOEXEC` nor `MSG_NOSIGNAL`: set close-on-exec
/// and no-SIGPIPE on each new or accepted socket instead.
#[cfg(target_vendor = "apple")]
fn apple_socket_setup(sock: &OwnedFd) -> Res<()> {
    rustix::io::fcntl_setfd(sock, rustix::io::FdFlags::CLOEXEC).map_err(from_rustix)?;
    sockopt::set_socket_nosigpipe(sock, true).map_err(from_rustix)
}

/// The DNS message in a send to port 53: the datagram itself on UDP; on
/// TCP the message behind its two-byte length, provided the send holds
/// exactly one whole message.  Anything else is no message, and the
/// policy refuses it.
fn dns_message(data: &[u8], udp: bool) -> &[u8] {
    if udp {
        return data;
    }
    match data {
        [hi, lo, rest @ ..] if rest.len() == usize::from(u16::from_be_bytes([*hi, *lo])) => rest,
        _ => &[],
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

/// Tokio may report WouldBlock without a syscall when its reactor has not
/// observed readiness yet. The guest can already have seen OS readiness in
/// net_poll, so check the socket directly rather than make it spin until the
/// synchronous VM entry ends. Other Tokio errors must reach the guest unchanged.
fn immediate_io<T>(result: Option<std::io::Result<T>>, syscall: impl FnOnce() -> Res<T>) -> Res<T> {
    match result {
        Some(Ok(value)) => Ok(value),
        Some(Err(error)) if error.kind() != std::io::ErrorKind::WouldBlock => {
            Err(errno::from_io(&error))
        }
        _ => syscall(),
    }
}

// ── SocketTable ─────────────────────────────────────────────────────

/// What the policy needs to know about a socket beyond its address: a
/// UDP socket aimed at port 53 carries DNS questions, which are checked
/// by name where they are sent.
#[derive(Clone, Copy, Debug)]
struct SockMeta {
    udp: bool,
    /// Where a `send` on it goes, once the guest connected it.
    peer: Option<SocketAddr>,
    /// `listen` succeeded on it: `SO_ACCEPTCONN`, which macOS has but
    /// rustix does not expose there.
    listening: bool,
    /// What a datagram socket is made again from on macOS, which can
    /// neither dissolve its association nor connect it a second time (see
    /// [`SocketTable::renew`]): its family and protocol (`None` for an
    /// accepted socket), and the address the guest bound it to.
    origin: Option<(AddressFamily, Option<Protocol>)>,
    bound: Option<SocketAddr>,
}

struct Socket {
    handle: OwnedFd,
    meta: SockMeta,
    registration: Option<AsyncSocket>,
    /// What the guest is parked on, as learnt from its `net_poll` answers:
    /// a socket it asked about and was told "not readable" / "not
    /// writable".  The inter-step wait watches exactly these, for exactly
    /// that.  Cleared when the condition is delivered (by a poll answer or
    /// by the wait itself); a socket nobody in the guest is waiting on is
    /// not watched, or unread data on it would make every wait return at
    /// once and spin the step loop.
    interests: PollFlags,
}

impl Socket {
    fn register(&mut self, runtime: tokio::runtime::Id) -> std::io::Result<&mut AsyncSocket> {
        if self
            .registration
            .as_ref()
            .is_none_or(|s| s.runtime != runtime)
        {
            let mut registered = AsyncSocket::new(&self.handle, self.meta, runtime)?;
            if let AsyncSocketIo::Listener { accepted, .. } = &mut registered.io {
                *accepted = self
                    .registration
                    .as_mut()
                    .and_then(AsyncSocket::take_accepted);
            }
            self.registration = Some(registered);
        }
        Ok(self.registration.as_mut().unwrap())
    }

    fn has_accepted(&self) -> bool {
        self.registration
            .as_ref()
            .is_some_and(AsyncSocket::has_accepted)
    }
}

/// Guest fd → host socket. Dropping an entry releases all its socket state.
struct SocketTable {
    sockets: HashMap<i32, Socket>,
    next_fd: i32,
}

impl SocketTable {
    fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            next_fd: 3, // skip stdin/stdout/stderr
        }
    }

    /// Record the outcome of a send on `fd` for the inter-step wait.
    fn note_send(&mut self, fd: i32, result: &Res<usize>) -> Res<()> {
        let socket = self.entry_mut(fd)?;
        match result {
            Err(e) if *e == errno::EAGAIN => {
                socket.interests.insert(PollFlags::OUT);
            }
            Ok(_) => {
                socket.interests.remove(PollFlags::OUT);
            }
            Err(_) => {}
        }
        Ok(())
    }

    fn insert(
        &mut self,
        sock: OwnedFd,
        udp: bool,
        origin: Option<(AddressFamily, Option<Protocol>)>,
    ) -> Res<i32> {
        if self.sockets.len() >= MAX_SOCKETS {
            return Err(errno::EMFILE);
        }
        let fd = self.next_fd;
        self.next_fd = self.next_fd.wrapping_add(1);
        self.sockets.insert(
            fd,
            Socket {
                handle: sock,
                meta: SockMeta {
                    udp,
                    peer: None,
                    listening: false,
                    origin,
                    bound: None,
                },
                registration: None,
                interests: PollFlags::empty(),
            },
        );
        Ok(fd)
    }

    fn meta(&self, fd: i32) -> Res<SockMeta> {
        self.sockets.get(&fd).map(|s| s.meta).ok_or(errno::EBADF)
    }

    fn entry_mut(&mut self, fd: i32) -> Res<&mut Socket> {
        self.sockets.get_mut(&fd).ok_or(errno::EBADF)
    }

    /// A fresh datagram socket like `fd`'s, with the options the guest can
    /// set: what [`renew`](Self::renew) and [`reconnect`](Self::reconnect)
    /// put in its place.
    #[cfg(target_vendor = "apple")]
    fn fresh_dgram(&self, fd: i32) -> Res<(OwnedFd, AddressFamily)> {
        let Some((af, proto)) = self.meta(fd)?.origin else {
            return Err(errno::EOPNOTSUPP);
        };
        let sock =
            net::socket_with(af, SocketType::DGRAM, socket_flags(), proto).map_err(from_rustix)?;
        apple_socket_setup(&sock)?;
        rustix::io::ioctl_fionbio(&sock, true).map_err(from_rustix)?;
        copy_dgram_options(self.get(fd)?, &sock, af);
        Ok((sock, af))
    }

    /// Dissolve a datagram socket's association on macOS, which refuses
    /// `connect(AF_UNSPEC)`: a fresh socket takes its place, bound where the
    /// guest bound the old one (as on Linux, a port the kernel picked is
    /// dropped).  Datagrams still queued are lost.
    #[cfg(target_vendor = "apple")]
    fn renew(&mut self, fd: i32) -> Res<()> {
        let bound = self.meta(fd)?.bound;
        let (sock, _) = self.fresh_dgram(fd)?;
        // Replacing the old socket closes it, which frees the port for the
        // new one: binding first would fail with EADDRINUSE.
        let entry = self.entry_mut(fd)?;
        entry.registration = None;
        entry.handle = sock;
        entry.meta.peer = None;
        if let Some(addr) = bound {
            net::bind(self.get(fd)?, &addr).map_err(from_rustix)?;
        }
        Ok(())
    }

    /// Connect a connected datagram socket to a new peer on macOS, which
    /// answers a second `connect` with `EISCONN`.  A fresh socket on the
    /// same local address is connected while the old one still holds it
    /// (both share it through SO_REUSEPORT meanwhile), and replaces it only
    /// if that worked.  As on Linux, the source address and port stay, and
    /// a connect that fails leaves the old association.  Datagrams still
    /// queued on the old socket are lost.
    #[cfg(target_vendor = "apple")]
    fn reconnect(&mut self, fd: i32, addr: SocketAddr) -> Res<()> {
        // Already this peer: nothing to do (and a second socket with the
        // same four-tuple would be refused).
        if self.meta(fd)?.peer == Some(addr) {
            return Ok(());
        }
        let local = net::getsockname(self.get(fd)?).map_err(from_rustix)?;
        let local = SocketAddr::try_from(local).map_err(|_| errno::EAFNOSUPPORT)?;
        let (sock, _) = self.fresh_dgram(fd)?;
        let old = self.get(fd)?;
        let reuse = sockopt::socket_reuseport(old).unwrap_or(false);
        sockopt::set_socket_reuseport(old, true).map_err(from_rustix)?;
        let r = sockopt::set_socket_reuseport(&sock, true)
            .and_then(|()| net::bind(&sock, &local))
            .and_then(|()| net::connect(&sock, &addr))
            .map_err(from_rustix);
        if let Err(e) = r {
            let _ = sockopt::set_socket_reuseport(old, reuse);
            return Err(e);
        }
        let _ = sockopt::set_socket_reuseport(&sock, reuse);
        // Replacing the old socket closes it.
        let entry = self.entry_mut(fd)?;
        entry.registration = None;
        entry.handle = sock;
        entry.meta.peer = Some(addr);
        Ok(())
    }

    fn get(&self, fd: i32) -> Res<&OwnedFd> {
        self.sockets.get(&fd).map(|s| &s.handle).ok_or(errno::EBADF)
    }

    fn remove(&mut self, fd: i32) -> Res<Socket> {
        self.sockets.remove(&fd).ok_or(errno::EBADF)
    }
}

/// Give `new` the options of `old` that the guest can set on a datagram
/// socket (see [`Net::setsockopt`]), before [`SocketTable::renew`] or
/// [`SocketTable::reconnect`] binds it: SO_REUSEADDR/SO_REUSEPORT decide
/// whether that bind succeeds.  An option the socket's family lacks fails
/// to read and is skipped.
#[cfg(target_vendor = "apple")]
fn copy_dgram_options(old: &OwnedFd, new: &OwnedFd, af: AddressFamily) {
    if let Ok(v) = sockopt::socket_reuseaddr(old) {
        let _ = sockopt::set_socket_reuseaddr(new, v);
    }
    if let Ok(v) = sockopt::socket_reuseport(old) {
        let _ = sockopt::set_socket_reuseport(new, v);
    }
    if let Ok(v) = sockopt::socket_broadcast(old) {
        let _ = sockopt::set_socket_broadcast(new, v);
    }
    if let Ok(v) = sockopt::socket_send_buffer_size(old) {
        let _ = sockopt::set_socket_send_buffer_size(new, v);
    }
    if let Ok(v) = sockopt::socket_recv_buffer_size(old) {
        let _ = sockopt::set_socket_recv_buffer_size(new, v);
    }
    if af == AddressFamily::INET6 {
        if let Ok(v) = sockopt::ipv6_v6only(old) {
            let _ = sockopt::set_ipv6_v6only(new, v);
        }
        if let Ok(v) = sockopt::ipv6_unicast_hops(old) {
            let _ = sockopt::set_ipv6_unicast_hops(new, Some(v));
        }
        if let Ok(v) = sockopt::ipv6_multicast_loop(old) {
            let _ = sockopt::set_ipv6_multicast_loop(new, v);
        }
    } else {
        if let Ok(v) = sockopt::ip_ttl(old) {
            let _ = sockopt::set_ip_ttl(new, v);
        }
        if let Ok(v) = sockopt::ip_tos(old) {
            let _ = sockopt::set_ip_tos(new, v);
        }
        if let Ok(v) = sockopt::ip_multicast_ttl(old) {
            let _ = sockopt::set_ip_multicast_ttl(new, v);
        }
        if let Ok(v) = sockopt::ip_multicast_loop(old) {
            let _ = sockopt::set_ip_multicast_loop(new, v);
        }
    }
}

// ── Net ─────────────────────────────────────────────────────────────

pub(crate) struct Net {
    /// The `net_*` closures share this state through an `Arc`, which
    /// needs it to be `Sync` (Hyperlight wants each closure `Send +
    /// 'static`).  Hence a mutex, even though a sandbox has one vCPU
    /// and never makes two host calls at once: the lock is uncontended
    /// and is held across the (non-blocking) syscalls without harm.
    table: Mutex<SocketTable>,
    /// `None` when the guest has no network: `socket()` is refused, so
    /// nothing else can happen.  The `net_*` functions exist all the same,
    /// since a restore must offer every host function the snapshot's
    /// guest can call.
    policy: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    async_waiting: AtomicBool,
    waiter: futures::task::AtomicWaker,
}

impl Drop for Net {
    fn drop(&mut self) {
        self.waiter.wake();
    }
}

impl Net {
    pub(crate) fn new(policy: NetworkPolicy, listen_ports: Option<ListenPorts>) -> Self {
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
            policy: Some(policy),
            listen_ports,
            async_waiting: AtomicBool::new(false),
            waiter: futures::task::AtomicWaker::new(),
        }
    }

    /// The network of a guest with no policy: every `socket()` is refused
    /// with `EACCES`, as a policy refuses a destination.
    pub(crate) fn disabled() -> Self {
        Self {
            table: Mutex::new(SocketTable::new()),
            policy: None,
            listen_ports: None,
            async_waiting: AtomicBool::new(false),
            waiter: futures::task::AtomicWaker::new(),
        }
    }

    fn table(&self) -> MutexGuard<'_, SocketTable> {
        self.table.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn socket_io<'a>(&self, table: &'a mut SocketTable, fd: i32) -> Res<Option<&'a AsyncSocketIo>> {
        // Boot, submit and restore can enter the guest outside a runtime.
        // They do immediate I/O only; queued futures wait for the next step.
        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            return Ok(None);
        };
        let entry = table.entry_mut(fd)?;
        let meta = entry.meta;
        let protocol = if meta.udp {
            net::ipproto::UDP
        } else {
            net::ipproto::TCP
        };
        let standard = meta
            .origin
            .is_none_or(|(_, p)| p.is_none_or(|p| p == protocol));
        // Keep fresh/unbound sockets and non-TCP/UDP protocols on the syscall
        // path. In particular, reactor registration must not precede UDP's
        // implicit bind on its first send.
        let established =
            meta.peer.is_some() || (meta.udp && meta.bound.is_some()) || meta.origin.is_none();
        if !standard || meta.listening || (!established && entry.registration.is_none()) {
            return Ok(None);
        }
        let socket = entry.register(runtime.id()).map_err(|error| {
            tracing::warn!(%error, fd, "cannot register socket for asynchronous I/O");
            errno::from_io(&error)
        })?;
        Ok(Some(&socket.io))
    }

    /// Outbound policy (TCP connect, UDP send/sendto).
    fn allow_outbound(&self, addr: &SocketAddr, udp: bool) -> Res<()> {
        let Some(policy) = &self.policy else {
            return Err(errno::EACCES);
        };
        let allowed = policy.allows(addr, udp);
        if allowed { Ok(()) } else { Err(errno::EACCES) }
    }

    fn authorized_reply(
        self: &Arc<Self>,
        addr: SocketAddr,
        udp: bool,
        _inbox: &Arc<Mutex<crate::async_calls::Inbox>>,
        operation: impl FnOnce(&Self) -> i32 + Send + 'static,
    ) -> Vec<u8> {
        let Some(policy) = &self.policy else {
            return int_reply(-errno::EACCES);
        };
        int_reply(if policy.allows(&addr, udp) {
            operation(self)
        } else {
            -errno::EACCES
        })
    }

    fn connect_reply(
        self: &Arc<Self>,
        fd: i32,
        addr: SocketAddr,
        inbox: &Arc<Mutex<crate::async_calls::Inbox>>,
    ) -> Vec<u8> {
        let meta = match self.table().meta(fd) {
            Ok(meta) => meta,
            Err(e) => return int_reply(-e),
        };
        if !meta.udp {
            return self.authorized_reply(addr, false, inbox, move |net| {
                ret(net.connect_allowed(fd, addr), |()| 0)
            });
        }
        int_reply(ret(self.connect_allowed(fd, addr), |()| 0))
    }

    fn send_reply(
        self: &Arc<Self>,
        fd: i32,
        data: Vec<u8>,
        inbox: &Arc<Mutex<crate::async_calls::Inbox>>,
    ) -> Vec<u8> {
        let meta = match self.table().meta(fd) {
            Ok(meta) => meta,
            Err(e) => return int_reply(-e),
        };
        if meta.udp
            && let Some(peer) = meta.peer
        {
            return self.authorized_reply(peer, true, inbox, move |net| {
                ret(net.send_allowed(fd, &data, meta), |sent| sent as i32)
            });
        }
        int_reply(ret(self.send_allowed(fd, &data, meta), |sent| sent as i32))
    }

    fn sendto_reply(
        self: &Arc<Self>,
        fd: i32,
        data: Vec<u8>,
        addr: SocketAddr,
        inbox: &Arc<Mutex<crate::async_calls::Inbox>>,
    ) -> Vec<u8> {
        let udp = match self.table().meta(fd) {
            Ok(meta) => meta.udp,
            Err(e) => return int_reply(-e),
        };
        if !udp {
            return self.send_reply(fd, data, inbox);
        }
        self.authorized_reply(addr, true, inbox, move |net| {
            ret(net.sendto_allowed(fd, &data, addr), |sent| sent as i32)
        })
    }

    /// Inbound policy (bind).  Ephemeral binds (port 0) are always
    /// allowed; any other port must be listed, whether the guest means to
    /// listen on it or to connect from it, so a guest given no ports can
    /// serve only on a port the host picks, as `--port` and
    /// `SandboxBuilder::listen_ports` promise.
    fn allow_bind(&self, addr: &SocketAddr) -> Res<()> {
        if addr.port() == 0 {
            return Ok(());
        }
        match &self.listen_ports {
            Some(lp) if lp.allows(addr.port()) => Ok(()),
            _ => Err(errno::EACCES),
        }
    }

    fn socket(&self, family: i32, ty: i32, proto: i32) -> Res<i32> {
        if self.policy.is_none() {
            return Err(errno::EACCES);
        }
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
        #[cfg(target_vendor = "apple")]
        apple_socket_setup(&sock)?;
        rustix::io::ioctl_fionbio(&sock, true).map_err(from_rustix)?;
        // Linux AF_INET6 sockets are dual-stack by default; Windows's are
        // v6-only.  Give the guest the Linux default (it can still opt in).
        #[cfg(windows)]
        if af == AddressFamily::INET6 {
            let _ = sockopt::set_ipv6_v6only(&sock, false);
        }
        self.table()
            .insert(sock, ty == SocketType::DGRAM, Some((af, proto)))
    }

    fn bind(&self, fd: i32, addr: SocketAddr) -> Res<()> {
        self.allow_bind(&addr)?;
        let mut tbl = self.table();
        net::bind(tbl.get(fd)?, &addr).map_err(from_rustix)?;
        tbl.entry_mut(fd)?.meta.bound = Some(addr);
        Ok(())
    }

    fn listen(&self, fd: i32, backlog: i32) -> Res<()> {
        let mut tbl = self.table();
        net::listen(tbl.get(fd)?, backlog).map_err(from_rustix)?;
        let socket = tbl.entry_mut(fd)?;
        if !socket.meta.listening {
            socket.registration = None;
            socket.meta.listening = true;
            self.waiter.wake();
        }
        Ok(())
    }

    fn accept(&self, fd: i32) -> Res<(i32, Option<SocketAddr>)> {
        let mut tbl = self.table();
        let buffered = tbl
            .entry_mut(fd)?
            .registration
            .as_mut()
            .and_then(AsyncSocket::take_accepted);
        let (conn, peer) = match buffered {
            Some(accepted) => accepted.map_err(|e| errno::from_io(&e))?,
            None => {
                let (conn, peer) =
                    net::acceptfrom_with(tbl.get(fd)?, socket_flags()).map_err(from_rustix)?;
                (conn, peer.and_then(|a| SocketAddr::try_from(a).ok()))
            }
        };
        #[cfg(target_vendor = "apple")]
        apple_socket_setup(&conn)?;
        rustix::io::ioctl_fionbio(&conn, true).map_err(from_rustix)?;
        // No peer is recorded: only a socket the guest itself aimed at port
        // 53 carries its DNS questions, and a remote client may pick any
        // source port, 53 included.
        Ok((tbl.insert(conn, false, None)?, peer))
    }

    /// Starts the handshake and returns at once: `EINPROGRESS` while it
    /// is under way, as a non-blocking connect(2) does.  The guest's
    /// socket layer then parks its thread on writability, which the
    /// inter-step wait watches (see [`Self::poll`]), and reads the outcome
    /// through `SO_ERROR`.  Waiting here instead held the vCPU and the
    /// embedder's thread for as long as the peer took: up to the OS's SYN
    /// retry timeout, about two minutes on Linux.
    fn connect(&self, fd: i32, addr: SocketAddr) -> Res<()> {
        let meta = self.table().meta(fd)?;
        // UDP connect sends nothing.  libc also uses it to discover address
        // families and source addresses; check its destination at each send.
        if !meta.udp {
            self.allow_outbound(&addr, false)?;
        }
        self.connect_allowed(fd, addr)
    }

    fn connect_allowed(&self, fd: i32, addr: SocketAddr) -> Res<()> {
        #[cfg(target_vendor = "apple")]
        let meta = self.table().meta(fd)?;
        let mut tbl = self.table();
        {
            let sock = tbl.get(fd)?;
            // Linux answers a second connect on a connected stream socket
            // with EISCONN; say so here, since nothing calls connect(2)
            // again on the host socket to make the OS observe the
            // established state.
            if sockopt::socket_type(sock) == Ok(SocketType::STREAM)
                && net::getpeername(sock).is_ok_and(|p| p.is_some())
            {
                return Err(errno::EISCONN);
            }
        }
        // Linux lets a datagram socket be connected again, to a new peer;
        // macOS answers EISCONN, so a fresh socket takes the new peer.
        #[cfg(target_vendor = "apple")]
        if meta.udp && meta.peer.is_some() {
            let result = tbl.reconnect(fd, addr);
            self.waiter.wake();
            return result;
        }
        let r = match net::connect(tbl.get(fd)?, &addr) {
            Ok(()) => Ok(()),
            Err(Errno::INPROGRESS) | Err(Errno::WOULDBLOCK) => Err(errno::EINPROGRESS),
            Err(e) => Err(from_rustix(e)),
        };
        // Later sends on this socket go to `addr`: for a UDP socket aimed
        // at port 53 they are DNS questions, checked by name in `send`.  A
        // connect that failed leaves the socket's association as it was.
        if r.is_ok() || r == Err(errno::EINPROGRESS) {
            tbl.entry_mut(fd)?.meta.peer = Some(addr);
        }
        r
    }

    /// `connect(2)` with `AF_UNSPEC`: dissolve a datagram socket's
    /// association, so a later `send` needs an address again and the next
    /// connect picks a source address of its destination's family.  glibc's
    /// `getaddrinfo` relies on it to probe every candidate through one IPv6
    /// socket.  A stream socket has no association to dissolve this way.
    fn disconnect(&self, fd: i32) -> Res<()> {
        let mut tbl = self.table();
        {
            let sock = tbl.get(fd)?;
            if sockopt::socket_type(sock) != Ok(SocketType::DGRAM) {
                return Err(errno::EOPNOTSUPP);
            }
            #[cfg(not(target_vendor = "apple"))]
            net::connect_unspec(sock).map_err(from_rustix)?;
        }
        #[cfg(target_vendor = "apple")]
        {
            let result = tbl.renew(fd);
            self.waiter.wake();
            result?;
        }
        tbl.entry_mut(fd)?.meta.peer = None;
        Ok(())
    }

    fn send(&self, fd: i32, data: &[u8]) -> Res<usize> {
        // A DNS question on a socket connected to port 53: the name is
        // checked here, where it still is one (see `NetworkPolicy::allows_query`),
        // with the table unlocked, as the check may ask the embedder's filter.
        let meta = self.table().meta(fd)?;
        if meta.udp
            && let Some(peer) = meta.peer
        {
            self.allow_outbound(&peer, true)?;
        }
        self.send_allowed(fd, data, meta)
    }

    fn send_allowed(&self, fd: i32, data: &[u8], meta: SockMeta) -> Res<usize> {
        if meta.peer.is_some_and(|p| p.port() == 53)
            && !self
                .policy
                .as_ref()
                .is_some_and(|p| p.allows_query(dns_message(data, meta.udp)))
        {
            return Err(errno::EACCES);
        }
        let mut tbl = self.table();
        // Checked for the peer it had then; one connected since is not.
        if tbl.meta(fd)?.peer != meta.peer {
            return Err(errno::EACCES);
        }
        let result = match self.socket_io(&mut tbl, fd)? {
            Some(AsyncSocketIo::Stream(io)) => Some(io.try_write(data)),
            Some(AsyncSocketIo::Datagram(io)) => Some(io.try_send(data)),
            _ => None,
        };
        let r = immediate_io(result, || {
            net::send(tbl.get(fd)?, data, send_flags()).map_err(from_rustix)
        });
        tbl.note_send(fd, &r)?;
        drop(tbl);
        self.waiter.wake();
        r
    }

    fn sendto(&self, fd: i32, data: &[u8], addr: SocketAddr) -> Res<usize> {
        let udp = self.table().meta(fd)?.udp;
        // A stream socket sends to its peer whatever address sendto names
        // (Linux and Windows ignore it), so checking `addr` would check the
        // wrong destination: a guest connected to a resolver on port 53
        // could name an allowed address and send anything there.  The peer
        // was checked at the TCP connect; `send` checks a DNS question to it.
        if !udp {
            return self.send(fd, data);
        }
        self.allow_outbound(&addr, udp)?;
        self.sendto_allowed(fd, data, addr)
    }

    fn sendto_allowed(&self, fd: i32, data: &[u8], addr: SocketAddr) -> Res<usize> {
        let udp = self.table().meta(fd)?.udp;
        // A DNS question sent unconnected: the same name check as `send`.
        if addr.port() == 53
            && !self
                .policy
                .as_ref()
                .is_some_and(|p| p.allows_query(dns_message(data, udp)))
        {
            return Err(errno::EACCES);
        }
        // macOS refuses sendto() on a connected socket (EISCONN) even to its
        // own peer, which Linux accepts.
        #[cfg(target_vendor = "apple")]
        {
            let meta = self.table().meta(fd)?;
            if udp && meta.peer == Some(addr) {
                return self.send_allowed(fd, data, meta);
            }
        }
        let mut tbl = self.table();
        let result = if udp {
            match self.socket_io(&mut tbl, fd)? {
                Some(AsyncSocketIo::Datagram(io)) => Some(io.try_send_to(data, addr)),
                _ => None,
            }
        } else {
            None
        };
        let r = immediate_io(result, || {
            net::sendto(tbl.get(fd)?, data, send_flags(), &addr).map_err(from_rustix)
        });
        tbl.note_send(fd, &r)?;
        drop(tbl);
        self.waiter.wake();
        r
    }

    /// Receive up to `len` bytes plus the source address when the
    /// socket has one (datagrams).  `len` is capped at what one host
    /// call can carry back besides the reply's length and address
    /// ([`RECVFROM_HDR`]); the guest asks for at most that anyway.
    fn recvfrom(&self, fd: i32, len: usize) -> Res<(Vec<u8>, Option<SocketAddr>)> {
        let mut buf = vec![0u8; len.min(HOST_CALL_MAX - RECVFROM_HDR)];
        let mut tbl = self.table();
        let result = match self.socket_io(&mut tbl, fd)? {
            Some(AsyncSocketIo::Stream(io)) => Some(io.try_read(&mut buf).map(|n| (n, None))),
            Some(AsyncSocketIo::Datagram(io)) => {
                Some(io.try_recv_from(&mut buf).map(|(n, from)| (n, Some(from))))
            }
            _ => None,
        };
        let fd = tbl.get(fd)?;
        let result = immediate_io(result, || {
            net::recvfrom(fd, &mut buf[..], RecvFlags::empty())
                .map(|(n, _, from)| (n, from.and_then(|a| SocketAddr::try_from(a).ok())))
                .map_err(from_rustix)
        });
        let (n, from) = match result {
            Ok(r) => r,
            // Windows surfaces an earlier ICMP "port unreachable" on a UDP
            // socket as a reset.  Linux reports that as ECONNREFUSED on a
            // connected socket and not at all on an unconnected one.
            #[cfg(windows)]
            Err(errno::ECONNRESET)
                if sockopt::socket_type(fd).is_ok_and(|t| t == SocketType::DGRAM) =>
            {
                return Err(match net::getpeername(fd) {
                    Ok(Some(_)) => errno::ECONNREFUSED,
                    _ => errno::EAGAIN,
                });
            }
            Err(e) => return Err(e),
        };
        drop(tbl);
        buf.truncate(n);
        // A DNS answer: under an allow list, the addresses it gives for a
        // listed name are recorded, so the connect that follows is known.
        if let (Some(src), Some(policy)) = (from, &self.policy) {
            policy.learn_from_dns_answer(src, &buf);
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
        let result = self.table().remove(fd).map(drop);
        self.waiter.wake();
        result
    }

    fn peer_addr(&self, fd: i32) -> Res<SocketAddr> {
        let tbl = self.table();
        match tbl
            .sockets
            .get(&fd)
            .and_then(|s| s.registration.as_ref())
            .map(|s| &s.io)
        {
            Some(AsyncSocketIo::Stream(io)) => {
                return io.peer_addr().map_err(|e| errno::from_io(&e));
            }
            Some(AsyncSocketIo::Datagram(io)) => {
                return io.peer_addr().map_err(|e| errno::from_io(&e));
            }
            _ => {}
        }
        net::getpeername(tbl.get(fd)?)
            .map_err(from_rustix)?
            .and_then(|a| SocketAddr::try_from(a).ok())
            .ok_or(errno::ENOTCONN)
    }

    fn local_addr(&self, fd: i32) -> Res<SocketAddr> {
        let tbl = self.table();
        let result = match tbl
            .sockets
            .get(&fd)
            .and_then(|s| s.registration.as_ref())
            .map(|s| &s.io)
        {
            Some(AsyncSocketIo::Stream(io)) => Some(io.local_addr()),
            Some(AsyncSocketIo::Datagram(io)) => Some(io.local_addr()),
            Some(AsyncSocketIo::Listener { io, .. }) => Some(io.local_addr()),
            None => None,
        };
        if let Some(result) = result {
            return result.map_err(|e| errno::from_io(&e));
        }
        let addr = net::getsockname(tbl.get(fd)?).map_err(from_rustix)?;
        SocketAddr::try_from(addr).map_err(|_| errno::EAFNOSUPPORT)
    }

    /// `getsockopt` for the int-valued options guests actually use.
    /// Anything else is `ENOPROTOOPT`.
    fn getsockopt(&self, fd: i32, level: i32, name: i32) -> Res<i32> {
        let tbl = self.table();
        let listening = tbl.meta(fd)?.listening;
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
            (SOL_SOCKET, SO_ACCEPTCONN) => Ok(i32::from(listening)),
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
    /// Poll the guest's sockets on its behalf (`net_poll`): `entries` are
    /// (guest fd, events) pairs, `timeout_ms` as for poll(2) except that a
    /// negative one (forever) is refused with `EINVAL`.  Returns poll's
    /// count and the revents per entry; an fd the guest does not own
    /// reports `POLLNVAL`.
    fn poll(&self, entries: &[(i32, i16)], timeout_ms: i32) -> (i32, Vec<i16>) {
        let mut tbl = self.table();
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
        // An invalid entry counts as ready, so with one present poll must
        // return at once, whatever the timeout.  Otherwise a wait inside a
        // host call would freeze the vCPU and the embedder's thread with no
        // way to interrupt either; waiting is the host's job between
        // entries (see poll_ready), so an indefinite timeout is refused.
        let buffered = entries.iter().any(|&(fd, events)| {
            events & POLLIN != 0 && tbl.sockets.get(&fd).is_some_and(Socket::has_accepted)
        });
        let timeout_ms = if invalid > 0 {
            0
        } else {
            match u64::try_from(timeout_ms) {
                Ok(ms) => ms,
                Err(_) => return (-errno::EINVAL, revents),
            }
        };
        let timeout_ms = if buffered { 0 } else { timeout_ms };
        let timeout = Timespec {
            tv_sec: (timeout_ms / 1000) as _,
            tv_nsec: ((timeout_ms % 1000) * 1_000_000) as _,
        };
        if fds.is_empty() {
            // Nothing to poll: sleep for the timeout, as Linux does.
            std::thread::sleep(Duration::from_millis(timeout_ms));
            return (invalid, revents);
        }
        let mut ready = match rustix::event::poll(&mut fds, Some(&timeout)) {
            Ok(ready) => {
                for (pfd, &i) in fds.iter().zip(&index) {
                    revents[i] = linux_revents(pfd.revents());
                }
                ready as i32 + invalid
            }
            Err(e) => -from_rustix(e),
        };
        drop(fds);
        if ready >= 0 {
            for (i, &(fd, events)) in entries.iter().enumerate() {
                if events & POLLIN != 0 && tbl.sockets.get(&fd).is_some_and(Socket::has_accepted) {
                    if revents[i] == 0 {
                        ready += 1;
                    }
                    revents[i] |= POLLIN;
                }
            }
        }
        // This is where the guest learns what it must wait for: hostsock
        // asks before every accept/recv/send and, told "not yet", parks
        // the thread on that condition.  Watch the socket for exactly
        // that in the inter-step wait; a condition reported ready (or an
        // error) needs no watching.
        for (i, &(fd, events)) in entries.iter().enumerate() {
            let Some(socket) = tbl.sockets.get_mut(&fd) else {
                continue;
            };
            let err = revents[i] & (POLLERR | POLLHUP | POLLNVAL) != 0;
            if events & POLLIN != 0 {
                socket
                    .interests
                    .set(PollFlags::IN, revents[i] & POLLIN == 0 && !err);
            }
            if events & POLLOUT != 0 {
                socket
                    .interests
                    .set(PollFlags::OUT, revents[i] & POLLOUT == 0 && !err);
            }
        }
        drop(tbl);
        self.waiter.wake();
        (ready, revents)
    }

    /// Drop every host socket.  Used when the guest state that owned them
    /// is discarded by an in-place restore; the restored guest re-creates
    /// the sockets it holds on its resume entry.
    pub(crate) fn reset(&self) {
        let mut tbl = self.table();
        tbl.sockets.clear();
        drop(tbl);
        self.waiter.wake();
    }

    /// Whether the guest is parked on any socket: something the
    /// inter-step wait could wake it for.
    pub(crate) fn has_waitable(&self) -> bool {
        self.table()
            .sockets
            .values()
            .any(|s| !s.interests.is_empty())
    }

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.waiter.register(cx.waker());
        let mut table = self.table();
        let mut sockets = table
            .sockets
            .values_mut()
            .filter(|s| !s.interests.is_empty())
            .peekable();
        if sockets.peek().is_none() {
            return Poll::Ready(Ok(()));
        }
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(std::io::Error::other)?
            .id();
        // Every poll is non-blocking; registrations cannot outlive or race
        // replacement of their owning entry while the table is locked.
        for socket in sockets {
            for events in [PollFlags::IN, PollFlags::OUT] {
                if socket.interests.contains(events)
                    && let Poll::Ready(result) = socket.register(runtime)?.poll_ready(cx, events)
                {
                    socket.interests.remove(events);
                    return Poll::Ready(result);
                }
            }
        }
        Poll::Pending
    }

    pub(crate) fn needs_async_wait(&self) -> bool {
        self.has_waitable() && !self.async_waiting.load(Ordering::Relaxed)
    }

    fn wait_reply(self: &Arc<Self>, inbox: &Arc<Mutex<crate::async_calls::Inbox>>) -> Vec<u8> {
        if !self.has_waitable() {
            return int_reply(0);
        }
        if self.async_waiting.swap(true, Ordering::Relaxed) {
            tracing::warn!("duplicate asynchronous network wait");
            return int_reply(-errno::EBUSY);
        }
        let wait = NetworkWait(Arc::downgrade(self));
        let reply = async move {
            futures::future::poll_fn(|cx| match wait.0.upgrade() {
                Some(net) => net.poll_ready(cx),
                None => Poll::Ready(Ok(())),
            })
            .await?;
            Ok(int_reply(1))
        };
        match inbox.lock().unwrap().submit_fallible_reply(reply) {
            Ok(id) => crate::async_calls::pending_reply(id),
            Err(error) => {
                tracing::warn!(%error, "cannot queue asynchronous network wait");
                int_reply(-errno::ENOBUFS)
            }
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
        // macOS reports a refused connect as HUP alone, where Linux adds OUT
        // (as it does for any stream socket not connected): a guest waits
        // for POLLOUT to see a connect through, then reads its error.
        #[cfg(target_vendor = "apple")]
        {
            r |= POLLOUT;
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

fn int_reply(value: i32) -> Vec<u8> {
    let mut reply = vec![crate::HOST_CALL_OK];
    reply.extend_from_slice(&value.to_le_bytes());
    reply
}

/// Register all `net_*` host functions.
///
/// `net` carries the policy (which outbound destinations are allowed,
/// which ports `net_bind` accepts) and the socket table the closures
/// share; the caller keeps its own handle for the inter-step wait of the
/// cooperative step model.
pub(crate) fn register(
    target: &mut impl Registerable,
    net: &Arc<Net>,
    profile: &Arc<crate::profile::Profile>,
    inbox: &Arc<Mutex<crate::async_calls::Inbox>>,
) -> hyperlight_host::Result<()> {
    let net = net.clone();

    let n = net.clone();
    let pending = inbox.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_wait_async",
        move || -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_wait_async");
            Ok(n.wait_reply(&pending))
        },
    )?;

    let n = net.clone();
    let pending = inbox.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_connect_async",
        move |fd: i32, family: i32, addr: String, port: i32| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_connect_async");
            let Some(addr) = parse_addr(family, &addr, port) else {
                return Ok(int_reply(-errno::EINVAL));
            };
            Ok(n.connect_reply(fd, addr, &pending))
        },
    )?;
    let n = net.clone();
    let pending = inbox.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_sendto_async",
        move |fd: i32,
              data: Vec<u8>,
              family: i32,
              addr: String,
              port: i32|
              -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_sendto_async");
            let Some(addr) = parse_addr(family, &addr, port) else {
                return Ok(int_reply(-errno::EINVAL));
            };
            Ok(n.sendto_reply(fd, data, addr, &pending))
        },
    )?;

    let n = net.clone();
    let pending = inbox.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_send_async",
        move |fd: i32, data: Vec<u8>| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_send_async");
            Ok(n.send_reply(fd, data, &pending))
        },
    )?;

    // net_socket(family, type, protocol) -> fd or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_socket",
        move |family: i32, ty: i32, proto: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_socket");
            Ok(ret(n.socket(family, ty, proto), |fd| fd))
        },
    )?;

    // net_bind(fd, family, addr, port) -> 0 or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_bind",
        move |fd: i32, family: i32, addr: String, port: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_bind");
            let Some(sa) = parse_addr(family, &addr, port) else {
                return Ok(-errno::EINVAL);
            };
            Ok(ret(n.bind(fd, sa), |()| 0))
        },
    )?;

    // net_listen(fd, backlog) -> 0 or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_listen",
        move |fd: i32, backlog: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_listen");
            Ok(ret(n.listen(fd, backlog), |()| 0))
        },
    )?;

    // net_accept(fd) -> [i32 new_fd | packed peer addr]
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_accept",
        move |fd: i32| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_accept");
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
    let prof = profile.clone();
    target.register_host_function(
        "net_connect",
        move |fd: i32, family: i32, addr: String, port: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_connect");
            let Some(sa) = parse_addr(family, &addr, port) else {
                return Ok(-errno::EINVAL);
            };
            Ok(ret(n.connect(fd, sa), |()| 0))
        },
    )?;

    // net_send(fd, data) -> bytes_sent or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_send",
        move |fd: i32, data: Vec<u8>| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_send");
            Ok(ret(n.send(fd, &data), |sent| sent as i32))
        },
    )?;

    // net_sendto(fd, data, family, addr, port) -> bytes_sent or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_sendto",
        move |fd: i32,
              data: Vec<u8>,
              family: i32,
              addr: String,
              port: i32|
              -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_sendto");
            let Some(sa) = parse_addr(family, &addr, port) else {
                return Ok(-errno::EINVAL);
            };
            Ok(ret(n.sendto(fd, &data, sa), |sent| sent as i32))
        },
    )?;

    // net_recvfrom(fd, len) -> [i32 len | packed src addr | data]
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_recvfrom",
        move |fd: i32, len: i32| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_recvfrom");
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
    let prof = profile.clone();
    target.register_host_function(
        "net_shutdown",
        move |fd: i32, how: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_shutdown");
            Ok(ret(n.shutdown(fd, how), |()| 0))
        },
    )?;

    // net_close(fd) -> 0 or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_close",
        move |fd: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_close");
            Ok(ret(n.close(fd), |()| 0))
        },
    )?;

    // net_disconnect(fd) -> 0 or -errno: connect(AF_UNSPEC) on a datagram socket
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_disconnect",
        move |fd: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_disconnect");
            Ok(ret(n.disconnect(fd), |()| 0))
        },
    )?;

    // net_getpeername(fd) -> [i32 status | packed addr]
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_getpeername",
        move |fd: i32| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_getpeername");
            Ok(ret_vec(n.peer_addr(fd).map(|sa| addr_result(&sa))))
        },
    )?;

    // net_getsockname(fd) -> [i32 status | packed addr]
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_getsockname",
        move |fd: i32| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_getsockname");
            Ok(ret_vec(n.local_addr(fd).map(|sa| addr_result(&sa))))
        },
    )?;

    // net_getsockopt(fd, level, optname) -> value or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_getsockopt",
        move |fd: i32, level: i32, optname: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_getsockopt");
            Ok(ret(n.getsockopt(fd, level, optname), |v| v))
        },
    )?;

    // net_setsockopt(fd, level, optname, value) -> 0 or -errno
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_setsockopt",
        move |fd: i32, level: i32, optname: i32, value: i32| -> hyperlight_host::Result<i32> {
            let _profile = prof.host("net_setsockopt");
            Ok(ret(n.setsockopt(fd, level, optname, value), |()| 0))
        },
    )?;

    // net_poll(pollfds, timeout_ms) -> [i32 ready | i16 revents per fd]
    //
    // Input is 8 bytes per fd: i32 fd, i16 events, i16 pad.
    let n = net.clone();
    let prof = profile.clone();
    target.register_host_function(
        "net_poll",
        move |pollfds: Vec<u8>, timeout_ms: i32| -> hyperlight_host::Result<Vec<u8>> {
            let _profile = prof.host("net_poll");
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
            // The guest kernel only ever asks with a zero timeout (it parks
            // its own thread and the host waits between entries); anything
            // else would stall the vCPU inside this call.
            let (ready, revents) = if timeout_ms == 0 {
                n.poll(&entries, 0)
            } else {
                (-errno::EINVAL, vec![0i16; entries.len()])
            };
            let mut buf = Vec::with_capacity(4 + revents.len() * 2);
            buf.extend(ready.to_le_bytes());
            for r in revents {
                buf.extend(r.to_le_bytes());
            }
            Ok(buf)
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
    use crate::async_calls::{PendingCalls, Wake};
    use crate::net_policy::AllowList;
    use futures::{FutureExt, future::poll_fn};
    use rustix::fd::{AsRawFd, RawFd};
    use std::collections::HashSet;
    use std::sync::atomic::AtomicUsize;

    #[derive(Default)]
    struct WakeCount(AtomicUsize);

    impl std::task::Wake for WakeCount {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn registrations(net: &Net) -> HashMap<i32, (tokio::runtime::Id, RawFd)> {
        net.table()
            .sockets
            .iter()
            .filter_map(|(&fd, s)| {
                let registered = s.registration.as_ref()?;
                let handle = match &registered.io {
                    AsyncSocketIo::Stream(io) => io.as_fd(),
                    AsyncSocketIo::Datagram(io) => io.as_fd(),
                    AsyncSocketIo::Listener { io, .. } => io.as_fd(),
                };
                Some((fd, (registered.runtime, handle.as_raw_fd())))
            })
            .collect()
    }

    fn cached_readable(net: &Net, fd: i32) -> Option<std::io::Result<()>> {
        let table = net.table();
        match &table.sockets[&fd].registration.as_ref().unwrap().io {
            AsyncSocketIo::Stream(io) => io.readable().now_or_never(),
            AsyncSocketIo::Datagram(io) => io.readable().now_or_never(),
            AsyncSocketIo::Listener { .. } => panic!("expected a stream or datagram"),
        }
    }

    fn open_net() -> Arc<Net> {
        Arc::new(Net::new(NetworkPolicy::AllowAll, None))
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

    async fn wait_network(net: &Arc<Net>) -> std::io::Result<()> {
        poll_fn(|cx| net.poll_ready(cx)).await
    }

    fn queue_network(net: &Arc<Net>, engine: &PendingCalls) {
        engine.inbox.lock().unwrap().negotiated = true;

        assert_eq!(net.wait_reply(&engine.inbox)[0], 2);
        assert!(net.async_waiting.load(Ordering::Relaxed));
    }

    #[test]
    fn immediate_io_only_uses_syscalls_without_tokio_or_on_would_block() {
        use std::io::ErrorKind;
        for (result, expected, fallback) in [
            (Some(Ok(7)), Ok(7), false),
            (
                Some(Err(ErrorKind::PermissionDenied.into())),
                Err(errno::EACCES),
                false,
            ),
            (Some(Err(ErrorKind::WouldBlock.into())), Ok(9), true),
            (None, Ok(9), true),
        ] {
            let mut called = false;
            assert_eq!(
                immediate_io(result, || {
                    called = true;
                    Ok(9)
                }),
                expected
            );
            assert_eq!(called, fallback);
        }
        assert_eq!(
            immediate_io(Some(Err(std::io::ErrorKind::WouldBlock.into())), || {
                Err::<usize, _>(errno::EPIPE)
            }),
            Err(errno::EPIPE)
        );
    }

    #[tokio::test]
    async fn tcp_transfers_progress_without_yielding_to_the_reactor() {
        let net = open_net();

        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 1).unwrap();
        let cli = net.socket(AF_INET, SOCK_STREAM, 6).unwrap();
        connect_blocking(&net, cli, net.local_addr(srv).unwrap()).unwrap();
        wait(&net, srv, POLLIN);
        let (conn, _) = net.accept(srv).unwrap();
        for _ in 0..2 {
            assert_eq!(net.send(cli, b"ping"), Ok(4));
            wait(&net, conn, POLLIN);
            assert_eq!(net.recvfrom(conn, 4).unwrap(), (b"ping".to_vec(), None));
            assert_eq!(net.send(conn, b"pong"), Ok(4));
            wait(&net, cli, POLLIN);
            assert_eq!(net.recvfrom(cli, 4).unwrap().0, b"pong");
        }
        assert_eq!(registrations(&net).len(), 2);
        assert_eq!(net.peer_addr(cli).unwrap(), net.local_addr(conn).unwrap());
        assert_eq!(net.peer_addr(conn).unwrap(), net.local_addr(cli).unwrap());
        net.shutdown(cli, SHUT_WR).unwrap();
        wait(&net, conn, POLLIN);
        assert!(net.recvfrom(conn, 1).unwrap().0.is_empty());
        assert!(
            registrations(&net)
                .values()
                .all(|(runtime, _)| *runtime == tokio::runtime::Handle::current().id())
        );
    }

    #[tokio::test]
    async fn udp_transfers_preserve_addresses_empty_datagrams_and_cached_readiness() {
        let net = open_net();

        let fd = net.socket(AF_INET, SOCK_DGRAM, 17).unwrap();
        net.bind(fd, loopback(0)).unwrap();
        let addr = net.local_addr(fd).unwrap();
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        let peer_addr = peer.local_addr().unwrap();
        assert_eq!(net.sendto(fd, b"ping", peer_addr), Ok(4));
        let mut buf = [0; 4];
        assert_eq!(peer.recv_from(&mut buf).unwrap(), (4, addr));
        assert_eq!(&buf, b"ping");
        peer.send_to(b"pong", addr).unwrap();
        wait(&net, fd, POLLIN);
        assert_eq!(
            net.recvfrom(fd, 4).unwrap(),
            (b"pong".to_vec(), Some(peer_addr))
        );

        assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
        peer.send_to(b"ready", addr).unwrap();
        tokio::time::timeout(Duration::from_secs(2), wait_network(&net))
            .await
            .unwrap()
            .unwrap();
        assert!(cached_readable(&net, fd).unwrap().is_ok());
        assert_eq!(net.recvfrom(fd, 8).unwrap().0, b"ready");
        assert_eq!(net.recvfrom(fd, 8), Err(errno::EAGAIN));
        assert!(
            cached_readable(&net, fd).is_none(),
            "Tokio I/O must clear stale readiness"
        );

        peer.send_to(b"", addr).unwrap();
        wait(&net, fd, POLLIN);
        assert_eq!(net.recvfrom(fd, 0).unwrap(), (Vec::new(), Some(peer_addr)));
        assert_eq!(net.recvfrom(fd, 8), Err(errno::EAGAIN));
        net.connect(fd, peer_addr).unwrap();
        assert_eq!(net.peer_addr(fd).unwrap(), peer_addr);
        assert_eq!(net.send(fd, b""), Ok(0));
        assert_eq!(peer.recv_from(&mut buf).unwrap(), (0, addr));
        assert_eq!(net.sendto(fd, b"same", peer_addr), Ok(4));
        assert_eq!(peer.recv_from(&mut buf).unwrap(), (4, addr));
        assert_eq!(&buf, b"same");
        net.disconnect(fd).unwrap();
        assert_eq!(net.peer_addr(fd), Err(errno::ENOTCONN));
        assert_eq!(net.sendto(fd, b"free", peer_addr), Ok(4));
        assert_eq!(
            peer.recv_from(&mut buf).unwrap(),
            (4, net.local_addr(fd).unwrap())
        );
        assert_eq!(&buf, b"free");

        let unbound = udp(&net);
        assert_eq!(net.sendto(unbound, b"auto", peer_addr), Ok(4));
        let source = peer.recv_from(&mut buf).unwrap().1;
        assert_eq!(source.port(), net.local_addr(unbound).unwrap().port());
        assert!(!registrations(&net).contains_key(&unbound));
    }

    #[test]
    fn setup_io_survives_runtime_changes_without_driving_futures() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        peer.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        let peer_addr = peer.local_addr().unwrap();
        let mut buf = [0; 4];
        for _ in 0..2 {
            let old = registrations(&net).get(&fd).copied();
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(async {
                assert_eq!(net.sendto(fd, b"data", peer_addr), Ok(4));
                assert_eq!(registrations(&net)[&fd].0, runtime.handle().id());
                assert_eq!(peer.recv_from(&mut buf).unwrap().0, 4);
                assert_eq!(&buf, b"data");
            });
            assert!(old.is_none_or(|old| old.1 != registrations(&net)[&fd].1));
            drop(runtime);
            assert_eq!(net.sendto(fd, b"sync", peer_addr), Ok(4));
            assert_eq!(peer.recv_from(&mut buf).unwrap().0, 4);
            assert_eq!(&buf, b"sync");
            peer.send_to(b"back", net.local_addr(fd).unwrap()).unwrap();
            wait(&net, fd, POLLIN);
            assert_eq!(net.recvfrom(fd, 4).unwrap().0, b"back");
        }
    }

    #[tokio::test]
    async fn tokio_transfers_keep_policy_checks_ahead_of_io() {
        let net = Arc::new(Net::new(NetworkPolicy::AllowList(AllowList::empty()), None));

        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        assert_eq!(net.sendto(fd, b"denied", loopback(9)), Err(errno::EACCES));
        assert!(registrations(&net).is_empty());
        // Establish a DNS association without the outbound-address check to
        // exercise the separate payload policy in connected send.
        net.connect_allowed(fd, loopback(53)).unwrap();
        assert_eq!(net.send(fd, b"not a DNS question"), Err(errno::EACCES));
        assert!(registrations(&net).is_empty());
    }

    #[test]
    fn readiness_io_errors_use_the_shared_failure_path() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        net.poll(&[(fd, POLLIN)], 0);
        let mut engine = PendingCalls::default();
        queue_network(&net, &engine);
        assert!(
            poll_fn(|cx| engine.poll_batch(cx))
                .now_or_never()
                .unwrap()
                .is_err()
        );
        assert!(!net.async_waiting.load(Ordering::Relaxed));
        engine.clear();
        assert!(!engine.is_pending());
    }

    #[tokio::test]
    async fn pending_readiness_observes_new_socket_interests() {
        let net = open_net();
        let first = udp(&net);
        net.bind(first, loopback(0)).unwrap();
        net.poll(&[(first, POLLIN)], 0);
        let pending = net.clone();
        let wait = tokio::spawn(async move { wait_network(&pending).await });
        tokio::task::yield_now().await;
        assert!(!wait.is_finished());
        assert!(registrations(&net).contains_key(&first));
        let second = udp(&net);
        net.bind(second, loopback(0)).unwrap();
        net.poll(&[(second, POLLIN)], 0);
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        peer.send_to(b"new", net.local_addr(second).unwrap())
            .unwrap();
        tokio::time::timeout(Duration::from_secs(1), wait)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(net.recvfrom(second, 3).unwrap().0, b"new");
    }

    #[tokio::test]
    async fn listening_replaces_stream_registration_and_wakes_the_waiter() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.table()
            .entry_mut(srv)
            .unwrap()
            .interests
            .insert(PollFlags::IN);
        let wakes = Arc::new(WakeCount::default());
        let waker = std::task::Waker::from(wakes.clone());
        let mut cx = Context::from_waker(&waker);
        assert!(net.poll_ready(&mut cx).is_pending());
        assert!(registrations(&net).contains_key(&srv));
        wakes.0.store(0, Ordering::Relaxed);

        net.listen(srv, 1).unwrap();
        assert!(wakes.0.load(Ordering::Relaxed) > 0);
        assert!(!registrations(&net).contains_key(&srv));
        assert!(net.has_waitable());
        assert!(net.poll_ready(&mut cx).is_pending());
        let peer = std::net::TcpStream::connect(net.local_addr(srv).unwrap()).unwrap();
        tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
            .await
            .unwrap()
            .unwrap();
        assert!(net.table().sockets[&srv].has_accepted());
        assert_eq!(net.accept(srv).unwrap().1, Some(peer.local_addr().unwrap()));
    }

    #[tokio::test]
    async fn pending_readiness_survives_udp_reconnect_and_disconnect() {
        let net = Arc::new(Net::new(NetworkPolicy::AllowAll, Some(ListenPorts::all())));
        let fd = udp(&net);
        let reservation = std::net::UdpSocket::bind(loopback(0)).unwrap();
        let addr = reservation.local_addr().unwrap();
        drop(reservation);
        net.bind(fd, addr).unwrap();
        let first = std::net::UdpSocket::bind(loopback(0)).unwrap();
        let second = std::net::UdpSocket::bind(loopback(0)).unwrap();
        net.connect(fd, first.local_addr().unwrap()).unwrap();

        for reconnect in [true, false] {
            assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
            let wakes = Arc::new(WakeCount::default());
            let waker = std::task::Waker::from(wakes.clone());
            let mut cx = Context::from_waker(&waker);
            assert!(net.poll_ready(&mut cx).is_pending());
            wakes.0.store(0, Ordering::Relaxed);
            if reconnect {
                net.connect(fd, second.local_addr().unwrap()).unwrap();
            } else {
                net.disconnect(fd).unwrap();
            }
            #[cfg(target_vendor = "apple")]
            assert!(wakes.0.load(Ordering::Relaxed) > 0);
            assert!(net.has_waitable());
            assert_eq!(net.local_addr(fd).unwrap(), addr);
            second.send_to(b"new", addr).unwrap();
            tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                .await
                .unwrap()
                .unwrap();
            assert_eq!(net.recvfrom(fd, 3).unwrap().0, b"new");
            assert_eq!(net.recvfrom(fd, 3), Err(errno::EAGAIN));
        }
    }

    #[test]
    fn readiness_submission_is_bounded_and_cancellation_releases_the_wait() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        net.poll(&[(fd, POLLIN)], 0);
        let mut engine = PendingCalls::default();
        queue_network(&net, &engine);

        assert_eq!(net.wait_reply(&engine.inbox), int_reply(-errno::EBUSY));
        engine.clear();
        assert!(!net.async_waiting.load(Ordering::Relaxed));
        let mut count = 0;
        while engine
            .inbox
            .lock()
            .unwrap()
            .submit_reply(std::future::pending())
            .is_ok()
        {
            count += 1;
            assert!(count <= 1024);
        }
        assert_eq!(count, 1024);
        assert_eq!(net.wait_reply(&engine.inbox), int_reply(-errno::ENOBUFS));
        assert!(!net.async_waiting.load(Ordering::Relaxed));
        engine.clear();
    }

    #[test]
    fn socket_churn_across_runtimes_releases_old_registrations() {
        let net = Arc::new(Net::new(NetworkPolicy::AllowAll, Some(ListenPorts::all())));
        let mut sockets: Vec<_> = (0..32)
            .map(|_| {
                let fd = udp(&net);
                net.bind(fd, loopback(0)).unwrap();
                (fd, net.local_addr(fd).unwrap())
            })
            .collect();
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        for turn in 0..8u8 {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(async {
                let old = registrations(&net);
                let interests: Vec<_> = sockets.iter().map(|(fd, _)| (*fd, POLLIN)).collect();
                assert_eq!(net.poll(&interests, 0).0, 0);
                assert!(wait_network(&net).now_or_never().is_none());
                let current = registrations(&net);
                assert!(current.values().all(|s| s.0 == runtime.handle().id()));
                assert!(old.iter().all(|(fd, s)| s.1 != current[fd].1));
                for (fd, addr) in sockets.iter_mut().step_by(2) {
                    net.close(*fd).unwrap();
                    assert!(!net.table().sockets.contains_key(fd));
                    *fd = udp(&net);
                    net.bind(*fd, *addr).unwrap();
                }
                let interests: Vec<_> = sockets.iter().map(|(fd, _)| (*fd, POLLIN)).collect();
                assert_eq!(net.poll(&interests, 0).0, 0);
                assert!(wait_network(&net).now_or_never().is_none());
                assert_eq!(registrations(&net).len(), sockets.len());
                for (_, addr) in &sockets {
                    assert_eq!(peer.send_to(&[turn], addr).unwrap(), 1);
                }
                let mut remaining: HashSet<_> = sockets.iter().map(|(fd, _)| *fd).collect();
                tokio::time::timeout(Duration::from_secs(2), async {
                    while !remaining.is_empty() {
                        wait_network(&net).await.unwrap();
                        remaining.retain(|fd| match net.recvfrom(*fd, 1) {
                            Ok((bytes, _)) => {
                                assert_eq!(bytes, [turn]);
                                false
                            }
                            Err(e) if e == errno::EAGAIN => true,
                            Err(e) => panic!("recv failed: errno {e}"),
                        });
                    }
                })
                .await
                .unwrap();
                net.poll(&interests, 0);
                assert!(wait_network(&net).now_or_never().is_none());
            });
        }
        net.reset();
        assert!(net.table().sockets.is_empty());
        for (_, addr) in sockets {
            let _rebound = std::net::UdpSocket::bind(addr).unwrap();
        }
    }

    #[test]
    fn typed_network_calls_reject_closed_fds_before_authorization() {
        let net = open_net();
        let inbox = Arc::new(Mutex::new(crate::async_calls::Inbox::default()));
        let fd = udp(&net);
        net.close(fd).unwrap();
        assert_eq!(
            net.connect_reply(fd, loopback(9), &inbox),
            int_reply(-errno::EBADF)
        );
        assert_eq!(
            net.send_reply(fd, vec![1], &inbox),
            int_reply(-errno::EBADF)
        );
        assert_eq!(
            net.sendto_reply(fd, vec![1], loopback(9), &inbox),
            int_reply(-errno::EBADF)
        );
        assert!(!inbox.lock().unwrap().is_pending());
    }

    #[tokio::test]
    async fn readiness_calls_share_batches_with_embedder_calls_and_timers() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        let mut engine = PendingCalls::default();
        engine.inbox.lock().unwrap().negotiated = true;
        queue_network(&net, &engine);
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        let registration = registrations(&net)[&fd];
        for _ in 0..3 {
            assert_eq!(registration, registrations(&net)[&fd]);
            engine
                .inbox
                .lock()
                .unwrap()
                .submit(
                    "ready".into(),
                    String::new(),
                    crate::closure_handler(|_| futures::future::ready(Ok("7".into())).boxed()),
                )
                .unwrap();
            assert_eq!(
                poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
                Wake::Calls
            );
            engine.delivered();
            assert_eq!(registration, registrations(&net)[&fd]);
        }
        peer.send_to(b"data", net.local_addr(fd).unwrap()).unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        engine.set_timer(Some(std::time::Instant::now() - Duration::from_secs(1)));
        engine
            .inbox
            .lock()
            .unwrap()
            .submit(
                "ready".into(),
                String::new(),
                crate::closure_handler(|_| futures::future::ready(Ok("7".into())).boxed()),
            )
            .unwrap();
        assert_eq!(
            poll_fn(|cx| engine.poll_batch(cx)).await.unwrap(),
            Wake::Calls
        );
        let batch = engine.take_batch();
        assert_eq!(u32::from_le_bytes(batch[4..8].try_into().unwrap()), 2);
        assert_eq!(
            batch.len(),
            39,
            "readiness uses an ordinary typed completion"
        );
        engine.delivered();
        assert!(!net.has_waitable());
        assert!(!engine.is_pending());
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
    }

    #[tokio::test]
    async fn readiness_rechecks_changed_interests_and_rearms_each_direction() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        for _ in 0..2 {
            assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
            let mut wait = Box::pin(wait_network(&net));
            assert!(futures::poll!(wait.as_mut()).is_pending());
            peer.send_to(b"data", net.local_addr(fd).unwrap()).unwrap();
            tokio::time::sleep(Duration::from_millis(10)).await;
            // A synchronous guest poll/recv consumes the interest before the
            // shared engine observes its queued readiness.
            assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 1);
            assert_eq!(net.recvfrom(fd, 16).unwrap().0, b"data");
            assert!(wait.now_or_never().unwrap().is_ok());

            net.table()
                .entry_mut(fd)
                .unwrap()
                .interests
                .insert(PollFlags::OUT);
            tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                .await
                .unwrap()
                .unwrap();
            assert!(!net.has_waitable());
        }
    }

    #[tokio::test]
    async fn persistent_waits_do_not_retain_closed_reset_or_dropped_sockets() {
        for action in 0..3 {
            let net = open_net();
            let fd = udp(&net);
            net.bind(fd, loopback(0)).unwrap();
            let addr = net.local_addr(fd).unwrap();
            net.poll(&[(fd, POLLIN)], 0);
            let mut engine = PendingCalls::default();
            queue_network(&net, &engine);
            let wakes = Arc::new(WakeCount::default());
            let waker = std::task::Waker::from(wakes.clone());
            let mut cx = Context::from_waker(&waker);
            assert!(engine.poll_batch(&mut cx).is_pending());
            assert!(registrations(&net).contains_key(&fd));
            wakes.0.store(0, Ordering::Relaxed);
            match action {
                0 => net.close(fd).unwrap(),
                1 => net.reset(),
                _ => drop(net),
            }
            // No intervening engine poll or prune: close/rebind can happen
            // during one guest entry while its old future is still retained.
            assert!(wakes.0.load(Ordering::Relaxed) > 0);
            let _rebound = std::net::UdpSocket::bind(addr).unwrap();
            assert_eq!(
                engine.poll_batch(&mut cx).map(Result::unwrap),
                Poll::Ready(Wake::Calls)
            );
            engine.clear();
        }
    }

    #[test]
    fn readiness_reports_io_errors_and_rebinds_after_runtime_changes() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        net.poll(&[(fd, POLLIN)], 0);
        assert!(
            wait_network(&net).now_or_never().unwrap().is_err(),
            "missing runtime must be an error"
        );
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            assert!(wait_network(&net).now_or_never().is_none());
        });
        drop(runtime);
        assert!(wait_network(&net).now_or_never().unwrap().is_err());
        net.poll(&[(fd, POLLIN)], 0);
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            assert!(wait_network(&net).now_or_never().is_none());
            let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
            peer.send_to(b"data", net.local_addr(fd).unwrap()).unwrap();
            tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                .await
                .unwrap()
                .unwrap();
        });
    }

    #[tokio::test]
    async fn clearing_shared_waits_keeps_live_sockets_but_discards_their_wakes() {
        let net = open_net();
        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        net.poll(&[(fd, POLLIN)], 0);
        let mut engine = PendingCalls::default();
        queue_network(&net, &engine);
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        engine.clear();
        assert!(!net.async_waiting.load(Ordering::Relaxed));
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        peer.send_to(b"data", net.local_addr(fd).unwrap()).unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(poll_fn(|cx| engine.poll_batch(cx)).now_or_never().is_none());
        queue_network(&net, &engine);
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), poll_fn(|cx| engine.poll_batch(cx)))
                .await
                .unwrap()
                .unwrap(),
            Wake::Calls
        );
        assert_eq!(net.recvfrom(fd, 16).unwrap().0, b"data");
    }

    #[tokio::test]
    async fn async_readiness_rearms_without_stale_events_and_releases_sockets() {
        let net = open_net();

        let fd = udp(&net);
        net.bind(fd, loopback(0)).unwrap();
        let peer = std::net::UdpSocket::bind(loopback(0)).unwrap();
        let addr = net.local_addr(fd).unwrap();
        for _ in 0..2 {
            assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
            assert!(
                tokio::time::timeout(Duration::from_millis(20), wait_network(&net))
                    .await
                    .is_err()
            );
            peer.send_to(b"data", addr).unwrap();
            tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                .await
                .unwrap()
                .unwrap();
            assert!(!net.has_waitable());
            assert_eq!(net.recvfrom(fd, 16).unwrap().0, b"data");
        }
        assert_eq!(registrations(&net).len(), 1);
        net.table().remove(fd).unwrap();
        assert!(net.table().sockets.is_empty());
    }

    #[tokio::test]
    async fn async_listener_preserves_accepts_across_wait_cancellation_and_guest_polling() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 4).unwrap();
        for _ in 0..2 {
            assert_eq!(net.poll(&[(srv, POLLIN | POLLOUT)], 0), (0, vec![0]));
            assert!(
                tokio::time::timeout(Duration::from_millis(20), wait_network(&net))
                    .await
                    .is_err()
            );
            let peer = std::net::TcpStream::connect(net.local_addr(srv).unwrap()).unwrap();
            tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                .await
                .unwrap()
                .unwrap();
            assert!(net.table().sockets[&srv].has_accepted());
            // A second wait must not consume another connection or overwrite
            // the first. Its writable interest must not wake spuriously.
            net.table()
                .entry_mut(srv)
                .unwrap()
                .interests
                .insert(PollFlags::IN);
            wait_network(&net).await.unwrap();
            assert_eq!(net.poll(&[(srv, POLLIN | POLLOUT)], 0), (1, vec![POLLIN]));
            let (fd, addr) = net.accept(srv).unwrap();
            assert_eq!(addr, Some(peer.local_addr().unwrap()));
            assert_eq!(net.recvfrom(fd, 1), Err(errno::EAGAIN));
            net.close(fd).unwrap();
        }
        net.close(srv).unwrap();
        assert!(net.table().sockets.is_empty());
    }

    #[tokio::test]
    async fn async_stream_connect_read_rearm_and_eof() {
        use std::io::Write;
        let net = open_net();

        let listener = std::net::TcpListener::bind(loopback(0)).unwrap();
        let fd = tcp(&net);
        let result = net.connect(fd, listener.local_addr().unwrap());
        assert!(
            result == Ok(()) || result == Err(errno::EINPROGRESS),
            "{result:?}"
        );
        let (mut peer, _) = listener.accept().unwrap();
        net.table()
            .entry_mut(fd)
            .unwrap()
            .interests
            .insert(PollFlags::OUT);
        tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(net.getsockopt(fd, SOL_SOCKET, SO_ERROR).unwrap(), 0);
        for _ in 0..2 {
            assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
            assert!(
                tokio::time::timeout(Duration::from_millis(20), wait_network(&net))
                    .await
                    .is_err()
            );
            peer.write_all(b"ping").unwrap();
            tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                .await
                .unwrap()
                .unwrap();
            assert_eq!(net.recvfrom(fd, 4).unwrap().0, b"ping");
            assert_eq!(net.recvfrom(fd, 4), Err(errno::EAGAIN));
            assert!(cached_readable(&net, fd).is_none());
        }
        assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
        peer.shutdown(std::net::Shutdown::Write).unwrap();
        tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
            .await
            .unwrap()
            .unwrap();
        assert!(net.recvfrom(fd, 1).unwrap().0.is_empty());
    }

    #[tokio::test]
    async fn async_stream_writability_recovers_after_backpressure() {
        use std::io::Read;
        let net = open_net();

        let listener = std::net::TcpListener::bind(loopback(0)).unwrap();
        let fd = tcp(&net);
        connect_blocking(&net, fd, listener.local_addr().unwrap()).unwrap();
        let (mut peer, _) = listener.accept().unwrap();
        net.setsockopt(fd, SOL_SOCKET, SO_SNDBUF, 4096).unwrap();
        let chunk = [1u8; 65536];
        let mut sent = 0;
        loop {
            match net.send(fd, &chunk) {
                Ok(n) => sent += n,
                Err(e) if e == errno::EAGAIN => break,
                Err(e) => panic!("send failed: {e}"),
            }
            assert!(sent < 16 * 1024 * 1024, "failed to fill the send buffer");
        }
        assert_eq!(net.poll(&[(fd, POLLOUT)], 0).0, 0);
        let reader = std::thread::spawn(move || {
            peer.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut data = vec![0; sent];
            peer.read_exact(&mut data).unwrap();
            peer
        });
        tokio::time::timeout(Duration::from_secs(5), wait_network(&net))
            .await
            .unwrap()
            .unwrap();
        let _peer = reader.join().unwrap();
        assert!(net.send(fd, b"more").unwrap() > 0);
    }

    #[tokio::test]
    async fn closing_or_resetting_sockets_wakes_pending_readiness_and_releases_ports() {
        use futures::{FutureExt, poll};
        for reset in [false, true] {
            let net = open_net();
            let fd = udp(&net);
            net.bind(fd, loopback(0)).unwrap();
            let addr = net.local_addr(fd).unwrap();
            assert_eq!(net.poll(&[(fd, POLLIN)], 0).0, 0);
            let mut wait = wait_network(&net).boxed();
            assert!(poll!(&mut wait).is_pending());
            if reset {
                net.reset();
            } else {
                net.close(fd).unwrap();
            }
            tokio::time::timeout(Duration::from_secs(1), wait)
                .await
                .unwrap()
                .unwrap();
            assert!(net.table().sockets.is_empty());
            assert!(!net.has_waitable());
            let _rebound = std::net::UdpSocket::bind(addr).unwrap();
        }
    }

    #[test]
    fn async_registration_moves_between_runtimes_without_losing_accepts() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 1).unwrap();
        assert_eq!(net.poll(&[(srv, POLLIN)], 0).0, 0);
        let peer = std::net::TcpStream::connect(net.local_addr(srv).unwrap()).unwrap();
        for _ in 0..2 {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(async {
                net.table()
                    .entry_mut(srv)
                    .unwrap()
                    .interests
                    .insert(PollFlags::IN);
                tokio::time::timeout(Duration::from_secs(1), wait_network(&net))
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(registrations(&net)[&srv].0, runtime.handle().id());
                assert!(net.table().sockets[&srv].has_accepted());
            });
        }
        assert_eq!(net.poll(&[(srv, POLLIN)], 0), (1, vec![POLLIN]));
        assert_eq!(net.accept(srv).unwrap().1, Some(peer.local_addr().unwrap()));
    }

    #[tokio::test]
    async fn buffered_listener_errors_reach_guest_accept_and_poll_counts_each_entry() {
        let net = open_net();
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 1).unwrap();
        assert_eq!(net.poll(&[(srv, POLLIN)], 0).0, 0);
        assert!(
            tokio::time::timeout(Duration::from_millis(20), wait_network(&net))
                .await
                .is_err()
        );
        {
            let mut table = net.table();
            let AsyncSocketIo::Listener { accepted, .. } = &mut table
                .entry_mut(srv)
                .unwrap()
                .registration
                .as_mut()
                .unwrap()
                .io
            else {
                panic!("listener registered as a stream");
            };
            *accepted = Some(Err(std::io::ErrorKind::ConnectionAborted.into()));
        }
        assert_eq!(
            net.poll(&[(srv, POLLIN), (srv, POLLIN), (-1, POLLOUT)], 1000),
            (3, vec![POLLIN, POLLIN, POLLNVAL])
        );
        assert_eq!(net.poll(&[(srv, POLLIN)], -1).0, -errno::EINVAL);
        assert_eq!(net.accept(srv), Err(errno::ECONNABORTED));
        assert_eq!(net.poll(&[(srv, POLLIN)], 0).0, 0);
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

    /// Connect as the guest's socket layer does: a connect in progress is
    /// completed by waiting for `POLLOUT` and reading `SO_ERROR`.
    fn connect_blocking(net: &Net, fd: i32, addr: SocketAddr) -> Res<()> {
        match net.connect(fd, addr) {
            Err(e) if e == errno::EINPROGRESS => {
                wait(net, fd, POLLOUT);
                match net.getsockopt(fd, SOL_SOCKET, SO_ERROR)? {
                    0 => Ok(()),
                    e => Err(e),
                }
            }
            r => r,
        }
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
        connect_blocking(&net, cli, addr).unwrap();
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
        connect_blocking(&net, cli, net.local_addr(srv).unwrap()).unwrap();
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
        connect_blocking(&net, cli, net.local_addr(srv).unwrap()).unwrap();
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
        connect_blocking(&net, b, a_addr).unwrap();
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

    /// What glibc's getaddrinfo() does to sort a dual-stack answer: one
    /// IPv6 UDP socket, connected to an IPv6 candidate, disconnected, then
    /// connected to an IPv4 one, whose source address must be v4-mapped.
    #[test]
    fn disconnect_resets_datagram_association() {
        let net = open_net();
        let Ok(s) = net.socket(AF_INET6, SOCK_DGRAM, 0) else {
            eprintln!("SKIP: no IPv6");
            return;
        };
        let v6 = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 8000);
        if net.connect(s, v6).is_err() {
            eprintln!("SKIP: no IPv6 loopback");
            return;
        }
        assert_eq!(net.peer_addr(s).unwrap(), v6);

        net.disconnect(s).unwrap();
        assert_eq!(net.peer_addr(s), Err(errno::ENOTCONN));

        // A dual-stack socket takes the IPv4 candidate as is on Linux, where
        // glibc runs; Windows wants it spelled v4-mapped and refuses a raw
        // IPv4 address on an IPv6 socket.
        #[cfg(target_os = "linux")]
        {
            net.connect(s, loopback(8000)).unwrap();
            assert_eq!(net.peer_addr(s).unwrap().port(), 8000);
            let local = net.local_addr(s).unwrap();
            assert!(
                matches!(local.ip(), IpAddr::V6(ip) if ip.to_ipv4_mapped().is_some()),
                "source after an IPv4 reconnect must be v4-mapped, got {local}"
            );
        }

        // A stream socket has no association to dissolve this way.
        assert_eq!(net.disconnect(tcp(&net)), Err(errno::EOPNOTSUPP));
    }

    /// A datagram socket bound to a port keeps it across a reconnect and a
    /// disconnect, as on Linux (macOS replaces the host socket for both).
    #[test]
    fn datagram_rebind_keeps_bound_port() {
        let port = std::net::UdpSocket::bind(loopback(0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let net = Net::new(
            NetworkPolicy::AllowAll,
            Some(ListenPorts::from_ports([port])),
        );
        let s = udp(&net);
        net.setsockopt(s, SOL_SOCKET, SO_REUSEADDR, 1).unwrap();
        net.bind(s, loopback(port)).unwrap();
        net.connect(s, loopback(9)).unwrap();
        net.connect(s, loopback(10)).unwrap();
        net.connect(s, loopback(10)).unwrap();
        assert_eq!(net.peer_addr(s).unwrap().port(), 10);
        assert_eq!(net.local_addr(s).unwrap().port(), port);
        // Windows takes SO_REUSEADDR without setting it (see `setsockopt`).
        #[cfg(not(windows))]
        assert_ne!(net.getsockopt(s, SOL_SOCKET, SO_REUSEADDR).unwrap(), 0);
        net.disconnect(s).unwrap();
        assert_eq!(net.local_addr(s).unwrap().port(), port);
        net.connect(s, loopback(11)).unwrap();
        assert_eq!(net.local_addr(s).unwrap().port(), port);

        // A reconnect that fails (an IPv6 peer for an IPv4 socket) leaves
        // the socket connected where it was.
        let v6 = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 9);
        assert!(net.connect(s, v6).is_err());
        assert_eq!(net.peer_addr(s).unwrap().port(), 11);
        assert_eq!(net.local_addr(s).unwrap().port(), port);
        net.send(s, b"still connected").unwrap();
    }

    #[test]
    fn connection_refused_is_linux_errno() {
        let net = open_net();
        // Bound but not listening: a SYN is answered with RST.  macOS drops
        // it instead, so there the port is freed and nothing holds it.
        let closed = tcp(&net);
        net.bind(closed, loopback(0)).unwrap();
        let addr = net.local_addr(closed).unwrap();
        #[cfg(target_vendor = "apple")]
        net.close(closed).unwrap();
        let c = tcp(&net);
        assert_eq!(connect_blocking(&net, c, addr), Err(errno::ECONNREFUSED));
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

    /// The filter decides a DNS question sent on a connected socket with
    /// the socket table unlocked: it is the embedder's code.
    #[test]
    fn filters_decide_connected_dns_sends_unlocked() {
        use std::sync::OnceLock;
        use std::sync::atomic::{AtomicBool, Ordering};

        use crate::net_policy::{Destination, Filter, NetworkFilter};

        struct Probe {
            net: Arc<OnceLock<Arc<Net>>>,
            asked_unlocked: Arc<AtomicBool>,
        }
        impl NetworkFilter for Probe {
            fn allows_name(&self, _: &str) -> bool {
                let unlocked = self.net.get().is_some_and(|n| n.table.try_lock().is_ok());
                self.asked_unlocked.store(unlocked, Ordering::SeqCst);
                true
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                true
            }
        }

        let cell = Arc::new(OnceLock::new());
        let asked_unlocked = Arc::new(AtomicBool::new(false));
        let filter = Filter::new(Probe {
            net: cell.clone(),
            asked_unlocked: asked_unlocked.clone(),
        });
        let net = Arc::new(Net::new(NetworkPolicy::Filter(filter), None));
        cell.set(net.clone()).ok();
        let u = udp(&net);
        // A documentation address: connecting a datagram socket sends
        // nothing, and what is sent after goes nowhere.
        net.connect(u, "192.0.2.1:53".parse().unwrap()).unwrap();
        let mut query = vec![0x12, 0x34, 0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0];
        query.extend_from_slice(b"\x07example\x03com\x00\x00\x01\x00\x01");
        let _ = net.send(u, &query);
        assert!(asked_unlocked.load(Ordering::SeqCst));
    }

    /// A sendto on a connected stream socket goes to its peer, and is
    /// judged by it, not by the address it names.
    #[test]
    fn stream_sendto_goes_to_and_is_judged_by_the_peer() {
        use std::io::Read as _;
        let net = Arc::new(Net::new(NetworkPolicy::AllowAll, None));
        let inbox = Arc::new(Mutex::new(crate::async_calls::Inbox::default()));
        let listener = std::net::TcpListener::bind(loopback(0)).unwrap();
        let s = tcp(&net);
        match net.connect(s, listener.local_addr().unwrap()) {
            Ok(()) | Err(errno::EINPROGRESS) => {}
            Err(e) => panic!("connect: {e}"),
        }
        let (mut peer, _) = listener.accept().unwrap();
        wait(&net, s, POLLOUT);
        // An address every policy refuses: what is named does not matter.
        let metadata: SocketAddr = "169.254.169.254:80".parse().unwrap();
        assert_eq!(net.sendto(s, b"hi", metadata), Ok(2));
        let mut got = [0u8; 2];
        peer.read_exact(&mut got).unwrap();
        assert_eq!(&got, b"hi");
        assert_eq!(
            net.sendto_reply(s, b"hi".to_vec(), metadata, &inbox),
            int_reply(2)
        );
        peer.read_exact(&mut got).unwrap();
        assert_eq!(&got, b"hi");
    }

    #[test]
    fn policy_denies_connect_and_sendto() {
        // An allow-list always blocks loopback.
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let net = Net::new(NetworkPolicy::AllowList(al), None);
        let t = tcp(&net);
        assert_eq!(net.connect(t, loopback(80)), Err(errno::EACCES));
        let u = udp(&net);
        assert_eq!(net.sendto(u, b"x", loopback(80)), Err(errno::EACCES));
    }

    #[test]
    fn restricted_datagram_connect_does_not_authorize_payloads() {
        use crate::net_policy::{BlockList, Destination, Filter, NetworkFilter};
        use std::net::UdpSocket;

        struct DenyAll;
        impl NetworkFilter for DenyAll {
            fn allows_name(&self, _: &str) -> bool {
                false
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                false
            }
        }

        let policies = [
            NetworkPolicy::AllowList(AllowList::empty()),
            NetworkPolicy::BlockList(BlockList::from_hosts(&["192.0.2.1"]).unwrap()),
            NetworkPolicy::Filter(Filter::new(DenyAll)),
        ];
        for policy in policies {
            let net = Arc::new(Net::new(policy, None));
            let inbox = Arc::new(Mutex::new(crate::async_calls::Inbox::default()));
            let first = UdpSocket::bind(loopback(0)).unwrap();
            let second = UdpSocket::bind(loopback(0)).unwrap();
            first.set_nonblocking(true).unwrap();
            second.set_nonblocking(true).unwrap();
            let s = udp(&net);
            for peer in [&first, &second] {
                let addr = peer.local_addr().unwrap();
                net.connect(s, addr).unwrap();
                assert_eq!(net.connect_reply(s, addr, &inbox), int_reply(0));
                assert_eq!(net.peer_addr(s).unwrap(), addr);
                assert_eq!(net.table().meta(s).unwrap().peer, Some(addr));
                assert!(net.local_addr(s).unwrap().ip().is_loopback());
                assert_ne!(net.local_addr(s).unwrap().port(), 0);
                assert_eq!(net.send(s, b"blocked"), Err(errno::EACCES));
                assert_eq!(net.send(s, b""), Err(errno::EACCES));
                assert_eq!(net.sendto(s, b"blocked", addr), Err(errno::EACCES));
                assert_eq!(
                    net.send_reply(s, b"blocked".to_vec(), &inbox),
                    int_reply(-errno::EACCES)
                );
                assert_eq!(
                    net.sendto_reply(s, b"blocked".to_vec(), addr, &inbox),
                    int_reply(-errno::EACCES)
                );
                assert_eq!(net.recvfrom(s, 64), Err(errno::EAGAIN));
            }
            let previous = net.peer_addr(s).unwrap();
            assert!(net.connect(s, "[::1]:9".parse().unwrap()).is_err());
            assert_eq!(net.peer_addr(s).unwrap(), previous);
            assert_eq!(net.send(s, b"still blocked"), Err(errno::EACCES));

            net.disconnect(s).unwrap();
            assert_eq!(net.peer_addr(s), Err(errno::ENOTCONN));
            assert_eq!(net.table().meta(s).unwrap().peer, None);
            assert!(net.send(s, b"no peer").is_err());
            let addr = first.local_addr().unwrap();
            assert_eq!(net.sendto(s, b"blocked", addr), Err(errno::EACCES));
            net.connect(s, addr).unwrap();
            assert_eq!(net.send(s, b"blocked again"), Err(errno::EACCES));
            for peer in [&first, &second] {
                assert_eq!(
                    peer.recv_from(&mut [0u8; 64]).unwrap_err().kind(),
                    std::io::ErrorKind::WouldBlock
                );
            }
            net.close(s).unwrap();
            assert_eq!(net.send(s, b"closed"), Err(errno::EBADF));
        }
    }

    #[test]
    fn restricted_ipv6_datagram_probes_cannot_send_to_host() {
        use std::net::UdpSocket;

        let net = Net::new(NetworkPolicy::AllowList(AllowList::empty()), None);
        let Ok(peer) = UdpSocket::bind("[::1]:0") else {
            eprintln!("SKIP: no IPv6 loopback");
            return;
        };
        peer.set_nonblocking(true).unwrap();
        let s = net.socket(AF_INET6, SOCK_DGRAM, 17).unwrap();
        let addr = peer.local_addr().unwrap();
        net.connect(s, addr).unwrap();
        assert_eq!(net.peer_addr(s).unwrap(), addr);
        assert_eq!(net.send(s, b"blocked"), Err(errno::EACCES));
        assert_eq!(net.sendto(s, b"blocked", addr), Err(errno::EACCES));
        assert_eq!(
            peer.recv_from(&mut [0u8; 64]).unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        net.disconnect(s).unwrap();

        let v4 = UdpSocket::bind(loopback(0)).unwrap();
        v4.set_nonblocking(true).unwrap();
        let mapped = SocketAddr::new(
            Ipv4Addr::LOCALHOST.to_ipv6_mapped().into(),
            v4.local_addr().unwrap().port(),
        );
        net.connect(s, mapped).unwrap();
        assert_eq!(net.send(s, b"mapped"), Err(errno::EACCES));
        assert_eq!(net.sendto(s, b"mapped", mapped), Err(errno::EACCES));
        assert_eq!(
            v4.recv_from(&mut [0u8; 64]).unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }

    #[test]
    fn datagram_probes_do_not_authorize_unlisted_or_metadata_destinations() {
        let net = Net::new(NetworkPolicy::AllowList(AllowList::empty()), None);
        for peer in ["192.0.2.1:9", "169.254.169.254:80", "0.0.0.0:9", "[::1]:9"] {
            let addr = peer.parse().unwrap();
            let s = udp(&net);
            // Routing and unspecified-peer behavior belong to the host.
            // An incompatible family also exercises real connect failure.
            let control =
                net::socket_with(AddressFamily::INET, SocketType::DGRAM, socket_flags(), None)
                    .unwrap();
            rustix::io::ioctl_fionbio(&control, true).unwrap();
            let expected = net::connect(&control, &addr).map_err(from_rustix);
            assert_eq!(net.connect(s, addr), expected, "UDP connect to {addr}");
            assert_eq!(net.sendto(s, b"blocked", addr), Err(errno::EACCES));
            if expected.is_ok() {
                assert_eq!(net.send(s, b"blocked"), Err(errno::EACCES));
                net.disconnect(s).unwrap();
            } else {
                assert_eq!(net.table().meta(s).unwrap().peer, None);
                assert_eq!(net.peer_addr(s), Err(errno::ENOTCONN));
                assert!(net.send(s, b"no peer").is_err());
            }
            net.reset();
            assert_eq!(net.send(s, b"reset"), Err(errno::EBADF));
            assert_eq!(net.poll(&[(s, POLLIN | POLLOUT)], 0), (1, vec![POLLNVAL]));
        }
    }

    #[test]
    fn connected_datagram_sends_recheck_the_filter_unlocked() {
        use crate::net_policy::{Destination, Filter, NetworkFilter};
        use std::sync::OnceLock;
        use std::sync::atomic::{AtomicBool, Ordering};

        struct Switch {
            net: Arc<OnceLock<Arc<Net>>>,
            allow: Arc<AtomicBool>,
        }
        impl NetworkFilter for Switch {
            fn allows_name(&self, name: &str) -> bool {
                name == "allowed.test"
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                assert!(self.net.get().unwrap().table.try_lock().is_ok());
                self.allow.load(Ordering::SeqCst)
            }
        }
        let cell = Arc::new(OnceLock::new());
        let allow = Arc::new(AtomicBool::new(false));
        let net = Arc::new(Net::new(
            NetworkPolicy::Filter(Filter::new(Switch {
                net: cell.clone(),
                allow: allow.clone(),
            })),
            None,
        ));
        cell.set(net.clone()).ok();
        let s = udp(&net);
        let addr = "192.0.2.1:9".parse().unwrap();
        net.connect(s, addr).unwrap();
        assert_eq!(net.send(s, b"denied"), Err(errno::EACCES));
        allow.store(true, Ordering::SeqCst);
        assert_eq!(net.send(s, b"allowed"), Ok(7));
        allow.store(false, Ordering::SeqCst);
        assert_eq!(net.send(s, b"revoked"), Err(errno::EACCES));
        assert_eq!(net.sendto(s, b"revoked", addr), Err(errno::EACCES));

        allow.store(true, Ordering::SeqCst);
        let resolver = "192.0.2.1:53".parse().unwrap();
        net.connect(s, resolver).unwrap();
        let mut query = vec![0x12, 0x34, 0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0];
        query.extend_from_slice(b"\x06denied\x04test\x00\x00\x01\x00\x01");
        for data in [b"not DNS".as_slice(), query.as_slice()] {
            assert_eq!(net.send(s, data), Err(errno::EACCES));
            assert_eq!(net.sendto(s, data, resolver), Err(errno::EACCES));
        }
    }

    /// What listen ports alone give a guest (GuestConfig::new): it serves
    /// connections from outside and opens none of its own, not even to
    /// its own listener or a resolver.
    #[test]
    fn empty_allow_list_serves_but_reaches_nothing() {
        let net = Net::new(
            NetworkPolicy::AllowList(AllowList::empty()),
            Some(ListenPorts::all()),
        );
        let srv = tcp(&net);
        net.bind(srv, loopback(0)).unwrap();
        net.listen(srv, 4).unwrap();
        let addr = net.local_addr(srv).unwrap();

        let _outside = std::net::TcpStream::connect(addr).unwrap();
        assert_ne!(wait(&net, srv, POLLIN) & POLLIN, 0);
        net.accept(srv).unwrap();

        let cli = tcp(&net);
        assert_eq!(net.connect(cli, addr), Err(errno::EACCES));
        let u = udp(&net);
        let resolver: SocketAddr = "8.8.8.8:53".parse().unwrap();
        assert_eq!(net.sendto(u, b"x", resolver), Err(errno::EACCES));
    }

    #[test]
    fn listen_ports_gate_bind() {
        let net = Net::new(
            NetworkPolicy::AllowAll,
            Some(ListenPorts::from_ports([8080])),
        );
        let s = tcp(&net);
        assert_eq!(net.bind(s, loopback(19_999)), Err(errno::EACCES));
        // Ephemeral binds are always allowed.
        net.bind(s, loopback(0)).unwrap();
    }

    /// Without listen ports every bind but an ephemeral one is refused,
    /// under a policy that allows everything: the guest can serve only on
    /// a port the host picks.
    #[test]
    fn no_listen_ports_rejects_every_bind() {
        let net = Net::new(NetworkPolicy::AllowAll, None);
        let s = tcp(&net);
        assert_eq!(net.bind(s, loopback(8080)), Err(errno::EACCES));
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
