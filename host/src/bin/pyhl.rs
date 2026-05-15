//! pyhl — run Python on hyperlight-unikraft with a persistent warmed interpreter.
//!
//! The binary wraps two things:
//!   `pyhl setup`  — installs the python-agent-driver image (kernel + CPIO) into
//!                   .pyhl/ so `pyhl run` can find it without the user having
//!                   to juggle paths.
//!   `pyhl run`    — runs a Python file or inline snippet against the installed
//!                   image. First call of the process pays the ~3.5s Py_Initialize
//!                   + warm-import cost; every subsequent invocation uses the
//!                   post-warmup snapshot and runs in ~100ms hermetic.
//!
//! Image resolution order, first hit wins:
//!   1. --dest PATH            (on the command line)
//!   2. $PYHL_HOME             (env var)
//!   3. ./.pyhl/               (cwd-relative)
//!   4. ~/.local/share/pyhl/   (XDG fallback)
//!
//! An installed image is just two files plus a metadata stamp:
//!   <home>/kernel           Unikraft kernel ELF
//!   <home>/initrd.cpio      driver + preloaded Python deps
//!   <home>/VERSION          source + timestamp (informational)

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use hyperlight_unikraft::pyhl::{
    copy_replace, discover_source_artifacts, extract_from_ghcr, ghcr_image_ref, GHCR_INITRD_REPO,
    GHCR_KERNEL_REPO,
};
use hyperlight_unikraft::{AllowList, BlockList, ListenPorts, NetworkPolicy, Preopen, Sandbox};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Parse a `--mount HOST[:GUEST]` argument to a `Preopen`. Default guest
/// path is `/host` when omitted.
fn parse_mount(spec: &str) -> Result<Preopen> {
    Preopen::parse_cli(spec).map_err(|e| anyhow!("invalid --mount {:?}: {}", spec, e))
}

fn build_network_policy(
    net: bool,
    net_allow: &[String],
    net_block: &[String],
    has_ports: bool,
) -> Result<Option<NetworkPolicy>> {
    if !net_allow.is_empty() {
        Ok(Some(NetworkPolicy::AllowList(AllowList::from_hosts(
            net_allow,
        )?)))
    } else if !net_block.is_empty() {
        Ok(Some(NetworkPolicy::BlockList(BlockList::from_hosts(
            net_block,
        )?)))
    } else if net || has_ports {
        Ok(Some(NetworkPolicy::AllowAll))
    } else {
        Ok(None)
    }
}

fn build_listen_ports(ports: &[u16]) -> Option<ListenPorts> {
    if ports.is_empty() {
        None
    } else {
        Some(ListenPorts::from_ports(ports.iter().copied()))
    }
}

/// Keep in sync with `py_initialize_once` in examples/python-agent-driver/
/// hl_pydriver.c. These modules are imported during `pyhl setup`'s warmup
/// so they're already in `sys.modules` in every `pyhl run` invocation —
/// no per-call import cost for any of them.
const PREIMPORTED_MODULES: &[&str] = &[
    "numpy",
    "pandas",
    "pydantic",
    "yaml",
    "jinja2",
    "bs4",
    "tabulate",
    "click",
    "tenacity",
    "tqdm",
    "openpyxl",
    "pypdf",
    "markdown_it",
    "PIL",
    "lxml",
    "cryptography",
    "dateutil",
    "dotenv",
];

/// Build the long-about blurb shown by `pyhl --help`. Lists the
/// third-party modules the warmup explicitly pre-imports.
fn long_about() -> String {
    let mut s = String::from(
        "Run Python on hyperlight-unikraft.\n\n\
         `pyhl setup` installs the python-agent-driver image and warms \
         up a Python interpreter snapshot so `pyhl run` can start in \
         ~100 ms hermetic per call (no Py_Initialize, no re-imports).\n\n\
         The full CPython standard library is available (os, shutil, \
         json, re, pathlib, sqlite3, subprocess shims, …). On top of \
         that, these third-party modules are pre-imported during \
         warmup, so your scripts can `import` them with zero cost:\n",
    );
    // Three columns of names, padded to 18 chars, to keep the help
    // readable in an 80-col terminal.
    for (i, m) in PREIMPORTED_MODULES.iter().enumerate() {
        if i % 3 == 0 {
            s.push_str("\n  ");
        }
        s.push_str(&format!("{m:<18}"));
    }
    s.push_str(
        "\n\n\
         Other third-party packages shipped in the rootfs still work — \
         they just pay the usual import cost on first access. Packages \
         not in the rootfs will raise ModuleNotFoundError.",
    );
    s
}

