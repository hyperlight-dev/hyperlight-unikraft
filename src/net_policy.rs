// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Network access policy — controls which destinations a guest can reach.
//!
//! ## Policy variants
//!
//! - [`NetworkPolicy::AllowAll`] — all outbound connections permitted.
//! - [`NetworkPolicy::AllowList`] — only listed hosts/IPs permitted.
//! - [`NetworkPolicy::BlockList`] — all except listed hosts/IPs permitted.
//! - [`NetworkPolicy::Filter`] — the embedder's [`NetworkFilter`] decides,
//!   by name and port, each time it is asked.
//!
//! All variants refuse the cloud metadata addresses unconditionally: the
//! link-local ranges (169.254.0.0/16, fe80::/10), where Azure, AWS and GCP
//! serve instance metadata at 169.254.169.254, and AWS's IPv6 endpoint
//! fd00:ec2::254, which is a unique local address rather than a
//! link-local one.
//!
//! [`AllowList`], [`BlockList`] and [`Filter`] additionally block loopback (127.0.0.0/8,
//! ::1), and the unspecified address (0.0.0.0, ::), which a connect sends
//! to loopback, because in the hostsock model a guest socket is a real host
//! socket, and host-local services trust loopback without authentication.
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
//!   connect that follows is recognised; under a filter, with the name
//!   they were given for, which the filter is asked about at the connect.
//! - **The destination** (`allows`): the address of every TCP connect and
//!   UDP send (connected or not).  UDP connect selects a peer and source
//!   address without sending anything, so libc can use it for address-family
//!   and routing probes, including loopback, without granting payload access.
//!   An allow list allows only what was listed, resolved when the
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

use std::collections::{HashMap, HashSet};
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

/// The most a block list waits for its names to resolve at a TCP connect
/// or UDP send.  The lookup runs while the VM is stopped inside the host call, so it is
/// bounded; a name that has not answered by then counts as blocked.
const LOOKUP_DEADLINE: Duration = Duration::from_millis(250);

/// The most resolver threads a block list may have running at once, across
/// all destination checks.  A `std` name lookup cannot be cancelled and
/// runs to completion even after the caller stops waiting, so without a cap
/// a guest that floods checks while DNS is slow could pile them up without limit.
const MAX_INFLIGHT_LOOKUPS: usize = 32;

/// Live resolver threads, counted against [`MAX_INFLIGHT_LOOKUPS`].
static INFLIGHT_LOOKUPS: AtomicUsize = AtomicUsize::new(0);

/// AWS's IPv6 instance metadata endpoint, per the EC2 user guide
/// ("Access instance metadata for an EC2 instance", IPv6 support).
const AWS_IMDS_V6: Ipv6Addr = Ipv6Addr::new(0xfd00, 0xec2, 0, 0, 0, 0, 0, 0x254);

// ── NetworkPolicy ──────────────────────────────────────────────────

