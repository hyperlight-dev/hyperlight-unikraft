// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
use std::path::PathBuf;
use std::sync::{Arc, Barrier};
use std::time::Instant;

use clap::{Parser, Subcommand};
use tracing::info;
use tracing_subscriber::EnvFilter;

use hyperlight_unikraft::{
    AllowList, BlockList, DEFAULT_SCRATCH_MB, Exec, ListenPorts, Mount, NetworkPolicy, OciTag,
    SNAPSHOT_TAG, SandboxBuilder, Snapshot, run,
};

/// Minimal Hyperlight host for Unikraft unikernels.
#[derive(Parser)]
#[command(name = "hluk")]
struct Cli {
    /// Log level for hluk diagnostics: error, warn, info, debug, trace.
    /// Off by default; pass --log-level info to see timing.
    #[arg(long, global = true)]
    log_level: Option<tracing::Level>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Boot a Unikraft guest and dispatch exec commands.
    Run(RunArgs),

    /// Snapshot operations: save a post-evolve snapshot to disk,
    /// or restore from a saved snapshot and dispatch.
    #[command(subcommand)]
    Snapshot(SnapshotCommand),

    /// Benchmark modes with structured timing output.
    #[command(subcommand)]
    Bench(BenchCommand),
}

#[derive(Subcommand)]
enum SnapshotCommand {
    /// Boot the guest, then save a snapshot to disk.
    Save(SaveArgs),

    /// Restore a guest from a saved snapshot and dispatch commands.
    Run(SnapshotRunArgs),
}

/// Arguments for `run` — boot the embedded kernel + initrd and dispatch.
#[derive(clap::Args)]
struct RunArgs {
    /// Script file (.py, .js, …) to execute in the guest.
    #[arg(conflicts_with = "exec")]
    script: Option<PathBuf>,

    /// Path to a CPIO initrd to map into the guest.
    #[arg(long)]
    initrd: Option<PathBuf>,

    /// Entry point binary path inside the initrd VFS.
    /// Auto-detected from the initrd if not specified.
    #[arg(long)]
    entry: Option<String>,

    /// Advanced: boot a kernel from this path instead of the embedded one.
    /// Must match the host ABI this build expects, or the guest will fault.
    /// Intended for kernel development.
    #[arg(long, value_name = "PATH")]
    kernel: Option<PathBuf>,

    /// Scratch memory in MiB (default 256; increase for large rootfs).
    #[arg(long, default_value_t = DEFAULT_SCRATCH_MB)]
    scratch_mb: usize,

    /// Inline code to execute (alternative to a script file).
    #[arg(long, conflicts_with = "script")]
    exec: Option<String>,

    /// Run a command that already lives in the guest filesystem: a path plus
    /// optional args (e.g. "/app/server --port 8080"). Unlike a script file
    /// (read from the host) or --exec (host code), this runs a file baked into
    /// the initrd. This is how urunc drives the guest. With no workload given,
    /// the guest's conventional entrypoint (/entrypoint.py, /entrypoint, …) runs.
    #[arg(long = "guest-exec", value_name = "COMMAND", conflicts_with_all = ["script", "exec"])]
    guest_exec: Option<String>,

    /// Mount a host directory into the guest filesystem.
    /// Format: HOST:GUEST[:ro] (e.g. /tmp/share:/mnt or /data:/mnt/data:ro).
    #[arg(long = "mount", value_name = "HOST:GUEST[:ro]")]
    mounts: Vec<String>,

    /// Enable host networking with no policy (all destinations allowed).
    #[arg(long, conflicts_with_all = ["net_allow", "net_block"])]
    net: bool,

    /// Allow-list: only permit connections to these hosts/IPs.
    /// Implies --net. Mutually exclusive with --net-block.
    #[arg(long = "net-allow", value_name = "HOST", conflicts_with = "net_block")]
    net_allow: Vec<String>,

    /// Block-list: deny connections to these hosts/IPs, allow everything else.
    /// Implies --net. Mutually exclusive with --net-allow.
    #[arg(long = "net-block", value_name = "HOST", conflicts_with = "net_allow")]
    net_block: Vec<String>,

    /// Ports the guest may bind to for inbound connections.
    /// Without this flag, bind() is rejected (outbound-only).
    #[arg(long = "port", value_name = "PORT")]
    ports: Vec<u16>,

    /// Set an environment variable in the guest (repeatable).
    /// Format: KEY=VALUE (e.g. --env MY_VAR=hello --env DEBUG=1).
    #[arg(long = "env", value_name = "KEY=VALUE")]
    envs: Vec<String>,
}

/// Arguments for `snapshot save`.
#[derive(clap::Args)]
struct SaveArgs {
    /// Path to a CPIO initrd to map into the guest.
    #[arg(long)]
    initrd: Option<PathBuf>,

