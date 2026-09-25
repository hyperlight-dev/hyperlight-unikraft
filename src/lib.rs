// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Hyperlight-Unikraft — host library for running Unikraft unikernels
//! on Hyperlight.
//!
//! ```no_run
//! use hyperlight_unikraft::SandboxBuilder;
//!
//! let mut sandbox = SandboxBuilder::from_initrd("rootfs/python.cpio")
//!     .scratch_mb(256)
//!     .boot()?;
//! sandbox.run("print('hello')")?;
//! let output = sandbox.drain_output();
//! assert!(output.contains("hello"));
//! # Ok::<(), hyperlight_unikraft::Error>(())
//! ```

use std::fmt::Write as _;
use std::fs::File;
use std::io::{Read as _, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub use hyperlight_host;

use hyperlight_host::{
    GuestBinary, HyperlightError, MultiUseSandbox, UninitializedSandbox, func::Registerable,
    sandbox::SandboxConfiguration, sandbox::snapshot::OciTag,
};

// Re-export snapshot types so dependents don't need hyperlight-host directly.
pub use hyperlight_host::{HostFunctions, sandbox::snapshot::Snapshot};

use tracing::{debug, info};

mod errno;
mod hostfs;
mod hostnet;
pub mod net_policy;

pub use net_policy::{AllowList, BlockList, ListenPorts, NetworkPolicy, ResolveError};

// ── Errors ──────────────────────────────────────────────────────────────

/// Why a sandbox operation failed.
///
/// The guest's conditions and this crate's contract have a variant each,
/// so an embedder can match them; what the hypervisor layer reports comes
/// through as [`Hyperlight`](Self::Hyperlight).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The driver reported the call failed, with the status it gave (see
    /// [`Yield::CallFailed`]).  The guest is alive; its output has the
    /// details.
    #[error("the call failed in the guest with status {status} (see its output)")]
    CallFailed { status: i32 },
    /// The guest process has exited with `status`: before the call
    /// returned, or before one could be submitted.  Nothing will run
    /// again.
    #[error("the guest exited with status {status}")]
    GuestExited { status: i32 },
    /// Every guest thread is blocked with no timer pending and no host
    /// socket that could wake it, while a call's return or the process's
    /// exit is still owed.
    #[error(
        "the guest is deadlocked: every thread is blocked with no timer pending and no host \
         socket that could wake it"
    )]
    Deadlocked,
    /// The guest has no driver to serve calls: its entry point is a plain
    /// program, driven with [`AppSandbox::join`].
    #[error(
        "the guest has no driver to serve calls: its entry point is a plain program (drive it \
         with join)"
    )]
    NoDriver,
    /// A call is already in flight; step until it is done first.
    #[error("a call is already in flight; step until it is done first")]
    CallInFlight,
    /// Nothing in the guest can exit: its driver is waiting for a call
    /// (use [`AppSandbox::run`] or [`AppSandbox::submit`], not `join`).
    #[error(
        "nothing in the guest can exit: its driver is waiting for a call (use run or submit \
         instead of join)"
    )]
    NothingToJoin,
    /// The kernel refused the call: nothing is reading `/dev/hlcall`.
    #[error("the guest refused the call: nothing is reading /dev/hlcall")]
    CallRejected,
    /// The guest halted without reporting a boundary or an exit: its
    /// kernel is not one of ours, or the entry failed.
    #[error("the guest halted without a word: its kernel reported neither a boundary nor an exit")]
    GuestSilent,
    /// The snapshot in `dir` was saved by a build with another
    /// [`SNAPSHOT_KEY`]: another kernel, or another host contract.
    #[error(
        "the snapshot in {} was saved by hyperlight-unikraft {saved_by}, whose kernel or host \
         contract differs from this build's, {this}; save it again with this build (`hluk \
         snapshot save`)",
        dir.display()
    )]
    SnapshotRelease {
        dir: PathBuf,
        saved_by: String,
        this: String,
    },
    /// More mounts, or longer entries, than the kernel takes.
    #[error(
        "a mount table of {mounts} mounts and {bytes} bytes of entries; the kernel takes at \
         most {MOUNTS_MAX} mounts and {FSTAB_ENTRIES_MAX} bytes"
    )]
    MountTable { mounts: usize, bytes: usize },
    /// A guest mount path the kernel's fstab list would misparse: not
    /// absolute, or containing whitespace, `:` or brackets.
    #[error(
        "guest mount path {guest_path:?} must be absolute and contain no whitespace, ':' or \
         brackets (it is passed to the kernel in its vfs.fstab list)"
    )]
    MountPath { guest_path: String },
    /// A host mount directory could not be opened.
    #[error(
        "cannot open the host directory {} for the guest mount {guest_path}: {source}",
        host_path.display()
    )]
    Mount {
        host_path: PathBuf,
        guest_path: String,
        #[source]
        source: std::io::Error,
    },
    /// A `from_snapshot` builder was given a kernel, initrd, entry point
    /// or scratch size; the snapshot carries its own.
    #[error(
        "a snapshot carries its own kernel, initrd, entry point and scratch size: \
         kernel/initrd/entry/scratch_mb do not apply to from_snapshot"
    )]
    SnapshotSettings,
    /// A name given to [`SandboxBuilder::host_function`] cannot be one:
    /// empty (it asks for the list of functions) or with a line break
    /// (the list has one name per line).
    #[error("{name:?} cannot name a host function: it must be non-empty and on one line")]
    HostFunctionName { name: String },
    /// A call's result was not UTF-8 text; [`AppSandbox::take_result`]
    /// hands it over as bytes.
    #[error("the call's result is not UTF-8 text")]
    ResultNotText,
    /// The script of an [`Exec::File`] could not be read.
    #[error("failed to read script {}: {source}", path.display())]
    Script {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    /// The hypervisor layer failed.
    #[error(transparent)]
    Hyperlight(#[from] HyperlightError),
}

/// The result of a sandbox operation.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A function the embedder offers the guest (see
/// [`SandboxBuilder::host_function`]): its argument text in, its result
/// text out, or an error message the guest raises as its own error.  The
/// drivers speak JSON on both sides; the library passes the text through.
pub type HostFunction =
    Arc<dyn Fn(&str) -> std::result::Result<String, String> + Send + Sync + 'static>;

/// The host functions a sandbox offers, by name.
type HostFunctionTable = Arc<std::collections::BTreeMap<String, HostFunction>>;

/// What `HostCall` answers: a tag byte, then the result or the error
/// message.  The drivers read it back (`hl_driver.h`'s `hl_host_call`).
const HOST_CALL_OK: u8 = 0;
const HOST_CALL_ERR: u8 = 1;

/// Run the host function `name` for the guest and encode its reply.  The
/// empty name, which no function can have, lists the registered names one
/// per line: how a driver learns what to offer before anything is called
/// (the quickjs driver builds its `host:` modules from it).
fn dispatch_host_call(table: &HostFunctionTable, name: &str, args: &[u8]) -> Vec<u8> {
    // The name is the guest's: quoted in an error, it is cut short, so a
    // long one cannot push the reply past what the guest can take.
    let shown = || name.chars().take(128).collect::<String>();
    let reply = if name.is_empty() {
        Ok(table
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join("\n"))
    } else {
        match (table.get(name), std::str::from_utf8(args)) {
            (Some(f), Ok(args)) => f(args),
            (Some(_), Err(_)) => Err(format!("{}: the arguments are not UTF-8 text", shown())),
            (None, _) => Err(format!("no host function {:?}", shown())),
        }
    };
    let (tag, text) = match reply {
        Ok(text) => (HOST_CALL_OK, text),
        Err(message) => (HOST_CALL_ERR, message),
    };
    // A reply the guest's buffer cannot take would arrive cut short; it
    // becomes an error the guest can report instead.
    let (tag, text) = if text.len() + 1 > HOST_CALL_MAX {
        (
            HOST_CALL_ERR,
            format!(
                "{}: a {}-byte result exceeds the {HOST_CALL_MAX}-byte host call limit",
                shown(),
                text.len()
            ),
        )
    } else {
        (tag, text)
    };
    let mut out = Vec::with_capacity(text.len() + 1);
    out.push(tag);
    out.extend_from_slice(text.as_bytes());
    out
}

// ── Constants ───────────────────────────────────────────────────────────

/// Embedded Unikraft app-elfloader kernel binary.
static KERNEL: &[u8] = include_bytes!("../kernel/elfloader_hyperlight-x86_64");

/// GPA where the initrd is mapped via `map_file_cow`.
///
/// Past the x86 LAPIC MMIO page (0xFEE0_0000) to avoid collisions
/// with KVM's in-kernel IRQCHIP reservation.
const INITRD_MAP_BASE: u64 = 0xFEF0_0000;

/// Scratch memory in MiB for a guest with no initrd to size from (a
/// native kernel), and the fallback when the initrd cannot be read.  A
/// guest booted from an initrd is sized by [`default_scratch_mb`].
///
/// The frame allocator gets 75% of the scratch; the rest covers CoW
/// faults and boot overhead.
pub const DEFAULT_SCRATCH_MB: usize = 256;

/// Guest memory per runtime image, in MiB: the sizes the repository's
/// tests and benchmarks run each published image with (the justfile's
/// `scratch_*`).  A rootfs is unpacked into this memory, so an image that
/// has grown past what these leave free needs more, and `hluk` says so.
pub const RUNTIME_SCRATCH_MB: &[(&str, usize)] = &[
    ("c", 64),
    ("rust", 64),
    ("quickjs", 64),
    ("go", 128),
    ("bash", 256),
    ("python", 256),
    ("python-shell", 256),
    ("wasmtime", 256),
    ("dotnet-aot", 256),
    ("node", 512),
    ("dotnet-jit", 768),
    ("powershell", 1024),
    ("agent", 1536),
];

/// The runtime image a driver binary belongs to, for an initrd that is
/// only a file.  The two Python variants share a driver and resolve to the
/// smaller; the `agent` image is known by name (see
/// [`runtime_scratch_mb`]).
const DRIVER_RUNTIME: &[(&str, &str)] = &[
    ("hl_pydriver", "python"),
    ("hl_pywarmdriver", "python-shell"),
    ("hl_nodedriver", "node"),
    ("hl_dotnetdriver", "dotnet-jit"),
    ("hl_pwshdriver", "powershell"),
    ("hl_quickjsdriver", "quickjs"),
    ("hl_wasmtimedriver", "wasmtime"),
    ("hl_bashdriver", "bash"),
    ("hl_godriver", "go"),
    ("hl_cdriver", "c"),
    ("hl_rustdriver", "rust"),
    ("hl_dotnetaotdriver", "dotnet-aot"),
];

/// The tested memory for a runtime image, by name (`python`, `node`,
/// `agent`, …): [`RUNTIME_SCRATCH_MB`].
pub fn runtime_scratch_mb(runtime: &str) -> Option<usize> {
    RUNTIME_SCRATCH_MB
        .iter()
        .find(|(name, _)| *name == runtime)
        .map(|(_, mb)| *mb)
}

/// The scratch memory a guest booted from `initrd` gets when none is asked
/// for: the runtime named in `etc/hluk-runtime` if the image carries one,
/// else the tested size of the runtime image its driver belongs to
/// ([`RUNTIME_SCRATCH_MB`]), else [`DEFAULT_SCRATCH_MB`].
/// [`SandboxBuilder::boot`] uses it when
/// [`scratch_mb`](SandboxBuilder::scratch_mb) was not called.
pub fn default_scratch_mb(initrd: &Path) -> usize {
    let scan = scan_cpio(initrd);
    // A runtime marker takes priority: images that share a driver binary
    // (agent and python-shell both use hl_pywarmdriver) write it so the
    // right scratch size is chosen without --scratch-mb.
    if let Some(mb) = scan.runtime.as_deref().and_then(runtime_scratch_mb) {
        return mb;
    }
    scan.entry
        .as_deref()
        .and_then(|p| p.rsplit('/').next())
        .and_then(driver_scratch_mb)
        .unwrap_or(DEFAULT_SCRATCH_MB)
}

/// [`default_scratch_mb`] from the driver's file name.
fn driver_scratch_mb(driver: &str) -> Option<usize> {
    DRIVER_RUNTIME
        .iter()
        .find(|(d, _)| *d == driver)
        .and_then(|(_, runtime)| runtime_scratch_mb(runtime))
}

#[cfg(test)]
mod scratch_tests {
    use super::*;

    #[test]
    fn every_driver_maps_to_a_tested_size() {
        for (driver, runtime) in DRIVER_RUNTIME {
            assert!(
                runtime_scratch_mb(runtime).is_some(),
                "{driver}: {runtime} is not in RUNTIME_SCRATCH_MB"
            );
        }
        assert_eq!(runtime_scratch_mb("python"), Some(256));
        assert_eq!(runtime_scratch_mb("agent"), Some(1536));
        assert_eq!(runtime_scratch_mb("cobol"), None);
        assert_eq!(driver_scratch_mb("hl_nodedriver"), Some(512));
        assert_eq!(
            driver_scratch_mb("hl_pywarmdriver"),
            Some(256),
            "the shared Python driver resolves to the smaller image"
        );
        assert_eq!(driver_scratch_mb("hl_execdriver"), None);
    }

    #[test]
    fn an_unreadable_initrd_gets_the_flat_default() {
        assert_eq!(
            default_scratch_mb(Path::new("/nonexistent/rootfs.cpio")),
            DEFAULT_SCRATCH_MB
        );
    }
}

/// Largest payload of one host call, in either direction.
///
/// The host decides this alone: it sizes the PEB I/O stacks
/// ([`IO_STACK_SIZE`]) and the guest sizes every transfer buffer from
/// the stack sizes it reads back out of the PEB
/// (`hl_hcall_max_payload()` in `plat/hyperlight/hcall.c`: the smaller
/// stack less a 4 KiB reserve for the FlatBuffer framing).  So a guest
/// never asks for, or sends, more than this, and the host functions
/// here only cap what they hand back (`net_recvfrom`) as a courtesy to
/// a guest that asks for more.
pub(crate) const HOST_CALL_MAX: usize = 64 * 1024;

/// PEB I/O stack size for host-call data transfer.
///
/// Both the input stack (host→guest results) and output stack
/// (guest→host calls) must hold a FlatBuffer-encoded message carrying
/// a [`HOST_CALL_MAX`] payload plus its framing, the same 4 KiB reserve
/// the guest subtracts.  Default Hyperlight stacks are only 16 KiB —
/// too small for large file or network transfers.
const IO_STACK_SIZE: usize = HOST_CALL_MAX + 4096;

/// PEB heap size.
///
/// Only needed for the boot stack (allocated before `ukplat_mem_init`).
/// Can be dropped to 0 once the guest allocates the boot stack from
/// scratch instead.
const HEAP_SIZE: u64 = 0x10_0000; // 1 MiB

/// What a snapshot depends on, as a tag suffix `k<kernel>-c<contract>`:
/// the embedded kernel's SHA-256 (its first 16 hex digits; the kernel's
/// code and host-call protocol are in the snapshot's memory) and the host
/// contract number kept in `build.rs` (the host functions and their
/// meaning, the buffer sizes, the layout, the MSRs, the hyperlight-host
/// release).  A snapshot loads under any release with the same key, so a
/// release that changes neither keeps every saved snapshot.
pub const SNAPSHOT_KEY: &str = env!("HLUK_SNAPSHOT_KEY");

/// The name a snapshot is saved under in its directory: this release and
/// its [`SNAPSHOT_KEY`], `<release>-<key>`.  A load matches the key
/// ([`load_snapshot`]); the release is for the message when none does.
/// (Both must stay valid in an OCI tag: no `+build` metadata; the unit
/// test below keeps that honest.)
fn snapshot_tag() -> OciTag {
    format!("{}-{SNAPSHOT_KEY}", env!("CARGO_PKG_VERSION"))
        .parse()
        .expect("the crate version and the snapshot key make a valid OCI tag")
}

/// The key of a snapshot tag, `<release>-<key>`, or `None` for a tag of
/// another shape (a snapshot from before keys was tagged by release alone).
fn snapshot_tag_key(tag: &str) -> Option<&str> {
    let at = tag.rfind("-k")?;
    let key = &tag[at + 1..];
    let (kernel, contract) = key[1..].split_once("-c")?;
    let hex = kernel.len() == 16 && kernel.bytes().all(|b| b.is_ascii_hexdigit());
    (hex && !contract.is_empty() && contract.bytes().all(|b| b.is_ascii_digit())).then_some(key)
}

/// A snapshot tag as a message names it: `0.14.1 (k…-c1)`, or the tag
/// itself when it carries no key.
fn describe_snapshot_tag(tag: &str) -> String {
    match snapshot_tag_key(tag) {
        Some(key) => format!("{} ({key})", &tag[..tag.len() - key.len() - 1]),
        None => tag.to_string(),
    }
}

/// Write a snapshot from [`AppSandbox::snapshot`] to `dir` as an OCI image
/// layout, named by this release and its [`SNAPSHOT_KEY`].  Another
/// process, or a later run of this one, reads it back with
/// [`load_snapshot`], [`SandboxBuilder::from_snapshot_dir`] or
/// [`AppSandbox::restore_from`]; any build with the same key can, since
/// the key is the kernel and the host contract a snapshot depends on.
/// [`AppSandbox::snapshot_to`] does both steps in one.
pub fn save_snapshot(snapshot: &Snapshot, dir: impl AsRef<Path>) -> Result<()> {
    let digest = snapshot.save(dir.as_ref(), &snapshot_tag())?;
    debug!(dir = %dir.as_ref().display(), %digest, "snapshot saved");
    Ok(())
}

/// Read a snapshot written by [`save_snapshot`] back into memory, to boot
/// ([`SandboxBuilder::from_snapshot`]) or restore ([`AppSandbox::restore`])
/// any number of guests from one load.  The snapshot must have been saved
/// with this build's [`SNAPSHOT_KEY`]; see [`save_snapshot`].
pub fn load_snapshot(dir: impl AsRef<Path>) -> Result<Arc<Snapshot>> {
    let dir = dir.as_ref();
    // The index names every save by release and key.  Load the one with
    // this build's key, whichever release saved it; otherwise say which
    // releases did, and what to do.  With no index to read, the layout
    // code reports that.
    let tag = match snapshot_tags(dir) {
        Some(tags) => match tags
            .iter()
            .find(|t| snapshot_tag_key(t) == Some(SNAPSHOT_KEY))
        {
            Some(tag) => tag.parse::<OciTag>().map_err(|e| {
                Error::Hyperlight(hyperlight_host::new_error!("snapshot tag {:?}: {}", tag, e))
            })?,
            None => {
                return Err(Error::SnapshotRelease {
                    dir: dir.to_path_buf(),
                    saved_by: if tags.is_empty() {
                        "an unknown release".to_string()
                    } else {
                        tags.iter()
                            .map(|t| describe_snapshot_tag(t))
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                    this: format!("{} ({SNAPSHOT_KEY})", env!("CARGO_PKG_VERSION")),
                });
            }
        },
        None => snapshot_tag(),
    };
    Ok(Arc::new(Snapshot::load(dir, tag)?))
}

/// The tags of the OCI index in `dir`, one per save.  `None` when there
/// is no index to read, which [`Snapshot::load`] reports.
fn snapshot_tags(dir: &Path) -> Option<Vec<String>> {
    let index: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.join("index.json")).ok()?).ok()?;
    Some(
        index["manifests"]
            .as_array()?
            .iter()
            .filter_map(|m| m["annotations"]["org.opencontainers.image.ref.name"].as_str())
            .map(str::to_string)
            .collect(),
    )
}

