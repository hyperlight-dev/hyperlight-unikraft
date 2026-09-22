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
//! All variants refuse the cloud metadata addresses unconditionally: the
//! link-local ranges (169.254.0.0/16, fe80::/10), where Azure, AWS and GCP
//! serve instance metadata at 169.254.169.254, and AWS's IPv6 endpoint
//! fd00:ec2::254, which is a unique local address rather than a
//! link-local one.
//!
//! [`AllowList`] and [`BlockList`] additionally block loopback (127.0.0.0/8,
//! ::1) because in the hostsock model a guest socket is a real host socket,
//! and host-local services trust loopback without authentication.
//! [`NetworkPolicy::AllowAll`] permits loopback to support intra-guest server+client
//! patterns (both endpoints are guest sockets on the host's loopback).
//!
//! ## Names and addresses
//!
//! A destination reaches the host as an address, never as a name: the
//! guest resolves names itself, with DNS questions that are ordinary UDP
//! messages to port 53, and then connects to what it was told.  The
//! policy is therefore enforced at three points:
//!
//! - **The question** (`allows_query`): the name is still a name here.
//!   Under an allow list the guest may only ask about listed names; under
//!   a block list it may not ask about a blocked one.  A guest that cannot
//!   ask never obtains the address.
//! - **The answer** (`learn_from_dns_answer`): under an allow list the
//!   addresses the guest was told for a listed name are recorded, so the
//!   connect that follows is recognised.
//! - **The destination** (`allows`): the address of every connect and
//!   sendto.  An allow list allows only what was listed, resolved when the
//!   list was built, or recorded from an answer; nothing is resolved on the
//!   host's behalf, so a guest connecting to an address it was never given
//!   is refused.  A block list refuses what was resolved when it was built
//!   and, since the guest may have an address without asking, whatever a
//!   blocked name resolves to now, looked up with a short deadline: a
//!   lookup that fails or runs late counts as blocked.
//!
//! The DNS exemption an allow list grants, port 53 at a known resolver,
//! covers UDP only, so a question cannot leave over a TCP connection the
//! question check does not read.  What no design covers: a block list
//! cannot refuse an address it was never shown to belong to the name.  An
//! embedder who needs a guarantee lists what is allowed.

use std::collections::HashSet;
use std::net::{IpAddr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::ops::RangeInclusive;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};
use std::{fmt, io, thread};

use tracing::debug;

/// Most addresses recorded from DNS answers.  Each is one a resolver gave
/// for a listed name, so a large set is no less safe; past it the guest
/// would be refused addresses it was rightly told.
const MAX_LEARNED_IPS: usize = 4096;

/// The most a block list waits for its names to resolve at a connect.  The
/// lookup runs while the VM is stopped inside the host call, so it is
/// bounded; a name that has not answered by then counts as blocked.
const LOOKUP_DEADLINE: Duration = Duration::from_millis(250);

/// The most resolver threads a block list may have running at once, across
/// all connects.  A `std` name lookup cannot be cancelled and runs to
/// completion even after the caller stops waiting, so without a cap a guest
/// that floods connects while DNS is slow could pile them up without limit.
const MAX_INFLIGHT_LOOKUPS: usize = 32;

/// Live resolver threads, counted against [`MAX_INFLIGHT_LOOKUPS`].
static INFLIGHT_LOOKUPS: AtomicUsize = AtomicUsize::new(0);