    /// Entry point binary path inside the initrd VFS.
    /// Auto-detected from the initrd if not specified.
    #[arg(long)]
    entry: Option<String>,

    /// Advanced: boot a kernel from this path instead of the embedded one.
    /// Must match the host ABI this build expects, or the guest will fault.
    /// Intended for kernel development.
    #[arg(long, value_name = "PATH")]
    kernel: Option<PathBuf>,

    /// Scratch memory in MiB (default 256; increase for large rootfs).
    #[arg(long, default_value_t = DEFAULT_SCRATCH_MB)]
    scratch_mb: usize,

    /// Directory to save the snapshot (OCI Image Layout).
    #[arg(short, long)]
    output: PathBuf,

    /// Mount a host directory into the guest filesystem.
    /// Format: HOST:GUEST[:ro] (e.g. /tmp/share:/mnt or /data:/mnt/data:ro).
    #[arg(long = "mount", value_name = "HOST:GUEST[:ro]")]
    mounts: Vec<String>,

    /// Enable host networking with no policy (all destinations allowed).
    #[arg(long, conflicts_with_all = ["net_allow", "net_block"])]
    net: bool,

    /// Allow-list: only permit connections to these hosts/IPs.
    #[arg(long = "net-allow", value_name = "HOST", conflicts_with = "net_block")]
    net_allow: Vec<String>,

    /// Block-list: deny connections to these hosts/IPs, allow everything else.
    #[arg(long = "net-block", value_name = "HOST", conflicts_with = "net_allow")]
    net_block: Vec<String>,

    /// Ports the guest may bind to for inbound connections.
    #[arg(long = "port", value_name = "PORT")]
    ports: Vec<u16>,
}

/// Arguments for `snapshot run`.
#[derive(clap::Args)]
struct SnapshotRunArgs {
    /// Path to a saved snapshot directory (OCI Image Layout).
    snapshot: PathBuf,

    /// Script file (.py, .js, …) to execute in the guest.
    #[arg(conflicts_with = "exec")]
    script: Option<PathBuf>,

    /// Inline code to execute (alternative to a script file).
    #[arg(long, conflicts_with = "script")]
    exec: Option<String>,

    /// Run a command that already lives in the guest filesystem (path plus
    /// optional args). With no workload given, the guest's conventional
    /// entrypoint runs. See `hluk run --help`.
    #[arg(long = "guest-exec", value_name = "COMMAND", conflicts_with_all = ["script", "exec"])]
    guest_exec: Option<String>,

    /// Mount a host directory into the guest filesystem.
    /// Format: HOST:GUEST[:ro] (e.g. /tmp/share:/mnt or /data:/mnt/data:ro).
    #[arg(long = "mount", value_name = "HOST:GUEST[:ro]")]
    mounts: Vec<String>,

    /// Enable host networking with no policy (all destinations allowed).
    #[arg(long, conflicts_with_all = ["net_allow", "net_block"])]
    net: bool,

    /// Allow-list: only permit connections to these hosts/IPs.
    #[arg(long = "net-allow", value_name = "HOST", conflicts_with = "net_block")]
    net_allow: Vec<String>,

    /// Block-list: deny connections to these hosts/IPs, allow everything else.
    #[arg(long = "net-block", value_name = "HOST", conflicts_with = "net_allow")]
    net_block: Vec<String>,

    /// Ports the guest may bind to for inbound connections.
    #[arg(long = "port", value_name = "PORT")]
    ports: Vec<u16>,

    /// Set an environment variable in the guest (repeatable).
    /// Format: KEY=VALUE (e.g. --env MY_VAR=hello --env DEBUG=1).
    #[arg(long = "env", value_name = "KEY=VALUE")]
    envs: Vec<String>,
}

#[derive(Subcommand)]
enum BenchCommand {
    /// Cold start: fresh boot (evolve) + dispatch, no snapshot.
    Cold(BenchColdArgs),

    /// Cold snapshot start: load snapshot from disk + restore + dispatch.
    ColdSnap(BenchSnapArgs),

    /// Warm with restore: load snapshot once, then loop (dispatch + restore).
    WarmRestore(BenchSnapArgs),

    /// Warm stateful: load snapshot once, then loop (dispatch only, no restore).
    WarmStateful(BenchSnapArgs),

    /// Parallel VMs: spawn N VMs concurrently from the same snapshot.
    Parallel(BenchParallelArgs),
}

/// Arguments for `bench cold`.
#[derive(clap::Args)]
struct BenchColdArgs {
    /// Path to a CPIO initrd.
    #[arg(long)]
    initrd: PathBuf,

    /// Script file to execute.
    script: PathBuf,

