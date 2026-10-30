// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The project workflow: `hluk init` writes a project from a template,
//! `hluk build` compiles it or builds its rootfs, `hluk run` runs a guest
//! named on the command line or the project in the current directory,
//! `hluk pull` refreshes the rootfs a project names, and `hluk cache` shows
//! what has been pulled and snapshotted.  A project's `hluk.toml`
//! ([`manifest`]) is the run's flags written down; every one of them can
//! still be given on the command line.

pub mod github;
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
    CONTAINER_SCRATCH_HEADROOM_MB, DEFAULT_SCRATCH_MB, Error, Exec, MIB, Mount, RUNTIME_SCRATCH_MB,
    SNAPSHOT_KEY, SandboxBuilder, cgroup_memory_limit_mb, default_scratch_mb, driver_entry,
    is_runtime_driver, runtime_scratch_mb, scratch_mb_from, scratch_mb_within,
};

use crate::{
    Call, CliResult, PortSpec, base_builder, boot, drive, elapsed_ms, parse_envs, parse_mounts,
    parse_net_policy, read_resolv_conf, resolve_exec,
};
use manifest::{Manifest, RootfsSource, Workload};
use rootfs::Fetch;
use template::{Origin, Template, Vars};

/// Arguments for `init`.
#[derive(Args)]
pub struct InitArgs {
    /// Directory to create the project in; `.` for the current one.
    /// Without it the project name is asked for and becomes the directory.
    dir: Option<PathBuf>,

    /// Template to start from: a built-in one (`hluk templates` lists
    /// them), a template directory, or github.com/OWNER/REPO[/PATH][@REF].
    /// Asked for when omitted and a terminal is attached.
    #[arg(short, long, value_name = "NAME|DIR|URL")]
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

    /// Guest memory in MiB, or `max`: what the memory limit hluk runs under
    /// (a container's) leaves once hluk has its own share. Default: the size
    /// the rootfs's runtime image is tested with.
    #[arg(long, value_name = "MIB|max")]
    scratch_mb: Option<ScratchArg>,

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

    /// Call a function the guest has defined, once the workload has run,
    /// and print its result: a handler the script defined, an export of the
    /// WebAssembly library --guest-exec loaded. With no workload, only the
    /// call runs (on a handler --warm-exec defined, say). The quickjs, node,
    /// python, dotnet-jit, java, php and wasmtime images serve guest
    /// function calls.
    #[arg(long, value_name = "FUNCTION")]
    call: Option<String>,

    /// The call's input: JSON, passed to the function as its argument (for
    /// WebAssembly, an array of its arguments). Default: none.
    #[arg(long, value_name = "JSON", requires = "call")]
    input: Option<String>,

