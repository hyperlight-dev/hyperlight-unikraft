// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Where a project's rootfs comes from: the local image cache a published
//! rootfs is pulled into, and the Docker build that turns a project's
//! Dockerfile into a CPIO.

use std::fs;
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use super::cpio;
use super::manifest::Manifest;
use super::registry::{Client, Reference, extract_initrd};

/// Where pulled rootfs images live: `$HLUK_CACHE_DIR`, else the platform's
/// cache directory (`$XDG_CACHE_HOME/hluk` or `~/.cache/hluk` on Unix,
/// `%LOCALAPPDATA%\hluk\cache` on Windows).  Shared by every project, so a
/// runtime is pulled once per machine.
pub fn cache_dir() -> Result<PathBuf, String> {
    if let Some(dir) = std::env::var_os("HLUK_CACHE_DIR") {
        return Ok(PathBuf::from(dir));
    }
    if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA")
            .map(|d| PathBuf::from(d).join("hluk").join("cache"))
            .ok_or_else(|| {
                "cannot find a cache directory: set HLUK_CACHE_DIR or LOCALAPPDATA".into()
            })
    } else if let Some(dir) = std::env::var_os("XDG_CACHE_HOME").filter(|d| !d.is_empty()) {
        Ok(PathBuf::from(dir).join("hluk"))
    } else {
        std::env::var_os("HOME")
            .map(|h| PathBuf::from(h).join(".cache").join("hluk"))
            .ok_or_else(|| "cannot find a cache directory: set HLUK_CACHE_DIR or HOME".into())
    }
}

/// The cached CPIO for `reference`, whether or not it has been pulled.
pub fn cached_initrd(reference: &Reference) -> Result<PathBuf, String> {
    Ok(cache_dir()?.join("rootfs").join(reference.cache_relpath()))
}

/// How [`ensure_initrd`] found the rootfs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fetch {
    /// Already in the cache, and (with `refresh`) still what the tag names.
    Cached,
    /// Downloaded: the layer's compressed size.
    Pulled { bytes: u64 },
}

