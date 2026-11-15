// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The `net_*` host functions, called with what a guest can send them.
//!
//! Each input is a run of calls under an allow list that lists nothing,
//! and no ports to listen on: the guest may serve on a port the host picks
//! (a bind to port 0), and reach nothing.  The run also plays the guest's
//! clients: from the host, over loopback, it connects and sends to the
//! guest's sockets.  On the host, a TCP listener and a UDP socket on
//! loopback stand for the services a guest must not reach; the guest is
//! given their addresses, among others.
//!
//! Every reply must be well formed, with a Linux errno for a failure; no
//! TCP connect may start, and no datagram be sent, as the policy allows
//! neither; the services must hear nothing; and the run must leave no file
//! descriptor open.  The waits are those of a guest and the host between
//! entries, with no time to wait (`net_poll` refuses any other), so a run
//! that takes long is a call that blocked.

#![no_main]

use std::collections::HashSet;
use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::OnceLock;
use std::time::Duration;

use hyperlight_common::flatbuffer_wrappers::function_types::{ParameterValue, ReturnValue};
use hyperlight_unikraft::fuzzing::HostCalls;
use hyperlight_unikraft::{AllowList, NetworkPolicy};
use hyperlight_unikraft_fuzz::{
    RawCall, bytes, call, check_error_reply, check_status, int, open_fds, reply_status,
};
use libfuzzer_sys::arbitrary::{self, Arbitrary};
use libfuzzer_sys::fuzz_target;

const AF_INET: i32 = 2;
const AF_INET6: i32 = 10;
const SOCK_STREAM: i32 = 1;
const SOCK_DGRAM: i32 = 2;
const SOL_SOCKET: i32 = 1;
const SO_TYPE: i32 = 3;
const EINPROGRESS: i32 = 115;
const POLLNVAL: i16 = 0x020;
/// Peers the run keeps at once.
const MAX_PEERS: usize = 16;

/// The services the guest must not reach, on loopback.
struct Services {
    tcp: TcpListener,
    udp: UdpSocket,
}

fn services() -> &'static Services {
    static SERVICES: OnceLock<Services> = OnceLock::new();
    SERVICES.get_or_init(|| {
        let tcp = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        tcp.set_nonblocking(true).unwrap();
        let udp = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        udp.set_nonblocking(true).unwrap();
        Services { tcp, udp }
    })
}

fuzz_target!(
    init: {
        services();
    },
    |ops: Vec<Op>| run(ops)
);

/// A socket: one the guest was given, by its place among them, or any fd.
#[derive(Arbitrary, Debug, Clone, Copy)]
enum Fd {
    Given(u8),
    Any(i32),
}

#[derive(Arbitrary, Debug, Clone, Copy)]
enum Family {
    Inet,
    Inet6,
    Any(i32),
}

impl Family {
    fn get(self) -> i32 {
        match self {
            Self::Inet => AF_INET,
            Self::Inet6 => AF_INET6,
            Self::Any(f) => f,
        }
    }
}

#[derive(Arbitrary, Debug, Clone, Copy)]
enum SockType {
    Stream,
    Dgram,
    Any(i32),
}

impl SockType {
    fn get(self) -> i32 {
        match self {
            Self::Stream => SOCK_STREAM,
            Self::Dgram => SOCK_DGRAM,
            Self::Any(t) => t,
        }
    }
}

/// A protocol: the type's default, TCP, UDP, ICMP, or any.
#[derive(Arbitrary, Debug, Clone, Copy)]
enum Protocol {
    Default,
    Tcp,
    Udp,
    Icmp,
    Any(i32),
}

impl Protocol {
    fn get(self) -> i32 {
        match self {
            Self::Default => 0,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Icmp => 1,
            Self::Any(p) => p,
        }
    }
}

/// An address, as the guest writes it: of the services, the host, a
/// documentation range, the metadata service, or any text.
#[derive(Arbitrary, Debug)]
enum Ip {
    Loopback,
    Unspecified,
    Loopback6,
    Unspecified6,
    /// `::ffff:127.0.0.1`, which reaches the services through a dual-stack
    /// socket.
    MappedLoopback,
    Documentation,
    Metadata,
    Any(String),
}

impl Ip {
    fn text(&self) -> String {
        match self {
            Self::Loopback => "127.0.0.1".into(),
            Self::Unspecified => "0.0.0.0".into(),
            Self::Loopback6 => "::1".into(),
            Self::Unspecified6 => "::".into(),
            Self::MappedLoopback => "::ffff:127.0.0.1".into(),
            Self::Documentation => "192.0.2.1".into(),
            Self::Metadata => "169.254.169.254".into(),
            Self::Any(s) => s.clone(),
        }
    }
}

