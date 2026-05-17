//! `pyhl` as a library — drive the python-agent-driver image from Rust
//! without going through the CLI.
//!
//! Two pieces:
//!
//! - [`install`] — one-time: take a source image (or a GHCR pull) and
//!   materialize `kernel`, `initrd.cpio`, and a warmed-up `snapshot.hls`
//!   in the image home.
//!
//! - [`Runtime`] — the steady-state object: holds an open
//!   [`crate::Sandbox`] loaded from the persisted snapshot, and
//!   exposes [`run_code`](Runtime::run_code) /
//!   [`run_script`](Runtime::run_script) for every subsequent invocation.
//!   Mounts can be supplied per runtime (one `Runtime`, many `run_*`
//!   calls against it — each hermetic via restore).
//!
//! Typical use:
//!
//! ```no_run
//! use hyperlight_unikraft::{pyhl, Preopen};
//! use std::path::Path;
//!
//! # fn main() -> anyhow::Result<()> {
//! let home = Path::new(".pyhl");
//! // One-time install (no-op if already present).
//! pyhl::install(&pyhl::InstallOptions {
//!     home,
//!     source: pyhl::InstallSource::Ghcr { tag: None }, // None = :latest
//!     mounts: &[],
//!     network: None,
//!     listen_ports: None,
//!     force: false,
//! })?;
//!
//! let mut rt = pyhl::Runtime::new(home, &[Preopen::new("./share", "/host")?], None, None)?;
//! rt.run_code("print('hello from rust')")?;
//! rt.run_code("print('hermetic second call')")?;  // fresh __main__ each time
//! # Ok(())
//! # }
//! ```
//!
//! The binary in `src/bin/pyhl.rs` is a thin wrapper over this API.

use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::{Preopen, Sandbox};

/// Set `HYPERLIGHT_INITIAL_SURROGATES=1` if unset (Windows only).
///
/// On Windows, Hyperlight pre-spawns surrogate processes (one per
/// potential sandbox). The default count is 512, each costing ~7 ms
/// of `CreateProcessA` latency. For pyhl's single-sandbox usage,
/// pinning to 1 avoids ~3.5 s of wasted startup.
///
/// # Safety
///
/// Must be called from the main thread before spawning any additional
/// threads or creating any `Sandbox`. Violating this is undefined
/// behavior due to `std::env::set_var` not being thread-safe.
pub fn default_surrogate_count() {
    #[cfg(target_os = "windows")]
    {
        if std::env::var_os("HYPERLIGHT_INITIAL_SURROGATES").is_none() {
            // SAFETY: Called from main thread before any sandbox or
            // thread pool is created. The binary entry points (cmd_run,
            // cmd_setup) call this first.
            unsafe {
                std::env::set_var("HYPERLIGHT_INITIAL_SURROGATES", "1");
            }
        }
    }
}

/// Standard file names inside an image home.
pub const KERNEL_FILE: &str = "kernel";
/// Standard file names inside an image home.
pub const INITRD_FILE: &str = "initrd.cpio";
/// Standard file names inside an image home.
pub const SNAPSHOT_FILE: &str = "snapshot.hls";
/// Standard file names inside an image home.
pub const VERSION_FILE: &str = "VERSION";

/// Configuration for [`install`].
pub struct InstallOptions<'a> {
    /// Target directory. Files are written at
    /// `{home}/{kernel,initrd.cpio,snapshot.hls,VERSION}`.
    pub home: &'a Path,

    /// Where to get the image from.
    pub source: InstallSource<'a>,

    /// Host → guest directory preopens. These are baked into the
    /// persisted snapshot (the guest mounts hostfs at `guest_path`
    /// during warmup). `Runtime::new` only remaps the host side.
    pub mounts: &'a [Preopen],

    /// Network policy. `None` disables networking; `Some(policy)`
    /// enables `net_*` tools with the given restrictions.
    pub network: Option<&'a crate::NetworkPolicy>,

    /// Ports the guest may bind to for inbound connections.
    /// `None` means outbound-only (when networking is enabled).
    pub listen_ports: Option<&'a crate::ListenPorts>,

    /// Overwrite an existing install.
    pub force: bool,
}

