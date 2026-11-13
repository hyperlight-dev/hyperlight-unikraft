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

use super::manifest::Manifest;
use super::registry::{Client, Reference, extract_initrd};
use hyperlight_unikraft::cpio;

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
            // A pinned image is shown by its tag, the name it is known by.
            short: format!(
                "{repo}:{}",
                reference.tag.as_deref().unwrap_or(&reference.reference)
            ),
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

/// When the snapshot in `dir` was written, which tells it from one another
/// run puts in its place.
pub fn snapshot_written(dir: &Path) -> Option<std::time::SystemTime> {
    fs::metadata(dir.join(STAMP_FILE))
        .and_then(|m| m.modified())
        .ok()
}

/// Move the snapshot written at `part`, under `stamp`, into `slot`.  What is
/// there already stays, and `part` goes, when it is a snapshot under the
/// same stamp (so of the same guest) that another run put there: runs may
/// be restoring from it.  It is replaced when it is anything else: an
/// incomplete or foreign directory, or the snapshot written at
/// `unusable`, which this run found would not load.
pub fn install_snapshot(
    part: &Path,
    slot: &Path,
    stamp: &str,
    unusable: Option<std::time::SystemTime>,
) -> Result<(), String> {
    for _ in 0..2 {
        if fs::rename(part, slot).is_ok() {
            return Ok(());
        }
        if !slot.is_dir() {
            break;
        }
        let ours = fs::read_to_string(slot.join(STAMP_FILE)).is_ok_and(|s| s == stamp);
        if ours && (unusable.is_none() || snapshot_written(slot) != unusable) {
            let _ = fs::remove_dir_all(part);
            return Ok(());
        }
        let _ = fs::remove_dir_all(slot);
    }
    let _ = fs::remove_dir_all(part);
    if fs::read_to_string(slot.join(STAMP_FILE)).is_ok_and(|s| s == stamp) {
        Ok(())
    } else {
        Err(format!(
            "cannot save the warm snapshot at {}",
            slot.display()
        ))
    }
}

/// How old a snapshot still being written may be before it is taken for
/// one a killed run left behind: saving one takes seconds.
const ABANDONED_PART_AGE: std::time::Duration = std::time::Duration::from_secs(3600);

/// Remove the `.part` directories in `dir` that runs killed while saving a
/// snapshot left behind.
pub fn remove_abandoned_parts(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let old = entry
            .metadata()
            .and_then(|m| m.modified())
            .is_ok_and(|t| t.elapsed().is_ok_and(|age| age > ABANDONED_PART_AGE));
        if path.extension().is_some_and(|e| e == "part") && path.is_dir() && old {
            let _ = fs::remove_dir_all(&path);
        }
    }
}

/// The warm snapshots kept of one guest that differ in their environment
/// alone (see `prune_superseded`).
const ENV_VARIANTS_KEPT: usize = 4;

/// Remove the snapshots of `stamp`'s rootfs that no run can restore any
/// more, other than `keep`: those of an earlier version of the file (same
/// path, different size or mtime: it was rebuilt), and those another hluk
/// build saved (another `key`: its kernel or host functions, which this
/// build refuses).  Without the second, every hluk upgrade left a
/// snapshot of each guest behind for good.  Snapshots of the same file
/// under other settings (memory, entry, `warm_exec`) are other guests and
/// stay; of those that differ in their environment alone, the newest
/// [`ENV_VARIANTS_KEPT`] do.  Returns how many went.
pub fn prune_superseded(stamp: &str, keep: &Path) -> Result<usize, String> {
    prune_among(list_snapshots()?, stamp, keep)
}