    /// Scratch memory in MiB.
    #[arg(long, default_value_t = DEFAULT_SCRATCH_MB)]
    scratch_mb: usize,

    /// Number of samples to run.
    #[arg(long, default_value_t = 20)]
    samples: usize,
}

/// Arguments for snapshot-based bench modes (cold-snap, warm-restore, warm-stateful).
#[derive(clap::Args)]
struct BenchSnapArgs {
    /// Path to a saved snapshot directory.
    snapshot: PathBuf,

    /// Script file to execute.
    script: PathBuf,

    /// Number of iterations / samples.
    #[arg(long, default_value_t = 20)]
    samples: usize,
}

/// Arguments for `bench parallel`.
#[derive(clap::Args)]
struct BenchParallelArgs {
    /// Path to a saved snapshot directory.
    snapshot: PathBuf,

    /// Script file to execute.
    script: PathBuf,

    /// Number of concurrent VMs.
    #[arg(long, default_value_t = 4)]
    vms: usize,

    /// Iterations per VM.
    #[arg(long, default_value_t = 10)]
    iterations: usize,
}

// ── Helpers ──────────────────────────────────────────────────────

/// Parse `--env KEY=VALUE` strings into `(key, value)` pairs.
///
/// Entries without `=` are silently skipped. The first `=` is the
/// split point, so values may contain `=`.
fn parse_envs(raw: &[String]) -> Vec<(&str, &str)> {
    raw.iter().filter_map(|e| e.split_once('=')).collect()
}

fn parse_mounts(raw: &[String]) -> Vec<Mount> {
    raw.iter()
        .filter_map(|m| {
            // On Windows, "C:\foo:/mnt" would split wrong at the drive
            // letter colon.  Detect "X:\" prefix and split after it.
            let (host, rest) = if m.len() >= 3
                && m.as_bytes()[0].is_ascii_alphabetic()
                && m.as_bytes()[1] == b':'
                && (m.as_bytes()[2] == b'\\' || m.as_bytes()[2] == b'/')
            {
                // Drive-letter prefix — split at the NEXT colon.
                let after_drive = &m[2..];
                let colon = after_drive.find(':')?;
                (&m[..2 + colon], &after_drive[colon + 1..])
            } else {
                m.split_once(':')?
            };
            let (guest, readonly) = match rest.rsplit_once(':') {
                Some((g, "ro")) => (g, true),
                _ => (rest, false),
            };
            Some(Mount {
                host_path: PathBuf::from(host),
                guest_path: guest.to_string(),
                readonly,
            })
        })
        .collect()
}

/// Convert CLI net flags into `(Option<NetworkPolicy>, Option<ListenPorts>)`.
fn parse_net_policy(
    net: bool,
    net_allow: &[String],
    net_block: &[String],
    ports: &[u16],
) -> Result<(Option<NetworkPolicy>, Option<ListenPorts>), String> {
    let policy = if !net_allow.is_empty() {
        Some(NetworkPolicy::AllowList(AllowList::from_hosts(net_allow)?))
    } else if !net_block.is_empty() {
        Some(NetworkPolicy::BlockList(BlockList::from_hosts(net_block)?))
    } else if net {
        Some(NetworkPolicy::AllowAll)
    } else {
        None
    };
    // Listen ports are meaningless without networking: `hostnet` is only
    // registered when a policy is present, so `--port` on its own would be a
    // silent no-op (the guest gets no networking at all).  Reject it up front
    // rather than let the user believe the guest can bind.
    if !ports.is_empty() && policy.is_none() {
        return Err(
            "--port requires networking to be enabled; pass --net (or --net-allow/--net-block) too"
                .to_string(),
        );
    }
    let listen = if !ports.is_empty() {
        Some(ListenPorts::from_ports(ports.iter().copied()))
    } else {
        None
    };
    Ok((policy, listen))
}

/// Resolve script/exec args into an Exec value.
///
/// Text files are passed as scripts.  Compiled binaries are rejected
/// with a helpful error — use `--mount` + `--exec` for those.
fn resolve_exec(
    script: Option<PathBuf>,
    exec: Option<String>,
) -> hyperlight_unikraft::hyperlight_host::Result<Option<Exec>> {
    match (script, exec) {
        (Some(path), _) => {
            // Verify the file is valid UTF-8 (i.e. a script, not a binary)
            if std::fs::read_to_string(&path).is_err() && path.exists() {
                let dir = path
                    .parent()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| ".".into());
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "binary".into());
                return Err(
                    hyperlight_unikraft::hyperlight_host::HyperlightError::Error(format!(
                        "{} is a compiled binary, not a script.\n\
                         To run compiled binaries, mount a directory containing the binary:\n  \
                         hluk run --initrd <rootfs.cpio> --mount {dir}:/mnt/bin --exec /mnt/bin/{name}",
                        path.display(),
                    )),
                );
            }
            Ok(Some(Exec::File(path)))
        }
        (_, Some(code)) => Ok(Some(Exec::Code(code))),
        _ => Ok(None),
    }
}