/// Where `install` pulls its kernel and CPIO from.
#[derive(Debug)]
pub enum InstallSource<'a> {
    /// Pull the published image from GHCR via docker/podman.
    /// `tag`: `None` pulls `:latest`; `Some("v0.3.1")` pins a release.
    Ghcr { tag: Option<&'a str> },
    /// Copy from a local python-agent-driver build tree.
    LocalDir(&'a Path),
    /// Explicit files — useful for custom image pipelines.
    Explicit { kernel: &'a Path, initrd: &'a Path },
}

/// Summary of an [`install`] run. Absolute paths to the installed files.
#[derive(Debug)]
pub struct InstallReport {
    pub home: PathBuf,
    pub kernel: PathBuf,
    pub initrd: PathBuf,
    pub snapshot: PathBuf,
    /// True if the install was a no-op (image already present and `force == false`).
    pub already_installed: bool,
    /// Wall-time of the warmup + snapshot persist step (0 when already installed).
    pub warmup_ms: f64,
}

/// Install — copy kernel + CPIO into `home`, warm up a sandbox, and
/// persist a post-warmup snapshot. Idempotent when `force == false`.
///
/// Only the `InstallSource::Ghcr` path touches the network; the others
/// are local file copies. See [`InstallSource`] for each variant's
/// semantics.
pub fn install(opts: &InstallOptions<'_>) -> Result<InstallReport> {
    default_surrogate_count();
    let home = opts.home.to_path_buf();
    let dst_kernel = home.join(KERNEL_FILE);
    let dst_initrd = home.join(INITRD_FILE);
    let dst_snapshot = home.join(SNAPSHOT_FILE);
    let dst_version = home.join(VERSION_FILE);

    let already = dst_kernel.is_file() && dst_initrd.is_file() && dst_snapshot.is_file();
    if already && !opts.force {
        return Ok(InstallReport {
            home,
            kernel: dst_kernel,
            initrd: dst_initrd,
            snapshot: dst_snapshot,
            already_installed: true,
            warmup_ms: 0.0,
        });
    }

    fs::create_dir_all(&home).with_context(|| format!("create image home {}", home.display()))?;

    let (src_label, src_kernel, src_initrd) = match &opts.source {
        InstallSource::LocalDir(dir) => {
            let (k, i) = discover_source_artifacts(dir)
                .with_context(|| format!("scan {} for image artifacts", dir.display()))?;
            (format!("local:{}", dir.display()), k, i)
        }
        InstallSource::Explicit { kernel, initrd } => (
            "explicit".to_string(),
            kernel.to_path_buf(),
            initrd.to_path_buf(),
        ),
        InstallSource::Ghcr { tag } => {
            let kernel_image = ghcr_image_ref(GHCR_KERNEL_REPO, *tag);
            let initrd_image = ghcr_image_ref(GHCR_INITRD_REPO, *tag);
            let scratch = home.join(".pyhl.download");
            fs::create_dir_all(&scratch)?;
            let k = scratch.join("kernel");
            let i = scratch.join("initrd.cpio");
            extract_from_ghcr(&kernel_image, "/kernel", &k)?;
            extract_from_ghcr(&initrd_image, "/initrd.cpio", &i)?;
            (format!("ghcr: {kernel_image} + {initrd_image}"), k, i)
        }
    };

    copy_replace(&src_kernel, &dst_kernel)
        .with_context(|| format!("install {}", dst_kernel.display()))?;
    copy_replace(&src_initrd, &dst_initrd)
        .with_context(|| format!("install {}", dst_initrd.display()))?;

    // Warmup + persist.
    let t = Instant::now();
    {
        let mut builder = Sandbox::builder(&dst_kernel)
            .initrd_file(&dst_initrd)
            .heap_size(3 * 512 * 1024 * 1024);
        for p in opts.mounts {
            builder = builder.preopen(p.clone());
        }
        if let Some(policy) = opts.network {
            builder = builder.network(policy.clone());
        }
        if let Some(lp) = opts.listen_ports {
            builder = builder.listen_ports(lp.clone());
        }
        let mut sbox = builder.build()?;
        sbox.restore()?;
        let _: () = sbox.call_named("run", "pass".to_string())?;
        sbox.snapshot_now()?;
        sbox.save_snapshot(&dst_snapshot)?;
    }
    let warmup_ms = t.elapsed().as_secs_f64() * 1000.0;

    let version = format!(
        "pyhl {pyhl_ver}\nsource: {src}\nkernel: {kern}\ninitrd: {initrd}\nsnapshot: {snap}\n",
        pyhl_ver = env!("CARGO_PKG_VERSION"),
        src = src_label,
        kern = src_kernel.display(),
        initrd = src_initrd.display(),
        snap = dst_snapshot.display(),
    );
    fs::write(&dst_version, version)?;

    Ok(InstallReport {
        home,
        kernel: dst_kernel,
        initrd: dst_initrd,
        snapshot: dst_snapshot,
        already_installed: false,
        warmup_ms,
    })
}

/// Per-call timing for `Runtime::run_*` if you care.
#[derive(Debug, Clone, Copy, Default)]
pub struct RunTiming {
    pub restore_ms: f64,
    pub call_ms: f64,
    pub exit_code: i32,
}

/// A pyhl runtime backed by a warmed-up snapshot. Cheap to keep around;
/// call `run_*` many times against the same instance to amortise the
/// load cost over many invocations.
pub struct Runtime {
    sandbox: Sandbox,
    /// True until the first run, when restore is still a no-op (the
    /// sandbox is already at the loaded-snapshot state).
    first_run: bool,
}

impl Runtime {
    /// Open a runtime against an existing install. Looks for
    /// `{home}/snapshot.hls` and mmap-loads it. `mounts` specify host
    /// directories to expose under the guest paths that were baked in
    /// at `install` time. `network` enables guest networking with the
    /// given policy (`None` = disabled). `listen_ports` controls which
    /// ports the guest may bind to (`None` = outbound-only).
    pub fn new(
        home: &Path,
        mounts: &[Preopen],
        network: Option<&crate::NetworkPolicy>,
        listen_ports: Option<&crate::ListenPorts>,
    ) -> Result<Self> {
        default_surrogate_count();
        let snap = home.join(SNAPSHOT_FILE);
        if !snap.is_file() {
            bail!(
                "no snapshot at {} — run pyhl::install first",
                snap.display()
            );
        }
        let initrd = home.join(INITRD_FILE);
        let initrd_ref = if initrd.is_file() {
            Some(initrd.as_path())
        } else {
            None
        };
        let sandbox = Sandbox::from_snapshot_file_configured(
            &snap,
            mounts,
            initrd_ref,
            network,
            listen_ports,
        )?;
        Ok(Self {
            sandbox,
            first_run: true,
        })
    }

    /// Execute a string of Python code. The call is hermetic: the
    /// guest's Python `__main__` dict is reset between calls by
    /// restoring the snapshot state.
    pub fn run_code(&mut self, code: &str) -> Result<RunTiming> {
        let mut t = RunTiming::default();
        if !self.first_run {
            let tr = Instant::now();
            self.sandbox.restore()?;
            t.restore_ms = tr.elapsed().as_secs_f64() * 1000.0;
        }
        self.first_run = false;
        self.sandbox.reset_exit_code();

        let tc = Instant::now();
        let _: () = self.sandbox.call_named("run", code.to_string())?;
        t.call_ms = tc.elapsed().as_secs_f64() * 1000.0;
        t.exit_code = self.sandbox.last_exit_code();
        Ok(t)
    }

    /// Convenience: read a file and run its contents.
    pub fn run_script(&mut self, path: &Path) -> Result<RunTiming> {
        let code =
            fs::read_to_string(path).with_context(|| format!("read script {}", path.display()))?;
        self.run_code(&code)
    }

    /// Force a restore before the next call (useful if the previous
    /// call was skipped or the caller wants a deterministic rewind
    /// point).
    pub fn reset(&mut self) -> Result<()> {
        self.sandbox.restore()?;
        self.first_run = false;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers shared with the `pyhl` binary. Marked `pub` (inside the `pyhl`
// module) so the bin crate can reuse them instead of keeping a parallel
// copy in sync.
// ---------------------------------------------------------------------------

/// Atomically copy `src` → `dst`: stage to `dst.pyhl.tmp`, then rename
/// into place so a failure doesn't leave a half-written file.
pub fn copy_replace(src: &Path, dst: &Path) -> Result<()> {
    let staging = dst.with_extension("pyhl.tmp");
    let _ = fs::remove_file(&staging);
    fs::copy(src, &staging)?;
    fs::rename(&staging, dst)?;
    Ok(())
}

/// Locate a `(kernel, initrd.cpio)` pair inside a python-agent-driver
/// build tree: `{dir}/.unikraft/build/*_hyperlight-x86_64` for the kernel
/// and `{dir}/*-initrd.cpio` for the rootfs. Used by `install` and by
/// the `pyhl setup --from` CLI path.
pub fn discover_source_artifacts(dir: &Path) -> Result<(PathBuf, PathBuf)> {
    if !dir.is_dir() {
        bail!("{} is not a directory", dir.display());
    }
    let build = dir.join(".unikraft/build");
    let kernel = if build.is_dir() {
        fs::read_dir(&build)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.ends_with("_hyperlight-x86_64") && !n.ends_with(".dbg"))
                    .unwrap_or(false)
            })
    } else {
        None
    }
    .ok_or_else(|| {
        anyhow!(
            "no built kernel under {} — run `just build` (or \
             kraft-hyperlight build --plat hyperlight --arch x86_64) \
             in {} first",
            build.display(),
            dir.display()
        )
    })?;

    let initrd = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("-initrd.cpio"))
                .unwrap_or(false)
        })
        .ok_or_else(|| {
            anyhow!(
                "no *-initrd.cpio in {} — run `just rootfs` there first",
                dir.display()
            )
        })?;

    Ok((kernel, initrd))
}

