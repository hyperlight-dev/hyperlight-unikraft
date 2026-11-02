// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use common::{UNUSED_PORT, host_ip, net_probe, require_rootfs};
use hyperlight_unikraft::{
    AllowList, BlockList, Destination, Filter, NetworkFilter, NetworkPolicy, SandboxBuilder,
};

/// Networking disabled by default — guest socket calls fail because
/// every socket() is refused with EACCES, as a policy refuses a destination.
#[test]
fn net_policy_disabled_by_default() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.run(SOCKET_PROBE).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("socket: errno 13"),
        "expected socket creation to fail with EACCES when networking is disabled, got: {output:?}",
    );
}

/// Prints whether a socket can be made, and its errno when it cannot.
const SOCKET_PROBE: &str = r#"
import socket
try:
    socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    print('socket: ok')
except OSError as e:
    print('socket: errno', e.errno)
"#;

/// The net_* host functions exist on every path, so a snapshot saved under
/// a policy restores without one: the restore is accepted and the guest
/// simply has no network.
#[test]
fn snapshot_under_a_policy_restores_without_one() {
    let dir = common::temp_dir("net-then-none");
    let snap = dir.path().join("snap");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    sandbox.run(SOCKET_PROBE).unwrap();
    assert!(sandbox.drain_output().contains("socket: ok"));
    sandbox.snapshot_to(&snap).unwrap();
    drop(sandbox);

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .boot()
        .unwrap();
    sandbox.run(SOCKET_PROBE).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("socket: errno 13"),
        "expected no network after a restore without a policy, got: {output:?}",
    );
}

/// And the other way round: a warm snapshot saved without a policy gets
/// the network the restore names.
#[test]
fn snapshot_without_a_policy_restores_under_one() {
    let dir = common::temp_dir("none-then-net");
    let snap = dir.path().join("snap");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .boot()
        .unwrap();
    sandbox.snapshot_to(&snap).unwrap();
    drop(sandbox);

    let mut sandbox = SandboxBuilder::from_snapshot_dir(&snap)
        .unwrap()
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    sandbox.run(SOCKET_PROBE).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("socket: ok"),
        "expected the network given at restore, got: {output:?}",
    );
}

/// AllowAll blocks link-local addresses (cloud metadata service).
#[test]
fn net_policy_allowall_blocks_link_local() {
    let output = net_probe(Some(NetworkPolicy::AllowAll), None, "169.254.169.254", 80);
    assert!(
        output.contains("TCP_BLOCKED"),
        "expected link-local 169.254.169.254 to be TCP-blocked even with AllowAll, got: {output:?}",
    );
    assert!(
        output.contains("UDP_BLOCKED"),
        "expected link-local 169.254.169.254 to be UDP-blocked even with AllowAll, got: {output:?}",
    );
}

/// AllowAll permits loopback — needed for intra-guest server+client
/// patterns.  In the hostsock model all guest sockets are host sockets,
/// so blocking loopback would prevent guest-internal networking.
/// AllowList/BlockList still block loopback (defense in depth).
#[test]
fn net_policy_allowall_permits_loopback() {
    let output = net_probe(
        Some(NetworkPolicy::AllowAll),
        None,
        "127.0.0.1",
        UNUSED_PORT,
    );
    // Loopback should pass the policy check.  TCP gives ECONNREFUSED
    // (nothing listening), not EACCES.  UDP sendto succeeds.
    assert!(
        !output.contains("TCP_BLOCKED"),
        "expected loopback 127.0.0.1 to be TCP-permitted by AllowAll, got: {output:?}",
    );
    assert!(
        !output.contains("UDP_BLOCKED"),
        "expected loopback 127.0.0.1 to be UDP-permitted by AllowAll, got: {output:?}",
    );
}

/// AllowList permits connections to a listed IP.
///
/// Uses the host's own IP on a non-listening port — the kernel RSTs
/// immediately (ECONNREFUSED), proving the policy check passed without
/// waiting for a remote TCP handshake.
#[test]
fn net_policy_allowlist_permits() {
    let ip = host_ip();
    let al = AllowList::from_hosts(&[ip.as_str()]).unwrap();
    let output = net_probe(Some(NetworkPolicy::AllowList(al)), None, &ip, UNUSED_PORT);
    assert!(
        !output.contains("TCP_BLOCKED"),
        "expected allowlisted IP {ip} to pass TCP policy check, got: {output:?}",
    );
    assert!(
        !output.contains("UDP_BLOCKED"),
        "expected allowlisted IP {ip} to pass UDP policy check, got: {output:?}",
    );
}