/// AWS's IPv6 instance metadata endpoint, per the EC2 user guide
/// ("Access instance metadata for an EC2 instance", IPv6 support).
const AWS_IMDS_V6: Ipv6Addr = Ipv6Addr::new(0xfd00, 0xec2, 0, 0, 0, 0, 0, 0x254);

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
    /// Whether the guest may reach `addr`, on a UDP socket or not.
    pub(crate) fn allows(&self, addr: &SocketAddr, udp: bool) -> bool {
        // An IPv4 address spelled as IPv4-mapped IPv6 (`::ffff:a.b.c.d`)
        // reaches the same IPv4 destination through a dual-stack socket,
        // so every rule below sees the IPv4 form.
        let ip = canonical(addr.ip());
        if is_metadata(&ip) {
            return false;
        }

        match self {
            // AllowAll — no filtering (loopback permitted for intra-guest
            // server+client patterns in the hostsock model).
            NetworkPolicy::AllowAll => true,
            // AllowList/BlockList — also block loopback.  Host services
            // trust loopback without auth, and in the hostsock model a
            // guest socket is a real host socket, so a guest connecting to
            // 127.0.0.1 reaches host-only services.
            NetworkPolicy::AllowList(al) => {
                !ip.is_loopback()
                    && (al.is_allowed(&ip) || (udp && addr.port() == 53 && al.is_resolver(&ip)))
            }
            NetworkPolicy::BlockList(bl) => !ip.is_loopback() && !bl.blocks(&ip),
        }
    }

    /// Whether the guest may send `data`, a DNS message bound for port 53.
    /// Under an allow list every question must name a listed host; under a
    /// block list none may name a blocked one.  Anything that is not a
    /// well-formed query is refused under either: nothing else has
    /// business on port 53.
    pub(crate) fn allows_query(&self, data: &[u8]) -> bool {
        let names = match self {
            NetworkPolicy::AllowAll => return true,
            NetworkPolicy::AllowList(al) => &al.hostnames,
            NetworkPolicy::BlockList(bl) => &bl.hostnames,
        };
        let Some(questions) = dns_question_names(data) else {
            return false;
        };
        match self {
            NetworkPolicy::AllowAll => true,
            NetworkPolicy::AllowList(_) => questions.iter().all(|q| names_contain(names, q)),
            NetworkPolicy::BlockList(_) => !questions.iter().any(|q| names_contain(names, q)),
        }
    }

    /// Record, under an allow list, the addresses a DNS answer gives for a
    /// listed name, so the connect that follows is recognised.  Only an
    /// answer from a resolver the guest may ask counts: a datagram from
    /// anywhere else claiming to be one teaches nothing.
    pub(crate) fn learn_from_dns_answer(&self, from: SocketAddr, data: &[u8]) {
        if let NetworkPolicy::AllowList(al) = self
            && from.port() == 53
            && al.is_resolver(&canonical(from.ip()))
        {
            learn_ips_from_dns_response(data, al);
        }
    }
}

/// Whether `name`, as a DNS question spells it, is one of `names`: the
/// comparison ignores case and a trailing dot.
fn names_contain(names: &[String], name: &str) -> bool {
    let name = name.trim_end_matches('.');
    names
        .iter()
        .any(|n| n.trim_end_matches('.').eq_ignore_ascii_case(name))
}

/// The IPv4 address behind an IPv4-mapped IPv6 one, else the address as is.
fn canonical(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map(IpAddr::V4).unwrap_or(ip),
        v4 => v4,
    }
}

/// Whether `ip` is a cloud metadata address, refused under every policy:
/// link-local (169.254.0.0/16, where Azure, AWS and GCP serve instance
/// metadata; fe80::/10), or AWS's IPv6 endpoint.
fn is_metadata(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_link_local(),
        IpAddr::V6(v6) => (v6.segments()[0] & 0xffc0) == 0xfe80 || *v6 == AWS_IMDS_V6,
    }
}

// ── ResolveError ───────────────────────────────────────────────────

/// A host named in an [`AllowList`] or [`BlockList`] could not be resolved.
///
/// The lists resolve their hostnames when they are built, so a name that
/// does not resolve is refused up front instead of silently matching
/// nothing.
#[derive(Debug)]
pub struct ResolveError {
    host: String,
    source: Option<io::Error>,
}

impl ResolveError {
    /// The entry that failed, as it was given.
    pub fn host(&self) -> &str {
        &self.host
    }
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            Some(e) => write!(f, "cannot resolve {:?}: {e}", self.host),
            None => write!(f, "{:?} resolved to no address", self.host),
        }
    }
}

impl std::error::Error for ResolveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e as _)
    }
}

/// Resolve every entry: IP literals as they are, hostnames through the
/// system resolver.  Returns the addresses and the hostnames among the
/// entries; the hostnames decide which DNS questions the guest may send,
/// and a block list looks them up again at each connect.
fn resolve_entries(
    entries: &[impl AsRef<str>],
) -> Result<(HashSet<IpAddr>, Vec<String>), ResolveError> {
    let mut ips = HashSet::new();
    let mut hostnames = Vec::new();
    for entry in entries {
        let entry = entry.as_ref();
        if let Ok(ip) = entry.parse::<IpAddr>() {
            ips.insert(ip);
            continue;
        }
        let addrs = (entry, 0u16).to_socket_addrs().map_err(|e| ResolveError {
            host: entry.to_string(),
            source: Some(e),
        })?;
        let mut found = false;
        for sa in addrs {
            ips.insert(sa.ip());
            found = true;
        }
        if !found {
            return Err(ResolveError {
                host: entry.to_string(),
                source: None,
            });
        }
        hostnames.push(entry.to_string());
    }
    Ok((ips, hostnames))
}