// ---------------------------------------------------------------------------
// GHCR image pull — shared by `pyhl setup` and any library caller going
// through InstallSource::Ghcr.
// ---------------------------------------------------------------------------

/// GHCR repository base (without tag) for the kernel image.
pub const GHCR_KERNEL_REPO: &str =
    "ghcr.io/hyperlight-dev/hyperlight-unikraft/python-agent-driver-kernel";
/// GHCR repository base (without tag) for the initrd image.
pub const GHCR_INITRD_REPO: &str =
    "ghcr.io/hyperlight-dev/hyperlight-unikraft/python-agent-driver-initrd";

/// Build a full OCI image reference from a repo base and an optional tag.
/// `None` resolves to `:latest`.
pub fn ghcr_image_ref(repo: &str, tag: Option<&str>) -> String {
    format!("{}:{}", repo, tag.unwrap_or("latest"))
}

/// Pull a single file out of an OCI image hosted on GHCR. Tries
/// `docker` / `podman` first; if neither is on `$PATH`, falls back to
/// the OCI registry HTTP API (no container runtime needed).
pub fn extract_from_ghcr(image: &str, src_path_in_image: &str, dst: &Path) -> Result<()> {
    match find_on_path(&["docker", "podman"]) {
        Some(tool) => extract_via_docker(tool, image, src_path_in_image, dst),
        None => {
            eprintln!("pyhl:   docker/podman not found, pulling via OCI registry API");
            extract_via_oci_api(image, src_path_in_image, dst)
        }
    }
}

