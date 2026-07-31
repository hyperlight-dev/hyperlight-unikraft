//! hyperlight-unikraft: run Unikraft kernels on Hyperlight
//!
//! Provides a [`Sandbox`] wrapper around Hyperlight's `MultiUseSandbox`
//! that manages the kernel lifecycle: create → evolve (init) → snapshot
//! → call.
//!
//! # Quick start
//!
//! ```no_run
//! use hyperlight_unikraft::Sandbox;
//! # fn main() -> anyhow::Result<()> {
//! let mut sbox = Sandbox::builder("./kernel")
//!     .initrd_file("./initrd.cpio")
//!     .heap_size(256 * 1024 * 1024)
//!     .build()?;
//! sbox.restore()?;
//! sbox.call_run()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Snapshot lifecycle
//!
//! The sandbox keeps a live snapshot and lets you rewind to it. This
//! underpins [`pyhl`]'s fast cold start and every hermetic-per-call
//! pattern.
//!
//! ```text
//!   Sandbox::builder(..).build()   →  evolve (boot + init); post-evolve snapshot captured
//!                   │
//!                   ▼
//!              sbox.restore()      ←──┐  rewind to snapshot
//!                   │                 │
//!                   ▼                 │
//!              sbox.call_*(..)        │  dispatch (hermetic via restore)
//!                   │                 │
//!                   └─────────────────┘
//! ```
//!
//! After a warmup `call_*`, use [`Sandbox::snapshot_now`] to capture
//! post-warmup state — subsequent `restore()` rewinds to that point,
//! skipping the warmup on every call.
//!
//! To persist across processes:
//!
//! - [`Sandbox::save_snapshot`] writes the current snapshot to disk.
//! - [`Sandbox::from_snapshot_file`] recreates a sandbox straight from
//!   the file on disk, bypassing evolve entirely. This is how
//!   `pyhl run` starts in ~100ms without re-doing `Py_Initialize`.
//!
//! # Host filesystem
//!
//! The guest can access host directories via [`Preopen`] + the
//! `__dispatch` RPC. [`FsSandbox`] rejects path-escape attempts and
//! `normalize_fs_error` rewrites host-OS-specific error wording so
//! the cross-platform Unikraft guest classifies errors uniformly.

pub mod pyhl;
pub mod stderr_capture;

/// Re-export of Hyperlight's interrupt handle, returned by
/// [`Sandbox::interrupt_handle`]. Re-exported so downstream crates can name
/// the type without taking a direct dependency on `hyperlight-host`.
pub use hyperlight_host::hypervisor::InterruptHandle;

use anyhow::{anyhow, Result};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use hyperlight_host::func::Registerable;
use hyperlight_host::sandbox::snapshot::{OciTag, Snapshot};
use hyperlight_host::sandbox::uninitialized::GuestEnvironment;
use hyperlight_host::sandbox::SandboxConfiguration;
use hyperlight_host::{GuestBinary, HostFunctions, MultiUseSandbox, UninitializedSandbox};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Magic header for cmdline embedded in initrd: "HLCMDLN\0"
const CMDLINE_MAGIC: &[u8; 8] = b"HLCMDLN\0";

/// Magic header for the optional hostfs mount point TLV that follows the
/// cmdline (same init_data page).
const MOUNT_MAGIC: &[u8; 8] = b"HLHSMNT\0";

/// Magic header for the optional wall-clock-at-boot TLV. Value is a
/// little-endian u64 of nanoseconds since the Unix epoch. The guest
/// adds its own monotonic delta at read time, so `time.time()` returns
/// a sensible wall time without any host round-trip per call.
const WALLTIME_MAGIC: &[u8; 8] = b"HLWALL0\0";

const PAGE_SIZE: usize = 4096;

/// Guest paths that would shadow the kernel's own ramfs and break the VM.
/// Reject these early on the host before we even boot the guest.
const RESERVED_GUEST_MOUNTPOINTS: &[&str] = &["/", "/bin", "/dev", "/proc", "/sys", "/usr"];

/// Cap for `fs_read_bytes` allocation to prevent guest-controlled OOM (16 MiB).
const MAX_FS_READ: u64 = 16 * 1024 * 1024;

/// Cap for `net_send`/`net_sendto` decoded payload (1 MiB).
const MAX_NET_SEND: usize = 1024 * 1024;

/// Cap for `fs_write`/`fs_write_bytes` payload to prevent guest-triggered OOM (16 MiB).
const MAX_FS_WRITE: usize = 16 * 1024 * 1024;

/// Cap for `fs_truncate` length to prevent disk exhaustion (1 GiB).
const MAX_TRUNCATE_LEN: u64 = 1024 * 1024 * 1024;

/// Cap for incoming dispatch payload size (64 MiB).
const MAX_DISPATCH_PAYLOAD: usize = 64 * 1024 * 1024;
const MAX_PENDING_ASYNC_TASKS: usize = 1024;
const REQUEST_ID_HEX_LEN: usize = 16;
const ASYNC_FRAME_MAGIC: &[u8; 4] = b"HLAF";
const ASYNC_FRAME_VERSION: u8 = 1;
const ASYNC_FRAME_HEADER_LEN: usize = 20;
const ASYNC_FRAME_MAX_LEN: usize = 65536;
const ASYNC_FRAME_REQUEST: u8 = 1;
const ASYNC_FRAME_RESULT: u8 = 2;
const ASYNC_FRAME_PENDING: u8 = 3;
const ASYNC_FRAME_BATCH: u8 = 4;

struct AsyncFrame<'a> {
    kind: u8,
    id: u64,
    payload: &'a [u8],
}

fn decode_async_frame(input: &[u8]) -> Result<Option<AsyncFrame<'_>>> {
    if !input.starts_with(ASYNC_FRAME_MAGIC) {
        return Ok(None);
    }
    if input.len() < ASYNC_FRAME_HEADER_LEN {
        return Err(anyhow!("truncated async control frame"));
    }
    if input[4] != ASYNC_FRAME_VERSION {
        return Err(anyhow!(
            "unsupported async control frame version {}",
            input[4]
        ));
    }
    if input[6] != 0 || input[7] != 0 {
        return Err(anyhow!("async control frame reserved bits are nonzero"));
    }
    let id = u64::from_le_bytes(input[8..16].try_into().unwrap());
    let payload_len = u32::from_le_bytes(input[16..20].try_into().unwrap()) as usize;
    if input.len() != ASYNC_FRAME_HEADER_LEN + payload_len {
        return Err(anyhow!("async control frame length mismatch"));
    }
    Ok(Some(AsyncFrame {
        kind: input[5],
        id,
        payload: &input[ASYNC_FRAME_HEADER_LEN..],
    }))
}

fn encode_async_frame(kind: u8, id: u64, payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(ASYNC_FRAME_HEADER_LEN + payload.len());
    frame.extend_from_slice(ASYNC_FRAME_MAGIC);
    frame.push(ASYNC_FRAME_VERSION);
    frame.push(kind);
    frame.extend_from_slice(&[0, 0]);
    frame.extend_from_slice(&id.to_le_bytes());
    frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    frame.extend_from_slice(payload);
    frame
}

fn format_request_id(request_id: u64) -> String {
    format!("{request_id:016x}")
}

fn parse_request_id(value: &serde_json::Value, field: &str) -> Result<u64> {
    let text = value
        .as_str()
        .ok_or_else(|| anyhow!("'{field}' must be a 16-character lowercase hex string"))?;
    if text.len() != REQUEST_ID_HEX_LEN
        || !text
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(anyhow!(
            "'{field}' must be a 16-character lowercase hex string"
        ));
    }
    let request_id = u64::from_str_radix(text, 16)
        .map_err(|_| anyhow!("'{field}' contains an invalid hexadecimal request ID"))?;
    if request_id == 0 {
        return Err(anyhow!("'{field}' must encode a nonzero request ID"));
    }
    Ok(request_id)
}

/// Cap for `__hl_sleep` duration to prevent unbounded host-thread blocking (60 s).
const MAX_SLEEP_NS: u64 = 60_000_000_000;