    /// Print where the time went, host side, when the run ends: each VM
    /// entry (guest and host function time, VM exits), each host function,
    /// boot and restore. Also HLUK_PROFILE=1.
    #[arg(long)]
    profile: bool,

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
    /// range LOW-HIGH, or `all` (repeatable); replaces a project's ports.
    /// Without this flag or a project's ports, bind() is rejected but for
    /// port 0: the guest may serve only on a port the host picks, and may
    /// not choose its source port either. Without --net, --net-allow,
    /// --net-block or a project's policy, the guest serves on them but may
    /// connect nowhere.
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
    let template = match args.template {
        Some(spec) => Template::resolve(&spec)?,
        None if prompt::interactive() => {
            let templates = Template::all()?;
            let width = templates.iter().map(|t| t.name.len()).max().unwrap_or(0);
            let chosen = prompt::select("Pick a template to start from:", &templates, |t| {
                format!("{:<width$}  {}", t.name, t.meta.description)
            })?;
            Template::find(&chosen.name)?.expect("listed")
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
            let name = prompt::ask("Project name", Some(&suggested_name(&template.name)))?;
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

    // The manifest is validated the way a hand-edited one is, and before
    // anything is written: a template with a bad one leaves no half-made
    // project behind.
    let rendered = files
        .iter()
        .find(|f| f.path == manifest::FILE_NAME)
        .ok_or_else(|| format!("template {}: no {}", template.name, manifest::FILE_NAME))?;
    Manifest::parse(&rendered.text)
        .map_err(|e| format!("template {}: {}: {e}", template.name, manifest::FILE_NAME))?;

    // An existing .gitignore is added to, never replaced: `hluk init .` in
    // a repository is the common case.  Anything else that exists needs
    // --force.
    if !args.force
        && let Some(f) = files
            .iter()
            .find(|f| f.path != GITIGNORE && dir.join(native(f.path)).exists())
    {
        return Err(format!(
            "{} exists; pass --force to overwrite the files the template provides",
            dir.join(native(f.path)).display()
        )
        .into());
    }
    fs::create_dir_all(&dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    for f in &files {
        let path = dir.join(native(f.path));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        if f.path == GITIGNORE && path.is_file() {
            let merged = merge_gitignore(&fs::read_to_string(&path)?, &f.text);
            fs::write(&path, merged)
        } else {
            fs::write(&path, &f.text)
        }
        .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        if f.executable {
            set_executable(&path)?;
        }
    }
    match &template.origin {
        Origin::BuiltIn => eprintln!(
            "Created {name} from the {} template in {}:",
            template.name,
            dir.display()
        ),
        origin => eprintln!(
            "Created {name} from the template at {origin} in {}:",
            dir.display()
        ),
    }
    for f in &files {
        eprintln!("  {}", f.path);
    }

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

    // A template that is not built in is someone else's code: say what
    // `hluk build` would run on this host, and what of the host `hluk run`
    // would hand the guest, before anyone runs it.
    if template.origin != Origin::BuiltIn {
        for mount in &manifest.run.mounts {
            eprintln!("Note: `hluk run` mounts this host directory into the guest: {mount}");
        }
        if manifest.run.net.is_some() {
            eprintln!("Note: `hluk run` gives the guest network access ([run.net] in hluk.toml)");
        }
        if let Some(build) = &manifest.build {
            eprintln!(
                "Note: `hluk build` runs this template's command on this host: {}",
                build.command
            );
        }
        if let RootfsSource::Dockerfile(dockerfile) = manifest.rootfs() {
            eprintln!(
                "Note: `hluk build` builds this template's {} with Docker",
                dockerfile.display()
            );
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

/// A project name to offer for `template`: `hello-<template>`, with what a
/// project name cannot hold (a repository's `.`, a directory's spaces)
/// turned into `-`.
fn suggested_name(template: &str) -> String {
    let name: String = template
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    format!("hello-{}", name.trim_matches('-'))
}

#[cfg(unix)]
fn set_executable(path: &Path) -> CliResult<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(perms.mode() | 0o111);
    fs::set_permissions(path, perms)
        .map_err(|e| format!("cannot make {} executable: {e}", path.display()).into())
}

#[cfg(not(unix))]
fn set_executable(_: &Path) -> CliResult<()> {
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
            t.name,
            t.meta.tier.map(|t| t.to_string()).unwrap_or_default(),
            t.meta.runtime,
            t.meta.description
        );
    }
    println!();
    println!(
        "Start one with `hluk init <dir> --template <name>`; tiers are docs/guest-support-tiers.md."
    );
    println!(
        "--template also takes a template directory or github.com/OWNER/REPO[/PATH][@REF]; \
         docs/templates.md shows how to write one."
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
    run_build(&manifest, true)
}

/// The build command first, so a Dockerfile can pick up what it made.
/// `rootfs_hint`: say when the rootfs outgrew the guest's memory; a run
/// that follows says so itself, with the size the guest gets.
fn run_build(manifest: &Manifest, rootfs_hint: bool) -> CliResult<()> {
    if !manifest.needs_build() {
        eprintln!("nothing to build: hluk.toml has no [build] command and no [rootfs] dockerfile");
        return Ok(());
    }
    if let Some(build) = &manifest.build {
        // The toolchain is the project's, not hluk's: say which one is
        // missing rather than let the shell's "not found" stand alone.
        if let Some(program) = command_program(&build.command)
            && !on_path(program, &manifest.dir)
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
        // The rootfs is built here and run elsewhere, so no limit of this
        // container's comes into it.
        if rootfs_hint {
            let (scratch_mb, by) = match manifest.run.scratch_mb {
                Some(ScratchArg::Mib(mb)) => (mb, SizedBy::Setting),
                Some(ScratchArg::Max) => (manifest.tested_scratch_mb(&path), SizedBy::MaxUnmet),
                None => (manifest.tested_scratch_mb(&path), SizedBy::Setting),
            };
            scratch_hint(size, scratch_mb, by);
        }
    }
    Ok(())
}

/// A guest's memory as asked for, by `--scratch-mb` or a manifest's
/// `scratch_mb`: MiB, or `max`, what the memory limit `hluk` runs under
/// leaves once `hluk` has its own share, so that a pod which runs `hluk
/// run` in its image names the size once, as its limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScratchArg {
    Mib(usize),
    Max,
}

impl ScratchArg {
    /// `n` MiB as a size: more than nothing, and a byte count that fits.
    fn mib(n: u64) -> Option<Self> {
        scratch_mb_from(n).map(ScratchArg::Mib)
    }
}

fn invalid_scratch(what: impl std::fmt::Display) -> String {
    format!("invalid guest memory {what}: expected MiB, 1 or more, or max")
}

impl std::str::FromStr for ScratchArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("max") {
            return Ok(ScratchArg::Max);
        }
        s.parse::<u64>()
            .ok()
            .and_then(ScratchArg::mib)
            .ok_or_else(|| invalid_scratch(format!("{s:?}")))
    }
}

/// In a manifest, `scratch_mb = 512` or `scratch_mb = "max"`; anything
/// else, a zero, a negative number, a fraction or another word, is
/// refused with the one message.
impl<'de> serde::Deserialize<'de> for ScratchArg {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        struct Visitor;
        impl serde::de::Visitor<'_> for Visitor {
            type Value = ScratchArg;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("guest memory in MiB, or \"max\"")
            }

            fn visit_u64<E: Error>(self, n: u64) -> Result<ScratchArg, E> {
                ScratchArg::mib(n).ok_or_else(|| E::custom(invalid_scratch(n)))
            }