/// MSRs the Unikraft guest reads/writes, which hyperlight 0.17.0's
/// default-deny KVM MSR filter must permit.
///
/// From 0.17.0 the vCPU runs behind a KVM MSR filter that faults (#GP)
/// on any guest rdmsr/wrmsr of an MSR the host has not declared.  The
/// kernel needs the SYSCALL entry set so the elfloader can drop ring-3
/// ELFs into a `syscall`, plus PAT, which the native paging init resets.
///
/// Both the boot path and the restore path (both under
/// [`SandboxBuilder::boot`]) must declare the SAME set: a snapshot persists exactly
/// the declared MSRs and restore rejects any it cannot map back onto the
/// restoring VM's declared set.
const GUEST_MSRS: &[u32] = &[
    0x277,       // IA32_PAT   — page-attribute table (paging init)
    0xC000_0081, // IA32_STAR  — syscall CS/SS selectors
    0xC000_0082, // IA32_LSTAR — syscall entry RIP
    0xC000_0084, // IA32_FMASK — syscall RFLAGS mask
];

/// The TSC frequency, in Hz, measured once against the monotonic clock.
///
/// Hyperlight passes the host TSC through unscaled (it only saves and
/// restores the TSC register), so the rate the guest sees is this one.
/// Twenty milliseconds against a nanosecond clock give it to a few parts
/// per million, provided the two endpoints are clean: each clock reading
/// is bracketed by two counter reads and kept only when nothing ran in
/// between, so a preemption cannot skew the one sample the process keeps.
/// The invariant TSC does not drift with the core frequency, so measuring
/// once is enough.
///
/// TODO: the kernel asks only at boot.  A snapshot restored on a host with
/// a different TSC rate keeps the old frequency, so its monotonic clock
/// and sleeps run fast or slow by the ratio; the resume entry should ask
/// again and re-base the clock (the wall clock is already re-anchored
/// there).
fn host_tsc_hz() -> u64 {
    static HZ: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *HZ.get_or_init(|| {
        #[cfg(target_arch = "x86_64")]
        {
            use std::arch::x86_64::_rdtsc;
            /// The clock and the counter at one instant: the tightest of
            /// up to a hundred brackets, taken at once when the bracket is
            /// under 10 000 ticks (a few microseconds at any plausible
            /// rate, which no preemption fits in).
            fn sample() -> (Instant, u64) {
                let mut best: Option<(u64, Instant, u64)> = None;
                for _ in 0..100 {
                    let a = unsafe { _rdtsc() };
                    let t = Instant::now();
                    let b = unsafe { _rdtsc() };
                    let width = b.wrapping_sub(a);
                    if best.is_none_or(|(w, _, _)| width < w) {
                        best = Some((width, t, a + width / 2));
                    }
                    if width < 10_000 {
                        break;
                    }
                }
                let (_, t, c) = best.expect("at least one sample");
                (t, c)
            }
            let (t0, c0) = sample();
            std::thread::sleep(Duration::from_millis(20));
            let (t1, c1) = sample();
            let ns = t1.duration_since(t0).as_nanos();
            ((c1 - c0) as u128 * 1_000_000_000 / ns) as u64
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            0
        }
    })
}

/// Declare [`GUEST_MSRS`] on a sandbox configuration.
fn apply_guest_msrs(cfg: &mut SandboxConfiguration) -> Result<()> {
    cfg.guest_msrs(GUEST_MSRS).map_err(|e| {
        Error::Hyperlight(hyperlight_host::new_error!("declaring guest MSRs: {}", e))
    })?;
    Ok(())
}

// ── Windows surrogate processes ────────────────────────────────────────

/// Choose how many WHP *surrogate processes* Hyperlight may use.
///
/// On Windows, Hyperlight maps each VM's memory through a helper
/// process so that many VMs can share one host process, and by default
/// pre-spawns 512 of them the first time a sandbox is created.  Pass
/// `0` to disable them: memory is mapped directly, there is no start-up
/// cost, and the process may hold **one live sandbox at a time**, which
/// is right for anything that runs one guest per process (the `hluk`
/// CLI does this).  Pass `max > 0` to allow that many concurrent
/// sandboxes, with helpers spawned on demand instead of up front.
///
/// Hyperlight reads this only from the environment, once, before the
/// first sandbox exists, so this sets `HYPERLIGHT_MAX_SURROGATES` and
/// `HYPERLIGHT_INITIAL_SURROGATES` and must be called before any
/// sandbox is created; later calls have no effect.  Left alone,
/// Hyperlight's own default applies.
#[cfg(windows)]
pub fn configure_surrogates(max: usize) {
    // SAFETY: on Windows `set_var` is backed by `SetEnvironmentVariableW`,
    // which is thread-safe; the unsafety of `set_var` concerns POSIX
    // `getenv` races, which cannot occur here.
    unsafe {
        std::env::set_var("HYPERLIGHT_MAX_SURROGATES", max.to_string());
        std::env::set_var("HYPERLIGHT_INITIAL_SURROGATES", "0");
    }
}

// ── Mount ───────────────────────────────────────────────────────────────

/// A host filesystem mount passed to the guest.
#[derive(Debug, Clone)]
pub struct Mount {
    /// Guest-visible mount point (e.g. `/mnt/data`).
    pub guest_path: String,
    /// Host directory to expose.
    pub host_path: PathBuf,
    /// Mount read-only (`true` → `MNT_RDONLY`, writes return `EROFS`).
    pub readonly: bool,
}

impl Mount {
    /// Create a read-write mount.
    ///
    /// Parameter order matches Docker convention: host (source) first,
    /// guest (target) second.
    pub fn rw(host_path: impl Into<PathBuf>, guest_path: impl Into<String>) -> Self {
        Self {
            guest_path: guest_path.into(),
            host_path: host_path.into(),
            readonly: false,
        }
    }

    /// Create a read-only mount.
    ///
    /// Parameter order matches Docker convention: host (source) first,
    /// guest (target) second.
    pub fn ro(host_path: impl Into<PathBuf>, guest_path: impl Into<String>) -> Self {
        Self {
            guest_path: guest_path.into(),
            host_path: host_path.into(),
            readonly: true,
        }
    }
}

// ── Cooperative step ────────────────────────────────────────────────────

/// Why [`AppSandbox::step`] handed control back.
///
/// The guest runs its scheduler only until every thread is blocked, then
/// yields the vCPU with a report; the host waits for what the report says
/// and re-enters.  Guest memory — every parked thread, the scheduler
/// queues, the application heap — persists across the boundary, and a
/// snapshot taken there resumes exactly where the guest left off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yield {
    /// The guest process ended and the kernel shut down with its exit
    /// status.  Nothing will run again; every later step reports this.
    Exited { status: i32 },
    /// The call started with [`AppSandbox::submit`] has returned.  The
    /// guest is alive and can take another.
    CallDone,
    /// The call returned but the driver reported it failed, with `status`
    /// as the driver put it: a program's or an `exit()`'s own code, 1 for
    /// an uncaught exception (what the runtime's script would exit with),
    /// -1 when the driver could not run the call at all.  The guest is
    /// alive and can take another call; its output has the details.
    CallFailed { status: i32 },
    /// Every guest thread is blocked.  `until` is the guest's next timer,
    /// or `None` when only I/O on one of its host sockets can wake it.
    ///
    /// [`AppSandbox::step`] returns this in two cases it does not tell
    /// apart: the guest ran and blocked again, or the timeout ran out and
    /// the VM was never entered (then `until` is the last one heard).
    /// TODO: a distinct variant if a caller ever needs to know whether
    /// guest code ran.
    Blocked { until: Option<Instant> },
}

/// One thing the guest said, through one of its six event host functions
/// (see the kernel's `plat/hyperlight/step.c`).  Three of them are a
/// [`Yield`] as is: `Yield(ns)` is [`Yield::Blocked`], `CallDone(status)`
/// is [`Yield::CallDone`] or [`Yield::CallFailed`], `Exited(status)` is
/// [`Yield::Exited`].  The other three are facts about the guest that no
/// step returns by themselves.  An entry can say several (`CallDone`,
/// then `Blocked`; or nothing at all), while the VM is still running; once
/// it halts, [`GuestConfig::absorb`] reads them in order and reduces them
/// to the one [`Yield`] the entry amounts to.
#[derive(Debug, Clone, Copy)]
enum Event {
    /// A way the entry could end, as the guest put it.
    Outcome(Yield),
    /// `/dev/hlcall` was opened: named calls are served.
    DriverReady,
    /// The driver took a named call.
    CallStarted,
    /// A named call had no reader, or did not fit: it never ran.
    CallRejected,
}

/// The host's standing picture of the guest, folded from its events.
#[derive(Debug, Default)]
struct Guest {
    /// Named calls are served.
    has_driver: bool,
    /// Between `CallStarted` and `CallDone`.
    call_in_flight: bool,
    /// Absolute deadline of its next timer, from its last `Yield`.
    next_wakeup_at: Option<Instant>,
}

// ── GuestConfig ─────────────────────────────────────────────────────────