/// AllowList blocks connections to an unlisted IP.
#[test]
fn net_policy_allowlist_blocks() {
    let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
    let output = net_probe(Some(NetworkPolicy::AllowList(al)), None, "1.2.3.4", 80);
    assert!(
        output.contains("TCP_BLOCKED"),
        "expected unlisted IP 1.2.3.4 to be TCP-blocked by AllowList, got: {output:?}",
    );
    assert!(
        output.contains("UDP_BLOCKED"),
        "expected unlisted IP 1.2.3.4 to be UDP-blocked by AllowList, got: {output:?}",
    );
}

#[test]
fn restricted_datagram_probes_cannot_reach_host_services() {
    use std::net::{TcpListener, UdpSocket};

    let mut receivers = Vec::new();
    let mut listeners = Vec::new();
    let mut targets = Vec::new();
    for (family, ip) in [(2, "127.0.0.1"), (10, "::1")] {
        let receiver = match UdpSocket::bind((ip, 0)) {
            Ok(s) => s,
            Err(e) if family == 10 => {
                eprintln!("SKIP: no IPv6 loopback: {e}");
                continue;
            }
            Err(e) => panic!("bind IPv4 loopback: {e}"),
        };
        let listener = TcpListener::bind((ip, 0)).unwrap();
        receiver.set_nonblocking(true).unwrap();
        listener.set_nonblocking(true).unwrap();
        targets.push((
            family,
            ip,
            receiver.local_addr().unwrap().port(),
            listener.local_addr().unwrap().port(),
        ));
        receivers.push(receiver);
        listeners.push(listener);
    }
    let code = format!(
        r#"
import errno, os, socket
for family, ip, udp_port, tcp_port in {targets:?}:
    s = socket.socket(family, socket.SOCK_DGRAM)
    peer = (ip, udp_port)
    s.connect(peer)
    for send in (
        lambda: s.send(b'send'),
        lambda: os.write(s.fileno(), b'write'),
        lambda: s.sendto(b'sendto', peer),
        lambda: s.sendmsg([b'send', b'msg']),
        lambda: s.sendmsg([b'named'], [], 0, peer),
    ):
        try:
            send()
        except OSError as e:
            assert e.errno == errno.EACCES, repr(e)
        else:
            raise AssertionError('host loopback UDP payload permitted')
    s.close()
    t = socket.socket(family, socket.SOCK_STREAM)
    try:
        t.connect((ip, tcp_port))
    except OSError as e:
        assert e.errno == errno.EACCES, repr(e)
    else:
        raise AssertionError('host loopback TCP permitted')
    t.close()
print('loopback probes ok; all payload paths refused')
"#
    );
    let snap = common::temp_dir("restricted-loopback");
    let mut sandbox = SandboxBuilder::from_initrd(require_rootfs("python"))
        .scratch_mb(256)
        .network(NetworkPolicy::AllowList(AllowList::empty()))
        .boot()
        .unwrap();
    sandbox.snapshot_to(snap.path()).unwrap();
    sandbox.run(code.as_str()).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("loopback probes ok; all payload paths refused"),
        "{output:?}"
    );
    drop(sandbox);
    let mut sandbox = SandboxBuilder::from_snapshot_dir(snap.path())
        .unwrap()
        .network(NetworkPolicy::AllowList(AllowList::empty()))
        .boot()
        .unwrap();
    sandbox.run(code.as_str()).unwrap();
    let output = sandbox.drain_output();
    assert!(
        output.contains("loopback probes ok; all payload paths refused"),
        "{output:?}"
    );
    for receiver in receivers {
        assert_eq!(
            receiver.recv_from(&mut [0u8; 64]).unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }
    for listener in listeners {
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
    }
}

/// BlockList blocks connections to a listed IP.
#[test]
fn net_policy_blocklist_blocks() {
    let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
    let output = net_probe(Some(NetworkPolicy::BlockList(bl)), None, "1.2.3.4", 80);
    assert!(
        output.contains("TCP_BLOCKED"),
        "expected listed IP 1.2.3.4 to be TCP-blocked by BlockList, got: {output:?}",
    );
    assert!(
        output.contains("UDP_BLOCKED"),
        "expected listed IP 1.2.3.4 to be UDP-blocked by BlockList, got: {output:?}",
    );
}