// ── AllowList ──────────────────────────────────────────────────────

/// A set of allowed network destinations.
///
/// Stores both literal IPs and hostnames.  The hostnames decide which
/// DNS questions the guest may ask; the addresses the guest is told for
/// them are recorded as they pass, so the policy follows DNS changes
/// (CDN rotation, etc.) without resolving anything on the guest's behalf.
#[derive(Clone, Debug)]
pub struct AllowList {
    allowed_ips: HashSet<IpAddr>,
    hostnames: Vec<String>,
    learned_ips: Arc<Mutex<HashSet<IpAddr>>>,
    /// Resolvers the guest was given (its `/etc/resolv.conf`), exempt on
    /// port 53 like the host's own.
    resolvers: Arc<Mutex<HashSet<IpAddr>>>,
}

impl AllowList {
    /// Build an allowlist from a mixed set of hostnames and IP literals.
    ///
    /// Hostnames are resolved at construction time (fail-closed).  From
    /// then on the guest may ask DNS about them, and what it is told is
    /// recorded; nothing else is resolved for it.
    pub fn from_hosts(entries: &[impl AsRef<str>]) -> Result<Self, ResolveError> {
        let (allowed_ips, hostnames) = resolve_entries(entries)?;
        Ok(Self {
            allowed_ips,
            hostnames,
            learned_ips: Arc::new(Mutex::new(HashSet::new())),
            resolvers: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    /// Also treat `ips` as DNS resolvers the guest may ask: the nameservers
    /// of the `resolv.conf` it was given (see `resolv_conf_nameservers`).
    pub fn exempt_resolvers(&self, ips: impl IntoIterator<Item = IpAddr>) {
        if let Ok(mut resolvers) = self.resolvers.lock() {
            resolvers.extend(ips.into_iter().map(canonical));
        }
    }

    /// Whether `ip` is a resolver the guest may ask on port 53: the host's
    /// own, a well-known public one, or one the guest was given.
    fn is_resolver(&self, ip: &IpAddr) -> bool {
        dns_resolvers().contains(ip)
            || self
                .resolvers
                .lock()
                .is_ok_and(|resolvers| resolvers.contains(ip))
    }

    fn is_allowed(&self, ip: &IpAddr) -> bool {
        if self.allowed_ips.contains(ip) {
            return true;
        }
        self.learned_ips
            .lock()
            .is_ok_and(|learned| learned.contains(ip))
    }

    /// Record a newly-learned IP (from DNS response inspection).
    pub(crate) fn learn_ip(&self, ip: IpAddr) {
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
/// Like [`AllowList`], stores both literal IPs and hostnames.  The
/// hostnames decide which DNS questions the guest may not ask, and are
/// looked up again at every connect (see [`BlockList::blocks`]) for the
/// guest that has an address without asking.
#[derive(Clone, Debug)]
pub struct BlockList {
    blocked_ips: HashSet<IpAddr>,
    hostnames: Vec<String>,
}

impl BlockList {
    /// Build a blocklist from a mixed set of hostnames and IP literals.
    ///
    /// Hostnames are resolved at construction time (fail-closed).
    pub fn from_hosts(entries: &[impl AsRef<str>]) -> Result<Self, ResolveError> {
        let (blocked_ips, hostnames) = resolve_entries(entries)?;
        Ok(Self {
            blocked_ips,
            hostnames,
        })
    }

    /// Whether `ip` is blocked: listed or resolved at build time, or what
    /// a blocked name resolves to now.  The lookup is bounded by
    /// [`LOOKUP_DEADLINE`], and a name that fails to resolve or runs late
    /// counts as blocked: when the list cannot tell, it refuses.
    fn blocks(&self, ip: &IpAddr) -> bool {
        if self.blocked_ips.contains(ip) {
            return true;
        }
        // No answer in time, or none at all: refuse.
        resolves_to_now(&self.hostnames, ip).unwrap_or(true)
    }
}

/// Whether one of `names` resolves to `ip` at this moment.  Every name is
/// looked up at once, each on its own thread since the resolver has no
/// deadline of its own, and the answer is `None` when any lookup fails or
/// outlives [`LOOKUP_DEADLINE`].
fn resolves_to_now(names: &[String], ip: &IpAddr) -> Option<bool> {
    if names.is_empty() {
        return Some(false);
    }
    let (tx, rx) = mpsc::channel();
    let mut spawned = 0usize;
    for name in names {
        // Bound the resolver threads a guest can have running at once.  A
        // lookup cannot be cancelled and runs to completion even after we
        // stop waiting on it, so over the cap the lookup is skipped and
        // treated as no answer -- the block list then fails closed below,
        // exactly as it does for a lookup that misses its deadline.
        if INFLIGHT_LOOKUPS.fetch_add(1, Ordering::Relaxed) >= MAX_INFLIGHT_LOOKUPS {
            INFLIGHT_LOOKUPS.fetch_sub(1, Ordering::Relaxed);
            debug!("block list: resolver thread cap reached; refusing");
            return None;
        }
        let (tx, name) = (tx.clone(), name.clone());
        thread::spawn(move || {
            let ips = (name.as_str(), 0u16)
                .to_socket_addrs()
                .map(|addrs| addrs.map(|a| a.ip()).collect::<HashSet<_>>());
            let _ = tx.send((name, ips));
            INFLIGHT_LOOKUPS.fetch_sub(1, Ordering::Relaxed);
        });
        spawned += 1;
    }
    drop(tx);
    let deadline = Instant::now() + LOOKUP_DEADLINE;
    let mut pending = spawned;
    while pending > 0 {
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok((_, Ok(ips))) if ips.contains(ip) => return Some(true),
            Ok((_, Ok(_))) => pending -= 1,
            Ok((name, Err(e))) => {
                debug!(%name, error = %e, "block list: lookup failed; refusing");
                return None;
            }
            Err(_) => {
                debug!("block list: lookup past its deadline; refusing");
                return None;
            }
        }
    }
    Some(false)
}

// ── ListenPorts ────────────────────────────────────────────────────

/// Controls which ports a guest may bind to for inbound connections.
///
/// Orthogonal to [`NetworkPolicy`] (which governs *outbound* destinations).
/// Without a `ListenPorts` allowlist, `net_bind` rejects every call
/// (outbound-only mode).  Single ports, ranges and "every port" compose:
/// [`ListenPorts::all`] is for an embedder whose own boundary, such as a
/// container's network namespace, already scopes what the guest exposes.
#[derive(Clone, Debug)]
pub struct ListenPorts {
    all: bool,
    ports: HashSet<u16>,
    ranges: Vec<RangeInclusive<u16>>,
}

impl ListenPorts {
    /// Create from an iterator of port numbers.
    pub fn from_ports(ports: impl IntoIterator<Item = u16>) -> Self {
        Self {
            all: false,
            ports: ports.into_iter().collect(),
            ranges: Vec::new(),
        }
    }

    /// Every port: the guest may bind any of them.
    pub fn all() -> Self {
        Self {
            all: true,
            ports: HashSet::new(),
            ranges: Vec::new(),
        }
    }

    /// Also permit every port in `range`.
    pub fn with_range(mut self, range: RangeInclusive<u16>) -> Self {
        self.ranges.push(range);
        self
    }

    /// Whether the guest may bind `port`.
    pub fn allows(&self, port: u16) -> bool {
        self.all || self.ports.contains(&port) || self.ranges.iter().any(|r| r.contains(&port))
    }
}

// ── DNS resolver exemption ─────────────────────────────────────────

/// The `nameserver` addresses of a `resolv.conf`.
pub(crate) fn resolv_conf_nameservers(content: &str) -> Vec<IpAddr> {
    content
        .lines()
        .filter_map(|line| line.trim().strip_prefix("nameserver"))
        .filter_map(|rest| rest.split_whitespace().next())
        .filter_map(|ip| ip.parse::<IpAddr>().ok())
        .collect()
}

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
                set.extend(resolv_conf_nameservers(&contents));
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

// ── DNS question names ─────────────────────────────────────────────

/// The names a DNS query asks about, or `None` if `data` is not a
/// well-formed standard query: the header says query (not response) and
/// opcode 0, and every question name parses.
pub(crate) fn dns_question_names(data: &[u8]) -> Option<Vec<String>> {
    if data.len() < 12 {
        return None;
    }
    let flags = u16::from_be_bytes([data[2], data[3]]);
    let is_response = (flags & 0x8000) != 0;
    let opcode = (flags >> 11) & 0xf;
    if is_response || opcode != 0 {
        return None;
    }
    let qdcount = u16::from_be_bytes([data[4], data[5]]) as usize;
    if qdcount == 0 {
        return None;
    }
    let mut pos = 12;
    let mut names = Vec::with_capacity(qdcount);
    for _ in 0..qdcount {
        names.push(dns_read_name(data, &mut pos)?);
        // QTYPE and QCLASS.
        pos += 4;
        if pos > data.len() {
            return None;
        }
    }
    Some(names)
}

// ── DNS response IP learning ───────────────────────────────────────

/// Extract IPs from a DNS response for hostnames that match the allow list.
/// Minimal parser — handles standard A (type 1) and AAAA (type 28) answers.
fn learn_ips_from_dns_response(data: &[u8], al: &AllowList) {
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
    let is_allowed_host = names_contain(&al.hostnames, &qname_lower);
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
        assert!(policy.allows(&addr, false));
    }

    #[test]
    fn allowall_blocks_link_local() {
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "169.254.169.254:80".parse().unwrap();
        assert!(!policy.allows(&addr, false));
    }

    /// The metadata addresses, IPv4 and IPv6, under every policy.
    #[test]
    fn metadata_addresses_are_refused_under_every_policy() {
        let policies = [
            NetworkPolicy::AllowAll,
            NetworkPolicy::AllowList(AllowList::from_hosts(&["fd00:ec2::254"]).unwrap()),
            NetworkPolicy::BlockList(BlockList::from_hosts(&["1.2.3.4"]).unwrap()),
        ];
        for addr in ["169.254.169.254:80", "[fe80::1]:80", "[fd00:ec2::254]:80"] {
            let addr: SocketAddr = addr.parse().unwrap();
            for policy in &policies {
                assert!(
                    !policy.allows(&addr, false),
                    "{addr} allowed under {policy:?}"
                );
            }
        }
    }

    /// A name that does not resolve is refused when the list is built,
    /// and the error says which.
    #[test]
    fn unresolvable_host_is_refused_up_front() {
        let err = AllowList::from_hosts(&["no-such-host.invalid"]).unwrap_err();
        assert_eq!(err.host(), "no-such-host.invalid");
        assert!(BlockList::from_hosts(&["no-such-host.invalid"]).is_err());
    }

    /// A DNS query for `names`, as a resolver would send it.
    fn dns_query(names: &[&str]) -> Vec<u8> {
        let mut m = vec![
            0x12,
            0x34,
            0x01,
            0x00,
            0,
            names.len() as u8,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        for name in names {
            for label in name.split('.') {
                m.push(label.len() as u8);
                m.extend_from_slice(label.as_bytes());
            }
            m.extend_from_slice(&[0, 0, 1, 0, 1]);
        }
        m
    }

    /// A DNS answer saying `name` is `ip`.
    fn dns_answer(name: &str, ip: IpAddr) -> Vec<u8> {
        let mut m = dns_query(&[name]);
        m[2] = 0x81;
        m[3] = 0x80;
        m[7] = 1;
        m.extend_from_slice(&[0xc0, 0x0c]);
        match ip {
            IpAddr::V4(v4) => {
                m.extend_from_slice(&[0, 1, 0, 1, 0, 0, 0, 60, 0, 4]);
                m.extend_from_slice(&v4.octets());
            }
            IpAddr::V6(v6) => {
                m.extend_from_slice(&[0, 28, 0, 1, 0, 0, 0, 60, 0, 16]);
                m.extend_from_slice(&v6.octets());
            }
        }
        m
    }

    /// The allow list lets DNS through to a known resolver on port 53, but
    /// only on UDP: a TCP connection there would carry questions the
    /// question check does not read.
    #[test]
    fn dns_exemption_is_udp_only() {
        let al = NetworkPolicy::AllowList(AllowList::from_hosts(&["93.184.216.34"]).unwrap());
        let resolver: SocketAddr = "8.8.8.8:53".parse().unwrap();
        assert!(al.allows(&resolver, true));
        assert!(!al.allows(&resolver, false));
    }

    /// Under an allow list the guest may ask DNS about listed names only;
    /// case and a trailing dot do not matter, and anything that is not a
    /// well-formed query is refused.
    #[test]
    fn allowlist_permits_questions_for_listed_names_only() {
        let al = NetworkPolicy::AllowList(AllowList::from_hosts(&["localhost"]).unwrap());
        assert!(al.allows_query(&dns_query(&["localhost"])));
        assert!(al.allows_query(&dns_query(&["LOCALHOST."])));
        assert!(!al.allows_query(&dns_query(&["example.com"])));
        assert!(!al.allows_query(&dns_query(&["localhost", "example.com"])));
        assert!(!al.allows_query(b"not a dns message"));
        let mut answer = dns_query(&["localhost"]);
        answer[2] |= 0x80; // a response, not a question
        assert!(!al.allows_query(&answer));
        assert!(NetworkPolicy::AllowAll.allows_query(&dns_query(&["anything.example"])));
    }

    /// Under a block list the guest may not ask DNS about a blocked name.
    #[test]
    fn blocklist_refuses_questions_for_blocked_names() {
        let bl = NetworkPolicy::BlockList(BlockList::from_hosts(&["localhost"]).unwrap());
        assert!(!bl.allows_query(&dns_query(&["localhost"])));
        assert!(!bl.allows_query(&dns_query(&["Localhost."])));
        assert!(bl.allows_query(&dns_query(&["example.com"])));
        assert!(!bl.allows_query(b"not a dns message"));
    }

    /// The addresses an answer gives for a listed name are recorded, and
    /// only those: an answer for another name teaches nothing.
    #[test]
    fn allowlist_records_addresses_from_answers() {
        let al = NetworkPolicy::AllowList(AllowList::from_hosts(&["localhost"]).unwrap());
        let resolver: SocketAddr = "8.8.8.8:53".parse().unwrap();
        let stranger: SocketAddr = "198.51.100.1:53".parse().unwrap();
        let told: SocketAddr = "203.0.113.9:443".parse().unwrap();
        let other: SocketAddr = "[2001:db8::9]:443".parse().unwrap();
        assert!(!al.allows(&told, false));
        // An answer from somewhere that is not a resolver teaches nothing.
        al.learn_from_dns_answer(stranger, &dns_answer("localhost", told.ip()));
        assert!(!al.allows(&told, false));
        al.learn_from_dns_answer(resolver, &dns_answer("localhost", told.ip()));
        assert!(al.allows(&told, false));
        al.learn_from_dns_answer(resolver, &dns_answer("example.com", other.ip()));
        assert!(!al.allows(&other, false));
        al.learn_from_dns_answer(resolver, &dns_answer("LOCALHOST", other.ip()));
        assert!(al.allows(&other, false));
        // A block list records nothing.
        let bl = NetworkPolicy::BlockList(BlockList::from_hosts(&["1.2.3.4"]).unwrap());
        bl.learn_from_dns_answer(resolver, &dns_answer("localhost", told.ip()));
        assert!(bl.allows(&told, false));
    }

    /// A block list refuses what a blocked name resolves to now, for the
    /// guest that connects to an address without asking.
    #[test]
    fn blocklist_blocks_what_a_name_resolves_to_now() {
        let bl = BlockList::from_hosts(&["localhost"]).unwrap();
        assert!(bl.blocks(&"127.0.0.1".parse().unwrap()));
        assert!(!bl.blocks(&"203.0.113.9".parse().unwrap()));
    }

    /// When the lookup cannot answer, the block list refuses.
    #[test]
    fn blocklist_refuses_when_a_lookup_fails() {
        let bl = BlockList {
            blocked_ips: HashSet::new(),
            hostnames: vec!["no-such-host.invalid".to_string()],
        };
        assert!(bl.blocks(&"203.0.113.9".parse().unwrap()));
    }

    #[test]
    fn dns_question_names_reads_every_question() {
        assert_eq!(
            dns_question_names(&dns_query(&["a.example", "b.example"])).unwrap(),
            vec!["a.example".to_string(), "b.example".to_string()]
        );
        assert!(dns_question_names(&dns_query(&[])).is_none());
        assert!(dns_question_names(&[0u8; 11]).is_none());
        let mut truncated = dns_query(&["a.example"]);
        truncated.truncate(14);
        assert!(dns_question_names(&truncated).is_none());
    }

    /// `::ffff:a.b.c.d` on a dual-stack socket reaches a.b.c.d, so every
    /// rule must see through it.
    #[test]
    fn ipv4_mapped_ipv6_is_checked_as_ipv4() {
        let metadata: SocketAddr = "[::ffff:169.254.169.254]:80".parse().unwrap();
        let loopback: SocketAddr = "[::ffff:127.0.0.1]:80".parse().unwrap();
        let blocked: SocketAddr = "[::ffff:10.0.0.5]:80".parse().unwrap();
        let allowed: SocketAddr = "[::ffff:93.184.216.34]:80".parse().unwrap();

        assert!(!NetworkPolicy::AllowAll.allows(&metadata, false));

        let bl = NetworkPolicy::BlockList(BlockList::from_hosts(&["10.0.0.5"]).unwrap());
        assert!(!bl.allows(&metadata, false));
        assert!(!bl.allows(&loopback, false));
        assert!(!bl.allows(&blocked, false));

        let al = NetworkPolicy::AllowList(AllowList::from_hosts(&["93.184.216.34"]).unwrap());
        assert!(!al.allows(&metadata, false));
        assert!(!al.allows(&loopback, false));
        assert!(al.allows(&allowed, false));
    }

    #[test]
    fn allowall_permits_loopback() {
        // AllowAll permits loopback — needed for intra-guest server+client
        // in the hostsock model where all guest sockets are host sockets.
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(policy.allows(&addr, false));
    }

    #[test]
    fn allowall_permits_ipv6_loopback() {
        let policy = NetworkPolicy::AllowAll;
        let addr: SocketAddr = "[::1]:80".parse().unwrap();
        assert!(policy.allows(&addr, false));
    }

    #[test]
    fn allowlist_blocks_loopback() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(!policy.allows(&addr, false));
    }

    #[test]
    fn blocklist_blocks_loopback() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(!policy.allows(&addr, false));
    }

    #[test]
    fn allowlist_permits_listed_ip() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: SocketAddr = "93.184.216.34:80".parse().unwrap();
        assert!(policy.allows(&addr, false));
    }

    #[test]
    fn allowlist_blocks_unlisted_ip() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: SocketAddr = "1.2.3.4:80".parse().unwrap();
        assert!(!policy.allows(&addr, false));
    }