/// Runtime parameters for the guest's host functions.
///
/// Built once during sandbox setup, then used to register identical
/// host functions for both the init and snapshot-restore paths.
///
/// The fields are internal — a caller receives a `GuestConfig` from
/// [`SandboxBuilder::boot`] and interacts with it through the
/// methods ([`set_env_vars`](Self::set_env_vars), [`drain_output`](Self::drain_output)).
pub(crate) struct GuestConfig {
    cmdline: String,
    scratch_size: usize,
    initrd_base: u64,
    initrd_size: u64,
    /// Host filesystem mounts.
    mounts: Vec<Mount>,
    /// Captured guest stdout — accumulated by the HostPrint callback.
    output: Arc<Mutex<String>>,
    /// NUL-separated KEY=VALUE pairs for guest env vars.
    env_str: Arc<Mutex<String>>,
    /// The guest's `/etc/resolv.conf`, written by the kernel at boot and on
    /// every restore; empty leaves the rootfs's own file in place.
    resolv_conf: Arc<Mutex<String>>,
    /// Host networking state (`None` = networking disabled).  Shared with
    /// the `net_*` host functions; kept here for the inter-step wait of
    /// the cooperative step model.
    net: Option<Arc<hostnet::Net>>,
    /// Events from the guest's last entry, in order; shared with the event
    /// host functions, which push, and drained by [`absorb`](Self::absorb).
    events: Arc<Mutex<Vec<Event>>>,
    /// What the call in flight returned (`CallResult`), taken by
    /// [`AppSandbox::take_result`]; cleared when a call is submitted.
    result: Arc<Mutex<Option<Vec<u8>>>>,
    /// The embedder's functions, served through `HostCall`.
    host_functions: HostFunctionTable,
    /// What those events add up to.
    guest: Mutex<Guest>,
}

impl GuestConfig {
    /// Assemble a config; `register` must be called on the sandbox next.
    #[allow(clippy::too_many_arguments)]
    fn new(
        cmdline: String,
        scratch_size: usize,
        initrd_base: u64,
        initrd_size: u64,
        mounts: Vec<Mount>,
        network: Option<NetworkPolicy>,
        listen_ports: Option<ListenPorts>,
    ) -> Self {
        // Networking is opt-in: no policy, no host sockets and nothing for
        // the inter-step wait to watch (the `net_*` functions still exist,
        // refusing; see `register`).
        let net = network.map(|policy| Arc::new(hostnet::Net::new(policy, listen_ports)));
        Self {
            cmdline,
            scratch_size,
            initrd_base,
            initrd_size,
            mounts,
            output: Arc::new(Mutex::new(String::new())),
            env_str: Arc::new(Mutex::new(String::new())),
            resolv_conf: Arc::new(Mutex::new(String::new())),
            net,
            events: Arc::new(Mutex::new(Vec::new())),
            result: Arc::new(Mutex::new(None)),
            host_functions: HostFunctionTable::default(),
            guest: Mutex::new(Guest::default()),
        }
    }

    /// The embedder's host functions; set before [`register`](Self::register).
    fn with_host_functions(mut self, table: HostFunctionTable) -> Self {
        self.host_functions = table;
        self
    }

    /// How much scratch memory to give the paging frame allocator (75%).
    fn paging_budget(&self) -> u64 {
        (self.scratch_size as u64) * 3 / 4
    }

    /// Top of the exception stack in guest virtual address space.
    fn exn_stack_top(&self) -> u64 {
        hyperlight_common::layout::SCRATCH_TOP_GVA as u64
            - hyperlight_common::layout::SCRATCH_TOP_EXN_STACK_OFFSET
            + 1
    }

    /// Set environment variables to pass to the guest.
    ///
    /// For env that should be present from the guest's first dispatch,
    /// prefer [`SandboxBuilder::env`], which applies these before the guest
    /// boots.  This setter updates the environment for subsequent `run()`
    /// dispatches: each dispatch calls `hl_env_refresh()`, which re-queries
    /// the host and calls `setenv()` for every returned variable.
    ///
    /// **Caveat — full replace, not merge**: This replaces the entire
    /// env var set, not merging with the previous one.  Variables
    /// removed from the host side will **not** be unset in the guest's
    /// glibc `environ` — `hl_env_refresh()` only calls `setenv()`,
    /// never `unsetenv()`.  Stale variables from earlier dispatches
    /// linger in the guest until it is restored from a snapshot.
    ///
    /// ```no_run
    /// # use hyperlight_unikraft::SandboxBuilder;
    /// let sandbox = SandboxBuilder::from_initrd("rootfs/python.cpio").boot().unwrap();
    /// sandbox.set_env_vars(&[("MY_VAR", "hello"), ("DEBUG", "1")]);
    /// // the next sandbox.run(…) observes them
    /// ```
    pub fn set_env_vars(&self, vars: &[(&str, &str)]) {
        let mut s = String::new();
        for (k, v) in vars {
            s.push_str(k);
            s.push('=');
            s.push_str(v);
            s.push('\0');
        }
        *self.env_str.lock().unwrap() = s;
    }

    /// The resolver configuration the kernel writes as the guest's
    /// `/etc/resolv.conf`: at boot, and again on every restore, so a
    /// snapshot taken elsewhere resolves names where it now runs.  Empty
    /// leaves the rootfs's own file in place.
    ///
    /// glibc's parallel A and AAAA queries do not work through the socket
    /// layer, so `options single-request` is added unless the caller set
    /// an options line of their own that has it.
    fn set_resolv_conf(&self, content: &str) {
        *self.resolv_conf.lock().unwrap() = with_single_request(content);
    }

    /// Drain captured guest output, clearing the buffer.
    pub fn drain_output(&self) -> String {
        self.output.lock().unwrap().split_off(0)
    }

    /// Register host functions on any [`Registerable`] target.
    ///
    /// Works for both the init path (`UninitializedSandbox`) and the
    /// snapshot-restore path (`HostFunctions`).
    pub fn register(&self, target: &mut impl Registerable) -> Result<()> {
        // Override Hyperlight's default HostPrint (which wraps output in
        // green ANSI on stdout) — send guest output to stdout uncolored,
        // and capture it for programmatic access.
        let output = self.output.clone();
        target.register_host_function(
            "HostPrint",
            move |msg: String| -> hyperlight_host::Result<i32> {
                use std::io::Write;
                let len = msg.len() as i32;
                print!("{msg}");
                let _ = std::io::stdout().flush();
                output.lock().unwrap().push_str(&msg);
                Ok(len)
            },
        )?;

        let cmdline = self.cmdline.clone();
        target
            .register_host_function("GetCmdLine", move || -> hyperlight_host::Result<String> {
                Ok(cmdline.clone())
            })?;

        // The mount table this host serves.  A restored guest makes its own
        // match on `resume`; a fresh guest boots with the same list from its
        // cmdline.
        let mounts = fstab_entries(&self.mounts)?;
        target
            .register_host_function("GetMounts", move || -> hyperlight_host::Result<String> {
                Ok(mounts.clone())
            })?;

        let budget = self.paging_budget();
        target.register_host_function(
            "GetPagingBudget",
            move || -> hyperlight_host::Result<u64> { Ok(budget) },
        )?;

        let base = self.initrd_base;
        target
            .register_host_function("GetInitrdBase", move || -> hyperlight_host::Result<u64> {
                Ok(base)
            })?;

        let size = self.initrd_size;
        target
            .register_host_function("GetInitrdSize", move || -> hyperlight_host::Result<u64> {
                Ok(size)
            })?;

        let est = self.exn_stack_top();
        target
            .register_host_function("GetExnStackTop", move || -> hyperlight_host::Result<u64> {
                Ok(est)
            })?;

        target.register_host_function("GetWallClockNs", || -> hyperlight_host::Result<u64> {
            Ok(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0))
        })?;

        // The guest's clock is the TSC, whose frequency KVM does not tell
        // it (no CPUID.15H, no hypervisor leaf); without this it assumes
        // 2.5 GHz and its clock runs fast or slow by the difference.
        target.register_host_function("GetTscHz", || -> hyperlight_host::Result<u64> {
            Ok(host_tsc_hz())
        })?;

        target
            .register_host_function("GetHostFsChunkSize", || -> hyperlight_host::Result<u64> {
                Ok(hostfs::CHUNK as u64)
            })?;

        // ── Environment variables ─────────────────────────────────
        let env_str = self.env_str.clone();
        target
            .register_host_function("GetEnvVars", move || -> hyperlight_host::Result<String> {
                Ok(env_str.lock().unwrap().clone())
            })?;

        // ── Resolver configuration ────────────────────────────────
        let resolv_conf = self.resolv_conf.clone();
        target.register_host_function(
            "GetResolvConf",
            move || -> hyperlight_host::Result<String> { Ok(resolv_conf.lock().unwrap().clone()) },
        )?;

        // ── Stdin ─────────────────────────────────────────────────
        target.register_host_function(
            "ReadStdin",
            move || -> hyperlight_host::Result<String> {
                use std::io::Read;
                let mut data = vec![0u8; 4096];
                let n = std::io::stdin().read(&mut data).unwrap_or(0);
                data.truncate(n);
                Ok(String::from_utf8_lossy(&data).into_owned())
            },
        )?;

        // ── Cooperative step ──────────────────────────────────────
        // The guest reports through named host functions, one fact each.
        // They are only recorded here, in order; `absorb` reads them once
        // the entry has halted.
        let events = self.events.clone();
        target.register_host_function("Yield", move |ns: u64| -> hyperlight_host::Result<i32> {
            // Keep the absolute deadline so time the host spends elsewhere
            // counts against it and the guest timer still fires on schedule.
            let until = (ns != 0).then(|| Instant::now() + Duration::from_nanos(ns));
            events
                .lock()
                .unwrap()
                .push(Event::Outcome(Yield::Blocked { until }));
            Ok(0)
        })?;
        let events = self.events.clone();
        target.register_host_function("DriverReady", move || -> hyperlight_host::Result<i32> {
            events.lock().unwrap().push(Event::DriverReady);
            Ok(0)
        })?;
        let events = self.events.clone();
        target.register_host_function("CallStarted", move || -> hyperlight_host::Result<i32> {
            events.lock().unwrap().push(Event::CallStarted);
            Ok(0)
        })?;
        let events = self.events.clone();
        target.register_host_function(
            "CallDone",
            move |status: i32| -> hyperlight_host::Result<i32> {
                let done = if status == 0 {
                    Yield::CallDone
                } else {
                    Yield::CallFailed { status }
                };
                events.lock().unwrap().push(Event::Outcome(done));
                Ok(0)
            },
        )?;
        // What the call returned, sent just before its CallDone.
        let result = self.result.clone();
        target.register_host_function(
            "CallResult",
            move |bytes: Vec<u8>| -> hyperlight_host::Result<i32> {
                *result.lock().unwrap() = Some(bytes);
                Ok(0)
            },
        )?;
        // The embedder's functions, all behind one name: the kernel
        // forwards a driver's HLCALL_IOC_HOSTCALL here as it is.
        let table = self.host_functions.clone();
        target.register_host_function(
            "HostCall",
            move |name: String, args: Vec<u8>| -> hyperlight_host::Result<Vec<u8>> {
                Ok(dispatch_host_call(&table, &name, &args))
            },
        )?;
        let events = self.events.clone();
        target.register_host_function(
            "CallRejected",
            move || -> hyperlight_host::Result<i32> {
                events.lock().unwrap().push(Event::CallRejected);
                Ok(0)
            },
        )?;
        let events = self.events.clone();
        target.register_host_function(
            "Exited",
            move |status: i32| -> hyperlight_host::Result<i32> {
                events
                    .lock()
                    .unwrap()
                    .push(Event::Outcome(Yield::Exited { status }));
                Ok(0)
            },
        )?;

        // The `fs_*` and `net_*` functions are registered on every path, so
        // a restore offers every host function the snapshot's guest can
        // call.  Without a policy the network refuses every `socket()`, so
        // a guest saved under one and restored without it has its sockets
        // die on resume, and one saved without and restored under one gets
        // to use the network.
        hostfs::register(target, &self.mounts)?;
        match &self.net {
            Some(net) => hostnet::register(target, net)?,
            None => hostnet::register(target, &Arc::new(hostnet::Net::disabled()))?,
        }

        Ok(())
    }

    // ── Cooperative step (driven by AppSandbox) ─────────────────────

    /// Whether a runtime driver is serving named calls, per what the
    /// guest has said.
    fn has_driver(&self) -> bool {
        self.guest.lock().unwrap().has_driver
    }

    /// Whether the guest is serving a named call, per what it has said.
    fn call_in_flight(&self) -> bool {
        self.guest.lock().unwrap().call_in_flight
    }

    /// The guest's next timer, from its last `Yield`.
    fn next_wakeup_at(&self) -> Option<Instant> {
        self.guest.lock().unwrap().next_wakeup_at
    }

    /// One VM entry: call `name`, then read what the guest said during it.
    /// The guest runs its scheduler until every thread is blocked and
    /// halts; see [`absorb`](Self::absorb) for how the events reduce.
    fn enter<Args>(&self, sandbox: &mut MultiUseSandbox, name: &str, args: Args) -> Result<Yield>
    where
        Args: hyperlight_host::func::ParameterTuple,
    {
        // An entry starts with an empty inbox: a previous one that failed
        // in the hypervisor may have left events behind.
        self.events.lock().unwrap().clear();
        sandbox.call::<()>(name, args)?;
        let yielded = self.absorb()?;
        debug!(?yielded, name, "entry");
        Ok(yielded)
    }

    /// Take the events of the entry that just halted, in order, fold them
    /// into the standing picture of the guest, and reduce them to what the
    /// entry amounts to: a terminal event (the call returned, the process
    /// ended) wins; else a `Yield` means a boundary was reached.  A halt
    /// with neither is an error: every kernel on this platform reports one
    /// or the other, so silence is a kernel that is not one of ours or a
    /// dispatch that failed.  A rejected call is an error too: nothing in
    /// the guest could run it.
    fn absorb(&self) -> Result<Yield> {
        let events = std::mem::take(&mut *self.events.lock().unwrap());
        let mut guest = self.guest.lock().unwrap();
        let mut terminal = None;
        let mut boundary = None;
        let mut rejected = false;

        guest.next_wakeup_at = None;
        for event in events {
            match event {
                Event::Outcome(Yield::Blocked { until }) => {
                    guest.next_wakeup_at = until;
                    boundary = Some(Yield::Blocked { until });
                }
                Event::Outcome(Yield::Exited { status }) => {
                    terminal = Some(Yield::Exited { status });
                }
                Event::Outcome(done @ (Yield::CallDone | Yield::CallFailed { .. })) => {
                    guest.call_in_flight = false;
                    // A failed call's result, should a driver send one, is
                    // not the call's.
                    if matches!(done, Yield::CallFailed { .. }) {
                        *self.result.lock().unwrap() = None;
                    }
                    terminal = Some(done);
                }
                Event::DriverReady => guest.has_driver = true,
                Event::CallStarted => guest.call_in_flight = true,
                Event::CallRejected => rejected = true,
            }
        }
        if rejected {
            return Err(Error::CallRejected);
        }
        terminal.or(boundary).ok_or(Error::GuestSilent)
    }

    /// Forget everything heard: the guest state the picture described was
    /// just replaced by a restored snapshot.  The `resume` entry that
    /// follows has the guest say again what still holds.
    fn forget(&self) {
        self.events.lock().unwrap().clear();
        // A result from the timeline the restore discarded is not this one's.
        *self.result.lock().unwrap() = None;
        *self.guest.lock().unwrap() = Guest::default();
    }

    /// Whether anything can make the guest runnable again: a pending
    /// timer, or a host socket that can still produce an event.  False
    /// means the guest is blocked for good: nothing the host watches, or
    /// could watch, will ever change its state.
    fn can_wake(&self) -> bool {
        self.next_wakeup_at().is_some() || self.net.as_ref().is_some_and(|n| n.has_waitable())
    }

    /// Park the host until the guest may be runnable again: its next timer
    /// is due, or one of its host sockets is ready.  Waits at most `cap`,
    /// or without limit for `None`.  Returns `false` if the cap ran out
    /// first, or if there was nothing to wait for.
    ///
    /// The VM is halted throughout; the host thread sits in `poll(2)` on
    /// the guest's sockets (a plain sleep when it has none).
    fn wait_runnable(&self, cap: Option<Duration>) -> bool {
        let deadline = self.next_wakeup_at();
        let to_timer = deadline.map(|d| d.saturating_duration_since(Instant::now()));
        let dur = match (to_timer, cap) {
            (Some(t), Some(c)) => Some(t.min(c)),
            (Some(t), None) => Some(t),
            (None, Some(c)) => Some(c),
            (None, None) => None,
        };
        let woke_on_io = match (dur, &self.net) {
            (Some(d), _) if d.is_zero() => false,
            (_, Some(net)) => net.wait_ready(dur),
            (Some(d), None) => {
                std::thread::sleep(d);
                false
            }
            // No timer, no cap, no sockets: nothing could end the wait.
            (None, None) => false,
        };
        woke_on_io || deadline.is_some_and(|d| Instant::now() >= d)
    }
}