/// Controls which network destinations a guest sandbox can reach.
///
/// By default, networking is **disabled**: the `net_*` host functions
/// refuse every `socket()` with `EACCES`.  Callers opt in via the library
/// API or the `--net` CLI flag.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum NetworkPolicy {
    /// All outbound connections are allowed (no filtering).
    AllowAll,
    /// Only connections to the listed destinations are permitted.
    AllowList(AllowList),
    /// All connections are allowed *except* to the listed destinations.
    BlockList(BlockList),
    /// The embedder's [`NetworkFilter`] decides, by name and port.
    Filter(Filter),
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
            // AllowList/BlockList/Filter — also block loopback.  Host
            // services trust loopback without auth, and in the hostsock
            // model a guest socket is a real host socket, so a guest
            // connecting to 127.0.0.1 reaches host-only services.
            NetworkPolicy::AllowList(al) => {
                !is_host_local(&ip)
                    && (al.is_allowed(&ip)
                        || (udp && addr.port() == 53 && al.resolvers.contains(&ip)))
            }
            NetworkPolicy::BlockList(bl) => !is_host_local(&ip) && !bl.blocks(&ip),
            NetworkPolicy::Filter(f) => {
                !is_host_local(&ip) && f.allows(SocketAddr::new(ip, addr.port()), udp)
            }
        }
    }

    /// Whether the guest may send `data`, a DNS message bound for port 53.
    /// Under an allow list every question must name a listed host; under a
    /// block list none may name a blocked one; under a filter it must
    /// allow every name asked.  Anything that is not a well-formed query
    /// is refused under any of them: nothing else has business on port 53.
    pub(crate) fn allows_query(&self, data: &[u8]) -> bool {
        if let NetworkPolicy::AllowAll = self {
            return true;
        }
        let Some(questions) = dns_question_names(data) else {
            return false;
        };
        match self {
            NetworkPolicy::AllowAll => true,
            NetworkPolicy::AllowList(al) => {
                questions.iter().all(|q| names_contain(&al.hostnames, q))
            }
            NetworkPolicy::BlockList(bl) => {
                !questions.iter().any(|q| names_contain(&bl.hostnames, q))
            }
            NetworkPolicy::Filter(f) => questions.iter().all(|q| f.allows_name(&normalize_name(q))),
        }
    }

    /// Record the addresses a DNS answer gives for a name the policy lets
    /// the guest reach, so the connect that follows is recognised: under
    /// an allow list, for a listed name; under a filter, for any name it
    /// allows, with the name, which the filter is asked about at the
    /// connect.  Only an answer from a resolver the guest may ask counts: a
    /// datagram from anywhere else claiming to be one teaches nothing.
    pub(crate) fn learn_from_dns_answer(&self, from: SocketAddr, data: &[u8]) {
        if from.port() != 53 {
            return;
        }
        let from = canonical(from.ip());
        match self {
            NetworkPolicy::AllowList(al) if al.resolvers.contains(&from) => {
                if let Some((name, ips)) = dns_answer_ips(data)
                    && names_contain(&al.hostnames, &name)
                {
                    ips.into_iter().for_each(|ip| al.learn_ip(ip));
                }
            }
            NetworkPolicy::Filter(f) if f.resolvers.contains(&from) => {
                if let Some((name, ips)) = dns_answer_ips(data) {
                    let name = normalize_name(&name);
                    if f.allows_name(&name) {
                        f.learn(&name, ips);
                    }
                }
            }
            _ => {}
        }
    }

    /// Also let the guest ask DNS of `ips`, the nameservers of the
    /// `resolv.conf` it was given, under the policies that limit which
    /// resolvers it may ask.
    pub(crate) fn exempt_resolvers(&self, ips: Vec<IpAddr>) {
        match self {
            NetworkPolicy::AllowList(al) => al.exempt_resolvers(ips),
            NetworkPolicy::Filter(f) => f.exempt_resolvers(ips),
            NetworkPolicy::AllowAll | NetworkPolicy::BlockList(_) => {}
        }
    }
}