            fn visit_i64<E: Error>(self, n: i64) -> Result<ScratchArg, E> {
                u64::try_from(n)
                    .ok()
                    .and_then(ScratchArg::mib)
                    .ok_or_else(|| E::custom(invalid_scratch(n)))
            }

            fn visit_str<E: Error>(self, s: &str) -> Result<ScratchArg, E> {
                s.parse().map_err(E::custom)
            }
        }
        d.deserialize_any(Visitor)
    }
}

/// Where a guest's size was asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asked {
    /// `--scratch-mb`.
    Flag(ScratchArg),
    /// The manifest's `scratch_mb`.
    Manifest(ScratchArg),
}

/// How a guest got its size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizedBy {
    /// `scratch_mb`, set or the tested default, or a snapshot's.
    Setting,
    /// `max`: what the memory limit `hluk` runs under leaves.
    Max,
    /// `max` asked for with no limit to take it from: the tested default,
    /// for a limit to come (`hluk build`, or a manifest run elsewhere).
    MaxUnmet,
}

/// What [`guest_memory`] decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuestMemory {
    /// The guest's memory in MiB.
    pub mb: usize,
    /// How it was sized.
    pub how: SizedBy,
    /// The memory limit `hluk` runs under in MiB, when one is in sight.
    pub limit: Option<usize>,
}

impl GuestMemory {
    /// A guest of `mb` MiB that is not up for sizing (a snapshot's), under
    /// `limit`, for the hints.
    pub fn given(mb: usize, limit: Option<usize>) -> Self {
        GuestMemory {
            mb,
            how: SizedBy::Setting,
            limit,
        }
    }

    /// Whether the limit holds the guest: there is none, or the guest and
    /// `hluk`'s own share are within it.
    pub fn fits(&self) -> bool {
        self.limit
            .is_none_or(|limit| self.mb.saturating_add(CONTAINER_SCRATCH_HEADROOM_MB) <= limit)
    }

    /// Whether the limit holds the guest and a snapshot of it too: the
    /// save builds the snapshot in memory, about as much again as the
    /// guest has used.
    pub fn holds_a_save(&self) -> bool {
        self.limit.is_none_or(|limit| {
            self.mb
                .saturating_mul(2)
                .saturating_add(CONTAINER_SCRATCH_HEADROOM_MB)
                <= limit
        })
    }
}

/// The guest's memory: `asked` (`--scratch-mb`, else the manifest's), else
/// `tested`, the size the rootfs's runtime image is tested with (the
/// library's default for a kernel with no rootfs).  `max` is what `limit`,
/// the memory limit `hluk` runs under in MiB, leaves once `hluk` has its
/// own share; on the flag it is an error without a limit to take it from,
/// and in a manifest, which is written for a pod but runs elsewhere too,
/// it falls back to `tested` with a note, as `hluk build` does.  Every
/// other size stands, whatever the limit: the guest's memory is backed as
/// it is touched, so a guest sized past the limit runs until it uses more
/// than the limit leaves, and [`container_hint`] says so.  `tested` is
/// asked for only when it is needed, as it may scan the rootfs.
pub fn guest_memory(
    asked: Option<Asked>,
    tested: impl FnOnce() -> Option<usize>,
    limit: Option<usize>,
) -> CliResult<GuestMemory> {
    let sized = |mb, how| GuestMemory { mb, how, limit };
    let tested = move |how| Ok(sized(tested().unwrap_or(DEFAULT_SCRATCH_MB), how));
    match (asked, limit.and_then(scratch_mb_within)) {
        (None, _) => tested(SizedBy::Setting),
        (Some(Asked::Flag(ScratchArg::Mib(mb)) | Asked::Manifest(ScratchArg::Mib(mb))), _) => {
            Ok(sized(mb, SizedBy::Setting))
        }
        (Some(_), Some(room)) => {
            eprintln!(
                "note: the guest gets {room} MiB, what the memory limit hluk runs under, {} MiB, \
                 leaves it",
                limit.unwrap_or(0)
            );
            Ok(sized(room, SizedBy::Max))
        }
        (Some(asked), None) => {
            let why = match limit {
                Some(limit) => format!(
                    "the memory limit hluk runs under, {limit} MiB, leaves too little for a \
                     guest once hluk has its {CONTAINER_SCRATCH_HEADROOM_MB}"
                ),
                None => "hluk runs under no memory limit to take it from".to_string(),
            };
            match asked {
                Asked::Flag(_) => Err(format!("--scratch-mb max: {why}").into()),
                Asked::Manifest(_) => {
                    eprintln!(
                        "note: scratch_mb = \"max\" in the manifest: {why}; the guest gets the \
                         size its image is tested with"
                    );
                    tested(SizedBy::MaxUnmet)
                }
            }
        }
    }
}