// ── CPIO helpers (private) ─────────────────────────────────────────────

/// Scan a newc-format CPIO archive for a Hyperlight driver binary.
///
/// The initrd is a CPIO archive containing the guest's root filesystem.
/// This function walks entries looking for `usr/local/bin/hl_*` or
/// `usr/bin/hl_*` — the conventional path for Hyperlight driver binaries
/// (e.g. `hl_pydriver`, `hl_nodedriver`) — and returns the first match
/// as a guest-absolute path.
///
/// Used internally by [`SandboxBuilder::boot`] to auto-detect the entry
/// point so callers don't need to set one manually.
fn find_cpio_entry(path: &Path) -> Option<String> {
    scan_cpio(path).entry
}

/// What a single pass over a CPIO archive found.
#[derive(Default)]
struct CpioScan {
    /// The driver entry point (`/usr/local/bin/hl_pydriver`), if any.
    entry: Option<String>,
    /// The runtime name from `etc/hluk-runtime`, if the image carries
    /// one.  Images that share a driver binary (the `agent` image and
    /// `python-shell` both use `hl_pywarmdriver`) write this file so
    /// [`default_scratch_mb`] sizes them correctly; without it, the
    /// driver-based fallback applies and the scratch hint fires when
    /// the rootfs outgrows that.
    runtime: Option<String>,
}

/// Scan a CPIO archive in one pass for the driver entry point and an
/// optional runtime marker file.
fn scan_cpio(path: &Path) -> CpioScan {
    scan_cpio_inner(path).unwrap_or_default()
}

fn scan_cpio_inner(path: &Path) -> Option<CpioScan> {
    let mut file = File::open(path).ok()?;
    let mut header = [0u8; 110];
    let mut entry = None;
    let mut runtime = None;

    loop {
        if file.read_exact(&mut header).is_err() {
            break;
        }

        let magic = std::str::from_utf8(&header[0..6]).ok()?;
        if magic != "070701" && magic != "070702" {
            break;
        }

        let namesize = u32::from_str_radix(std::str::from_utf8(&header[94..102]).ok()?, 16).ok()?;
        let filesize = u64::from_str_radix(std::str::from_utf8(&header[54..62]).ok()?, 16).ok()?;

        let mut name_buf = vec![0u8; namesize as usize];
        file.read_exact(&mut name_buf).ok()?;
        let name = std::str::from_utf8(&name_buf).ok()?.trim_end_matches('\0');

        if name == "TRAILER!!!" {
            break;
        }

        let name_padding = (4 - ((110 + namesize) % 4)) % 4;
        file.seek(SeekFrom::Current(name_padding as i64)).ok()?;

        if entry.is_none()
            && (name.starts_with("usr/local/bin/hl_") || name.starts_with("usr/bin/hl_"))
        {
            entry = Some(format!("/{name}"));
        }

        // A rootfs may carry its runtime name so scratch memory can be
        // sized even when the driver binary is shared between images.
        if runtime.is_none() && name == "etc/hluk-runtime" && filesize < 64 {
            let mut buf = vec![0u8; filesize as usize];
            if file.read_exact(&mut buf).is_ok() {
                if let Ok(s) = std::str::from_utf8(&buf) {
                    let s = s.trim();
                    if !s.is_empty() {
                        runtime = Some(s.to_string());
                    }
                }
                let data_padding = (4 - (filesize % 4)) % 4;
                file.seek(SeekFrom::Current(data_padding as i64)).ok()?;
                if entry.is_some() {
                    break;
                }
                continue;
            }
        }

        let data_padding = (4 - (filesize % 4)) % 4;
        file.seek(SeekFrom::Current((filesize + data_padding) as i64))
            .ok()?;

        if entry.is_some() && runtime.is_some() {
            break;
        }
    }

    Some(CpioScan { entry, runtime })
}

/// Resolve the entry point: explicit value → auto-detected from initrd → None.
fn resolve_entry(entry: &Option<String>, initrd: &Option<PathBuf>) -> Option<String> {
    if let Some(e) = entry {
        return Some(e.clone());
    }
    if let Some(path) = initrd
        && let Some(detected) = find_cpio_entry(path)
    {
        info!(entry = %detected, "auto-detected driver entry point");
        return Some(detected);
    }
    None
}

// ── Public API ─────────────────────────────────────────────────────────

/// Mounts the kernel takes: its resume-time reconcile keeps this many
/// (`HOSTFS_RESUME_MOUNTS_MAX` in lib/hostfs).
pub const MOUNTS_MAX: usize = 32;

/// Bytes of `vfs.fstab` entries the kernel takes: its cmdline buffer holds
/// 4 KiB, less the program name, the entry point and the parameter's own
/// syntax; its resume-time list buffer is larger.
pub const FSTAB_ENTRIES_MAX: usize = 3584;

/// The guest's mount table for `mounts`, as `vfs.fstab` entries: one
/// hostfs entry per mount, whose source-device field is the mount's index
/// (hostfs routes host calls by it) and whose options make the mount
/// point.  A fresh guest reads them off its cmdline ([`fstab_arg`]); a
/// restored guest fetches them through `GetMounts` on its `resume` entry.
/// The list is unquoted: entries are separated by spaces and fields by
/// colons, so a guest path holding either is refused here, where the
/// error can say why.
fn fstab_entries(mounts: &[Mount]) -> Result<String> {
    let mut entries = String::new();
    for (i, m) in mounts.iter().enumerate() {
        let unfit = |c: char| c.is_whitespace() || matches!(c, ':' | '[' | ']');
        if !m.guest_path.starts_with('/') || m.guest_path.contains(unfit) {
            return Err(Error::MountPath {
                guest_path: m.guest_path.clone(),
            });
        }
        if i > 0 {
            entries.push(' ');
        }
        // Format: sdev:path:drv:flags:opts:ukopts.  flags: MNT_RDONLY is
        // 0x1.  ukopts: mkmp creates the mount point if missing.  No
        // quotes: uk_libparam does not strip them.
        let flags = if m.readonly { "0x1" } else { "0x0" };
        write!(entries, "{i}:{}:hostfs:{flags}::mkmp", m.guest_path).unwrap();
    }
    if mounts.len() > MOUNTS_MAX || entries.len() > FSTAB_ENTRIES_MAX {
        return Err(Error::MountTable {
            mounts: mounts.len(),
            bytes: entries.len(),
        });
    }
    Ok(entries)
}

/// The kernel's `vfs.fstab` cmdline parameter for `mounts` (empty for
/// none): [`fstab_entries`] in the brackets uk_libparam expects.
fn fstab_arg(mounts: &[Mount]) -> Result<String> {
    let entries = fstab_entries(mounts)?;
    if entries.is_empty() {
        return Ok(entries);
    }
    Ok(format!(" vfs.fstab=[{entries}]"))
}

/// Assemble the uninitialized sandbox and its [`GuestConfig`] from the
/// pieces a [`SandboxBuilder`] gathered.  `kernel` is `None` for the
/// embedded [`KERNEL`], `Some` for an external one; `initrd` is `None`
/// for a self-contained kernel that carries its own workload.
#[allow(clippy::too_many_arguments)]
fn assemble_sandbox(
    kernel: &Option<PathBuf>,
    initrd: &Option<PathBuf>,
    entry: &Option<String>,
    scratch_mb: usize,
    mounts: Vec<Mount>,
    network: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    host_functions: HostFunctionTable,
) -> Result<(UninitializedSandbox, GuestConfig)> {
    let scratch_size = scratch_mb * 1024 * 1024;
    info!(scratch_mb, "guest memory");
    let mut cfg = SandboxConfiguration::default();
    cfg.set_scratch_size(scratch_size);
    cfg.set_heap_size(HEAP_SIZE);

    cfg.set_input_data_size(IO_STACK_SIZE);
    cfg.set_output_data_size(IO_STACK_SIZE);

    // Permit the guest to touch the MSRs the Unikraft kernel programs
    apply_guest_msrs(&mut cfg)?;

    let guest_binary = match kernel {
        Some(path) => {
            info!(path = %path.display(), "booting external kernel (advanced)");
            GuestBinary::FilePath(path.clone())
        }
        None => GuestBinary::Buffer(KERNEL.to_vec()),
    };
    let mut usandbox = UninitializedSandbox::new(guest_binary, Some(cfg))?;

    let (initrd_base, initrd_size) = if let Some(path) = initrd {
        let size = usandbox.map_file_cow(path, INITRD_MAP_BASE)?;
        info!(
            path = %path.display(),
            size,
            gpa = format_args!("{INITRD_MAP_BASE:#x}"),
            "mapped initrd",
        );
        (INITRD_MAP_BASE, size)
    } else {
        (0, 0)
    };

    let entry = resolve_entry(entry, initrd);

    // Build the kernel command line.
    //
    // Unikraft's uklibparam parser requires a `--` separator between
    // kernel parameters (like vfs.fstab) and application arguments
    // (like the entry point path).  Without `--`, uklibparam skips
    // parsing entirely and cmdline parameters are silently ignored.
    //
    // Layout: <progname> [kernel params...] -- [entry point]
    let mut cmdline = "unikraft-hyperlight".to_string();

    cmdline.push_str(&fstab_arg(&mounts)?);

    // Entry point path.  The `--` separator is needed only when there
    // are kernel params (like vfs.fstab) before it — uklibparam strips
    // everything up to `--` and adjusts argv so the elfloader sees the
    // driver path at argv[1].  Without kernel params, skip `--` so
    // argv[1] is the path directly (uklibparam's scan returns 0 for a
    // leading `--` and skips adjustment).
    if let Some(e) = &entry {
        if !mounts.is_empty() {
            write!(cmdline, " -- {e}").unwrap();
        } else {
            write!(cmdline, " {e}").unwrap();
        }
    }

    let config = GuestConfig::new(
        cmdline,
        scratch_size,
        initrd_base,
        initrd_size,
        mounts,
        network,
        listen_ports,
    )
    .with_host_functions(host_functions);

    config.register(&mut usandbox)?;

    debug!(cmdline = %config.cmdline, "sandbox created");

    Ok((usandbox, config))
}

/// Builds and boots a sandbox.
///
/// Pick a source — [`from_initrd`](Self::from_initrd) (a CPIO rootfs on the
/// embedded kernel, the usual case), [`from_kernel`](Self::from_kernel) (a
/// self-contained external kernel), or [`from_snapshot`](Self::from_snapshot)
/// (resume a saved guest) — chain the settings you need, then
/// [`boot`](Self::boot).  Everything but the source has a default, so
/// `SandboxBuilder::from_initrd(path).boot()` is a complete call.
///
/// [`boot`](Self::boot) brings the guest to a running state (evolving a fresh
/// guest, or restoring a snapshot) and hands back a [`AppSandbox`].
///
/// ```no_run
/// use hyperlight_unikraft::{SandboxBuilder, Mount, NetworkPolicy};
///
/// let mut sandbox = SandboxBuilder::from_initrd("rootfs/python.cpio")
///     .scratch_mb(256)
///     .mount(Mount::ro("/data", "/mnt/data"))
///     .network(NetworkPolicy::AllowAll)
///     .env("GREETING", "hi")
///     .boot()?;
/// sandbox.run("import os; print(os.environ['GREETING'])")?;
/// # Ok::<(), hyperlight_unikraft::Error>(())
/// ```
pub struct SandboxBuilder {
    kernel: Option<PathBuf>,
    initrd: Option<PathBuf>,
    entry: Option<String>,
    scratch_mb: Option<usize>,
    /// When set, [`boot`](Self::boot) restores this snapshot instead of
    /// booting a fresh guest; `kernel`/`initrd`/`entry`/`scratch_mb` are
    /// then an error (the snapshot carries them).
    snapshot: Option<Arc<Snapshot>>,
    mounts: Vec<Mount>,
    network: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    env_vars: Vec<(String, String)>,
    resolv_conf: Option<String>,
    host_functions: std::collections::BTreeMap<String, HostFunction>,
}