/// A name as a DNS question spells it, the way a [`NetworkFilter`] is
/// given it: in lower case, without the trailing dot.
fn normalize_name(name: &str) -> String {
    name.trim_end_matches('.').to_ascii_lowercase()
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

/// Whether a connection to `ip` stays on the host: loopback, or the
/// unspecified address (`0.0.0.0`, `::`), which a connect sends to
/// loopback.
fn is_host_local(ip: &IpAddr) -> bool {
    ip.is_loopback() || ip.is_unspecified()
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
/// and a block list looks them up again at each TCP connect or UDP send.
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
    resolvers: Resolvers,
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
            resolvers: Resolvers::default(),
        })
    }

    /// An allow list that lists nothing: no destination is reachable.
    pub fn empty() -> Self {
        Self {
            allowed_ips: HashSet::new(),
            hostnames: Vec::new(),
            learned_ips: Arc::new(Mutex::new(HashSet::new())),
            resolvers: Resolvers::default(),
        }
    }

    /// Also treat `ips` as DNS resolvers the guest may ask: the nameservers
    /// of the `resolv.conf` it was given (see `resolv_conf_nameservers`).
    pub fn exempt_resolvers(&self, ips: impl IntoIterator<Item = IpAddr>) {
        self.resolvers.extend(ips);
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

// ── Resolvers ──────────────────────────────────────────────────────

/// The resolvers a guest may ask on port 53 under an allow list or a
/// filter: the host's own, the well-known public ones, and those the guest
/// was given (its `/etc/resolv.conf`).
#[derive(Clone, Debug, Default)]
struct Resolvers(Arc<Mutex<HashSet<IpAddr>>>);

impl Resolvers {
    fn extend(&self, ips: impl IntoIterator<Item = IpAddr>) {
        if let Ok(mut resolvers) = self.0.lock() {
            resolvers.extend(ips.into_iter().map(canonical));
        }
    }

    fn contains(&self, ip: &IpAddr) -> bool {
        dns_resolvers().contains(ip) || self.0.lock().is_ok_and(|given| given.contains(ip))
    }
}

// ── Filter ─────────────────────────────────────────────────────────

/// Most names recorded for one address.  Hosts behind one CDN share
/// addresses; past this the first names seen are the ones that count.
const MAX_NAMES_PER_IP: usize = 16;

/// Decides what a guest under [`NetworkPolicy::Filter`] may reach.
///
/// The guest resolves names itself and connects to addresses, so the host
/// sees a name only in a DNS question.  The filter is asked twice:
/// whether the guest may ask about a name ([`allows_name`]), and whether
/// it may connect or send to an address ([`allows`]), given the names DNS
/// told it the address belongs to.  Both are asked each time, so a filter
/// whose answers change (an allow list the embedder edits while the guest
/// runs) takes effect at the next question, TCP connect or UDP send,
/// without a re-boot.
///
/// Both run while the guest is stopped in a host call: they should answer
/// from memory, not the network, and must not call into the sandbox.  A
/// panic in either counts as a refusal.  `allows_name` is asked again when
/// an answer arrives, before its addresses are credited to the name, so it
/// may be asked more than once per lookup.  Loopback, the unspecified
/// address and the cloud metadata addresses are refused before the filter
/// is asked, and questions to the resolvers an allow list exempts are
/// allowed, as under [`NetworkPolicy::AllowList`].
///
/// [`allows_name`]: NetworkFilter::allows_name
/// [`allows`]: NetworkFilter::allows
pub trait NetworkFilter: Send + Sync {
    /// Whether the guest may ask DNS about `name`, in lower case and
    /// without a trailing dot.  A name refused here is never resolved, so
    /// the guest has no address for it.
    fn allows_name(&self, name: &str) -> bool;

    /// Whether the guest may TCP-connect or send a datagram to `destination`.
    fn allows(&self, destination: &Destination<'_>) -> bool;
}

/// A destination a [`NetworkFilter`] is asked about.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Destination<'a> {
    /// The address, IPv4-mapped IPv6 given as IPv4.
    pub addr: SocketAddr,
    /// Whether the socket is a UDP one.
    pub udp: bool,
    /// The names the guest was told `addr` belongs to, from the answers
    /// to questions [`NetworkFilter::allows_name`] allowed, in lower case;
    /// empty for an address the guest did not get from DNS (a literal).
    pub names: &'a [String],
}

impl<'a> Destination<'a> {
    /// A destination as the host describes it to the filter, for an
    /// embedder testing its own.
    pub fn new(addr: SocketAddr, udp: bool, names: &'a [String]) -> Self {
        Self { addr, udp, names }
    }
}

/// A [`NetworkFilter`] with what the host records for it: the names DNS
/// gave each address, and the resolvers the guest may ask.
///
/// Clones share that record, so give each sandbox a `Filter` of its own;
/// [`Filter::from_arc`] shares the embedder's rules between them.
#[derive(Clone)]
pub struct Filter {
    filter: Arc<dyn NetworkFilter>,
    learned: Arc<Mutex<HashMap<IpAddr, Vec<String>>>>,
    resolvers: Resolvers,
}

impl Filter {
    pub fn new(filter: impl NetworkFilter + 'static) -> Self {
        Self::from_arc(Arc::new(filter))
    }

    /// A filter the embedder keeps a handle on, to change what it allows.
    pub fn from_arc(filter: Arc<dyn NetworkFilter>) -> Self {
        Self {
            filter,
            learned: Arc::default(),
            resolvers: Resolvers::default(),
        }
    }

    /// Also treat `ips` as DNS resolvers the guest may ask, as
    /// [`AllowList::exempt_resolvers`] does.
    pub fn exempt_resolvers(&self, ips: impl IntoIterator<Item = IpAddr>) {
        self.resolvers.extend(ips);
    }

    fn allows(&self, addr: SocketAddr, udp: bool) -> bool {
        if udp && addr.port() == 53 && self.resolvers.contains(&addr.ip()) {
            return true;
        }
        // Copied out, so the embedder's filter runs with no lock held.
        let names = self
            .learned
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&addr.ip())
            .cloned()
            .unwrap_or_default();
        ask(|| self.filter.allows(&Destination::new(addr, udp, &names)))
    }

    fn allows_name(&self, name: &str) -> bool {
        ask(|| self.filter.allows_name(name))
    }

    fn learn(&self, name: &str, ips: Vec<IpAddr>) {
        let mut learned = self.learned.lock().unwrap_or_else(|e| e.into_inner());
        for ip in ips {
            let ip = canonical(ip);
            if !learned.contains_key(&ip) && learned.len() >= MAX_LEARNED_IPS {
                continue;
            }
            let names = learned.entry(ip).or_default();
            if names.len() < MAX_NAMES_PER_IP && !names.iter().any(|n| n == name) {
                names.push(name.to_string());
            }
        }
    }
}