#[derive(Arbitrary, Debug)]
enum Port {
    /// Any port, for a bind.
    Zero,
    TcpService,
    UdpService,
    Dns,
    Any(i32),
}

impl Port {
    fn get(&self) -> i32 {
        match self {
            Self::Zero => 0,
            Self::TcpService => services().tcp.local_addr().unwrap().port().into(),
            Self::UdpService => services().udp.local_addr().unwrap().port().into(),
            Self::Dns => 53,
            Self::Any(p) => *p,
        }
    }
}

#[derive(Arbitrary, Debug)]
struct Addr {
    family: Family,
    ip: Ip,
    port: Port,
}

impl Addr {
    fn args(&self) -> [ParameterValue; 3] {
        [
            ParameterValue::Int(self.family.get()),
            ParameterValue::String(self.ip.text()),
            ParameterValue::Int(self.port.get()),
        ]
    }
}

/// A socket option: one the host knows, by its place among them, or any.
#[derive(Arbitrary, Debug, Clone, Copy)]
enum SockOpt {
    Known(u8),
    Any(i32, i32),
}

/// The (level, name) pairs the host handles, and three it refuses.
const OPTIONS: &[(i32, i32)] = &[
    (1, 2),   // SO_REUSEADDR
    (1, 3),   // SO_TYPE
    (1, 4),   // SO_ERROR
    (1, 6),   // SO_BROADCAST
    (1, 7),   // SO_SNDBUF
    (1, 8),   // SO_RCVBUF
    (1, 9),   // SO_KEEPALIVE
    (1, 10),  // SO_OOBINLINE
    (1, 13),  // SO_LINGER
    (1, 15),  // SO_REUSEPORT
    (1, 20),  // SO_RCVTIMEO
    (1, 21),  // SO_SNDTIMEO
    (1, 30),  // SO_ACCEPTCONN
    (1, 38),  // SO_PROTOCOL
    (1, 39),  // SO_DOMAIN
    (0, 1),   // IP_TOS
    (0, 2),   // IP_TTL
    (0, 11),  // IP_RECVERR
    (0, 33),  // IP_MULTICAST_TTL
    (0, 34),  // IP_MULTICAST_LOOP
    (6, 1),   // TCP_NODELAY
    (6, 4),   // TCP_KEEPIDLE
    (6, 5),   // TCP_KEEPINTVL
    (6, 6),   // TCP_KEEPCNT
    (41, 16), // IPV6_UNICAST_HOPS
    (41, 19), // IPV6_MULTICAST_LOOP
    (41, 25), // IPV6_RECVERR
    (41, 26), // IPV6_V6ONLY
];

impl SockOpt {
    fn get(self) -> (i32, i32) {
        match self {
            Self::Known(i) => OPTIONS[usize::from(i) % OPTIONS.len()],
            Self::Any(level, name) => (level, name),
        }
    }
}

#[derive(Arbitrary, Debug)]
enum Op {
    Socket(Family, SockType, Protocol),
    Bind(Fd, Addr),
    Listen(Fd, i32),
    Accept(Fd),
    Connect(Fd, Addr),
    Send(Fd, Vec<u8>),
    SendTo(Fd, Vec<u8>, Addr),
    RecvFrom(Fd, i32),
    Shutdown(Fd, i32),
    Close(Fd),
    Disconnect(Fd),
    GetPeerName(Fd),
    GetSockName(Fd),
    GetSockOpt(Fd, SockOpt),
    SetSockOpt(Fd, SockOpt, i32),
    /// `net_poll` with these (fd, events) pairs, then these bytes, as the
    /// guest packs them, and this timeout.
    Poll(Vec<(Fd, i16)>, Vec<u8>, i32),
    /// The host's wait between entries.
    Wait,
    /// The host's in-place restore.
    Reset,
    /// From the host, connect to the guest's socket.
    PeerConnect(Fd),
    /// From the host, send a datagram to the guest's socket.
    PeerDatagram(Fd, Vec<u8>),
    /// Send, or read, on a connection the host made.
    PeerSend(u8, Vec<u8>),
    PeerRead(u8),
    PeerClose(u8),
    Raw(RawCall),
}