/// Make sure the CPIO for `image` is in the cache and return its path.
/// With `refresh`, the registry is asked again and a tag that moved is
/// re-downloaded; without it, a cached file is taken as is.
pub fn ensure_initrd(image: &str, refresh: bool) -> Result<(PathBuf, Fetch), String> {
    let reference = Reference::parse(image)?;
    let dest = cached_initrd(&reference)?;
    let digest_file = dest.with_extension("cpio.digest");
    let cached_digest = fs::read_to_string(&digest_file).ok();
    if dest.is_file() && cached_digest.is_some() && !refresh {
        return Ok((dest, Fetch::Cached));
    }

    let mut client = Client::new();
    let (_, layers) = client.manifest(&reference)?;
    let layer = match layers.as_slice() {
        [layer] => layer.clone(),
        [] => return Err(format!("{reference}: the image has no layers")),
        _ => {
            return Err(format!(
                "{reference}: the image has {} layers; a hluk `:initrd` image has one",
                layers.len()
            ));
        }
    };
    if dest.is_file() && cached_digest.as_deref() == Some(layer.digest.as_str()) {
        return Ok((dest, Fetch::Cached));
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    let mut progress = Progress::start(&reference);
    let pid = std::process::id();
    let layer_path = dest.with_extension(format!("layer.{pid}.part"));
    let cpio_path = dest.with_extension(format!("cpio.{pid}.part"));
    // Download, verify and unpack beside the destination; whatever is
    // left of a failed attempt goes with it.
    let fetched = (|| {
        let mut file = fs::File::create(&layer_path)
            .map_err(|e| format!("cannot write {}: {e}", layer_path.display()))?;
        client.download_layer(&reference, &layer, &mut file, &mut |done, total| {
            progress.update(done, total);
        })?;
        let blob = fs::File::open(&layer_path).map_err(|e| e.to_string())?;
        extract_initrd(&layer, blob, &cpio_path)
    })();
    let _ = fs::remove_file(&layer_path);
    if let Err(e) = fetched {
        let _ = fs::remove_file(&cpio_path);
        return Err(e);
    }
    fs::rename(&cpio_path, &dest)
        .map_err(|e| format!("cannot move into place {}: {e}", dest.display()))?;
    fs::write(&digest_file, &layer.digest).map_err(|e| e.to_string())?;
    progress.finish(layer.size);
    Ok((dest, Fetch::Pulled { bytes: layer.size }))
}

/// A one-line download indicator on stderr.  On a terminal the line is
/// redrawn in place, at most a few times a second and only for a transfer
/// worth watching; it is kept short, since a line wider than the terminal
/// wraps and `\r` then only returns to the start of its tail.  Off a
/// terminal one line goes out at the start and one at the end.
struct Progress {
    /// The full reference, for the closing line.
    reference: String,
    /// `repository-tail:tag`, for the redrawn line.
    short: String,
    started: Instant,
    tty: bool,
    last_draw: Option<Instant>,
}

/// Transfers under this show no running progress: they are over before a
/// redraw would mean anything.
const PROGRESS_MIN_BYTES: u64 = 4 * 1024 * 1024;

impl Progress {
    fn start(reference: &Reference) -> Progress {
        let tty = std::io::stderr().is_terminal();
        if !tty {
            eprintln!("pulling {reference}...");
        }
        let repo = reference
            .repository
            .rsplit('/')
            .next()
            .unwrap_or(&reference.repository);
        Progress {
            reference: reference.to_string(),
            short: format!("{repo}:{}", reference.reference),
            started: Instant::now(),
            tty,
            last_draw: None,
        }
    }

    fn update(&mut self, done: u64, total: u64) {
        if !self.tty || total < PROGRESS_MIN_BYTES {
            return;
        }
        let due = self
            .last_draw
            .is_none_or(|t| t.elapsed() >= Duration::from_millis(200));
        if !due && done < total {
            return;
        }
        self.last_draw = Some(Instant::now());
        eprint!(
            "\rpulling {}  {:.1} / {:.1} MiB ({:>3}%)",
            self.short,
            mib(done),
            mib(total),
            done * 100 / total
        );
        let _ = std::io::stderr().flush();
    }

    fn finish(self, bytes: u64) {
        let secs = self.started.elapsed().as_secs_f64();
        if self.last_draw.is_some() {
            eprint!("\r\x1b[2K");
        }
        eprintln!(
            "pulled {}  {:.1} MiB in {secs:.1} s",
            self.reference,
            mib(bytes)
        );
    }
}

pub fn mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

// ── Warm snapshots ───────────────────────────────────────────────

/// The file in a cached snapshot's directory holding the stamp it was
/// taken under (see `stamp` in the command module).
pub const STAMP_FILE: &str = "stamp";

/// Where warm snapshots live: one directory per stamp, named by its hash,
/// shared by every project whose guest is the same.
pub fn snapshots_dir() -> Result<PathBuf, String> {
    Ok(cache_dir()?.join("snapshots"))
}

/// The cache slot for a snapshot with this stamp.
pub fn snapshot_slot(stamp: &str) -> Result<PathBuf, String> {
    let hex: String = Sha256::digest(stamp.as_bytes())
        .iter()
        .take(8)
        .map(|b| format!("{b:02x}"))
        .collect();
    Ok(snapshots_dir()?.join(hex))
}

/// The cached snapshot taken under exactly this stamp, if there is one.
pub fn find_snapshot(stamp: &str) -> Result<Option<PathBuf>, String> {
    let slot = snapshot_slot(stamp)?;
    let stamped = fs::read_to_string(slot.join(STAMP_FILE)).is_ok_and(|s| s == stamp);
    Ok(stamped.then_some(slot))
}

/// Remove the snapshots taken from an earlier version of `stamp`'s rootfs
/// file (same path, different size or mtime: it was rebuilt), other than
/// `keep`.  Snapshots of the same file under other settings (memory,
/// entry, `warm_exec`) are other guests and stay.  Returns how many went.
pub fn prune_superseded(stamp: &str, keep: &Path) -> Result<usize, String> {
    let Some(rootfs) = stamp_field(stamp, "rootfs") else {
        return Ok(0);
    };
    let same_file = |other: &str| {
        stamp_field(other, "size") == stamp_field(stamp, "size")
            && stamp_field(other, "mtime") == stamp_field(stamp, "mtime")
    };
    let mut pruned = 0;
    for entry in list_snapshots()? {
        if entry.dir != keep
            && stamp_field(&entry.stamp, "rootfs") == Some(rootfs)
            && !same_file(&entry.stamp)
        {
            fs::remove_dir_all(&entry.dir)
                .map_err(|e| format!("cannot remove {}: {e}", entry.dir.display()))?;
            pruned += 1;
        }
    }
    Ok(pruned)
}

/// One `key=value` line of a stamp.
fn stamp_field<'a>(stamp: &'a str, key: &str) -> Option<&'a str> {
    stamp.lines().find_map(|line| {
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix('='))
    })
}

