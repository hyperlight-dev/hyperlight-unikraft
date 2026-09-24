// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The project workflow: `hluk init` writes a project from a template,
//! `hluk build` compiles it or builds its rootfs, `hluk run` runs a guest
//! named on the command line or the project in the current directory,
//! `hluk pull` refreshes the rootfs a project names, and `hluk cache` shows
//! what has been pulled and snapshotted.  A project's `hluk.toml`
//! ([`manifest`]) is the run's flags written down; every one of them can
//! still be given on the command line.

pub mod cpio;
pub mod manifest;
pub mod prompt;
pub mod registry;
pub mod rootfs;
pub mod template;

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::time::{Instant, UNIX_EPOCH};

use clap::{Args, Subcommand};
use tracing::info;

use hyperlight_unikraft::{
    Error, Exec, Mount, RUNTIME_SCRATCH_MB, SNAPSHOT_KEY, SandboxBuilder, default_scratch_mb,
    runtime_scratch_mb,
};

use crate::{
    CliResult, PortSpec, base_builder, drive, elapsed_ms, parse_envs, parse_mounts,
    parse_net_policy, read_resolv_conf, resolve_exec,
};
use manifest::{Manifest, RootfsSource, Workload};
use rootfs::Fetch;
use template::{Template, Vars};

/// Arguments for `init`.
#[derive(Args)]
pub struct InitArgs {
    /// Directory to create the project in; `.` for the current one.
    /// Without it the project name is asked for and becomes the directory.
    dir: Option<PathBuf>,

    /// Template to start from; `hluk templates` lists them.  Asked for
    /// when omitted and a terminal is attached.
    #[arg(short, long, value_name = "NAME")]
    template: Option<String>,

    /// Project name (default: the directory's name).  Letters, digits,
    /// `-` and `_`, starting with a letter.
    #[arg(long)]
    name: Option<String>,

    /// Registry the rootfs images come from (also $HLUK_REGISTRY).
    /// [default: ghcr.io/hyperlight-dev/hyperlight-unikraft]
    #[arg(long, value_name = "HOST/PATH")]
    registry: Option<String>,

    /// Release whose images the project pins (also $HLUK_IMAGE_VERSION).
    /// For a build between releases, which has no images of its own.
    /// [default: this hluk's version]
    #[arg(long, value_name = "X.Y.Z")]
    image_version: Option<String>,

    /// Write the project but do not pull its rootfs; `hluk run` will.
    #[arg(long)]
    no_pull: bool,

    /// Overwrite files the template provides if they exist.
    #[arg(long)]
    force: bool,
}

/// Arguments the manifest-driven commands share.
#[derive(Args)]
pub struct ProjectArgs {
    /// The manifest, or a directory holding one [default: ./hluk.toml].
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    manifest: Option<PathBuf>,
}

/// Arguments for `run`: a guest named on the command line (`--initrd`,
/// `--runtime`, `--kernel`), or the project in the current directory, whose
/// manifest supplies what the flags do not.
#[derive(Args)]
pub struct RunArgs {
    /// Script file (.py, .js, …) to execute in the guest.
    #[arg(conflicts_with = "exec")]
    script: Option<PathBuf>,

    /// Path to a CPIO initrd to map into the guest.
    #[arg(long, conflicts_with_all = ["runtime", "file"])]
    initrd: Option<PathBuf>,

    /// A published runtime image by name (python, node, agent, …) or a
    /// full image reference, pulled into the cache when missing; runs warm
    /// unless --cold.
    #[arg(long, value_name = "NAME|IMAGE", conflicts_with = "file")]
    runtime: Option<String>,

    /// The project's manifest, or a directory holding one [default:
    /// ./hluk.toml, when neither --initrd nor --runtime names a guest].
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    file: Option<PathBuf>,

    /// Entry point binary path inside the initrd VFS.
    /// Auto-detected from the initrd if not specified.
    #[arg(long)]
    entry: Option<String>,

    /// Advanced: boot a kernel from this path instead of the embedded one.
    /// Must match the host ABI this build expects, or the guest will fault.
    /// Intended for kernel development.
    #[arg(long, value_name = "PATH", conflicts_with_all = ["runtime", "file"])]
    kernel: Option<PathBuf>,

    /// Guest memory in MiB. Default: the size the rootfs's runtime image is
    /// tested with.
    #[arg(long, value_name = "MIB")]
    scratch_mb: Option<usize>,

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

    /// Ports the guest may bind to for inbound connections: a port, a
    /// range LOW-HIGH, or `all` (repeatable). Without this flag, bind()
    /// is rejected (outbound-only).
    #[arg(long = "port", value_name = "PORT|LOW-HIGH|all")]
    ports: Vec<PortSpec>,

    /// Set an environment variable in the guest (repeatable).
    /// Format: KEY=VALUE (e.g. --env MY_VAR=hello --env DEBUG=1).
    #[arg(long = "env", value_name = "KEY=VALUE")]
    envs: Vec<String>,