/// The embedder's answer, a panic counting as a refusal: it unwinds
/// through no host call, which would end the sandbox.
fn ask(question: impl FnOnce() -> bool) -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(question)).unwrap_or(false)
}

impl fmt::Debug for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Not waiting: this may be formatted from inside a filter's answer.
        let learned = self.learned.try_lock().map(|l| l.len()).ok();
        f.debug_struct("Filter")
            .field("learned_ips", &learned)
            .field("resolvers", &self.resolvers)
            .finish_non_exhaustive()
    }
}

// ── BlockList ──────────────────────────────────────────────────────

/// A set of blocked network destinations.
///
/// Like [`AllowList`], stores both literal IPs and hostnames.  The
/// hostnames decide which DNS questions the guest may not ask, and are
/// looked up again at every TCP connect or UDP send (see [`BlockList::blocks`]) for the
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
/// Without a `ListenPorts` allowlist, `net_bind` rejects every call but
/// an ephemeral one (port 0), a bind that only picks a source port
/// included: the guest can serve only on a port the host picks.  Single
/// ports, ranges and "every port" compose:
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

/// The name a DNS response answers and the addresses it gives for it, or
/// `None` if `data` is no response with answers.  Minimal parser: handles
/// standard A (type 1) and AAAA (type 28) answers, and credits every one to
/// the name asked, through any CNAME chain.
fn dns_answer_ips(data: &[u8]) -> Option<(String, Vec<IpAddr>)> {
    if data.len() < 12 {
        return None;
    }
    let flags = u16::from_be_bytes([data[2], data[3]]);
    let is_response = (flags & 0x8000) != 0;
    if !is_response {
        return None;
    }
    let qdcount = u16::from_be_bytes([data[4], data[5]]) as usize;
    let ancount = u16::from_be_bytes([data[6], data[7]]) as usize;
    if qdcount == 0 || ancount == 0 {
        return None;
    }

    // Parse question section to extract the queried name.
    let mut pos = 12;
    let qname = dns_read_name(data, &mut pos)?;
    // Skip QTYPE (2) + QCLASS (2)
    pos += 4;
    if pos > data.len() {
        return None;
    }

    // Parse answer records; a malformed one ends the list.
    let mut ips = Vec::new();
    for _ in 0..ancount {
        if dns_read_name(data, &mut pos).is_none() || pos + 10 > data.len() {
            break;
        }
        let rtype = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let rdlen = u16::from_be_bytes([data[pos + 8], data[pos + 9]]) as usize;
        pos += 10;
        if pos + rdlen > data.len() {
            break;
        }
        match rtype {
            1 if rdlen == 4 => {
                ips.push(IpAddr::V4(std::net::Ipv4Addr::new(
                    data[pos],
                    data[pos + 1],
                    data[pos + 2],
                    data[pos + 3],
                )));
            }
            28 if rdlen == 16 => {
                let mut octets = [0u8; 16];
                octets.copy_from_slice(&data[pos..pos + 16]);
                ips.push(IpAddr::V6(std::net::Ipv6Addr::from(octets)));
            }
            _ => {}
        }
        pos += rdlen;
    }
    Some((qname, ips))
}