// ── Commands ─────────────────────────────────────────────────────

/// Start a [`SandboxBuilder`] from the CLI's `--kernel` / `--initrd`.  A run
/// needs a workload — an external kernel, a rootfs, or both — so neither is an
/// error rather than a sandbox with nothing to boot.
fn base_builder(
    kernel: Option<PathBuf>,
    initrd: Option<PathBuf>,
) -> hyperlight_unikraft::hyperlight_host::Result<SandboxBuilder> {
    match (kernel, initrd) {
        (Some(kernel), Some(initrd)) => Ok(SandboxBuilder::from_kernel(kernel).initrd(initrd)),
        (Some(kernel), None) => Ok(SandboxBuilder::from_kernel(kernel)),
        (None, Some(initrd)) => Ok(SandboxBuilder::from_initrd(initrd)),
        (None, None) => Err(
            hyperlight_unikraft::hyperlight_host::HyperlightError::Error(
                "no workload: pass --initrd <rootfs.cpio> or --kernel <kernel>".to_string(),
            ),
        ),
    }
}

fn cmd_run(args: RunArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let mounts = parse_mounts(&args.mounts);
    let (policy, listen) =
        parse_net_policy(args.net, &args.net_allow, &args.net_block, &args.ports)
            .map_err(hyperlight_unikraft::hyperlight_host::HyperlightError::Error)?;

    // Precedence: a host script or --exec code; else a guest command
    // (--guest-exec); else, with no workload at all, the rootfs's conventional
    // entrypoint. The last two are the model a container runtime (urunc) uses.
    let exec = resolve_exec(args.script, args.exec)?
        .unwrap_or_else(|| Exec::Guest(args.guest_exec.unwrap_or_default()));
    let envs = parse_envs(&args.envs);

    let mut builder = base_builder(args.kernel, args.initrd)?
        .scratch_mb(args.scratch_mb)
        .mounts(mounts);
    if let Some(entry) = args.entry {
        builder = builder.entry(entry);
    }
    if let Some(policy) = policy {
        builder = builder.network(policy);
    }
    if let Some(listen) = listen {
        builder = builder.listen_ports(listen);
    }
    for (key, value) in envs {
        builder = builder.env(key, value);
    }
    let t = Instant::now();
    let (mut sandbox, _config) = builder.boot()?;
    info!(elapsed_ms = t.elapsed().as_secs_f64() * 1000.0, "boot");

    let t = Instant::now();
    run(&mut sandbox, exec)?;
    info!(elapsed_ms = t.elapsed().as_secs_f64() * 1000.0, "exec");

    Ok(())
}

fn cmd_snapshot_save(args: SaveArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let mounts = parse_mounts(&args.mounts);
    let (policy, listen) =
        parse_net_policy(args.net, &args.net_allow, &args.net_block, &args.ports)
            .map_err(hyperlight_unikraft::hyperlight_host::HyperlightError::Error)?;
    let mut builder = base_builder(args.kernel, args.initrd)?
        .scratch_mb(args.scratch_mb)
        .mounts(mounts);
    if let Some(entry) = args.entry {
        builder = builder.entry(entry);
    }
    if let Some(policy) = policy {
        builder = builder.network(policy);
    }
    if let Some(listen) = listen {
        builder = builder.listen_ports(listen);
    }
    let (mut sandbox, _config) = builder.boot()?;

    let t = Instant::now();
    let snap = sandbox.snapshot()?;
    info!(
        elapsed_ms = t.elapsed().as_secs_f64() * 1000.0,
        "snapshot captured",
    );

    let tag: OciTag = SNAPSHOT_TAG.parse().expect("valid OCI tag");

    let t = Instant::now();
    let digest = snap.save(&args.output, &tag)?;
    let save_ms = t.elapsed().as_secs_f64() * 1000.0;
    info!(
        path = %args.output.display(),
        digest = %digest,
        elapsed_ms = save_ms,
        "snapshot saved",
    );

    eprintln!(
        "Snapshot saved to {} ({:.1} ms)",
        args.output.display(),
        save_ms,
    );

    Ok(())
}