/// What the run knows of the guest's sockets.
#[derive(Default)]
struct Guest {
    /// Every fd the guest was given, in order, closed or not.
    given: Vec<i32>,
    /// The fds `net_accept` gave, still open: the only sockets a send may
    /// leave from.
    accepted: HashSet<i32>,
    peers: Vec<TcpStream>,
}

impl Guest {
    fn fd(&self, fd: Fd) -> i32 {
        match fd {
            Fd::Given(i) if !self.given.is_empty() => self.given[usize::from(i) % self.given.len()],
            Fd::Given(i) => i32::from(i),
            Fd::Any(fd) => fd,
        }
    }

    fn give(&mut self, fd: i32, accepted: bool) {
        // A new socket may reuse a closed one's fd.
        self.accepted.remove(&fd);
        if accepted {
            self.accepted.insert(fd);
        }
        self.given.push(fd);
    }

    fn peer(&mut self, i: u8) -> Option<&mut TcpStream> {
        let n = self.peers.len();
        (n > 0).then(|| &mut self.peers[usize::from(i) % n])
    }
}

fn run(ops: Vec<Op>) {
    let fds = open_fds();
    {
        let calls = HostCalls::new(
            &[],
            Some(NetworkPolicy::AllowList(AllowList::empty())),
            None,
        )
        .unwrap();
        let mut guest = Guest::default();
        for op in ops {
            step(&calls, &mut guest, op);
        }
        let mut buf = [0u8; 64];
        match services().tcp.accept() {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            other => panic!("the guest reached the TCP service: {other:?}"),
        }
        match services().udp.recv_from(&mut buf) {
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            other => panic!("the guest reached the UDP service: {other:?}"),
        }
    }
    assert_eq!(open_fds(), fds, "the run left file descriptors open");
}