fn extract_via_docker(tool: &str, image: &str, src_path_in_image: &str, dst: &Path) -> Result<()> {
    use std::process::Command;

    let run = |cmd: &mut Command, label: &str| -> Result<std::process::Output> {
        let out = cmd
            .output()
            .with_context(|| format!("spawn {tool} {label}"))?;
        if !out.status.success() {
            bail!(
                "{tool} {label} failed (exit {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        Ok(out)
    };

    eprintln!("pyhl:   pull {image}");
    run(Command::new(tool).args(["pull", image]), "pull")?;

    let cname = format!("pyhl-extract-{}", std::process::id());
    let _ = Command::new(tool).args(["rm", "-f", &cname]).output();

    let _ = run(
        Command::new(tool).args(["create", "--name", &cname, image, "/bogus-entry"]),
        "create",
    )?;

    let _cleanup = GhcrCleanup {
        tool: tool.to_string(),
        cname: cname.clone(),
    };

    run(
        Command::new(tool).args([
            "cp",
            &format!("{cname}:{src_path_in_image}"),
            dst.to_str()
                .ok_or_else(|| anyhow!("dst is not valid UTF-8"))?,
        ]),
        "cp",
    )?;

    Ok(())
}

/// Pull a single file from a GHCR OCI image using the registry HTTP API.
/// No container runtime required.
fn extract_via_oci_api(image: &str, src_path_in_image: &str, dst: &Path) -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let (repo, tag) = parse_image_ref(image)?;

    eprintln!("pyhl:   pull {image} (via OCI API)");

    let token_url = format!("https://ghcr.io/token?scope=repository:{repo}:pull");
    let token_body: serde_json::Value = serde_json::from_slice(
        &ureq::get(&token_url)
            .call()
            .with_context(|| format!("fetch GHCR auth token for {repo}"))?
            .into_body()
            .read_to_vec()?,
    )?;
    let token = token_body["token"]
        .as_str()
        .ok_or_else(|| anyhow!("GHCR token response missing 'token' field"))?;

    let manifest_url = format!("https://ghcr.io/v2/{repo}/manifests/{tag}");
    let manifest: serde_json::Value = serde_json::from_slice(
        &ureq::get(&manifest_url)
            .header("Authorization", &format!("Bearer {token}"))
            .header(
                "Accept",
                "application/vnd.oci.image.manifest.v1+json,\
                 application/vnd.docker.distribution.manifest.v2+json",
            )
            .call()
            .with_context(|| format!("fetch manifest for {repo}:{tag}"))?
            .into_body()
            .read_to_vec()?,
    )?;

    let digest = manifest["layers"]
        .as_array()
        .and_then(|layers| layers.first())
        .and_then(|l| l["digest"].as_str())
        .ok_or_else(|| anyhow!("manifest has no layers for {repo}:{tag}"))?;

    let blob_url = format!("https://ghcr.io/v2/{repo}/blobs/{digest}");
    let body = ureq::get(&blob_url)
        .header("Authorization", &format!("Bearer {token}"))
        .call()
        .with_context(|| format!("fetch blob {digest}"))?
        .into_body()
        .with_config()
        .limit(512 * 1024 * 1024)
        .read_to_vec()?;

    let target_name = src_path_in_image.trim_start_matches('/');
    let gz = GzDecoder::new(body.as_slice());
    let mut archive = Archive::new(gz);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path
            .to_str()
            .is_some_and(|p| p.trim_start_matches("./") == target_name)
        {
            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = std::fs::File::create(dst)?;
            std::io::copy(&mut entry, &mut out)?;
            return Ok(());
        }
    }

    bail!("file '{target_name}' not found in OCI layer for {repo}:{tag}")
}