/// BlockList permits connections to an unlisted IP.
///
/// Uses the host's own IP (same as allowlist_permits) for instant response.
#[test]
fn net_policy_blocklist_permits() {
    let ip = host_ip();
    let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
    let output = net_probe(Some(NetworkPolicy::BlockList(bl)), None, &ip, UNUSED_PORT);
    assert!(
        !output.contains("TCP_BLOCKED"),
        "expected unlisted IP {ip} to pass TCP BlockList policy check, got: {output:?}",
    );
    assert!(
        !output.contains("UDP_BLOCKED"),
        "expected unlisted IP {ip} to pass UDP BlockList policy check, got: {output:?}",
    );
}

/// A guest under an allow list of one name can resolve and reach that
/// name; it cannot ask DNS about another name, and it cannot connect to
/// an address it was never given.  Needs the internet, as
/// `python_http_get` does.
#[test]
fn net_policy_allowlist_by_name_end_to_end() {
    let rootfs = require_rootfs("python");
    let al = AllowList::from_hosts(&["example.com"]).unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowList(al))
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno, socket, urllib.request\n",
            "print('listed', urllib.request.urlopen('http://example.com', timeout=15).status)\n",
            "try:\n",
            "    socket.getaddrinfo('github.com', 80)\n",
            "    print('unlisted resolved')\n",
            "except OSError:\n",
            "    print('unlisted refused')\n",
            "s = socket.socket()\n",
            "s.settimeout(5)\n",
            "try:\n",
            "    s.connect(('1.1.1.1', 80))\n",
            "    print('literal connected')\n",
            "except OSError as e:\n",
            "    print('literal', 'refused' if e.errno == errno.EACCES else e)\n",
        ))
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("listed 200"), "{out:?}");
    assert!(out.contains("unlisted refused"), "{out:?}");
    assert!(out.contains("literal refused"), "{out:?}");
}

/// A filter that allows one name on one port, while `on`.
struct OneNameOnePort {
    name: &'static str,
    port: u16,
    on: std::sync::atomic::AtomicBool,
}

impl NetworkFilter for OneNameOnePort {
    fn allows_name(&self, name: &str) -> bool {
        self.on.load(std::sync::atomic::Ordering::Relaxed) && name == self.name
    }
    fn allows(&self, d: &Destination<'_>) -> bool {
        d.addr.port() == self.port && d.names.iter().any(|n| self.allows_name(n))
    }
}

/// A guest under a filter reaches the name it allows on the port it
/// allows, and nothing else: not that name on another port, not another
/// name, not a literal address.  Turning the filter off applies at the
/// next run, without booting again.  Needs the internet.
#[test]
fn net_policy_filter_end_to_end() {
    let rootfs = require_rootfs("python");
    let filter = std::sync::Arc::new(OneNameOnePort {
        name: "example.com",
        port: 80,
        on: true.into(),
    });
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::Filter(Filter::from_arc(filter.clone())))
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno, socket, urllib.request\n",
            "print('allowed', urllib.request.urlopen('http://example.com', timeout=15).status)\n",
            "def connect(host, port):\n",
            "    s = socket.socket()\n",
            "    s.settimeout(5)\n",
            "    try:\n",
            "        s.connect((host, port))\n",
            "        return 'connected'\n",
            "    except OSError as e:\n",
            "        return 'refused' if e.errno == errno.EACCES else repr(e)\n",
            "    finally:\n",
            "        s.close()\n",
            "print('other port', connect('example.com', 443))\n",
            "print('literal', connect('1.1.1.1', 80))\n",
            "try:\n",
            "    socket.getaddrinfo('github.com', 80)\n",
            "    print('other name resolved')\n",
            "except OSError:\n",
            "    print('other name refused')\n",
        ))
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("allowed 200"), "{out:?}");
    assert!(out.contains("other port refused"), "{out:?}");
    assert!(out.contains("literal refused"), "{out:?}");
    assert!(out.contains("other name refused"), "{out:?}");

    filter.on.store(false, std::sync::atomic::Ordering::Relaxed);
    sandbox
        .run(concat!(
            "import urllib.request\n",
            "try:\n",
            "    urllib.request.urlopen('http://example.com', timeout=15)\n",
            "    print('still reached')\n",
            "except OSError:\n",
            "    print('now refused')\n",
        ))
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("now refused"), "{out:?}");
}