impl SandboxBuilder {
    /// A builder with no source and every setting at its default.
    fn empty() -> Self {
        Self {
            kernel: None,
            initrd: None,
            entry: None,
            scratch_mb: None,
            snapshot: None,
            mounts: Vec::new(),
            network: None,
            listen_ports: None,
            env_vars: Vec::new(),
            resolv_conf: None,
            host_functions: std::collections::BTreeMap::new(),
        }
    }

    /// Boot the embedded kernel with `path` — a CPIO archive — as the guest
    /// rootfs.  The usual entry point: the workload lives in the rootfs.
    pub fn from_initrd(path: impl Into<PathBuf>) -> Self {
        Self {
            initrd: Some(path.into()),
            ..Self::empty()
        }
    }

    /// Boot an external kernel that carries its own workload — a native
    /// app-in-kernel build, or a locally built `elfloader_hyperlight-x86_64`
    /// for kernel development.
    ///
    /// **Advanced.** The embedded kernel is the only combination this crate is
    /// tested against; an external kernel must match the ABI the host expects
    /// (PEB layout, load/base address, and the host-function set the drivers
    /// rely on) or the guest faults at boot.  Add an [`initrd`](Self::initrd)
    /// if the external kernel also wants a rootfs.
    pub fn from_kernel(path: impl Into<PathBuf>) -> Self {
        Self {
            kernel: Some(path.into()),
            ..Self::empty()
        }
    }

    /// Resume a guest from a saved snapshot instead of booting a fresh one.
    /// The snapshot carries the guest's cmdline/initrd/layout, so the
    /// kernel/initrd/entry/scratch settings do not apply here.
    ///
    /// The guest re-establishes its own host sockets on its first step: a
    /// listener is bound again (its port must be in this sandbox's
    /// [`listen_ports`](Self::listen_ports)), and connections whose peers
    /// died with the old host read as closed.  Nothing needs to be saved
    /// beside the snapshot.
    ///
    /// The [`mount`](Self::mount)s given here are the restored guest's: on
    /// its `resume` entry the kernel makes its mount table match them, so a
    /// snapshot saved without mounts serves any mount set.
    pub fn from_snapshot(snapshot: Arc<Snapshot>) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..Self::empty()
        }
    }

    /// [`from_snapshot`](Self::from_snapshot) with the snapshot read from
    /// `dir`, where [`AppSandbox::snapshot_to`] wrote it.
    pub fn from_snapshot_dir(dir: impl AsRef<Path>) -> Result<Self> {
        Ok(Self::from_snapshot(load_snapshot(dir)?))
    }

    /// CPIO rootfs to map for the guest (see [`from_initrd`](Self::from_initrd)).
    pub fn initrd(mut self, path: impl Into<PathBuf>) -> Self {
        self.initrd = Some(path.into());
        self
    }

    /// Swap in an external kernel (see [`from_kernel`](Self::from_kernel) for
    /// the ABI caveats).
    pub fn kernel(mut self, path: impl Into<PathBuf>) -> Self {
        self.kernel = Some(path.into());
        self
    }

    /// Override the auto-detected guest entry point.
    pub fn entry(mut self, entry: impl Into<String>) -> Self {
        self.entry = Some(entry.into());
        self
    }

    /// Scratch memory in MiB.  Without it, [`default_scratch_mb`] of the
    /// initrd, or [`DEFAULT_SCRATCH_MB`] for a kernel with none.
    pub fn scratch_mb(mut self, mb: usize) -> Self {
        self.scratch_mb = Some(mb);
        self
    }

    /// Add one host-directory mount.
    pub fn mount(mut self, mount: Mount) -> Self {
        self.mounts.push(mount);
        self
    }

    /// Add several host-directory mounts.
    pub fn mounts(mut self, mounts: impl IntoIterator<Item = Mount>) -> Self {
        self.mounts.extend(mounts);
        self
    }

    /// Enable host networking under the given policy.
    pub fn network(mut self, policy: NetworkPolicy) -> Self {
        self.network = Some(policy);
        self
    }

    /// Ports the guest may bind for inbound connections (requires a network policy).
    pub fn listen_ports(mut self, ports: ListenPorts) -> Self {
        self.listen_ports = Some(ports);
        self
    }

    /// Set a guest environment variable (repeatable).
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.push((key.into(), value.into()));
        self
    }

    /// The guest's `/etc/resolv.conf`: the kernel writes `content` there at
    /// boot, and again on a restore, so a snapshot resolves names where it
    /// now runs rather than where it was taken.  Without this the rootfs's
    /// own file stands.  `options single-request` is added unless present,
    /// since parallel A and AAAA queries do not work through the socket
    /// layer.
    pub fn resolv_conf(mut self, content: impl Into<String>) -> Self {
        self.resolv_conf = Some(content.into());
        self
    }

    /// Offer the guest a function of the host's, by name: the way to give
    /// code in the sandbox one capability rather than a directory or the
    /// network.  It takes the guest's argument text and returns its result
    /// text, or an error message the guest raises; the drivers pass JSON.
    /// JavaScript calls it as `host.call(name, ...args)` (quickjs, node),
    /// and in quickjs `a.b` is also the export `b` of the module `host:a`;
    /// Python as `hyperlight.call(name, *args)` or `hyperlight.host.a.b()`
    /// (python, python-shell, agent); C# as `Host.Call(name, args)`
    /// (dotnet-jit); the wasmtime image satisfies a module's or a
    /// component's import `a.b` with it.
    ///
    /// ```no_run
    /// # use hyperlight_unikraft::SandboxBuilder;
    /// let mut sandbox = SandboxBuilder::from_initrd("rootfs/quickjs.cpio")
    ///     .host_function("math.add", |args| {
    ///         let [a, b]: [f64; 2] = serde_json::from_str(args).map_err(|e| e.to_string())?;
    ///         Ok((a + b).to_string())
    ///     })
    ///     .boot()?;
    /// sandbox.run("import { add } from 'host:math'; console.log(add(2, 3))")?;
    /// # Ok::<(), hyperlight_unikraft::Error>(())
    /// ```
    ///
    /// Registered with the sandbox, not the guest, so a guest restored from
    /// a snapshot calls the functions of the builder that restores it.
    pub fn host_function<F>(mut self, name: impl Into<String>, function: F) -> Self
    where
        F: Fn(&str) -> std::result::Result<String, String> + Send + Sync + 'static,
    {
        self.host_functions.insert(name.into(), Arc::new(function));
        self
    }

    /// Register the host functions, bring the guest to a running state, and
    /// return it as a [`AppSandbox`].
    ///
    /// A fresh guest ([`from_initrd`](Self::from_initrd) /
    /// [`from_kernel`](Self::from_kernel)) is evolved (booted); a
    /// [`from_snapshot`](Self::from_snapshot) source is restored.
    pub fn boot(self) -> Result<AppSandbox> {
        let Self {
            kernel,
            initrd,
            entry,
            scratch_mb,
            snapshot,
            mounts,
            network,
            listen_ports,
            env_vars,
            resolv_conf,
            host_functions,
        } = self;
        // The empty name asks for the list of functions, one per line, so
        // neither it nor a line break can be in a function's name.
        if let Some(name) = host_functions
            .keys()
            .find(|n| n.is_empty() || n.contains(['\n', '\r']))
        {
            return Err(Error::HostFunctionName { name: name.clone() });
        }
        let host_functions: HostFunctionTable = Arc::new(host_functions);

        let restored = snapshot.is_some();
        if restored
            && (kernel.is_some() || initrd.is_some() || entry.is_some() || scratch_mb.is_some())
        {
            return Err(Error::SnapshotSettings);
        }
        let env_refs: Vec<(&str, &str)> = env_vars
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        // A guest that may reach only listed names must still be able to
        // ask the resolver it was given.
        if let (Some(rc), Some(NetworkPolicy::AllowList(al))) = (&resolv_conf, &network) {
            al.exempt_resolvers(net_policy::resolv_conf_nameservers(rc));
        }
        let (sandbox, cfg) = match snapshot {
            Some(snapshot) => {
                let (sandbox, cfg) =
                    restore_snapshot(snapshot, mounts, network, listen_ports, host_functions)?;
                cfg.set_env_vars(&env_refs);
                // Read by the resume entry below, which rewrites the file.
                if let Some(rc) = &resolv_conf {
                    cfg.set_resolv_conf(rc);
                }
                (sandbox, cfg)
            }
            None => {
                // Scan the initrd once for the driver entry point and
                // the optional runtime marker; the result sizes the
                // scratch memory and resolves the entry so
                // `assemble_sandbox` does not scan a second time.
                let scan = if scratch_mb.is_none() || entry.is_none() {
                    initrd.as_deref().map(scan_cpio).unwrap_or_default()
                } else {
                    CpioScan::default()
                };
                let scratch = scratch_mb.unwrap_or_else(|| {
                    scan.runtime
                        .as_deref()
                        .and_then(runtime_scratch_mb)
                        .or_else(|| {
                            scan.entry
                                .as_deref()
                                .and_then(|p| p.rsplit('/').next())
                                .and_then(driver_scratch_mb)
                        })
                        .unwrap_or(DEFAULT_SCRATCH_MB)
                });
                let entry = entry.or_else(|| {
                    if let Some(ref d) = scan.entry {
                        info!(entry = %d, "auto-detected driver entry point");
                    }
                    scan.entry
                });
                let (usandbox, cfg) = assemble_sandbox(
                    &kernel,
                    &initrd,
                    &entry,
                    scratch,
                    mounts,
                    network,
                    listen_ports,
                    host_functions,
                )?;
                // Before the boot: the kernel fetches the environment once
                // on its way to main(), so an entry-point program starts
                // with these; a driver refreshes them on every call anyway.
                cfg.set_env_vars(&env_refs);
                // Likewise fetched once the rootfs is mounted, before main().
                if let Some(rc) = &resolv_conf {
                    cfg.set_resolv_conf(rc);
                }
                let sandbox = match usandbox.evolve() {
                    Ok(sandbox) => sandbox,
                    Err(e) => {
                        // The guest's last words are the diagnosis.
                        let output = cfg.drain_output();
                        if !output.is_empty() {
                            tracing::error!(%output, "guest console output before the boot failure");
                        }
                        return Err(e.into());
                    }
                };
                (sandbox, cfg)
            }
        };

        // A fresh guest has spoken during boot: a step-model kernel reports
        // boot complete, and a process that ran to completion meanwhile (a
        // program as the entry point, a native kernel whose main()
        // returned) reports its exit status on the way down.  A restored
        // guest says nothing until its resume entry below.
        let exited = if restored {
            None
        } else {
            match cfg.absorb()? {
                exit @ Yield::Exited { .. } => Some(exit),
                _ => None,
            }
        };
        let mut app = AppSandbox {
            sandbox,
            config: cfg,
            exited,
            pending: None,
        };
        if restored {
            // Put the image right for this host before anyone can observe
            // it: the kernel reseeds its CSPRNG and re-establishes the
            // guest's host sockets (see `resume`).
            app.resume()?;
        }
        Ok(app)
    }
}

/// Whether a `resolv.conf` line is an `options` line carrying the
/// `single-request` option itself: as a token, so `single-request-reopen`
/// (another option) does not count, and not in a comment.
fn options_line_has_single_request(line: &str) -> bool {
    let line = line.trim_start();
    line.starts_with("options")
        && line
            .split_whitespace()
            .skip(1)
            .any(|t| t == "single-request")
}

/// `content` with `options single-request` in it: on its own options line if
/// it has one, else on a new line.  Empty content stays empty (the rootfs's
/// file stands).  glibc's parallel A and AAAA queries do not work through
/// the socket layer, and the option makes them sequential.
fn with_single_request(content: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    if !lines.is_empty() && !lines.iter().any(|l| options_line_has_single_request(l)) {
        match lines
            .iter_mut()
            .find(|l| l.trim_start().starts_with("options"))
        {
            Some(options) => options.push_str(" single-request"),
            None => lines.push("options single-request".into()),
        }
    }
    let mut s = lines.join("\n");
    if !s.is_empty() {
        s.push('\n');
    }
    s
}

/// What to execute in the guest.
#[derive(Debug, Clone)]
pub enum Exec {
    /// Inline code string — passed to the guest's dispatch callback.
    Code(String),
    /// Script file — read to string and passed to the guest's dispatch callback.
    File(PathBuf),
    /// Run a command that already lives in the guest filesystem: a path plus
    /// optional args (e.g. `"/app/server --port 8080"`). The driver runs the
    /// named guest file with that argv — the exec driver `execv`s it, the
    /// interpreters run it as a script with `sys.argv`/`process.argv` set.
    ///
    /// This is how a container runtime (urunc) drives the guest: the app is
    /// baked into the initrd and its command comes from the image, matching
    /// how every other urunc VMM passes the app's command line. An empty
    /// command means "run the conventional entrypoint" (`/entrypoint.py`,
    /// `/entrypoint`, …). Dispatched at the same point as any other `Exec`, so
    /// it works identically on a fresh boot or a restored snapshot.
    Guest(String),
    /// Call the function `function` the guest has defined, with `input`,
    /// for its result: in the quickjs and node images a global function,
    /// in the python ones a function of `__main__`, in dotnet-jit a public
    /// static method, called with `input` parsed as JSON and its return
    /// value (awaited) serialized back; in the wasmtime image an export of
    /// the module or component the guest has loaded, `input` a JSON array
    /// of its arguments.  Other images fail the call.  [`AppSandbox::call`] waits for the result;
    /// [`AppSandbox::take_result`] collects it after a
    /// [`submit`](AppSandbox::submit).
    Call { function: String, input: String },
}

impl From<&str> for Exec {
    fn from(code: &str) -> Self {
        Exec::Code(code.to_string())
    }
}

impl From<String> for Exec {
    fn from(code: String) -> Self {
        Exec::Code(code)
    }
}

// ── AppSandbox ──────────────────────────────────────────────────────────