/// Why one cooperative guest step returned control to the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollOutcome {
    /// The guest explicitly exited or halted without a scheduler yield.
    Exited,
    /// The scheduler is idle and has no timer or host call to wake it.
    Idle,
    /// The scheduler is idle until the reported relative deadline.
    Timer(Duration),
    /// One or more guest host calls are being driven off the vCPU thread.
    HostCallsPending {
        /// A concurrent guest timer, if one was reported by the scheduler.
        next_wakeup: Option<Duration>,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum GuestPollSignal {
    #[default]
    None,
    Yielded {
        deadline_ns: Option<u64>,
    },
    Exited,
}

type SharedPollSignal = Arc<Mutex<GuestPollSignal>>;

fn classify_poll_signal(signal: GuestPollSignal, host_calls_pending: bool) -> PollOutcome {
    let GuestPollSignal::Yielded { deadline_ns } = signal else {
        return PollOutcome::Exited;
    };
    let deadline = deadline_ns.map(Duration::from_nanos);
    if host_calls_pending {
        PollOutcome::HostCallsPending {
            next_wakeup: deadline,
        }
    } else if let Some(after) = deadline {
        PollOutcome::Timer(after)
    } else {
        PollOutcome::Idle
    }
}

/// Shared cancellation primitive for `__hl_sleep`. Calling
/// [`SleepCancel::cancel`] wakes up any in-progress sleep immediately so
/// the host function returns and the hypervisor execution loop can detect
/// the pending cancellation.
#[derive(Clone)]
pub struct SleepCancel(Arc<(Mutex<bool>, Condvar)>);

impl SleepCancel {
    fn new() -> Self {
        Self(Arc::new((Mutex::new(false), Condvar::new())))
    }

    /// Wake any in-progress `__hl_sleep` immediately.
    pub fn cancel(&self) {
        let (lock, cvar) = &*self.0;
        *lock.lock().unwrap() = true;
        cvar.notify_all();
    }

    /// Reset so the next guest call can sleep normally.
    pub fn reset(&self) {
        *self.0 .0.lock().unwrap() = false;
    }

    fn wait(&self, dur: Duration) {
        let (lock, cvar) = &*self.0;
        let guard = lock.lock().unwrap();
        if *guard {
            return;
        }
        // wait_timeout_while handles spurious wakeups by re-checking the
        // predicate; we only return early when actually cancelled.
        let _ = cvar.wait_timeout_while(guard, dur, |cancelled| !*cancelled);
    }
}

/// Cap for `fs_list` directory entries to prevent host OOM on huge directories.
const MAX_DIR_ENTRIES: usize = 100_000;

/// Default socket timeout for read/write/connect operations (30 s).
const SOCKET_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Re-entry interval used when the inter-step readiness wait has filtered out a
/// terminally-EOF socket. Short enough that a guest which *does* still want to
/// observe the close sees it promptly, long enough to turn a full-rate poll spin
/// into a park.
const EOF_RECHECK: std::time::Duration = std::time::Duration::from_millis(50);

/// A preopened host directory exposed to the guest.
///
/// Semantics mirror Wasmtime's `preopened_dir`: `host_dir` is canonicalised
/// at construction time and used as the sandbox root for every RPC the
/// guest issues; `guest_path` is the absolute path inside the guest where
/// `lib/hostfs` mounts it.
#[derive(Clone, Debug)]
pub struct Preopen {
    pub host_dir: std::path::PathBuf,
    pub guest_path: String,
    pub read_only: bool,
}

impl Preopen {
    /// Construct a preopen. `guest_path` must be absolute (`/something`)
    /// and not shadow a reserved kernel directory — see
    /// `RESERVED_GUEST_MOUNTPOINTS`.
    pub fn new<P: AsRef<Path>>(host_dir: P, guest_path: impl Into<String>) -> Result<Self> {
        let guest_path = guest_path.into();
        if !guest_path.starts_with('/') {
            return Err(anyhow!(
                "guest mount path {:?} must be absolute",
                guest_path
            ));
        }
        for reserved in RESERVED_GUEST_MOUNTPOINTS {
            if guest_path == *reserved || guest_path.starts_with(&format!("{}/", reserved)) {
                return Err(anyhow!(
                    "refusing to mount at guest path {:?}: shadows reserved kernel dir",
                    guest_path
                ));
            }
        }
        let host_dir = std::fs::canonicalize(host_dir.as_ref()).map_err(|e| {
            anyhow!(
                "canonicalize preopen host dir {:?}: {}",
                host_dir.as_ref(),
                e
            )
        })?;
        Ok(Self {
            host_dir,
            guest_path,
            read_only: false,
        })
    }

    /// Mark this preopen as read-only.
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Parse a `HOST[:GUEST]` CLI argument. When `GUEST` is omitted the
    /// default guest mount point is `/host`.
    pub fn parse_cli(s: &str) -> Result<Self> {
        // Windows absolute paths contain ':'. Disambiguate by splitting on
        // the *last* colon only if the right side looks like an absolute
        // guest path (starts with /). Otherwise treat the whole string as
        // the host dir.
        if let Some(idx) = s.rfind(':') {
            let (host, guest) = s.split_at(idx);
            let guest = &guest[1..];
            if guest.starts_with('/') {
                return Self::new(host, guest);
            }
        }
        Self::new(s, "/host")
    }
}

// ---------------------------------------------------------------------------
// Network policy
// ---------------------------------------------------------------------------

/// Controls which network destinations a guest sandbox can reach.
///
/// By default, networking is **disabled** (no `net_*` tools are registered).
/// Callers must opt in via [`SandboxBuilder::network`] or the `--net` CLI flag.
#[derive(Clone, Debug)]
pub enum NetworkPolicy {
    /// All outbound connections are allowed (no filtering).
    AllowAll,
    /// Only connections to the listed destinations are permitted.
    AllowList(AllowList),
    /// All connections are allowed *except* to the listed destinations.
    BlockList(BlockList),
}

/// A set of allowed network destinations.
///
/// Stores both literal IPs and hostnames. At check time, hostnames are
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
    /// Hostnames are verified to be resolvable at construction time
    /// (fail-closed). At check time they are re-resolved so CDN/anycast
    /// rotation doesn't cause false denials.
    pub fn from_hosts(entries: &[impl AsRef<str>]) -> Result<Self> {
        use std::net::ToSocketAddrs;
        let mut allowed_ips = HashSet::new();
        let mut hostnames = Vec::new();
        for entry in entries {
            let entry = entry.as_ref();
            if let Ok(ip) = entry.parse::<IpAddr>() {
                allowed_ips.insert(ip);
            } else {
                let addrs = (entry, 0u16)
                    .to_socket_addrs()
                    .map_err(|e| anyhow!("resolve {:?}: {}", entry, e))?;
                let mut found = false;
                for sa in addrs {
                    allowed_ips.insert(sa.ip());
                    found = true;
                }
                if !found {
                    return Err(anyhow!("hostname {:?} resolved to zero addresses", entry));
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
        if let Ok(learned) = self.learned_ips.lock() {
            if learned.contains(ip) {
                return true;
            }
        }
        // Re-resolve hostnames to catch CDN/anycast IP rotation.
        use std::net::ToSocketAddrs;
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

    fn learn_ip(&self, ip: IpAddr) {
        if let Ok(mut learned) = self.learned_ips.lock() {
            if learned.len() < MAX_LEARNED_IPS {
                learned.insert(ip);
            }
        }
    }
}

/// A set of blocked network destinations.
///
/// Like [`AllowList`], stores both literal IPs and hostnames. At check
/// time, hostnames are re-resolved so the policy tracks DNS changes.
#[derive(Clone, Debug)]
pub struct BlockList {
    blocked_ips: HashSet<IpAddr>,
    hostnames: Vec<String>,
}

impl BlockList {
    /// Build a blocklist from a mixed set of hostnames and IP literals.
    ///
    /// Hostnames are verified to be resolvable at construction time
    /// (fail-closed). At check time they are re-resolved so CDN/anycast
    /// rotation doesn't cause false passes.
    pub fn from_hosts(entries: &[impl AsRef<str>]) -> Result<Self> {
        use std::net::ToSocketAddrs;
        let mut blocked_ips = HashSet::new();
        let mut hostnames = Vec::new();
        for entry in entries {
            let entry = entry.as_ref();
            if let Ok(ip) = entry.parse::<IpAddr>() {
                blocked_ips.insert(ip);
            } else {
                let addrs = (entry, 0u16)
                    .to_socket_addrs()
                    .map_err(|e| anyhow!("resolve {:?}: {}", entry, e))?;
                let mut found = false;
                for sa in addrs {
                    blocked_ips.insert(sa.ip());
                    found = true;
                }
                if !found {
                    return Err(anyhow!("hostname {:?} resolved to zero addresses", entry));
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
        use std::net::ToSocketAddrs;
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

/// DNS resolver IPs that the AllowList exempts on port 53.
///
/// Includes the host's configured resolvers (from `/etc/resolv.conf` on
/// Unix, `ipconfig /all` on Windows) **plus** well-known public DNS
/// servers (Google, Cloudflare) that the guest may hardcode in its own
/// `/etc/resolv.conf`.
fn dns_resolvers() -> &'static HashSet<IpAddr> {
    static RESOLVERS: std::sync::OnceLock<HashSet<IpAddr>> = std::sync::OnceLock::new();
    RESOLVERS.get_or_init(|| {
        let mut set = HashSet::new();
        // Well-known public DNS that the guest's initrd may hardcode.
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
                    if let Some(rest) = line.strip_prefix("nameserver") {
                        if let Some(ip_str) = rest.split_whitespace().next() {
                            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                                set.insert(ip);
                            }
                        }
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

impl NetworkPolicy {
    fn check(&self, addr: &std::net::SocketAddr) -> Result<()> {
        // Normalise IPv4-mapped IPv6 (`::ffff:a.b.c.d`) to the IPv4 address it
        // actually reaches. Linux routes those to the IPv4 stack, so without
        // this every rule below could be evaded by rewriting the destination in
        // mapped form: `::ffff:127.0.0.1` reached host loopback services, and
        // `::ffff:169.254.169.254` reached instance metadata. Doing it here
        // covers the allow/block lists too, not just the unconditional denials.
        //
        // Only the mapped form needs unwrapping. The deprecated IPv4-compatible
        // form (`::a.b.c.d`) is not routed as IPv4, and unwrapping it here would
        // be actively wrong -- `Ipv6Addr::to_ipv4` maps `::1` to `0.0.0.1`,
        // which would turn loopback into an ordinary address.
        let ip = match addr.ip() {
            std::net::IpAddr::V6(v6) => v6
                .to_ipv4_mapped()
                .map(std::net::IpAddr::V4)
                .unwrap_or(std::net::IpAddr::V6(v6)),
            v4 => v4,
        };

        // Block link-local addresses (169.254.0.0/16, fe80::/10) for all
        // policy variants.  In cloud environments the IPv4 link-local range
        // hosts the instance metadata service (169.254.169.254) which hands
        // out credentials without authentication.
        let is_link_local = match ip {
            std::net::IpAddr::V4(v4) => v4.is_link_local(),
            std::net::IpAddr::V6(v6) => {
                let seg = v6.segments();
                (seg[0] & 0xffc0) == 0xfe80
            }
        };
        if is_link_local {
            return Err(anyhow!(
                "network policy denies connection to link-local address {}",
                addr
            ));
        }

        // Block loopback addresses (127.0.0.0/8, ::1) for all policy
        // variants. Host-local services typically trust loopback traffic
        // and perform no authentication.
        if ip.is_loopback() {
            return Err(anyhow!(
                "network policy denies connection to loopback address {}",
                addr
            ));
        }

        match self {
            NetworkPolicy::AllowAll => Ok(()),
            NetworkPolicy::AllowList(al) => {
                if al.is_allowed(&ip) || (addr.port() == 53 && dns_resolvers().contains(&ip)) {
                    Ok(())
                } else {
                    Err(anyhow!("network policy denies connection to {}", addr))
                }
            }
            NetworkPolicy::BlockList(bl) => {
                if bl.is_blocked(&ip) {
                    Err(anyhow!("network policy denies connection to {}", addr))
                } else {
                    Ok(())
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Listen-port allowlist (inbound)
// ---------------------------------------------------------------------------

/// Controls which ports a guest may bind to for inbound connections.
///
/// Orthogonal to [`NetworkPolicy`] (which governs *outbound* destinations).
/// Without a `ListenPorts` allowlist, `net_bind` / `net_listen` /
/// `net_accept` are still registered but `net_bind` rejects every call.
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
    fn check(&self, port: u16) -> Result<()> {
        if self.ports.contains(&port) {
            Ok(())
        } else {
            Err(anyhow!(
                "Permission denied: port {} not in listen allowlist ({:?})",
                port,
                self.ports
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Configuration for a Unikraft VM.
#[non_exhaustive]
pub struct VmConfig {
    pub heap_size: u64,
    pub stack_size: u64,
    pub io_buffer_size: usize,
}

/// Hyperlight I/O buffer size (128 KiB). Each host function call is serialized
/// into a FlatBuffer and pushed onto a shared-memory stack whose capacity is
/// `io_buffer_size`. The hostfs VFS layer chunks writes at 32 KiB, but after
/// base64 encoding + JSON envelope + FlatBuffer framing a single chunk
/// occupies ~44 KiB. The Hyperlight SDK default (16 KiB) is too small; 128 KiB
/// accommodates any single-chunk RPC with comfortable headroom.
const DEFAULT_IO_BUFFER_SIZE: usize = 128 * 1024;

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            heap_size: 512 * 1024 * 1024,
            stack_size: 8 * 1024 * 1024,
            io_buffer_size: DEFAULT_IO_BUFFER_SIZE,
        }
    }
}

impl VmConfig {
    /// Set the guest heap size in bytes. Convenience chainable setter
    /// for building a `VmConfig` inline.
    pub fn with_heap_size(mut self, size: u64) -> Self {
        self.heap_size = size;
        self
    }

    /// Set the guest stack size in bytes. Chainable setter.
    pub fn with_stack_size(mut self, size: u64) -> Self {
        self.stack_size = size;
        self
    }

    /// Set the I/O buffer size for host function calls (default 128 KiB).
    pub fn with_io_buffer_size(mut self, size: usize) -> Self {
        self.io_buffer_size = size;
        self
    }

    fn sandbox_config(&self) -> SandboxConfiguration {
        let mut cfg = SandboxConfiguration::default();
        cfg.set_heap_size(self.heap_size);
        cfg.set_input_data_size(self.io_buffer_size);
        cfg.set_output_data_size(self.io_buffer_size);

        // Scratch holds page tables + CoW copies of writable pages touched at
        // runtime.  pt_estimate covers page tables; the base covers kernel
        // boot, CPIO extraction, ELF loading, and language runtime startup.
        // Use 25% of heap as base: large guests (e.g. Node.js) load 100+ MB
        // ELF binaries whose PT_LOAD segments trigger per-page CoW copies.
        let pt_estimate = ((self.heap_size as usize / (2 * 1024 * 1024)) + 16) * PAGE_SIZE;
        let base = std::cmp::max(self.heap_size as usize / 4, 64 * 1024 * 1024);
        let scratch = (pt_estimate + base).next_multiple_of(PAGE_SIZE);
        cfg.set_scratch_size(scratch);
        cfg
    }
}

/// Parse memory size string (e.g., "512Mi", "1Gi") into bytes.
pub fn parse_memory(mem_str: &str) -> Result<u64> {
    let s = mem_str.trim();
    if let Some(v) = s.strip_suffix("Gi") {
        Ok(v.parse::<u64>()? * 1024 * 1024 * 1024)
    } else if let Some(v) = s.strip_suffix("Mi") {
        Ok(v.parse::<u64>()? * 1024 * 1024)
    } else if let Some(v) = s.strip_suffix("Ki") {
        Ok(v.parse::<u64>()? * 1024)
    } else if let Some(v) = s.strip_suffix("G") {
        Ok(v.parse::<u64>()? * 1_000_000_000)
    } else if let Some(v) = s.strip_suffix("M") {
        Ok(v.parse::<u64>()? * 1_000_000)
    } else if let Some(v) = s.strip_suffix("K") {
        Ok(v.parse::<u64>()? * 1000)
    } else {
        s.parse()
            .map_err(|e| anyhow!("Invalid memory format: {}", e))
    }
}

// ---------------------------------------------------------------------------
// Initrd cmdline prepend
// ---------------------------------------------------------------------------

/// Serialize the shared "cmdline + preopens + wall clock" TLV block into `buf`.
///
/// Layout:
///   [HLCMDLN\0][cmdline_len u32][cmdline…][\0]
///   [HLHSMNT\0][count u32]([path_len u32][path…][\0])*count  (optional block)
///   [HLWALL0\0][8 u32][wall_ns_le u64]
///
/// Callers are responsible for any trailing padding / metadata (e.g. the
/// mapped-initrd-size footer used by `build_cmdline_initdata`).
fn write_cmdline_mount_tlv(buf: &mut Vec<u8>, cmdline_bytes: &[u8], preopens: &[Preopen]) {
    let cmdline_len = cmdline_bytes.len() as u32;
    buf.extend_from_slice(CMDLINE_MAGIC);
    buf.extend_from_slice(&cmdline_len.to_le_bytes());
    buf.extend_from_slice(cmdline_bytes);
    buf.push(0);

    if !preopens.is_empty() {
        buf.extend_from_slice(MOUNT_MAGIC);
        buf.extend_from_slice(&(preopens.len() as u32).to_le_bytes());
        for p in preopens {
            let b = p.guest_path.as_bytes();
            buf.extend_from_slice(&(b.len() as u32).to_le_bytes());
            buf.extend_from_slice(b);
            buf.push(0);
        }
    }

    // Wall clock: read the host's time once at VM build time and embed
    // as ns since epoch. The guest will add its own monotonic delta.
    let wall_ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    buf.extend_from_slice(WALLTIME_MAGIC);
    buf.extend_from_slice(&8u32.to_le_bytes());
    buf.extend_from_slice(&wall_ns.to_le_bytes());
}

/// Build init_data with cmdline + preopens + mapped initrd size (for
/// map_file_cow mode). The mapped file size is stored in the last 8
/// bytes of the page-aligned header.
fn build_cmdline_initdata(
    app_args: &[String],
    mapped_initrd_size: u64,
    preopens: &[Preopen],
) -> Option<Vec<u8>> {
    let cmdline = app_args.join(" ");
    if cmdline.is_empty() && mapped_initrd_size == 0 && preopens.is_empty() {
        return None;
    }

    let cmdline_bytes = cmdline.as_bytes();
    let mut buf = Vec::new();
    write_cmdline_mount_tlv(&mut buf, cmdline_bytes, preopens);

    let padded = (buf.len() + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    buf.resize(padded - 8, 0);
    buf.extend_from_slice(&mapped_initrd_size.to_le_bytes());
    Some(buf)
}

/// Prepend application arguments + preopens as a header in the initrd.
pub fn prepend_cmdline_to_initrd(
    initrd: Option<&[u8]>,
    app_args: &[String],
    preopens: &[Preopen],
) -> Option<Vec<u8>> {
    let cmdline = app_args.join(" ");

    if cmdline.is_empty() && initrd.is_none() && preopens.is_empty() {
        return None;
    }
    if cmdline.is_empty() && preopens.is_empty() {
        return initrd.map(|d| d.to_vec());
    }

    let cmdline_bytes = cmdline.as_bytes();
    let mut buf = Vec::new();
    write_cmdline_mount_tlv(&mut buf, cmdline_bytes, preopens);

    let padded = (buf.len() + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    buf.resize(padded, 0);
    if let Some(data) = initrd {
        buf.extend_from_slice(data);
    }
    Some(buf)
}

// ---------------------------------------------------------------------------
// Tool dispatch (host functions callable from guest)
// ---------------------------------------------------------------------------

/// Registry of tool handlers callable from guest user-space via `/dev/hcall`.
pub struct ToolRegistry {
    tools:
        HashMap<String, Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value> + Send + Sync>>,
    /// Optional async-tool dispatch side (populated when the builder registers
    /// any [`SandboxBuilder::tool_async`] handler). Async tools return a yield
    /// completion request ID immediately; their futures are driven off the vCPU
    /// thread by [`Sandbox::drive_host_functions`].
    async_side: Option<AsyncDispatch>,
}

enum ToolDispatchOutcome {
    Complete(serde_json::Value),
    Pending(u64),
}

impl ToolRegistry {
    /// Create an empty registry. Add handlers with
    /// [`register`](Self::register) before wiring it into a sandbox.
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            async_side: None,
        }
    }

    /// Register a named handler. The handler receives the JSON-encoded
    /// `args` payload the guest sent and returns a `serde_json::Value`
    /// that becomes the `{"result": ...}` portion of the response.
    /// Errors returned by the handler become `{"error": "..."}`.
    pub fn register<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value> + Send + Sync + 'static,
    {
        self.tools.insert(name.to_string(), Box::new(handler));
    }

    /// Get (creating if necessary) the async-dispatch side of this registry.
    ///
    /// The async side is shared (via `Arc`) with the [`AsyncToolState`] driver
    /// installed on the [`Sandbox`], so tools registered here are driven off
    /// the vCPU thread by [`Sandbox::drive_host_functions`].
    fn async_side_mut(&mut self) -> &mut AsyncDispatch {
        self.async_side.get_or_insert_with(|| AsyncDispatch {
            factories: HashMap::new(),
            pending: Arc::new(Mutex::new(HashMap::new())),
            queue: Arc::new(Mutex::new(std::collections::VecDeque::new())),
        })
    }

    /// Register an async tool factory. On invocation the tool returns a yield
    /// completion request ID immediately (off-loading any blocking work), and its
    /// future is driven by [`Sandbox::drive_host_functions`]. Used for the
    /// networking tools (natively async on the Tokio reactor), `__hl_sleep`,
    /// and user [`SandboxBuilder::tool_async`] handlers.
    fn register_async_factory(&mut self, name: &str, factory: ToolFactory) {
        self.async_side_mut()
            .factories
            .insert(name.to_string(), factory);
    }

    /// Build the driver-side [`AsyncToolState`] that shares this registry's
    /// pending/queue maps, or `None` if no async tools are registered.
    fn make_async_state(&self) -> Option<AsyncToolState> {
        self.async_side.as_ref().map(|a| AsyncToolState {
            pending: a.pending.clone(),
            queue: a.queue.clone(),
            running: FuturesUnordered::new(),
            factories: a.factories.clone(),
        })
    }

    /// Test helper: dispatch a request and, if the tool yielded an async
    /// completion request ID, drive its queued future to completion on a temporary
    /// runtime and return the resolved response in the same
    /// `{"result": …}` / `{"error": …}` shape the guest receives when the
    /// completion is delivered in the next `poll` batch. Lets synchronous unit
    /// tests exercise the now-async tools (e.g. `__hl_sleep`) without a full
    /// poll loop.
    ///
    /// Async tools require a binary request frame around the JSON tool payload.
    #[cfg(test)]
    fn dispatch_drive(&self, payload: &[u8]) -> Vec<u8> {
        let resp = self.dispatch(payload);
        let Ok(Some(frame)) = decode_async_frame(&resp) else {
            return resp;
        };
        if frame.kind != ASYNC_FRAME_PENDING {
            return resp;
        }
        let token_id = frame.id;
        let Some(asy) = &self.async_side else {
            return resp;
        };
        let fut = asy.queue.lock().unwrap().pop_front();
        if let Some((tok, fut)) = fut {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            let res = rt.block_on(fut);
            if let Some(task) = asy.pending.lock().unwrap().get_mut(&tok) {
                task.state = PendingState::Ready(res.map_err(|e| e.to_string()));
            }
        }
        // The completion is delivered to the guest in the next poll batch as
        // {"result": …} / {"error": …}; reproduce that shape here.
        let value = match asy
            .pending
            .lock()
            .unwrap()
            .remove(&token_id)
            .map(|t| t.state)
        {
            Some(PendingState::Ready(Ok(v))) => serde_json::json!({ "result": v }),
            Some(PendingState::Ready(Err(e))) => serde_json::json!({ "error": e }),
            _ => return resp,
        };
        let payload = serde_json::to_vec(&value).unwrap();
        encode_async_frame(ASYNC_FRAME_RESULT, token_id, &payload)
    }

    /// Decode a guest-side `__dispatch` request, look up the handler by
    /// name, invoke it, and encode the response as JSON bytes.
    ///
    /// Cooperative requests use a binary async-control frame carrying a
    /// guest-assigned nonzero request ID and a JSON tool payload. Legacy
    /// synchronous callers may still pass the JSON tool payload directly.
    ///
    /// Responses are `{"result": <value>}` or `{"error": "<msg>"}`.
    /// Unknown tool names, JSON errors, missing/malformed/zero IDs, and duplicate
    /// in-flight IDs all become error responses; this function never panics.
    ///
    /// Set `HL_DISPATCH_DEBUG=1` in the environment to dump each call's
    /// payload and result to stderr — useful when diagnosing
    /// guest/host protocol mismatches.
    pub fn dispatch(&self, payload: &[u8]) -> Vec<u8> {
        if payload.len() > MAX_DISPATCH_PAYLOAD {
            return serde_json::to_vec(&serde_json::json!({
                "error": format!("payload too large: {} bytes (max {})", payload.len(), MAX_DISPATCH_PAYLOAD)
            }))
            .unwrap_or_default();
        }
        let debug = std::env::var("HL_DISPATCH_DEBUG")
            .ok()
            .map(|v| v == "1")
            .unwrap_or(false);
        if debug {
            let preview = if payload.len() > 200 {
                &payload[..200]
            } else {
                payload
            };
            eprintln!(
                "[__dispatch] payload.len={} preview={:?}",
                payload.len(),
                std::str::from_utf8(preview).unwrap_or("<non-utf8>")
            );
        }
        let decoded_frame = match decode_async_frame(payload) {
            Ok(frame) => frame,
            Err(e) => {
                let json = serde_json::to_vec(&serde_json::json!({
                    "error": e.to_string()
                }))
                .unwrap_or_default();
                return encode_async_frame(ASYNC_FRAME_RESULT, 0, &json);
            }
        };
        let (json_payload, request_id) = match decoded_frame.as_ref() {
            Some(frame) if frame.kind == ASYNC_FRAME_REQUEST && frame.id != 0 => {
                (frame.payload, Some(frame.id))
            }
            Some(frame) => {
                let reason = if frame.id == 0 {
                    "request ID must be nonzero".to_string()
                } else {
                    format!("unexpected frame kind {}", frame.kind)
                };
                let json = serde_json::to_vec(&serde_json::json!({
                    "error": format!("invalid async request frame: {reason}")
                }))
                .unwrap_or_default();
                return encode_async_frame(ASYNC_FRAME_RESULT, frame.id, &json);
            }
            None => (payload, None),
        };
        let result = (|| -> Result<ToolDispatchOutcome> {
            let req: serde_json::Value = serde_json::from_slice(json_payload)?;
            let name = req["name"]
                .as_str()
                .ok_or_else(|| anyhow!("missing 'name'"))?;
            let args = req.get("args").cloned().unwrap_or(serde_json::Value::Null);
            // Async tools: require a cooperative request ID, then dispatch.
            // Sync tools: fall through to the sync handler table below.
            if let Some(asy) = &self.async_side {
                if asy.factories.contains_key(name) {
                    match request_id {
                        // Cooperative guest: register the task and let it
                        // park; the result arrives in a later poll batch.
                        Some(id) => {
                            asy.dispatch_async(id, name, &args)?;
                            return Ok(ToolDispatchOutcome::Pending(id));
                        }
                        // Legacy synchronous guest (built without
                        // CONFIG_HYPERLIGHT_POLL): it blocks the vCPU thread
                        // on this call and has no drive loop to resolve a
                        // request ID, so run the tool to completion here and
                        // answer inline. This is the same blocking behaviour
                        // these guests had before the tools became async.
                        None => {
                            let factory = asy.factories.get(name).unwrap();
                            return block_on_tool_future(factory(args))
                                .map(ToolDispatchOutcome::Complete);
                        }
                    }
                }
            }
            let handler = self
                .tools
                .get(name)
                .ok_or_else(|| anyhow!("unknown tool: {}", name))?;
            Ok(ToolDispatchOutcome::Complete(handler(args)?))
        })();
        if debug {
            match &result {
                Ok(ToolDispatchOutcome::Complete(v)) => {
                    eprintln!("[__dispatch] OK: {}", v)
                }
                Ok(ToolDispatchOutcome::Pending(id)) => {
                    eprintln!("[__dispatch] PENDING: {id}")
                }
                Err(e) => eprintln!("[__dispatch] ERR: {}", e),
            }
        }
        let json = match result {
            Ok(ToolDispatchOutcome::Complete(v)) => serde_json::json!({ "result": v }),
            Ok(ToolDispatchOutcome::Pending(id)) => {
                return encode_async_frame(ASYNC_FRAME_PENDING, id, &[]);
            }
            Err(e) => {
                // Normalize common error strings so the cross-platform
                // Unikraft guest doesn't depend on host-OS-specific
                // wording to classify the error.
                //
                // The guest's `lib/hostfs` substring-matches on the
                // error payload to pick a POSIX errno. On Linux the
                // wording is the canonical "No such file or directory";
                // on Windows Rust produces "The system cannot find the
                // file specified.", which fell through the match and
                // triggered a fatal-error path in vfscore (observed
                // crash at hostfs-posix-c:open /host/greeting.txt).
                //
                // Keep the underlying error code (`os error N`) in the
                // string so downstream debugging stays faithful.
                serde_json::json!({ "error": normalize_fs_error(&e.to_string()) })
            }
        };
        let json = serde_json::to_vec(&json)
            .unwrap_or_else(|_| b"{\"error\":\"serialization failed\"}".to_vec());
        match request_id {
            Some(id) => encode_async_frame(ASYNC_FRAME_RESULT, id, &json),
            None => json,
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A boxed, `Send` future produced by an async tool handler.
type ToolFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<serde_json::Value>> + Send>>;
type RunningToolFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = (u64, Result<serde_json::Value>)> + Send>>;

/// Runtime used to resolve async tool futures for legacy synchronous guests.
///
/// Created on first use and kept for the process lifetime: these calls arrive
/// on the vCPU thread one at a time, so a single worker is sufficient.
fn legacy_dispatch_runtime() -> Result<&'static tokio::runtime::Runtime> {
    static RT: std::sync::OnceLock<Option<tokio::runtime::Runtime>> = std::sync::OnceLock::new();
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .ok()
    })
    .as_ref()
    .ok_or_else(|| anyhow!("could not start a runtime to resolve a synchronous host call"))
}

/// Run `fut` to completion, blocking the calling thread.
///
/// The future is spawned onto a dedicated runtime and the result collected
/// over a channel, rather than calling `Runtime::block_on` directly, so this
/// cannot panic with "cannot start a runtime from within a runtime" when the
/// caller already happens to be inside one.
fn block_on_tool_future(fut: ToolFuture) -> Result<serde_json::Value> {
    let rt = legacy_dispatch_runtime()?;
    let (tx, rx) = std::sync::mpsc::channel();
    rt.spawn(async move {
        let _ = tx.send(fut.await);
    });
    rx.recv()
        .unwrap_or_else(|_| Err(anyhow!("async tool future was dropped before completion")))
}

/// A boxed async-tool factory: called with the guest's `args`, returns the
/// future that computes the result. `Arc`-wrapped so the same factory can be
/// shared with the off-vCPU driver ([`AsyncToolState`]) and re-invoked to
/// resume a *pending* task after a snapshot restore (see
/// [`Sandbox::restore_async_tasks`]).
type ToolFactory = Arc<dyn Fn(serde_json::Value) -> ToolFuture + Send + Sync>;

/// Build an async-tool factory from a synchronous, potentially-blocking
/// handler by running it on Tokio's blocking thread pool
/// ([`tokio::task::spawn_blocking`]). Used for `__hl_sleep`, whose timer wait
/// (and socket-readiness poll) is a genuinely blocking operation: the guest
/// call yields its guest-provided request ID immediately and the wait runs off the vCPU
/// thread, so it never stalls the host executor.
///
/// The networking tools no longer use this — they are natively async on
/// Tokio's `net` reactor (see [`register_net_tools`]).
///
/// The handler must be `Clone` (it is re-invoked once per call) and must not
/// hold any lock across the blocking call.
fn blocking_tool_factory<H>(handler: H) -> ToolFactory
where
    H: Fn(serde_json::Value) -> Result<serde_json::Value> + Clone + Send + Sync + 'static,
{
    Arc::new(move |args| {
        let h = handler.clone();
        Box::pin(async move {
            match tokio::task::spawn_blocking(move || h(args)).await {
                Ok(res) => res,
                Err(e) => Err(anyhow!("blocking tool task failed: {e}")),
            }
        })
    })
}

/// State of an in-flight async tool call, keyed by its guest-provided request ID.
#[derive(Debug)]
enum PendingState {
    /// The future is still being driven by `drive_host_functions`.
    Running,
    /// The future has resolved; the next `poll` batch delivers this to the
    /// guest (`Ok` → `{"result": …}`, `Err` → `{"error": …}`).
    Ready(std::result::Result<serde_json::Value, String>),
}

/// A tracked async task: the originating tool `name` and `args` plus its
/// current [`PendingState`]. Held in the shared `pending` map so a checkpoint
/// can capture every in-flight (`Running`) and completed-but-undelivered
/// (`Ready`) task, including enough information to resume or re-deliver it on
/// restore (see [`Sandbox::export_async_tasks`] /
/// [`Sandbox::restore_async_tasks`]).
struct PendingTask {
    /// The async tool that was invoked (e.g. `net_recv`).
    name: String,
    /// The JSON arguments the guest passed — enough to re-invoke the tool.
    args: serde_json::Value,
    /// Whether the task is still running or has a result waiting for delivery.
    state: PendingState,
}

/// The registry-side (synchronous, vCPU-thread) half of async tool support.
///
/// Lives inside [`ToolRegistry`] so [`ToolRegistry::dispatch`] can, entirely
/// synchronously, turn a call to an async tool into a validated request ID +
/// a queued future + a yield sentinel. It never blocks on a future — the
/// futures are driven off-thread by [`Sandbox::drive_host_functions`], which
/// shares `pending`/`queue`; their results are delivered to the guest in the
/// next `poll` batch (see [`Sandbox::drain_completion_batch`]).
struct AsyncDispatch {
    factories: HashMap<String, ToolFactory>,
    pending: Arc<Mutex<HashMap<u64, PendingTask>>>,
    queue: Arc<Mutex<std::collections::VecDeque<(u64, ToolFuture)>>>,
}

impl AsyncDispatch {
    /// Registers `request_id` and queues its future to be driven off-thread.
    ///
    /// The pending entry is established **before** the future is pushed to the
    /// queue so that the driver can always find an entry for any token it
    /// dequeues, even if it races with the dispatch thread.
    fn dispatch_async(&self, request_id: u64, name: &str, args: &serde_json::Value) -> Result<()> {
        // self.factories.get(name) is guaranteed Some by the caller check.
        let factory = self.factories.get(name).unwrap();
        let mut pending = self.pending.lock().unwrap();
        if pending.contains_key(&request_id) {
            return Err(anyhow!(
                "duplicate in-flight request ID {}: a task with this ID is already pending",
                request_id
            ));
        }
        if pending.len() >= MAX_PENDING_ASYNC_TASKS {
            return Err(anyhow!(
                "too many in-flight async tasks (max {})",
                MAX_PENDING_ASYNC_TASKS
            ));
        }
        // Establish the pending entry BEFORE queuing the future so the driver
        // always finds a record for every token it dequeues.
        pending.insert(
            request_id,
            PendingTask {
                name: name.to_string(),
                args: args.clone(),
                state: PendingState::Running,
            },
        );
        drop(pending);
        let fut = factory(args.clone());
        self.queue.lock().unwrap().push_back((request_id, fut));
        Ok(())
    }
}

/// The sandbox-side (async, off-vCPU) half of async tool support: owns the
/// running futures. Shares `pending`/`queue` with the [`AsyncDispatch`] in the
/// registry. Driven by [`Sandbox::drive_host_functions`].
struct AsyncToolState {
    pending: Arc<Mutex<HashMap<u64, PendingTask>>>,
    queue: Arc<Mutex<std::collections::VecDeque<(u64, ToolFuture)>>>,
    running: FuturesUnordered<RunningToolFuture>,
    /// Clone of the registry's async-tool factories, so a task that was still
    /// `Running` at snapshot time can be re-invoked (with its saved args) to
    /// resume on restore (see [`Sandbox::restore_async_tasks`]).
    factories: HashMap<String, ToolFactory>,
}

impl AsyncToolState {
    /// Serialize every tracked task (pending + completed-but-undelivered) into
    /// `{"tasks": [ … ]}`. Backing implementation of
    /// [`Sandbox::export_async_tasks`] — see that method for the entry shapes.
    fn export_tasks(&self) -> serde_json::Value {
        use serde_json::json;
        let pending = self.pending.lock().unwrap();
        let tasks: Vec<serde_json::Value> = pending
            .iter()
            .map(|(token, task)| match &task.state {
                PendingState::Running => json!({
                    "token": format_request_id(*token),
                    "name": task.name,
                    "args": task.args,
                    "status": "pending",
                }),
                PendingState::Ready(Ok(v)) => json!({
                    "token": format_request_id(*token),
                    "name": task.name,
                    "args": task.args,
                    "status": "completed",
                    "result": v,
                }),
                PendingState::Ready(Err(e)) => json!({
                    "token": format_request_id(*token),
                    "name": task.name,
                    "args": task.args,
                    "status": "completed",
                    "error": e,
                }),
            })
            .collect();
        json!({ "tasks": tasks })
    }

    /// Repopulate the tracker from an [`export_tasks`](Self::export_tasks)
    /// value. Backing implementation of [`Sandbox::restore_async_tasks`] — see
    /// that method for the per-task restore rules.
    ///
    /// The restore is **transactional**: the entire snapshot is validated
    /// before any shared state is mutated. A snapshot is rejected (and nothing
    /// is restored — no partial state is left behind) if any entry has a
    /// malformed or zero hexadecimal `token`, if two entries share a token, or if a token
    /// collides with a task already in-flight in the current registry. Only
    /// after full validation are the pending map and drive queue mutated, under
    /// a single held lock so the commit is atomic against concurrent dispatch.
    fn restore_tasks(&mut self, data: &serde_json::Value) -> Result<()> {
        let tasks = data["tasks"]
            .as_array()
            .ok_or_else(|| anyhow!("task snapshot missing 'tasks' array"))?;
        if tasks.len() > MAX_PENDING_ASYNC_TASKS {
            return Err(anyhow!(
                "task snapshot contains too many async tasks: {} (max {})",
                tasks.len(),
                MAX_PENDING_ASYNC_TASKS
            ));
        }

        // Hold the pending lock across validate + commit so the duplicate check
        // against the live registry and the eventual insert are one atomic step
        // (no window for a concurrent dispatch to insert a colliding token).
        let mut pending = self.pending.lock().unwrap();
        if pending.len().saturating_add(tasks.len()) > MAX_PENDING_ASYNC_TASKS {
            return Err(anyhow!(
                "restoring {} async tasks would exceed the per-sandbox limit of {}",
                tasks.len(),
                MAX_PENDING_ASYNC_TASKS
            ));
        }

        // Phase 1 — validate every entry before touching `pending` or `queue`.
        // On any error we return here, having mutated nothing.
        let mut seen: std::collections::HashSet<u64> =
            std::collections::HashSet::with_capacity(tasks.len());
        for entry in tasks {
            let token = parse_request_id(&entry["token"], "token")?;
            if !seen.insert(token) {
                return Err(anyhow!(
                    "duplicate token {} within task snapshot: refusing partial restore",
                    token
                ));
            }
            if pending.contains_key(&token) {
                return Err(anyhow!(
                    "duplicate token {} in task snapshot: would overwrite an existing in-flight task",
                    token
                ));
            }
        }

        // Phase 2 — all entries validated; build the pending records and any
        // resume futures. Nothing below can fail, so there is no partial-restore
        // window. Futures are collected first and pushed to the shared queue in
        // one batch under the queue lock at the end.
        let mut new_futures: Vec<(u64, ToolFuture)> = Vec::new();
        for entry in tasks {
            // `token` was validated as a nonzero hexadecimal u64 in phase 1.
            let token = parse_request_id(&entry["token"], "token").unwrap();
            let name = entry["name"].as_str().unwrap_or("").to_string();
            let args = entry
                .get("args")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            let status = entry["status"].as_str().unwrap_or("pending");

            let state = if status == "completed" {
                // Result already computed pre-checkpoint — restore it verbatim
                // so the next poll batch re-delivers it.
                if let Some(err) = entry.get("error").and_then(|e| e.as_str()) {
                    PendingState::Ready(Err(err.to_string()))
                } else {
                    let result = entry
                        .get("result")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null);
                    PendingState::Ready(Ok(result))
                }
            } else if let Some(factory) = self.factories.get(&name) {
                // Pending and the tool still exists — re-invoke it with the
                // saved args so the work resumes off the vCPU thread.
                let fut = factory(args.clone());
                new_futures.push((token, fut));
                PendingState::Running
            } else {
                // Pending but the tool can't be re-driven after restore
                // (e.g. a user async tool, not re-registered): deliver an error
                // so the guest unparks rather than hanging on a dead token.
                PendingState::Ready(Err(format!(
                    "async task '{name}' could not be resumed after restore"
                )))
            };

            pending.insert(token, PendingTask { name, args, state });
        }

        if !new_futures.is_empty() {
            let mut queue = self.queue.lock().unwrap();
            for (token, fut) in new_futures {
                queue.push_back((token, fut));
            }
        }
        Ok(())
    }
}

/// Rewrite host-OS-specific error wording to the canonical Linux form
/// so the Unikraft guest's `lib/hostfs` can classify errors by substring
/// match without caring which host it's running on. Linux wording is
/// canonical because that's what the guest was written against.
///
/// Only rewrites the message when we can identify the error by its
/// `os error N` suffix (that `N` is the POSIX errno — cross-platform).
/// Otherwise passes the string through unchanged so unusual errors are
/// still visible in debug output.
fn normalize_fs_error(s: &str) -> String {
    // Map: POSIX errno -> canonical Linux std::io::Error wording.
    //
    //   2  ENOENT  "No such file or directory"
    //  13  EACCES  "Permission denied"
    //  17  EEXIST  "File exists"
    //  20  ENOTDIR "Not a directory"
    //  21  EISDIR  "Is a directory"
    //  39  ENOTEMPTY "Directory not empty"
    const MAP: &[(&str, &str)] = &[
        ("(os error 2)", "No such file or directory"),
        ("(os error 13)", "Permission denied"),
        ("(os error 17)", "File exists"),
        ("(os error 20)", "Not a directory"),
        ("(os error 21)", "Is a directory"),
        ("(os error 39)", "Directory not empty"),
    ];
    for (marker, canonical) in MAP {
        if s.contains(marker) {
            // Keep the prefix (e.g., `fs_stat "/host/greeting.txt":`) so
            // debugging is still legible; just replace the body wording.
            if let Some(idx) = s.find(": ") {
                let prefix = &s[..idx];
                return format!("{prefix}: {canonical} {marker}");
            }
            return format!("{canonical} {marker}");
        }
    }
    s.to_string()
}

// ---------------------------------------------------------------------------
// Filesystem sandbox — Phase A of host-mediated POSIX FS access
// ---------------------------------------------------------------------------

/// A sandboxed view of a host directory that the guest can read/write via
/// host function calls. All guest-supplied paths are resolved relative to
/// `root`; any attempt to escape the root (`..`, absolute paths, symlinks
/// pointing outside) is rejected.
///
/// Phase A deliberately exposes an explicit RPC surface: the guest calls
/// `fs_read` / `fs_write` / `fs_list` / `fs_stat` / `fs_mkdir` / `fs_unlink`
/// by name. Phase B will add a transparent POSIX shim in Unikraft that
/// forwards VFS operations to these same host handlers.
#[derive(Clone)]
pub struct FsSandbox {
    root: std::path::PathBuf,
}

impl FsSandbox {
    /// Create a new sandbox rooted at `root` (must be an existing directory).
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self> {
        let root = std::fs::canonicalize(root.as_ref())
            .map_err(|e| anyhow!("canonicalize mount root {:?}: {}", root.as_ref(), e))?;
        if !root.is_dir() {
            return Err(anyhow!("mount root is not a directory: {:?}", root));
        }
        Ok(Self { root })
    }

    /// The canonicalized host-side root directory. All guest-supplied
    /// paths are resolved relative to this; escapes are rejected.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Resolve a guest-supplied path to a host path that is guaranteed to
    /// live under `root`. Returns an error on any escape attempt.
    ///
    /// Strategy:
    ///  - Strip any leading `/` so guest paths are relative to the mount.
    ///  - Logically normalise `.` / `..` without touching the filesystem.
    ///  - If the resolved path exists, `canonicalize` to follow symlinks
    ///    and verify the target is under `root`.
    ///  - If it doesn't exist (e.g. creating a new file), canonicalise the
    ///    nearest existing ancestor and append the remaining components —
    ///    this still catches symlinked ancestors that escape the root.
    pub(crate) fn resolve(&self, guest_path: &str) -> Result<std::path::PathBuf> {
        use std::path::{Component, PathBuf};
        let rel = guest_path.trim_start_matches('/');
        let joined = self.root.join(rel);
        // Logical resolution first: reject ".." once we're rooted.
        let mut logical = PathBuf::new();
        for c in joined.components() {
            match c {
                Component::ParentDir => {
                    if !logical.pop() {
                        return Err(anyhow!("path escapes mount root: {:?}", guest_path));
                    }
                }
                Component::CurDir => {}
                c => logical.push(c),
            }
        }
        if !logical.starts_with(&self.root) {
            return Err(anyhow!("path escapes mount root: {:?}", guest_path));
        }
        // Symlink check: canonicalise the deepest existing ancestor.
        let mut existing = logical.as_path();
        let mut tail: Vec<&std::ffi::OsStr> = Vec::new();
        let resolved_ancestor = loop {
            if existing.exists() {
                break std::fs::canonicalize(existing)
                    .map_err(|e| anyhow!("canonicalize {:?}: {}", existing, e))?;
            }
            let Some(name) = existing.file_name() else {
                return Err(anyhow!("path has no existing ancestor: {:?}", logical));
            };
            tail.push(name);
            existing = existing
                .parent()
                .ok_or_else(|| anyhow!("path has no existing ancestor: {:?}", logical))?;
        };
        if !resolved_ancestor.starts_with(&self.root) {
            return Err(anyhow!(
                "path escapes mount root (symlink): {:?}",
                guest_path
            ));
        }
        let mut out = resolved_ancestor;
        for name in tail.into_iter().rev() {
            out.push(name);
            // Walk the symlink chain (with hop limit) to catch escapes
            // through dangling or chained symlinks.
            const MAX_SYMLINK_HOPS: usize = 40;
            let mut cursor = out.clone();
            for _ in 0..MAX_SYMLINK_HOPS {
                let Ok(meta) = std::fs::symlink_metadata(&cursor) else {
                    break;
                };
                if !meta.file_type().is_symlink() {
                    break;
                }
                let target = std::fs::read_link(&cursor)?;
                let abs = if target.is_absolute() {
                    target
                } else {
                    cursor.parent().unwrap_or(&self.root).join(&target)
                };
                let mut norm = std::path::PathBuf::new();
                for c in abs.components() {
                    match c {
                        std::path::Component::ParentDir => {
                            norm.pop();
                        }
                        std::path::Component::CurDir => {}
                        c => norm.push(c),
                    }
                }
                if !norm.starts_with(&self.root) {
                    return Err(anyhow!(
                        "symlink target escapes mount root: {:?}",
                        guest_path
                    ));
                }
                cursor = norm;
            }
        }
        Ok(out)
    }
}

/// Internal helper: assemble the final tool registry from caller-supplied
/// tools plus any preopened directories. Multiple preopens share one set
/// of fs_* tool handlers that route by guest-path prefix: the handler
/// inspects the `path` argument, finds the matching preopen, and
/// resolves the tail under that host directory.
fn build_tools(
    user_tools: Option<ToolRegistry>,
    preopens: &[Preopen],
) -> Result<Option<ToolRegistry>> {
    if preopens.is_empty() {
        return Ok(user_tools);
    }
    let mut registry = user_tools.unwrap_or_default();
    let router = FsRouter::new(preopens)?;
    router.register(&mut registry);
    Ok(Some(registry))
}

/// Register internal tools (`__hl_exit`, `__hl_sleep`) on a tool registry.
/// These are plumbing used by the guest driver (`hl_pydriver.c`) and are
/// always present regardless of user-supplied tools or preopens.
///
/// Networking tools are only registered when a [`NetworkPolicy`] is provided.
fn register_internal_tools(
    tools: &mut ToolRegistry,
    exit_code: &Arc<AtomicI32>,
    poll_signal: &SharedPollSignal,
    sleep_cancel: &SleepCancel,
    network: Option<&NetworkPolicy>,
    listen_ports: Option<&ListenPorts>,
) -> Option<Arc<Mutex<SocketTable>>> {
    let ec = exit_code.clone();
    let exit_signal = poll_signal.clone();
    tools.register("__hl_exit", move |args| {
        let code = args["code"].as_i64().unwrap_or(1) as i32;
        ec.store(code, Ordering::Relaxed);
        *exit_signal.lock().unwrap() = GuestPollSignal::Exited;
        Ok(serde_json::json!({}))
    });
    // Cooperative poll model: the guest reports the nanoseconds until its
    // next scheduler wakeup right before it yields the vCPU back to the host.
    // 0 means no pending timer; 1 is the guest's minimum nonzero value for a
    // timer already due, causing an immediate re-poll. `poll` reads this to
    // decide how long to wait before the next `poll`. See plat/hyperlight/poll.c.
    let yield_signal = poll_signal.clone();
    tools.register("__hl_poll_yield", move |args| {
        let ns = args
            .get("ns")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| anyhow!("__hl_poll_yield: ns must be a non-negative integer"))?;
        let mut signal = yield_signal.lock().unwrap();
        match *signal {
            GuestPollSignal::None => {
                *signal = GuestPollSignal::Yielded {
                    deadline_ns: (ns != 0).then_some(ns),
                };
            }
            // An explicit exit is authoritative. The idle pump may still yield
            // later in the same dispatch while unwinding the scheduler.
            GuestPollSignal::Exited => {}
            GuestPollSignal::Yielded { .. } => {
                return Err(anyhow!(
                    "__hl_poll_yield: guest yielded more than once in one poll step"
                ));
            }
        }
        Ok(serde_json::json!({}))
    });
    // Create socket table early so __hl_sleep can poll sockets while sleeping.
    let socket_table = network.map(|_| Arc::new(Mutex::new(SocketTable::new())));

    let sc = sleep_cancel.clone();
    let st_for_sleep = socket_table.clone();
    // Async: under the cooperative poll model the sleep runs off the vCPU
    // thread (blocking pool via `spawn_blocking`) and is driven to completion
    // by `Sandbox::drive_host_functions`. Legacy guests without a poll loop
    // call it unframed, and `ToolRegistry::dispatch` resolves it inline.
    tools.register_async_factory(
        "__hl_sleep",
        blocking_tool_factory(move |args| {
            let ns = args["ns"].as_u64().unwrap_or(0).min(MAX_SLEEP_NS);
            if ns > 0 {
                if let Some(ref table) = st_for_sleep {
                    return hl_sleep_poll_sockets(&sc, table, ns);
                }
                sc.wait(Duration::from_nanos(ns));
            }
            Ok(serde_json::json!({}))
        }),
    );

    if let (Some(policy), Some(ref table)) = (network, &socket_table) {
        register_net_tools(tools, policy, listen_ports, table.clone());
    }
    socket_table
}

// ---------------------------------------------------------------------------
// Host-proxied networking (hostsock)
// ---------------------------------------------------------------------------

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpSocket, TcpStream, UdpSocket};

/// A connected/bound socket wrapped in the appropriate Tokio type for
/// datagram-vs-stream I/O. Built on demand from a `socket2` dup by
/// [`dup_into_tokio`].
enum TokioSock {
    Tcp(TcpStream),
    Udp(UdpSocket),
}

/// Duplicate the socket behind `fd` out of the table and return the dup plus
/// its recorded `sock_type`, dropping the table lock before the caller awaits.
///
/// Every dup shares the same underlying kernel socket object, so connection
/// state established on a dup (connect, the accepted stream, a UDP peer) is
/// reflected on the table's original fd, and closing the dup afterwards leaves
/// the socket open as long as the table still references it. Only one I/O op
/// per socket is ever in flight (single guest thread), so the dup can't race.
fn dup_socket(table: &Mutex<SocketTable>, fd: u64) -> Result<(Socket, i32)> {
    let tbl = table.lock().unwrap();
    let sock = tbl.get_socket(fd)?.try_clone()?;
    let sock_type = tbl.get_sock_type(fd)?;
    Ok((sock, sock_type))
}

/// Convert a `socket2` dup into a Tokio stream/datagram socket for async I/O.
/// Sets the dup non-blocking (a prerequisite for the Tokio reactor); this only
/// affects the shared kernel socket's blocking mode, not its socket options.
fn dup_into_tokio(sock: Socket, sock_type: i32) -> std::io::Result<TokioSock> {
    sock.set_nonblocking(true)?;
    // SOCK_DGRAM = 2
    if sock_type == 2 {
        let std: std::net::UdpSocket = sock.into();
        Ok(TokioSock::Udp(UdpSocket::from_std(std)?))
    } else {
        let std: std::net::TcpStream = sock.into();
        Ok(TokioSock::Tcp(TcpStream::from_std(std)?))
    }
}

/// Convert a `socket2` dup into a not-yet-connected Tokio [`TcpSocket`] so an
/// async `connect` preserves any binds/sockopts already applied to the socket.
fn dup_into_tcp_socket(sock: Socket) -> std::io::Result<TcpSocket> {
    sock.set_nonblocking(true)?;
    #[cfg(unix)]
    {
        use std::os::unix::io::{FromRawFd, IntoRawFd};
        Ok(unsafe { TcpSocket::from_raw_fd(sock.into_raw_fd()) })
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::{FromRawSocket, IntoRawSocket};
        Ok(unsafe { TcpSocket::from_raw_socket(sock.into_raw_socket()) })
    }
}

/// Await a socket I/O future bounded by [`SOCKET_TIMEOUT`], mapping an expiry
/// to a `WouldBlock` error so the guest sees the same error class it did under
/// the previous `SO_*TIMEO`-based blocking implementation.
async fn io_timeout<T>(fut: impl std::future::Future<Output = std::io::Result<T>>) -> Result<T> {
    match tokio::time::timeout(SOCKET_TIMEOUT, fut).await {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err(anyhow!(std::io::Error::from(
            std::io::ErrorKind::WouldBlock
        ))),
    }
}

struct HostSocket {
    socket: Socket,
    sock_type: i32,
    /// Address this socket is bound to, if it has been `bind()`-ed.
    /// Recorded so listening sockets can be re-created on restore (the
    /// host socket table is not part of the guest snapshot).
    bound_addr: Option<SocketAddr>,
    /// `listen()` backlog, set once the socket becomes a listener.
    /// `Some` marks this entry as a listener for [`SocketTable::export_listeners`].
    listen_backlog: Option<i32>,
}

impl HostSocket {
    fn new(socket: Socket, sock_type: i32) -> Self {
        Self {
            socket,
            sock_type,
            bound_addr: None,
            listen_backlog: None,
        }
    }
}

/// Maximum number of IPs learned from DNS responses for AllowList policy.
const MAX_LEARNED_IPS: usize = 256;

const MAX_SOCKETS: usize = 1024;

struct SocketTable {
    sockets: HashMap<u64, HostSocket>,
    next_id: u64,
}

impl SocketTable {
    fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            next_id: 1,
        }
    }

    fn insert(&mut self, sock: HostSocket) -> Result<u64> {
        if self.sockets.len() >= MAX_SOCKETS {
            return Err(anyhow!("socket limit reached (max {})", MAX_SOCKETS));
        }
        let id = self.next_id;
        self.next_id += 1;
        self.sockets.insert(id, sock);
        Ok(id)
    }

    fn clear(&mut self) {
        self.sockets.clear();
        self.next_id = 1;
    }

    fn get(&self, fd: u64) -> Result<&HostSocket> {
        self.sockets
            .get(&fd)
            .ok_or_else(|| anyhow!("bad_fd: {}", fd))
    }

    fn get_socket(&self, fd: u64) -> Result<&Socket> {
        Ok(&self.get(fd)?.socket)
    }

    fn get_sock_type(&self, fd: u64) -> Result<i32> {
        Ok(self.get(fd)?.sock_type)
    }

    fn get_mut(&mut self, fd: u64) -> Result<&mut HostSocket> {
        self.sockets
            .get_mut(&fd)
            .ok_or_else(|| anyhow!("bad_fd: {}", fd))
    }

    /// Serialize the currently-listening sockets (and the id counter)
    /// to JSON so they can be re-created on the host after a restore.
    ///
    /// Only listening sockets are exported: a checkpoint of an idle
    /// server has just its listener(s) open, and established/accepted
    /// connections cannot be revived across a restore anyway. The guest
    /// keeps using the same fd (= table id) for its listener, so we must
    /// re-create each listener under its original id.
    fn export_listeners(&self) -> serde_json::Value {
        use serde_json::json;
        let listeners: Vec<serde_json::Value> = self
            .sockets
            .iter()
            .filter_map(|(id, sock)| {
                let backlog = sock.listen_backlog?;
                let addr = sock.bound_addr?;
                let family: i32 = match addr {
                    SocketAddr::V4(_) => 2,
                    SocketAddr::V6(_) => 10,
                };
                Some(json!({
                    "id": id,
                    "family": family,
                    "sock_type": sock.sock_type,
                    "addr": addr.ip().to_string(),
                    "port": addr.port(),
                    "backlog": backlog,
                }))
            })
            .collect();
        json!({ "next_id": self.next_id, "listeners": listeners })
    }

    /// Re-create listening sockets described by [`Self::export_listeners`]
    /// and re-insert them under their original ids, restoring `next_id`.
    /// Called on restore, before the guest resumes, so the resumed guest
    /// finds its listener fds backed by real, bound, listening sockets.
    fn restore_listeners(&mut self, data: &serde_json::Value) -> Result<()> {
        let listeners = data["listeners"]
            .as_array()
            .ok_or_else(|| anyhow!("listener snapshot missing 'listeners' array"))?;
        for entry in listeners {
            let id = entry["id"]
                .as_u64()
                .ok_or_else(|| anyhow!("listener entry missing 'id'"))?;
            let family = entry["family"].as_i64().unwrap_or(2) as i32;
            let sock_type = entry["sock_type"].as_i64().unwrap_or(1) as i32;
            let backlog = entry["backlog"].as_i64().unwrap_or(128) as i32;
            let addr = parse_sockaddr(entry)?;

            let domain = match family {
                2 => Domain::IPV4,
                10 => Domain::IPV6,
                _ => {
                    return Err(anyhow!(
                        "unsupported family {} in listener snapshot",
                        family
                    ))
                }
            };
            let stype = match sock_type {
                1 => Type::STREAM,
                2 => Type::DGRAM,
                _ => {
                    return Err(anyhow!(
                        "unsupported type {} in listener snapshot",
                        sock_type
                    ))
                }
            };
            let sock = Socket::new(domain, stype, None)?;
            // The original listener's host port may still be in TIME_WAIT
            // from the torn-down VM; allow rebinding it immediately.
            sock.set_reuse_address(true)?;
            sock.set_read_timeout(Some(SOCKET_TIMEOUT))?;
            sock.set_write_timeout(Some(SOCKET_TIMEOUT))?;
            let sa: SockAddr = addr.into();
            sock.bind(&sa)
                .map_err(|e| anyhow!("re-binding restored listener to {addr}: {e}"))?;
            sock.listen(backlog)
                .map_err(|e| anyhow!("re-listening restored listener on {addr}: {e}"))?;
            self.sockets.insert(
                id,
                HostSocket {
                    socket: sock,
                    sock_type,
                    bound_addr: Some(addr),
                    listen_backlog: Some(backlog),
                },
            );
        }
        if let Some(next_id) = data["next_id"].as_u64() {
            self.next_id = self.next_id.max(next_id);
        }
        Ok(())
    }

    fn remove(&mut self, fd: u64) -> Result<()> {
        self.sockets
            .remove(&fd)
            .map(|_| ())
            .ok_or_else(|| anyhow!("bad_fd: {}", fd))
    }
}

fn parse_sockaddr(args: &serde_json::Value) -> Result<SocketAddr> {
    let addr_str = args["addr"]
        .as_str()
        .ok_or_else(|| anyhow!("missing 'addr'"))?;
    let port = args["port"].as_u64().unwrap_or(0) as u16;
    let ip: std::net::IpAddr = addr_str.parse().map_err(|e| anyhow!("bad addr: {}", e))?;
    Ok(SocketAddr::new(ip, port))
}

fn sockaddr_to_json(addr: SocketAddr) -> serde_json::Value {
    let family: i32 = match addr {
        SocketAddr::V4(_) => 2,
        SocketAddr::V6(_) => 10,
    };
    serde_json::json!({
        "family": family,
        "addr": addr.ip().to_string(),
        "port": addr.port(),
    })
}

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
    let is_allowed_host = al.hostnames.iter().any(|h| h.to_lowercase() == qname_lower);
    if !is_allowed_host {
        return;
    }

    // Parse answer records and learn IPs.
    for _ in 0..ancount {
        // Skip name (may be a pointer)
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

/// Read a DNS name at `pos`, advancing pos past it. Returns the decoded name.
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

// ---------------------------------------------------------------------------
// Named handler functions for each net_* tool.
// ---------------------------------------------------------------------------

fn handle_net_socket(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let family = args["family"].as_i64().unwrap_or(2) as i32; // AF_INET=2
    let sock_type = args["type"].as_i64().unwrap_or(1) as i32; // SOCK_STREAM=1
    let protocol = args["protocol"].as_i64().unwrap_or(0) as i32;

    let domain = match family {
        2 => Domain::IPV4,
        10 => Domain::IPV6,
        _ => return Err(anyhow!("InvalidInput: unsupported family {}", family)),
    };
    let stype = match sock_type {
        1 => Type::STREAM,
        2 => Type::DGRAM,
        _ => return Err(anyhow!("InvalidInput: unsupported type {}", sock_type)),
    };
    let proto = if protocol == 0 {
        None
    } else {
        Some(Protocol::from(protocol))
    };
    let sock = Socket::new(domain, stype, proto)?;
    sock.set_read_timeout(Some(SOCKET_TIMEOUT))?;
    sock.set_write_timeout(Some(SOCKET_TIMEOUT))?;
    let fd = table
        .lock()
        .unwrap()
        .insert(HostSocket::new(sock, sock_type))?;
    Ok(json!({ "fd": fd }))
}

/// Answer musl's `AI_ADDRCONFIG` probe without exposing loopback to the guest.
///
/// `getaddrinfo(AI_ADDRCONFIG)` decides whether to request A and/or AAAA
/// records by asking "is this family configured?", and musl phrases that
/// question as: open a UDP socket and `connect()` it to that family's loopback
/// address, port 65535. See musl `src/network/getaddrinfo.c`.
///
/// We deny loopback to guests on purpose, so that probe used to come back as a
/// policy error. musl only accepts `EADDRNOTAVAIL`/`EAFNOSUPPORT`/
/// `EHOSTUNREACH`/`ENETDOWN`/`ENETUNREACH` as "family unconfigured" and turns
/// anything else into `EAI_SYSTEM`, so the whole lookup failed. Node's HTTP
/// client sets `AI_ADDRCONFIG` whenever no explicit `family` is given, which is
/// why `http.get` by hostname failed while `dns.lookup` worked.
///
/// Answering with an accepted errno instead would not help: both families would
/// then report "unconfigured" and musl returns `EAI_NODATA`. The probe has to
/// be able to *succeed*.
///
/// So run the probe on the host, on a throwaway socket. The host does all of
/// the guest's routing, so its answer is the correct one — this makes guest
/// `getaddrinfo` behave exactly as it would natively on the host. The guest's
/// own socket is never connected to loopback, so no data path is opened, and
/// `net_send`/`net_sendto` cannot reach loopback afterwards.
fn addrconfig_probe(addr: &std::net::SocketAddr) -> Result<serde_json::Value> {
    use serde_json::json;

    let domain = match addr {
        std::net::SocketAddr::V4(_) => Domain::IPV4,
        std::net::SocketAddr::V6(_) => Domain::IPV6,
    };
    let probed = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))
        .and_then(|s| s.connect(&(*addr).into()));

    match probed {
        Ok(()) => Ok(json!({})),
        // "AddrNotAvail" is mapped to EADDRNOTAVAIL by the guest driver, which
        // is on musl's accepted list, so the family is reported unconfigured
        // rather than failing the lookup outright.
        Err(e) => Err(anyhow!(
            "AddrNotAvail: address family not configured on the host ({})",
            e
        )),
    }
}

/// Does this connect match musl's `AI_ADDRCONFIG` probe exactly?
///
/// Kept as narrow as the probe itself (UDP, loopback, port 65535) so that any
/// other connect to loopback still gets an honest denial. See
/// [`addrconfig_probe`].
fn is_addrconfig_probe(sock_type: i32, addr: &std::net::SocketAddr) -> bool {
    // SOCK_DGRAM = 2
    sock_type == 2 && addr.port() == 65535 && addr.ip().is_loopback()
}

async fn handle_net_connect(
    table: Arc<Mutex<SocketTable>>,
    policy: Arc<NetworkPolicy>,
    args: serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let addr = parse_sockaddr(&args)?;
    // Dup the socket out and drop the table lock before the async connect, so
    // the lock is never held across an await. The dup shares the kernel socket,
    // so the connection it establishes is visible on the table's fd.
    let (sock, sock_type) = dup_socket(&table, fd)?;
    // A UDP connect to loopback:65535 transmits nothing; it is musl's
    // AI_ADDRCONFIG routability probe. Answer it on the host instead of
    // rejecting it, and leave the guest's socket untouched.
    if is_addrconfig_probe(sock_type, &addr) {
        return addrconfig_probe(&addr);
    }
    policy.check(&addr)?;
    if sock_type == 2 {
        // UDP connect just records the default peer; it returns immediately
        // and needs no reactor round-trip.
        sock.connect(&addr.into())?;
        return Ok(json!({}));
    }
    let tsock = dup_into_tcp_socket(sock)?;
    // The returned TcpStream is dropped: closing the dup leaves the connection
    // open on the shared kernel socket (still referenced by the table's fd).
    io_timeout(tsock.connect(addr)).await?;
    Ok(json!({}))
}

fn handle_net_bind(
    table: &Mutex<SocketTable>,
    listen_ports: Option<&ListenPorts>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let addr = parse_sockaddr(args)?;
    match listen_ports {
        Some(ports) => ports.check(addr.port())?,
        None => return Err(anyhow!("Permission denied: no --port specified for bind")),
    }
    let sa: SockAddr = addr.into();
    let mut tbl = table.lock().unwrap();
    let hs = tbl.get_mut(fd)?;
    hs.socket.bind(&sa)?;
    // Record the actual bound address so this listener can be re-created
    // under the same fd on restore. Prefer the kernel-assigned local
    // address (handles ephemeral port 0) and fall back to the request.
    let local = hs
        .socket
        .local_addr()
        .ok()
        .and_then(|a| a.as_socket())
        .unwrap_or(addr);
    hs.bound_addr = Some(local);
    Ok(json!({}))
}

fn handle_net_listen(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let backlog = args["backlog"].as_i64().unwrap_or(128) as i32;
    let mut tbl = table.lock().unwrap();
    let hs = tbl.get_mut(fd)?;
    hs.socket.listen(backlog)?;
    hs.listen_backlog = Some(backlog);
    Ok(json!({}))
}

async fn handle_net_accept(
    table: Arc<Mutex<SocketTable>>,
    args: serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    // Dup the listener out and drop the table lock before the async accept.
    let (listener_sock, parent_type) = dup_socket(&table, fd)?;
    listener_sock.set_nonblocking(true)?;
    let std_listener: std::net::TcpListener = listener_sock.into();
    let listener = TcpListener::from_std(std_listener)?;
    // No artificial timeout: a server parked on accept waits indefinitely and
    // is woken by the reactor when a connection arrives. Cancellation happens
    // naturally if the driving future is dropped.
    let (stream, peer) = listener.accept().await?;
    // Hand the accepted connection to the table as a blocking socket2 socket so
    // the sync handlers (getpeername, setsockopt, …) keep working on it.
    let std_stream = stream.into_std()?;
    let new_sock = Socket::from(std_stream);
    new_sock.set_nonblocking(false)?;
    new_sock.set_read_timeout(Some(SOCKET_TIMEOUT))?;
    new_sock.set_write_timeout(Some(SOCKET_TIMEOUT))?;
    let new_fd = table
        .lock()
        .unwrap()
        .insert(HostSocket::new(new_sock, parent_type))?;
    Ok(json!({
        "fd": new_fd,
        "addr": peer.ip().to_string(),
        "port": peer.port(),
    }))
}

async fn handle_net_send(
    table: Arc<Mutex<SocketTable>>,
    args: serde_json::Value,
) -> Result<serde_json::Value> {
    use base64::Engine;
    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let data_b64 = args["data"]
        .as_str()
        .ok_or_else(|| anyhow!("missing 'data'"))?;
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| anyhow!("base64 decode: {}", e))?;
    if data.len() > MAX_NET_SEND {
        return Err(anyhow!(
            "net_send: payload too large ({} bytes, max {})",
            data.len(),
            MAX_NET_SEND
        ));
    }
    let (sock, sock_type) = dup_socket(&table, fd)?;
    let sent = match dup_into_tokio(sock, sock_type)? {
        TokioSock::Tcp(mut s) => io_timeout(s.write(&data)).await?,
        TokioSock::Udp(u) => io_timeout(u.send(&data)).await?,
    };
    Ok(json!({ "sent": sent }))
}

async fn handle_net_sendto(
    table: Arc<Mutex<SocketTable>>,
    policy: Arc<NetworkPolicy>,
    args: serde_json::Value,
) -> Result<serde_json::Value> {
    use base64::Engine;
    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let data_b64 = args["data"]
        .as_str()
        .ok_or_else(|| anyhow!("missing 'data'"))?;
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| anyhow!("base64 decode: {}", e))?;
    if data.len() > MAX_NET_SEND {
        return Err(anyhow!(
            "net_sendto: payload too large ({} bytes, max {})",
            data.len(),
            MAX_NET_SEND
        ));
    }
    let addr = parse_sockaddr(&args)?;
    policy.check(&addr)?;
    let (sock, sock_type) = dup_socket(&table, fd)?;
    // The guest uses `sendto` generically: for a connected stream socket the
    // destination is implicit (send to the peer), for a datagram socket it is
    // the supplied address.
    let sent = match dup_into_tokio(sock, sock_type)? {
        TokioSock::Tcp(mut s) => io_timeout(s.write(&data)).await?,
        TokioSock::Udp(u) => io_timeout(u.send_to(&data, addr)).await?,
    };
    Ok(json!({ "sent": sent }))
}

async fn handle_net_recv(
    table: Arc<Mutex<SocketTable>>,
    args: serde_json::Value,
) -> Result<serde_json::Value> {
    use base64::Engine;
    use serde_json::json;
    use tokio::io::AsyncReadExt;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let len = (args["len"].as_u64().unwrap_or(4096) as usize).min(65536);
    let (sock, sock_type) = dup_socket(&table, fd)?;
    let mut buf = vec![0u8; len];
    // No artificial timeout: a guest parked on recv waits indefinitely for
    // data and is woken by the reactor; the driving future can be dropped to
    // cancel.
    let n = match dup_into_tokio(sock, sock_type)? {
        TokioSock::Tcp(mut s) => s.read(&mut buf).await?,
        TokioSock::Udp(u) => u.recv(&mut buf).await?,
    };
    buf.truncate(n);
    let encoded = base64::engine::general_purpose::STANDARD.encode(&buf);
    Ok(json!({ "data": encoded, "len": n }))
}

async fn handle_net_recvfrom(
    table: Arc<Mutex<SocketTable>>,
    policy: Arc<NetworkPolicy>,
    args: serde_json::Value,
) -> Result<serde_json::Value> {
    use base64::Engine;
    use serde_json::json;
    use tokio::io::AsyncReadExt;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let len = (args["len"].as_u64().unwrap_or(4096) as usize).min(65536);
    let (sock, sock_type) = dup_socket(&table, fd)?;
    let mut buf = vec![0u8; len];
    // The guest uses `recvfrom` generically: for a connected stream socket the
    // source is the peer, for a datagram socket it comes from `recv_from`.
    // No artificial timeout (see handle_net_recv).
    let (n, peer) = match dup_into_tokio(sock, sock_type)? {
        TokioSock::Tcp(mut s) => {
            let peer = s.peer_addr()?;
            let n = s.read(&mut buf).await?;
            (n, peer)
        }
        TokioSock::Udp(u) => u.recv_from(&mut buf).await?,
    };
    buf.truncate(n);

    // Learn IPs from DNS responses so AllowList stays current with
    // anycast/CDN rotation (guest may resolve via a different DNS
    // server than the host, getting different IPs for the same name).
    if peer.port() == 53 {
        if let NetworkPolicy::AllowList(al) = &*policy {
            learn_ips_from_dns_response(&buf, al);
        }
    }

    let encoded = base64::engine::general_purpose::STANDARD.encode(&buf);
    Ok(json!({
        "data": encoded,
        "len": n,
        "addr": peer.ip().to_string(),
        "port": peer.port(),
    }))
}

fn handle_net_close(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    table.lock().unwrap().remove(fd)?;
    Ok(json!({}))
}

fn handle_net_shutdown(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let how = args["how"].as_i64().unwrap_or(2) as i32;
    let shutdown = match how {
        0 => std::net::Shutdown::Read,
        1 => std::net::Shutdown::Write,
        _ => std::net::Shutdown::Both,
    };
    let tbl = table.lock().unwrap();
    let sock = tbl.get_socket(fd)?;
    sock.shutdown(shutdown)?;
    Ok(json!({}))
}

fn handle_net_setsockopt(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let level = args["level"].as_i64().unwrap_or(0) as i32;
    let optname = args["optname"].as_i64().unwrap_or(0) as i32;
    let value = args["value"].as_i64().unwrap_or(0) as i32;
    let tbl = table.lock().unwrap();
    let sock = tbl.get_socket(fd)?;
    match (level, optname) {
        // SOL_SOCKET(1), SO_REUSEADDR(2)
        (1, 2) => sock.set_reuse_address(value != 0)?,
        // SOL_SOCKET(1), SO_KEEPALIVE(9)
        (1, 9) => sock.set_keepalive(value != 0)?,
        // IPPROTO_TCP(6), TCP_NODELAY(1)
        (6, 1) => sock.set_nodelay(value != 0)?,
        // Silently accepted — the dispatch round-trip makes
        // guest-side timeouts and error-reporting opts
        // counterproductive; the guest's own retry logic suffices.
        _ => {}
    }
    Ok(json!({}))
}

fn handle_net_getsockopt(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let level = args["level"].as_i64().unwrap_or(0) as i32;
    let optname = args["optname"].as_i64().unwrap_or(0) as i32;
    let tbl = table.lock().unwrap();
    let sock = tbl.get_socket(fd)?;
    let val: i32 = match (level, optname) {
        // SOL_SOCKET(1), SO_TYPE(3)
        (1, 3) => tbl.get_sock_type(fd)?,
        // SOL_SOCKET(1), SO_REUSEADDR(2)
        (1, 2) => sock.reuse_address()? as i32,
        // IPPROTO_TCP(6), TCP_NODELAY(1)
        (6, 1) => sock.nodelay()? as i32,
        _ => 0,
    };
    Ok(json!({ "value": val }))
}

fn handle_net_getpeername(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let tbl = table.lock().unwrap();
    let sock = tbl.get_socket(fd)?;
    let peer = sock.peer_addr()?;
    if let Some(addr) = peer.as_socket() {
        Ok(sockaddr_to_json(addr))
    } else {
        Ok(json!({ "addr": "0.0.0.0", "port": 0 }))
    }
}

fn handle_net_getsockname(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;

    let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
    let tbl = table.lock().unwrap();
    let sock = tbl.get_socket(fd)?;
    let local = sock.local_addr()?;
    if let Some(addr) = local.as_socket() {
        Ok(sockaddr_to_json(addr))
    } else {
        Ok(json!({ "addr": "0.0.0.0", "port": 0 }))
    }
}

/// Poll host sockets for readiness using the real OS `poll()` syscall.
///
/// Input:  `{"fds": [{"fd": N, "events": N}, ...], "timeout_ms": N}`
/// Output: `{"ready": [{"fd": N, "revents": N}, ...]}`
///
/// `events`/`revents` use standard POSIX poll flags (POLLIN=1, POLLOUT=4).
#[cfg(unix)]
fn handle_net_poll(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;
    use std::os::unix::io::AsRawFd;

    let fds_val = args
        .get("fds")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("net_poll: missing 'fds' array"))?;
    let timeout_ms = args
        .get("timeout_ms")
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
        .clamp(0, libc::c_int::MAX as i64) as libc::c_int;

    let tbl = table.lock().unwrap();
    let mut pollfds: Vec<libc::pollfd> = Vec::new();
    let mut guest_fds: Vec<u64> = Vec::new();
    let mut ready = Vec::new();

    for entry in fds_val {
        let fd = entry
            .get("fd")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("net_poll: entry missing 'fd'"))?;
        let raw_events = entry.get("events").and_then(|v| v.as_i64()).unwrap_or(0);
        if !(0..=i16::MAX as i64).contains(&raw_events) {
            return Err(anyhow!("net_poll: events {raw_events} out of i16 range"));
        }
        let events = raw_events as i16;
        if let Ok(sock) = tbl.get_socket(fd) {
            pollfds.push(libc::pollfd {
                fd: sock.as_raw_fd(),
                events,
                revents: 0,
            });
            guest_fds.push(fd);
        } else {
            ready.push(json!({ "fd": fd, "revents": libc::POLLNVAL as i64 }));
        }
    }
    drop(tbl);

    if pollfds.is_empty() {
        return Ok(json!({"ready": ready}));
    }

    let ret = unsafe {
        libc::poll(
            pollfds.as_mut_ptr(),
            pollfds.len() as libc::nfds_t,
            timeout_ms,
        )
    };

    if ret < 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() != Some(libc::EINTR) {
            return Err(anyhow!("net_poll: poll() failed: {err}"));
        }
    }

    for (i, pfd) in pollfds.iter().enumerate() {
        if pfd.revents != 0 {
            ready.push(json!({
                "fd": guest_fds[i],
                "revents": pfd.revents as i64,
            }));
        }
    }
    Ok(json!({"ready": ready}))
}

#[cfg(windows)]
fn handle_net_poll(
    table: &Mutex<SocketTable>,
    args: &serde_json::Value,
) -> Result<serde_json::Value> {
    use serde_json::json;
    use std::os::windows::io::AsRawSocket;
    use windows_sys::Win32::Networking::WinSock::{
        WSAPoll, POLLERR as W_POLLERR, POLLHUP as W_POLLHUP, POLLNVAL as W_POLLNVAL, POLLRDNORM,
        POLLWRNORM, WSAPOLLFD,
    };

    const POSIX_POLLIN: i16 = 0x0001;
    const POSIX_POLLOUT: i16 = 0x0004;
    const POSIX_POLLERR: i16 = 0x0008;
    const POSIX_POLLHUP: i16 = 0x0010;
    const POSIX_POLLNVAL: i16 = 0x0020;

    fn posix_to_win(posix: i16) -> i16 {
        let mut win: i16 = 0;
        if posix & POSIX_POLLIN != 0 {
            win |= POLLRDNORM;
        }
        if posix & POSIX_POLLOUT != 0 {
            win |= POLLWRNORM;
        }
        win
    }

    fn win_to_posix(win: i16) -> i16 {
        let mut posix: i16 = 0;
        if win & POLLRDNORM != 0 {
            posix |= POSIX_POLLIN;
        }
        if win & POLLWRNORM != 0 {
            posix |= POSIX_POLLOUT;
        }
        if win & W_POLLERR != 0 {
            posix |= POSIX_POLLERR;
        }
        if win & W_POLLHUP != 0 {
            posix |= POSIX_POLLHUP;
        }
        if win & W_POLLNVAL != 0 {
            posix |= POSIX_POLLNVAL;
        }
        posix
    }

    let fds_val = args
        .get("fds")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("net_poll: missing 'fds' array"))?;
    let timeout_ms = args
        .get("timeout_ms")
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
        .clamp(0, i32::MAX as i64) as i32;

    let tbl = table.lock().unwrap();
    let mut pollfds: Vec<WSAPOLLFD> = Vec::new();
    let mut guest_fds: Vec<u64> = Vec::new();
    let mut ready = Vec::new();

    for entry in fds_val {
        let fd = entry
            .get("fd")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("net_poll: entry missing 'fd'"))?;
        let raw_events = entry.get("events").and_then(|v| v.as_i64()).unwrap_or(0);
        if !(0..=i16::MAX as i64).contains(&raw_events) {
            return Err(anyhow!("net_poll: events {raw_events} out of i16 range"));
        }
        let events = posix_to_win(raw_events as i16);
        if let Ok(sock) = tbl.get_socket(fd) {
            pollfds.push(WSAPOLLFD {
                fd: sock.as_raw_socket() as usize,
                events,
                revents: 0,
            });
            guest_fds.push(fd);
        } else {
            ready.push(json!({ "fd": fd, "revents": POSIX_POLLNVAL as i64 }));
        }
    }
    drop(tbl);

    if pollfds.is_empty() {
        return Ok(json!({"ready": ready}));
    }

    let ret = unsafe { WSAPoll(pollfds.as_mut_ptr(), pollfds.len() as u32, timeout_ms) };

    if ret < 0 {
        let err = std::io::Error::last_os_error();
        return Err(anyhow!("net_poll: WSAPoll() failed: {err}"));
    }

    for (i, pfd) in pollfds.iter().enumerate() {
        if pfd.revents != 0 {
            ready.push(json!({
                "fd": guest_fds[i],
                "revents": win_to_posix(pfd.revents) as i64,
            }));
        }
    }
    Ok(json!({"ready": ready}))
}

/// `__hl_sleep` variant: poll all sockets in the table while sleeping.
/// Returns early if any socket becomes ready.
#[cfg(unix)]
fn hl_sleep_poll_sockets(
    _sc: &SleepCancel,
    table: &Mutex<SocketTable>,
    ns: u64,
) -> Result<serde_json::Value> {
    use serde_json::json;
    use std::os::unix::io::AsRawFd;

    let tbl = table.lock().unwrap();
    let mut pollfds: Vec<libc::pollfd> = tbl
        .sockets
        .values()
        .map(|hs| libc::pollfd {
            fd: hs.socket.as_raw_fd(),
            // Readability/errors only — see wait_readable_or_timeout: POLLOUT is
            // ~always ready on a connected socket and would spin the caller.
            events: libc::POLLIN | libc::POLLERR,
            revents: 0,
        })
        .collect();
    drop(tbl);

    if pollfds.is_empty() {
        _sc.wait(Duration::from_nanos(ns));
        return Ok(json!({}));
    }

    let timeout_ms = ((ns / 1_000_000) as libc::c_int).clamp(1, 30_000);
    let ret = unsafe {
        libc::poll(
            pollfds.as_mut_ptr(),
            pollfds.len() as libc::nfds_t,
            timeout_ms,
        )
    };

    if ret < 0 {
        let err = std::io::Error::last_os_error();
        if err.raw_os_error() != Some(libc::EINTR) {
            return Err(anyhow!("hl_sleep poll failed: {err}"));
        }
    }

    Ok(json!({"socket_ready": ret > 0}))
}

#[cfg(windows)]
fn hl_sleep_poll_sockets(
    _sc: &SleepCancel,
    table: &Mutex<SocketTable>,
    ns: u64,
) -> Result<serde_json::Value> {
    use serde_json::json;
    use std::os::windows::io::AsRawSocket;
    use windows_sys::Win32::Networking::WinSock::{WSAPoll, POLLRDNORM, WSAPOLLFD};

    let tbl = table.lock().unwrap();
    let mut pollfds: Vec<WSAPOLLFD> = tbl
        .sockets
        .values()
        .map(|hs| WSAPOLLFD {
            fd: hs.socket.as_raw_socket() as usize,
            // Readability only. POLLWRNORM is ~always ready on a connected
            // socket and would spin the caller; the guest only parks on
            // readable events (accept/recv). POLLERR is output-only on Windows.
            events: POLLRDNORM as i16,
            revents: 0,
        })
        .collect();
    drop(tbl);

    if pollfds.is_empty() {
        _sc.wait(Duration::from_nanos(ns));
        return Ok(json!({}));
    }

    let timeout_ms = ((ns / 1_000_000) as i32).clamp(1, 30_000);
    let ret = unsafe { WSAPoll(pollfds.as_mut_ptr(), pollfds.len() as u32, timeout_ms) };

    if ret < 0 {
        let err = std::io::Error::last_os_error();
        return Err(anyhow!("hl_sleep WSAPoll failed: {err}"));
    }

    Ok(json!({"socket_ready": ret > 0}))
}

// ---------------------------------------------------------------------------

fn register_net_tools(
    tools: &mut ToolRegistry,
    policy: &NetworkPolicy,
    listen_ports: Option<&ListenPorts>,
    table: Arc<Mutex<SocketTable>>,
) {
    let policy = Arc::new(policy.clone());

    let t = table.clone();
    tools.register("net_socket", move |args| handle_net_socket(&t, &args));

    let t = table.clone();
    let pol = policy.clone();
    tools.register_async_factory(
        "net_connect",
        Arc::new(move |args| {
            let (t, pol) = (t.clone(), pol.clone());
            Box::pin(handle_net_connect(t, pol, args))
        }),
    );

    let t = table.clone();
    let lp = listen_ports.cloned().map(Arc::new);
    tools.register("net_bind", move |args| {
        handle_net_bind(&t, lp.as_deref(), &args)
    });

    let t = table.clone();
    tools.register("net_listen", move |args| handle_net_listen(&t, &args));

    let t = table.clone();
    tools.register_async_factory(
        "net_accept",
        Arc::new(move |args| {
            let t = t.clone();
            Box::pin(handle_net_accept(t, args))
        }),
    );

    let t = table.clone();
    tools.register_async_factory(
        "net_send",
        Arc::new(move |args| {
            let t = t.clone();
            Box::pin(handle_net_send(t, args))
        }),
    );

    let t = table.clone();
    let pol = policy.clone();
    tools.register_async_factory(
        "net_sendto",
        Arc::new(move |args| {
            let (t, pol) = (t.clone(), pol.clone());
            Box::pin(handle_net_sendto(t, pol, args))
        }),
    );

    let t = table.clone();
    tools.register_async_factory(
        "net_recv",
        Arc::new(move |args| {
            let t = t.clone();
            Box::pin(handle_net_recv(t, args))
        }),
    );

    let t = table.clone();
    let pol = policy.clone();
    tools.register_async_factory(
        "net_recvfrom",
        Arc::new(move |args| {
            let (t, pol) = (t.clone(), pol.clone());
            Box::pin(handle_net_recvfrom(t, pol, args))
        }),
    );

    let t = table.clone();
    tools.register("net_close", move |args| handle_net_close(&t, &args));

    let t = table.clone();
    tools.register("net_shutdown", move |args| handle_net_shutdown(&t, &args));

    let t = table.clone();
    tools.register("net_setsockopt", move |args| {
        handle_net_setsockopt(&t, &args)
    });

    let t = table.clone();
    tools.register("net_getsockopt", move |args| {
        handle_net_getsockopt(&t, &args)
    });

    let t = table.clone();
    tools.register("net_getpeername", move |args| {
        handle_net_getpeername(&t, &args)
    });

    let t = table.clone();
    tools.register("net_getsockname", move |args| {
        handle_net_getsockname(&t, &args)
    });

    {
        let t = table.clone();
        tools.register("net_poll", move |args| handle_net_poll(&t, &args));
    }
}

/// Routes incoming fs_* tool calls to the matching `FsSandbox` by
/// matching the guest-supplied path against each preopen's guest path.
#[derive(Clone)]
struct FsRouter {
    entries: Vec<(String, FsSandbox, bool)>,
}

impl FsRouter {
    fn new(preopens: &[Preopen]) -> Result<Self> {
        let mut entries = Vec::with_capacity(preopens.len());
        for p in preopens {
            entries.push((
                p.guest_path.clone(),
                FsSandbox::new(&p.host_dir)?,
                p.read_only,
            ));
        }
        // Sort by descending prefix length so longer matches win (e.g.
        // /data/public should match before /data).
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        Ok(Self { entries })
    }

    /// Pick the preopen matching `path` and return its sandbox,
    /// the path-relative-to-that-sandbox, and whether it is read-only.
    fn route<'a>(&'a self, path: &'a str) -> Result<(&'a FsSandbox, &'a str, bool)> {
        for (prefix, fs, ro) in &self.entries {
            if path == prefix {
                return Ok((fs, "", *ro));
            }
            if let Some(tail) = path.strip_prefix(prefix).and_then(|t| t.strip_prefix('/')) {
                return Ok((fs, tail, *ro));
            }
        }
        Err(anyhow!(
            "path {:?} does not match any preopened mount",
            path
        ))
    }

    fn require_writable<'a>(&'a self, path: &'a str) -> Result<(&'a FsSandbox, &'a str)> {
        let (fs, rel, ro) = self.route(path)?;
        if ro {
            return Err(anyhow!("read-only mount: write to {} denied", path));
        }
        Ok((fs, rel))
    }

    fn register(self, registry: &mut ToolRegistry) {
        use serde_json::json;

        let r = self.clone();
        registry.register("fs_read", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_read: missing 'path'"))?;
            let (fs, rel, _ro) = r.route(path)?;
            let target = fs.resolve(rel)?;
            let size = std::fs::metadata(&target)
                .map_err(|e| anyhow!("fs_read {:?}: {}", path, e))?
                .len();
            if size > MAX_FS_READ {
                return Err(anyhow!(
                    "fs_read {:?}: file too large ({} bytes, max {})",
                    path,
                    size,
                    MAX_FS_READ
                ));
            }
            let text = std::fs::read_to_string(&target)
                .map_err(|e| anyhow!("fs_read {:?}: {}", path, e))?;
            Ok(json!({ "text": text }))
        });

        let r = self.clone();
        registry.register("fs_write", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_write: missing 'path'"))?;
            let text = args["text"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_write: missing 'text'"))?;
            if text.len() > MAX_FS_WRITE {
                return Err(anyhow!(
                    "fs_write: payload too large ({} bytes, max {})",
                    text.len(),
                    MAX_FS_WRITE
                ));
            }
            let append = args["append"].as_bool().unwrap_or(false);
            let (fs, rel) = r.require_writable(path)?;
            let target = fs.resolve(rel)?;
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(!append)
                .append(append)
                .open(&target)
                .map_err(|e| anyhow!("fs_write {:?}: {}", path, e))?;
            f.write_all(text.as_bytes())
                .map_err(|e| anyhow!("fs_write {:?}: {}", path, e))?;
            Ok(json!({ "bytes_written": text.len() }))
        });

        let r = self.clone();
        registry.register("fs_list", move |args| {
            let path = args["path"].as_str().unwrap_or("");
            let (fs, rel, _ro) = r.route(path)?;
            let target = fs.resolve(rel)?;
            let mut entries = Vec::new();
            for entry in
                std::fs::read_dir(&target).map_err(|e| anyhow!("fs_list {:?}: {}", path, e))?
            {
                if entries.len() >= MAX_DIR_ENTRIES {
                    return Err(anyhow!(
                        "fs_list {:?}: too many entries (max {})",
                        path,
                        MAX_DIR_ENTRIES
                    ));
                }
                let entry = entry?;
                let ft = entry.file_type()?;
                entries.push(json!({
                    "name": entry.file_name().to_string_lossy().into_owned(),
                    "is_dir": ft.is_dir(),
                    "is_file": ft.is_file(),
                    "is_symlink": ft.is_symlink(),
                }));
            }
            Ok(json!({ "entries": entries }))
        });

        let r = self.clone();
        registry.register("fs_stat", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_stat: missing 'path'"))?;
            let (fs, rel, _ro) = r.route(path)?;
            let target = fs.resolve(rel)?;
            let md =
                std::fs::metadata(&target).map_err(|e| anyhow!("fs_stat {:?}: {}", path, e))?;
            let mtime_ns = md
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            let atime_ns = md
                .accessed()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0);
            Ok(json!({
                "size": md.len(),
                "is_dir": md.is_dir(),
                "is_file": md.is_file(),
                "mtime_ns": mtime_ns,
                "atime_ns": atime_ns,
            }))
        });

        let r = self.clone();
        registry.register("fs_read_bytes", move |args| {
            use base64::Engine;
            use std::io::{Read, Seek, SeekFrom};
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_read_bytes: missing 'path'"))?;
            let offset = args["offset"].as_u64().unwrap_or(0);
            let want = args["len"].as_u64().unwrap_or(65536).min(MAX_FS_READ);
            let (fs, rel, _ro) = r.route(path)?;
            let target = fs.resolve(rel)?;
            let mut f = std::fs::File::open(&target)
                .map_err(|e| anyhow!("fs_read_bytes {:?}: {}", path, e))?;
            if offset > 0 {
                f.seek(SeekFrom::Start(offset))
                    .map_err(|e| anyhow!("fs_read_bytes seek {:?}: {}", path, e))?;
            }
            let mut buf = vec![0u8; want as usize];
            let n = f
                .read(&mut buf)
                .map_err(|e| anyhow!("fs_read_bytes {:?}: {}", path, e))?;
            buf.truncate(n);
            let eof = n < want as usize;
            Ok(json!({
                "data": base64::engine::general_purpose::STANDARD.encode(&buf),
                "eof": eof, "bytes_read": n,
            }))
        });

        let r = self.clone();
        registry.register("fs_write_bytes", move |args| {
            use base64::Engine;
            use std::io::{Seek, SeekFrom, Write};
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_write_bytes: missing 'path'"))?;
            let data_b64 = args["data"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_write_bytes: missing 'data'"))?;
            if data_b64.len() > MAX_FS_WRITE * 4 / 3 + 4 {
                return Err(anyhow!(
                    "fs_write_bytes: payload too large (max {} decoded bytes)",
                    MAX_FS_WRITE
                ));
            }
            let data = base64::engine::general_purpose::STANDARD
                .decode(data_b64)
                .map_err(|e| anyhow!("fs_write_bytes: bad base64: {}", e))?;
            if data.len() > MAX_FS_WRITE {
                return Err(anyhow!(
                    "fs_write_bytes: decoded payload too large ({} bytes, max {})",
                    data.len(),
                    MAX_FS_WRITE
                ));
            }
            let offset = args["offset"].as_u64();
            if let Some(off) = offset {
                if off > MAX_TRUNCATE_LEN {
                    return Err(anyhow!(
                        "fs_write_bytes: offset too large ({off}, max {MAX_TRUNCATE_LEN})"
                    ));
                }
            }
            let append = args["append"].as_bool().unwrap_or(false);
            let (fs, rel) = r.require_writable(path)?;
            let target = fs.resolve(rel)?;
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(offset.is_none() && !append)
                .append(append)
                .open(&target)
                .map_err(|e| anyhow!("fs_write_bytes {:?}: {}", path, e))?;
            if let Some(off) = offset {
                if !append {
                    f.seek(SeekFrom::Start(off))
                        .map_err(|e| anyhow!("fs_write_bytes seek {:?}: {}", path, e))?;
                }
            }
            f.write_all(&data)
                .map_err(|e| anyhow!("fs_write_bytes {:?}: {}", path, e))?;
            Ok(json!({ "bytes_written": data.len() }))
        });

        let r = self.clone();
        registry.register("fs_truncate", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_truncate: missing 'path'"))?;
            let length = args["length"]
                .as_u64()
                .ok_or_else(|| anyhow!("fs_truncate: missing 'length'"))?;
            if length > MAX_TRUNCATE_LEN {
                return Err(anyhow!(
                    "fs_truncate: length too large ({} bytes, max {})",
                    length,
                    MAX_TRUNCATE_LEN
                ));
            }
            let (fs, rel) = r.require_writable(path)?;
            let target = fs.resolve(rel)?;
            let f = std::fs::OpenOptions::new()
                .write(true)
                .open(&target)
                .map_err(|e| anyhow!("fs_truncate {:?}: {}", path, e))?;
            f.set_len(length)
                .map_err(|e| anyhow!("fs_truncate {:?}: {}", path, e))?;
            Ok(json!({}))
        });

        let r = self.clone();
        registry.register("fs_mkdir", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_mkdir: missing 'path'"))?;
            let parents = args["parents"].as_bool().unwrap_or(false);
            let (fs, rel) = r.require_writable(path)?;
            let target = fs.resolve(rel)?;
            if parents {
                std::fs::create_dir_all(&target)
            } else {
                std::fs::create_dir(&target)
            }
            .map_err(|e| anyhow!("fs_mkdir {:?}: {}", path, e))?;
            Ok(json!({}))
        });

        let r = self.clone();
        registry.register("fs_unlink", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_unlink: missing 'path'"))?;
            let (fs, rel) = r.require_writable(path)?;
            let target = fs.resolve(rel)?;
            if target == *fs.root() {
                return Err(anyhow!("fs_unlink: cannot remove mount root"));
            }
            let md =
                std::fs::metadata(&target).map_err(|e| anyhow!("fs_unlink {:?}: {}", path, e))?;
            if md.is_dir() {
                std::fs::remove_dir(&target)
            } else {
                std::fs::remove_file(&target)
            }
            .map_err(|e| anyhow!("fs_unlink {:?}: {}", path, e))?;
            Ok(json!({}))
        });
    }
}