/// A guest under a block list of one name cannot resolve it, cannot
/// connect to the address it has now even without resolving, and can
/// reach anything else.  Needs the internet.
#[test]
fn net_policy_blocklist_by_name_end_to_end() {
    use std::net::ToSocketAddrs;
    let rootfs = require_rootfs("python");
    // What the name resolves to right now, for the guest that has the
    // address without asking.
    let blocked_ip = ("example.com", 80)
        .to_socket_addrs()
        .unwrap()
        .find(|a| a.is_ipv4())
        .unwrap()
        .ip();
    let bl = BlockList::from_hosts(&["example.com"]).unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::BlockList(bl))
        .boot()
        .unwrap();
    let code = format!(
        concat!(
            "import errno, socket, urllib.request\n",
            "try:\n",
            "    urllib.request.urlopen('http://example.com', timeout=15)\n",
            "    print('blocked name reached')\n",
            "except Exception as e:\n",
            "    print('blocked name failed:', type(e).__name__)\n",
            "def connect(ip):\n",
            "    s = socket.socket()\n",
            "    s.settimeout(5)\n",
            "    try:\n",
            "        s.connect((ip, 80))\n",
            "        return 'connected'\n",
            "    except OSError as e:\n",
            "        return 'refused' if e.errno == errno.EACCES else repr(e)\n",
            "print('blocked address', connect('{ip}'))\n",
            "print('other address', connect('1.1.1.1'))\n",
        ),
        ip = blocked_ip
    );
    sandbox.run(&*code).unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("blocked name failed"), "{out:?}");
    assert!(out.contains("blocked address refused"), "{out:?}");
    assert!(out.contains("other address connected"), "{out:?}");
}

/// glibc's getaddrinfo() sorts a dual-stack answer by probing every
/// candidate through one IPv6 UDP socket, disconnecting it with an
/// AF_UNSPEC connect in between.  A passive lookup, what `http.server`
/// does to bind, is the shortest path to that: it must not abort the guest.
#[test]
fn net_getaddrinfo_dual_stack_passive() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    let result = sandbox.run(
        "import socket\n\
         ai = socket.getaddrinfo(None, 8000, socket.AF_UNSPEC, socket.SOCK_STREAM, 0, socket.AI_PASSIVE)\n\
         print('FAMILIES', sorted({a[0] for a in ai}))\n\
         print('GAI_OK')",
    );
    let output = sandbox.drain_output();
    assert!(
        result.is_ok() && output.contains("GAI_OK"),
        "expected the dual-stack passive lookup to succeed, got result={result:?}, output={output:?}",
    );
}

/// A stream that arrives in two parts is read as fast as it arrives, with
/// a socket timeout set (Python then polls before each read): the second
/// part must wake the guest, not wait out the timeout.
#[test]
fn net_recv_wakes_for_data_after_a_short_read() {
    use std::io::Write;
    use std::net::TcpListener;
    use std::time::Duration;

    let rootfs = require_rootfs("python");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        let (mut conn, _) = listener.accept().unwrap();
        conn.write_all(&[b'a'; 4096]).unwrap();
        std::thread::sleep(Duration::from_millis(300));
        conn.write_all(&[b'b'; 4096]).unwrap();
    });
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .network(NetworkPolicy::AllowAll)
        .boot()
        .unwrap();
    let code = format!(
        "import socket, time\n\
         s = socket.create_connection(('127.0.0.1', {port}), timeout=10)\n\
         t, n = time.time(), 0\n\
         while (b := s.recv(65536)):\n\
         \x20   n += len(b)\n\
         print('read', n, 'in', round(time.time() - t, 1))\n"
    );
    sandbox.run(code.as_str()).unwrap();
    server.join().unwrap();
    let output = sandbox.drain_output();
    let secs: f64 = output
        .trim()
        .rsplit(' ')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(f64::MAX);
    assert!(
        output.starts_with("read 8192 in") && secs < 5.0,
        "expected both parts well before the 10 s timeout, got: {output:?}",
    );
}
