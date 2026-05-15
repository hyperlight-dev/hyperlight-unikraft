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
//!   `pyhl run` starts in ~200ms without re-doing `Py_Initialize`.
//!
//! # Host filesystem
//!
//! The guest can access host directories via [`Preopen`] + the
//! `__dispatch` RPC. [`FsSandbox`] rejects path-escape attempts and
//! `normalize_fs_error` rewrites host-OS-specific error wording so
//! the cross-platform Unikraft guest classifies errors uniformly.

pub mod pyhl;
pub mod stderr_capture;

use anyhow::{anyhow, Result};
use hyperlight_host::func::Registerable;
use hyperlight_host::sandbox::snapshot::Snapshot;
use hyperlight_host::sandbox::uninitialized::GuestEnvironment;
use hyperlight_host::sandbox::SandboxConfiguration;
use hyperlight_host::{GuestBinary, HostFunctions, MultiUseSandbox, UninitializedSandbox};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::time::Duration;

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

/// Cap for `__hl_sleep` duration to prevent unbounded host-thread blocking (60 s).
const MAX_SLEEP_NS: u64 = 60_000_000_000;

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
        })
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
        })
    }

    fn is_allowed(&self, ip: &IpAddr) -> bool {
        if self.allowed_ips.contains(ip) {
            return true;
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

fn dns_resolvers() -> &'static HashSet<IpAddr> {
    static RESOLVERS: std::sync::OnceLock<HashSet<IpAddr>> = std::sync::OnceLock::new();
    RESOLVERS.get_or_init(|| {
        #[cfg(unix)]
        {
            let mut set = HashSet::new();
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
            set
        }
        #[cfg(not(unix))]
        {
            HashSet::new()
        }
    })
}

impl NetworkPolicy {
    fn check(&self, addr: &std::net::SocketAddr) -> Result<()> {
        match self {
            NetworkPolicy::AllowAll => Ok(()),
            NetworkPolicy::AllowList(al) => {
                if al.is_allowed(&addr.ip())
                    || (addr.port() == 53 && dns_resolvers().contains(&addr.ip()))
                {
                    Ok(())
                } else {
                    Err(anyhow!("network policy denies connection to {}", addr))
                }
            }
            NetworkPolicy::BlockList(bl) => {
                if bl.is_blocked(&addr.ip()) {
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
pub struct VmConfig {
    pub heap_size: u64,
    pub stack_size: u64,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            heap_size: 512 * 1024 * 1024,
            stack_size: 8 * 1024 * 1024,
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

    fn sandbox_config(&self) -> SandboxConfiguration {
        let mut cfg = SandboxConfiguration::default();
        cfg.set_heap_size(self.heap_size);

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
}

impl ToolRegistry {
    /// Create an empty registry. Add handlers with
    /// [`register`](Self::register) before wiring it into a sandbox.
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
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

    /// Decode a guest-side `__dispatch` request, look up the handler by
    /// name, invoke it, and encode the response as JSON bytes.
    ///
    /// The request shape is `{"name": "...", "args": <value>}`, and the
    /// response is either `{"result": <value>}` or `{"error": "<msg>"}`.
    /// Unknown tool names and JSON errors both become error responses;
    /// this function never panics.
    ///
    /// Set `HL_DISPATCH_DEBUG=1` in the environment to dump each call's
    /// payload and result to stderr — useful when diagnosing
    /// guest/host protocol mismatches.
    pub fn dispatch(&self, payload: &[u8]) -> Vec<u8> {
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
        let result = (|| -> Result<serde_json::Value> {
            let req: serde_json::Value = serde_json::from_slice(payload)?;
            let name = req["name"]
                .as_str()
                .ok_or_else(|| anyhow!("missing 'name'"))?;
            let args = req.get("args").cloned().unwrap_or(serde_json::Value::Null);
            let handler = self
                .tools
                .get(name)
                .ok_or_else(|| anyhow!("unknown tool: {}", name))?;
            handler(args)
        })();
        if debug {
            match &result {
                Ok(v) => eprintln!("[__dispatch] OK: {}", v),
                Err(e) => eprintln!("[__dispatch] ERR: {}", e),
            }
        }
        let json = match result {
            Ok(v) => serde_json::json!({ "result": v }),
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
        serde_json::to_vec(&json)
            .unwrap_or_else(|_| b"{\"error\":\"serialization failed\"}".to_vec())
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
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
    network: Option<&NetworkPolicy>,
    listen_ports: Option<&ListenPorts>,
) {
    let ec = exit_code.clone();
    tools.register("__hl_exit", move |args| {
        let code = args["code"].as_i64().unwrap_or(1) as i32;
        ec.store(code, Ordering::Relaxed);
        Ok(serde_json::json!({}))
    });
    tools.register("__hl_sleep", |args| {
        let ns = args["ns"].as_u64().unwrap_or(0).min(MAX_SLEEP_NS);
        if ns > 0 {
            std::thread::sleep(std::time::Duration::from_nanos(ns));
        }
        Ok(serde_json::json!({}))
    });
    if let Some(policy) = network {
        register_net_tools(tools, policy, listen_ports);
    }
}

// ---------------------------------------------------------------------------
// Host-proxied networking (hostsock)
// ---------------------------------------------------------------------------

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::SocketAddr;
use std::sync::Mutex;

enum HostSocket {
    Socket(Socket, i32),
}

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

    fn insert(&mut self, sock: HostSocket) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.sockets.insert(id, sock);
        id
    }

    fn get(&self, fd: u64) -> Result<&HostSocket> {
        self.sockets
            .get(&fd)
            .ok_or_else(|| anyhow!("bad_fd: {}", fd))
    }

    fn get_socket(&self, fd: u64) -> Result<&Socket> {
        match self.get(fd)? {
            HostSocket::Socket(s, _) => Ok(s),
        }
    }

    fn get_sock_type(&self, fd: u64) -> Result<i32> {
        match self.get(fd)? {
            HostSocket::Socket(_, t) => Ok(*t),
        }
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

fn register_net_tools(
    tools: &mut ToolRegistry,
    policy: &NetworkPolicy,
    listen_ports: Option<&ListenPorts>,
) {
    use base64::Engine;
    use serde_json::json;

    let table = Arc::new(Mutex::new(SocketTable::new()));
    let policy = Arc::new(policy.clone());

    // net_socket
    let t = table.clone();
    tools.register("net_socket", move |args| {
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
        let fd = t
            .lock()
            .unwrap()
            .insert(HostSocket::Socket(sock, sock_type));
        Ok(json!({ "fd": fd }))
    });

    // net_connect
    let t = table.clone();
    let pol = policy.clone();
    tools.register("net_connect", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let addr = parse_sockaddr(&args)?;
        pol.check(&addr)?;
        let sa: SockAddr = addr.into();
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        sock.connect(&sa)?;
        Ok(json!({}))
    });

    // net_bind — gated by listen-port allowlist
    let t = table.clone();
    let lp = listen_ports.cloned().map(Arc::new);
    tools.register("net_bind", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let addr = parse_sockaddr(&args)?;
        match lp.as_ref() {
            Some(ports) => ports.check(addr.port())?,
            None => return Err(anyhow!("Permission denied: no --port specified for bind")),
        }
        let sa: SockAddr = addr.into();
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        sock.bind(&sa)?;
        Ok(json!({}))
    });

    // net_listen
    let t = table.clone();
    tools.register("net_listen", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let backlog = args["backlog"].as_i64().unwrap_or(128) as i32;
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        sock.listen(backlog)?;
        Ok(json!({}))
    });

    // net_accept
    let t = table.clone();
    tools.register("net_accept", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let (new_sock, peer, parent_type) = {
            let tbl = t.lock().unwrap();
            let sock = tbl.get_socket(fd)?;
            let (s, p) = sock.accept()?;
            let st = tbl.get_sock_type(fd)?;
            (s, p, st)
        };
        let peer_addr: Option<SocketAddr> = peer.as_socket();
        let new_fd = t
            .lock()
            .unwrap()
            .insert(HostSocket::Socket(new_sock, parent_type));
        let mut resp = json!({ "fd": new_fd });
        if let Some(pa) = peer_addr {
            resp["addr"] = json!(pa.ip().to_string());
            resp["port"] = json!(pa.port());
        }
        Ok(resp)
    });

    // net_send
    let t = table.clone();
    tools.register("net_send", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let data_b64 = args["data"]
            .as_str()
            .ok_or_else(|| anyhow!("missing 'data'"))?;
        let data = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .map_err(|e| anyhow!("base64 decode: {}", e))?;
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        let sent = sock.send(&data)?;
        Ok(json!({ "sent": sent }))
    });

    // net_sendto
    let t = table.clone();
    let pol = policy.clone();
    tools.register("net_sendto", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let data_b64 = args["data"]
            .as_str()
            .ok_or_else(|| anyhow!("missing 'data'"))?;
        let data = base64::engine::general_purpose::STANDARD
            .decode(data_b64)
            .map_err(|e| anyhow!("base64 decode: {}", e))?;
        let addr = parse_sockaddr(&args)?;
        pol.check(&addr)?;
        let sa: SockAddr = addr.into();
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        let sent = sock.send_to(&data, &sa)?;
        Ok(json!({ "sent": sent }))
    });

    // net_recv (alias for net_recvfrom with no addr returned for stream)
    let t = table.clone();
    tools.register("net_recv", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let len = args["len"].as_u64().unwrap_or(4096) as usize;
        let mut buf = vec![std::mem::MaybeUninit::uninit(); len.min(65536)];
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        let n = sock.recv(&mut buf)?;
        let data: Vec<u8> = buf[..n]
            .iter()
            .map(|b| unsafe { b.assume_init() })
            .collect();
        let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
        Ok(json!({ "data": encoded, "len": n }))
    });

    // net_recvfrom
    let t = table.clone();
    tools.register("net_recvfrom", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let len = args["len"].as_u64().unwrap_or(4096) as usize;
        let mut buf = vec![0u8; len.min(65536)];

        let buf_init =
            unsafe { &mut *(buf.as_mut_slice() as *mut [u8] as *mut [std::mem::MaybeUninit<u8>]) };

        let (n, peer) = {
            let tbl = t.lock().unwrap();
            let sock = tbl.get_socket(fd)?;
            sock.recv_from(buf_init)?
        };
        buf.truncate(n);
        let encoded = base64::engine::general_purpose::STANDARD.encode(&buf);
        let mut resp = json!({ "data": encoded, "len": n });
        if let Some(pa) = peer.as_socket() {
            resp["addr"] = json!(pa.ip().to_string());
            resp["port"] = json!(pa.port());
        }
        Ok(resp)
    });

    // net_close
    let t = table.clone();
    tools.register("net_close", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        t.lock().unwrap().remove(fd)?;
        Ok(json!({}))
    });

    // net_shutdown
    let t = table.clone();
    tools.register("net_shutdown", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let how = args["how"].as_i64().unwrap_or(2) as i32;
        let shutdown = match how {
            0 => std::net::Shutdown::Read,
            1 => std::net::Shutdown::Write,
            _ => std::net::Shutdown::Both,
        };
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        sock.shutdown(shutdown)?;
        Ok(json!({}))
    });

    // net_setsockopt
    let t = table.clone();
    tools.register("net_setsockopt", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let level = args["level"].as_i64().unwrap_or(0) as i32;
        let optname = args["optname"].as_i64().unwrap_or(0) as i32;
        let value = args["value"].as_i64().unwrap_or(0) as i32;
        let tbl = t.lock().unwrap();
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
    });

    // net_getsockopt
    let t = table.clone();
    tools.register("net_getsockopt", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let level = args["level"].as_i64().unwrap_or(0) as i32;
        let optname = args["optname"].as_i64().unwrap_or(0) as i32;
        let tbl = t.lock().unwrap();
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
    });

    // net_getpeername
    let t = table.clone();
    tools.register("net_getpeername", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        let peer = sock.peer_addr()?;
        if let Some(addr) = peer.as_socket() {
            Ok(sockaddr_to_json(addr))
        } else {
            Ok(json!({ "addr": "0.0.0.0", "port": 0 }))
        }
    });

    // net_getsockname
    let t = table.clone();
    tools.register("net_getsockname", move |args| {
        let fd = args["fd"].as_u64().ok_or_else(|| anyhow!("missing 'fd'"))?;
        let tbl = t.lock().unwrap();
        let sock = tbl.get_socket(fd)?;
        let local = sock.local_addr()?;
        if let Some(addr) = local.as_socket() {
            Ok(sockaddr_to_json(addr))
        } else {
            Ok(json!({ "addr": "0.0.0.0", "port": 0 }))
        }
    });
}