/// Longest name, in its dotted text form, that DNS carries (RFC 1035's
/// 255 octets on the wire).
const MAX_NAME_LEN: usize = 253;

/// Read a DNS name at `pos`, advancing pos past it.
///
/// Only a name a resolver could be asked about reads: at most
/// [`MAX_NAME_LEN`] characters, in labels of printable ASCII with no dot of
/// their own, so two names on the wire never read as the same text (the
/// label `a.b` as the two labels `a` and `b`), and a forged message cannot
/// make a long one out of a compression loop.
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
        // 0x40 and 0x80: extended and reserved label types, not a length.
        if len & 0xC0 != 0 {
            return None;
        }
        p += 1;
        if p + len > data.len() {
            return None;
        }
        let label = &data[p..p + len];
        if !label.iter().all(|b| b.is_ascii_graphic() && *b != b'.') {
            return None;
        }
        if !name.is_empty() {
            name.push('.');
        }
        if name.len() + len > MAX_NAME_LEN {
            return None;
        }
        // Printable ASCII, checked above.
        name.extend(label.iter().map(|&b| char::from(b)));
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

    /// A filter that allows `name` on `port`, and the literal `ip` on any
    /// port; the names it is asked about are kept, to check what it saw.
    struct NameAndPort {
        name: String,
        port: u16,
        ip: Option<IpAddr>,
        allowed: Mutex<bool>,
    }

    impl NetworkFilter for NameAndPort {
        fn allows_name(&self, name: &str) -> bool {
            *self.allowed.lock().unwrap() && name == self.name
        }

        fn allows(&self, d: &Destination<'_>) -> bool {
            *self.allowed.lock().unwrap()
                && (Some(d.addr.ip()) == self.ip
                    || (d.addr.port() == self.port && d.names.contains(&self.name)))
        }
    }

    fn name_and_port(name: &str, port: u16, ip: Option<&str>) -> Arc<NameAndPort> {
        Arc::new(NameAndPort {
            name: name.into(),
            port,
            ip: ip.map(|ip| ip.parse().unwrap()),
            allowed: Mutex::new(true),
        })
    }

    /// Questions are put to the filter by name, as DNS spells it in lower
    /// case and without the trailing dot; a malformed one is refused.
    #[test]
    fn filter_judges_questions_by_name() {
        let policy = NetworkPolicy::Filter(Filter::new(NameAndPort {
            name: "api.example.com".into(),
            port: 443,
            ip: None,
            allowed: Mutex::new(true),
        }));
        assert!(policy.allows_query(&dns_query(&["api.example.com"])));
        assert!(policy.allows_query(&dns_query(&["API.Example.COM"])));
        assert!(!policy.allows_query(&dns_query(&["example.com"])));
        assert!(!policy.allows_query(&dns_query(&["api.example.com", "evil.example"])));
        assert!(!policy.allows_query(b"not dns"));
    }

    /// An address is allowed by the names DNS gave it, on the filter's
    /// terms, here one port; answers from a stranger and for names the
    /// filter refuses teach nothing.
    #[test]
    fn filter_judges_addresses_by_the_names_dns_gave_them() {
        let policy = NetworkPolicy::Filter(Filter::from_arc(name_and_port(
            "api.example.com",
            443,
            None,
        )));
        let resolver: SocketAddr = "8.8.8.8:53".parse().unwrap();
        let stranger: SocketAddr = "198.51.100.1:53".parse().unwrap();
        let told: SocketAddr = "203.0.113.9:443".parse().unwrap();
        let other: SocketAddr = "203.0.113.10:443".parse().unwrap();
        assert!(!policy.allows(&told, false));
        policy.learn_from_dns_answer(stranger, &dns_answer("api.example.com", told.ip()));
        assert!(!policy.allows(&told, false));
        policy.learn_from_dns_answer(resolver, &dns_answer("API.example.com", told.ip()));
        assert!(policy.allows(&told, false));
        // The filter's port, not any port.
        assert!(!policy.allows(&SocketAddr::new(told.ip(), 80), false));
        // A name the filter refuses is not credited with its addresses.
        policy.learn_from_dns_answer(resolver, &dns_answer("evil.example", other.ip()));
        assert!(!policy.allows(&other, false));
        // IPv4-mapped IPv6 is the IPv4 address.
        let mapped: SocketAddr = "[::ffff:203.0.113.9]:443".parse().unwrap();
        assert!(policy.allows(&mapped, false));
    }

    /// What the filter allows is asked again at every connect, so a change
    /// takes effect at once, without a new policy.
    #[test]
    fn filter_changes_take_effect_at_the_next_connect() {
        let filter = name_and_port("api.example.com", 443, Some("192.0.2.7"));
        let policy = NetworkPolicy::Filter(Filter::from_arc(filter.clone()));
        let literal: SocketAddr = "192.0.2.7:5432".parse().unwrap();
        assert!(policy.allows(&literal, false));
        *filter.allowed.lock().unwrap() = false;
        assert!(!policy.allows(&literal, false));
        assert!(!policy.allows_query(&dns_query(&["api.example.com"])));
    }

    /// Loopback and the metadata addresses are refused whatever the filter
    /// says; the exempt resolvers take questions on UDP only.
    #[test]
    fn filter_keeps_the_fixed_refusals_and_the_resolver_exemption() {
        let filter = Filter::new(NameAndPort {
            name: String::new(),
            port: 0,
            ip: None,
            allowed: Mutex::new(true),
        });
        filter.exempt_resolvers(["192.0.2.53".parse().unwrap()]);
        let policy = NetworkPolicy::Filter(filter);
        struct All;
        impl NetworkFilter for All {
            fn allows_name(&self, _: &str) -> bool {
                true
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                true
            }
        }
        let all = NetworkPolicy::Filter(Filter::new(All));
        for addr in [
            "127.0.0.1:80",
            "[::1]:80",
            "169.254.169.254:80",
            "[fd00:ec2::254]:80",
        ] {
            assert!(!all.allows(&addr.parse().unwrap(), false), "{addr}");
        }
        for resolver in ["8.8.8.8:53", "192.0.2.53:53"] {
            let resolver: SocketAddr = resolver.parse().unwrap();
            assert!(policy.allows(&resolver, true));
            assert!(!policy.allows(&resolver, false));
        }
    }

    /// The names recorded for one address are capped.
    #[test]
    fn filter_caps_the_names_per_address() {
        let filter = Filter::new(NameAndPort {
            name: String::new(),
            port: 0,
            ip: None,
            allowed: Mutex::new(true),
        });
        let ip: IpAddr = "203.0.113.9".parse().unwrap();
        for i in 0..MAX_NAMES_PER_IP + 4 {
            filter.learn(&format!("host{i}.example"), vec![ip]);
        }
        filter.learn("host0.example", vec![ip]);
        assert_eq!(filter.learned.lock().unwrap()[&ip].len(), MAX_NAMES_PER_IP);
    }

    /// A filter that panics refuses, and one that formats the policy while
    /// it answers does not wait on itself.
    #[test]
    fn filter_panics_refuse_and_formatting_inside_does_not_block() {
        struct Panics;
        impl NetworkFilter for Panics {
            fn allows_name(&self, _: &str) -> bool {
                panic!("filter bug")
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                panic!("filter bug")
            }
        }
        let policy = NetworkPolicy::Filter(Filter::new(Panics));
        assert!(!policy.allows(&"203.0.113.9:443".parse().unwrap(), false));
        assert!(!policy.allows_query(&dns_query(&["example.com"])));

        struct Formats(std::sync::OnceLock<Filter>);
        impl NetworkFilter for Formats {
            fn allows_name(&self, _: &str) -> bool {
                true
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                format!("{:?}", self.0.get().unwrap()).contains("Filter")
            }
        }
        let formats = Arc::new(Formats(std::sync::OnceLock::new()));
        let filter = Filter::from_arc(formats.clone());
        formats.0.set(filter.clone()).unwrap();
        let policy = NetworkPolicy::Filter(filter);
        policy.learn_from_dns_answer(
            "8.8.8.8:53".parse().unwrap(),
            &dns_answer("example.com", "203.0.113.9".parse().unwrap()),
        );
        assert!(policy.allows(&"203.0.113.9:443".parse().unwrap(), false));
    }

    /// The unspecified address reaches the host's loopback when connected
    /// to, so whatever refuses loopback refuses it too.
    #[test]
    fn the_unspecified_address_is_refused_with_loopback() {
        struct All;
        impl NetworkFilter for All {
            fn allows_name(&self, _: &str) -> bool {
                true
            }
            fn allows(&self, _: &Destination<'_>) -> bool {
                true
            }
        }
        let policies = [
            NetworkPolicy::AllowList(AllowList::from_hosts(&["0.0.0.0", "::"]).unwrap()),
            NetworkPolicy::BlockList(BlockList::from_hosts(&["1.2.3.4"]).unwrap()),
            NetworkPolicy::Filter(Filter::new(All)),
        ];
        for policy in &policies {
            for addr in ["0.0.0.0:443", "[::]:443", "[::ffff:0.0.0.0]:443"] {
                assert!(
                    !policy.allows(&addr.parse().unwrap(), false),
                    "{policy:?} {addr}"
                );
            }
        }
    }

    /// A question whose name could not be asked of a resolver is refused:
    /// a label with a dot in it, one with bytes that are not printable
    /// ASCII, a name longer than DNS carries.
    #[test]
    fn names_no_resolver_could_be_asked_are_refused() {
        let policy = NetworkPolicy::AllowList(AllowList::from_hosts(&["localhost"]).unwrap());
        let raw = |labels: &[&[u8]]| {
            let mut m = vec![0x12, 0x34, 0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0];
            for label in labels {
                m.push(label.len() as u8);
                m.extend_from_slice(label);
            }
            m.extend_from_slice(&[0, 0, 1, 0, 1]);
            m
        };
        assert!(policy.allows_query(&raw(&[b"localhost"])));
        // The one label "local.host" is not the name local.host.
        assert!(dns_question_names(&raw(&[b"local.host"])).is_none());
        assert!(dns_question_names(&raw(&[b"local\xffhost"])).is_none());
        assert!(dns_question_names(&raw(&[b"local host"])).is_none());
        let long = vec![b"a".repeat(63); 4];
        let labels: Vec<&[u8]> = long.iter().map(Vec::as_slice).collect();
        assert!(dns_question_names(&raw(&labels)).is_none());
        let fits = [
            b"a".repeat(63),
            b"a".repeat(63),
            b"a".repeat(63),
            b"a".repeat(61),
        ];
        let labels: Vec<&[u8]> = fits.iter().map(Vec::as_slice).collect();
        assert_eq!(
            dns_question_names(&raw(&labels)).unwrap()[0].len(),
            MAX_NAME_LEN
        );
    }
}