#[derive(Parser)]
#[command(
    name = "pyhl",
    version,
    about = "Run Python on hyperlight-unikraft",
    long_about = long_about()
)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install the python-agent-driver image (kernel + CPIO) so `pyhl run` can find it.
    Setup(SetupArgs),

    /// Run Python code against the installed image.
    Run(RunArgs),
}

#[derive(Args)]
struct SetupArgs {
    /// Where to install. Defaults to ./.pyhl/ (or ~/.local/share/pyhl/ if cwd
    /// is not writable). Also honors $PYHL_HOME.
    #[arg(long, env = "PYHL_HOME")]
    dest: Option<PathBuf>,

    /// Install from a local python-agent-driver build directory instead of
    /// downloading from GHCR. The directory must contain a `.unikraft/build`
    /// tree with a compiled kernel and a `*-initrd.cpio` alongside —
    /// typically `examples/python-agent-driver` in a checkout of
    /// danbugs/hyperlight-unikraft after `just build && just rootfs`.
    ///
    /// Without --from, pyhl pulls the pre-published image from GHCR (requires
    /// docker or podman on $PATH).
    #[arg(long, value_name = "DIR", conflicts_with = "tag")]
    from: Option<PathBuf>,

    /// Pin a specific GHCR release tag (e.g., "v0.3.1") instead of
    /// pulling :latest. Ignored when --from is used.
    #[arg(long, value_name = "TAG", conflicts_with = "from")]
    tag: Option<String>,

    /// Overwrite an existing installed image without prompting.
    #[arg(long)]
    force: bool,

    /// Expose a host directory to the guest at a fixed guest path.
    /// Format: HOST_DIR[:GUEST_PATH] (default GUEST_PATH is `/host`).
    /// Repeat for multiple mounts.
    ///
    /// The *guest path* is baked into the persisted snapshot (the guest
    /// mounts hostfs during warmup), so `pyhl run --mount` can only
    /// remap the host side later — the guest path must match what was
    /// given to `setup`.
    #[arg(long = "mount", value_name = "HOST[:GUEST]")]
    mounts: Vec<String>,

    /// Enable guest networking.
    #[arg(long)]
    net: bool,

    /// Restrict guest networking to the listed hosts/IPs.
    /// Implies --net. Repeatable.
    #[arg(
        long = "net-allow",
        value_name = "HOST_OR_IP",
        conflicts_with = "net_block"
    )]
    net_allow: Vec<String>,

    /// Block the listed hosts/IPs; all other destinations are allowed.
    /// Implies --net. Repeatable.
    #[arg(
        long = "net-block",
        value_name = "HOST_OR_IP",
        conflicts_with = "net_allow"
    )]
    net_block: Vec<String>,

    /// Allow the guest to bind (listen) on the given port. Implies --net.
    /// Without this flag, `net_bind` is rejected (outbound-only).
    /// Repeatable: `--port 8080 --port 3000`.
    #[arg(long, value_name = "PORT")]
    port: Vec<u16>,
}

#[derive(Args)]
struct RunArgs {
    /// Path to a Python script. Mutually exclusive with -c.
    script: Option<PathBuf>,

    /// Inline Python code. Mutually exclusive with <SCRIPT>.
    #[arg(short = 'c', long = "code", value_name = "CODE")]
    code: Option<String>,

    /// Run this many ADDITIONAL times after the first (each invocation is
    /// hermetic — fresh Python state via snapshot/restore).
    #[arg(long, default_value_t = 0, value_name = "N")]
    repeat: u32,

    /// Override the image directory.
    #[arg(long, env = "PYHL_HOME", value_name = "DIR")]
    dest: Option<PathBuf>,

