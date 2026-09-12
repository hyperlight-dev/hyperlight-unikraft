// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Network access policy — controls which destinations a guest can reach.
//!
//! ## Policy variants
//!
//! - [`NetworkPolicy::AllowAll`] — all outbound connections permitted.
//! - [`NetworkPolicy::AllowList`] — only listed hosts/IPs permitted.
//! - [`NetworkPolicy::BlockList`] — all except listed hosts/IPs permitted.
//!
//! All variants block link-local (169.254.0.0/16, fe80::/10) unconditionally
//! — these host cloud metadata services (e.g. 169.254.169.254).
//!
//! [`AllowList`] and [`BlockList`] additionally block loopback (127.0.0.0/8,
//! ::1) because in the hostsock model a guest socket is a real host socket,
//! and host-local services trust loopback without authentication.
//! [`AllowAll`] permits loopback to support intra-guest server+client
//! patterns (both endpoints are guest sockets on the host's loopback).
//!
//! ## DNS-aware enforcement
//!
//! [`AllowList`] tracks hostnames and re-resolves them at check time so
//! CDN/anycast IP rotation doesn't cause false denials. DNS responses
//! (port 53) are inspected and resolved IPs are learned dynamically so
//! a guest connecting to a just-resolved IP is allowed even if the IP
//! wasn't in the initial resolution set.

use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::sync::{Arc, Mutex};

/// Maximum number of IPs learned from DNS responses.
const MAX_LEARNED_IPS: usize = 256;

// ── NetworkPolicy ──────────────────────────────────────────────────

/// Controls which network destinations a guest sandbox can reach.
///
/// By default, networking is **disabled** (no `net_*` host functions are
/// registered).  Callers must opt in via the library API or the `--net`
/// CLI flag.
#[derive(Clone, Debug)]
pub enum NetworkPolicy {
    /// All outbound connections are allowed (no filtering).
    AllowAll,
    /// Only connections to the listed destinations are permitted.
    AllowList(AllowList),
    /// All connections are allowed *except* to the listed destinations.
    BlockList(BlockList),
}

impl NetworkPolicy {
    /// Check whether a connection to `addr` is permitted.
    ///
    /// Returns `Ok(())` if allowed, `Err(reason)` if denied.
    pub fn check(&self, addr: &SocketAddr) -> Result<(), String> {
        // Link-local — blocks cloud metadata (169.254.169.254) and
        // IPv6 link-local (fe80::/10) for all policy variants.
        let is_link_local = match addr.ip() {
            IpAddr::V4(v4) => v4.is_link_local(),
            IpAddr::V6(v6) => {
                let seg = v6.segments();
                (seg[0] & 0xffc0) == 0xfe80
            }
        };
        if is_link_local {
            return Err(format!(
                "network policy denies connection to link-local address {addr}"
            ));
        }

        match self {
            // AllowAll — no filtering (loopback permitted for intra-guest
            // server+client patterns in the hostsock model).
            NetworkPolicy::AllowAll => Ok(()),
            // AllowList/BlockList — also block loopback.  Host services
            // trust loopback without auth, and in the hostsock model a
            // guest socket is a real host socket, so a guest connecting to
            // 127.0.0.1 reaches host-only services.
            NetworkPolicy::AllowList(al) => {
                if addr.ip().is_loopback() {
                    return Err(format!(
                        "network policy denies connection to loopback address {addr}"
                    ));
                }
                if al.is_allowed(&addr.ip())
                    || (addr.port() == 53 && dns_resolvers().contains(&addr.ip()))
                {
                    Ok(())
                } else {
                    Err(format!("network policy denies connection to {addr}"))
                }
            }
            NetworkPolicy::BlockList(bl) => {
                if addr.ip().is_loopback() {
                    return Err(format!(
                        "network policy denies connection to loopback address {addr}"
                    ));
                }
                if bl.is_blocked(&addr.ip()) {
                    Err(format!("network policy denies connection to {addr}"))
                } else {
                    Ok(())
                }
            }
        }
    }
}

// ── AllowList ──────────────────────────────────────────────────────

/// A set of allowed network destinations.
///
/// Stores both literal IPs and hostnames.  At check time, hostnames are
/// re-resolved so the policy tracks DNS changes (CDN rotation, etc.).
#[derive(Clone, Debug)]
pub struct AllowList {
    allowed_ips: HashSet<IpAddr>,
    hostnames: Vec<String>,
    learned_ips: Arc<Mutex<HashSet<IpAddr>>>,
}