/// Say when the guest will not fit: a guest given more than the memory
/// limit `hluk` runs under leaves, by `--scratch-mb`, a manifest, a
/// snapshot, or a tested size past the limit, is killed by the host's
/// kernel, not the guest's, once it uses that much.
pub fn container_hint(sizing: &GuestMemory) {
    if let (false, Some(limit)) = (sizing.fits(), sizing.limit) {
        eprintln!(
            "note: the guest gets {} MiB but hluk runs under a memory limit of {limit} MiB, of \
             which hluk needs about {CONTAINER_SCRATCH_HEADROOM_MB}; the guest will be killed once it uses more \
             than that leaves",
            sizing.mb
        );
    }
}

/// Say, before a snapshot is saved, when the limit holds the guest but
/// may not hold the save ([`GuestMemory::holds_a_save`]); a guest that
/// does not fit at all has had [`container_hint`]'s word.
pub fn save_memory_note(sizing: &GuestMemory) {
    if let (true, false, Some(limit)) = (sizing.fits(), sizing.holds_a_save(), sizing.limit) {
        eprintln!(
            "note: saving the guest's snapshot takes about as much memory again as the guest \
             has used, and the memory limit hluk runs under, {limit} MiB, may not hold a {} MiB \
             guest and its snapshot both",
            sizing.mb
        );
    }
}

/// The size the runtime image at `path` is tested with: by name for a
/// `--runtime` or a manifest's image (a registry's `agent`, or an image
/// reference ending in it), by the rootfs's runtime marker or driver
/// otherwise.
pub fn tested_scratch_mb(runtime: Option<&str>, path: &Path) -> usize {
    runtime
        .and_then(registry::runtime_name)
        .and_then(runtime_scratch_mb)
        .unwrap_or_else(|| default_scratch_mb(path))
}