/// A cached warm snapshot.
pub struct SnapshotEntry {
    pub dir: PathBuf,
    pub stamp: String,
    pub bytes: u64,
}

impl SnapshotEntry {
    /// What the snapshot was taken from, for `hluk cache ls`.
    pub fn describe(&self) -> String {
        let mut out = String::new();
        for key in ["rootfs", "scratch_mb", "entry", "warm_exec"] {
            if let Some(value) = stamp_field(&self.stamp, key)
                && !value.is_empty()
            {
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(key);
                out.push('=');
                if value.contains(' ') {
                    out.push_str(&format!("{value:?}"));
                } else {
                    out.push_str(value);
                }
            }
        }
        out
    }
}

/// Every complete snapshot in the cache (a `.part` still being written is
/// not one).
pub fn list_snapshots() -> Result<Vec<SnapshotEntry>, String> {
    let dir = snapshots_dir()?;
    let Ok(entries) = fs::read_dir(&dir) else {
        return Ok(Vec::new());
    };
    let mut out = Vec::new();
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_dir() || path.extension().is_some() {
            continue;
        }
        let Ok(stamp) = fs::read_to_string(path.join(STAMP_FILE)) else {
            continue;
        };
        let bytes = dir_size(&path);
        out.push(SnapshotEntry {
            dir: path,
            stamp,
            bytes,
        });
    }
    out.sort_by(|a, b| a.dir.cmp(&b.dir));
    Ok(out)
}

/// Every pulled rootfs in the cache, with its size.
pub fn list_rootfs() -> Result<Vec<(PathBuf, u64)>, String> {
    fn walk(dir: &Path, out: &mut Vec<(PathBuf, u64)>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "cpio") {
                let bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                out.push((path, bytes));
            }
        }
    }
    let mut out = Vec::new();
    walk(&cache_dir()?.join("rootfs"), &mut out);
    out.sort();
    Ok(out)
}

/// The bytes under `path`.
pub fn dir_size(path: &Path) -> u64 {
    let Ok(entries) = fs::read_dir(path) else {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    };
    entries
        .filter_map(Result::ok)
        .map(|e| {
            let p = e.path();
            if p.is_dir() {
                dir_size(&p)
            } else {
                fs::metadata(&p).map(|m| m.len()).unwrap_or(0)
            }
        })
        .sum()
}

/// Remove `path` and everything under it, returning the bytes freed; a
/// path that is not there frees nothing.
pub fn remove_dir(path: &Path) -> Result<u64, String> {
    if !path.exists() {
        return Ok(0);
    }
    let bytes = dir_size(path);
    fs::remove_dir_all(path).map_err(|e| format!("cannot remove {}: {e}", path.display()))?;
    Ok(bytes)
}

/// Where `hluk build` writes a Dockerfile rootfs.
pub fn built_rootfs(manifest: &Manifest) -> PathBuf {
    manifest.state_dir().join("rootfs.cpio")
}