/// Routes incoming fs_* tool calls to the matching `FsSandbox` by
/// matching the guest-supplied path against each preopen's guest path.
#[derive(Clone)]
struct FsRouter {
    entries: Vec<(String, FsSandbox)>,
}

impl FsRouter {
    fn new(preopens: &[Preopen]) -> Result<Self> {
        let mut entries = Vec::with_capacity(preopens.len());
        for p in preopens {
            entries.push((p.guest_path.clone(), FsSandbox::new(&p.host_dir)?));
        }
        // Sort by descending prefix length so longer matches win (e.g.
        // /data/public should match before /data).
        entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        Ok(Self { entries })
    }

    /// Pick the preopen matching `path` and return its sandbox plus
    /// the path-relative-to-that-sandbox.
    fn route<'a>(&'a self, path: &'a str) -> Result<(&'a FsSandbox, &'a str)> {
        for (prefix, fs) in &self.entries {
            if path == prefix {
                return Ok((fs, ""));
            }
            if let Some(tail) = path.strip_prefix(prefix).and_then(|t| t.strip_prefix('/')) {
                return Ok((fs, tail));
            }
        }
        Err(anyhow!(
            "path {:?} does not match any preopened mount",
            path
        ))
    }

    fn register(self, registry: &mut ToolRegistry) {
        use serde_json::json;

        let r = self.clone();
        registry.register("fs_read", move |args| {
            let path = args["path"]
                .as_str()
                .ok_or_else(|| anyhow!("fs_read: missing 'path'"))?;
            let (fs, rel) = r.route(path)?;
            let target = fs.resolve(rel)?;
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
            let append = args["append"].as_bool().unwrap_or(false);
            let (fs, rel) = r.route(path)?;
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
            let (fs, rel) = r.route(path)?;
            let target = fs.resolve(rel)?;
            let mut entries = Vec::new();
            for entry in
                std::fs::read_dir(&target).map_err(|e| anyhow!("fs_list {:?}: {}", path, e))?
            {
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
            let (fs, rel) = r.route(path)?;
            let target = fs.resolve(rel)?;
            let md =
                std::fs::metadata(&target).map_err(|e| anyhow!("fs_stat {:?}: {}", path, e))?;
            Ok(json!({
                "size": md.len(),
                "is_dir": md.is_dir(),
                "is_file": md.is_file(),
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
            let (fs, rel) = r.route(path)?;
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
            let data = base64::engine::general_purpose::STANDARD
                .decode(data_b64)
                .map_err(|e| anyhow!("fs_write_bytes: bad base64: {}", e))?;
            let offset = args["offset"].as_u64();
            let append = args["append"].as_bool().unwrap_or(false);
            let (fs, rel) = r.route(path)?;
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
            let (fs, rel) = r.route(path)?;
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
            let (fs, rel) = r.route(path)?;
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
            let (fs, rel) = r.route(path)?;
            let target = fs.resolve(rel)?;
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
    /// File mapping to re-register after snapshot restore.
    /// Snapshot restore unmaps all non-snapshot regions.
    file_mapping_path: Option<std::path::PathBuf>,
    file_mapping_base: u64,
    exit_code: Arc<AtomicI32>,
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
    preopens: Vec<Preopen>,
    network: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    tools: ToolRegistry,
    has_tools: bool,
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

    /// Boot the VM, run init, and take a post-init snapshot.
    pub fn build(self) -> Result<Sandbox> {
        let config = VmConfig {
            heap_size: self.heap_size.unwrap_or(512 * 1024 * 1024),
            stack_size: self.stack_size.unwrap_or(8 * 1024 * 1024),
        };
        let tools = if self.has_tools {
            Some(self.tools)
        } else {
            None
        };
        let net = self.network.as_ref();
        let lp = self.listen_ports.as_ref();
        match self.initrd {
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
        }
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
            preopens: Vec::new(),
            network: None,
            listen_ports: None,
            tools: ToolRegistry::new(),
            has_tools: false,
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
        let mut tools = build_tools(tools, preopens)?.unwrap_or_default();
        register_internal_tools(&mut tools, &exit_code, network, listen_ports);
        let tools = Arc::new(tools);
        let tools_ref = tools.clone();
        usbox.register_host_function("__dispatch", move |payload: Vec<u8>| -> Vec<u8> {
            tools_ref.dispatch(&payload)
        })?;

        Self::finish_evolve(usbox, None, 0, exit_code)
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
        // Place at 3 GiB — high enough to not overlap any reasonable
        // primary shared memory region, within the 4 GiB identity map.
        const INITRD_MAP_BASE: u64 = 0xC000_0000; // 3 GiB
        if let Some(path) = initrd_path {
            usbox.map_file_cow(path, INITRD_MAP_BASE, Some("initrd"))?;
        }

        let exit_code = Arc::new(AtomicI32::new(0));
        let mut tools = build_tools(tools, preopens)?.unwrap_or_default();
        register_internal_tools(&mut tools, &exit_code, network, listen_ports);
        let tools = Arc::new(tools);
        let tools_ref = tools.clone();
        usbox.register_host_function("__dispatch", move |payload: Vec<u8>| -> Vec<u8> {
            tools_ref.dispatch(&payload)
        })?;

        Self::finish_evolve(
            usbox,
            initrd_path.map(|p| p.to_path_buf()),
            INITRD_MAP_BASE,
            exit_code,
        )
    }

    fn finish_evolve(
        usbox: UninitializedSandbox,
        file_mapping_path: Option<std::path::PathBuf>,
        file_mapping_base: u64,
        exit_code: Arc<AtomicI32>,
    ) -> Result<Self> {
        let mut inner = usbox.evolve()?;
        let snapshot = inner.snapshot().ok();
        Ok(Self {
            inner,
            snapshot,
            file_mapping_path,
            file_mapping_base,
            exit_code,
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
        // Re-register file mapping after restore (snapshot restore
        // unmaps all non-snapshot regions including file mappings)
        if let Some(ref path) = self.file_mapping_path {
            self.inner
                .map_file_cow(path, self.file_mapping_base, Some("initrd"))?;
        }
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
        let _: () = self.inner.call("run", ())?;
        Ok(())
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
        Ok(self.inner.call(func_name, args)?)
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

    /// Persist the current snapshot to disk as a sparse file.
    ///
    /// After writing the raw HLS snapshot, zero-filled 4 KiB pages are
    /// punched out with `fallocate(PUNCH_HOLE)` so they consume no disk
    /// space. The file is still mmap-loadable — holes read back as
    /// zeros with no decompression overhead.
    pub fn save_snapshot<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let snap = self
            .snapshot
            .as_ref()
            .ok_or_else(|| anyhow!("no snapshot present; build() or snapshot_now() first"))?;
        snap.to_file(path.as_ref())?;
        sparsify_snapshot(path.as_ref())?;
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
    /// Uses `Snapshot::from_file_unchecked`, which skips the SHA-256
    /// verification over the file. We trust snapshots written by our
    /// own `save_snapshot()` earlier in the same process family (the
    /// pyhl install dir), and the hash verify alone costs ~500ms on
    /// a 2.5 GB snapshot — enough to double the whole `pyhl run` wall
    /// time on simple scripts.
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
    /// guest VA (0xC000_0000). Required when the snapshot was taken
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
        let loaded = Snapshot::from_file_unchecked(path.as_ref())?;
        let arc = Arc::new(loaded);

        let exit_code = Arc::new(AtomicI32::new(0));
        let mut tools = build_tools(None, preopens)?.unwrap_or_default();
        register_internal_tools(&mut tools, &exit_code, network, listen_ports);
        let tools = Arc::new(tools);
        let tools_ref = tools.clone();

        let mut host_funcs = HostFunctions::default();
        host_funcs.register_host_function("__dispatch", move |payload: Vec<u8>| -> Vec<u8> {
            tools_ref.dispatch(&payload)
        })?;

        let mut inner = MultiUseSandbox::from_snapshot(arc.clone(), host_funcs, None)?;

        const INITRD_MAP_BASE: u64 = 0xC000_0000;
        if let Some(ref initrd_path) = initrd {
            inner.map_file_cow(initrd_path, INITRD_MAP_BASE, Some("initrd"))?;
        }

        Ok(Self {
            inner,
            snapshot: Some(arc),
            file_mapping_path: initrd,
            file_mapping_base: INITRD_MAP_BASE,
            exit_code,
        })
    }
}

// ---------------------------------------------------------------------------
// Convenience: run_vm (single-shot execution)
// ---------------------------------------------------------------------------

/// Run a Unikraft kernel to completion (single-shot). Thin shim over
/// [`Sandbox::builder`] for callers that don't need the full fluent API.
pub fn run_vm(
    kernel_path: &Path,
    initrd: Option<&[u8]>,
    app_args: &[String],
    config: VmConfig,
) -> Result<()> {
    let _ = Sandbox::evolve_inline(kernel_path, initrd, app_args, config, None, &[], None, None)?;
    Ok(())
}

/// Run a Unikraft kernel with tool dispatch support.
pub fn run_vm_with_tools(
    kernel_path: &Path,
    initrd: Option<&[u8]>,
    app_args: &[String],
    config: VmConfig,
    tools: ToolRegistry,
) -> Result<()> {
    let _ = Sandbox::evolve_inline(
        kernel_path,
        initrd,
        app_args,
        config,
        Some(tools),
        &[],
        None,
        None,
    )?;
    Ok(())
}

/// Run a Unikraft kernel with preopened host directories exposed via
/// `lib/hostfs`. Sandbox escape attempts are rejected host-side.
pub fn run_vm_with_preopens(
    kernel_path: &Path,
    initrd: Option<&[u8]>,
    app_args: &[String],
    config: VmConfig,
    preopens: &[Preopen],
) -> Result<()> {
    let _ = Sandbox::evolve_inline(
        kernel_path,
        initrd,
        app_args,
        config,
        None,
        preopens,
        None,
        None,
    )?;
    Ok(())
}

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
// Snapshot sparsification
// ---------------------------------------------------------------------------

/// Punch holes in zero-filled 4 KiB pages of a snapshot file.
///
/// The HLS snapshot format is a 4 KiB header followed by a dense memory
/// blob where ~80 % of pages are all-zeros (unused heap). Punching them
/// with `fallocate(PUNCH_HOLE)` turns the file sparse — the zeros still
/// read back via mmap but consume no disk blocks.
#[cfg(target_os = "linux")]
fn sparsify_snapshot(path: &Path) -> Result<()> {
    use std::os::unix::io::AsRawFd;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let len = file.metadata()?.len();
    let mmap = unsafe { memmap2::Mmap::map(&file)? };

    const PAGE: usize = 4096;
    const HEADER: usize = PAGE;
    let zero_page = [0u8; PAGE];

    let mut punched = 0u64;
    let mut offset = HEADER;
    while offset + PAGE <= len as usize {
        if mmap[offset..offset + PAGE] == zero_page {
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
        offset += PAGE;
    }
    drop(mmap);

    if punched > 0 {
        let disk_mib = (len - punched * PAGE as u64) / 1024 / 1024;
        eprintln!("  sparsified: {disk_mib} MiB on disk (punched {punched} zero pages)",);
    }

    Ok(())
}

/// Windows equivalent: mark the file sparse with FSCTL_SET_SPARSE, then
/// punch zero ranges with FSCTL_SET_ZERO_DATA.
#[cfg(target_os = "windows")]
fn sparsify_snapshot(path: &Path) -> Result<()> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::System::Ioctl::{FSCTL_SET_SPARSE, FSCTL_SET_ZERO_DATA};
    use windows_sys::Win32::System::IO::DeviceIoControl;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let len = file.metadata()?.len();
    let handle = file.as_raw_handle();

    // Mark file as sparse.
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

    let mmap = unsafe { memmap2::Mmap::map(&file)? };

    const PAGE: usize = 4096;
    const HEADER: usize = PAGE;
    let zero_page = [0u8; PAGE];

    // Coalesce contiguous zero pages into ranges for fewer syscalls.
    let mut punched = 0u64;
    let mut offset = HEADER;
    while offset + PAGE <= len as usize {
        if mmap[offset..offset + PAGE] != zero_page {
            offset += PAGE;
            continue;
        }
        let range_start = offset;
        while offset + PAGE <= len as usize && mmap[offset..offset + PAGE] == zero_page {
            offset += PAGE;
        }
        let range_end = offset;

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
            punched += (range_end - range_start) as u64 / PAGE as u64;
        }
    }
    drop(mmap);

    if punched > 0 {
        let disk_mib = (len - punched * PAGE as u64) / 1024 / 1024;
        eprintln!("  sparsified: {disk_mib} MiB on disk (punched {punched} zero pages)",);
    }

    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn sparsify_snapshot(_path: &Path) -> Result<()> {
    Ok(())
}

// ---------------------------------------------------------------------------
// FsSandbox tests — prove that host-side path resolution rejects escapes.
//
// These cover both attack vectors the host can see: lexical ".." /
// absolute paths passed in an RPC arg, and symlinks inside the mount
// that point outside it.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
    fn test_port53_arbitrary_ip_blocked() {
        // RFC5737 TEST-NET-2 address — guaranteed not in /etc/resolv.conf.
        let al = AllowList::from_hosts(&["198.51.100.1"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr: std::net::SocketAddr = "198.51.100.99:53".parse().unwrap();
        assert!(
            policy.check(&addr).is_err(),
            "port 53 to a non-resolver IP must be denied"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_port53_real_resolver_allowed() {
        let resolvers = dns_resolvers();
        if resolvers.is_empty() {
            eprintln!("skipping: no resolvers found in /etc/resolv.conf");
            return;
        }
        let resolver_ip = *resolvers.iter().next().unwrap();
        // Allowlist uses a TEST-NET IP that won't match the resolver.
        let al = AllowList::from_hosts(&["198.51.100.1"]).unwrap();
        let policy = NetworkPolicy::AllowList(al);
        let addr = std::net::SocketAddr::new(resolver_ip, 53);
        assert!(
            policy.check(&addr).is_ok(),
            "port 53 to a configured resolver ({}) must be allowed",
            resolver_ip
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
    fn net_tools_registered_with_blocklist() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let bl = BlockList::from_hosts(&["1.2.3.4"]).unwrap();
        register_internal_tools(
            &mut tools,
            &exit_code,
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
        register_internal_tools(&mut tools, &exit_code, None, None);
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "net_socket should not exist: {s}");
    }

    #[test]
    fn net_tools_registered_with_allow_all() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        register_internal_tools(&mut tools, &exit_code, Some(&NetworkPolicy::AllowAll), None);
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
        register_internal_tools(&mut tools, &exit_code, Some(&NetworkPolicy::AllowAll), None);
        // Create a socket first
        let req = br#"{"name":"net_socket","args":{"family":2,"type":1}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"fd\""), "net_socket should work: {s}");
        // Try to bind — should fail because no listen_ports
        let req = br#"{"name":"net_bind","args":{"fd":0,"addr":"127.0.0.1","port":8080}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(s.contains("\"error\""), "net_bind should be denied: {s}");
        assert!(s.contains("no --port"), "{s}");
    }

    #[test]
    fn net_bind_allowed_with_matching_port() {
        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        let lp = ListenPorts::from_ports([8080]);
        register_internal_tools(
            &mut tools,
            &exit_code,
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
        let lp = ListenPorts::from_ports([8080]);
        register_internal_tools(
            &mut tools,
            &exit_code,
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

    #[test]
    fn test_sleep_capped() {
        assert_eq!(MAX_SLEEP_NS, 60_000_000_000);

        let mut tools = ToolRegistry::new();
        let exit_code = Arc::new(AtomicI32::new(0));
        register_internal_tools(&mut tools, &exit_code, None, None);

        let req = br#"{"name":"__hl_sleep","args":{"ns":0}}"#;
        let resp = tools.dispatch(req);
        let s = std::str::from_utf8(&resp).unwrap();
        assert!(!s.contains("\"error\""), "sleep(0) should succeed: {s}");
    }

    #[test]
    fn net_getsockopt_returns_correct_type_for_dgram() {
        let mut reg = ToolRegistry::new();
        let policy = NetworkPolicy::AllowAll;
        register_net_tools(&mut reg, &policy, None);

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
}