// ---------------------------------------------------------------------------
// Sandbox — the primary API (via `Sandbox::builder()`)
// ---------------------------------------------------------------------------

/// A Unikraft sandbox backed by Hyperlight's `MultiUseSandbox`.
///
/// Construct one with [`Sandbox::builder`]. Lifecycle:
///   1. `.build()` — creates the VM and runs guest init, takes a snapshot
///   2. [`Sandbox::restore`] — rewinds the VM to the post-init snapshot
///   3. [`Sandbox::call_run`] — runs the guest application
pub struct Sandbox {
    inner: MultiUseSandbox,
    /// Post-init snapshot for fast restore between calls.
    snapshot: Option<Arc<Snapshot>>,
    /// Initrd path — re-mapped after every restore() since restore
    /// overwrites the region with the snapshot's original memory.
    initrd_path: Option<std::path::PathBuf>,
    exit_code: Arc<AtomicI32>,
    /// Typed signal reported by `__hl_exit` or `__hl_poll_yield` during the
    /// current cooperative guest step.
    poll_signal: SharedPollSignal,
    /// Absolute deadline derived from `poll_signal` at the end of each
    /// [`Sandbox::poll`]: `Some(instant)` when the guest reported a pending
    /// timer, `None` for an indefinite park (no timer) or exit. Consumed by
    /// [`Sandbox::drive_host_functions`] as the inter-step wait bound and
    /// surfaced to callers via [`Sandbox::next_wakeup`]. Storing it as an
    /// *absolute* instant means any time spent between the two calls is
    /// subtracted, so the guest's timer still fires on schedule.
    next_wakeup_at: Option<Instant>,
    /// Shared socket table — cleared on [`Sandbox::restore`] so that
    /// host-side fds don't leak across guest restore cycles.
    socket_table: Option<Arc<Mutex<SocketTable>>>,
    /// Cancellation token for in-progress `__hl_sleep` host calls.
    sleep_cancel: SleepCancel,
    /// Off-vCPU driver state for async tools (see
    /// [`SandboxBuilder::tool_async`] / [`Sandbox::drive_host_functions`]).
    /// `None` unless async tools were registered.
    async_state: Option<AsyncToolState>,
}