/// [`prune_superseded`] over `snapshots`.
fn prune_among(snapshots: Vec<SnapshotEntry>, stamp: &str, keep: &Path) -> Result<usize, String> {
    let Some(rootfs) = stamp_field(stamp, "rootfs") else {
        return Ok(0);
    };
    let restorable = |other: &str| {
        ["key", "size", "mtime"]
            .iter()
            .all(|k| stamp_field(other, k) == stamp_field(stamp, k))
    };
    // Snapshots keyed by an environment (see `stamp`) differ in it alone:
    // keep the newest few, so runs alternating between a few values all
    // stay warm, but each new value a run passes does not add one for good.
    fn without_env(s: &str) -> Vec<&str> {
        s.lines().filter(|l| !l.starts_with("env=")).collect()
    }
    let mut pruned = 0;
    let mut variants = Vec::new();
    for entry in snapshots {
        if entry.dir == keep || stamp_field(&entry.stamp, "rootfs") != Some(rootfs) {
            continue;
        }
        if !restorable(&entry.stamp) {
            fs::remove_dir_all(&entry.dir)
                .map_err(|e| format!("cannot remove {}: {e}", entry.dir.display()))?;
            pruned += 1;
        } else if without_env(&entry.stamp) == without_env(stamp) {
            let saved = fs::metadata(&entry.dir).and_then(|m| m.modified()).ok();
            variants.push((saved, entry.dir));
        }
    }
    // Newest first; past the few kept (ours is one of them), they go.
    variants.sort_by_key(|v| std::cmp::Reverse(v.0));
    // Best effort: a concurrent run may have pruned one first, or (on
    // Windows) have one open; the next save tries again.
    for (_, dir) in variants.into_iter().skip(ENV_VARIANTS_KEPT - 1) {
        if fs::remove_dir_all(&dir).is_ok() {
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
                && value != "\"\""
            {
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(key);
                out.push('=');
                if key == "warm_exec" {
                    // Debug-quoted in the stamp already
                    out.push_str(value);
                } else if value.contains(' ') {
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
    // For the host's CPU, which the guest runs on, whatever platform
    // Docker would default to.
    let platform = if cfg!(target_arch = "aarch64") {
        "linux/arm64"
    } else {
        "linux/amd64"
    };
    let status = Command::new("docker")
        .args(["build", "--platform", platform, "-t", &tag, "-f"])
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
    // Before docker starts, so a failure here leaves no child to reap.
    let part = output.with_extension("cpio.part");
    let file =
        fs::File::create(&part).map_err(|e| format!("cannot write {}: {e}", part.display()))?;
    let mut child = match Command::new("docker")
        .args(["export", cid])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            let _ = fs::remove_file(&part);
            return Err(docker_error(e));
        }
    };
    let mut tar = std::io::BufReader::with_capacity(64 << 10, child.stdout.take().expect("piped"));
    // Borrowed, so the pipe stays open until docker's fate is known: a
    // closed pipe kills it with SIGPIPE, which would read as its failure.
    let result = cpio::from_tar(&mut tar, std::io::BufWriter::with_capacity(256 << 10, file));
    // A conversion that fails while docker still streams is the cause; it
    // is killed, and its status after that says nothing.  If docker had
    // already exited, its stream may have been cut short, so both count.
    // Docker's own reason, if any, is on its stderr, which is ours.
    let killed =
        result.is_err() && !matches!(child.try_wait(), Ok(Some(_))) && child.kill().is_ok();
    drop(tar);
    let status = match child.wait() {
        Err(e) => Err(format!("waiting for docker export: {e}")),
        Ok(status) if !killed && !status.success() => {
            Err(format!("docker export failed ({status})"))
        }
        Ok(_) => Ok(()),
    };
    let result = match (result, status) {
        // After a kill, only a failed wait comes back as Err.
        (Err(e), Err(docker)) => Err(format!("{docker}: {e}")),
        (Err(e), _) => Err(e.to_string()),
        (Ok(_), Err(docker)) => Err(docker),
        (Ok(stats), Ok(())) => Ok(stats),
    };
    let stats = result.inspect_err(|_| {
        let _ = fs::remove_file(&part);
    })?;
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

    /// Snapshots no run can restore go: a rebuilt rootfs's, and another
    /// hluk build's; another guest of the same file, and another rootfs's,
    /// stay.
    #[test]
    fn prune_drops_what_cannot_be_restored() {
        let dir = tempfile::tempdir().unwrap();
        let stamp = |key: &str, rootfs: &str, mtime: u32, scratch: u32| {
            format!(
                "key={key}\nrootfs={rootfs}\nsize=1\nmtime={mtime}\nscratch_mb={scratch}\nentry=\nwarm_exec=\"\"\n"
            )
        };
        let entry = |name: &str, stamp: String| {
            let d = dir.path().join(name);
            std::fs::create_dir(&d).unwrap();
            SnapshotEntry {
                dir: d,
                stamp,
                bytes: 0,
            }
        };
        let ours = stamp("k1", "/r.cpio", 2, 256);
        let snapshots = vec![
            entry("ours", ours.clone()),
            entry("rebuilt", stamp("k1", "/r.cpio", 1, 256)),
            entry("old-hluk", stamp("k0", "/r.cpio", 2, 256)),
            entry("other-guest", stamp("k1", "/r.cpio", 2, 512)),
            entry("other-rootfs", stamp("k0", "/s.cpio", 2, 256)),
        ];
        let keep = dir.path().join("ours");
        assert_eq!(prune_among(snapshots, &ours, &keep), Ok(2));
        let left = |name: &str| dir.path().join(name).exists();
        assert!(left("ours") && left("other-guest") && left("other-rootfs"));
        assert!(!left("rebuilt") && !left("old-hluk"));
    }

    /// A saved snapshot moves into an empty slot.  One another run placed
    /// there under the same stamp stays (runs may restore from it) and ours
    /// goes; anything else there is replaced: a directory without the stamp,
    /// one under another stamp, and the snapshot this run found would not
    /// load, but not one a concurrent run already put in its place.
    #[test]
    fn install_keeps_only_a_good_snapshot_already_in_the_slot() {
        let dir = tempfile::tempdir().unwrap();
        let n = std::cell::Cell::new(0);
        let part = |stamp: Option<&str>, content: &str| {
            n.set(n.get() + 1);
            let d = dir.path().join(format!("slot.{}.part", n.get()));
            std::fs::create_dir(&d).unwrap();
            std::fs::write(d.join("content"), content).unwrap();
            if let Some(stamp) = stamp {
                std::fs::write(d.join(STAMP_FILE), stamp).unwrap();
            }
            d
        };
        let slot = dir.path().join("slot");
        let content = || std::fs::read_to_string(slot.join("content")).unwrap();

        install_snapshot(&part(Some("s"), "first"), &slot, "s", None).unwrap();
        assert_eq!(content(), "first");
        install_snapshot(&part(Some("s"), "second"), &slot, "s", None).unwrap();
        assert_eq!(content(), "first", "a concurrent run's snapshot stays");
        assert_eq!(
            std::fs::read_dir(dir.path()).unwrap().count(),
            1,
            "ours goes"
        );

        // Found unusable here, and already repaired by another run: the
        // repair stays.  Still the one found unusable: it is replaced.
        let broken = snapshot_written(&slot);
        std::fs::remove_dir_all(&slot).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        install_snapshot(&part(Some("s"), "repaired"), &slot, "s", None).unwrap();
        install_snapshot(&part(Some("s"), "late repair"), &slot, "s", broken).unwrap();
        assert_eq!(content(), "repaired", "a concurrent repair stays");
        let found = snapshot_written(&slot);
        install_snapshot(&part(Some("s"), "fixed"), &slot, "s", found).unwrap();
        assert_eq!(
            content(),
            "fixed",
            "the snapshot found unusable is replaced"
        );

        for junk in [None, Some("other")] {
            std::fs::remove_dir_all(&slot).unwrap();
            std::fs::rename(part(junk, "junk"), &slot).unwrap();
            install_snapshot(&part(Some("s"), "saved"), &slot, "s", None).unwrap();
            assert_eq!(content(), "saved", "a slot holding {junk:?} is replaced");
        }
    }

    /// The `.part` directories a killed run left behind go once they are
    /// old enough that no run is still writing them; snapshots stay.
    #[cfg(unix)]
    #[test]
    fn abandoned_parts_go() {
        let dir = tempfile::tempdir().unwrap();
        for name in ["ab.1.part", "ab.2.part", "ab"] {
            std::fs::create_dir(dir.path().join(name)).unwrap();
        }
        let old = std::time::SystemTime::now() - ABANDONED_PART_AGE * 2;
        std::fs::File::open(dir.path().join("ab.1.part"))
            .unwrap()
            .set_modified(old)
            .unwrap();
        remove_abandoned_parts(dir.path());
        assert!(
            !dir.path().join("ab.1.part").exists(),
            "an abandoned part goes"
        );
        assert!(
            dir.path().join("ab.2.part").exists(),
            "one being written stays"
        );
        assert!(dir.path().join("ab").exists(), "a snapshot stays");
    }

    #[test]
    fn a_stamp_describes_itself() {
        let entry = SnapshotEntry {
            dir: PathBuf::from("/c/s/ab"),
            stamp: "key=k\nrootfs=/p/r.cpio\nsize=1\nmtime=2\nscratch_mb=768\nentry=\nwarm_exec=\"import flask, pandas\"\n".into(),
            bytes: 0,
        };
        assert_eq!(
            entry.describe(),
            "rootfs=/p/r.cpio scratch_mb=768 warm_exec=\"import flask, pandas\""
        );
        assert_eq!(stamp_field(&entry.stamp, "scratch_mb"), Some("768"));
        assert_eq!(stamp_field(&entry.stamp, "entry"), Some(""));
        let bare = SnapshotEntry {
            stamp: "key=k\nrootfs=/p/r.cpio\nscratch_mb=768\nentry=\nwarm_exec=\"\"\n".into(),
            ..entry
        };
        assert_eq!(bare.describe(), "rootfs=/p/r.cpio scratch_mb=768");
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