fn cmd_snapshot_run(args: SnapshotRunArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let tag: OciTag = SNAPSHOT_TAG.parse().expect("valid OCI tag");
    let mounts = parse_mounts(&args.mounts);
    let (policy, listen) =
        parse_net_policy(args.net, &args.net_allow, &args.net_block, &args.ports)
            .map_err(hyperlight_unikraft::hyperlight_host::HyperlightError::Error)?;

    let t = Instant::now();
    let snap: Arc<Snapshot> = Arc::new(Snapshot::load(&args.snapshot, tag)?);
    info!(
        path = %args.snapshot.display(),
        elapsed_ms = t.elapsed().as_secs_f64() * 1000.0,
        "snapshot loaded",
    );

    let envs = parse_envs(&args.envs);

    let mut builder = SandboxBuilder::from_snapshot(snap).mounts(mounts);
    if let Some(policy) = policy {
        builder = builder.network(policy);
    }
    if let Some(listen) = listen {
        builder = builder.listen_ports(listen);
    }
    for (key, value) in envs {
        builder = builder.env(key, value);
    }

    let t = Instant::now();
    let (mut sandbox, _config) = builder.boot()?;
    info!(
        elapsed_ms = t.elapsed().as_secs_f64() * 1000.0,
        "restored from snapshot",
    );

    // Precedence: host script / --exec code; else --guest-exec; else the
    // rootfs's conventional entrypoint.
    let exec = resolve_exec(args.script, args.exec)?
        .unwrap_or_else(|| Exec::Guest(args.guest_exec.unwrap_or_default()));
    let t = Instant::now();
    run(&mut sandbox, exec)?;
    info!(elapsed_ms = t.elapsed().as_secs_f64() * 1000.0, "exec");
    Ok(())
}

// ── Bench helpers ────────────────────────────────────────────────

/// Read a script file and return its source for dispatch.
fn read_script(path: &std::path::Path) -> hyperlight_unikraft::hyperlight_host::Result<String> {
    std::fs::read_to_string(path).map_err(|e| {
        hyperlight_unikraft::hyperlight_host::HyperlightError::Error(format!(
            "failed to read {}: {e}",
            path.display(),
        ))
    })
}

/// Percentile from a **sorted** slice (linear interpolation).
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.len() == 1 {
        return sorted[0];
    }
    let idx = p / 100.0 * (sorted.len() - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    let frac = idx - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

/// Print summary line: median, p95, min, max.
fn print_summary(label: &str, field: &str, values: &[f64]) {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = percentile(&sorted, 50.0);
    let p95 = percentile(&sorted, 95.0);
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    println!(
        "BENCH {label} {field} median={median:.3} p95={p95:.3} min={min:.3} max={max:.3} samples={}",
        values.len(),
    );
}

/// Print snapshot size on disk as a BENCH line.
fn print_snapshot_size(label: &str, snap_dir: &std::path::Path) {
    fn dir_size(path: &std::path::Path) -> std::io::Result<u64> {
        let mut total = 0;
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            if meta.is_file() {
                total += meta.len();
            } else if meta.is_dir() {
                total += dir_size(&entry.path())?;
            }
        }
        Ok(total)
    }
    if let Ok(bytes) = dir_size(snap_dir) {
        let mib = bytes as f64 / (1024.0 * 1024.0);
        println!("BENCH {label} snapshot_mib={mib:.1}");
    }
}

/// Print resident memory as a BENCH line.
/// This is the density-relevant metric — it scales linearly with VM count.
/// Linux reports `RssAnon` (anonymous resident pages).  Windows reports the
/// working set: guest memory there is a section mapping, which the
/// private-commit counters do not attribute to the process.  The two are
/// comparable within an OS, not across them.
fn print_rss(label: &str) {
    #[cfg(target_os = "linux")]
    if let Ok(status) = std::fs::read_to_string("/proc/self/status")
        && let Some(kb) = status
            .lines()
            .find(|l| l.starts_with("RssAnon:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u64>().ok())
    {
        println!("BENCH {label} rss_mb={}", kb / 1024);
    }
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::ProcessStatus::{
            K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS,
        };
        use windows_sys::Win32::System::Threading::GetCurrentProcess;

        let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { std::mem::zeroed() };
        let size = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        // SAFETY: `counters` is a valid, writable struct of the size passed
        // in `cb`.
        let ok = unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &raw mut counters, size) };
        if ok != 0 {
            println!(
                "BENCH {label} rss_mb={}",
                counters.WorkingSetSize / (1024 * 1024)
            );
        }
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    let _ = label;
}

// ── Bench commands ───────────────────────────────────────────────

/// Cold start: fresh boot + dispatch, N independent samples.
fn bench_cold(args: BenchColdArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let source = read_script(&args.script)?;
    let mut boots = Vec::with_capacity(args.samples);
    let mut execs = Vec::with_capacity(args.samples);
    let mut totals = Vec::with_capacity(args.samples);

    for i in 0..args.samples {
        let t0 = Instant::now();
        let (mut sandbox, _) = SandboxBuilder::from_initrd(args.initrd.clone())
            .scratch_mb(args.scratch_mb)
            .boot()?;
        let boot_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t1 = Instant::now();
        run(&mut sandbox, source.as_str())?;
        let exec_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let total_ms = t0.elapsed().as_secs_f64() * 1000.0;
        println!(
            "BENCH cold sample={i} boot_ms={boot_ms:.3} exec_ms={exec_ms:.3} total_ms={total_ms:.3}"
        );
        boots.push(boot_ms);
        execs.push(exec_ms);
        totals.push(total_ms);
    }

    print_summary("cold", "boot_ms", &boots);
    print_summary("cold", "exec_ms", &execs);
    print_summary("cold", "total_ms", &totals);
    print_rss("cold");
    Ok(())
}