/// The rootfs is unpacked into the guest's memory, so one that takes more
/// than half of it leaves the runtime little to work with: an image that
/// grew past the size its runtime is tested with (packages added by a
/// Dockerfile) needs `scratch_mb` set, or a bigger container.  Say so,
/// with a size that leaves room, rather than let the guest fail to unpack
/// it.
fn scratch_hint(rootfs_bytes: u64, scratch_mb: usize, by: SizedBy) {
    let rootfs_mb = rootfs_bytes.div_ceil(MIB as u64) as usize;
    if rootfs_mb * 2 <= scratch_mb {
        return;
    }
    let suggested = (rootfs_mb * 4).next_multiple_of(256);
    let container = suggested + CONTAINER_SCRATCH_HEADROOM_MB;
    match by {
        SizedBy::Setting => eprintln!(
            "note: the rootfs is {rootfs_mb} MiB and [run] scratch_mb = {scratch_mb}; the \
             rootfs is unpacked into that memory, so set scratch_mb to {suggested} or more"
        ),
        SizedBy::Max => eprintln!(
            "note: the rootfs is {rootfs_mb} MiB and max leaves the guest {scratch_mb} MiB \
             under the memory limit hluk runs under; the rootfs is unpacked into that memory, \
             so give the container {container} MiB or more, {suggested} for the guest and \
             {CONTAINER_SCRATCH_HEADROOM_MB} for hluk"
        ),
        SizedBy::MaxUnmet => eprintln!(
            "note: the rootfs is {rootfs_mb} MiB, more than half the {scratch_mb} MiB its \
             runtime is tested with, and max leaves the size to the memory limit hluk runs \
             under; the rootfs is unpacked into the guest's memory, so give the container \
             {container} MiB or more, {suggested} for the guest and \
             {CONTAINER_SCRATCH_HEADROOM_MB} for hluk"
        ),
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
/// x && make)` starts `cd`); `None` when it is the shell's own, or when
/// only the shell knows what it is: a word it expands, unquotes or splits
/// first (`${CC:-gcc}`, `"$GO"`, `~/bin/tool`, `cmd`'s `%CC%`, `./[b]uild.sh`).
fn command_program(command: &str) -> Option<&str> {
    let assignment = |word: &str| {
        word.split_once('=').is_some_and(|(name, _)| {
            name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
    };
    let program = command
        .split_whitespace()
        .map(|word| word.trim_start_matches(['(', '{']))
        .find(|word| !word.is_empty() && !assignment(word))?;
    // Only a plain word names the program as written; the shell expands,
    // unquotes or splits anything else first, so leave that to it.
    let plain = |c: char| {
        c.is_ascii_alphanumeric() || "._+/-".contains(c) || (cfg!(windows) && "\\:".contains(c))
    };
    let expanded = program.starts_with('-') || !program.chars().all(plain);
    (!expanded && !SHELL_BUILTINS.contains(&program)).then_some(program)
}

/// Whether `program` resolves: a path to a file, relative to `dir`, where
/// the command runs, or a name found in PATH (with the executable
/// extensions Windows adds).
fn on_path(program: &str, dir: &Path) -> bool {
    if program.contains('/') || program.contains('\\') {
        return dir.join(program).is_file();
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

/// The ELF machine (`e_machine`) of a program the guest can run, and its
/// name: the host's architecture.
const GUEST_ELF_ARCH: (u16, &str) = if cfg!(target_arch = "aarch64") {
    (183, "aarch64")
} else {
    (62, "x86_64")
};

/// A program the exec driver runs from a host mount must be a PIE: the
/// guest's ELF loader maps it into its own address space.  Read the header
/// here and say how to fix it, rather than let the guest report "Image
/// format not recognized", as it does for a binary for the host's OS or for
/// another architecture too.  Anything that is not an ELF file under a mount
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
    let mut header = [0u8; 20];
    if file.read_exact(&mut header).is_err() {
        return Ok(());
    }
    // A binary the host's own compiler made (Mach-O, PE) rather than a
    // Linux one: the usual slip on macOS and Windows.
    if matches!(
        &header[..4],
        [0xcf, 0xfa, 0xed, 0xfe] | [0xca, 0xfe, 0xba, 0xbe]
    ) || &header[..2] == b"MZ"
    {
        return Err(format!(
            "{} is a binary for the host OS, not a Linux ELF the guest can run: build it for \
             Linux on this CPU ({})",
            host.display(),
            GUEST_ELF_ARCH.1
        )
        .into());
    }
    if &header[..4] != b"\x7fELF" {
        return Ok(());
    }
    // e_machine at offset 18.
    let machine = u16::from_le_bytes([header[18], header[19]]);
    if machine != GUEST_ELF_ARCH.0 {
        return Err(format!(
            "{} is built for another architecture (ELF machine {machine}); this host runs {} \
             guests",
            host.display(),
            GUEST_ELF_ARCH.1
        )
        .into());
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
            run_build(&m, false)?;
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
    // What the guest may reach and the ports it may serve on are settled
    // apart: the command line's outbound flags replace the project's
    // policy, and its --port the project's ports.
    // The project's policy is read only if it is used: reading it resolves
    // its names, which an overriding run must not fail on.
    let policy_given = args.net || !args.net_allow.is_empty() || !args.net_block.is_empty();
    let (policy, manifest_listen) = match &manifest {
        Some(m) if !policy_given => m.network()?,
        Some(m) if args.ports.is_empty() => (None, m.listen_ports()?),
        _ => (None, None),
    };
    let policy = if policy_given {
        parse_net_policy(args.net, &args.net_allow, &args.net_block, &[])?.0
    } else {
        policy
    };
    let listen = if args.ports.is_empty() {
        manifest_listen
    } else {
        parse_net_policy(false, &[], &[], &args.ports)?.1
    };
    // The project's environment is part of its guest, warm snapshot
    // included; the command line's is this run's alone.
    let manifest_env: Vec<(String, String)> = manifest
        .as_ref()
        .map(|m| {
            m.run
                .env
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
        .unwrap_or_default();
    let mut env = manifest_env.clone();
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

    // Memory: the flag (`max` is what the container's memory limit
    // leaves), else the manifest's, else the size the runtime image is
    // tested with, by name for --runtime and a published image, by the
    // rootfs's runtime marker or driver otherwise, and the library's
    // default for a kernel with no rootfs, or what the memory limit leaves
    // for `max`.
    let limit = cgroup_memory_limit_mb();
    let sizing = guest_memory(
        args.scratch_mb.map(Asked::Flag).or_else(|| {
            manifest
                .as_ref()
                .and_then(|m| m.run.scratch_mb.map(Asked::Manifest))
        }),
        || {
            let path = rootfs.as_deref()?;
            Some(match &manifest {
                Some(m) => m.tested_scratch_mb(path),
                None => tested_scratch_mb(args.runtime.as_deref(), path),
            })
        },
        limit,
    )?;
    let scratch_mb = sizing.mb;
    container_hint(&sizing);
    if let Some(path) = &rootfs {
        let size = fs::metadata(path)
            .map_err(|e| format!("cannot read the rootfs {}: {e}", path.display()))?
            .len();
        scratch_hint(size, scratch_mb, sizing.how);
    }
    if let Exec::Code(command) = &exec {
        check_pie(command, &mounts)?;
    }
    if let Some(entry) = &entry {
        check_pie(entry, &mounts)?;
    }

    // A warm run restores the cached snapshot of exactly this guest, when
    // there is one, and saves one otherwise.
    // A runtime driver takes the environment per call, so a run's own can
    // reach it after the warm snapshot is saved.  A guest without one (an
    // entry-point program) reads it once, at boot, and so does a
    // warm_exec, which runs before the save: those boot with all of it,
    // and the snapshot is keyed by it.
    let driver = warm
        && match &entry {
            // The program, not its arguments, and one of the runtime
            // drivers by name: any other program may read its environment
            // only as it starts.
            Some(e) => e
                .split_whitespace()
                .next()
                .and_then(|program| program.rsplit('/').next())
                .is_some_and(is_runtime_driver),
            // Every published runtime image, named bare, has one: spare
            // the scan.  A full reference may name anyone's image.
            None if args.runtime.as_deref().is_some_and(|r| !r.contains('/')) => true,
            None => rootfs.as_deref().and_then(driver_entry).is_some(),
        };
    let env_after_save = driver && warm_exec.is_none();
    let boot_env = if env_after_save { &manifest_env } else { &env };
    let warm_key = match (warm, &rootfs) {
        (false, _) => None,
        (true, Some(path)) => Some(stamp(
            path,
            scratch_mb,
            entry.as_deref(),
            warm_exec.as_deref(),
            boot_env,
        )?),
        (true, None) => {
            return Err("--warm needs a rootfs: a project, --runtime or --initrd".into());
        }
    };
    let mut restored = false;
    // A cached snapshot of this guest that would not load, by when it was
    // written: the one this run saves replaces it, unless another run has
    // replaced it first.
    let mut unusable = None;
    let mut builder = None;
    if warm_key.is_some() {
        // Here, and not only when saving, so that a cache only ever
        // restored from loses what killed runs left too.
        rootfs::remove_abandoned_parts(&rootfs::snapshots_dir()?);
    }
    if let Some(key) = &warm_key
        && let Some(dir) = rootfs::find_snapshot(key)?
    {
        match SandboxBuilder::from_snapshot_dir(&dir) {
            Ok(b) => {
                builder = Some(b);
                restored = true;
            }
            Err(e) => {
                info!(error = %e, "cached warm snapshot cannot be used; booting fresh");
                unusable = rootfs::snapshot_written(&dir);
            }
        }
    }
    if warm && !restored {
        save_memory_note(&sizing);
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
            b = b.scratch_mb(scratch_mb);
            if let Some(e) = &entry {
                b = b.entry(e);
            }
            b
        }
    };
    builder = builder.mounts(mounts);
    if args.profile {
        builder = builder.profile(true);
    }
    if let Some(policy) = policy {
        builder = builder.network(policy);
    }
    if let Some(listen) = listen {
        builder = builder.listen_ports(listen);
    }
    // A guest booted to be saved as the warm snapshot is everyone's: what
    // it starts with (a Python's sys.path, from PYTHONPATH) is kept in
    // it.  This run's own variables reach a driver after the save instead.
    let saving_warm = warm_key.is_some() && !restored;
    for (key, value) in if saving_warm { boot_env } else { &env } {
        builder = builder.env(key, value);
    }
    if let Some(rc) = resolv {
        builder = builder.resolv_conf(rc);
    }

    // A terminal for the guest from boot, unless the boot ends in a warm
    // snapshot, whose runtime must not keep what this process's console
    // happens to be.  An entry-point program, with no driver, is the
    // workload, and runs during the boot: it has one either way, and one
    // that is still running when the snapshot is taken (a server) keeps
    // what it made of this run's console.
    builder = builder.console_terminal(!saving_warm || !driver);
    let t = Instant::now();
    let mut sandbox = boot(builder)?;
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
            crate::kernel_tail::workload(sandbox.run(code.as_str()))?;
            info!(elapsed_ms = elapsed_ms(t), "warm_exec done");
        }
        // Written beside its slot under this process's own name and moved
        // in whole: concurrent first runs of one guest each write their
        // own, and a run that dies half way leaves no snapshot that looks
        // complete.
        let slot = rootfs::snapshot_slot(&key)?;
        let part = slot.with_extension(format!("{}.part", std::process::id()));
        let _ = fs::remove_dir_all(&part);
        if let Some(parent) = slot.parent() {
            fs::create_dir_all(parent)?;
        }
        match sandbox.snapshot_to(&part) {
            Ok(_) => {
                fs::write(part.join(rootfs::STAMP_FILE), &key)?;
                rootfs::install_snapshot(&part, &slot, &key, unusable)?;
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
    if saving_warm && env_after_save {
        let vars: Vec<(&str, &str)> = env.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        sandbox.set_env_vars(&vars);
    }

    drive(
        &mut sandbox,
        no_workload,
        exec,
        Call::new(args.call.clone(), args.input.clone()),
    )
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
/// the code run before the snapshot, and the environment the guest boots
/// with: the project's `[run] env`, or all of it for a driverless image or
/// a run with `warm_exec`.  Mounts, the network policy and the rest of the
/// environment are supplied on restore, so they can change without one.
/// The text names the cache slot and is stored in it.
fn stamp(
    rootfs: &Path,
    scratch_mb: usize,
    entry: Option<&str>,
    warm_exec: Option<&str>,
    env: &[(String, String)],
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
    let mut stamp = format!(
        "key={SNAPSHOT_KEY}\nrootfs={}\nsize={}\nmtime={mtime}\nscratch_mb={scratch_mb}\nentry={}\nwarm_exec={:?}\n",
        canonical.display(),
        meta.len(),
        entry.unwrap_or(""),
        // Debug-quoted, as env below: its lines are code, not stamp lines.
        warm_exec.unwrap_or("")
    );
    // Debug-quoted: a value may hold a newline.  A guest that boots with no
    // environment adds no lines.
    for (k, v) in env {
        stamp.push_str(&format!("env={k:?}={v:?}\n"));
    }
    Ok(stamp)
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
    fn a_scratch_value_is_mib_or_max() {
        assert_eq!("512".parse(), Ok(ScratchArg::Mib(512)));
        assert_eq!(" max ".parse(), Ok(ScratchArg::Max));
        assert_eq!("MAX".parse(), Ok(ScratchArg::Max));
        assert!("lots".parse::<ScratchArg>().is_err());
        #[derive(serde::Deserialize)]
        struct Run {
            scratch_mb: Option<ScratchArg>,
        }
        let run = |s: &str| toml::from_str::<Run>(s).map(|r| r.scratch_mb);
        assert_eq!(run("scratch_mb = 512").unwrap(), Some(ScratchArg::Mib(512)));
        assert_eq!(run("scratch_mb = \"max\"").unwrap(), Some(ScratchArg::Max));
        assert_eq!(run("").unwrap(), None);
        assert!(run("scratch_mb = \"lots\"").is_err());
        assert!(run("scratch_mb = 1.5").is_err());
        assert!(run("scratch_mb = 0").is_err());
        assert!(run("scratch_mb = -1").is_err());
        assert!(run("scratch_mb = true").is_err());
        assert!(
            run("scratch_mb = 18446744073709551615").is_err(),
            "a size whose bytes do not fit"
        );
        assert!("0".parse::<ScratchArg>().is_err());
        assert!("-1".parse::<ScratchArg>().is_err());
        assert!("18446744073709551615".parse::<ScratchArg>().is_err());
        assert_eq!(
            "lots".parse::<ScratchArg>().unwrap_err(),
            "invalid guest memory \"lots\": expected MiB, 1 or more, or max"
        );
    }

    #[test]
    fn guest_memory_is_asked_else_tested_and_max_is_the_limits() {
        use SizedBy::{Max, MaxUnmet, Setting};
        let tested = || Some(256);
        let got = |sizing: CliResult<GuestMemory>| sizing.map(|s| (s.mb, s.how)).unwrap();
        // The tested size, the library's default without one, whatever the limit.
        assert_eq!(got(guest_memory(None, tested, None)), (256, Setting));
        assert_eq!(
            got(guest_memory(None, || None, None)),
            (DEFAULT_SCRATCH_MB, Setting)
        );
        assert_eq!(got(guest_memory(None, tested, Some(1024))), (256, Setting));
        assert_eq!(
            got(guest_memory(None, tested, Some(200))),
            (256, Setting),
            "a limit never resizes a guest; the hint speaks instead"
        );
        assert!(!guest_memory(None, tested, Some(200)).unwrap().fits());
        // A size asked for stands, from the flag or the manifest.
        let flag = Some(Asked::Flag(ScratchArg::Mib(1024)));
        assert_eq!(got(guest_memory(flag, tested, None)), (1024, Setting));
        assert_eq!(got(guest_memory(flag, tested, Some(200))), (1024, Setting));
        assert_eq!(
            got(guest_memory(flag, || panic!("not needed"), Some(1024))),
            (1024, Setting),
            "the rootfs is not scanned for a size that was asked for"
        );
        let manifest = Some(Asked::Manifest(ScratchArg::Mib(128)));
        assert_eq!(
            got(guest_memory(manifest, tested, Some(1024))),
            (128, Setting)
        );
        // `max` is what the limit leaves, and nothing without one.
        let max = Some(Asked::Flag(ScratchArg::Max));
        assert_eq!(got(guest_memory(max, tested, Some(1024))), (960, Max));
        assert_eq!(got(guest_memory(max, tested, Some(128))), (64, Max));
        assert!(guest_memory(max, tested, Some(100)).is_err());
        assert!(guest_memory(max, tested, None).is_err());
        // A manifest's `max` falls back to the tested size where there is no limit to take.
        let max = Some(Asked::Manifest(ScratchArg::Max));
        assert_eq!(got(guest_memory(max, tested, Some(1024))), (960, Max));
        assert_eq!(got(guest_memory(max, tested, None)), (256, MaxUnmet));
        assert_eq!(got(guest_memory(max, tested, Some(100))), (256, MaxUnmet));
    }

    #[test]
    fn the_guest_fits_the_limit_or_the_hint_says_not() {
        let tested = || Some(256);
        let fits = |asked, limit| guest_memory(asked, tested, limit).unwrap().fits();
        let flag = |mb| Some(Asked::Flag(ScratchArg::Mib(mb)));
        assert!(fits(None, None), "no limit, no doubt");
        assert!(fits(flag(1024), None));
        assert!(fits(None, Some(1024)));
        assert!(!fits(None, Some(200)), "the tested size past the limit");
        assert!(fits(Some(Asked::Flag(ScratchArg::Max)), Some(1024)));
        assert!(!fits(flag(1024), Some(200)), "asked for more than there is");
        assert!(fits(flag(136), Some(200)), "asked for just what there is");
        assert!(!fits(flag(137), Some(200)));
        assert!(
            fits(flag(32), Some(100)),
            "a small guest fits a limit too small to size one by"
        );
        assert!(!GuestMemory::given(1536, Some(512)).fits());
        assert!(GuestMemory::given(1536, None).fits());
        assert!(
            !GuestMemory::given(usize::MAX, Some(512)).fits(),
            "an absurd size does not wrap into a fit"
        );
    }

    #[test]
    fn the_limit_holds_a_save_or_the_note_says_not() {
        let holds = |mb, limit| GuestMemory::given(mb, limit).holds_a_save();
        assert!(holds(1472, None));
        assert!(
            holds(200, Some(512)),
            "twice the guest and hluk's share fit"
        );
        assert!(holds(224, Some(512)));
        assert!(!holds(225, Some(512)));
        assert!(!holds(448, Some(512)), "what max gives never leaves room");
        assert!(!holds(usize::MAX / 2 + 1, Some(512)));
    }

    #[test]
    fn the_suggested_name_is_a_valid_one() {
        for (template, want) in [
            ("python", "hello-python"),
            ("my.template", "hello-my-template"),
            ("My Template!", "hello-My-Template"),
        ] {
            let name = suggested_name(template);
            assert_eq!(name, want);
            validate_name(&name).unwrap();
        }
    }

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
        // Only the shell knows what these run.
        assert_eq!(command_program("${CC:-gcc} -o bin/app main.c"), None);
        assert_eq!(command_program("CC=clang \"$CC\" main.c"), None);
        assert_eq!(command_program("~/bin/build"), None);
        assert_eq!(command_program("${CC:=gcc} main.c"), None);
        assert_eq!(command_program("%CC% main.c"), None);
        assert_eq!(command_program("./build.sh --release"), Some("./build.sh"));
        assert_eq!(command_program("(make)"), None);
        assert_eq!(command_program("make&&cp out bin/"), None);
        assert_eq!(command_program("./[b]uild.sh"), None);
        assert_eq!(command_program("CC=$(command -v gcc) $CC main.c"), None);
        assert_eq!(command_program("2=foo make"), None);
        if !cfg!(windows) {
            assert_eq!(command_program("\\make -j4"), None);
            assert_eq!(command_program("./build\\ tool.sh"), None);
        }
        let here = Path::new(".");
        assert!(on_path("sh", here) || on_path("cmd", here));
        assert!(!on_path("hluk-no-such-program-xyz", here));
    }

    #[test]
    fn a_build_script_is_found_where_the_command_runs() {
        let project = tempfile::tempdir().unwrap();
        fs::create_dir(project.path().join("scripts")).unwrap();
        fs::write(project.path().join("scripts").join("build.sh"), "").unwrap();
        let elsewhere = tempfile::tempdir().unwrap();
        assert!(on_path("./scripts/build.sh", project.path()));
        assert!(on_path("scripts/build.sh", project.path()));
        assert!(!on_path("./scripts/build.sh", elsewhere.path()));
        let absolute = project.path().join("scripts").join("build.sh");
        assert!(on_path(absolute.to_str().unwrap(), elsewhere.path()));
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
        let a = stamp(&rootfs, 256, None, None, &[]).unwrap();
        assert_eq!(a, stamp(&rootfs, 256, None, None, &[]).unwrap());
        assert_ne!(a, stamp(&rootfs, 512, None, None, &[]).unwrap());
        assert_ne!(a, stamp(&rootfs, 256, Some("/bin/app"), None, &[]).unwrap());
        assert_ne!(
            a,
            stamp(&rootfs, 256, None, Some("import flask"), &[]).unwrap()
        );
        let env = [("PYTHONPATH".to_string(), "/app".to_string())];
        assert_ne!(a, stamp(&rootfs, 256, None, None, &env).unwrap());
        fs::write(&rootfs, b"0707010").unwrap();
        assert_ne!(a, stamp(&rootfs, 256, None, None, &[]).unwrap());
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
        exec[18..20].copy_from_slice(&GUEST_ELF_ARCH.0.to_le_bytes());
        fs::write(dir.path().join("fixed"), &exec).unwrap();
        let mut pie = exec.clone();
        pie[16..18].copy_from_slice(&3u16.to_le_bytes());
        fs::write(dir.path().join("pie"), &pie).unwrap();
        // A PIE for the other architecture, and a macOS binary.
        let mut foreign = pie.clone();
        foreign[18..20]
            .copy_from_slice(&(if GUEST_ELF_ARCH.0 == 62 { 183u16 } else { 62 }).to_le_bytes());
        fs::write(dir.path().join("foreign"), &foreign).unwrap();
        let mut macho = vec![0u8; 64];
        macho[..4].copy_from_slice(&[0xcf, 0xfa, 0xed, 0xfe]);
        fs::write(dir.path().join("macho"), &macho).unwrap();
        fs::write(dir.path().join("script"), b"print('hi')").unwrap();
        let mounts = [Mount::ro(dir.path(), "/mnt/app")];
        let err = check_pie("/mnt/app/fixed --flag", &mounts)
            .unwrap_err()
            .to_string();
        assert!(err.contains("not position-independent"), "{err}");
        let err = check_pie("/mnt/app/foreign", &mounts)
            .unwrap_err()
            .to_string();
        assert!(err.contains("another architecture"), "{err}");
        let err = check_pie("/mnt/app/macho", &mounts)
            .unwrap_err()
            .to_string();
        assert!(err.contains("not a Linux ELF"), "{err}");
        check_pie("/mnt/app/pie", &mounts).unwrap();
        check_pie("/mnt/app/script", &mounts).unwrap();
        check_pie("/mnt/app/missing", &mounts).unwrap();
        check_pie("print(1)", &mounts).unwrap();
    }
}