/// A booted guest: the Hyperlight sandbox and the host-side state its host
/// functions share, as one handle.  Returned by [`SandboxBuilder::boot`].
///
/// Between calls the guest is always parked at a *boundary*: every guest
/// thread is blocked and the vCPU is halted.  Each method here enters the
/// VM, lets the guest scheduler run until it blocks again, and returns at
/// the next boundary.  A [`snapshot`](Self::snapshot) taken at any
/// boundary resumes exactly there.
///
/// Two kinds of workload:
///
/// * A *driver* image (the Python, Node, … rootfs) boots a runtime that
///   waits for calls.  [`run`](Self::run) dispatches one and waits for it;
///   [`submit`](Self::submit) dispatches one and hands the boundaries in
///   between to the caller through [`step`](Self::step).
/// * An *entry-point* image (`--entry /bin/server`) is the workload
///   itself.  There is nothing to dispatch; [`join`](Self::join) keeps it
///   going until it exits.
///
/// A guest restored from a snapshot is announced to the kernel with a
/// `resume` entry before [`SandboxBuilder::boot`] (or
/// [`restore`](Self::restore)) returns.  The kernel reseeds its CSPRNG
/// there, so two guests restored from the same image do not draw the same
/// random bytes, and re-establishes its host sockets, so a server keeps
/// its listener and sees its old connections as closed.
///
/// [`snapshot`](Self::snapshot) captures the guest at the current
/// boundary and [`snapshot_to`](Self::snapshot_to) writes it to disk;
/// [`SandboxBuilder::from_snapshot`] / [`from_snapshot_dir`](SandboxBuilder::from_snapshot_dir)
/// and [`restore`](Self::restore) / [`restore_from`](Self::restore_from)
/// bring it back.  Dropping the sandbox tears
/// the VM down and releases every host socket it held.
pub struct AppSandbox {
    sandbox: MultiUseSandbox,
    config: GuestConfig,
    /// The [`Yield::Exited`] the guest process ended with, once it has;
    /// every later step reports it again.
    exited: Option<Yield>,
    /// A terminal result produced while delivering a call, handed out by
    /// the next [`step`](Self::step).
    pending: Option<Yield>,
}

impl AppSandbox {
    /// Execute code or a script file in the guest and wait for it to finish.
    ///
    /// Accepts inline code (`"print('hi')"`), a file path
    /// (`Exec::File("hello.py".into())`) or a guest command
    /// ([`Exec::Guest`]).  The call is served by the guest's driver on its
    /// own thread; meanwhile the host parks on the guest's timers and
    /// sockets, costing no CPU, until the driver reports the call done.
    ///
    /// ```no_run
    /// # use hyperlight_unikraft::SandboxBuilder;
    /// let mut sandbox = SandboxBuilder::from_initrd("rootfs/python.cpio").boot()?;
    /// sandbox.run("import time; time.sleep(2); print('later')")?;  // ~2 s, VM halted meanwhile
    /// assert!(sandbox.drain_output().contains("later"));
    /// # Ok::<(), hyperlight_unikraft::Error>(())
    /// ```
    ///
    /// Errors if the guest has no driver ([`Error::NoDriver`]), if the
    /// driver reports the call failed ([`Error::CallFailed`], an uncaught
    /// exception say), if the guest exits before the call returns
    /// ([`Error::GuestExited`]), or if the guest deadlocks
    /// ([`Error::Deadlocked`]): every thread blocked, no timer pending and
    /// no host socket that could wake it.
    pub fn run(&mut self, exec: impl Into<Exec>) -> Result<()> {
        self.submit(exec)?;
        loop {
            match self.step_with(None)? {
                Yield::CallDone => return Ok(()),
                Yield::CallFailed { status } => return Err(Error::CallFailed { status }),
                Yield::Exited { status } => return Err(Error::GuestExited { status }),
                Yield::Blocked { .. } => {}
            }
        }
    }

    /// Call a function the guest has defined and wait for its result: a
    /// handler loaded once (by a [`run`](Self::run), or in a warm snapshot)
    /// and called many times with different input.  See [`Exec::Call`] for
    /// what each image does with `function` and `input`.
    ///
    /// ```no_run
    /// # use hyperlight_unikraft::SandboxBuilder;
    /// let mut sandbox = SandboxBuilder::from_initrd("rootfs/quickjs.cpio").boot()?;
    /// sandbox.run("function greet(event) { return { message: 'Hello, ' + event.name } }")?;
    /// let out = sandbox.call("greet", r#"{"name":"World"}"#)?;
    /// assert_eq!(out, r#"{"message":"Hello, World"}"#);
    /// # Ok::<(), hyperlight_unikraft::Error>(())
    /// ```
    ///
    /// Errors as [`run`](Self::run) does, and with
    /// [`Error::ResultNotText`] when the result is not UTF-8.
    pub fn call(&mut self, function: &str, input: &str) -> Result<String> {
        self.run(Exec::Call {
            function: function.to_string(),
            input: input.to_string(),
        })?;
        let bytes = self.take_result().unwrap_or_default();
        String::from_utf8(bytes).map_err(|e| {
            // Put it back, so take_result hands it over as bytes.
            *self.config.result.lock().unwrap() = Some(e.into_bytes());
            Error::ResultNotText
        })
    }

    /// The result of the last call, if its driver sent one: for a call
    /// submitted with [`submit`](Self::submit) and driven with
    /// [`step`](Self::step) to [`Yield::CallDone`].  Taken, so a second
    /// take returns `None`.
    pub fn take_result(&self) -> Option<Vec<u8>> {
        self.config.result.lock().unwrap().take()
    }

    /// Keep the guest running until its process exits, and return its exit
    /// status.
    ///
    /// For an entry-point workload, where the program is the guest's PID 1
    /// and there is nothing to dispatch:
    ///
    /// ```no_run
    /// # use hyperlight_unikraft::{ListenPorts, NetworkPolicy, SandboxBuilder};
    /// let mut sandbox = SandboxBuilder::from_initrd("rootfs/counter.cpio")
    ///     .entry("/bin/server --port 80")
    ///     .network(NetworkPolicy::AllowAll)
    ///     .listen_ports(ListenPorts::from_ports([80]))
    ///     .boot()?;                 // returns at the server's first accept()
    /// let status = sandbox.join()?; // returns when the server process exits
    /// # Ok::<(), hyperlight_unikraft::Error>(())
    /// ```
    ///
    /// Errors at once on a driver image with no call in flight
    /// ([`Error::NothingToJoin`]): a driver waiting for calls never exits
    /// on its own, so the join could never return.  Use [`run`](Self::run)
    /// or [`submit`](Self::submit) there.  Also errors if the guest
    /// deadlocks ([`Error::Deadlocked`], see [`run`](Self::run)).
    pub fn join(&mut self) -> Result<i32> {
        if self.exited.is_none() && self.has_driver() && !self.config.call_in_flight() {
            return Err(Error::NothingToJoin);
        }
        loop {
            match self.step_with(None)? {
                Yield::Exited { status } => return Ok(status),
                // A driver's call finished, but a driver never exits on its
                // own, so there is nothing more to join -- report it rather
                // than loop forever on an idle driver.
                Yield::CallDone | Yield::CallFailed { .. } if self.has_driver() => {
                    return Err(Error::NothingToJoin);
                }
                Yield::CallDone | Yield::CallFailed { .. } | Yield::Blocked { .. } => {}
            }
        }
    }

    /// Hand the driver a call without waiting for it to finish.
    ///
    /// The call is delivered by one VM entry, so the guest runs until it
    /// blocks before this returns.  From then on drive it with
    /// [`step`](Self::step); [`Yield::CallDone`] marks the return.  This is
    /// how to keep the boundaries for yourself: to talk to a server the
    /// script started, or to checkpoint in the middle of the call.
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// # use hyperlight_unikraft::{SandboxBuilder, Yield};
    /// # let mut sandbox = SandboxBuilder::from_initrd("rootfs/python.cpio").boot()?;
    /// sandbox.submit("import select; print('a'); select.select([], [], [], 1); print('b')")?;
    /// // 'a' is printed and the script is parked on its one-second wait.
    /// let y = sandbox.step(Duration::from_secs(5))?;
    /// // Waited ~1 s (VM halted), re-entered, 'b' printed, script returned.
    /// assert_eq!(y, Yield::CallDone);
    /// # Ok::<(), hyperlight_unikraft::Error>(())
    /// ```
    ///
    /// Errors if the guest has exited ([`Error::GuestExited`]), has no
    /// driver ([`Error::NoDriver`]), or already has a call in flight
    /// ([`Error::CallInFlight`]: the kernel serves one at a time).
    pub fn submit(&mut self, exec: impl Into<Exec>) -> Result<()> {
        if let Some(Yield::Exited { status }) = self.exited {
            return Err(Error::GuestExited { status });
        }
        // The kernel would reject it too (nothing reads /dev/hlcall);
        // refusing here is earlier and says why.
        if !self.has_driver() {
            return Err(Error::NoDriver);
        }
        if self.config.call_in_flight() {
            return Err(Error::CallInFlight);
        }
        // A finished outcome still waiting for a step (the call a restored
        // snapshot had in flight, finishing on its resume) is not this
        // call's: it must not end this call's wait.
        if matches!(
            self.pending,
            Some(Yield::CallDone | Yield::CallFailed { .. })
        ) {
            self.pending = None;
        }
        *self.config.result.lock().unwrap() = None;
        let yielded = match exec.into() {
            Exec::Code(code) => self.config.enter(&mut self.sandbox, "Exec", code)?,
            Exec::File(path) => {
                let code = std::fs::read_to_string(&path).map_err(|source| Error::Script {
                    path: path.clone(),
                    source,
                })?;
                self.config.enter(&mut self.sandbox, "Exec", code)?
            }
            // Dispatched under its own name so the driver runs the named
            // guest file (empty command → its conventional entrypoint)
            // rather than treating the payload as inline code.
            Exec::Guest(cmd) => self.config.enter(&mut self.sandbox, "GuestExec", cmd)?,
            Exec::Call { function, input } => {
                self.config
                    .enter(&mut self.sandbox, "Call", (function, input))?
            }
        };
        match self.note(yielded) {
            Yield::Blocked { .. } => {}
            terminal => self.pending = Some(terminal),
        }
        Ok(())
    }

    /// Announce a restore to the kernel with one `resume` entry, on which
    /// it reseeds its CSPRNG and re-establishes the guest's host sockets.
    /// Like a step, it runs the scheduler to the next boundary, so a
    /// terminal outcome (a call in flight at snapshot time finishing, or
    /// the process exiting) is kept for the next [`step`](Self::step).
    fn resume(&mut self) -> Result<()> {
        let yielded = self.config.enter(&mut self.sandbox, "resume", ())?;
        match self.note(yielded) {
            Yield::Blocked { .. } => {}
            terminal => self.pending = Some(terminal),
        }
        Ok(())
    }

    /// Remember an exit; everything else needs no bookkeeping.
    fn note(&mut self, yielded: Yield) -> Yield {
        if let Yield::Exited { .. } = yielded {
            self.exited = Some(yielded);
        }
        yielded
    }

    /// Advance the guest by one step.
    ///
    /// Waits until the guest may be runnable again (its next timer is due,
    /// or one of its host sockets is readable), at most `timeout`; then
    /// enters the VM, lets the scheduler run until every guest thread is
    /// blocked, and returns why it stopped.  If `timeout` runs out first
    /// nothing has changed, and the last [`Yield::Blocked`] is returned
    /// without entering, so `Duration::ZERO` is a non-blocking probe.
    ///
    /// The VM is halted for the whole wait; the host thread sits in
    /// `poll(2)`.  [`run`](Self::run) and [`join`](Self::join) are loops
    /// over this with no bound: they wait until a timer or a socket wakes
    /// the guest, and fail if nothing could.
    ///
    /// Errors if the guest is deadlocked ([`Error::Deadlocked`]): nothing
    /// can wake it and something is owed that only it can produce (see
    /// [`run`](Self::run)).
    pub fn step(&mut self, timeout: Duration) -> Result<Yield> {
        self.step_with(Some(timeout))
    }

    /// [`step`](Self::step) with an optional bound; `None` waits until the
    /// guest can run, however long that takes, and returns at once when
    /// nothing could ever wake it (see [`GuestConfig::can_wake`]).
    fn step_with(&mut self, timeout: Option<Duration>) -> Result<Yield> {
        if let Some(y) = self.pending.take() {
            return Ok(y);
        }
        if let Some(exit) = self.exited {
            return Ok(exit);
        }
        // A guest nothing can wake -- no timer, no live host socket -- while
        // something is owed that only it can produce (a call's return, or
        // an entry-point workload's exit) is stuck for good: waiting any
        // longer, for any timeout, would be waiting forever.  A driver
        // idle between calls is not that; the host wakes it by submitting.
        if !self.config.can_wake() && (self.config.call_in_flight() || !self.config.has_driver()) {
            return Err(Error::Deadlocked);
        }
        // Wait for the guest to become runnable.
        if !self.config.wait_runnable(timeout) {
            return Ok(Yield::Blocked {
                until: self.config.next_wakeup_at(),
            });
        }
        let yielded = self.config.enter(&mut self.sandbox, "step", ())?;
        Ok(self.note(yielded))
    }

    /// Capture the guest at the current boundary: every parked thread, the
    /// scheduler queues, the application heap.  A guest resumed from it
    /// (in this process with [`restore`](Self::restore), or anywhere with
    /// [`SandboxBuilder::from_snapshot`]) picks up exactly here, in the
    /// middle of a call or not, and re-establishes its host sockets by
    /// itself.
    ///
    /// Errors with [`Error::GuestExited`] once the guest process has
    /// exited: there is nothing left to resume.
    pub fn snapshot(&mut self) -> Result<Arc<Snapshot>> {
        if let Some(Yield::Exited { status }) = self.exited {
            return Err(Error::GuestExited { status });
        }
        Ok(self.sandbox.snapshot()?)
    }

    /// [`snapshot`](Self::snapshot), then [`save_snapshot`] it to `dir`.
    /// The snapshot is also returned, for use in this process.
    pub fn snapshot_to(&mut self, dir: impl AsRef<Path>) -> Result<Arc<Snapshot>> {
        let snapshot = self.snapshot()?;
        save_snapshot(&snapshot, dir)?;
        Ok(snapshot)
    }

    /// Restore a snapshot into this sandbox in place and put the restored
    /// guest right for this host with a `resume` entry, as
    /// [`SandboxBuilder::boot`] does, with this sandbox's mounts.
    pub fn restore(&mut self, snapshot: Arc<Snapshot>) -> Result<()> {
        self.sandbox.restore(snapshot)?;
        // The host sockets belong to the guest state just discarded; the
        // restored guest re-creates the ones it holds on its resume entry.
        if let Some(net) = &self.config.net {
            net.reset();
        }
        self.exited = None;
        self.pending = None;
        self.config.forget();
        self.resume()
    }

    /// [`restore`](Self::restore) with the snapshot read from `dir`, where
    /// [`snapshot_to`](Self::snapshot_to) wrote it.
    pub fn restore_from(&mut self, dir: impl AsRef<Path>) -> Result<()> {
        self.restore(load_snapshot(dir)?)
    }