fn step(calls: &HostCalls, guest: &mut Guest, op: Op) {
    let int_call = |name: &str, args| int(name, call(calls, name, args));
    let bytes_call = |name: &str, args| bytes(name, call(calls, name, args));
    let fd = |f: Fd| ParameterValue::Int(guest.fd(f));
    match op {
        Op::Socket(family, ty, proto) => {
            let args = vec![
                ParameterValue::Int(family.get()),
                ParameterValue::Int(ty.get()),
                ParameterValue::Int(proto.get()),
            ];
            let new = int_call("net_socket", args);
            if new >= 0 {
                guest.give(new, false);
            }
        }
        Op::Bind(f, addr) => {
            let mut args = vec![fd(f)];
            args.extend(addr.args());
            int_call("net_bind", args);
        }
        Op::Listen(f, backlog) => {
            int_call("net_listen", vec![fd(f), ParameterValue::Int(backlog)]);
        }
        Op::Accept(f) => {
            let reply = bytes_call("net_accept", vec![fd(f)]);
            let status = reply_status("net_accept", &reply);
            if status < 0 {
                return check_error_reply("net_accept", &reply, status);
            }
            if reply.len() > 4 {
                check_addr("net_accept", &reply[4..], false);
            }
            guest.give(status, true);
        }
        Op::Connect(f, addr) => {
            let sock = guest.fd(f);
            let stream = sock_type(calls, sock) == Some(SOCK_STREAM);
            let mut args = vec![ParameterValue::Int(sock)];
            args.extend(addr.args());
            let r = int_call("net_connect", args);
            // A UDP connect sends nothing, and is not checked: its sends are.
            assert!(
                !stream || (r != 0 && r != -EINPROGRESS),
                "net_connect to {addr:?} started under a policy that allows nothing"
            );
        }
        Op::Send(f, data) => {
            let sock = guest.fd(f);
            let len = data.len();
            let sent = int_call(
                "net_send",
                vec![ParameterValue::Int(sock), ParameterValue::VecBytes(data)],
            );
            check_sent("net_send", guest, sock, sent, len);
        }
        Op::SendTo(f, data, addr) => {
            let sock = guest.fd(f);
            let len = data.len();
            let mut args = vec![ParameterValue::Int(sock), ParameterValue::VecBytes(data)];
            args.extend(addr.args());
            let sent = int_call("net_sendto", args);
            check_sent("net_sendto", guest, sock, sent, len);
        }
        Op::RecvFrom(f, len) => {
            let reply = bytes_call("net_recvfrom", vec![fd(f), ParameterValue::Int(len)]);
            let status = reply_status("net_recvfrom", &reply);
            if status < 0 {
                return check_error_reply("net_recvfrom", &reply, status);
            }
            let addr_len = check_addr("net_recvfrom", &reply[4..], true);
            let data = reply.len() - 4 - addr_len;
            assert_eq!(
                status as usize, data,
                "net_recvfrom: a length other than its data's"
            );
            assert!(
                data <= usize::try_from(len).unwrap_or(0),
                "net_recvfrom: {data} bytes, past the {len} asked"
            );
        }
        Op::Shutdown(f, how) => {
            int_call("net_shutdown", vec![fd(f), ParameterValue::Int(how)]);
        }
        Op::Close(f) => {
            let sock = guest.fd(f);
            if int_call("net_close", vec![ParameterValue::Int(sock)]) == 0 {
                guest.accepted.remove(&sock);
            }
        }
        Op::Disconnect(f) => {
            int_call("net_disconnect", vec![fd(f)]);
        }
        Op::GetPeerName(f) => name_reply(
            "net_getpeername",
            bytes_call("net_getpeername", vec![fd(f)]),
        ),
        Op::GetSockName(f) => name_reply(
            "net_getsockname",
            bytes_call("net_getsockname", vec![fd(f)]),
        ),
        Op::GetSockOpt(f, opt) => {
            let (level, name) = opt.get();
            let args = vec![fd(f), ParameterValue::Int(level), ParameterValue::Int(name)];
            // A value, which may be negative, or -errno: only the shape.
            match call(calls, "net_getsockopt", args) {
                ReturnValue::Int(_) => {}
                other => panic!("net_getsockopt: returned {other:?}, not an i32"),
            }
        }
        Op::SetSockOpt(f, opt, value) => {
            let (level, name) = opt.get();
            let args = vec![
                fd(f),
                ParameterValue::Int(level),
                ParameterValue::Int(name),
                ParameterValue::Int(value),
            ];
            int_call("net_setsockopt", args);
        }
        Op::Poll(entries, extra, timeout) => {
            let mut packed = Vec::new();
            for (f, events) in &entries {
                packed.extend(guest.fd(*f).to_le_bytes());
                packed.extend(events.to_le_bytes());
                packed.extend([0, 0]);
            }
            packed.extend(&extra);
            let count = packed.len() / 8;
            let reply = bytes_call(
                "net_poll",
                vec![
                    ParameterValue::VecBytes(packed.clone()),
                    ParameterValue::Int(timeout),
                ],
            );
            let ready = reply_status("net_poll", &reply);
            assert_eq!(reply.len(), 4 + 2 * count, "net_poll: a revents per entry");
            if timeout != 0 {
                assert!(ready < 0, "net_poll: a timeout of {timeout} ms was taken");
                return;
            }
            if ready >= 0 {
                assert!(
                    ready as usize <= count,
                    "net_poll: {ready} ready of {count}"
                );
            }
            // An fd the guest does not hold is POLLNVAL, and nothing else.
            for (i, revents) in reply[4..].chunks(2).enumerate() {
                let revents = i16::from_le_bytes([revents[0], revents[1]]);
                let entry = i32::from_le_bytes(packed[i * 8..i * 8 + 4].try_into().unwrap());
                if entry < 0 {
                    assert_eq!(revents, POLLNVAL, "net_poll: fd {entry}");
                }
            }
        }
        Op::Wait => {
            calls.net_wait(Duration::ZERO);
        }
        Op::Reset => {
            calls.net_reset();
            guest.accepted.clear();
        }
        Op::PeerConnect(f) => {
            if guest.peers.len() >= MAX_PEERS {
                return;
            }
            let Some(addr) = reachable(calls, guest.fd(f), SOCK_STREAM) else {
                return;
            };
            // On loopback a connect to a listener is done at once, and one
            // to anything else refused at once.
            if let Ok(peer) = TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
                peer.set_nonblocking(true).unwrap();
                guest.peers.push(peer);
            }
        }
        Op::PeerDatagram(f, data) => {
            let Some(addr) = reachable(calls, guest.fd(f), SOCK_DGRAM) else {
                return;
            };
            let local: IpAddr = if addr.is_ipv4() {
                Ipv4Addr::LOCALHOST.into()
            } else {
                Ipv6Addr::LOCALHOST.into()
            };
            if let Ok(peer) = UdpSocket::bind((local, 0)) {
                let _ = peer.send_to(&data, addr);
            }
        }
        Op::PeerSend(i, data) => {
            if let Some(peer) = guest.peer(i) {
                let _ = peer.write(&data);
            }
        }
        Op::PeerRead(i) => {
            if let Some(peer) = guest.peer(i) {
                let _ = peer.read(&mut [0u8; 4096]);
            }
        }
        Op::PeerClose(i) => {
            let n = guest.peers.len();
            if n > 0 {
                guest.peers.swap_remove(usize::from(i) % n);
            }
        }
        Op::Raw(raw) => {
            let Some((name, args, reply)) = raw.call(calls) else {
                return;
            };
            let status = match reply {
                ReturnValue::VecBytes(reply) => reply_status(&name, &reply),
                // net_getsockopt's value may be negative.
                ReturnValue::Int(v) if name == "net_getsockopt" => v,
                reply => int(&name, reply),
            };
            // The sockets it gave the guest, or took, as the typed calls'
            match (name.as_str(), args.first()) {
                ("net_socket", _) if status >= 0 => guest.give(status, false),
                ("net_accept", _) if status >= 0 => guest.give(status, true),
                ("net_close", Some(ParameterValue::Int(fd))) if status == 0 => {
                    guest.accepted.remove(fd);
                }
                _ => {}
            }
        }
    }
}