/// Build the manifest's Dockerfile with Docker and export the image's
/// filesystem as the project's CPIO.  The build context is the manifest's
/// directory, so the Dockerfile can `COPY` the project's files.
pub fn build_dockerfile(
    manifest: &Manifest,
    dockerfile: &Path,
) -> Result<(PathBuf, cpio::Stats), String> {
    let dockerfile = manifest.resolve(dockerfile);
    if !dockerfile.is_file() {
        return Err(format!("no Dockerfile at {}", dockerfile.display()));
    }
    // A Docker tag is lowercase letters, digits and a few separators; a
    // hand-edited name may carry anything.
    let tag: String = format!("hluk-{}-rootfs", manifest.app.name.to_ascii_lowercase())
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') {
                c
            } else {
                '-'
            }
        })
        .collect();
    let output = built_rootfs(manifest);
    fs::create_dir_all(output.parent().unwrap()).map_err(|e| e.to_string())?;

    eprintln!("building {} as {tag}", dockerfile.display());
    let status = Command::new("docker")
        .args(["build", "-t", &tag, "-f"])
        .arg(&dockerfile)
        .arg(&manifest.dir)
        .status()
        .map_err(docker_error)?;
    if !status.success() {
        return Err(format!("docker build failed ({status})"));
    }

    // A scratch image with no CMD refuses `docker create` unless given an
    // entrypoint; `/` is one it never runs.
    let cid = docker_output(&["create", "--entrypoint=/", &tag])
        .or_else(|_| docker_output(&["create", &tag]))?;
    let cid = cid.trim().to_string();
    let result = export(&cid, &output);
    let _ = Command::new("docker")
        .args(["rm", &cid])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let stats = result?;
    Ok((output, stats))
}

/// `docker export` the container to `output` as a CPIO.
fn export(cid: &str, output: &Path) -> Result<cpio::Stats, String> {
    let mut child = Command::new("docker")
        .args(["export", cid])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(docker_error)?;
    let tar = child.stdout.take().expect("piped");
    let part = output.with_extension("cpio.part");
    let stats = {
        let file =
            fs::File::create(&part).map_err(|e| format!("cannot write {}: {e}", part.display()))?;
        let result = cpio::from_tar(tar, std::io::BufWriter::new(file));
        let status = child.wait().map_err(|e| e.to_string())?;
        if !status.success() {
            let _ = fs::remove_file(&part);
            return Err(format!("docker export failed ({status})"));
        }
        result.inspect_err(|_| {
            let _ = fs::remove_file(&part);
        })?
    };
    fs::rename(&part, output)
        .map_err(|e| format!("cannot move into place {}: {e}", output.display()))?;
    Ok(stats)
}

fn docker_output(args: &[&str]) -> Result<String, String> {
    let out = Command::new("docker")
        .args(args)
        .stderr(Stdio::piped())
        .output()
        .map_err(docker_error)?;
    if !out.status.success() {
        return Err(format!(
            "docker {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn docker_error(e: std::io::Error) -> String {
    if e.kind() == std::io::ErrorKind::NotFound {
        "docker is not on PATH: a [rootfs] dockerfile is built with Docker (Docker Desktop on \
         Windows and macOS). Pull a published image instead with [rootfs] image, or build the \
         CPIO elsewhere and name it with [rootfs] path"
            .to_string()
    } else {
        format!("cannot run docker: {e}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stamp_describes_itself() {
        let entry = SnapshotEntry {
            dir: PathBuf::from("/c/s/ab"),
            stamp: "key=k\nrootfs=/p/r.cpio\nsize=1\nmtime=2\nscratch_mb=768\nentry=\nwarm_exec=import flask, pandas\n".into(),
            bytes: 0,
        };
        assert_eq!(
            entry.describe(),
            "rootfs=/p/r.cpio scratch_mb=768 warm_exec=\"import flask, pandas\""
        );
        assert_eq!(stamp_field(&entry.stamp, "scratch_mb"), Some("768"));
        assert_eq!(stamp_field(&entry.stamp, "entry"), Some(""));
        assert_eq!(stamp_field(&entry.stamp, "nope"), None);
    }

    #[test]
    fn cache_dir_honours_the_override() {
        // Environment is process-wide; this test sets and restores one
        // variable and the suite has no other reader of it.
        let before = std::env::var_os("HLUK_CACHE_DIR");
        unsafe { std::env::set_var("HLUK_CACHE_DIR", "/tmp/hluk-cache-test") };
        assert_eq!(cache_dir().unwrap(), PathBuf::from("/tmp/hluk-cache-test"));
        let r = Reference::parse("ghcr.io/o/r/python:initrd-v1").unwrap();
        assert_eq!(
            cached_initrd(&r).unwrap(),
            PathBuf::from("/tmp/hluk-cache-test/rootfs/ghcr.io/o/r/python/initrd-v1.cpio")
        );
        match before {
            Some(v) => unsafe { std::env::set_var("HLUK_CACHE_DIR", v) },
            None => unsafe { std::env::remove_var("HLUK_CACHE_DIR") },
        }
    }
}