    /// Expose a host directory to the guest for this run. Same format
    /// as `pyhl setup --mount`. The guest-path must match what was
    /// baked into the snapshot at setup time; only the host side is
    /// remappable per-run.
    #[arg(long = "mount", value_name = "HOST[:GUEST]")]
    mounts: Vec<String>,

    /// Enable guest networking.
    #[arg(long)]
    net: bool,

    /// Restrict guest networking to the listed hosts/IPs.
    /// Implies --net. Repeatable.
    #[arg(
        long = "net-allow",
        value_name = "HOST_OR_IP",
        conflicts_with = "net_block"
    )]
    net_allow: Vec<String>,

    /// Block the listed hosts/IPs; all other destinations are allowed.
    /// Implies --net. Repeatable.
    #[arg(
        long = "net-block",
        value_name = "HOST_OR_IP",
        conflicts_with = "net_allow"
    )]
    net_block: Vec<String>,

    /// Allow the guest to bind (listen) on the given port. Implies --net.
    /// Without this flag, `net_bind` is rejected (outbound-only).
    /// Repeatable: `--port 8080 --port 3000`.
    #[arg(long, value_name = "PORT")]
    port: Vec<u16>,

    /// Print evolve/warmup/per-run timing to stderr. Off by default so the
    /// user's script output is clean.
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Don't reseed random / numpy.random at the start of each call.
    ///
    /// By default pyhl prepends `random.seed()` and `np.random.seed()`
    /// (with fresh host entropy) to every script so `random.random()`
    /// and `np.random.randint()` yield different values per invocation —
    /// matching `python3` behavior.
    ///
    /// Pass `--deterministic` to skip the reseed. Every run then sees
    /// the exact same RNG state captured in the snapshot (useful when
    /// you want bit-for-bit reproducibility across calls).
    #[arg(long = "deterministic")]
    deterministic: bool,
}

// -- image-home resolution ----------------------------------------------------

const CWD_HOME: &str = ".pyhl";
const KERNEL_FILE: &str = "kernel";
const INITRD_FILE: &str = "initrd.cpio";
const SNAPSHOT_FILE: &str = "snapshot.hls";
const VERSION_FILE: &str = "VERSION";

/// Resolve the image home to use. Tries (in order): explicit, PYHL_HOME,
/// ./.pyhl/, ~/.local/share/pyhl/. For `run`, the first one that already
/// contains a usable image is picked. For `setup`, the first writable one
/// is picked.
fn resolve_home(explicit: Option<&Path>, mode: ResolveMode) -> Result<PathBuf> {
    if let Some(p) = explicit {
        return Ok(p.to_path_buf());
    }
    let cwd = std::env::current_dir().context("read cwd")?.join(CWD_HOME);
    let xdg = xdg_share_home().join("pyhl");

    match mode {
        ResolveMode::ForRun => {
            if image_installed(&cwd) {
                return Ok(cwd);
            }
            if image_installed(&xdg) {
                return Ok(xdg);
            }
            Err(anyhow!(
                "no pyhl image installed.\n\
                 searched: {}, {}\n\
                 run `pyhl setup --from <path/to/python-agent-driver>` first.",
                cwd.display(),
                xdg.display()
            ))
        }
        ResolveMode::ForSetup => {
            // Default to cwd-local to keep the artifact close to the project;
            // caller can override with --dest/$PYHL_HOME.
            Ok(cwd)
        }
    }
}

enum ResolveMode {
    ForRun,
    ForSetup,
}

fn xdg_share_home() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/"));
            home.join(".local/share")
        })
}

fn image_installed(home: &Path) -> bool {
    home.join(KERNEL_FILE).is_file() && home.join(INITRD_FILE).is_file()
}

// -- `setup` ------------------------------------------------------------------