/// Where the initrd comes from — either a file (zero-copy `map_file_cow`)
/// or an in-memory buffer (copied into snapshot memory).
enum InitrdSource {
    File(std::path::PathBuf),
    Bytes(Vec<u8>),
}

/// Fluent builder for [`Sandbox`]. Returned by [`Sandbox::builder`].
///
/// ```no_run
/// use hyperlight_unikraft::{Preopen, Sandbox};
///
/// let sandbox = Sandbox::builder("kernel.bin")
///     .initrd_file("app.cpio")
///     .args(["arg1", "arg2"])
///     .heap_size(16 << 20)
///     .preopen(Preopen::new("./work", "/data")?)
///     .tool("echo", |args| Ok(args))
///     .build()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct SandboxBuilder {
    kernel: std::path::PathBuf,
    initrd: Option<InitrdSource>,
    args: Vec<String>,
    heap_size: Option<u64>,
    stack_size: Option<u64>,
    io_buffer_size: Option<usize>,
    preopens: Vec<Preopen>,
    network: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    tools: ToolRegistry,
    has_tools: bool,
    /// Async tool factories registered via [`SandboxBuilder::tool_async`].
    async_tools: HashMap<String, ToolFactory>,
}

impl SandboxBuilder {
    /// The initrd CPIO archive, mapped zero-copy into guest memory.
    pub fn initrd_file<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.initrd = Some(InitrdSource::File(path.into()));
        self
    }

    /// An in-memory initrd buffer. Copied into snapshot memory.
    /// Prefer [`initrd_file`](Self::initrd_file) for anything non-trivial.
    pub fn initrd_bytes(mut self, bytes: Vec<u8>) -> Self {
        self.initrd = Some(InitrdSource::Bytes(bytes));
        self
    }

    /// Application arguments, passed to the guest via the cmdline header.
    pub fn args<S, I>(mut self, args: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    /// Append a single argument. Repeatable.
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Guest heap size in bytes (default 512 MiB).
    pub fn heap_size(mut self, bytes: u64) -> Self {
        self.heap_size = Some(bytes);
        self
    }

    /// Guest stack size in bytes (default 8 MiB).
    pub fn stack_size(mut self, bytes: u64) -> Self {
        self.stack_size = Some(bytes);
        self
    }

    /// Shared-memory I/O buffer size for host function calls (default 128 KiB).
    /// Must be large enough to hold a single base64-encoded hostfs write chunk
    /// plus JSON and FlatBuffer framing (~44 KiB for the default 32 KiB chunk).
    pub fn io_buffer_size(mut self, bytes: usize) -> Self {
        self.io_buffer_size = Some(bytes);
        self
    }

    /// Expose a host directory to the guest. `lib/hostfs` mounts each
    /// `preopen.host_dir` at `preopen.guest_path`; FS tool handlers
    /// cover all of them and route by guest path prefix. Repeatable —
    /// call multiple times to expose several directories.
    pub fn preopen(mut self, preopen: Preopen) -> Self {
        self.preopens.push(preopen);
        self
    }

    /// Enable guest networking with the given policy.
    ///
    /// Without this call, no `net_*` tools are registered and the guest
    /// has no network access.
    pub fn network(mut self, policy: NetworkPolicy) -> Self {
        self.network = Some(policy);
        self
    }

    /// Allow the guest to bind to the given ports for inbound connections.
    ///
    /// Requires [`network`](Self::network) to also be set — without a
    /// network policy the net tools are not registered at all. When net
    /// tools *are* registered but no `listen_ports` is set, `net_bind`
    /// rejects every call (outbound-only mode).
    pub fn listen_ports(mut self, ports: ListenPorts) -> Self {
        self.listen_ports = Some(ports);
        self
    }

    /// Register a host function callable from the guest via `__dispatch`.
    pub fn tool<F>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Result<serde_json::Value> + Send + Sync + 'static,
    {
        self.tools.register(name, handler);
        self.has_tools = true;
        self
    }

    /// Register an **async** host function callable from the guest.
    ///
    /// Unlike [`tool`](Self::tool), the handler returns a future. When the
    /// guest calls the tool, the host immediately answers with a yield
    /// completion token (the guest's calling thread cooperatively parks) and
    /// the future is driven off the vCPU thread by
    /// [`Sandbox::drive_host_functions`]. Once it resolves, its result is
    /// delivered to the guest in the next [`Sandbox::poll`] batch, and the
    /// original guest call returns — all transparently to guest code.
    ///
    /// This enables the target driving loop:
    /// ```no_run
    /// # use hyperlight_unikraft::Sandbox;
    /// # use core::task::Poll;
    /// # async fn run(mut sandbox: Sandbox) -> anyhow::Result<()> {
    /// loop {
    ///     match sandbox.poll()? {
    ///         Poll::Ready(()) => break,
    ///         Poll::Pending => sandbox.drive_host_functions().await,
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// The handler must be `Send + Sync + 'static` and its future `Send`, so it
    /// can be driven on a multi-threaded Tokio runtime.
    pub fn tool_async<F, Fut>(mut self, name: &str, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<serde_json::Value>> + Send + 'static,
    {
        self.async_tools.insert(
            name.to_string(),
            Arc::new(move |args| Box::pin(handler(args))),
        );
        self.has_tools = true;
        self
    }

    /// Boot the VM, run init, and take a post-init snapshot.
    pub fn build(mut self) -> Result<Sandbox> {
        let config = VmConfig {
            heap_size: self.heap_size.unwrap_or(512 * 1024 * 1024),
            stack_size: self.stack_size.unwrap_or(8 * 1024 * 1024),
            io_buffer_size: self.io_buffer_size.unwrap_or(DEFAULT_IO_BUFFER_SIZE),
        };

        // Fold any user async tools (from `tool_async`) into the registry's
        // async side. The internal tools (`register_internal_tools`, called
        // inside evolve) add the blocking networking factories to the same
        // async side, and evolve builds the shared driver state
        // (`AsyncToolState`) from it — so a single `drive_host_functions`
        // drives both user and internal async tools.
        if !self.async_tools.is_empty() {
            let asy = self.tools.async_side_mut();
            for (name, factory) in std::mem::take(&mut self.async_tools) {
                asy.factories.insert(name, factory);
            }
            self.has_tools = true;
        }

        let tools = if self.has_tools {
            Some(self.tools)
        } else {
            None
        };
        let net = self.network.as_ref();
        let lp = self.listen_ports.as_ref();
        let sandbox = match self.initrd {
            Some(InitrdSource::File(path)) => Sandbox::evolve_mapped(
                &self.kernel,
                Some(&path),
                &self.args,
                config,
                tools,
                &self.preopens,
                net,
                lp,
            ),
            Some(InitrdSource::Bytes(bytes)) => Sandbox::evolve_inline(
                &self.kernel,
                Some(&bytes),
                &self.args,
                config,
                tools,
                &self.preopens,
                net,
                lp,
            ),
            None => Sandbox::evolve_mapped(
                &self.kernel,
                None,
                &self.args,
                config,
                tools,
                &self.preopens,
                net,
                lp,
            ),
        }?;
        Ok(sandbox)
    }
}

impl Sandbox {
    /// Start building a sandbox. See [`SandboxBuilder`] for the chainable
    /// configuration methods.
    pub fn builder<P: Into<std::path::PathBuf>>(kernel: P) -> SandboxBuilder {
        SandboxBuilder {
            kernel: kernel.into(),
            initrd: None,
            args: Vec::new(),
            heap_size: None,
            stack_size: None,
            io_buffer_size: None,
            preopens: Vec::new(),
            network: None,
            listen_ports: None,
            tools: ToolRegistry::new(),
            has_tools: false,
            async_tools: HashMap::new(),
        }
    }

    /// Low-level: boot with an in-memory initrd buffer. Prefer the builder.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evolve_inline(
        kernel_path: &Path,
        initrd: Option<&[u8]>,
        app_args: &[String],
        config: VmConfig,
        tools: Option<ToolRegistry>,
        preopens: &[Preopen],
        network: Option<&NetworkPolicy>,
        listen_ports: Option<&ListenPorts>,
    ) -> Result<Self> {
        if !kernel_path.exists() {
            return Err(anyhow!("Kernel not found: {:?}", kernel_path));
        }

        let extended_initrd = prepend_cmdline_to_initrd(initrd, app_args, preopens);
        let env = GuestEnvironment::new(
            GuestBinary::FilePath(kernel_path.to_string_lossy().to_string()),
            extended_initrd.as_deref(),
        );

        let mut usbox = UninitializedSandbox::new(env, Some(config.sandbox_config()))?;

        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_signal = Arc::new(Mutex::new(GuestPollSignal::None));
        let sleep_cancel = SleepCancel::new();
        let mut tools = build_tools(tools, preopens)?.unwrap_or_default();
        let socket_table = register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_signal,
            &sleep_cancel,
            network,
            listen_ports,
        );
        let async_state = tools.make_async_state();
        let tools = Arc::new(tools);
        let tools_ref = tools.clone();
        usbox.register_host_function("__dispatch", move |payload: Vec<u8>| -> Vec<u8> {
            tools_ref.dispatch(&payload)
        })?;

        Self::finish_evolve(
            usbox,
            None,
            exit_code,
            poll_signal,
            sleep_cancel,
            socket_table,
            async_state,
        )
    }

    /// Low-level: boot with a zero-copy mapped initrd file. Prefer the builder.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evolve_mapped(
        kernel_path: &Path,
        initrd_path: Option<&Path>,
        app_args: &[String],
        config: VmConfig,
        tools: Option<ToolRegistry>,
        preopens: &[Preopen],
        network: Option<&NetworkPolicy>,
        listen_ports: Option<&ListenPorts>,
    ) -> Result<Self> {
        if !kernel_path.exists() {
            return Err(anyhow!("Kernel not found: {:?}", kernel_path));
        }

        // Get file size before creating sandbox
        let mapped_size = match initrd_path {
            Some(path) if path.exists() => std::fs::metadata(path)?.len(),
            Some(path) => return Err(anyhow!("Initrd not found: {:?}", path)),
            None => 0,
        };

        // Build init_data with cmdline + preopens + mapped file size
        let cmdline_data = build_cmdline_initdata(app_args, mapped_size, preopens);
        let env = GuestEnvironment::new(
            GuestBinary::FilePath(kernel_path.to_string_lossy().to_string()),
            cmdline_data.as_deref(),
        );

        let mut usbox = UninitializedSandbox::new(env, Some(config.sandbox_config()))?;

        // Map the initrd file (zero-copy via mmap)
        // Place past the x86 LAPIC MMIO page (0xFEE0_0000) so that
        // large initrds (>1 GiB) don't overlap it and trigger EEXIST
        // from KVM_SET_USER_MEMORY_REGION on kernels where in-kernel
        // IRQCHIP reserves that range.
        const INITRD_MAP_BASE: u64 = 0xFEF0_0000;
        let initrd_owned = if let Some(path) = initrd_path {
            usbox.map_file_cow(path, INITRD_MAP_BASE)?;
            Some(path.to_path_buf())
        } else {
            None
        };

        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_signal = Arc::new(Mutex::new(GuestPollSignal::None));
        let sleep_cancel = SleepCancel::new();
        let mut tools = build_tools(tools, preopens)?.unwrap_or_default();
        let socket_table = register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_signal,
            &sleep_cancel,
            network,
            listen_ports,
        );
        let async_state = tools.make_async_state();
        let tools = Arc::new(tools);
        let tools_ref = tools.clone();
        usbox.register_host_function("__dispatch", move |payload: Vec<u8>| -> Vec<u8> {
            tools_ref.dispatch(&payload)
        })?;

        Self::finish_evolve(
            usbox,
            initrd_owned,
            exit_code,
            poll_signal,
            sleep_cancel,
            socket_table,
            async_state,
        )
    }

    fn finish_evolve(
        usbox: UninitializedSandbox,
        initrd_path: Option<std::path::PathBuf>,
        exit_code: Arc<AtomicI32>,
        poll_signal: SharedPollSignal,
        sleep_cancel: SleepCancel,
        socket_table: Option<Arc<Mutex<SocketTable>>>,
        async_state: Option<AsyncToolState>,
    ) -> Result<Self> {
        let mut inner = usbox.evolve()?;
        let snapshot = inner.snapshot().ok();
        Ok(Self {
            inner,
            snapshot,
            initrd_path,
            exit_code,
            poll_signal,
            next_wakeup_at: None,
            socket_table,
            sleep_cancel,
            async_state,
        })
    }

    /// Restore the sandbox to its post-init snapshot.
    ///
    /// This is a fast operation (host-level CoW via mmap) that resets all
    /// guest memory to the state captured after init.
    pub fn restore(&mut self) -> Result<()> {
        if let Some(ref snap) = self.snapshot {
            self.inner.restore(snap.clone())?;
        }
        const INITRD_MAP_BASE: u64 = 0xFEF0_0000;
        if let Some(ref path) = self.initrd_path {
            self.inner.map_file_cow(path, INITRD_MAP_BASE)?;
        }
        if let Some(ref table) = self.socket_table {
            table.lock().unwrap().clear();
        }
        *self.poll_signal.lock().unwrap() = GuestPollSignal::None;
        self.next_wakeup_at = None;
        Ok(())
    }

    /// Call the dispatch function to re-run the application.
    ///
    /// Requires a prior `restore()` to reset guest state.
    /// The dispatch function pops the FunctionCall from input,
    /// runs the application, pushes a void result, and halts.
    pub fn call_run(&mut self) -> Result<()> {
        // call() with Void return type — the function name doesn't matter
        // to the guest (it ignores it and just runs the app).
        *self.poll_signal.lock().unwrap() = GuestPollSignal::None;
        let _: () = self.inner.call("run", ())?;
        self.check_guest_ran_to_completion("call_run")
    }

    /// Error if the guest parked instead of finishing.
    ///
    /// A guest built with `CONFIG_HYPERLIGHT_POLL` returns to the host
    /// whenever it would block, expecting the caller to re-enter via
    /// [`Sandbox::poll`]. The blocking entry points invoke the guest exactly
    /// once, so such a yield means the work was abandoned part-way: the guest
    /// is still parked and no error is reported anywhere. Without this check
    /// the call silently succeeds having done only part of the work — e.g. a
    /// script stops at its first socket read with no diagnostic at all.
    fn check_guest_ran_to_completion(&self, entry: &str) -> Result<()> {
        if let GuestPollSignal::Yielded { .. } = *self.poll_signal.lock().unwrap() {
            return Err(anyhow!(
                "{entry}: the guest yielded instead of running to completion — \
                 it is built with CONFIG_HYPERLIGHT_POLL and must be driven by \
                 the cooperative poll pump (Sandbox::poll_run_async, or the \
                 --poll flag), not by a single blocking call"
            ));
        }
        Ok(())
    }

    /// Drain resolved async-tool results into one bounded binary batch.
    ///
    /// Each entry is `{request_id: u64, payload_len: u32, JSON payload}`. An
    /// entry is removed from the pending map only after it fits in this batch,
    /// so results beyond the 64 KiB guest transport limit remain queued for the
    /// next poll instead of being truncated.
    fn drain_completion_batch(&mut self) -> Vec<u8> {
        /// Per-entry `{request_id: u64, payload_len: u32}` prefix.
        const ENTRY_HEADER_LEN: usize = 12;
        let budget = ASYNC_FRAME_MAX_LEN - ASYNC_FRAME_HEADER_LEN;

        let mut entries = Vec::new();
        let mut count = 0u64;
        if let Some(st) = self.async_state.as_ref() {
            st.pending.lock().unwrap().retain(|id, task| {
                let PendingState::Ready(res) = &task.state else {
                    return true;
                };
                let value = match res {
                    Ok(v) => serde_json::json!({ "result": v }),
                    Err(e) => serde_json::json!({ "error": e }),
                };
                let mut payload = serde_json::to_vec(&value).unwrap_or_else(|_| {
                    b"{\"error\":\"completion serialization failed\"}".to_vec()
                });
                // A result that can never fit becomes an error rather than
                // blocking the entry forever.
                if ENTRY_HEADER_LEN + payload.len() > budget {
                    payload = b"{\"error\":\"async completion exceeds transport limit\"}".to_vec();
                }
                // Out of room in this batch — keep it for the next poll.
                if entries.len() + ENTRY_HEADER_LEN + payload.len() > budget {
                    return true;
                }
                entries.extend_from_slice(&id.to_le_bytes());
                entries.extend_from_slice(&(payload.len() as u32).to_le_bytes());
                entries.extend_from_slice(&payload);
                count += 1;
                false
            });
        }
        encode_async_frame(ASYNC_FRAME_BATCH, count, &entries)
    }

    /// Run one cooperative guest step and return its typed reason for yielding.
    ///
    /// This method owns vCPU progress: it delivers completed host-call results,
    /// runs the guest scheduler until it exits or yields, and records any timer
    /// deadline for [`Sandbox::drive_host_functions`]. When it returns
    /// [`PollOutcome::HostCallsPending`], the caller should invoke
    /// `drive_host_functions` before polling the guest again.
    ///
    /// A direct VM halt without `__hl_poll_yield` is classified as
    /// [`PollOutcome::Exited`]. This covers normal C process completion via
    /// `uk_pm_shutdown(SYSHALT)` as well as explicit `__hl_exit`.
    ///
    /// Requires a prior [`Sandbox::restore`] before the first step of a fresh
    /// run.
    pub fn poll_outcome(&mut self) -> Result<PollOutcome> {
        *self.poll_signal.lock().unwrap() = GuestPollSignal::None;
        // Pass the binary batch of completed/errored async host tasks as the
        // guest `poll` function's byte-vector argument. The guest routes each ID's
        // result to the matching parked host call (see
        // plat/hyperlight/poll.c `hyperlight_poll_deliver_arg` and
        // hcall.c `hyperlight_hcall_deliver_batch`), so a parked call resumes
        // without ever issuing a follow-up host function.
        let batch = self.drain_completion_batch();
        let _: () = self.inner.call("poll", batch)?;
        let signal = *self.poll_signal.lock().unwrap();
        self.next_wakeup_at = None;

        let outcome = classify_poll_signal(signal, self.has_pending_host_calls());
        let deadline = match outcome {
            PollOutcome::Timer(after) => Some(after),
            PollOutcome::HostCallsPending { next_wakeup } => next_wakeup,
            PollOutcome::Exited | PollOutcome::Idle => None,
        };
        if let Some(after) = deadline {
            // Keep an absolute deadline for drive_host_functions(), so time
            // spent by the caller between the two operations is subtracted.
            self.next_wakeup_at = Some(Instant::now() + after);
        }
        Ok(outcome)
    }

    fn has_pending_host_calls(&self) -> bool {
        // Every queued and in-flight future has a `pending` entry recorded
        // before it is queued, so this one map covers all three stages.
        self.async_state
            .as_ref()
            .is_some_and(|state| !state.pending.lock().unwrap().is_empty())
    }

    /// Compatibility wrapper over [`Sandbox::poll_outcome`].
    ///
    /// Maps [`PollOutcome::Exited`] to [`core::task::Poll::Ready`] and every
    /// yielded outcome to [`core::task::Poll::Pending`]. Use `poll_outcome`
    /// when the caller needs to distinguish idle, timer, and host-call waits.
    pub fn poll(&mut self) -> Result<core::task::Poll<()>> {
        Ok(match self.poll_outcome()? {
            PollOutcome::Exited => core::task::Poll::Ready(()),
            PollOutcome::Idle | PollOutcome::Timer(_) | PollOutcome::HostCallsPending { .. } => {
                core::task::Poll::Pending
            }
        })
    }

    /// Time remaining until the guest's next scheduled wakeup, as captured by
    /// the most recent [`Sandbox::poll`].
    ///
    /// - `Some(d)` — the guest is parked on a timer; re-poll within `d` (or
    ///   sooner if external input arrives, e.g. via [`SleepCancel::cancel`]).
    ///   [`Duration::ZERO`] means the deadline has already elapsed.
    /// - `None` — after a [`Poll::Pending`](core::task::Poll::Pending) poll,
    ///   the guest went idle with **no** pending timer (re-poll when external
    ///   input becomes available); after a
    ///   [`Poll::Ready`](core::task::Poll::Ready) poll, the guest has exited.
    pub fn next_wakeup(&self) -> Option<Duration> {
        self.next_wakeup_at
            .map(|at| at.saturating_duration_since(Instant::now()))
    }

    /// Drive host-side async work between [`Sandbox::poll`] steps: registered
    /// async tools (see [`SandboxBuilder::tool_async`] and the blocking
    /// networking tools) *and* the inter-step wait for host-socket readiness
    /// and the guest's next-wakeup timer — all off the vCPU thread.
    ///
    /// Call this whenever [`Sandbox::poll`] returns
    /// [`Poll::Pending`](core::task::Poll::Pending); the canonical loop is:
    ///
    /// ```no_run
    /// # async fn f(mut sbox: hyperlight_unikraft::Sandbox) -> anyhow::Result<()> {
    /// use core::task::Poll;
    /// loop {
    ///     match sbox.poll()? {
    ///         Poll::Ready(()) => break,
    ///         Poll::Pending => sbox.drive_host_functions().await,
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// Behaviour:
    ///   1. If any async-tool futures are in flight, absorb newly-submitted
    ///      ones into a `FuturesUnordered` and `await` one completion (bounded
    ///      by the guest's timer deadline), recording the result so the next
    ///      [`Sandbox::poll`] batch delivers it and the parked guest call
    ///      resumes.
    ///   2. Otherwise, `await` until any host socket becomes readable or the
    ///      guest's timer deadline elapses (the folded-in async inter-step
    ///      wait), so a guest parked on `accept`/`recv` is re-driven promptly.
    ///   3. If there is nothing to wait on (indefinite idle, no sockets, no
    ///      in-flight work), fall back to a bounded sleep so the loop never
    ///      busy-spins.
    ///
    /// Note on sleeping: in the cooperative poll model the guest never issues a
    /// blocking `__hl_sleep`; it parks on the scheduler and reports its next
    /// deadline, which step 1/2 above `await` via [`tokio::time::sleep`]. So a
    /// guest sleep is serviced asynchronously here without ever blocking a host
    /// thread.
    pub async fn drive_host_functions(&mut self) {
        // Time remaining until the guest's next-wakeup deadline captured by the
        // last poll (`None` = indefinite park / no pending timer). Derived from
        // the absolute instant, so any time already spent since the poll is
        // subtracted and the guest's timer still fires on schedule.
        let remaining = self.next_wakeup();

        // Step 1: drive in-flight async tool futures, if any.
        if let Some(st) = self.async_state.as_mut() {
            // Absorb futures queued by dispatch since the last drive.
            loop {
                let next = st.queue.lock().unwrap().pop_front();
                match next {
                    Some((token, fut)) => {
                        st.running.push(Box::pin(async move { (token, fut.await) }));
                    }
                    None => break,
                }
            }

            if !st.running.is_empty() {
                // Await one completion, bounded by the guest's timer deadline
                // (if any) so a pending guest timer still fires on time.
                let completed = match remaining {
                    None => st.running.next().await,
                    Some(dur) => tokio::select! {
                        out = st.running.next() => out,
                        _ = tokio::time::sleep(dur) => None,
                    },
                };
                if let Some((token, res)) = completed {
                    let mut pending = st.pending.lock().unwrap();
                    let state = PendingState::Ready(res.map_err(|e| e.to_string()));
                    match pending.get_mut(&token) {
                        // Normal path: flip the tracked task to Ready, keeping
                        // its recorded name/args for a possible snapshot.
                        Some(task) => task.state = state,
                        // No tracked entry (shouldn't happen) — record it so the
                        // result is still delivered on the next poll batch.
                        None => {
                            pending.insert(
                                token,
                                PendingTask {
                                    name: String::new(),
                                    args: serde_json::Value::Null,
                                    state,
                                },
                            );
                        }
                    }
                }
                return;
            }
        }

        // Step 2: no async futures in flight — wait for host-socket readiness
        // and/or the guest's timer deadline (the async inter-step wait).
        let (fds, filtered_eof) = self.socket_wait_fds();
        if !fds.is_empty() || remaining.is_some() || filtered_eof {
            // If a terminally-EOF socket was filtered out we may be wrong about
            // the guest having lost interest in it (its interest set is guest-side
            // state the host can't see), so cap the wait and re-enter the guest
            // promptly. Bounds the cost of a wrong guess at EOF_RECHECK instead
            // of a 30 s stall, while still replacing a full-rate spin with a park.
            let mut dur = remaining.unwrap_or(SOCKET_TIMEOUT);
            if filtered_eof {
                dur = dur.min(EOF_RECHECK);
            }
            Self::wait_readable_or_timeout(fds, dur).await;
        } else {
            // Step 3: indefinite idle with no wake source — bounded fallback so
            // the poll loop can't busy-spin.
            tokio::time::sleep(SOCKET_TIMEOUT).await;
        }
    }

    /// Drive the guest to completion cooperatively from a Tokio runtime,
    /// returning its exit code — the async, poll-driven analog of
    /// [`Sandbox::call_run`].
    ///
    /// (Named `poll_run_async` rather than `call_run_async` because the latter
    /// already exists for a different purpose — a pausable/snapshotable run
    /// handle. This method instead runs the cooperative *poll* loop.)
    ///
    /// This is the full poll loop as a single `async fn`, built on the unified
    /// [`Sandbox::poll`] / [`Sandbox::drive_host_functions`] pair (the same
    /// shape as the [`SandboxBuilder::tool_async`] example):
    ///
    /// - **CPU step** — [`Sandbox::poll`] runs the vCPU via a blocking
    ///   `KVM_RUN`. It is called inline here, so it briefly occupies the
    ///   current worker thread for the (bounded) duration of one scheduler
    ///   pump. On a multi-threaded runtime other tasks continue on other
    ///   workers. [`Poll::Ready(())`](core::task::Poll::Ready) ends the loop.
    /// - **Drive step** — on [`Poll::Pending`](core::task::Poll::Pending),
    ///   [`Sandbox::drive_host_functions`] makes host-side progress: it drives
    ///   any in-flight async-tool futures (e.g. `net_connect`/`send`/`sendto`,
    ///   `sleep`) off the vCPU thread, and otherwise `await`s host-socket
    ///   readiness and/or the guest's timer deadline on the reactor — so the
    ///   task yields instead of busy-looping while the guest is parked on
    ///   `accept`/`recv` or awaiting an async host function.
    ///
    /// Requires a prior [`Sandbox::restore`] to reset guest state before the
    /// first step of a fresh run (same contract as [`Sandbox::poll`]). The exit
    /// code is the value the guest reported via `__hl_exit` (0 if it halted
    /// without an explicit code); read it again later with
    /// [`Sandbox::last_exit_code`].
    ///
    /// Cancellation composes: drop the returned future (e.g. via
    /// [`tokio::time::timeout`] or a `select!`) to stop driving the guest
    /// between steps. A step already in progress runs to its next yield first,
    /// since [`Sandbox::poll`] is synchronous.
    pub async fn poll_run_async(&mut self) -> Result<i32> {
        loop {
            match self.poll()? {
                core::task::Poll::Ready(()) => return Ok(self.last_exit_code()),
                core::task::Poll::Pending => self.drive_host_functions().await,
            }
        }
    }

    /// True if `fd` is a stream socket in a terminal EOF state: the peer has
    /// closed and no unread data is queued.
    ///
    /// Such a socket is permanently readable at the epoll level. Leaving it in
    /// the inter-step readiness set makes [`Sandbox::wait_readable_or_timeout`]
    /// return instantly forever, so the poll loop busy-spins at full
    /// vmexit rate instead of parking. Nothing can ever change on it again —
    /// there is no edge left to wait for.
    ///
    /// `MSG_PEEK` keeps the probe non-destructive: `0` means EOF, `1` means
    /// unread data is queued (and stays queued), and `EAGAIN` means the
    /// connection is open but idle. Restricted to stream sockets because a
    /// zero-length UDP datagram would otherwise be misread as EOF; listeners
    /// return an error rather than `0`, so they are never filtered.
    #[cfg(unix)]
    fn is_drained_eof(fd: i32, sock_type: i32) -> bool {
        // SOCK_STREAM = 1 (matches the guest-side value stored in the table).
        if sock_type != 1 {
            return false;
        }
        let mut byte = 0u8;
        let n = unsafe {
            libc::recv(
                fd,
                &mut byte as *mut u8 as *mut libc::c_void,
                1,
                libc::MSG_PEEK | libc::MSG_DONTWAIT,
            )
        };
        n == 0
    }

    /// Fds to watch for the inter-step readiness wait, and whether any socket
    /// was filtered out as terminally EOF (see [`Sandbox::is_drained_eof`]).
    #[cfg(unix)]
    fn socket_wait_fds(&self) -> (Vec<i32>, bool) {
        use std::os::unix::io::AsRawFd;
        let Some(table) = &self.socket_table else {
            return (Vec::new(), false);
        };
        let tbl = table.lock().unwrap();
        let mut fds = Vec::with_capacity(tbl.sockets.len());
        let mut filtered = false;
        for hs in tbl.sockets.values() {
            let fd = hs.socket.as_raw_fd();
            if Self::is_drained_eof(fd, hs.sock_type) {
                filtered = true;
            } else {
                fds.push(fd);
            }
        }
        (fds, filtered)
    }

    #[cfg(not(unix))]
    fn socket_wait_fds(&self) -> (Vec<i32>, bool) {
        (Vec::new(), false)
    }

    /// Wait until any of `fds` becomes readable, or `timeout` elapses.
    ///
    /// Each fd is *borrowed*: it is wrapped in an [`AsyncFd`] purely to observe
    /// read readiness via the Tokio reactor. `AsyncFd<RawFd>` never performs
    /// I/O and never closes the fd (dropping an `i32` is a no-op), so the
    /// sockets may stay blocking. A fresh registration is built per call so
    /// the changing socket set is always reflected, and already-readable fds
    /// are reported immediately (epoll delivers current readiness at
    /// registration). If registration fails we fall back to the timer so the
    /// caller can't wedge.
    #[cfg(unix)]
    async fn wait_readable_or_timeout(fds: Vec<i32>, timeout: Duration) {
        use std::future::{poll_fn, Future};
        use std::task::Poll;
        use tokio::io::{unix::AsyncFd, Interest};

        let sleep = tokio::time::sleep(timeout);

        if fds.is_empty() {
            sleep.await;
            return;
        }

        let guards: Vec<AsyncFd<i32>> = {
            let mut v = Vec::with_capacity(fds.len());
            for fd in fds {
                match AsyncFd::with_interest(fd, Interest::READABLE) {
                    Ok(g) => v.push(g),
                    Err(_) => {
                        sleep.await;
                        return;
                    }
                }
            }
            v
        };

        let mut readable: Vec<_> = guards.iter().map(|g| Box::pin(g.readable())).collect();
        // Resolve as soon as *any* socket is readable. The ready-guard is
        // dropped without clearing readiness on purpose: the pending
        // recv/accept is serviced inside the guest on the next poll, and
        // the following wait builds a fresh AsyncFd.
        let any_readable = poll_fn(|cx| {
            for fut in readable.iter_mut() {
                if let Poll::Ready(res) = fut.as_mut().poll(cx) {
                    return Poll::Ready(res.map(|_| ()));
                }
            }
            Poll::Pending
        });

        tokio::select! {
            _ = any_readable => {}
            _ = sleep => {}
        }
    }

    /// Windows fallback: socket readiness for the async poll loop is not wired
    /// through `AsyncFd`; honour the timer deadline (socket wakeups still flow
    /// through the guest's `__hl_sleep` path, as in the sync case).
    #[cfg(not(unix))]
    async fn wait_readable_or_timeout(_fds: Vec<i32>, timeout: Duration) {
        tokio::time::sleep(timeout).await;
    }

    /// Call a named guest function with typed parameters.
    ///
    /// Thin passthrough to [`MultiUseSandbox::call`] so callers can take
    /// advantage of Hyperlight's multi-function dispatch when the loaded
    /// ELF uses it (e.g. registering an `init` for one-time warm-up and
    /// a `run` for per-call work — see the FC-aware dispatch callback in
    /// plat/hyperlight/dispatch.c).
    ///
    /// Requires a prior `restore()` to reset guest state to the snapshot
    /// the caller wants to run against.
    pub fn call_named<Output, Args>(&mut self, func_name: &str, args: Args) -> Result<Output>
    where
        Output: hyperlight_host::func::SupportedReturnType,
        Args: hyperlight_host::func::ParameterTuple,
    {
        *self.poll_signal.lock().unwrap() = GuestPollSignal::None;
        let out = self.inner.call(func_name, args)?;
        self.check_guest_ran_to_completion("call_named")?;
        Ok(out)
    }

    /// Read the exit code reported by the guest via `__hl_exit`.
    /// Defaults to 0 (success) if the guest never called it.
    pub fn last_exit_code(&self) -> i32 {
        self.exit_code.load(Ordering::Relaxed)
    }

    /// Reset the stored exit code to 0. Call before each guest
    /// invocation so a previous non-zero code doesn't leak.
    pub fn reset_exit_code(&self) {
        self.exit_code.store(0, Ordering::Relaxed);
    }

    /// Obtain a handle that can interrupt a running guest call from
    /// another thread. See [`hyperlight_host::hypervisor::InterruptHandle`].
    pub fn interrupt_handle(&self) -> Arc<dyn hyperlight_host::hypervisor::InterruptHandle> {
        self.inner.interrupt_handle()
    }

    /// Obtain the sleep-cancellation token. Call `.cancel()` on it to
    /// wake any in-progress `__hl_sleep` immediately.
    pub fn sleep_cancel(&self) -> SleepCancel {
        self.sleep_cancel.clone()
    }

    /// Take a new snapshot of the current guest state.
    ///
    /// Useful for the "snapshot after one-time warm-up" pattern: call
    /// `init` once to set up expensive state (e.g. `Py_Initialize` +
    /// heavy imports), then snapshot_now() here to capture the post-
    /// warm-up memory. Subsequent `restore()` calls will return the VM
    /// to this warm state, so per-call work skips the warm-up entirely.
    ///
    /// After this call, future `restore()` calls rewind to the *new*
    /// snapshot rather than the post-evolve one.
    pub fn snapshot_now(&mut self) -> Result<()> {
        let snap = self.inner.snapshot()?;
        self.snapshot = Some(snap);
        Ok(())
    }

    /// Serialize the guest's currently-listening host sockets so they can
    /// be persisted alongside a snapshot and re-created on restore.
    ///
    /// The host socket table lives in host-process memory and is *not*
    /// part of the guest snapshot, so a restored guest would otherwise
    /// find its listener fds dangling. Persist this value next to the
    /// snapshot and pass it back to [`Sandbox::restore_listeners`] before
    /// resuming the guest. Returns `None` when the sandbox has no socket
    /// table (i.e. no network policy configured).
    pub fn export_listeners(&self) -> Option<serde_json::Value> {
        self.socket_table
            .as_ref()
            .map(|t| t.lock().unwrap().export_listeners())
    }

    /// Re-create listening host sockets previously produced by
    /// [`Sandbox::export_listeners`], re-inserting them under their
    /// original fds. Call before the guest resumes so its listener fds
    /// are backed by real, bound, listening sockets.
    pub fn restore_listeners(&mut self, data: &serde_json::Value) -> Result<()> {
        if let Some(ref table) = self.socket_table {
            table.lock().unwrap().restore_listeners(data)?;
        }
        Ok(())
    }

    /// Serialize every async host-tool task the sandbox is currently tracking
    /// so it can be folded into a checkpoint alongside the guest memory image.
    ///
    /// The guest parks on a completion token when it calls an async tool; the
    /// work runs off the vCPU thread and its result is handed back in the next
    /// [`Sandbox::poll`] batch. A checkpoint can land while tasks are still
    /// **pending** (work in flight) or **completed-but-undelivered** (the result
    /// is computed but the guest hasn't polled for it yet). Both kinds live only
    /// in host-process memory — *not* in the guest snapshot — so without this a
    /// restored guest would wait forever for tokens it can never be told about.
    ///
    /// Returns `{"tasks": [ … ]}`, one object per tracked task:
    /// - pending: `{"token","name","args","status":"pending"}`
    /// - completed: `{"token","name","args","status":"completed","result":…}`
    ///   or `{…,"status":"completed","error":"…"}`
    ///
    /// Pass the value to [`Sandbox::restore_async_tasks`] on the restored
    /// sandbox before resuming the guest. Returns `None` when no async tools are
    /// configured (nothing to track).
    pub fn export_async_tasks(&self) -> Option<serde_json::Value> {
        self.async_state.as_ref().map(|st| st.export_tasks())
    }

    /// Repopulate the async-task tracker from a value produced by
    /// [`Sandbox::export_async_tasks`], so a restored guest resumes exactly
    /// where the checkpoint was taken. Call before the guest is polled.
    ///
    /// Per task:
    /// - **completed** — the saved `result`/`error` is re-queued for delivery;
    ///   the next [`Sandbox::poll`] batch hands it to the parked guest call,
    ///   which then returns as if the checkpoint never happened.
    /// - **pending, tool re-registered** — the tool is re-invoked with the saved
    ///   `args` so the work resumes off the vCPU thread and completes normally.
    /// - **pending, tool NOT re-registered** — (e.g. a user `tool_async` handler,
    ///   which is not restored) an error result is delivered for that token so
    ///   the guest unparks and can handle it instead of hanging forever.
    ///
    /// Does nothing when no async tools are configured.
    pub fn restore_async_tasks(&mut self, data: &serde_json::Value) -> Result<()> {
        if let Some(st) = self.async_state.as_mut() {
            st.restore_tasks(data)?;
        }
        Ok(())
    }

    /// Persist the current snapshot to disk in OCI image-layout format.
    ///
    /// `path` is the directory for the OCI layout (created if absent).
    /// The snapshot is tagged `latest`.
    pub fn save_snapshot<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let snap = self
            .snapshot
            .as_ref()
            .ok_or_else(|| anyhow!("no snapshot present; build() or snapshot_now() first"))?;
        let tag = OciTag::new("latest").map_err(|e| anyhow!("{e}"))?;
        snap.save(path.as_ref(), &tag).map_err(|e| anyhow!("{e}"))?;
        sparsify_oci_blobs(path.as_ref())?;
        Ok(())
    }

    /// Load a previously-persisted snapshot from disk and create a
    /// `Sandbox` directly from it, bypassing the entire evolve path.
    /// Every subsequent `call*` runs against the snapshot's post-warmup
    /// state; `restore()` rewinds to it.
    ///
    /// This is the `pyhl run` fast path: `pyhl setup` persists the
    /// warm-Python snapshot once, and every `pyhl run` instantiates
    /// straight from it — no kernel boot, no Py_Initialize.
    ///
    /// Uses `Snapshot::load`, which skips SHA-256 digest verification.
    /// We trust snapshots written by our own `save_snapshot()` in the
    /// same install dir; checked_load costs ~500ms on a 2.5 GB
    /// snapshot — enough to double the whole `pyhl run` wall time.
    pub fn from_snapshot_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_snapshot_file_full(path, &[], None, None, None)
    }

    /// Load a previously-persisted snapshot and register a
    /// preopen-backed `__dispatch` host function on the loaded sandbox,
    /// so guest code that does file I/O through `lib/hostfs` has
    /// working RPC paths.
    ///
    /// The snapshot must have been taken while the guest had hostfs
    /// mounted at each preopen's guest_path (i.e. `pyhl setup` was
    /// invoked with the same guest_paths). At run time only the
    /// `host_dir` side is remapped — the guest-side mount point is
    /// fixed at setup time because it lives in the snapshot's memory
    /// image.
    pub fn from_snapshot_file_with<P: AsRef<Path>>(path: P, preopens: &[Preopen]) -> Result<Self> {
        Self::from_snapshot_file_full(path, preopens, None, None, None)
    }

    /// Load a snapshot with an initrd file re-mapped at the standard
    /// guest VA (0xFEF0_0000). Required when the snapshot was taken
    /// from a cpiovfs-backed guest whose VFS nodes point into the
    /// initrd region.
    pub fn from_snapshot_file_with_initrd<P: AsRef<Path>, I: AsRef<Path>>(
        path: P,
        preopens: &[Preopen],
        initrd: I,
    ) -> Result<Self> {
        Self::from_snapshot_file_full(
            path,
            preopens,
            Some(initrd.as_ref().to_path_buf()),
            None,
            None,
        )
    }

    /// Load a snapshot with full configuration: preopens, initrd,
    /// network policy, and listen-port allowlist.
    pub fn from_snapshot_file_configured<P: AsRef<Path>>(
        path: P,
        preopens: &[Preopen],
        initrd: Option<&Path>,
        network: Option<&NetworkPolicy>,
        listen_ports: Option<&ListenPorts>,
    ) -> Result<Self> {
        Self::from_snapshot_file_full(
            path,
            preopens,
            initrd.map(|p| p.to_path_buf()),
            network,
            listen_ports,
        )
    }

    fn from_snapshot_file_full<P: AsRef<Path>>(
        path: P,
        preopens: &[Preopen],
        initrd: Option<std::path::PathBuf>,
        network: Option<&NetworkPolicy>,
        listen_ports: Option<&ListenPorts>,
    ) -> Result<Self> {
        let tag = OciTag::new("latest").map_err(|e| anyhow!("{e}"))?;
        let loaded = Snapshot::load(path.as_ref(), tag).map_err(|e| anyhow!("{e}"))?;
        let arc = Arc::new(loaded);

        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_signal = Arc::new(Mutex::new(GuestPollSignal::None));
        let sleep_cancel = SleepCancel::new();
        let mut tools = build_tools(None, preopens)?.unwrap_or_default();
        let socket_table = register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_signal,
            &sleep_cancel,
            network,
            listen_ports,
        );
        let async_state = tools.make_async_state();
        let tools = Arc::new(tools);
        let tools_ref = tools.clone();

        let mut host_funcs = HostFunctions::default();
        host_funcs.register_host_function("__dispatch", move |payload: Vec<u8>| -> Vec<u8> {
            tools_ref.dispatch(&payload)
        })?;

        let mut inner = MultiUseSandbox::from_snapshot(arc.clone(), host_funcs, None)?;

        const INITRD_MAP_BASE: u64 = 0xFEF0_0000;
        if let Some(ref initrd_path) = initrd {
            inner.map_file_cow(initrd_path, INITRD_MAP_BASE)?;
        }

        Ok(Self {
            inner,
            snapshot: Some(arc),
            initrd_path: initrd,
            exit_code,
            poll_signal,
            next_wakeup_at: None,
            socket_table,
            sleep_cancel,
            async_state,
        })
    }
}