    /// A resolver configuration file to install as the guest's
    /// /etc/resolv.conf, at boot and on a restore (nameservers, search
    /// domains, options). Without it the rootfs's own file stands.
    #[arg(long = "resolv-conf", value_name = "FILE")]
    resolv_conf: Option<PathBuf>,

    /// Restore this guest's warm snapshot from the cache, saving one first
    /// when there is none: the default for --runtime, and for a project
    /// whose manifest says warm = true.  (A snapshot is keyed by the
    /// embedded kernel, so not with --kernel.)
    #[arg(long, conflicts_with_all = ["cold", "kernel"])]
    warm: bool,

    /// Boot fresh from the rootfs, whatever the manifest says.
    #[arg(long)]
    cold: bool,

    /// Code the runtime runs once before the warm snapshot is taken, so
    /// what it loads is in it (the manifest's warm_exec).
    #[arg(long = "warm-exec", value_name = "CODE")]
    warm_exec: Option<String>,

    /// Run `hluk build` first (a project).
    #[arg(long, conflicts_with_all = ["initrd", "runtime", "kernel"])]
    build: bool,
}

/// Arguments for `cache`.
#[derive(Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    command: Option<CacheCommand>,
}

#[derive(Subcommand)]
enum CacheCommand {
    /// List the pulled rootfs images and the warm snapshots (the default),
    /// as paths under the cache directory.
    Ls {
        /// Only the warm snapshots.
        #[arg(long, conflicts_with = "rootfs")]
        snapshots: bool,
        /// Only the rootfs images.
        #[arg(long)]
        rootfs: bool,
    },

    /// Remove them: everything, or only --snapshots or --rootfs.
    Clean {
        #[arg(long)]
        snapshots: bool,
        #[arg(long)]
        rootfs: bool,
    },
}

// ── init ─────────────────────────────────────────────────────────