fn cmd_setup(args: SetupArgs) -> Result<()> {
    let home = resolve_home(args.dest.as_deref(), ResolveMode::ForSetup)?;

    let dst_kernel = home.join(KERNEL_FILE);
    let dst_initrd = home.join(INITRD_FILE);
    let dst_snapshot = home.join(SNAPSHOT_FILE);
    let dst_version = home.join(VERSION_FILE);

    if image_installed(&home) && dst_snapshot.is_file() && !args.force {
        eprintln!(
            "pyhl: image already installed at {} (use --force to overwrite)",
            home.display()
        );
        eprintln!("  kernel:   {}", dst_kernel.display());
        eprintln!("  initrd:   {}", dst_initrd.display());
        eprintln!("  snapshot: {}", dst_snapshot.display());
        return Ok(());
    }

    fs::create_dir_all(&home).with_context(|| format!("create image home {}", home.display()))?;

    let (source_label, src_kernel, src_initrd) = match args.from.as_deref() {
        Some(dir) => {
            let (k, i) = discover_source_artifacts(dir)
                .with_context(|| format!("scanning {} for image artifacts", dir.display()))?;
            (dir.display().to_string(), k, i)
        }
        None => {
            // No --from: pull from GHCR. Uses docker or podman under the hood
            // because that's the standard OCI client everyone has and avoids
            // linking an oci-distribution client into pyhl.
            let tag = args.tag.as_deref();
            let kernel_image = ghcr_image_ref(GHCR_KERNEL_REPO, tag);
            let initrd_image = ghcr_image_ref(GHCR_INITRD_REPO, tag);
            eprintln!("pyhl: downloading image from GHCR…");
            let tmp = home.join(".pyhl.download");
            fs::create_dir_all(&tmp)?;
            let kernel_path = tmp.join("kernel");
            let initrd_path = tmp.join("initrd.cpio");
            extract_from_ghcr(&kernel_image, "/kernel", &kernel_path)?;
            extract_from_ghcr(&initrd_image, "/initrd.cpio", &initrd_path)?;
            (
                format!("{kernel_image} + {initrd_image}"),
                kernel_path,
                initrd_path,
            )
        }
    };

    copy_replace(&src_kernel, &dst_kernel)
        .with_context(|| format!("install {}", dst_kernel.display()))?;
    copy_replace(&src_initrd, &dst_initrd)
        .with_context(|| format!("install {}", dst_initrd.display()))?;

    // Remove the download scratch dir if we made one.
    let scratch = home.join(".pyhl.download");
    if scratch.is_dir() {
        let _ = fs::remove_dir_all(&scratch);
    }

    // Warm up a sandbox, take a snapshot after Py_Initialize + preloaded
    // imports, and persist it to disk. Every subsequent `pyhl run`
    // will MultiUseSandbox::from_snapshot() this file, which skips both
    // kernel boot (evolve) and the 3.5s Python warmup — the whole cost
    // is paid here, once.
    //
    // If --mount was passed, also tell the guest to mount hostfs at the
    // given guest path(s) during boot. The guest-side mount point is
    // baked into the snapshot's memory image; at `pyhl run --mount` the
    // host_dir side is remappable but the guest path is fixed.
    let setup_preopens: Vec<Preopen> = args
        .mounts
        .iter()
        .map(|m| parse_mount(m))
        .collect::<Result<_>>()?;
    let listen_ports = build_listen_ports(&args.port);
    let network = build_network_policy(
        args.net,
        &args.net_allow,
        &args.net_block,
        listen_ports.is_some(),
    )?;

    eprintln!("pyhl: warming up Python and persisting snapshot…");
    let t_warm = Instant::now();
    {
        let mut builder = Sandbox::builder(&dst_kernel)
            .initrd_file(&dst_initrd)
            .heap_size(3 * 512 * 1024 * 1024);
        for p in &setup_preopens {
            builder = builder.preopen(p.clone());
        }
        if let Some(ref policy) = network {
            builder = builder.network(policy.clone());
        }
        if let Some(ref lp) = listen_ports {
            builder = builder.listen_ports(lp.clone());
        }
        let mut sbox = builder.build()?;
        sbox.restore()?;
        let _: () = sbox.call_named("run", "pass".to_string())?;
        sbox.snapshot_now()?;
        sbox.save_snapshot(&dst_snapshot)?;
    }
    eprintln!(
        "pyhl:   warmup + persist = {:.1}s (one-time)",
        t_warm.elapsed().as_secs_f64()
    );

    let version = format!(
        "pyhl {pyhl_ver}\nsource: {src}\nkernel: {kern}\ninitrd: {initrd}\nsnapshot: {snap}\ninstalled: {ts}\n",
        pyhl_ver = env!("CARGO_PKG_VERSION"),
        src = source_label,
        kern = src_kernel.display(),
        initrd = src_initrd.display(),
        snap = dst_snapshot.display(),
        ts = now_iso8601(),
    );
    fs::write(&dst_version, version)?;

    eprintln!("pyhl: installed image to {}", home.display());
    eprintln!(
        "  kernel:   {} ({} MiB)",
        dst_kernel.display(),
        mib(&dst_kernel)
    );
    eprintln!(
        "  initrd:   {} ({} MiB)",
        dst_initrd.display(),
        mib(&dst_initrd)
    );
    eprintln!(
        "  snapshot: {} ({} MiB on disk, {} MiB apparent)",
        dst_snapshot.display(),
        disk_mib(&dst_snapshot),
        mib(&dst_snapshot)
    );
    Ok(())
}

