// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use common::{UNUSED_PORT, host_ip, net_probe, require_rootfs};
use hyperlight_unikraft::{AllowList, BlockList, NetworkPolicy, SandboxBuilder};

/// Networking disabled by default — guest socket calls fail because
/// net_* host functions aren't registered at all.
#[test]
fn net_policy_disabled_by_default() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .boot()
        .unwrap();
    // socket() calls the net_socket host function which isn't registered,
    // causing the guest to abort — run() returns an error.
    let result = sandbox.run(
        "import socket; s = socket.socket(socket.AF_INET, socket.SOCK_STREAM); print('SOCKET_OK')",
    );
    let output = sandbox.drain_output();
    assert!(
        result.is_err() || !output.contains("SOCKET_OK"),
        "expected socket creation to fail when networking is disabled, got result={result:?}, output={output:?}",
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