// ---------------------------------------------------------------------------
// Convenience: run_vm_capture_output (single-shot execution with output)
// ---------------------------------------------------------------------------

/// Output captured from a VM execution.
pub struct VmOutput {
    pub output: String,
    pub setup_time: Duration,
    pub evolve_time: Duration,
}

/// Run a Unikraft kernel and capture its console output.
///
/// Unikraft console output goes through Hyperlight's port I/O to host stderr.
/// This function redirects stderr to a temp file during the call phase to
/// capture it.  The Unikraft dispatch lifecycle is:
///   evolve (boot+init+snapshot) → restore → call_run (app output here)
pub fn run_vm_capture_output(
    kernel_path: &Path,
    initrd: Option<&[u8]>,
    app_args: &[String],
    config: VmConfig,
) -> Result<VmOutput> {
    let setup_start = std::time::Instant::now();

    // Phase 1: evolve — boots the kernel and takes a post-init snapshot.
    // No application output happens here.
    let mut sandbox =
        Sandbox::evolve_inline(kernel_path, initrd, app_args, config, None, &[], None, None)?;
    let setup_time = setup_start.elapsed();

    // Redirect stderr to a temp file before the call phase
    let capture_file = std::env::temp_dir().join(format!(
        "hl-capture-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let capture = stderr_capture::Capture::redirect_to_file(&capture_file)?;

    // Phase 2: restore + call — application runs and produces output
    let evolve_start = std::time::Instant::now();
    sandbox.restore()?;
    let call_result = sandbox.call_run();
    let evolve_time = evolve_start.elapsed();

    // Restore stderr
    capture.restore()?;

    // Read captured output
    let captured = std::fs::read(&capture_file).unwrap_or_default();
    let _ = std::fs::remove_file(&capture_file);
    let captured = String::from_utf8_lossy(&captured).into_owned();

    if let Err(e) = call_result {
        return Err(anyhow!(
            "VM call failed: {}\n--- captured output ---\n{}",
            e,
            captured
        ));
    }

    Ok(VmOutput {
        output: captured,
        setup_time,
        evolve_time,
    })
}

// ---------------------------------------------------------------------------
// FsSandbox tests — prove that host-side path resolution rejects escapes.
//
// These cover both attack vectors the host can see: lexical ".." /
// absolute paths passed in an RPC arg, and symlinks inside the mount
// that point outside it.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// OCI blob sparsification
// ---------------------------------------------------------------------------

fn sparsify_oci_blobs(oci_dir: &Path) -> Result<()> {
    let blobs_dir = oci_dir.join("blobs").join("sha256");
    if !blobs_dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(&blobs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && entry.metadata()?.len() > 1024 * 1024 {
            sparsify_file(&path)?;
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn sparsify_file(path: &Path) -> Result<()> {
    use std::io::Read;
    use std::os::unix::io::AsRawFd;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let len = file.metadata()?.len();

    const PAGE: usize = 4096;
    let zero_page = [0u8; PAGE];
    let mut buf = [0u8; PAGE];
    let mut reader = std::io::BufReader::with_capacity(PAGE, &file);

    let mut punched = 0u64;
    let mut offset: u64 = 0;
    while offset + PAGE as u64 <= len {
        reader.read_exact(&mut buf)?;
        if buf == zero_page {
            let ret = unsafe {
                libc::fallocate(
                    file.as_raw_fd(),
                    libc::FALLOC_FL_PUNCH_HOLE | libc::FALLOC_FL_KEEP_SIZE,
                    offset as i64,
                    PAGE as i64,
                )
            };
            if ret == 0 {
                punched += 1;
            }
        }
        offset += PAGE as u64;
    }

    if punched > 0 {
        let disk_mib = (len - punched * PAGE as u64) / 1024 / 1024;
        eprintln!("  sparsified: {disk_mib} MiB on disk (punched {punched} zero pages)");
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn sparsify_file(path: &Path) -> Result<()> {
    use std::io::Read;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::System::Ioctl::{FSCTL_SET_SPARSE, FSCTL_SET_ZERO_DATA};
    use windows_sys::Win32::System::IO::DeviceIoControl;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let len = file.metadata()?.len();
    let handle = file.as_raw_handle();

    let ok = unsafe {
        DeviceIoControl(
            handle,
            FSCTL_SET_SPARSE,
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Ok(());
    }

    const PAGE: usize = 4096;
    let zero_page = [0u8; PAGE];
    let mut buf = [0u8; PAGE];
    let mut reader = std::io::BufReader::with_capacity(PAGE, &file);

    let mut punched = 0u64;
    let mut offset: u64 = 0;
    while offset + PAGE as u64 <= len {
        reader.read_exact(&mut buf)?;
        if buf != zero_page {
            offset += PAGE as u64;
            continue;
        }
        let range_start = offset;
        offset += PAGE as u64;
        let mut range_end = offset;
        while offset + PAGE as u64 <= len {
            reader.read_exact(&mut buf)?;
            offset += PAGE as u64;
            if buf != zero_page {
                break;
            }
            range_end = offset;
        }

        #[repr(C)]
        struct FileZeroDataInformation {
            file_offset: i64,
            beyond_final_zero: i64,
        }

        let info = FileZeroDataInformation {
            file_offset: range_start as i64,
            beyond_final_zero: range_end as i64,
        };
        let ok = unsafe {
            DeviceIoControl(
                handle,
                FSCTL_SET_ZERO_DATA,
                &info as *const _ as *const _,
                std::mem::size_of::<FileZeroDataInformation>() as u32,
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if ok != 0 {
            punched += (range_end - range_start) / PAGE as u64;
        }
    }

    if punched > 0 {
        let disk_mib = (len - punched * PAGE as u64) / 1024 / 1024;
        eprintln!("  sparsified: {disk_mib} MiB on disk (punched {punched} zero pages)");
    }
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn sparsify_file(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn framed_request(id: u64, json: &[u8]) -> Vec<u8> {
        encode_async_frame(ASYNC_FRAME_REQUEST, id, json)
    }

    fn framed_json(response: &[u8], kind: u8, id: u64) -> serde_json::Value {
        let frame = decode_async_frame(response)
            .unwrap()
            .expect("response must be framed");
        assert_eq!(frame.kind, kind);
        assert_eq!(frame.id, id);
        serde_json::from_slice(frame.payload).expect("frame payload must be JSON")
    }

    #[test]
    fn async_frame_roundtrip_preserves_binary_payload() {
        let payload = b"\0{\"result\":\"not control JSON\"}\xff";
        let encoded = encode_async_frame(ASYNC_FRAME_RESULT, 42, payload);
        let frame = decode_async_frame(&encoded).unwrap().unwrap();

        assert_eq!(frame.kind, ASYNC_FRAME_RESULT);
        assert_eq!(frame.id, 42);
        assert_eq!(frame.payload, payload);
    }

    #[test]
    fn async_frame_rejects_malformed_header_and_length() {
        let mut encoded = encode_async_frame(ASYNC_FRAME_REQUEST, 1, b"{}");
        encoded[4] = ASYNC_FRAME_VERSION + 1;
        assert!(decode_async_frame(&encoded).is_err());

        let mut encoded = encode_async_frame(ASYNC_FRAME_REQUEST, 1, b"{}");
        encoded[16..20].copy_from_slice(&3u32.to_le_bytes());
        assert!(decode_async_frame(&encoded).is_err());
    }

    /// Async `wait_readable_or_timeout` behaviour, proving two things:
    /// 1. It returns via the timer when no fd is readable (idle wait).
    /// 2. It returns promptly once an fd becomes readable — using a *blocking*
    ///    `UnixStream`, demonstrating that `AsyncFd` needs no `O_NONBLOCK`.
    #[cfg(unix)]
    #[tokio::test]
    async fn wait_readable_or_timeout_wakes_on_readable_blocking_fd() {
        use std::io::Write;
        use std::os::unix::io::AsRawFd;
        use std::os::unix::net::UnixStream;

        // Blocking by default — intentionally not set to non-blocking, to
        // prove AsyncFd needs no O_NONBLOCK for readiness.
        let (rx, mut tx) = UnixStream::pair().unwrap();

        // (1) Nothing to read yet: the ~100ms timer should elapse.
        let start = std::time::Instant::now();
        Sandbox::wait_readable_or_timeout(vec![rx.as_raw_fd()], Duration::from_millis(100)).await;
        assert!(
            start.elapsed() >= Duration::from_millis(80),
            "expected to wait out the timeout, elapsed {:?}",
            start.elapsed()
        );

        // (2) Peer writes → fd becomes readable → return well before the cap.
        tx.write_all(b"x").unwrap();
        tx.flush().unwrap();
        let start = std::time::Instant::now();
        Sandbox::wait_readable_or_timeout(vec![rx.as_raw_fd()], Duration::from_secs(5)).await;
        assert!(
            start.elapsed() < Duration::from_secs(1),
            "expected a prompt readable wake, elapsed {:?}",
            start.elapsed()
        );
    }

    /// With no sockets, the wait is purely the timer.
    #[cfg(unix)]
    #[tokio::test]
    async fn wait_readable_or_timeout_empty_honours_timer() {
        let start = std::time::Instant::now();
        Sandbox::wait_readable_or_timeout(Vec::new(), Duration::from_millis(80)).await;
        assert!(start.elapsed() >= Duration::from_millis(60));
    }

    /// Build an [`AsyncToolState`] with the given factories, wired to fresh
    /// shared maps — enough to unit-test export/restore without a real VM.
    #[cfg(test)]
    fn async_state_with(factories: HashMap<String, ToolFactory>) -> AsyncToolState {
        AsyncToolState {
            pending: Arc::new(Mutex::new(HashMap::new())),
            queue: Arc::new(Mutex::new(std::collections::VecDeque::new())),
            running: FuturesUnordered::new(),
            factories,
        }
    }

    /// A pending task whose tool is still registered is re-queued (re-driven)
    /// on restore; a completed task's result/error is restored verbatim for
    /// re-delivery; and a pending task whose tool is gone becomes an error so
    /// the guest can't hang on it. Snapshot tokens use fixed-width hex strings.
    #[test]
    fn async_task_export_restore_roundtrip() {
        // Source state: one running task (net_recv), one completed-ok task,
        // one completed-error task. Internal keys remain u64 IDs.
        let src = async_state_with(HashMap::new());
        {
            let mut p = src.pending.lock().unwrap();
            p.insert(
                1u64,
                PendingTask {
                    name: "net_recv".into(),
                    args: serde_json::json!({ "fd": 4, "len": 16 }),
                    state: PendingState::Running,
                },
            );
            p.insert(
                2u64,
                PendingTask {
                    name: "net_recv".into(),
                    args: serde_json::json!({ "fd": 5 }),
                    state: PendingState::Ready(Ok(serde_json::json!({ "data": "aGk=" }))),
                },
            );
            p.insert(
                3u64,
                PendingTask {
                    name: "user_tool".into(),
                    args: serde_json::Value::Null,
                    state: PendingState::Ready(Err("boom".into())),
                },
            );
        }

        let snapshot = src.export_tasks();
        let tasks = snapshot["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 3);
        assert!(
            tasks.iter().all(|t| {
                t["token"]
                    .as_str()
                    .is_some_and(|s| s.len() == REQUEST_ID_HEX_LEN)
            }),
            "all snapshot token values must be fixed-width hex strings, got: {snapshot}"
        );

        // Restore into a fresh state that only re-registers `net_recv`
        // (a re-invokable factory) — `user_tool` is intentionally absent.
        let mut factories: HashMap<String, ToolFactory> = HashMap::new();
        factories.insert(
            "net_recv".into(),
            Arc::new(|_args| Box::pin(async { Ok(serde_json::json!("resumed")) })),
        );
        let mut dst = async_state_with(factories);
        dst.restore_tasks(&snapshot).unwrap();

        let p = dst.pending.lock().unwrap();
        assert_eq!(p.len(), 3);

        // (1) Running task with a live factory → re-queued as Running, and its
        //     future was pushed onto the drive queue.
        assert!(matches!(p[&1u64].state, PendingState::Running));
        assert_eq!(p[&1u64].name, "net_recv");
        assert_eq!(dst.queue.lock().unwrap().len(), 1);

        // (2) Completed-ok task → result restored verbatim for re-delivery.
        match &p[&2u64].state {
            PendingState::Ready(Ok(v)) => assert_eq!(v["data"], "aGk="),
            other => panic!("expected Ready(Ok), got {other:?}"),
        }

        // (3) Completed-error task → error restored verbatim.
        match &p[&3u64].state {
            PendingState::Ready(Err(e)) => assert_eq!(e, "boom"),
            other => panic!("expected Ready(Err), got {other:?}"),
        }
    }

    /// A task that was *pending* on a tool no longer registered after restore
    /// becomes a delivered error, so the resumed guest unparks instead of
    /// hanging forever on a token that can never be fulfilled.
    /// Snapshot token must be a fixed-width hexadecimal string.
    #[test]
    fn pending_task_without_factory_becomes_error_on_restore() {
        let snapshot = serde_json::json!({
            "tasks": [
                { "token": "0000000000000009", "name": "gone_tool", "args": {}, "status": "pending" }
            ]
        });
        let mut dst = async_state_with(HashMap::new());
        dst.restore_tasks(&snapshot).unwrap();

        let p = dst.pending.lock().unwrap();
        match &p[&9u64].state {
            PendingState::Ready(Err(e)) => assert!(e.contains("could not be resumed")),
            other => panic!("expected Ready(Err), got {other:?}"),
        }
        // Nothing to re-drive.
        assert!(dst.queue.lock().unwrap().is_empty());
    }

    /// A framed request dispatching an async tool returns a pending frame with
    /// the same numeric request ID.
    #[test]
    fn dispatch_async_frame_yields_matching_id() {
        let mut tools = ToolRegistry::new();
        tools.register_async_factory(
            "echo_async",
            Arc::new(|args| Box::pin(async move { Ok(args) })),
        );

        let req = framed_request(42, br#"{"name":"echo_async","args":"hi"}"#);
        let resp = tools.dispatch(&req);
        let frame = decode_async_frame(&resp).unwrap().unwrap();
        assert_eq!(frame.kind, ASYNC_FRAME_PENDING);
        assert_eq!(frame.id, 42);
        assert!(frame.payload.is_empty());
    }

    #[test]
    fn request_id_parser_requires_canonical_nonzero_hex() {
        assert_eq!(
            parse_request_id(&serde_json::json!("ffffffffffffffff"), "__hl_request_id").unwrap(),
            u64::MAX
        );

        for invalid in [
            serde_json::json!(42),
            serde_json::json!("2a"),
            serde_json::json!("000000000000002A"),
            serde_json::json!("0000000000000000"),
        ] {
            assert!(
                parse_request_id(&invalid, "__hl_request_id").is_err(),
                "non-canonical request ID should be rejected: {invalid}"
            );
        }
    }

    /// Calling an async tool without a binary request frame — as a legacy
    /// non-poll guest does — resolves the tool inline and returns its result
    /// as plain JSON, since such a guest has no drive loop.
    #[test]
    fn dispatch_async_without_frame_resolves_inline() {
        let mut tools = ToolRegistry::new();
        tools.register_async_factory(
            "echo_async",
            Arc::new(|args| Box::pin(async move { Ok(args) })),
        );

        let req = br#"{"name":"echo_async","args":"hi"}"#;
        let resp = tools.dispatch(req);
        assert!(
            decode_async_frame(&resp).unwrap().is_none(),
            "legacy caller must receive plain JSON, not a frame"
        );
        let v: serde_json::Value = serde_json::from_slice(&resp).unwrap();
        assert_eq!(
            v["result"], "hi",
            "async tool result should be returned inline: {v}"
        );
    }

    /// A binary request frame with ID 0 is rejected and no task is queued.
    #[test]
    fn dispatch_async_zero_id_returns_error() {
        let mut tools = ToolRegistry::new();
        tools.register_async_factory(
            "echo_async",
            Arc::new(|args| Box::pin(async move { Ok(args) })),
        );

        let req = framed_request(0, br#"{"name":"echo_async","args":"hi"}"#);
        let resp = tools.dispatch(&req);
        let value = framed_json(&resp, ASYNC_FRAME_RESULT, 0);
        let s = value.to_string();
        assert!(
            s.contains("\"error\""),
            "zero ID should produce an error: {s}"
        );
        assert!(
            s.contains("nonzero"),
            "error should mention the ID must be nonzero: {s}"
        );
        let asy = tools.async_side.as_ref().unwrap();
        assert!(
            asy.pending.lock().unwrap().is_empty(),
            "no task should be registered for a rejected zero ID"
        );
        assert!(
            asy.queue.lock().unwrap().is_empty(),
            "no future should be queued for a rejected zero ID"
        );
    }

    /// A duplicate in-flight ID is rejected without overwriting the existing
    /// pending task or queuing a second future.
    #[test]
    fn dispatch_async_duplicate_id_rejected_no_overwrite() {
        let mut tools = ToolRegistry::new();
        tools.register_async_factory(
            "slow_async",
            Arc::new(|_args| {
                Box::pin(async {
                    tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
                    Ok(serde_json::Value::Null)
                })
            }),
        );

        let req1 = framed_request(99, br#"{"name":"slow_async","args":{"seq":1}}"#);
        let resp1 = tools.dispatch(&req1);
        let frame1 = decode_async_frame(&resp1).unwrap().unwrap();
        assert_eq!(frame1.kind, ASYNC_FRAME_PENDING);
        assert_eq!(frame1.id, 99);

        let req2 = framed_request(99, br#"{"name":"slow_async","args":{"seq":2}}"#);
        let resp2 = tools.dispatch(&req2);
        let s2 = framed_json(&resp2, ASYNC_FRAME_RESULT, 99).to_string();
        assert!(
            s2.contains("\"error\""),
            "duplicate ID should return an error: {s2}"
        );
        assert!(
            s2.contains("duplicate") || s2.contains("already pending"),
            "error should mention duplicate/pending: {s2}"
        );

        let asy = tools.async_side.as_ref().unwrap();
        let pending = asy.pending.lock().unwrap();
        let task = pending.get(&99u64).expect("original task must still exist");
        assert_eq!(task.args["seq"], 1, "original args must be intact");
        drop(pending);
        assert_eq!(
            asy.queue.lock().unwrap().len(),
            1,
            "only one future should have been queued"
        );
    }

    /// Distinct guest IDs cannot grow the host's pending-task registry without
    /// bound. Once the per-sandbox limit is reached, no additional future is
    /// queued and the existing registry remains intact.
    #[test]
    fn dispatch_async_pending_limit_rejected() {
        let mut tools = ToolRegistry::new();
        tools.register_async_factory(
            "async_tool",
            Arc::new(|args| Box::pin(async move { Ok(args) })),
        );
        let asy = tools.async_side.as_ref().unwrap();
        {
            let mut pending = asy.pending.lock().unwrap();
            for id in 1..=MAX_PENDING_ASYNC_TASKS as u64 {
                pending.insert(
                    id,
                    PendingTask {
                        name: "async_tool".into(),
                        args: serde_json::Value::Null,
                        state: PendingState::Running,
                    },
                );
            }
        }

        let request_id = MAX_PENDING_ASYNC_TASKS as u64 + 1;
        let req = framed_request(request_id, br#"{"name":"async_tool","args":null}"#);
        let resp = tools.dispatch(&req);
        let value = framed_json(&resp, ASYNC_FRAME_RESULT, request_id);
        assert!(
            value["error"]
                .as_str()
                .is_some_and(|e| e.contains("too many in-flight")),
            "pending-limit rejection should be explicit: {value}"
        );
        assert_eq!(asy.pending.lock().unwrap().len(), MAX_PENDING_ASYNC_TASKS);
        assert!(asy.queue.lock().unwrap().is_empty());
    }

    /// `restore_tasks` rejects a snapshot that contains a duplicate token and
    /// commits **nothing** — the restore is transactional, so neither the first
    /// nor the second colliding entry is left behind (no partial restore).
    #[test]
    fn snapshot_restore_rejects_duplicate_token() {
        let snapshot = serde_json::json!({
            "tasks": [
                { "token": "0000000000000005", "name": "tool_a", "args": {}, "status": "completed", "result": "first" },
                { "token": "0000000000000005", "name": "tool_b", "args": {}, "status": "completed", "result": "second" }
            ]
        });
        let mut dst = async_state_with(HashMap::new());
        let err = dst.restore_tasks(&snapshot).unwrap_err();
        assert!(
            err.to_string().contains("duplicate"),
            "restore should reject duplicate token 5: {err}"
        );
        let p = dst.pending.lock().unwrap();
        assert!(
            p.is_empty(),
            "a rejected snapshot must leave no partial state, got: {} entries",
            p.len()
        );
        drop(p);
        assert!(
            dst.queue.lock().unwrap().is_empty(),
            "a rejected snapshot must not queue any resume futures"
        );
    }

    /// `restore_tasks` rejects a snapshot whose token collides with a task
    /// already in-flight in the current registry, and commits nothing from the
    /// snapshot — the pre-existing task is untouched and the other (non-
    /// colliding) snapshot entry is NOT partially restored.
    #[test]
    fn snapshot_restore_rejects_collision_with_registry_no_partial() {
        let mut dst = async_state_with(HashMap::new());
        // Seed the live registry with an in-flight task under token 7.
        dst.pending.lock().unwrap().insert(
            7u64,
            PendingTask {
                name: "live_tool".into(),
                args: serde_json::json!({ "keep": true }),
                state: PendingState::Running,
            },
        );

        // Snapshot: one fresh token (8) and one colliding token (7).
        let snapshot = serde_json::json!({
            "tasks": [
                { "token": "0000000000000008", "name": "tool_new", "args": {}, "status": "completed", "result": "x" },
                { "token": "0000000000000007", "name": "tool_dup", "args": {}, "status": "completed", "result": "y" }
            ]
        });
        let err = dst.restore_tasks(&snapshot).unwrap_err();
        assert!(
            err.to_string().contains("duplicate"),
            "restore should reject the registry collision on token 7: {err}"
        );

        let p = dst.pending.lock().unwrap();
        // Pre-existing task intact.
        let live = p.get(&7u64).expect("pre-existing task must survive");
        assert_eq!(live.name, "live_tool", "existing task must be untouched");
        assert_eq!(live.args["keep"], true);
        // Non-colliding entry must NOT have been partially restored.
        assert!(
            p.get(&8u64).is_none(),
            "no snapshot entry may be partially restored on rejection"
        );
        assert_eq!(p.len(), 1, "only the pre-existing task should remain");
    }

    /// `restore_tasks` rejects a zero token (never a valid guest request ID).
    #[test]
    fn snapshot_restore_rejects_zero_token() {
        let snapshot = serde_json::json!({
            "tasks": [
                { "token": "0000000000000000", "name": "tool_a", "args": {}, "status": "pending" }
            ]
        });
        let mut dst = async_state_with(HashMap::new());
        let err = dst.restore_tasks(&snapshot).unwrap_err();
        assert!(
            err.to_string().contains("nonzero"),
            "restore should reject a zero token: {err}"
        );
        assert!(dst.pending.lock().unwrap().is_empty());
    }

    /// `restore_tasks` rejects strings that are not canonical request IDs.
    #[test]
    fn snapshot_restore_rejects_malformed_hex_token() {
        let snapshot = serde_json::json!({
            "tasks": [
                { "token": "__hlasync-1", "name": "tool_a", "args": {}, "status": "pending" }
            ]
        });
        let mut dst = async_state_with(HashMap::new());
        let err = dst.restore_tasks(&snapshot).unwrap_err();
        assert!(
            err.to_string().contains("16-character lowercase hex"),
            "restore should reject a malformed token: {err}"
        );
    }

    fn tmpdir(label: &str) -> std::path::PathBuf {
        let p =
            std::env::temp_dir().join(format!("hl-fs-sandbox-{}-{}", label, std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn normalize_enoent_rewrites_windows_wording_to_linux() {
        // Windows Rust I/O wording:
        let win = "fs_stat \"/host/x\": The system cannot find the file specified. (os error 2)";
        let out = normalize_fs_error(win);
        assert!(
            out.contains("No such file or directory"),
            "expected Linux wording, got: {out}"
        );
        assert!(out.contains("(os error 2)"));
        assert!(out.starts_with("fs_stat \"/host/x\":"));
    }

    #[test]
    fn normalize_leaves_linux_wording_alone() {
        let linux = "fs_stat \"/host/x\": No such file or directory (os error 2)";
        let out = normalize_fs_error(linux);
        assert!(out.contains("No such file or directory (os error 2)"));
    }

    #[test]
    fn normalize_passes_unknown_errors_through() {
        let weird = "fs_stat \"/host/x\": something extremely unusual happened";
        let out = normalize_fs_error(weird);
        assert_eq!(out, weird);
    }

    #[test]
    fn resolve_rejects_parent_escape() {
        let root = tmpdir("parent");
        let fs = FsSandbox::new(&root).unwrap();
        let err = fs.resolve("../etc/passwd").unwrap_err().to_string();
        assert!(err.contains("escapes mount root"), "{err}");
    }

    #[test]
    fn resolve_rejects_deep_parent_escape() {
        let root = tmpdir("deep");
        let fs = FsSandbox::new(&root).unwrap();
        let err = fs.resolve("a/b/../../../outside").unwrap_err().to_string();
        assert!(err.contains("escapes mount root"), "{err}");
    }

    #[test]
    fn resolve_treats_absolute_paths_as_mount_relative() {
        // A leading '/' is stripped, so "/etc/passwd" becomes
        // "etc/passwd" under the mount — not the host's /etc/passwd.
        let root = tmpdir("abs");
        fs::create_dir(root.join("etc")).unwrap();
        fs::write(root.join("etc/passwd"), "fake").unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let resolved = fs_sb.resolve("/etc/passwd").unwrap();
        assert_eq!(resolved, fs_sb.root().join("etc").join("passwd"));
    }

    #[cfg(unix)]
    #[test]
    fn resolve_rejects_symlink_escape() {
        use std::os::unix::fs::symlink;
        let root = tmpdir("symlink");
        let outside = tmpdir("outside");
        fs::write(outside.join("secret"), "nope").unwrap();
        symlink(outside.join("secret"), root.join("leak")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let err = fs_sb.resolve("leak").unwrap_err().to_string();
        assert!(err.contains("escapes mount root"), "{err}");
    }

    #[cfg(unix)]
    #[test]
    fn resolve_rejects_symlink_escape_via_ancestor() {
        // A symlinked parent directory is just as effective: any child
        // under it resolves outside the root.
        use std::os::unix::fs::symlink;
        let root = tmpdir("ancestor");
        let outside = tmpdir("outside-anc");
        symlink(&outside, root.join("shortcut")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let err = fs_sb.resolve("shortcut/anything").unwrap_err().to_string();
        assert!(err.contains("escapes mount root"), "{err}");
    }

    #[cfg(unix)]
    #[test]
    fn resolve_rejects_dangling_symlink_escape() {
        use std::os::unix::fs::symlink;
        let root = tmpdir("dangling-escape");
        let outside = tmpdir("dangling-escape-out");
        symlink(outside.join("nonexistent"), root.join("bad_link")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let err = fs_sb.resolve("bad_link").unwrap_err().to_string();
        assert!(
            err.contains("escapes mount root"),
            "expected escape error, got: {err}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn resolve_allows_valid_internal_symlink() {
        use std::os::unix::fs::symlink;
        let root = tmpdir("valid-internal");
        fs::write(root.join("real_file.txt"), "hello").unwrap();
        symlink(root.join("real_file.txt"), root.join("good_link")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let resolved = fs_sb.resolve("good_link").unwrap();
        assert!(
            resolved.starts_with(&root),
            "expected path under root, got: {resolved:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn resolve_allows_dangling_symlink_inside_root() {
        use std::os::unix::fs::symlink;
        let root = tmpdir("dangling-inside");
        symlink(root.join("future_file.txt"), root.join("ok_link")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let resolved = fs_sb.resolve("ok_link").unwrap();
        assert!(
            resolved.starts_with(&root),
            "expected path under root, got: {resolved:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn resolve_rejects_symlink_chain_escape() {
        use std::os::unix::fs::symlink;
        let root = tmpdir("chain-escape");
        let outside = tmpdir("chain-outside");
        symlink(&outside, root.join("link_b")).unwrap();
        symlink(root.join("link_b"), root.join("link_a")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let err = fs_sb.resolve("link_a").unwrap_err().to_string();
        assert!(
            err.contains("escapes mount root"),
            "expected escape error, got: {err}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn resolve_rejects_chained_dangling_symlink_escape() {
        use std::os::unix::fs::symlink;
        let root = tmpdir("chain-dangling");
        let outside = tmpdir("chain-dangling-out");
        // link_b -> dangling path outside root
        symlink(outside.join("nonexistent"), root.join("link_b")).unwrap();
        // link_a -> link_b (which is under root, but chains outside)
        symlink(root.join("link_b"), root.join("link_a")).unwrap();
        let fs_sb = FsSandbox::new(&root).unwrap();
        let err = fs_sb.resolve("link_a").unwrap_err().to_string();
        assert!(
            err.contains("escapes mount root"),
            "expected escape error, got: {err}"
        );
    }

    #[test]
    fn resolve_allows_paths_under_the_root() {
        let root = tmpdir("allow");
        let fs = FsSandbox::new(&root).unwrap();
        let resolved = fs.resolve("subdir/file.txt").unwrap();
        assert!(resolved.starts_with(fs.root()), "{resolved:?}");
    }

    #[test]
    fn fs_read_over_dispatch_rejects_escape() {
        // End-to-end through the tool registry: the error surface the
        // guest actually sees.
        let root = tmpdir("dispatch");
        let preopens = vec![Preopen::new(&root, "/host").unwrap()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_read","args":{"path":"/host/../outside.txt"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("escapes mount root"), "{s}");
    }

    #[test]
    fn preopen_parse_defaults_guest_to_host() {
        let dir = tmpdir("po1");
        let p = Preopen::parse_cli(dir.to_str().unwrap()).unwrap();
        assert_eq!(p.guest_path, "/host");
        assert_eq!(p.host_dir, std::fs::canonicalize(&dir).unwrap());
    }

    #[test]
    fn preopen_parse_accepts_custom_guest_path() {
        let dir = tmpdir("po2");
        let spec = format!("{}:/data", dir.display());
        let p = Preopen::parse_cli(&spec).unwrap();
        assert_eq!(p.guest_path, "/data");
    }

    #[test]
    fn preopen_rejects_reserved_guest_path() {
        let dir = tmpdir("po3");
        for reserved in &["/", "/bin", "/dev", "/proc", "/sys", "/usr", "/bin/foo"] {
            let err = Preopen::new(&dir, *reserved).unwrap_err().to_string();
            assert!(err.contains("reserved"), "{reserved}: {err}");
        }
    }

    #[test]
    fn preopen_rejects_relative_guest_path() {
        let dir = tmpdir("po4");
        let err = Preopen::new(&dir, "relative").unwrap_err().to_string();
        assert!(err.contains("absolute"), "{err}");
    }

    fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|w| w == needle)
    }

    #[test]
    fn initdata_carries_mount_tlv_when_preopens_set() {
        let root_a = tmpdir("mnt-a");
        let root_b = tmpdir("mnt-b");
        let preopens = vec![
            Preopen::new(&root_a, "/data").unwrap(),
            Preopen::new(&root_b, "/logs").unwrap(),
        ];
        let buf = build_cmdline_initdata(&["/hello".to_string()], 0, &preopens).expect("initdata");
        assert!(buf.starts_with(CMDLINE_MAGIC), "cmdline magic missing");
        let off = find_subslice(&buf, MOUNT_MAGIC).expect("mount magic missing");
        let count_off = off + MOUNT_MAGIC.len();
        let count = u32::from_le_bytes(buf[count_off..count_off + 4].try_into().unwrap());
        assert_eq!(count, 2);
        // First path is /data, second is /logs.
        let mut p = count_off + 4;
        for expected in ["/data", "/logs"] {
            let len = u32::from_le_bytes(buf[p..p + 4].try_into().unwrap()) as usize;
            assert_eq!(&buf[p + 4..p + 4 + len], expected.as_bytes());
            assert_eq!(buf[p + 4 + len], 0);
            p += 4 + len + 1;
        }
    }

    #[test]
    fn initdata_omits_mount_tlv_when_no_preopens() {
        let buf = build_cmdline_initdata(&["/hello".to_string()], 0, &[]).expect("initdata");
        assert!(buf.starts_with(CMDLINE_MAGIC));
        assert!(
            find_subslice(&buf, MOUNT_MAGIC).is_none(),
            "no mount TLV expected"
        );
    }

    #[test]
    fn fs_write_then_read_roundtrip() {
        let root = tmpdir("roundtrip");
        let preopens = vec![Preopen::new(&root, "/host").unwrap()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let w = br#"{"name":"fs_write","args":{"path":"/host/hello.txt","text":"hi"}}"#;
        let resp = reg.dispatch(w);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"bytes_written\":2"), "{s}");

        let r = br#"{"name":"fs_read","args":{"path":"/host/hello.txt"}}"#;
        let resp = reg.dispatch(r);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"text\":\"hi\""), "{s}");
    }

    // -- Read-only mount tests ------------------------------------------------

    #[test]
    fn readonly_mount_allows_reads() {
        let root = tmpdir("ro-read");
        fs::write(root.join("file.txt"), b"hello").unwrap();
        let preopens = vec![Preopen::new(&root, "/data").unwrap().read_only()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_read","args":{"path":"/data/file.txt"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"text\":\"hello\""), "{s}");
    }

    #[test]
    fn readonly_mount_blocks_fs_write() {
        let root = tmpdir("ro-write");
        let preopens = vec![Preopen::new(&root, "/data").unwrap().read_only()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_write","args":{"path":"/data/new.txt","text":"nope"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("read-only mount"), "{s}");
    }

    #[test]
    fn readonly_mount_blocks_fs_mkdir() {
        let root = tmpdir("ro-mkdir");
        let preopens = vec![Preopen::new(&root, "/data").unwrap().read_only()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_mkdir","args":{"path":"/data/subdir"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("read-only mount"), "{s}");
    }

    #[test]
    fn readonly_mount_blocks_fs_unlink() {
        let root = tmpdir("ro-unlink");
        fs::write(root.join("victim.txt"), b"data").unwrap();
        let preopens = vec![Preopen::new(&root, "/data").unwrap().read_only()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_unlink","args":{"path":"/data/victim.txt"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("read-only mount"), "{s}");
        assert!(
            root.join("victim.txt").exists(),
            "file should not be deleted"
        );
    }

    #[test]
    fn readonly_mount_blocks_fs_truncate() {
        let root = tmpdir("ro-trunc");
        fs::write(root.join("file.txt"), b"hello world").unwrap();
        let preopens = vec![Preopen::new(&root, "/data").unwrap().read_only()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_truncate","args":{"path":"/data/file.txt","length":0}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("read-only mount"), "{s}");
    }

    #[test]
    fn readonly_mount_blocks_fs_write_bytes() {
        let root = tmpdir("ro-wbytes");
        let preopens = vec![Preopen::new(&root, "/data").unwrap().read_only()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req = br#"{"name":"fs_write_bytes","args":{"path":"/data/bin.dat","data":"AAAA"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("read-only mount"), "{s}");
    }

    #[test]
    fn mixed_rw_and_ro_mounts() {
        let rw_root = tmpdir("mixed-rw");
        let ro_root = tmpdir("mixed-ro");
        fs::write(ro_root.join("existing.txt"), b"read me").unwrap();
        let preopens = vec![
            Preopen::new(&rw_root, "/rw").unwrap(),
            Preopen::new(&ro_root, "/ro").unwrap().read_only(),
        ];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        // Write to rw mount succeeds
        let req = br#"{"name":"fs_write","args":{"path":"/rw/ok.txt","text":"yes"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"bytes_written\""), "{s}");

        // Read from ro mount succeeds
        let req = br#"{"name":"fs_read","args":{"path":"/ro/existing.txt"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"text\":\"read me\""), "{s}");

        // Write to ro mount fails
        let req = br#"{"name":"fs_write","args":{"path":"/ro/nope.txt","text":"no"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "{s}");
        assert!(s.contains("read-only mount"), "{s}");
    }

    // -- NetworkPolicy tests --------------------------------------------------

    #[test]
    fn network_policy_allow_all_permits_any() {
        let policy = NetworkPolicy::AllowAll;
        let addr: std::net::SocketAddr = "1.2.3.4:443".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn network_policy_allowlist_permits_listed_ip() {
        let al = AllowList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: std::net::SocketAddr = "1.2.3.4:443".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn network_policy_allowlist_denies_unlisted_ip() {
        let al = AllowList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: std::net::SocketAddr = "5.6.7.8:80".parse().unwrap();
        let err = policy.check(&addr).unwrap_err();
        assert!(err.to_string().contains("network policy denies"), "{err}");
    }

    #[test]
    fn allowlist_permits_dns_to_well_known_resolvers() {
        // AllowList exempts port 53 to well-known public DNS and host
        // resolvers so the guest's hardcoded nameservers (8.8.8.8 etc.)
        // work even when the host uses different resolvers.
        let al = AllowList::from_hosts(&["198.51.100.1"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        for ip in ["8.8.8.8:53", "8.8.4.4:53", "1.1.1.1:53", "1.0.0.1:53"] {
            let addr: std::net::SocketAddr = ip.parse().unwrap();
            assert!(
                policy.check(&addr).is_ok(),
                "AllowList must permit DNS (port 53) to well-known resolver {}",
                ip
            );
        }
    }

    #[test]
    fn allowlist_denies_port53_to_arbitrary_ip() {
        // Port 53 is NOT a blanket bypass — only known DNS resolvers
        // are exempted. RFC5737 TEST-NET addresses are never resolvers.
        let al = AllowList::from_hosts(&["198.51.100.1"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: std::net::SocketAddr = "198.51.100.99:53".parse().unwrap();
        assert!(
            policy.check(&addr).is_err(),
            "port 53 to an unknown IP must still be denied"
        );
    }

    #[test]
    fn allowlist_denies_non_dns_to_unlisted_ip() {
        let al = AllowList::from_hosts(&["198.51.100.1"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: std::net::SocketAddr = "198.51.100.99:443".parse().unwrap();
        assert!(
            policy.check(&addr).is_err(),
            "AllowList must deny non-DNS traffic to unlisted IPs"
        );
    }

    #[test]
    fn test_port53_blocklist_enforced() {
        // RFC5737 TEST-NET-1 address — blocklist always wins.
        let bl = BlockList::from_hosts(&["192.0.2.1"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: std::net::SocketAddr = "192.0.2.1:53".parse().unwrap();
        assert!(
            policy.check(&addr).is_err(),
            "blocklisted IP must be denied even on port 53"
        );
    }

    #[test]
    fn allowlist_resolves_hostnames() {
        let al = AllowList::from_hosts(&["localhost"]).unwrap();
        assert!(
            al.is_allowed(&"127.0.0.1".parse().unwrap()) || al.is_allowed(&"::1".parse().unwrap())
        );
    }

    #[test]
    fn allowlist_rejects_unresolvable_hostname() {
        let result = AllowList::from_hosts(&["this.host.definitely.does.not.exist.example"]);
        assert!(result.is_err());
    }

    #[test]
    fn network_policy_blocklist_permits_unlisted_ip() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: std::net::SocketAddr = "5.6.7.8:443".parse().unwrap();
        assert!(policy.check(&addr).is_ok());
    }

    #[test]
    fn network_policy_blocklist_denies_listed_ip() {
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: std::net::SocketAddr = "1.2.3.4:80".parse().unwrap();
        let err = policy.check(&addr).unwrap_err();
        assert!(err.to_string().contains("network policy denies"), "{err}");
    }

    #[test]
    fn network_policy_blocklist_denies_blocked_ip_on_port53() {
        // RFC5737 TEST-NET-3 — blocklist always wins over DNS exemption.
        let bl = BlockList::from_hosts(&["203.0.113.1"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: std::net::SocketAddr = "203.0.113.1:53".parse().unwrap();
        assert!(
            policy.check(&addr).is_err(),
            "blocked IP must be denied even on port 53"
        );
    }

    #[test]
    fn blocklist_permits_dns_to_non_blocked_ip() {
        let bl = BlockList::from_hosts(&["203.0.113.1"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        let addr: std::net::SocketAddr = "8.8.8.8:53".parse().unwrap();
        assert!(
            policy.check(&addr).is_ok(),
            "BlockList must allow DNS to non-blocked IPs"
        );
    }

    #[test]
    fn blocklist_resolves_hostnames() {
        let bl = BlockList::from_hosts(&["localhost"]).unwrap();
        assert!(
            bl.is_blocked(&"127.0.0.1".parse().unwrap()) || bl.is_blocked(&"::1".parse().unwrap())
        );
    }

    #[test]
    fn blocklist_rejects_unresolvable_hostname() {
        let result = BlockList::from_hosts(&["this.host.definitely.does.not.exist.example"]);
        assert!(result.is_err());
    }

    #[test]
    fn net_link_local_blocked() {
        use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

        // AllowAll still blocks link-local
        let policy = NetworkPolicy::AllowAll;
        let meta = SocketAddr::new(Ipv4Addr::new(169, 254, 169, 254).into(), 80);
        assert!(
            policy.check(&meta).is_err(),
            "AllowAll must block IPv4 link-local"
        );

        let link6 = SocketAddr::new(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1).into(), 80);
        assert!(
            policy.check(&link6).is_err(),
            "AllowAll must block IPv6 link-local"
        );

        // BlockList (even empty) also blocks link-local
        let bl = BlockList::from_hosts(&["192.0.2.1"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        assert!(
            policy.check(&meta).is_err(),
            "BlockList must block IPv4 link-local"
        );
        assert!(
            policy.check(&link6).is_err(),
            "BlockList must block IPv6 link-local"
        );
    }

    #[test]
    fn net_loopback_blocked() {
        use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

        let policy = NetworkPolicy::AllowAll;
        let lo4 = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 80);
        assert!(
            policy.check(&lo4).is_err(),
            "AllowAll must block IPv4 loopback"
        );

        let lo6 = SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 80);
        assert!(
            policy.check(&lo6).is_err(),
            "AllowAll must block IPv6 loopback"
        );

        // BlockList should also block loopback
        let bl = BlockList::from_hosts(&["192.0.2.1"]).unwrap();
        let policy = NetworkPolicy::BlockList(bl);
        assert!(
            policy.check(&lo4).is_err(),
            "BlockList must block IPv4 loopback"
        );
        assert!(
            policy.check(&lo6).is_err(),
            "BlockList must block IPv6 loopback"
        );
    }

    #[test]
    fn net_tools_registered_with_blocklist() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_deadline,
            &sc,
            Some(&NetworkPolicy::BlockList(bl)),
            None,
        );
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"fd\""), "net_socket should work: {s}");
    }

    #[test]
    fn net_tools_not_registered_without_policy() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "net_socket should not exist: {s}");
    }

    #[test]
    fn net_tools_registered_with_allow_all() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_deadline,
            &sc,
            Some(&NetworkPolicy::AllowAll),
            None,
        );
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"fd\""), "net_socket should work: {s}");
    }

    // -- ListenPorts tests -----------------------------------------------------

    #[test]
    fn listen_ports_permits_listed_port() {
        let lp = ListenPorts::from_ports([8080]);
        assert!(lp.check(8080).is_ok());
    }

    #[test]
    fn listen_ports_denies_unlisted_port() {
        let lp = ListenPorts::from_ports([8080]);
        let err = lp.check(9090).unwrap_err();
        assert!(err.to_string().contains("Permission denied"), "{err}");
    }

    #[test]
    fn net_bind_denied_without_listen_ports() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_deadline,
            &sc,
            Some(&NetworkPolicy::AllowAll),
            None,
        );
        // Create a socket first
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        let v: serde_json::Value = serde_json::from_str(s).unwrap();
        let fd = v["result"]["fd"].as_u64().unwrap();
        // Try to bind — should fail because no listen_ports
        let req = format!(
            r#"{{"name":"net_bind","args":{{"fd":{},"addr":"127.0.0.1","port":8080}}}}"#,
            fd
        );
        let resp = tools.dispatch(req.as_bytes());
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "net_bind should be denied: {s}");
        assert!(s.contains("no --port"), "{s}");
    }

    #[test]
    fn net_bind_allowed_with_matching_port() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        let lp = ListenPorts::from_ports([8080]);
        register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_deadline,
            &sc,
            Some(&NetworkPolicy::AllowAll),
            Some(&lp),
        );
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"fd\""), "net_socket should work: {s}");
        let v: serde_json::Value = serde_json::from_slice(&resp).unwrap();
        let fd = v["result"]["fd"].as_u64().unwrap();
        let req =
            format!(r#"{{"name":"net_bind","args":{{"fd":{fd},"addr":"127.0.0.1","port":8080}}}}"#);
        let resp = tools.dispatch(req.as_bytes());
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(!s.contains("\"error\""), "net_bind should succeed: {s}");
    }

    #[test]
    fn net_bind_denied_with_wrong_port() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        let lp = ListenPorts::from_ports([8080]);
        register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_deadline,
            &sc,
            Some(&NetworkPolicy::AllowAll),
            Some(&lp),
        );
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let v: serde_json::Value = serde_json::from_slice(&resp).unwrap();
        let fd = v["result"]["fd"].as_u64().unwrap();
        let req =
            format!(r#"{{"name":"net_bind","args":{{"fd":{fd},"addr":"127.0.0.1","port":9090}}}}"#);
        let resp = tools.dispatch(req.as_bytes());
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "net_bind should be denied: {s}");
        assert!(s.contains("Permission denied"), "{s}");
    }

    // -- Resource-limit tests ---------------------------------------------------

    #[test]
    fn test_fs_read_bytes_capped() {
        let root = tmpdir("readcap");
        fs::write(root.join("small.bin"), b"hello").unwrap();
        let preopens = vec![Preopen::new(&root, "/host").unwrap()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        let req =
            br#"{"name":"fs_read_bytes","args":{"path":"/host/small.bin","len":1099511627776}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(!s.contains("\"error\""), "should succeed: {s}");
        assert!(s.contains("\"bytes_read\":5"), "{s}");
    }

    /// A guest built without `CONFIG_HYPERLIGHT_POLL` (e.g. the pyhl python
    /// driver) issues an *unframed* JSON host call and blocks the vCPU thread
    /// until it returns — it has no drive loop and cannot decode a `PENDING`
    /// frame. Async tools must therefore resolve inline for these callers.
    ///
    /// Regression: async tools once rejected unframed calls outright, which
    /// made `time.sleep()` a silent no-op and broke all guest networking.
    #[test]
    fn legacy_unframed_call_resolves_async_tool_inline() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        // No async frame, exactly as a legacy guest sends it.
        let raw = br#"{"name":"__hl_sleep","args":{"ns":1000000}}"#;
        let start = std::time::Instant::now();
        let resp = tools.dispatch(raw);
        let elapsed = start.elapsed();

        // The reply must be plain JSON the legacy guest can parse, not a
        // binary async-control frame.
        assert!(
            decode_async_frame(&resp).unwrap().is_none(),
            "legacy caller must not receive a binary frame: {:?}",
            String::from_utf8_lossy(&resp)
        );
        let v: serde_json::Value =
            serde_json::from_slice(&resp).expect("legacy reply must be JSON");
        assert!(
            v.get("error").is_none(),
            "unframed async tool call must succeed, got {v}"
        );
        assert!(v.get("result").is_some(), "expected a result, got {v}");
        // It must actually have waited rather than returning immediately.
        assert!(
            elapsed >= Duration::from_micros(900),
            "sleep returned too fast to have run: {elapsed:?}"
        );
    }

    #[test]
    fn test_sleep_capped() {
        assert_eq!(MAX_SLEEP_NS, 60_000_000_000);

        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        let req = framed_request(1, br#"{"name":"__hl_sleep","args":{"ns":0}}"#);
        let resp = tools.dispatch_drive(&req);
        let s = framed_json(&resp, ASYNC_FRAME_RESULT, 1).to_string();
        assert!(!s.contains("\"error\""), "sleep(0) should succeed: {s}");
    }

    #[test]
    fn test_poll_yield_reports_deadline() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        let resp = tools.dispatch(br#"{"name":"__hl_poll_yield","args":{"ns":1234}}"#);
        assert!(!std::str::from_utf8(&resp).unwrap().contains("\"error\""));
        assert_eq!(
            *poll_deadline.lock().unwrap(),
            GuestPollSignal::Yielded {
                deadline_ns: Some(1234)
            }
        );
    }

    #[test]
    fn test_poll_signal_classifies_all_outcomes() {
        assert_eq!(
            classify_poll_signal(GuestPollSignal::None, false),
            PollOutcome::Exited,
            "a bare VM halt is terminal"
        );
        assert_eq!(
            classify_poll_signal(GuestPollSignal::Exited, true),
            PollOutcome::Exited,
            "explicit exit takes precedence over pending host work"
        );
        assert_eq!(
            classify_poll_signal(GuestPollSignal::Yielded { deadline_ns: None }, false),
            PollOutcome::Idle
        );
        assert_eq!(
            classify_poll_signal(
                GuestPollSignal::Yielded {
                    deadline_ns: Some(42)
                },
                false
            ),
            PollOutcome::Timer(Duration::from_nanos(42))
        );
        assert_eq!(
            classify_poll_signal(
                GuestPollSignal::Yielded {
                    deadline_ns: Some(42)
                },
                true
            ),
            PollOutcome::HostCallsPending {
                next_wakeup: Some(Duration::from_nanos(42))
            }
        );
    }

    #[test]
    fn test_poll_yield_preserves_immediate_repoll_deadline() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        tools.dispatch(br#"{"name":"__hl_poll_yield","args":{"ns":1}}"#);
        assert_eq!(
            *poll_deadline.lock().unwrap(),
            GuestPollSignal::Yielded {
                deadline_ns: Some(1)
            },
            "an already-due guest timer must remain distinct from no timer"
        );
    }

    #[test]
    fn test_poll_yield_zero_reports_idle() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        tools.dispatch(br#"{"name":"__hl_poll_yield","args":{"ns":0}}"#);
        assert_eq!(
            *poll_deadline.lock().unwrap(),
            GuestPollSignal::Yielded { deadline_ns: None }
        );
    }

    #[test]
    fn test_exit_survives_late_poll_yield() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        // The app exits first, then the idle pump yields within the same
        // `poll` call. The exit signal must not be lost.
        tools.dispatch(br#"{"name":"__hl_exit","args":{"code":0}}"#);
        tools.dispatch(br#"{"name":"__hl_poll_yield","args":{"ns":0}}"#);

        assert_eq!(*poll_deadline.lock().unwrap(), GuestPollSignal::Exited);
    }

    #[test]
    fn test_poll_yield_rejects_missing_deadline() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_signal = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_signal, &sc, None, None);

        let resp = tools.dispatch(br#"{"name":"__hl_poll_yield","args":{}}"#);
        assert!(std::str::from_utf8(&resp).unwrap().contains("\"error\""));
        assert_eq!(*poll_signal.lock().unwrap(), GuestPollSignal::None);
    }

    #[test]
    fn test_poll_yield_rejects_duplicate_signal() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_signal = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_signal, &sc, None, None);

        tools.dispatch(br#"{"name":"__hl_poll_yield","args":{"ns":0}}"#);
        let resp = tools.dispatch(br#"{"name":"__hl_poll_yield","args":{"ns":1}}"#);
        assert!(std::str::from_utf8(&resp).unwrap().contains("\"error\""));
        assert_eq!(
            *poll_signal.lock().unwrap(),
            GuestPollSignal::Yielded { deadline_ns: None }
        );
    }

    #[test]
    fn test_sleep_cancel_wakes_immediately() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        register_internal_tools(&mut tools, &exit_code, &poll_deadline, &sc, None, None);

        let sc2 = sc.clone();
        let handle = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(200));
            sc2.cancel();
        });

        let start = std::time::Instant::now();
        let req = framed_request(1, br#"{"name":"__hl_sleep","args":{"ns":60000000000}}"#);
        // Drive the now-async sleep future to completion (as the poll loop's
        // drive_host_functions would); the cancel from the other thread should
        // make the blocking sleep return promptly.
        let resp = tools.dispatch_drive(&req);
        let elapsed = start.elapsed();
        handle.join().unwrap();
        sc.reset();

        let s = framed_json(&resp, ASYNC_FRAME_RESULT, 1).to_string();
        assert!(!s.contains("\"error\""), "sleep should succeed: {s}");
        assert!(
            elapsed.as_secs() < 5,
            "cancelled sleep should wake promptly, took {:.1}s",
            elapsed.as_secs_f64()
        );
    }

    #[test]
    fn net_getsockopt_returns_correct_type_for_dgram() {
        let mut reg = ToolRegistry::new();
        let policy = NetworkPolicy::AllowAll;
        let table = Arc::new(Mutex::new(SocketTable::new()));
        register_net_tools(&mut reg, &policy, None, table);

        let req = br#"{"name":"net_socket","args":{"family":2,"type":2}}"#;
        let resp = std::str::from_utf8(&reg.dispatch(req)).unwrap().to_string();
        let v: serde_json::Value = serde_json::from_str(&resp).unwrap();
        let fd = v["result"]["fd"].as_u64().unwrap();

        let req =
            format!(r#"{{"name":"net_getsockopt","args":{{"fd":{fd},"level":1,"optname":3}}}}"#);
        let resp = std::str::from_utf8(&reg.dispatch(req.as_bytes()))
            .unwrap()
            .to_string();
        let v: serde_json::Value = serde_json::from_str(&resp).unwrap();
        assert_eq!(
            v["result"]["value"], 2,
            "SO_TYPE should return 2 (DGRAM), got: {resp}"
        );
    }

    // -- SocketTable lifecycle tests ---------------------------------------------

    #[test]
    fn net_socket_cap() {
        let mut table = SocketTable::new();
        for _ in 0..MAX_SOCKETS {
            let sock = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
            table.insert(HostSocket::new(sock, 1)).unwrap();
        }
        let sock = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        assert!(table.insert(HostSocket::new(sock, 1)).is_err());
    }

    #[test]
    fn socket_table_clear() {
        let mut table = SocketTable::new();
        let sock = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        table.insert(HostSocket::new(sock, 1)).unwrap();
        assert_eq!(table.sockets.len(), 1);
        table.clear();
        assert_eq!(table.sockets.len(), 0);
        assert_eq!(table.next_id, 1);
    }

    #[test]
    fn export_restore_listeners_roundtrip() {
        // Bind + listen a real socket, record it as a listener the way
        // handle_net_bind/handle_net_listen now do, then export, clear, and
        // restore — verifying the listener comes back under its original fd.
        let mut table = SocketTable::new();
        let sock = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        sock.set_reuse_address(true).unwrap();
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        sock.bind(&addr.into()).unwrap();
        sock.listen(128).unwrap();
        let local = sock.local_addr().unwrap().as_socket().unwrap();
        let fd = table.insert(HostSocket::new(sock, 1)).unwrap();
        {
            let hs = table.get_mut(fd).unwrap();
            hs.bound_addr = Some(local);
            hs.listen_backlog = Some(128);
        }

        let exported = table.export_listeners();
        assert_eq!(exported["listeners"].as_array().unwrap().len(), 1);
        let saved_next = table.next_id;

        // Simulate teardown/restore: the guest keeps using `fd`, but the
        // host table starts empty.
        table.clear();
        assert!(table.get(fd).is_err());

        table.restore_listeners(&exported).unwrap();

        let hs = table.get(fd).expect("listener fd must be re-created");
        assert_eq!(hs.listen_backlog, Some(128));
        assert_eq!(hs.bound_addr.map(|a| a.port()), Some(local.port()));
        assert_eq!(table.next_id, saved_next, "next_id must be preserved");
    }

    #[test]
    fn test_fs_read_size_cap() {
        use std::io::Write;

        let root = tmpdir("fs_read_cap");
        // Create a file that exceeds MAX_FS_READ (16 MiB) using a sparse file
        let big_path = root.join("big.txt");
        let f = std::fs::File::create(&big_path).unwrap();
        f.set_len(MAX_FS_READ + 1).unwrap();
        drop(f);

        // Also create a small file to verify normal reads work
        let small_path = root.join("small.txt");
        let mut sf = std::fs::File::create(&small_path).unwrap();
        sf.write_all(b"hello").unwrap();
        drop(sf);

        let preopens = vec![Preopen::new(&root, "/host").unwrap()];
        let mut reg = ToolRegistry::new();
        FsRouter::new(&preopens).unwrap().register(&mut reg);

        // Small file should succeed
        let req = br#"{"name":"fs_read","args":{"path":"/host/small.txt"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(!s.contains("\"error\""), "small read should succeed: {s}");

        // Large file should fail with "too large"
        let req = br#"{"name":"fs_read","args":{"path":"/host/big.txt"}}"#;
        let resp = reg.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(
            s.contains("too large"),
            "expected 'too large' error, got: {s}"
        );
    }

    #[test]
    fn test_net_send_size_cap() {
        use base64::Engine;

        let mut reg = ToolRegistry::new();
        let policy = NetworkPolicy::AllowAll;
        let table = Arc::new(Mutex::new(SocketTable::new()));
        register_net_tools(&mut reg, &policy, None, table);

        // Create a socket
        let req = br#"{"name":"net_socket","args":{"family":2,"type":2}}"#;
        let resp = std::str::from_utf8(&reg.dispatch(req)).unwrap().to_string();
        let v: serde_json::Value = serde_json::from_str(&resp).unwrap();
        let fd = v["result"]["fd"].as_u64().unwrap();

        // Create payload larger than MAX_NET_SEND (1 MiB)
        let big_payload = vec![0u8; MAX_NET_SEND + 1];
        let b64 = base64::engine::general_purpose::STANDARD.encode(&big_payload);
        let json = format!(r#"{{"name":"net_send","args":{{"fd":{fd},"data":"{b64}"}}}}"#);
        let req = framed_request(1, json.as_bytes());
        let raw = reg.dispatch_drive(&req);
        let resp = framed_json(&raw, ASYNC_FRAME_RESULT, 1).to_string();
        assert!(
            resp.contains("too large"),
            "expected 'too large' error for net_send, got: {resp}"
        );

        let json = format!(
            r#"{{"name":"net_sendto","args":{{"fd":{fd},"data":"{b64}","addr":"127.0.0.1","port":9999}}}}"#
        );
        let req = framed_request(2, json.as_bytes());
        let raw = reg.dispatch_drive(&req);
        let resp = framed_json(&raw, ASYNC_FRAME_RESULT, 2).to_string();
        assert!(
            resp.contains("too large"),
            "expected 'too large' error for net_sendto, got: {resp}"
        );
    }

    /// `::ffff:a.b.c.d` reaches the IPv4 stack, so every policy rule has to see
    /// through it. Before this was normalised a guest could reach host loopback
    /// services and cloud instance metadata by simply rewriting the destination
    /// in mapped form (verified end-to-end: the payload arrived at a real
    /// 127.0.0.1 listener).
    #[test]
    fn ipv4_mapped_ipv6_cannot_evade_policy() {
        let denied = |p: &NetworkPolicy, s: &str| {
            let addr: std::net::SocketAddr = s.parse().unwrap();
            p.check(&addr).is_err()
        };

        for policy in [
            NetworkPolicy::AllowAll,
            NetworkPolicy::BlockList(BlockList::from_hosts(&["198.51.100.7"]).unwrap()),
        ] {
            assert!(
                denied(&policy, "[::ffff:127.0.0.1]:80"),
                "mapped loopback must be denied"
            );
            assert!(
                denied(&policy, "[::ffff:127.0.0.53]:53"),
                "all of 127.0.0.0/8 must be denied in mapped form"
            );
            assert!(
                denied(&policy, "[::ffff:169.254.169.254]:80"),
                "mapped link-local (instance metadata) must be denied"
            );
            // The plain forms were already denied; keep them that way.
            assert!(denied(&policy, "127.0.0.1:80"));
            assert!(denied(&policy, "[::1]:80"));
            assert!(denied(&policy, "169.254.169.254:80"));
        }

        // A blocklisted host must stay blocked when written in mapped form.
        let bl = NetworkPolicy::BlockList(BlockList::from_hosts(&["198.51.100.7"]).unwrap());
        assert!(denied(&bl, "198.51.100.7:443"));
        assert!(
            denied(&bl, "[::ffff:198.51.100.7]:443"),
            "blocklist must see through IPv4-mapped IPv6"
        );

        // ...and an allowlisted host must still be reachable in mapped form,
        // so normalising does not break legitimate traffic.
        let al = NetworkPolicy::AllowList(AllowList::from_hosts(&["198.51.100.9"]).unwrap());
        assert!(!denied(&al, "198.51.100.9:443"));
        assert!(!denied(&al, "[::ffff:198.51.100.9]:443"));
        assert!(denied(&al, "[::ffff:198.51.100.10]:443"));

        // The deprecated IPv4-compatible form is NOT routed as IPv4 by the
        // kernel, and unwrapping it would misclassify `::1`, so it must keep
        // being treated as a plain IPv6 address.
        let allow = NetworkPolicy::AllowAll;
        assert!(denied(&allow, "[::1]:80"), "::1 must remain loopback");
    }

    /// The `AI_ADDRCONFIG` probe (a UDP connect to loopback) must be answered,
    /// or musl turns it into `EAI_SYSTEM` and every default-hints
    /// `getaddrinfo` fails. Answering it must NOT open a path to loopback:
    /// TCP connect and datagram delivery there stay denied.
    #[test]
    fn addrconfig_probe_answered_but_loopback_stays_denied() {
        use base64::Engine;

        let mut reg = ToolRegistry::new();
        let table = Arc::new(Mutex::new(SocketTable::new()));
        register_net_tools(&mut reg, &NetworkPolicy::AllowAll, None, table);

        let new_sock = |reg: &mut ToolRegistry, stype: u8| -> u64 {
            let json = format!(r#"{{"name":"net_socket","args":{{"family":2,"type":{stype}}}}}"#);
            let resp = String::from_utf8(reg.dispatch(json.as_bytes()).to_vec()).unwrap();
            serde_json::from_str::<serde_json::Value>(&resp).unwrap()["result"]["fd"]
                .as_u64()
                .unwrap()
        };
        let connect = |reg: &mut ToolRegistry, fd: u64, id: u64, ip: &str, port: u16| -> String {
            let json = format!(
                r#"{{"name":"net_connect","args":{{"fd":{fd},"addr":"{ip}","port":{port}}}}}"#
            );
            let raw = reg.dispatch_drive(&framed_request(id, json.as_bytes()));
            framed_json(&raw, ASYNC_FRAME_RESULT, id).to_string()
        };

        // The probe itself: UDP connect to loopback:65535 is answered, so musl
        // sees the family as configured rather than erroring out.
        let udp = new_sock(&mut reg, 2);
        let resp = connect(&mut reg, udp, 1, "127.0.0.1", 65535);
        assert!(
            !resp.contains("error"),
            "AI_ADDRCONFIG probe must be answered, got: {resp}"
        );

        // ...but the probed socket still cannot deliver a datagram there.
        let b64 = base64::engine::general_purpose::STANDARD.encode(b"payload");
        let json = format!(
            r#"{{"name":"net_sendto","args":{{"fd":{udp},"data":"{b64}","addr":"127.0.0.1","port":65535}}}}"#
        );
        let raw = reg.dispatch_drive(&framed_request(2, json.as_bytes()));
        let resp = framed_json(&raw, ASYNC_FRAME_RESULT, 2).to_string();
        assert!(
            resp.contains("loopback"),
            "sendto to loopback must stay denied, got: {resp}"
        );

        // ...and a real (TCP) connection to loopback is still refused.
        let tcp = new_sock(&mut reg, 1);
        let resp = connect(&mut reg, tcp, 3, "127.0.0.1", 65535);
        assert!(
            resp.contains("loopback"),
            "TCP connect to loopback must stay denied, got: {resp}"
        );

        // The exception is only the probe's exact shape: a UDP connect to any
        // other loopback port is still an honest denial.
        let udp2 = new_sock(&mut reg, 2);
        let resp = connect(&mut reg, udp2, 4, "127.0.0.1", 53);
        assert!(
            resp.contains("loopback"),
            "UDP connect to loopback:53 must stay denied, got: {resp}"
        );
    }

    #[test]
    fn allowlist_learned_ips_capped() {
        use std::net::{IpAddr, Ipv4Addr};
        let al = AllowList::from_hosts(&["192.0.2.1"]).unwrap();
        // Fill up to the cap
        for i in 0..MAX_LEARNED_IPS {
            let ip = IpAddr::V4(Ipv4Addr::new(10, 0, (i / 256) as u8, (i % 256) as u8));
            al.learn_ip(ip);
        }
        assert_eq!(al.learned_ips.lock().unwrap().len(), MAX_LEARNED_IPS);
        // One more should NOT be added
        al.learn_ip(IpAddr::V4(Ipv4Addr::new(10, 1, 0, 1)));
        assert_eq!(al.learned_ips.lock().unwrap().len(), MAX_LEARNED_IPS);
    }

    #[test]
    fn net_socket_has_default_timeout() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let poll_deadline = Arc::new(Mutex::new(GuestPollSignal::None));
        let sc = SleepCancel::new();
        let table = register_internal_tools(
            &mut tools,
            &exit_code,
            &poll_deadline,
            &sc,
            Some(&NetworkPolicy::AllowAll),
            None,
        )
        .expect("network tools should be registered");

        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(!s.contains("error"), "socket creation should succeed: {s}");

        let v: serde_json::Value = serde_json::from_str(s).unwrap();
        let fd = v["result"]["fd"].as_u64().unwrap();
        assert!(fd > 0);

        let tbl = table.lock().unwrap();
        let sock = tbl.get_socket(fd).unwrap();
        assert_eq!(sock.read_timeout().unwrap(), Some(SOCKET_TIMEOUT));
        assert_eq!(sock.write_timeout().unwrap(), Some(SOCKET_TIMEOUT));
    }

    #[test]
    fn dns_read_name_rejects_circular_pointer() {
        // Two compression pointers pointing at each other: offset 0 → 2, offset 2 → 0
        let data = [0xC0, 0x02, 0xC0, 0x00];
        let mut pos = 0usize;
        assert!(
            dns_read_name(&data, &mut pos).is_none(),
            "circular DNS compression pointers should be rejected"
        );
    }
}