fn mib(p: &Path) -> u64 {
    fs::metadata(p).map(|m| m.len() / 1024 / 1024).unwrap_or(0)
}

#[cfg(unix)]
fn disk_mib(p: &Path) -> u64 {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(p)
        .map(|m| m.blocks() * 512 / 1024 / 1024)
        .unwrap_or_else(|_| mib(p))
}

#[cfg(windows)]
fn disk_mib(p: &Path) -> u64 {
    use std::os::windows::ffi::OsStrExt;
    let wide: Vec<u16> = p
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut high: u32 = 0;
    let low = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetCompressedFileSizeW(wide.as_ptr(), &mut high)
    };
    if low == u32::MAX {
        return mib(p);
    }
    let bytes = ((high as u64) << 32) | (low as u64);
    bytes / 1024 / 1024
}

#[cfg(not(any(unix, windows)))]
fn disk_mib(p: &Path) -> u64 {
    mib(p)
}

/// Lightweight timestamp (seconds since epoch in ISO-8601-ish) so we don't
/// need to pull chrono just for the VERSION stamp.
fn now_iso8601() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{secs}")
}

// -- `run` --------------------------------------------------------------------

fn cmd_run(args: RunArgs) -> Result<()> {
    let code = match (args.script.as_deref(), args.code.as_deref()) {
        (Some(_), Some(_)) => bail!("pass either <SCRIPT> or -c <CODE>, not both"),
        (Some(p), None) => {
            fs::read_to_string(p).with_context(|| format!("read script {}", p.display()))?
        }
        (None, Some(c)) => c.to_string(),
        (None, None) => bail!("provide a script path or -c <CODE>"),
    };

    let home = resolve_home(args.dest.as_deref(), ResolveMode::ForRun)?;
    let snapshot = home.join(SNAPSHOT_FILE);

    // Fast path: `pyhl setup` already warmed up a sandbox, ran
    // Py_Initialize + preloaded modules, captured the state, and
    // persisted it to snapshot.hls. Here we mmap that file back and
    // instantiate a sandbox directly — no kernel boot, no Python init.
    if !snapshot.is_file() {
        return Err(anyhow!(
            "no warmed-up snapshot at {}.\n\
             run `pyhl setup` first (or `pyhl setup --force` if you have\n\
             an older install without the snapshot file).",
            snapshot.display()
        ));
    }

    // If --mount was passed, parse the specs and register fs_* host
    // handlers on the loaded sandbox so guest file I/O routes back here.
    let run_preopens: Vec<Preopen> = args
        .mounts
        .iter()
        .map(|m| parse_mount(m))
        .collect::<Result<_>>()?;
    let listen_ports = build_listen_ports(&args.port);
    let network = build_network_policy(
        args.net,
        &args.net_allow,
        &args.net_block,
        listen_ports.is_some(),
    )?;

    let initrd = home.join(INITRD_FILE);

    let t_load = Instant::now();
    let initrd_ref = if initrd.is_file() {
        Some(initrd.as_path())
    } else {
        None
    };
    let mut sandbox = Sandbox::from_snapshot_file_configured(
        &snapshot,
        &run_preopens,
        initrd_ref,
        network.as_ref(),
        listen_ports.as_ref(),
    )?;
    if args.verbose {
        eprintln!(
            "[pyhl] load_snapshot={:.1}ms",
            t_load.elapsed().as_secs_f64() * 1000.0
        );
    }

    // The loaded snapshot IS the warm state. On the first iteration we
    // can go straight to `call` — the sandbox is already at that state.
    // Restore between subsequent iterations to keep them hermetic
    // (rewinds globals + any stdout buffering from the previous call).
    let total = args.repeat + 1;
    for i in 1..=total {
        let restore_ms = if i == 1 {
            0.0
        } else {
            let t_restore = Instant::now();
            sandbox.restore()?;
            t_restore.elapsed().as_secs_f64() * 1000.0
        };

        // Reseed Python's RNGs with fresh host entropy unless the user
        // asked for bit-for-bit reproducibility across calls. Every run
        // picks up a new seed so np.random.randint / random.random
        // match python3's "different result every invocation" behavior.
        let payload = if args.deterministic {
            code.clone()
        } else {
            let mut full = String::with_capacity(code.len() + 256);
            full.push_str(&reseed_prelude());
            full.push_str(&code);
            full
        };

        sandbox.reset_exit_code();
        let t_call = Instant::now();
        let _: () = sandbox.call_named("run", payload)?;
        let call_ms = t_call.elapsed().as_secs_f64() * 1000.0;
        let exit_code = sandbox.last_exit_code();
        if args.verbose {
            eprintln!(
                "[pyhl] run {i}/{total} restore={restore_ms:.1}ms call={call_ms:.1}ms exit={exit_code} (hermetic)"
            );
        }
        if exit_code != 0 {
            std::process::exit(exit_code);
        }
    }

    Ok(())
}