/// Parse "ghcr.io/org/repo:tag" into ("org/repo", "tag").
fn parse_image_ref(image: &str) -> Result<(&str, &str)> {
    let without_registry = image
        .strip_prefix("ghcr.io/")
        .ok_or_else(|| anyhow!("expected ghcr.io/ prefix in image ref: {image}"))?;
    match without_registry.rsplit_once(':') {
        Some((repo, tag)) => Ok((repo, tag)),
        None => Ok((without_registry, "latest")),
    }
}

struct GhcrCleanup {
    tool: String,
    cname: String,
}

impl Drop for GhcrCleanup {
    fn drop(&mut self) {
        let _ = std::process::Command::new(&self.tool)
            .args(["rm", "-f", &self.cname])
            .output();
    }
}

/// Return the first name in `names` present as an executable on `$PATH`.
/// Avoids a `which` crate dep — we only need a boolean existence check.
pub fn find_on_path(names: &[&'static str]) -> Option<&'static str> {
    let path = std::env::var_os("PATH")?;
    for name in names {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(name);
            if let Ok(md) = candidate.metadata() {
                if md.is_file() {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if md.permissions().mode() & 0o111 == 0 {
                            continue;
                        }
                    }
                    return Some(name);
                }
            }
        }
    }
    None
}