/// Cold snapshot: load from disk + restore + dispatch, N independent samples.
fn bench_cold_snap(args: BenchSnapArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let source = read_script(&args.script)?;
    let tag: OciTag = SNAPSHOT_TAG.parse().expect("valid OCI tag");
    let mut loads = Vec::with_capacity(args.samples);
    let mut restores = Vec::with_capacity(args.samples);
    let mut execs = Vec::with_capacity(args.samples);
    let mut totals = Vec::with_capacity(args.samples);

    for i in 0..args.samples {
        let t0 = Instant::now();
        let snap = Arc::new(Snapshot::load(&args.snapshot, tag.clone())?);
        let load_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t1 = Instant::now();
        let (mut sandbox, _config) = SandboxBuilder::from_snapshot(snap).boot()?;
        let restore_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let t2 = Instant::now();
        run(&mut sandbox, source.as_str())?;
        let exec_ms = t2.elapsed().as_secs_f64() * 1000.0;

        let total_ms = t0.elapsed().as_secs_f64() * 1000.0;
        println!(
            "BENCH cold-snap sample={i} load_ms={load_ms:.3} restore_ms={restore_ms:.3} \
             exec_ms={exec_ms:.3} total_ms={total_ms:.3}"
        );
        loads.push(load_ms);
        restores.push(restore_ms);
        execs.push(exec_ms);
        totals.push(total_ms);
    }

    print_summary("cold-snap", "load_ms", &loads);
    print_summary("cold-snap", "restore_ms", &restores);
    print_summary("cold-snap", "exec_ms", &execs);
    print_summary("cold-snap", "total_ms", &totals);
    print_snapshot_size("cold-snap", &args.snapshot);
    print_rss("cold-snap");
    Ok(())
}

/// Warm with restore: load snapshot once, then loop dispatch + restore.
fn bench_warm_restore(args: BenchSnapArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let source = read_script(&args.script)?;
    let tag: OciTag = SNAPSHOT_TAG.parse().expect("valid OCI tag");

    let t0 = Instant::now();
    let snap = Arc::new(Snapshot::load(&args.snapshot, tag)?);
    let (mut sandbox, _config) = SandboxBuilder::from_snapshot(snap.clone()).boot()?;
    let setup_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!("BENCH warm-restore setup_ms={setup_ms:.3}");

    let mut execs = Vec::with_capacity(args.samples);
    let mut restores = Vec::with_capacity(args.samples);

    for i in 0..args.samples {
        let t1 = Instant::now();
        run(&mut sandbox, source.as_str())?;
        let exec_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let t2 = Instant::now();
        sandbox.restore(snap.clone())?;
        let restore_ms = t2.elapsed().as_secs_f64() * 1000.0;

        println!("BENCH warm-restore sample={i} exec_ms={exec_ms:.3} restore_ms={restore_ms:.3}");
        execs.push(exec_ms);
        restores.push(restore_ms);
    }

    print_summary("warm-restore", "exec_ms", &execs);
    print_summary("warm-restore", "restore_ms", &restores);
    print_snapshot_size("warm-restore", &args.snapshot);
    print_rss("warm-restore");
    Ok(())
}

/// Warm stateful: load snapshot once, then loop dispatch without restore.
fn bench_warm_stateful(args: BenchSnapArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let source = read_script(&args.script)?;
    let tag: OciTag = SNAPSHOT_TAG.parse().expect("valid OCI tag");

    let t0 = Instant::now();
    let snap = Arc::new(Snapshot::load(&args.snapshot, tag)?);
    let (mut sandbox, _config) = SandboxBuilder::from_snapshot(snap).boot()?;
    let setup_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!("BENCH warm-stateful setup_ms={setup_ms:.3}");

    let mut execs = Vec::with_capacity(args.samples);

    for i in 0..args.samples {
        let t1 = Instant::now();
        run(&mut sandbox, source.as_str())?;
        let exec_ms = t1.elapsed().as_secs_f64() * 1000.0;

        println!("BENCH warm-stateful sample={i} exec_ms={exec_ms:.3}");
        execs.push(exec_ms);
    }

    print_summary("warm-stateful", "exec_ms", &execs);
    print_snapshot_size("warm-stateful", &args.snapshot);
    print_rss("warm-stateful");
    Ok(())
}