/// The type of the guest's socket `fd` (`SOCK_STREAM` or `SOCK_DGRAM`), as
/// `SO_TYPE` says, or `None` if the guest holds no such socket.
fn sock_type(calls: &HostCalls, fd: i32) -> Option<i32> {
    let args = vec![
        ParameterValue::Int(fd),
        ParameterValue::Int(SOL_SOCKET),
        ParameterValue::Int(SO_TYPE),
    ];
    match calls.call("net_getsockopt", args) {
        Ok(ReturnValue::Int(ty)) if ty >= 0 => Some(ty),
        _ => None,
    }
}

/// A send succeeds only on a connection the guest accepted: the policy
/// allows no destination, and an accepted connection's peer is a client.
fn check_sent(name: &str, guest: &Guest, fd: i32, sent: i32, len: usize) {
    check_status(name, sent);
    if sent >= 0 {
        assert!(
            guest.accepted.contains(&fd),
            "{name}: sent {sent} bytes from fd {fd}, which no client connected to"
        );
        assert!(sent as usize <= len, "{name}: sent {sent} of {len} bytes");
    }
}

/// A packed address, at the start of `rest`: family, port, length and the
/// address.  Returns its size.  With `then_data`, data follows it, and it
/// may be the empty address (family 0), which a stream socket's data has.
fn check_addr(name: &str, rest: &[u8], then_data: bool) -> usize {
    assert!(rest.len() >= 7, "{name}: an address cut short");
    let family = i32::from_le_bytes(rest[..4].try_into().unwrap());
    let len = usize::from(rest[6]);
    match (family, len) {
        (AF_INET, 4) | (AF_INET6, 16) => {}
        (0, 0) if then_data => {}
        _ => panic!("{name}: an address of family {family} and length {len}"),
    }
    assert!(rest.len() >= 7 + len, "{name}: an address cut short");
    if !then_data {
        assert_eq!(rest.len(), 7 + len, "{name}: bytes after the address");
    }
    7 + len
}

/// `net_getpeername`'s or `net_getsockname`'s reply: status, then an
/// address.
fn name_reply(name: &str, reply: Vec<u8>) {
    let status = reply_status(name, &reply);
    if status != 0 {
        assert!(status < 0, "{name}: status {status}");
        return check_error_reply(name, &reply, status);
    }
    check_addr(name, &reply[4..], false);
}

/// Where the host reaches the guest's socket `fd`, if one of type `ty`:
/// its bound port, on the loopback address of its family.  Of that type
/// only, as TCP and UDP ports are apart: a guest's TCP port may be the UDP
/// service's.
fn reachable(calls: &HostCalls, fd: i32, ty: i32) -> Option<SocketAddr> {
    if sock_type(calls, fd) != Some(ty) {
        return None;
    }
    let reply = match calls.call("net_getsockname", vec![ParameterValue::Int(fd)]) {
        Ok(ReturnValue::VecBytes(reply)) => reply,
        _ => return None,
    };
    if reply.len() < 11 || reply[..4] != [0; 4] {
        return None;
    }
    let family = i32::from_le_bytes(reply[4..8].try_into().unwrap());
    let port = u16::from_le_bytes([reply[8], reply[9]]);
    let ip: IpAddr = match family {
        AF_INET => Ipv4Addr::LOCALHOST.into(),
        AF_INET6 => Ipv6Addr::LOCALHOST.into(),
        _ => return None,
    };
    (port != 0).then_some(SocketAddr::new(ip, port))
}