    /// Whether a runtime driver in the guest serves calls, so whether
    /// [`run`](Self::run) and [`submit`](Self::submit) can work.  False for
    /// an entry-point image, which is driven with [`join`](Self::join).
    /// Known once [`SandboxBuilder::boot`] returns, for a fresh and a
    /// restored guest alike.
    pub fn has_driver(&self) -> bool {
        self.config.has_driver()
    }

    /// Take the guest output captured so far.
    pub fn drain_output(&self) -> String {
        self.config.drain_output()
    }

    /// Set guest environment variables for the next call.
    pub fn set_env_vars(&self, vars: &[(&str, &str)]) {
        self.config.set_env_vars(vars)
    }

    /// A handle that can break an entry in progress from another thread
    /// (see [`hyperlight_host::hypervisor::InterruptHandle`]): the way out
    /// of a step that will not come back, at the cost of the sandbox.
    pub fn interrupt_handle(&self) -> Arc<dyn hyperlight_host::hypervisor::InterruptHandle> {
        self.sandbox.interrupt_handle()
    }
}

/// Restore a sandbox from a saved snapshot — the [`SandboxBuilder::from_snapshot`]
/// path.
///
/// Creates a default [`GuestConfig`] (the snapshot already has the guest's
/// cmdline/initrd), registers host functions, and rebuilds a
/// [`MultiUseSandbox`] from the snapshot.  `mounts` are the restored
/// guest's: the kernel reads them through `GetMounts` on its `resume`
/// entry and makes its mount table match.
fn restore_snapshot(
    snapshot: Arc<Snapshot>,
    mounts: Vec<Mount>,
    network: Option<NetworkPolicy>,
    listen_ports: Option<ListenPorts>,
    host_functions: HostFunctionTable,
) -> Result<(MultiUseSandbox, GuestConfig)> {
    let config = GuestConfig::new(
        String::new(),
        DEFAULT_SCRATCH_MB * 1024 * 1024,
        0,
        0,
        mounts,
        network,
        listen_ports,
    )
    .with_host_functions(host_functions);
    let mut hf = HostFunctions::default();
    config.register(&mut hf)?;

    // The restoring VM must declare the same MSRs as the saving VM: the
    // snapshot persists exactly those MSRs and restore validates them
    // against this set (see [`GUEST_MSRS`]).  from_snapshot takes the
    // layout sizes from the snapshot itself, so we only set the ones we
    // know to match (avoiding a spurious layout-override warning) and let
    // it override scratch.
    let mut sbcfg = SandboxConfiguration::default();
    sbcfg.set_input_data_size(IO_STACK_SIZE);
    sbcfg.set_output_data_size(IO_STACK_SIZE);
    sbcfg.set_heap_size(HEAP_SIZE);
    apply_guest_msrs(&mut sbcfg)?;

    let sandbox = MultiUseSandbox::from_snapshot(snapshot, hf, Some(sbcfg))?;
    Ok((sandbox, config))
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    /// OCI tags forbid `+`, so a `+build` suffix on the crate version would
    /// make every snapshot save panic.
    #[test]
    fn snapshot_tag_is_the_release_and_the_key() {
        let tag = snapshot_tag().to_string();
        assert_eq!(tag, format!("{}-{SNAPSHOT_KEY}", env!("CARGO_PKG_VERSION")));
        assert_eq!(snapshot_tag_key(&tag), Some(SNAPSHOT_KEY));
        assert_eq!(
            describe_snapshot_tag(&tag),
            format!("{} ({SNAPSHOT_KEY})", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn snapshot_tag_key_reads_only_a_keyed_tag() {
        assert_eq!(snapshot_tag_key("0.14.0"), None);
        assert_eq!(
            snapshot_tag_key("0.15.0-rc1-kdeadbeefdeadbeef-c2"),
            Some("kdeadbeefdeadbeef-c2")
        );
        assert_eq!(snapshot_tag_key("0.15.0-kappa-c2"), None);
        assert_eq!(describe_snapshot_tag("0.14.0"), "0.14.0");
    }

    #[test]
    fn paging_budget_is_75_percent() {
        let cfg = GuestConfig::new(
            String::new(),
            256 * 1024 * 1024,
            0,
            0,
            Vec::new(),
            None,
            None,
        );
        assert_eq!(cfg.paging_budget(), 192 * 1024 * 1024);
    }

    #[test]
    fn resolve_entry_prefers_explicit() {
        let explicit = Some("/custom/bin/myapp".to_string());
        let initrd = Some(PathBuf::from("/nonexistent/initrd.cpio"));
        assert_eq!(
            resolve_entry(&explicit, &initrd),
            Some("/custom/bin/myapp".to_string())
        );
    }

    // -- CPIO parsing --
    //
    // These test the internal CPIO scanner against synthetic archives
    // to verify it correctly finds driver binaries and handles edge
    // cases (no driver, data-heavy entries, empty archives).

    /// Build a minimal newc-format CPIO entry.
    fn cpio_entry(name: &str, data: &[u8]) -> Vec<u8> {
        let namesize = name.len() + 1; // include NUL
        let filesize = data.len();
        let header = format!(
            "070701\
             00000000\
             00000000\
             00000000\
             00000000\
             00000001\
             00000000\
             {:08X}\
             00000000\
             00000000\
             00000000\
             00000000\
             {:08X}\
             00000000",
            filesize, namesize,
        );
        assert_eq!(header.len(), 110);

        let mut buf = Vec::new();
        buf.extend_from_slice(header.as_bytes());
        buf.extend_from_slice(name.as_bytes());
        buf.push(0);
        let name_pad = (4 - ((110 + namesize) % 4)) % 4;
        buf.extend(std::iter::repeat_n(0u8, name_pad));
        buf.extend_from_slice(data);
        let data_pad = (4 - (filesize % 4)) % 4;
        buf.extend(std::iter::repeat_n(0u8, data_pad));
        buf
    }

    fn cpio_trailer() -> Vec<u8> {
        cpio_entry("TRAILER!!!", &[])
    }

    fn write_cpio(dir: &Path, name: &str, entries: &[(&str, &[u8])]) -> PathBuf {
        let path = dir.join(name);
        let mut f = File::create(&path).unwrap();
        for (entry_name, data) in entries {
            f.write_all(&cpio_entry(entry_name, data)).unwrap();
        }
        f.write_all(&cpio_trailer()).unwrap();
        path
    }

    fn test_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("hl-test-{label}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn find_cpio_entry_detects_drivers() {
        let dir = test_dir("cpio-drivers");
        // pydriver in usr/local/bin
        let cpio = write_cpio(&dir, "py.cpio", &[("usr/local/bin/hl_pydriver", b"ELF")]);
        assert_eq!(
            find_cpio_entry(&cpio),
            Some("/usr/local/bin/hl_pydriver".to_string())
        );

        // nodedriver in usr/bin
        let cpio = write_cpio(&dir, "node.cpio", &[("usr/bin/hl_nodedriver", b"ELF")]);
        assert_eq!(
            find_cpio_entry(&cpio),
            Some("/usr/bin/hl_nodedriver".to_string())
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn find_cpio_entry_skips_non_drivers() {
        let dir = test_dir("cpio-nodriver");
        let big_data = vec![0xABu8; 1024];
        let cpio = write_cpio(
            &dir,
            "test.cpio",
            &[("etc/big_config", &big_data), ("usr/bin/python3", b"ELF")],
        );
        assert_eq!(find_cpio_entry(&cpio), None);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn find_cpio_entry_skips_data_to_find_driver() {
        let dir = test_dir("cpio-skip-data");
        let big_data = vec![0xABu8; 4096];
        let cpio = write_cpio(
            &dir,
            "test.cpio",
            &[
                ("etc/config", &big_data),
                ("usr/local/bin/hl_pydriver", b"ELF"),
            ],
        );
        assert_eq!(
            find_cpio_entry(&cpio),
            Some("/usr/local/bin/hl_pydriver".to_string())
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    // -- Mount / fstab tests --

    #[test]
    fn mount_rw_host_first() {
        let m = Mount::rw("/host/dir", "/guest/path");
        assert_eq!(m.host_path, PathBuf::from("/host/dir"));
        assert_eq!(m.guest_path, "/guest/path");
        assert!(!m.readonly);
    }

    #[test]
    fn mount_ro_host_first() {
        let m = Mount::ro("/host/dir", "/guest/path");
        assert_eq!(m.host_path, PathBuf::from("/host/dir"));
        assert_eq!(m.guest_path, "/guest/path");
        assert!(m.readonly);
    }

    /// An OCI layout directory whose index names one manifest per tag.
    fn layout_with_tags(tags: &[&str]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("oci-layout"),
            r#"{"imageLayoutVersion":"1.0.0"}"#,
        )
        .unwrap();
        let manifests: Vec<String> = tags
            .iter()
            .map(|t| {
                format!(
                    r#"{{"mediaType":"application/vnd.oci.image.manifest.v1+json","digest":"sha256:0000000000000000000000000000000000000000000000000000000000000000","size":1,"annotations":{{"org.opencontainers.image.ref.name":"{t}"}}}}"#
                )
            })
            .collect();
        std::fs::write(
            dir.path().join("index.json"),
            format!(
                r#"{{"schemaVersion":2,"manifests":[{}]}}"#,
                manifests.join(",")
            ),
        )
        .unwrap();
        dir
    }

    fn load_error(dir: &Path) -> Error {
        match load_snapshot(dir) {
            Err(e) => e,
            Ok(_) => panic!("a snapshot with no blobs loaded"),
        }
    }

    #[test]
    fn a_snapshot_with_another_key_names_the_release_that_saved_it() {
        let dir = layout_with_tags(&["0.14.0", "0.0.1-k0000000000000000-c0"]);
        let err = load_error(dir.path());
        match &err {
            Error::SnapshotRelease { saved_by, this, .. } => {
                assert_eq!(saved_by, "0.14.0, 0.0.1 (k0000000000000000-c0)");
                assert_eq!(
                    *this,
                    format!("{} ({SNAPSHOT_KEY})", env!("CARGO_PKG_VERSION"))
                );
            }
            other => panic!("expected SnapshotRelease, got {other}"),
        }
        assert!(err.to_string().contains("save it again"));
    }

    #[test]
    fn a_snapshot_with_this_key_loads_whatever_release_saved_it() {
        // The tag matches on the key, so the load goes on to the layout
        // code, which fails on the missing blob rather than on the tag.
        let dir = layout_with_tags(&[&format!("0.0.1-{SNAPSHOT_KEY}")]);
        let err = load_error(dir.path());
        assert!(
            !matches!(err, Error::SnapshotRelease { .. }),
            "refused by key: {err}"
        );
        assert!(
            !err.to_string().contains("no manifest tagged"),
            "tag not matched: {err}"
        );
    }

    #[test]
    fn fstab_entries_are_the_arg_without_the_brackets() {
        let mounts = [Mount::rw("/a", "/mnt/a"), Mount::ro("/b", "/mnt/b")];
        let entries = fstab_entries(&mounts).unwrap();
        assert_eq!(
            entries,
            "0:/mnt/a:hostfs:0x0::mkmp 1:/mnt/b:hostfs:0x1::mkmp"
        );
        assert_eq!(
            fstab_arg(&mounts).unwrap(),
            format!(" vfs.fstab=[{entries}]")
        );
        assert_eq!(fstab_entries(&[]).unwrap(), "");
    }

    #[test]
    fn fstab_entries_stop_at_what_the_kernel_takes() {
        let many: Vec<Mount> = (0..=MOUNTS_MAX)
            .map(|i| Mount::rw("/tmp", format!("/mnt/{i}")))
            .collect();
        assert!(fstab_entries(&many[..MOUNTS_MAX]).is_ok());
        assert!(matches!(
            fstab_entries(&many),
            Err(Error::MountTable { mounts, .. }) if mounts == MOUNTS_MAX + 1
        ));
        let long = [Mount::rw(
            "/tmp",
            format!("/mnt/{}", "x".repeat(FSTAB_ENTRIES_MAX)),
        )];
        assert!(matches!(
            fstab_entries(&long),
            Err(Error::MountTable { .. })
        ));
    }

    #[test]
    fn fstab_single_rw_mount() {
        let arg = fstab_arg(&[Mount::rw("/tmp/share", "/mnt/host")]).unwrap();
        assert_eq!(arg, " vfs.fstab=[0:/mnt/host:hostfs:0x0::mkmp]");
    }

    #[test]
    fn fstab_no_mounts_is_empty() {
        assert_eq!(fstab_arg(&[]).unwrap(), "");
    }

    #[test]
    fn fstab_rejects_a_path_the_kernel_would_misparse() {
        for bad in ["relative", "/mnt/a b", "/mnt/a:b", "/mnt/[a]"] {
            assert!(
                fstab_arg(&[Mount::rw("/tmp", bad)]).is_err(),
                "{bad:?} was accepted"
            );
        }
    }

    /// Reproduce the exact byte output of the C `fb_encode_generic` encoder
    /// for `fs_stat(0, "hello.txt")` and verify Hyperlight can parse it.
    #[test]
    fn flatbuffer_generic_encoder_roundtrip() {
        use hyperlight_common::flatbuffer_wrappers::function_call::FunctionCall;

        // Construct the exact bytes the C fb_encode_generic would produce
        // for hl_hcall_vecbytes("fs_stat", [hlint(0), hlstring("hello.txt")], 2).
        //
        // This replicates the logic in hcall.c fb_encode_generic.
        let c_bytes = build_c_generic_fb(
            "fs_stat",
            2, // HL_FCT_HOST
            9, // HL_RT_VECBYTES (= hlsizeprefixedbuffer)
            &[CParam::Int(0), CParam::Str("hello.txt")],
        );

        eprintln!("C-encoded ({} bytes):", c_bytes.len());
        for (i, chunk) in c_bytes.chunks(16).enumerate() {
            eprint!("  {:04x}:", i * 16);
            for b in chunk {
                eprint!(" {:02x}", b);
            }
            eprintln!();
        }

        // Try to parse the C-style bytes.
        let c_parsed = FunctionCall::try_from(c_bytes.as_slice());
        assert!(
            c_parsed.is_ok(),
            "C-encoded FunctionCall should parse: {:?}",
            c_parsed.err()
        );
        let c_parsed = c_parsed.unwrap();
        assert_eq!(c_parsed.function_name, "fs_stat");
    }

    /// Roundtrip test for fs_write_bytes with empty VecBytes.
    #[test]
    fn flatbuffer_generic_encoder_roundtrip_write() {
        use hyperlight_common::flatbuffer_wrappers::function_call::FunctionCall;

        let c_bytes = build_c_generic_fb(
            "fs_write_bytes",
            2, // HL_FCT_HOST
            0, // HL_RT_INT
            &[
                CParam::Int(0),             // mount_idx
                CParam::Str("written.txt"), // path
                CParam::ULong(0),           // offset
                CParam::Int(0),             // append
                CParam::VecBytes(&[]),      // empty data
            ],
        );

        eprintln!("C-encoded fs_write_bytes ({} bytes):", c_bytes.len());
        for (i, chunk) in c_bytes.chunks(16).enumerate() {
            eprint!("  {:04x}:", i * 16);
            for b in chunk {
                eprint!(" {:02x}", b);
            }
            eprintln!();
        }

        let result = FunctionCall::try_from(c_bytes.as_slice());
        match &result {
            Ok(fc) => eprintln!("PARSED: name={}", fc.function_name),
            Err(e) => eprintln!("FAILED: {:?}", e),
        }
        assert!(result.is_ok(), "Should parse: {:?}", result.err());
    }

    // Helper types and function to replicate fb_encode_generic from hcall.c
    enum CParam<'a> {
        Int(i32),
        Str(&'a str),
        ULong(u64),
        VecBytes(&'a [u8]),
    }

    fn align4(x: usize) -> usize {
        (x + 3) & !3
    }
    fn align2(x: usize) -> usize {
        (x + 1) & !1
    }
    /// Smallest value >= x that is congruent to 4 mod 8.
    /// Ensures u64 field at (result + 4) is 8-byte aligned.
    fn align8_off4(x: usize) -> usize {
        ((x + 3) & !7) | 4
    }

    fn ew16(buf: &mut [u8], pos: usize, val: u16) {
        buf[pos] = val as u8;
        buf[pos + 1] = (val >> 8) as u8;
    }
    fn ew32(buf: &mut [u8], pos: usize, val: u32) {
        buf[pos] = val as u8;
        buf[pos + 1] = (val >> 8) as u8;
        buf[pos + 2] = (val >> 16) as u8;
        buf[pos + 3] = (val >> 24) as u8;
    }
    fn ew64(buf: &mut [u8], pos: usize, val: u64) {
        for i in 0..8 {
            buf[pos + i] = (val >> (i * 8)) as u8;
        }
    }

    fn build_c_generic_fb(name: &str, call_type: u8, ret_type: u8, params: &[CParam]) -> Vec<u8> {
        let nlen = name.len();
        let np = params.len();

        const PM_VT_SZ: usize = 8;
        const PM_TBL_SZ: usize = 12;
        const VW_SCALAR_VT_SZ: usize = 6;
        const VW_INT_TBL_SZ: usize = 8;
        const VW_ULONG_TBL_SZ: usize = 12;
        const VW_REF_TBL_SZ: usize = 8;

        struct PLay {
            pvt: usize,
            ptbl: usize,
            vvt: usize,
            vtbl: usize,
            vdata: usize,
            vvtsz: usize,
            vtblsz: usize,
        }

        let mut pos: usize = 36;
        let pvec = if np > 0 {
            let v = align4(pos);
            pos = v + 4 + np * 4;
            v
        } else {
            0
        };

        let mut pl: Vec<PLay> = Vec::new();
        for param in params.iter().take(np) {
            let pvt = align2(pos);
            let ptbl = align4(pvt + PM_VT_SZ);
            let (vvtsz, vtblsz) = match param {
                CParam::Int(_) => (VW_SCALAR_VT_SZ, VW_INT_TBL_SZ),
                CParam::ULong(_) => (VW_SCALAR_VT_SZ, VW_ULONG_TBL_SZ),
                CParam::Str(_) | CParam::VecBytes(_) => (VW_SCALAR_VT_SZ, VW_REF_TBL_SZ),
            };
            let vvt = align2(ptbl + PM_TBL_SZ);
            let vtbl = match param {
                CParam::ULong(_) => align8_off4(vvt + vvtsz),
                _ => align4(vvt + vvtsz),
            };
            pos = vtbl + vtblsz;
            pl.push(PLay {
                pvt,
                ptbl,
                vvt,
                vtbl,
                vdata: 0,
                vvtsz,
                vtblsz,
            });
        }

        // Variable-length data
        for (i, param) in params.iter().enumerate().take(np) {
            match param {
                CParam::Str(s) => {
                    pl[i].vdata = align4(pos);
                    pos = pl[i].vdata + 4 + align4(s.len() + 1);
                }
                CParam::VecBytes(v) => {
                    pl[i].vdata = align4(pos);
                    let dlen = if v.is_empty() { 1 } else { v.len() };
                    pos = pl[i].vdata + 4 + align4(dlen);
                }
                _ => {}
            }
        }

        let fnpos = align4(pos);
        pos = fnpos + 4 + align4(nlen + 1);
        let total = pos;

        let mut buf = vec![0u8; total];

        // Size prefix
        ew32(&mut buf, 0, (total - 4) as u32);
        // Root offset
        ew32(&mut buf, 4, 16);

        // Root vtable at 8
        ew16(&mut buf, 8, 12);
        ew16(&mut buf, 10, 16);
        ew16(&mut buf, 12, 4); // VT+4: function_name
        ew16(&mut buf, 14, if np > 0 { 8 } else { 0 }); // VT+6: parameters
        ew16(&mut buf, 16, 12); // VT+8: function_call_type
        ew16(&mut buf, 18, 13); // VT+10: expected_return_type

        // Root table at 20
        ew32(&mut buf, 20, 12); // soffset → vtable at 8
        ew32(&mut buf, 24, (fnpos - 24) as u32); // func name uoffset
        if np > 0 {
            ew32(&mut buf, 28, (pvec - 28) as u32); // params vector uoffset
        }
        buf[32] = call_type;
        buf[33] = ret_type;

        // Params vector
        if np > 0 {
            ew32(&mut buf, pvec, np as u32);
            for (i, layout) in pl.iter().enumerate().take(np) {
                let ep = pvec + 4 + i * 4;
                ew32(&mut buf, ep, (layout.ptbl - ep) as u32);
            }
        }

        // Each parameter
        for (param, layout) in params.iter().zip(pl.iter()).take(np) {
            // Parameter vtable
            ew16(&mut buf, layout.pvt, PM_VT_SZ as u16);
            ew16(&mut buf, layout.pvt + 2, PM_TBL_SZ as u16);
            ew16(&mut buf, layout.pvt + 4, 4);
            ew16(&mut buf, layout.pvt + 6, 8);

            // Parameter table
            ew32(&mut buf, layout.ptbl, (layout.ptbl - layout.pvt) as u32);
            let pv_type = match param {
                CParam::Int(_) => 1u8,      // HL_PV_HLINT
                CParam::ULong(_) => 4u8,    // HL_PV_HLULONG (was incorrectly 5=hlfloat!)
                CParam::Str(_) => 7u8,      // HL_PV_HLSTRING
                CParam::VecBytes(_) => 9u8, // HL_PV_HLVECBYTES
            };
            buf[layout.ptbl + 4] = pv_type;
            ew32(
                &mut buf,
                layout.ptbl + 8,
                (layout.vtbl - (layout.ptbl + 8)) as u32,
            );

            // Value vtable
            ew16(&mut buf, layout.vvt, layout.vvtsz as u16);
            ew16(&mut buf, layout.vvt + 2, layout.vtblsz as u16);
            ew16(&mut buf, layout.vvt + 4, 4);

            // Value table
            ew32(&mut buf, layout.vtbl, (layout.vtbl - layout.vvt) as u32);

            match param {
                CParam::Int(v) => {
                    ew32(&mut buf, layout.vtbl + 4, *v as u32);
                }
                CParam::ULong(v) => {
                    ew64(&mut buf, layout.vtbl + 4, *v);
                }
                CParam::Str(s) => {
                    ew32(
                        &mut buf,
                        layout.vtbl + 4,
                        (layout.vdata - (layout.vtbl + 4)) as u32,
                    );
                    ew32(&mut buf, layout.vdata, s.len() as u32);
                    buf[layout.vdata + 4..layout.vdata + 4 + s.len()].copy_from_slice(s.as_bytes());
                }
                CParam::VecBytes(v) => {
                    ew32(
                        &mut buf,
                        layout.vtbl + 4,
                        (layout.vdata - (layout.vtbl + 4)) as u32,
                    );
                    ew32(&mut buf, layout.vdata, v.len() as u32);
                    if !v.is_empty() {
                        buf[layout.vdata + 4..layout.vdata + 4 + v.len()].copy_from_slice(v);
                    }
                }
            }
        }

        // Function name string
        ew32(&mut buf, fnpos, nlen as u32);
        buf[fnpos + 4..fnpos + 4 + nlen].copy_from_slice(name.as_bytes());

        buf
    }

    /// Roundtrip test for fs_read_bytes(mount_idx=0, path="test.txt", offset=0, len=32768)
    /// which uses u64 parameters.
    #[test]
    fn flatbuffer_generic_encoder_roundtrip_ulong() {
        use hyperlight_common::flatbuffer_wrappers::function_call::FunctionCall;

        let c_bytes = build_c_generic_fb(
            "fs_read_bytes",
            2, // HL_FCT_HOST
            9, // HL_RT_VECBYTES
            &[
                CParam::Int(0),
                CParam::Str("test.txt"),
                CParam::ULong(0),
                CParam::ULong(32768),
            ],
        );

        eprintln!("C-encoded fs_read_bytes ({} bytes):", c_bytes.len());
        for (i, chunk) in c_bytes.chunks(16).enumerate() {
            eprint!("  {:04x}:", i * 16);
            for b in chunk {
                eprint!(" {:02x}", b);
            }
            eprintln!();
        }

        let c_parsed = FunctionCall::try_from(c_bytes.as_slice());
        assert!(
            c_parsed.is_ok(),
            "C-encoded FunctionCall should parse: {:?}",
            c_parsed.err()
        );
        let c_parsed = c_parsed.unwrap();
        assert_eq!(c_parsed.function_name, "fs_read_bytes");
        assert_eq!(c_parsed.parameters.as_ref().unwrap().len(), 4);
    }

    /// Verify that the C encoder's ULong discriminant matches HL_PV_HLULONG=4
    /// (not 5=hlfloat) and that 8-byte alignment is respected.
    #[test]
    fn c_encoder_ulong_alignment_check() {
        use hyperlight_common::flatbuffer_wrappers::function_call::FunctionCall;

        // First, fix the discriminant and test with type=4 (the REAL HL_PV_HLULONG)
        let c_bytes = build_c_generic_fb(
            "fs_write_bytes",
            2, // HL_FCT_HOST
            0, // HL_RT_INT
            &[
                CParam::Int(0),
                CParam::Str("written.txt"),
                CParam::ULong(0),
                CParam::Int(0),
                CParam::VecBytes(&[]),
            ],
        );

        eprintln!("C-encoded fs_write_bytes ({} bytes):", c_bytes.len());
        for (i, chunk) in c_bytes.chunks(16).enumerate() {
            eprint!("  {:04x}:", i * 16);
            for b in chunk {
                eprint!(" {:02x}", b);
            }
            eprintln!();
        }

        let result = FunctionCall::try_from(c_bytes.as_slice());
        match &result {
            Ok(fc) => eprintln!("PARSED: name={}", fc.function_name),
            Err(e) => eprintln!("FAILED: {:?}", e),
        }
        assert!(result.is_ok(), "Should parse: {:?}", result.err());
    }

    #[test]
    fn fstab_multiple_mixed_mounts() {
        let arg = fstab_arg(&[Mount::rw("/a", "/mnt/a"), Mount::ro("/b", "/mnt/b")]).unwrap();
        assert_eq!(
            arg,
            " vfs.fstab=[0:/mnt/a:hostfs:0x0::mkmp 1:/mnt/b:hostfs:0x1::mkmp]"
        );
    }

    #[test]
    fn single_request_is_added_as_a_token_on_the_options_line() {
        assert_eq!(with_single_request(""), "");
        assert_eq!(
            with_single_request("nameserver 10.0.0.1"),
            "nameserver 10.0.0.1\noptions single-request\n"
        );
        assert_eq!(
            with_single_request("nameserver 10.0.0.1\noptions ndots:5\n"),
            "nameserver 10.0.0.1\noptions ndots:5 single-request\n"
        );
        // Already there: left alone.
        assert_eq!(
            with_single_request("options ndots:5 single-request\n"),
            "options ndots:5 single-request\n"
        );
        // A different option, and a comment, are not the option.
        assert_eq!(
            with_single_request("options single-request-reopen\n"),
            "options single-request-reopen single-request\n"
        );
        assert_eq!(
            with_single_request("# single-request\nnameserver 10.0.0.1\n"),
            "# single-request\nnameserver 10.0.0.1\noptions single-request\n"
        );
    }

    #[test]
    fn default_scratch_follows_the_driver_in_the_initrd() {
        let dir = std::env::temp_dir().join(format!("hluk-scratch-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let node = write_cpio(
            &dir,
            "node.cpio",
            &[("usr/local/bin/hl_nodedriver", b"ELF")],
        );
        assert_eq!(default_scratch_mb(&node), 512);
        let plain = write_cpio(&dir, "plain.cpio", &[("bin/app", b"ELF")]);
        assert_eq!(default_scratch_mb(&plain), DEFAULT_SCRATCH_MB);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn runtime_marker_overrides_driver_detection() {
        let dir = test_dir("cpio-marker");
        // The agent image shares hl_pywarmdriver with python-shell but
        // carries etc/hluk-runtime naming the actual runtime.
        let agent = write_cpio(
            &dir,
            "agent.cpio",
            &[
                ("usr/local/bin/hl_pywarmdriver", b"ELF"),
                ("etc/hluk-runtime", b"agent\n"),
            ],
        );
        assert_eq!(default_scratch_mb(&agent), 1536, "marker wins over driver");
        let scan = scan_cpio(&agent);
        assert_eq!(scan.runtime.as_deref(), Some("agent"));
        assert_eq!(
            scan.entry.as_deref(),
            Some("/usr/local/bin/hl_pywarmdriver")
        );

        // Without the marker the same driver maps to python-shell.
        let shell = write_cpio(
            &dir,
            "shell.cpio",
            &[("usr/local/bin/hl_pywarmdriver", b"ELF")],
        );
        assert_eq!(default_scratch_mb(&shell), 256);
        assert!(scan_cpio(&shell).runtime.is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