/// Parallel VMs: spawn N threads, each restoring from the same snapshot.
fn bench_parallel(args: BenchParallelArgs) -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let source = Arc::new(read_script(&args.script)?);
    let tag: OciTag = SNAPSHOT_TAG.parse().expect("valid OCI tag");
    let snap = Arc::new(Snapshot::load(&args.snapshot, tag)?);

    // Barrier so all VMs start at the same time.
    let barrier = Arc::new(Barrier::new(args.vms));
    let wall_start = Instant::now();

    let handles: Vec<_> = (0..args.vms)
        .map(|vm_id| {
            let snap = snap.clone();
            let source = source.clone();
            let barrier = barrier.clone();
            let iterations = args.iterations;

            std::thread::spawn(move || -> Result<Vec<f64>, String> {
                barrier.wait();
                let vm_start = Instant::now();

                let (mut sandbox, _config) = SandboxBuilder::from_snapshot(snap.clone())
                    .boot()
                    .map_err(|e| e.to_string())?;
                let mut execs = Vec::with_capacity(iterations);

                for iter in 0..iterations {
                    let t = Instant::now();
                    run(&mut sandbox, source.as_str()).map_err(|e| e.to_string())?;
                    let exec_ms = t.elapsed().as_secs_f64() * 1000.0;

                    let t = Instant::now();
                    sandbox.restore(snap.clone()).map_err(|e| e.to_string())?;
                    let restore_ms = t.elapsed().as_secs_f64() * 1000.0;

                    println!(
                        "BENCH parallel vm={vm_id} iter={iter} \
                         exec_ms={exec_ms:.3} restore_ms={restore_ms:.3}"
                    );
                    execs.push(exec_ms);
                }

                let vm_total_ms = vm_start.elapsed().as_secs_f64() * 1000.0;
                let vm_throughput = iterations as f64 / (vm_total_ms / 1000.0);
                println!(
                    "BENCH parallel vm={vm_id} total_ms={vm_total_ms:.3} \
                     iterations={iterations} throughput={vm_throughput:.1}/s"
                );
                Ok(execs)
            })
        })
        .collect();

    let mut errors = Vec::new();
    let mut all_execs = Vec::new();
    for (i, h) in handles.into_iter().enumerate() {
        match h.join().unwrap_or_else(|_| Err("thread panicked".into())) {
            Ok(execs) => all_execs.extend(execs),
            Err(e) => errors.push(format!("vm {i}: {e}")),
        }
    }

    let wall_ms = wall_start.elapsed().as_secs_f64() * 1000.0;
    let total_calls = args.vms * args.iterations;
    let throughput = total_calls as f64 / (wall_ms / 1000.0);
    println!(
        "BENCH parallel summary vms={} iterations={} total_calls={total_calls} \
         wall_ms={wall_ms:.3} throughput={throughput:.1}/s",
        args.vms, args.iterations,
    );
    if !all_execs.is_empty() {
        print_summary("parallel", "exec_ms", &all_execs);
    }
    print_snapshot_size("parallel", &args.snapshot);
    print_rss("parallel");

    if !errors.is_empty() {
        return Err(
            hyperlight_unikraft::hyperlight_host::HyperlightError::Error(errors.join("; ")),
        );
    }
    Ok(())
}

// ── Main ─────────────────────────────────────────────────────────