    #[test]
    fn allowlist_exempts_dns_on_port_53() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        // Google DNS on port 53, over UDP, should be exempted.
        let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
        assert!(policy.allows(&addr, true));
        // But not on other ports.
        let addr: SocketAddr = "8.8.8.8:80".parse().unwrap();
        assert!(!policy.allows(&addr, true));
    }

    #[test]
    fn blocklist_blocks_listed_ip() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: SocketAddr = "1.2.3.4:80".parse().unwrap();
        assert!(!policy.allows(&addr, false));
    }

    #[test]
    fn blocklist_permits_unlisted_ip() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: SocketAddr = "93.184.216.34:80".parse().unwrap();
        assert!(policy.allows(&addr, false));
    }

    #[test]
    fn listen_ports_permits_listed_port() {
        let lp = ListenPorts::from_ports([8080, 3000]);
        assert!(lp.allows(8080));
        assert!(lp.allows(3000));
    }

    #[test]
    fn listen_ports_blocks_unlisted_port() {
        let lp = ListenPorts::from_ports([8080]);
        assert!(!lp.allows(9090));
    }

    #[test]
    fn listen_ports_all_permits_every_port() {
        let lp = ListenPorts::all();
        assert!(lp.allows(1));
        assert!(lp.allows(8080));
        assert!(lp.allows(65535));
    }

    #[test]
    fn allowlist_exempts_the_guest_resolver_on_port_53() {
        let al = AllowList::from_hosts(&["93.184.216.34"]).unwrap();
        let resolver: SocketAddr = "10.96.0.10:53".parse().unwrap();
        let policy = NetworkPolicy::AllowList(al.clone());
        assert!(!policy.allows(&resolver, true));
        al.exempt_resolvers(resolv_conf_nameservers(
            "nameserver 10.96.0.10\nsearch default.svc.cluster.local\n",
        ));
        assert!(policy.allows(&resolver, true));
        // Only as a resolver: not on other ports, not over TCP.
        assert!(!policy.allows(&"10.96.0.10:80".parse().unwrap(), true));
        assert!(!policy.allows(&resolver, false));
    }

    #[test]
    fn listen_ports_range_is_inclusive() {
        let lp = ListenPorts::from_ports([22]).with_range(8000..=8010);
        assert!(lp.allows(22));
        assert!(lp.allows(8000));
        assert!(lp.allows(8010));
        assert!(!lp.allows(7999));
        assert!(!lp.allows(8011));
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