impl AllowList {
    /// Build an allowlist from a mixed set of hostnames and IP literals.
    ///
    /// Hostnames are resolved at construction time (fail-closed).
    /// At check time they are re-resolved so CDN/anycast rotation
    /// doesn't cause false denials.
    pub fn from_hosts(entries: &[impl AsRef<str>]) -> Result<Self, String> {
        let mut allowed_ips = HashSet::new();
        let mut hostnames = Vec::new();
        for entry in entries {
            let entry = entry.as_ref();
            if let Ok(ip) = entry.parse::<IpAddr>() {
                allowed_ips.insert(ip);
            } else {
                let addrs = (entry, 0u16)
                    .to_socket_addrs()
                    .map_err(|e| format!("resolve {entry:?}: {e}"))?;
                let mut found = false;
                for sa in addrs {
                    allowed_ips.insert(sa.ip());
                    found = true;
                }
                if !found {
                    return Err(format!("hostname {entry:?} resolved to zero addresses"));
                }
                hostnames.push(entry.to_string());
            }
        }
        Ok(Self {
            allowed_ips,
            hostnames,
            learned_ips: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    fn is_allowed(&self, ip: &IpAddr) -> bool {
        if self.allowed_ips.contains(ip) {
            return true;
        }
        if let Ok(learned) = self.learned_ips.lock()
            && learned.contains(ip)
        {
            return true;
        }
        // Re-resolve hostnames to catch CDN/anycast IP rotation.
        for host in &self.hostnames {
            if let Ok(addrs) = (host.as_str(), 0u16).to_socket_addrs() {
                for sa in addrs {
                    if &sa.ip() == ip {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Record a newly-learned IP (from DNS response inspection).
    pub fn learn_ip(&self, ip: IpAddr) {
        if let Ok(mut learned) = self.learned_ips.lock()
            && learned.len() < MAX_LEARNED_IPS
        {
            learned.insert(ip);
        }
    }
}

// ── BlockList ──────────────────────────────────────────────────────

/// A set of blocked network destinations.
///
/// Like [`AllowList`], stores both literal IPs and hostnames.  At check
/// time, hostnames are re-resolved so the policy tracks DNS changes.
#[derive(Clone, Debug)]
pub struct BlockList {
    blocked_ips: HashSet<IpAddr>,
    hostnames: Vec<String>,
}

impl BlockList {
    /// Build a blocklist from a mixed set of hostnames and IP literals.
    ///
    /// Hostnames are resolved at construction time (fail-closed).
    pub fn from_hosts(entries: &[impl AsRef<str>]) -> Result<Self, String> {
        let mut blocked_ips = HashSet::new();
        let mut hostnames = Vec::new();
        for entry in entries {
            let entry = entry.as_ref();
            if let Ok(ip) = entry.parse::<IpAddr>() {
                blocked_ips.insert(ip);
            } else {
                let addrs = (entry, 0u16)
                    .to_socket_addrs()
                    .map_err(|e| format!("resolve {entry:?}: {e}"))?;
                let mut found = false;
                for sa in addrs {
                    blocked_ips.insert(sa.ip());
                    found = true;
                }
                if !found {
                    return Err(format!("hostname {entry:?} resolved to zero addresses"));
                }
                hostnames.push(entry.to_string());
            }
        }
        Ok(Self {
            blocked_ips,
            hostnames,
        })
    }

    fn is_blocked(&self, ip: &IpAddr) -> bool {
        if self.blocked_ips.contains(ip) {
            return true;
        }
        for host in &self.hostnames {
            if let Ok(addrs) = (host.as_str(), 0u16).to_socket_addrs() {
                for sa in addrs {
                    if &sa.ip() == ip {
                        return true;
                    }
                }
            }
        }
        false
    }
}

// ── ListenPorts ────────────────────────────────────────────────────

/// Controls which ports a guest may bind to for inbound connections.
///
/// Orthogonal to [`NetworkPolicy`] (which governs *outbound* destinations).
/// Without a `ListenPorts` allowlist, `net_bind` rejects every call
/// (outbound-only mode).
#[derive(Clone, Debug)]
pub struct ListenPorts {
    ports: HashSet<u16>,
}

impl ListenPorts {
    /// Create from an iterator of port numbers.
    pub fn from_ports(ports: impl IntoIterator<Item = u16>) -> Self {
        Self {
            ports: ports.into_iter().collect(),
        }
    }

    /// Returns `Ok(())` if `port` is in the allowlist.
    pub fn check(&self, port: u16) -> Result<(), String> {
        if self.ports.contains(&port) {
            Ok(())
        } else {
            Err(format!(
                "Permission denied: port {port} not in listen allowlist ({:?})",
                self.ports
            ))
        }
    }
}

// ── DNS resolver exemption ─────────────────────────────────────────

/// DNS resolver IPs that the AllowList auto-exempts on port 53.
///
/// Includes the host's configured resolvers (from `/etc/resolv.conf`)
/// **plus** well-known public DNS servers that the guest may hardcode.
fn dns_resolvers() -> &'static HashSet<IpAddr> {
    static RESOLVERS: std::sync::OnceLock<HashSet<IpAddr>> = std::sync::OnceLock::new();
    RESOLVERS.get_or_init(|| {
        let mut set = HashSet::new();
        // Well-known public DNS the guest's initrd may hardcode.
        for ip in [
            "8.8.8.8", "8.8.4.4", // Google
            "1.1.1.1", "1.0.0.1", // Cloudflare
        ] {
            set.insert(ip.parse::<IpAddr>().unwrap());
        }
        #[cfg(unix)]
        {
            if let Ok(contents) = std::fs::read_to_string("/etc/resolv.conf") {
                for line in contents.lines() {
                    let line = line.trim();
                    if let Some(rest) = line.strip_prefix("nameserver")
                        && let Some(ip_str) = rest.split_whitespace().next()
                        && let Ok(ip) = ip_str.parse::<IpAddr>()
                    {
                        set.insert(ip);
                    }
                }
            }
        }
        #[cfg(windows)]
        {
            if let Ok(output) = std::process::Command::new("ipconfig").arg("/all").output() {
                let text = String::from_utf8_lossy(&output.stdout);
                let mut in_dns_block = false;
                for line in text.lines() {
                    let trimmed = line.trim();
                    if let Some(rest) = trimmed.strip_prefix("DNS Servers") {
                        in_dns_block = true;
                        let value = rest.trim_start_matches(['.', ' ', ':']);
                        if let Ok(ip) = value.parse::<IpAddr>() {
                            set.insert(ip);
                        }
                    } else if in_dns_block {
                        if let Ok(ip) = trimmed.parse::<IpAddr>() {
                            set.insert(ip);
                        } else {
                            in_dns_block = false;
                        }
                    }
                }
            }
        }
        set
    })
}

// ── DNS response IP learning ───────────────────────────────────────

/// Extract IPs from a DNS response for hostnames that match the allow list.
/// Minimal parser — handles standard A (type 1) and AAAA (type 28) answers.
pub fn learn_ips_from_dns_response(data: &[u8], al: &AllowList) {
    if data.len() < 12 {
        return;
    }
    let flags = u16::from_be_bytes([data[2], data[3]]);
    let is_response = (flags & 0x8000) != 0;
    if !is_response {
        return;
    }
    let qdcount = u16::from_be_bytes([data[4], data[5]]) as usize;
    let ancount = u16::from_be_bytes([data[6], data[7]]) as usize;
    if qdcount == 0 || ancount == 0 {
        return;
    }

    // Parse question section to extract the queried name.
    let mut pos = 12;
    let qname = match dns_read_name(data, &mut pos) {
        Some(n) => n,
        None => return,
    };
    // Skip QTYPE (2) + QCLASS (2)
    pos += 4;
    if pos > data.len() {
        return;
    }

    // Check if the queried name matches any allowed hostname.
    let qname_lower = qname.to_lowercase();
    let is_allowed_host = al.hostnames.iter().any(|h| h.to_lowercase() == qname_lower);
    if !is_allowed_host {
        return;
    }

    // Parse answer records and learn IPs.
    for _ in 0..ancount {
        if dns_read_name(data, &mut pos).is_none() {
            return;
        }
        if pos + 10 > data.len() {
            return;
        }
        let rtype = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let rdlen = u16::from_be_bytes([data[pos + 8], data[pos + 9]]) as usize;
        pos += 10;
        if pos + rdlen > data.len() {
            return;
        }
        match rtype {
            1 if rdlen == 4 => {
                let ip = IpAddr::V4(std::net::Ipv4Addr::new(
                    data[pos],
                    data[pos + 1],
                    data[pos + 2],
                    data[pos + 3],
                ));
                al.learn_ip(ip);
            }
            28 if rdlen == 16 => {
                let mut octets = [0u8; 16];
                octets.copy_from_slice(&data[pos..pos + 16]);
                al.learn_ip(IpAddr::V6(std::net::Ipv6Addr::from(octets)));
            }
            _ => {}
        }
        pos += rdlen;
    }
}

/// Read a DNS name at `pos`, advancing pos past it.
fn dns_read_name(data: &[u8], pos: &mut usize) -> Option<String> {
    let mut name = String::new();
    let mut p = *pos;
    let mut jumped = false;
    let mut jump_save = 0;
    let mut hops = 0u8;
    loop {
        if p >= data.len() {
            return None;
        }
        let len = data[p] as usize;
        if len == 0 {
            p += 1;
            break;
        }
        if (len & 0xC0) == 0xC0 {
            if p + 1 >= data.len() {
                return None;
            }
            hops += 1;
            if hops > 128 {
                return None;
            }
            let offset = ((len & 0x3F) << 8) | data[p + 1] as usize;
            if !jumped {
                jump_save = p + 2;
                jumped = true;
            }
            p = offset;
            continue;
        }
        p += 1;
        if p + len > data.len() {
            return None;
        }
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(&String::from_utf8_lossy(&data[p..p + len]));
        p += len;
    }
    if jumped {
        *pos = jump_save;
    } else {
        *pos = p;
    }
    Some(name)
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowall_permits_normal_address() {
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "93.184.216.34:80".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn allowall_blocks_link_local() {
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "169.254.169.254:80".parse().unwrap();
        assert!(policy.check(&addr).is_err());
    }

    #[test]
    fn allowall_permits_loopback() {
        // AllowAll permits loopback — needed for intra-guest server+client
        // in the hostsock model where all guest sockets are host sockets.
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn allowall_permits_ipv6_loopback() {
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "[::1]:80".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn allowlist_blocks_loopback() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(policy.check(&addr).is_err());
    }

    #[test]
    fn blocklist_blocks_loopback() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(policy.check(&addr).is_err());
    }

    #[test]
    fn allowlist_permits_listed_ip() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: SocketAddr = "93.184.216.34:80".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn allowlist_blocks_unlisted_ip() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: SocketAddr = "1.2.3.4:80".parse().unwrap();
        assert!(policy.check(&addr).is_err());
    }

    #[test]
    fn allowlist_exempts_dns_on_port_53() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        // Google DNS on port 53 should be exempted.
        let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
        // But not on other ports.
        let addr: SocketAddr = "8.8.8.8:80".parse().unwrap();
        assert!(policy.check(&addr).is_err());
    }

    #[test]
    fn blocklist_blocks_listed_ip() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: SocketAddr = "1.2.3.4:80".parse().unwrap();
        assert!(policy.check(&addr).is_err());
    }

    #[test]
    fn blocklist_permits_unlisted_ip() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: SocketAddr = "93.184.216.34:80".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn listen_ports_permits_listed_port() {
        let lp = ListenPorts::from_ports([8080, 3000]);
        assert!(lp.check(8080).is_ok());
        assert!(lp.check(3000).is_ok());
    }

    #[test]
    fn listen_ports_blocks_unlisted_port() {
        let lp = ListenPorts::from_ports([8080]);
        assert!(lp.check(9090).is_err());
    }

    #[test]
    fn allowlist_learns_ip() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let learned_ip: IpAddr = "10.0.0.1".parse().unwrap();
        // Not allowed before learning.
        assert!(!al.is_allowed(&learned_ip));
        // Learn it.
        al.learn_ip(learned_ip);
        // Now allowed.
        assert!(al.is_allowed(&learned_ip));
    }

    #[test]
    fn learn_ip_cap() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        // Fill up the learned set.
        for i in 0..MAX_LEARNED_IPS {
            al.learn_ip(IpAddr::V4(std::net::Ipv4Addr::new(
                10,
                (i >> 8) as u8,
                i as u8,
                1,
            )));
        }
        // One more should not be inserted.
        let extra: IpAddr = "172.16.0.1".parse().unwrap();
        al.learn_ip(extra);
        assert!(!al.is_allowed(&extra));
    }
}