fn main() -> hyperlight_unikraft::hyperlight_host::Result<()> {
    let cli = Cli::parse();

    if let Some(level) = cli.log_level {
        // RUST_LOG overrides --log-level when set; otherwise scope to
        // our crate only so library noise doesn't leak through.
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(format!("hyperlight_unikraft={level}")));
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_writer(std::io::stderr)
            .init();
    }

    // One guest at a time, so skip Hyperlight's 512 pre-spawned helper
    // processes on Windows; `bench parallel` needs one per VM.
    #[cfg(windows)]
    hyperlight_unikraft::configure_surrogates(match &cli.command {
        Command::Bench(BenchCommand::Parallel(args)) => args.vms,
        _ => 0,
    });

    match cli.command {
        Command::Run(args) => cmd_run(args),
        Command::Snapshot(cmd) => match cmd {
            SnapshotCommand::Save(args) => cmd_snapshot_save(args),
            SnapshotCommand::Run(args) => cmd_snapshot_run(args),
        },
        Command::Bench(cmd) => match cmd {
            BenchCommand::Cold(args) => bench_cold(args),
            BenchCommand::ColdSnap(args) => bench_cold_snap(args),
            BenchCommand::WarmRestore(args) => bench_warm_restore(args),
            BenchCommand::WarmStateful(args) => bench_warm_stateful(args),
            BenchCommand::Parallel(args) => bench_parallel(args),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_envs_basic() {
        let input = vec!["KEY=value".into(), "DEBUG=1".into()];
        let envs = parse_envs(&input);
        assert_eq!(envs, vec![("KEY", "value"), ("DEBUG", "1")]);
    }

    #[test]
    fn parse_envs_value_with_equals() {
        let input = vec!["CONN=host=db;port=5432".into()];
        let envs = parse_envs(&input);
        assert_eq!(envs, vec![("CONN", "host=db;port=5432")]);
    }

    #[test]
    fn parse_envs_empty_value() {
        let input = vec!["EMPTY=".into()];
        let envs = parse_envs(&input);
        assert_eq!(envs, vec![("EMPTY", "")]);
    }

    #[test]
    fn parse_envs_skips_invalid() {
        let input = vec!["GOOD=1".into(), "no_equals".into(), "ALSO=ok".into()];
        let envs = parse_envs(&input);
        assert_eq!(envs, vec![("GOOD", "1"), ("ALSO", "ok")]);
    }

    #[test]
    fn parse_envs_empty_input() {
        let envs = parse_envs(&[]);
        assert!(envs.is_empty());
    }

    #[test]
    fn parse_unix_rw_mount() {
        let mounts = parse_mounts(&["/tmp/share:/mnt/host".into()]);
        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].host_path, PathBuf::from("/tmp/share"));
        assert_eq!(mounts[0].guest_path, "/mnt/host");
        assert!(!mounts[0].readonly);
    }

    #[test]
    fn parse_unix_ro_mount() {
        let mounts = parse_mounts(&["/data:/mnt/data:ro".into()]);
        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].host_path, PathBuf::from("/data"));
        assert_eq!(mounts[0].guest_path, "/mnt/data");
        assert!(mounts[0].readonly);
    }

    #[test]
    fn parse_multiple_mounts() {
        let mounts = parse_mounts(&["/a:/mnt/a".into(), "/b:/mnt/b:ro".into()]);
        assert_eq!(mounts.len(), 2);
        assert!(!mounts[0].readonly);
        assert!(mounts[1].readonly);
    }

    #[test]
    fn parse_windows_drive_rw() {
        let mounts = parse_mounts(&[r"C:\Users\data:/mnt/data".into()]);
        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].host_path, PathBuf::from(r"C:\Users\data"));
        assert_eq!(mounts[0].guest_path, "/mnt/data");
        assert!(!mounts[0].readonly);
    }

    #[test]
    fn parse_windows_drive_ro() {
        let mounts = parse_mounts(&[r"D:\share:/mnt/host:ro".into()]);
        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].host_path, PathBuf::from(r"D:\share"));
        assert_eq!(mounts[0].guest_path, "/mnt/host");
        assert!(mounts[0].readonly);
    }

    #[test]
    fn parse_relative_path() {
        let mounts = parse_mounts(&["./data:/mnt".into()]);
        assert_eq!(mounts.len(), 1);
        assert_eq!(mounts[0].host_path, PathBuf::from("./data"));
        assert_eq!(mounts[0].guest_path, "/mnt");
    }

    #[test]
    fn parse_invalid_no_colon() {
        let mounts = parse_mounts(&["invalid".into()]);
        assert!(mounts.is_empty());
    }

    #[test]
    fn net_policy_none_by_default() {
        let (policy, listen) = parse_net_policy(false, &[], &[], &[]).unwrap();
        assert!(policy.is_none());
        assert!(listen.is_none());
    }

    #[test]
    fn net_policy_bare_net_is_allow_all() {
        let (policy, listen) = parse_net_policy(true, &[], &[], &[]).unwrap();
        assert!(matches!(policy, Some(NetworkPolicy::AllowAll)));
        assert!(listen.is_none());
    }

    #[test]
    fn net_policy_port_with_net_sets_listen() {
        let (policy, listen) = parse_net_policy(true, &[], &[], &[8080]).unwrap();
        assert!(matches!(policy, Some(NetworkPolicy::AllowAll)));
        assert!(listen.is_some());
    }

    #[test]
    fn net_policy_port_with_allow_list_sets_listen() {
        let (policy, listen) =
            parse_net_policy(false, &["example.com".into()], &[], &[8080]).unwrap();
        assert!(matches!(policy, Some(NetworkPolicy::AllowList(_))));
        assert!(listen.is_some());
    }

    #[test]
    fn net_policy_port_without_net_is_rejected() {
        // --port alone would be a silent no-op (hostnet is only registered
        // when a policy is present), so it must be an error, not accepted.
        let err = parse_net_policy(false, &[], &[], &[8080]).unwrap_err();
        assert!(err.contains("--port requires networking"), "got: {err}");
    }
}