/// Python prelude that re-seeds `random` and (optionally) `numpy.random`
/// with fresh host entropy. Matches what each fresh `python3` invocation
/// would do automatically: `random.seed()` / `np.random.seed()` without
/// an argument pulls from `os.urandom()` at import time.
///
/// We seed from the host side because the guest's entropy source is
/// snapshotted too — calling `random.seed()` inside the guest without
/// arguments would re-read the same `os.urandom` state every time.
///
/// The helper names start with `_pyhl_` to avoid colliding with anything
/// the user might define; `del` cleans up so their namespace is tidy.
fn reseed_prelude() -> String {
    let seed = fresh_seed();
    format!(
        "import random as _pyhl_random\n\
         _pyhl_random.seed({seed})\n\
         try:\n\
         \x20   import numpy.random as _pyhl_nprnd\n\
         \x20   _pyhl_nprnd.seed({seed_lo})\n\
         \x20   del _pyhl_nprnd\n\
         except ImportError:\n\
         \x20   pass\n\
         del _pyhl_random\n",
        seed = seed,
        // numpy.random.seed accepts 0..=2**32-1, so take the low 32 bits.
        seed_lo = (seed as u32),
    )
}

/// Produce a fresh 128-bit seed per call. Mixes high-resolution wall
/// time, process/thread ids, and a monotonically-increasing counter so
/// repeated calls within the same process also differ.
fn fresh_seed() -> u128 {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id() as u128;
    let ctr = COUNTER.fetch_add(1, Ordering::Relaxed) as u128;
    // Mix via SplitMix-ish folding. Not cryptographic — we only need
    // different-enough bits per call, and Python/numpy's own RNGs do
    // the heavy lifting once seeded.
    let mut x = nanos.wrapping_mul(0x9E37_79B9_7F4A_7C15_9E37_79B9_7F4A_7C15);
    x ^= pid.wrapping_mul(0xBF58_476D_1CE4_E5B9_BF58_476D_1CE4_E5B9);
    x ^= ctr.wrapping_mul(0x94D0_49BB_1331_11EB_94D0_49BB_1331_11EB);
    x
}

// -- main ---------------------------------------------------------------------

fn main() -> Result<()> {
    hyperlight_unikraft::pyhl::default_surrogate_count();
    let cli = Cli::parse();
    match cli.cmd {
        Command::Setup(args) => cmd_setup(args),
        Command::Run(args) => cmd_run(args),
    }
}