pub fn init(args: InitArgs) -> CliResult<()> {
    let templates = Template::all()?;
    let template = match args.template {
        Some(name) => Template::find(&name)?
            .ok_or_else(|| format!("no template {name:?}; `hluk templates` lists them"))?,
        None if prompt::interactive() => {
            let width = templates.iter().map(|t| t.name.len()).max().unwrap_or(0);
            let chosen = prompt::select("Pick a template to start from:", &templates, |t| {
                format!("{:<width$}  {}", t.name, t.meta.description)
            })?;
            Template::find(chosen.name)?.expect("listed")
        }
        None => {
            return Err(
                "no terminal to ask on: pass --template <name> (`hluk templates` lists them)"
                    .into(),
            );
        }
    };

    let (dir, name) = match (args.dir, args.name) {
        (Some(dir), Some(name)) => (dir, name),
        (Some(dir), None) => {
            let name = dir_name(&dir)?;
            (dir, name)
        }
        (None, Some(name)) => (PathBuf::from(&name), name),
        (None, None) if prompt::interactive() => {
            let name = prompt::ask("Project name", Some(&format!("hello-{}", template.name)))?;
            (PathBuf::from(&name), name)
        }
        (None, None) => return Err("pass a directory to create the project in".into()),
    };
    validate_name(&name)?;

    let registry = registry::registry(args.registry);
    let version = registry::image_version(args.image_version);
    let image = registry::initrd_image(&registry, &template.meta.runtime, &version);
    let base = registry::base_image(&registry, &template.meta.runtime, &version);
    let files = template.render(&Vars {
        name: &name,
        version: &version,
        registry: &registry,
        image: &image,
        base: &base,
    });

    // An existing .gitignore is added to, never replaced: `hluk init .` in
    // a repository is the common case.  Anything else that exists needs
    // --force.
    if !args.force
        && let Some((rel, _)) = files
            .iter()
            .find(|(rel, _)| *rel != GITIGNORE && dir.join(native(rel)).exists())
    {
        return Err(format!(
            "{} exists; pass --force to overwrite the files the template provides",
            dir.join(native(rel)).display()
        )
        .into());
    }
    fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    for (rel, text) in &files {
        let path = dir.join(native(rel));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        let text = if *rel == GITIGNORE && path.is_file() {
            merge_gitignore(&fs::read_to_string(&path)?, text)
        } else {
            text.clone()
        };
        fs::write(&path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    }
    eprintln!(
        "Created {name} from the {} template in {}:",
        template.name,
        dir.display()
    );
    for (rel, _) in &files {
        eprintln!("  {rel}");
    }

    // Read back what was written: a template's manifest is validated the
    // same way a hand-edited one is.
    let manifest = Manifest::load(&dir.join(manifest::FILE_NAME))?;
    if let RootfsSource::Image(image) = manifest.rootfs()
        && !args.no_pull
    {
        let (path, fetch) = rootfs::ensure_initrd(image, false)?;
        match fetch {
            Fetch::Cached => eprintln!("Rootfs {image} is already cached at {}", path.display()),
            Fetch::Pulled { bytes } => eprintln!(
                "Rootfs cached at {} ({:.1} MiB pulled)",
                path.display(),
                rootfs::mib(bytes)
            ),
        }
    }

    eprintln!();
    eprintln!("Next:");
    if dir != Path::new(".") {
        eprintln!("  cd {}", dir.display());
    }
    if manifest.needs_build() {
        eprintln!("  hluk build");
    }
    eprintln!("  hluk run");
    Ok(())
}

/// The one template file that is merged into an existing one.
const GITIGNORE: &str = ".gitignore";

/// `existing` with the lines of `template` it does not have yet appended.
fn merge_gitignore(existing: &str, template: &str) -> String {
    let missing: Vec<&str> = template
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter(|line| !existing.lines().any(|have| have.trim() == line.trim()))
        .collect();
    if missing.is_empty() {
        return existing.to_string();
    }
    let mut out = existing.to_string();
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("\n# hluk\n");
    for line in missing {
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// A project name from the directory it goes in.
fn dir_name(dir: &Path) -> CliResult<String> {
    let dir = if dir.as_os_str().is_empty() || dir == Path::new(".") {
        std::env::current_dir()?
    } else {
        dir.to_path_buf()
    };
    let canonical = dir.canonicalize().unwrap_or(dir);
    canonical
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or_else(|| "cannot name the project after this directory; pass --name".into())
}

/// A name that works everywhere it ends up: a Cargo package, a Go module,
/// a Docker tag, a filename.
fn validate_name(name: &str) -> CliResult<()> {
    let ok = name.len() <= 64
        && name.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if ok {
        Ok(())
    } else {
        Err(format!(
            "project name {name:?}: use letters, digits, `-` and `_`, starting with a letter \
             (pass --name to choose one)"
        )
        .into())
    }
}

/// A template's `/`-separated path as a native one.
fn native(rel: &str) -> PathBuf {
    rel.split('/').collect()
}

// ── templates ────────────────────────────────────────────────────

pub fn templates() -> CliResult<()> {
    let templates = Template::all()?;
    let name_w = templates
        .iter()
        .map(|t| t.name.len())
        .max()
        .unwrap_or(4)
        .max(4);
    let runtime_w = templates
        .iter()
        .map(|t| t.meta.runtime.len())
        .max()
        .unwrap_or(7)
        .max(7);
    println!(
        "{:<name_w$}  TIER  {:<runtime_w$}  DESCRIPTION",
        "NAME", "RUNTIME"
    );
    for t in &templates {
        println!(
            "{:<name_w$}  {:>4}  {:<runtime_w$}  {}",
            t.name, t.meta.tier, t.meta.runtime, t.meta.description
        );
    }
    println!();
    println!(
        "Start one with `hluk init <dir> --template <name>`; tiers are docs/guest-support-tiers.md."
    );
    Ok(())
}

// ── pull ─────────────────────────────────────────────────────────

pub fn pull(args: ProjectArgs) -> CliResult<()> {
    let manifest = Manifest::load(&Manifest::locate(args.manifest.as_deref())?)?;
    match manifest.rootfs() {
        RootfsSource::Image(image) => {
            let (path, fetch) = rootfs::ensure_initrd(image, true)?;
            match fetch {
                Fetch::Cached => eprintln!("{image} is up to date at {}", path.display()),
                Fetch::Pulled { bytes } => eprintln!(
                    "cached at {} ({:.1} MiB pulled)",
                    path.display(),
                    rootfs::mib(bytes)
                ),
            }
        }
        RootfsSource::Dockerfile(path) => eprintln!(
            "nothing to pull: the rootfs is built from {} by `hluk build`",
            path.display()
        ),
        RootfsSource::Path(path) => {
            eprintln!(
                "nothing to pull: the rootfs is the local file {}",
                path.display()
            )
        }
    }
    Ok(())
}

// ── build ────────────────────────────────────────────────────────

pub fn build(args: ProjectArgs) -> CliResult<()> {
    let manifest = Manifest::load(&Manifest::locate(args.manifest.as_deref())?)?;
    run_build(&manifest)
}

/// The build command first, so a Dockerfile can pick up what it made.
fn run_build(manifest: &Manifest) -> CliResult<()> {
    if !manifest.needs_build() {
        eprintln!("nothing to build: hluk.toml has no [build] command and no [rootfs] dockerfile");
        return Ok(());
    }
    if let Some(build) = &manifest.build {
        // The toolchain is the project's, not hluk's: say which one is
        // missing rather than let the shell's "not found" stand alone.
        if let Some(program) = command_program(&build.command)
            && !on_path(program)
        {
            return Err(format!(
                "the build command needs `{program}`, which is not on PATH: install it, or \
                 build elsewhere and point the manifest at the result"
            )
            .into());
        }
        eprintln!("$ {}", build.command);
        let status = shell(&build.command, &manifest.dir)
            .map_err(|e| format!("cannot run the build command: {e}"))?;
        if !status.success() {
            let why = if status.code() == Some(127) {
                ": a program it names is not installed"
            } else {
                ""
            };
            return Err(format!("the build command failed ({status}){why}").into());
        }
        // What it built is what the guest will exec: check it now.
        if let Workload::Exec(command) = manifest.workload() {
            check_pie(&command, &manifest.mounts()?)?;
        }
    }
    if let RootfsSource::Dockerfile(dockerfile) = manifest.rootfs() {
        let (path, stats) = rootfs::build_dockerfile(manifest, dockerfile)?;
        eprintln!(
            "rootfs: {} ({} entries, {:.1} MiB)",
            path.display(),
            stats.entries,
            rootfs::mib(stats.bytes)
        );
        let size = fs::metadata(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?
            .len();
        scratch_hint(size, manifest.scratch_mb(&path));
    }
    Ok(())
}

/// The rootfs is unpacked into the guest's memory, so one that takes more
/// than half of it leaves the runtime little to work with: an image that
/// grew past the size its runtime is tested with (packages added by a
/// Dockerfile) needs `scratch_mb` set.  Say so, with a size that leaves
/// room, rather than let the guest fail to unpack it.
fn scratch_hint(rootfs_bytes: u64, scratch_mb: usize) {
    let rootfs_mb = rootfs_bytes.div_ceil(1024 * 1024) as usize;
    if rootfs_mb * 2 > scratch_mb {
        let suggested = (rootfs_mb * 4).next_multiple_of(256);
        eprintln!(
            "note: the rootfs is {rootfs_mb} MiB and [run] scratch_mb = {scratch_mb}; the rootfs \
             is unpacked into that memory, so set scratch_mb to {suggested} or more"
        );
    }
}

/// Shell builtins and syntax a build command may start with, which no PATH
/// lookup would find; a command starting with one is left to the shell.
const SHELL_BUILTINS: &[&str] = &[
    "cd", "export", "set", "source", ".", "eval", "exec", "echo", "true", "false", "test", "[",
    "if", "for", "while", "case", "mkdir",
];

/// The program a shell command line starts with, past any `KEY=VALUE`
/// assignments and grouping (`CGO_ENABLED=0 go build …` starts `go`, `(cd
/// x && make)` starts `cd`); `None` when it is the shell's own.
fn command_program(command: &str) -> Option<&str> {
    let program = command
        .split_whitespace()
        .map(|word| word.trim_start_matches(['(', '{']))
        .find(|word| !word.is_empty() && !word.contains('='))?;
    (!SHELL_BUILTINS.contains(&program)).then_some(program)
}

/// Whether `program` resolves: a path that exists, or a name found in PATH
/// (with the executable extensions Windows adds).
fn on_path(program: &str) -> bool {
    if program.contains('/') || program.contains('\\') {
        return Path::new(program).exists();
    }
    let Some(path) = std::env::var_os("PATH") else {
        return true;
    };
    let exts: Vec<String> = if cfg!(windows) {
        std::env::var("PATHEXT")
            .unwrap_or_else(|_| ".EXE;.CMD;.BAT".into())
            .split(';')
            .map(|e| e.to_ascii_lowercase())
            .collect()
    } else {
        Vec::new()
    };
    std::env::split_paths(&path).any(|dir| {
        dir.join(program).is_file()
            || exts
                .iter()
                .any(|ext| dir.join(format!("{program}{ext}")).is_file())
    })
}

/// Run a command line through the platform's shell in `dir`.
fn shell(command: &str, dir: &Path) -> std::io::Result<ExitStatus> {
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.args(["/C", command]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", command]);
        c
    };
    cmd.current_dir(dir).status()
}

/// A program the exec driver runs from a host mount must be a PIE: the
/// guest's ELF loader maps it into its own address space.  Read the header
/// here and say how to fix it, rather than let the guest report "Image
/// format not recognized".  Anything that is not an ELF file under a mount
/// is left alone: code for an interpreter, a path inside the rootfs.
fn check_pie(command: &str, mounts: &[Mount]) -> CliResult<()> {
    let Some(program) = command.split_whitespace().next() else {
        return Ok(());
    };
    let Some(host) = host_path(program, mounts) else {
        return Ok(());
    };
    let Ok(mut file) = fs::File::open(&host) else {
        return Ok(());
    };
    let mut header = [0u8; 18];
    if file.read_exact(&mut header).is_err() || &header[..4] != b"\x7fELF" {
        return Ok(());
    }
    // e_type at offset 16: 2 is ET_EXEC (linked at a fixed address), 3 is
    // ET_DYN (a PIE, or a shared object).
    let e_type = u16::from_le_bytes([header[16], header[17]]);
    if e_type == 2 {
        return Err(format!(
            "{} is not position-independent (ET_EXEC), and the guest's ELF loader needs a PIE. \
             Build with -buildmode=pie (Go), -fPIE -static-pie (C), -C relocation-model=pie \
             (Rust); .NET publishes one by default",
            host.display()
        )
        .into());
    }
    Ok(())
}

/// The host file a guest path names through the mounts, if one covers it.
fn host_path(guest: &str, mounts: &[Mount]) -> Option<PathBuf> {
    mounts.iter().find_map(|m| {
        let rest = guest.strip_prefix(m.guest_path.trim_end_matches('/'))?;
        match rest.strip_prefix('/') {
            Some(rest) => Some(m.host_path.join(rest)),
            None if rest.is_empty() => Some(m.host_path.clone()),
            // `/mnt/ap` does not cover `/mnt/app/x`.
            None => None,
        }
    })
}

// ── run ──────────────────────────────────────────────────────────

pub fn run(args: RunArgs) -> CliResult<()> {
    // Where the guest comes from: a rootfs or kernel named on the command
    // line, a published image by name, or the project in this directory.
    let explicit = args.kernel.is_some() || args.initrd.is_some();
    let mut manifest = None;
    let mut rootfs: Option<PathBuf> = args.initrd.clone();
    if let Some(path) = &args.initrd
        && !path.is_file()
    {
        return Err(format!("--initrd {} does not exist", path.display()).into());
    }
    if let Some(runtime) = &args.runtime {
        check_runtime_name(runtime)?;
        rootfs = Some(rootfs::ensure_initrd(&registry::runtime_image(runtime), false)?.0);
    } else if !explicit {
        let m = Manifest::load(&Manifest::locate(args.file.as_deref())?)?;
        if args.build {
            run_build(&m)?;
        }
        rootfs = Some(project_rootfs(&m)?);
        manifest = Some(m);
    }

    // What to run: the command line's, else the manifest's.
    let exec = match resolve_exec(args.script.clone(), args.exec.clone())? {
        Some(exec) => Some(exec),
        None => match &args.guest_exec {
            Some(command) => Some(Exec::Guest(command.clone())),
            None => manifest.as_ref().and_then(|m| match m.workload() {
                Workload::Script(path) => Some(Exec::File(path)),
                Workload::Exec(code) => Some(Exec::Code(code)),
                Workload::GuestExec(command) => Some(Exec::Guest(command)),
                Workload::None => None,
            }),
        },
    };
    if let Some(Exec::File(path)) = &exec
        && !path.is_file()
    {
        return Err(format!("script {} does not exist", path.display()).into());
    }
    let no_workload = exec.is_none();
    let exec = exec.unwrap_or_else(|| Exec::Guest(String::new()));

    // Capabilities: the manifest's, with the command line's added (mounts,
    // environment) or in their place (network).
    let mut mounts = match &manifest {
        Some(m) => m.mounts()?,
        None => Vec::new(),
    };
    mounts.extend(parse_mounts(&args.mounts)?);
    let net_given = args.net
        || !args.net_allow.is_empty()
        || !args.net_block.is_empty()
        || !args.ports.is_empty();
    let (policy, listen) = if net_given {
        parse_net_policy(args.net, &args.net_allow, &args.net_block, &args.ports)?
    } else if let Some(m) = &manifest {
        m.network()?
    } else {
        (None, None)
    };
    let mut env: Vec<(String, String)> = manifest
        .as_ref()
        .map(|m| {
            m.run
                .env
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
        .unwrap_or_default();
    env.extend(
        parse_envs(&args.envs)?
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string())),
    );
    let resolv_path = args.resolv_conf.clone().or_else(|| {
        manifest
            .as_ref()
            .and_then(|m| m.run.resolv_conf.as_ref().map(|p| m.resolve(p)))
    });
    let resolv = read_resolv_conf(resolv_path.as_ref())?;
    let entry = args
        .entry
        .clone()
        .or_else(|| manifest.as_ref().and_then(|m| m.run.entry.clone()));
    let warm_exec = args
        .warm_exec
        .clone()
        .or_else(|| manifest.as_ref().and_then(|m| m.run.warm_exec.clone()));
    let warm = if args.cold {
        false
    } else if args.warm {
        true
    } else {
        manifest
            .as_ref()
            .map_or(args.runtime.is_some(), |m| m.run.warm)
    };

    // Memory: the flag, the manifest's, else the size the runtime image is
    // tested with, by name for --runtime and a published image, by the
    // driver in the rootfs otherwise.  A kernel with no rootfs takes the
    // library's default.
    let scratch_mb = args.scratch_mb.or_else(|| {
        let path = rootfs.as_deref()?;
        Some(match (&manifest, &args.runtime) {
            (Some(m), _) => m.scratch_mb(path),
            (None, Some(runtime)) => registry::runtime_name(runtime)
                .and_then(runtime_scratch_mb)
                .unwrap_or_else(|| default_scratch_mb(path)),
            (None, None) => default_scratch_mb(path),
        })
    });
    if let (Some(path), Some(mb)) = (&rootfs, scratch_mb) {
        let size = fs::metadata(path)
            .map_err(|e| format!("cannot read the rootfs {}: {e}", path.display()))?
            .len();
        scratch_hint(size, mb);
    }
    if let Exec::Code(command) = &exec {
        check_pie(command, &mounts)?;
    }
    if let Some(entry) = &entry {
        check_pie(entry, &mounts)?;
    }

    // A warm run restores the cached snapshot of exactly this guest, when
    // there is one, and saves one otherwise.
    let warm_key = match (warm, &rootfs, scratch_mb) {
        (false, _, _) => None,
        (true, Some(path), Some(mb)) => {
            Some(stamp(path, mb, entry.as_deref(), warm_exec.as_deref())?)
        }
        (true, _, _) => {
            return Err("--warm needs a rootfs: a project, --runtime or --initrd".into());
        }
    };
    let mut restored = false;
    let mut builder = None;
    if let Some(key) = &warm_key
        && let Some(dir) = rootfs::find_snapshot(key)?
    {
        match SandboxBuilder::from_snapshot_dir(&dir) {
            Ok(b) => {
                builder = Some(b);
                restored = true;
            }
            Err(e) => info!(error = %e, "cached warm snapshot cannot be used; booting fresh"),
        }
    }
    let mut builder = match builder {
        Some(b) => b,
        None => {
            let mut b = if explicit {
                base_builder(args.kernel.clone(), args.initrd.clone())?
            } else if let Some(path) = &rootfs {
                SandboxBuilder::from_initrd(path)
            } else {
                return Err(
                    "no workload: pass --initrd <rootfs.cpio>, --kernel <kernel> or \
                            --runtime <name>, or run in a project directory (hluk init)"
                        .into(),
                );
            };
            if let Some(mb) = scratch_mb {
                b = b.scratch_mb(mb);
            }
            if let Some(e) = &entry {
                b = b.entry(e);
            }
            b
        }
    };
    builder = builder.mounts(mounts);
    if let Some(policy) = policy {
        builder = builder.network(policy);
    }
    if let Some(listen) = listen {
        builder = builder.listen_ports(listen);
    }
    for (key, value) in env {
        builder = builder.env(key, value);
    }
    if let Some(rc) = resolv {
        builder = builder.resolv_conf(rc);
    }

    let t = Instant::now();
    let mut sandbox = builder.boot()?;
    info!(elapsed_ms = elapsed_ms(t), restored, "boot");

    if let Some(key) = warm_key
        && !restored
    {
        let t = Instant::now();
        // What the project wants loaded before the snapshot: the runtime
        // driver runs it like any call, and what it leaves behind (modules
        // in the interpreter's cache, JIT'd code) is captured.
        if let Some(code) = &warm_exec {
            if !sandbox.has_driver() {
                return Err("warm_exec needs a runtime driver in the rootfs".into());
            }
            sandbox.run(code.as_str())?;
            info!(elapsed_ms = elapsed_ms(t), "warm_exec done");
        }
        // Written beside its slot and moved in whole, so a run that dies
        // half way leaves no snapshot that looks complete.
        let slot = rootfs::snapshot_slot(&key)?;
        let part = slot.with_extension("part");
        let _ = fs::remove_dir_all(&part);
        if let Some(parent) = slot.parent() {
            fs::create_dir_all(parent)?;
        }
        match sandbox.snapshot_to(&part) {
            Ok(_) => {
                fs::write(part.join(rootfs::STAMP_FILE), &key)?;
                // Remove any earlier snapshot and move ours in.  If a
                // concurrent `hluk run` placed an equivalent one between
                // our remove and rename, the rename fails; that snapshot
                // is the same (same stamp hash), so ours is redundant.
                let _ = fs::remove_dir_all(&slot);
                if fs::rename(&part, &slot).is_err() {
                    if slot.is_dir() {
                        // Another run saved an equivalent snapshot first.
                        let _ = fs::remove_dir_all(&part);
                    } else {
                        fs::rename(&part, &slot)?;
                    }
                }
                let pruned = rootfs::prune_superseded(&key, &slot)?;
                info!(
                    elapsed_ms = elapsed_ms(t),
                    path = %slot.display(),
                    pruned,
                    "warm snapshot saved"
                );
            }
            // An entry-point program that ran to completion during boot
            // leaves nothing to warm; the run still reports how it ended.
            Err(Error::GuestExited { .. }) => {
                let _ = fs::remove_dir_all(&part);
                info!("the guest exited during boot; no warm snapshot saved");
            }
            Err(e) => {
                let _ = fs::remove_dir_all(&part);
                return Err(e.into());
            }
        }
    }

    drive(&mut sandbox, no_workload, exec)
}

/// A bare `--runtime` name must be one of the published images; a full
/// reference is anyone's to name.
pub fn check_runtime_name(spec: &str) -> CliResult<()> {
    if spec.contains('/') || runtime_scratch_mb(spec).is_some() {
        return Ok(());
    }
    let known: Vec<&str> = RUNTIME_SCRATCH_MB.iter().map(|(name, _)| *name).collect();
    Err(format!(
        "no runtime image called {spec:?}; the published ones are {}, or pass a full image \
         reference",
        known.join(", ")
    )
    .into())
}

/// The project's rootfs on disk, pulling a published image when needed.
fn project_rootfs(manifest: &Manifest) -> CliResult<PathBuf> {
    Ok(match manifest.rootfs() {
        RootfsSource::Image(image) => rootfs::ensure_initrd(image, false)?.0,
        RootfsSource::Dockerfile(_) => {
            let path = rootfs::built_rootfs(manifest);
            if !path.is_file() {
                return Err(
                    "the rootfs has not been built yet: run `hluk build`, or `hluk run --build`"
                        .into(),
                );
            }
            path
        }
        RootfsSource::Path(path) => {
            let path = manifest.resolve(path);
            if !path.is_file() {
                return Err(format!("[rootfs] path {} does not exist", path.display()).into());
            }
            path
        }
    })
}

/// What a warm snapshot depends on: the build's snapshot key, the rootfs
/// file (by path, size and mtime), the guest's memory and entry point, and
/// the code run before the snapshot.  Mounts, network policy and
/// environment are supplied on restore, so they can change without one.
/// The text names the cache slot and is stored in it.
fn stamp(
    rootfs: &Path,
    scratch_mb: usize,
    entry: Option<&str>,
    warm_exec: Option<&str>,
) -> CliResult<String> {
    let canonical = rootfs
        .canonicalize()
        .map_err(|e| format!("cannot read the rootfs {}: {e}", rootfs.display()))?;
    let meta = fs::metadata(&canonical)
        .map_err(|e| format!("cannot read the rootfs {}: {e}", canonical.display()))?;
    let mtime = meta
        .modified()?
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    Ok(format!(
        "key={SNAPSHOT_KEY}\nrootfs={}\nsize={}\nmtime={mtime}\nscratch_mb={scratch_mb}\nentry={}\nwarm_exec={}\n",
        canonical.display(),
        meta.len(),
        entry.unwrap_or(""),
        warm_exec.unwrap_or("")
    ))
}

// ── cache ────────────────────────────────────────────────────────

pub fn cache(args: CacheArgs) -> CliResult<()> {
    match args.command.unwrap_or(CacheCommand::Ls {
        snapshots: false,
        rootfs: false,
    }) {
        CacheCommand::Ls {
            snapshots: only_snapshots,
            rootfs: only_rootfs,
        } => {
            let root = rootfs::cache_dir()?;
            println!("cache: {}", root.display());
            // Entries are shown under the root, and a snapshot's rootfs too
            // when it is a pulled image, so a line stays readable.
            let rel = |p: &Path| {
                p.strip_prefix(&root)
                    .map(|r| r.display().to_string())
                    .unwrap_or_else(|_| p.display().to_string())
            };
            if !only_snapshots {
                let images = rootfs::list_rootfs()?;
                println!();
                println!("rootfs images ({})", images.len());
                for (path, bytes) in &images {
                    println!("  {}  {:.1} MiB", rel(path), rootfs::mib(*bytes));
                }
            }
            if !only_rootfs {
                let snapshots = rootfs::list_snapshots()?;
                println!();
                println!("warm snapshots ({})", snapshots.len());
                for s in &snapshots {
                    let described = s
                        .describe()
                        .replace(&format!("rootfs={}/", root.display()), "rootfs=");
                    println!(
                        "  {}  {:.1} MiB  {described}",
                        rel(&s.dir),
                        rootfs::mib(s.bytes)
                    );
                }
            }
            Ok(())
        }
        CacheCommand::Clean {
            snapshots,
            rootfs: images,
        } => {
            let both = !snapshots && !images;
            let mut freed = 0;
            if snapshots || both {
                freed += rootfs::remove_dir(&rootfs::snapshots_dir()?)?;
            }
            if images || both {
                freed += rootfs::remove_dir(&rootfs::cache_dir()?.join("rootfs"))?;
            }
            eprintln!("removed {:.1} MiB", rootfs::mib(freed));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_checked() {
        for ok in ["hello", "hello-world", "hello_2", "A"] {
            validate_name(ok).unwrap_or_else(|e| panic!("{ok}: {e}"));
        }
        for bad in ["", "1abc", "-x", "a b", "a/b", "a.b", &"x".repeat(65)] {
            assert!(validate_name(bad).is_err(), "{bad:?} accepted");
        }
    }

    #[test]
    fn build_command_program_skips_assignments() {
        assert_eq!(
            command_program("CGO_ENABLED=0 GOOS=linux go build -o bin/app ."),
            Some("go")
        );
        assert_eq!(command_program("cargo build --release"), Some("cargo"));
        assert_eq!(command_program("(cd src && make)"), None);
        assert_eq!(
            command_program("mkdir -p bin && gcc -o bin/app main.c"),
            None
        );
        assert_eq!(command_program("   "), None);
        assert!(on_path("sh") || on_path("cmd"));
        assert!(!on_path("hluk-no-such-program-xyz"));
    }

    #[test]
    fn gitignore_is_merged_not_replaced() {
        let merged = merge_gitignore("target/\n", "# comment\n.hluk/\ntarget/\n");
        assert_eq!(merged, "target/\n\n# hluk\n.hluk/\n");
        assert_eq!(merge_gitignore(".hluk/\n", ".hluk/\n"), ".hluk/\n");
        assert_eq!(merge_gitignore("", ".hluk/\n"), "\n# hluk\n.hluk/\n");
    }

    #[test]
    fn an_unknown_runtime_name_is_refused_before_the_network() {
        let err = check_runtime_name("pythn").unwrap_err().to_string();
        assert!(err.contains("python") && err.contains("pythn"), "{err}");
        check_runtime_name("python").unwrap();
        check_runtime_name("ghcr.io/o/r/anything:tag").unwrap();
    }

    #[test]
    fn template_paths_become_native() {
        assert_eq!(native("src/main.rs"), Path::new("src").join("main.rs"));
        assert_eq!(native("hluk.toml"), PathBuf::from("hluk.toml"));
    }

    #[test]
    fn stamp_changes_with_the_rootfs_and_settings() {
        let dir = tempfile::tempdir().unwrap();
        let rootfs = dir.path().join("r.cpio");
        fs::write(&rootfs, b"070701").unwrap();
        let a = stamp(&rootfs, 256, None, None).unwrap();
        assert_eq!(a, stamp(&rootfs, 256, None, None).unwrap());
        assert_ne!(a, stamp(&rootfs, 512, None, None).unwrap());
        assert_ne!(a, stamp(&rootfs, 256, Some("/bin/app"), None).unwrap());
        assert_ne!(a, stamp(&rootfs, 256, None, Some("import flask")).unwrap());
        fs::write(&rootfs, b"0707010").unwrap();
        assert_ne!(a, stamp(&rootfs, 256, None, None).unwrap());
        assert!(a.contains(SNAPSHOT_KEY));
    }

    #[test]
    fn a_guest_path_maps_to_its_host_file_through_the_mounts() {
        let mounts = [
            Mount::ro("/host/bin", "/mnt/app"),
            Mount::rw("/data", "/mnt/data"),
        ];
        assert_eq!(
            host_path("/mnt/app/app", &mounts),
            Some(PathBuf::from("/host/bin/app"))
        );
        assert_eq!(
            host_path("/mnt/app", &mounts),
            Some(PathBuf::from("/host/bin"))
        );
        let slashed = [Mount::ro("/host/bin", "/mnt/app/")];
        assert_eq!(
            host_path("/mnt/app/app", &slashed),
            Some(PathBuf::from("/host/bin/app"))
        );
        assert_eq!(
            host_path("/mnt/data/x/y.csv", &mounts),
            Some(PathBuf::from("/data/x/y.csv"))
        );
        assert_eq!(host_path("/mnt/application", &mounts), None);
        assert_eq!(host_path("/usr/bin/python3", &mounts), None);
    }

    #[test]
    fn a_fixed_address_binary_is_refused_and_a_pie_is_not() {
        let dir = tempfile::tempdir().unwrap();
        let mut exec = vec![0u8; 64];
        exec[..4].copy_from_slice(b"\x7fELF");
        exec[16..18].copy_from_slice(&2u16.to_le_bytes());
        fs::write(dir.path().join("fixed"), &exec).unwrap();
        let mut pie = exec.clone();
        pie[16..18].copy_from_slice(&3u16.to_le_bytes());
        fs::write(dir.path().join("pie"), &pie).unwrap();
        fs::write(dir.path().join("script"), b"print('hi')").unwrap();
        let mounts = [Mount::ro(dir.path(), "/mnt/app")];
        let err = check_pie("/mnt/app/fixed --flag", &mounts)
            .unwrap_err()
            .to_string();
        assert!(err.contains("not position-independent"), "{err}");
        check_pie("/mnt/app/pie", &mounts).unwrap();
        check_pie("/mnt/app/script", &mounts).unwrap();
        check_pie("/mnt/app/missing", &mounts).unwrap();
        check_pie("print(1)", &mounts).unwrap();
    }
}
