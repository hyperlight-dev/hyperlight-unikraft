// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The `fs_*` host functions, called with what a guest can send them.
//!
//! Each input is a run of calls on four mounts: a host directory read-write
//! and read-only, and an embedder's backend read-write and read-only, whose
//! answers the input scripts, lies included.  The directory sits two levels
//! below directories the mount must not reach, each with a secret file, and
//! holds symbolic links that point out of it.  Every reply must be well
//! formed, with a Linux errno for a failure; a read-only mount must refuse
//! every write with `EROFS`, and its backend never be asked to write; and
//! after the run, nothing outside the mount may have changed, no read may
//! have returned a secret, and no file in the mount may be a secret's hard
//! link or set-user-ID, set-group-ID or sticky.

#![no_main]

use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hyperlight_common::flatbuffer_wrappers::function_types::{ParameterValue, ReturnValue};
use hyperlight_unikraft::fuzzing::{FS_CHUNK, HostCalls};
use hyperlight_unikraft::{DirEntry, FileStat, FsBackend, Mount, S_IFDIR, S_IFMT, S_IFREG};
use hyperlight_unikraft_fuzz::{
    RawCall, bytes, call, check_error_reply, check_status, int, open_fds, reply_status,
};
use libfuzzer_sys::arbitrary::{self, Arbitrary};
use libfuzzer_sys::fuzz_target;

const SECRET: &[u8] = b"a secret the guest must never read";
const EROFS: i32 = 30;
/// The longest name a listing carries, and link target `fs_readlink` does.
const NAME_MAX: usize = 255;
const PATH_MAX: usize = 4095;
/// `fs_stat`'s reply: status, size, mode, two kind bytes and three times.
const STAT_LEN: usize = 4 + 8 + 4 + 2 + 3 * 12;

fuzz_target!(|input: Input| run(input));

#[derive(Arbitrary, Debug)]
struct Input {
    /// What the backend mounts answer.
    script: Script,
    ops: Vec<Op>,
}

/// One mount, by its index, or any index.
#[derive(Arbitrary, Debug, Clone, Copy)]
enum MountIdx {
    Dir,
    DirReadOnly,
    Backend,
    BackendReadOnly,
    Any(i32),
}

impl MountIdx {
    fn index(self) -> i32 {
        match self {
            Self::Dir => 0,
            Self::DirReadOnly => 1,
            Self::Backend => 2,
            Self::BackendReadOnly => 3,
            Self::Any(i) => i,
        }
    }

    fn readonly(self) -> bool {
        matches!(self.index(), 1 | 3)
    }
}

/// A path as the guest names it: mostly made of the names in the mount and
/// of `.`, `..` and empty components, which is where escapes would be.
#[derive(Arbitrary, Debug)]
enum GuestPath {
    Parts { absolute: bool, parts: Vec<Part> },
    Any(String),
}

#[derive(Arbitrary, Debug)]
enum Part {
    Name(u8),
    Dot,
    DotDot,
    Empty,
}

/// The names in the mount, those above it, and one that is nowhere.
const NAMES: &[&str] = &[
    "file", "dir", "nested", "link", "escape", "absolute", "secret", "mnt", "new",
];

impl GuestPath {
    fn text(&self) -> String {
        match self {
            Self::Parts { absolute, parts } => {
                let parts: Vec<&str> = parts
                    .iter()
                    .map(|p| match p {
                        Part::Name(i) => NAMES[usize::from(*i) % NAMES.len()],
                        Part::Dot => ".",
                        Part::DotDot => "..",
                        Part::Empty => "",
                    })
                    .collect();
                let path = parts.join("/");
                if *absolute { format!("/{path}") } else { path }
            }
            Self::Any(s) => s.clone(),
        }
    }
}

#[derive(Arbitrary, Debug)]
enum Op {
    Stat(MountIdx, GuestPath),
    Read(MountIdx, GuestPath, u64, u64),
    Write(MountIdx, GuestPath, u64, i32, Vec<u8>),
    Mkdir(MountIdx, GuestPath),
    Unlink(MountIdx, GuestPath),
    Truncate(MountIdx, GuestPath, u64),
    List(MountIdx, GuestPath),
    Rename(MountIdx, GuestPath, GuestPath),
    Symlink(MountIdx, GuestPath, GuestPath),
    Readlink(MountIdx, GuestPath),
    Link(MountIdx, GuestPath, GuestPath),
    Chmod(MountIdx, GuestPath, i32),
    SetTimes(MountIdx, GuestPath, i32, i64, i32, i64, i32),
    Raw(RawCall),
}

impl Op {
    fn mount(&self) -> Option<MountIdx> {
        match self {
            Self::Stat(m, _)
            | Self::Read(m, ..)
            | Self::Write(m, ..)
            | Self::Mkdir(m, _)
            | Self::Unlink(m, _)
            | Self::Truncate(m, ..)
            | Self::List(m, _)
            | Self::Rename(m, ..)
            | Self::Symlink(m, ..)
            | Self::Readlink(m, _)
            | Self::Link(m, ..)
            | Self::Chmod(m, ..)
            | Self::SetTimes(m, ..) => Some(*m),
            Self::Raw(_) => None,
        }
    }

    /// Whether the call writes, which a read-only mount refuses.
    fn writes(&self) -> bool {
        !matches!(
            self,
            Self::Stat(..) | Self::Read(..) | Self::List(..) | Self::Readlink(..) | Self::Raw(_)
        )
    }
}

// ── The scripted backend ────────────────────────────────────────────

/// A backend's answer: a value, or an error with any OS code.
#[derive(Arbitrary, Debug, Clone)]
enum Answer<T> {
    Ok(T),
    Os(i32),
    Other,
}

impl<T: Clone> Answer<T> {
    fn get(&self) -> io::Result<T> {
        match self {
            Self::Ok(v) => Ok(v.clone()),
            Self::Os(code) => Err(io::Error::from_raw_os_error(*code)),
            Self::Other => Err(io::Error::other("the backend failed")),
        }
    }
}

/// A time before or after the epoch.
#[derive(Arbitrary, Debug, Clone, Copy)]
struct Time {
    before_epoch: bool,
    secs: u64,
    nanos: u32,
}

impl Time {
    fn get(self) -> SystemTime {
        let d = Duration::new(self.secs, self.nanos % 1_000_000_000);
        let t = if self.before_epoch {
            UNIX_EPOCH.checked_sub(d)
        } else {
            UNIX_EPOCH.checked_add(d)
        };
        t.unwrap_or(UNIX_EPOCH)
    }
}

#[derive(Arbitrary, Debug, Clone)]
struct Script {
    stat: Answer<(u64, u32, [Time; 3])>,
    /// How many bytes a read says it read, which may be more than it was
    /// given room for.
    read: Answer<usize>,
    /// A listing: these entries, this many times over (a large directory).
    list: Answer<(Vec<(String, bool)>, u16)>,
    read_link: Answer<String>,
    /// Every other method's.
    other: Answer<()>,
}

/// A backend that answers as the script says, and notes whether it was
/// asked to write.
struct Scripted {
    script: Script,
    wrote: AtomicBool,
}

impl Scripted {
    fn write_op(&self) -> io::Result<()> {
        self.wrote.store(true, Ordering::Relaxed);
        self.script.other.get()
    }
}

impl FsBackend for Scripted {
    fn stat(&self, _: &str) -> io::Result<FileStat> {
        let (size, mode, [a, m, c]) = self.script.stat.get()?;
        // The mode's low bit picks a file or a directory.
        let stat = if mode & 1 == 0 {
            FileStat::file(size, mode)
        } else {
            FileStat::dir(mode)
        };
        Ok(stat.times(a.get(), m.get(), c.get()))
    }
    fn read(&self, _: &str, _: u64, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.script.read.get()?;
        let fill = n.min(buf.len());
        buf[..fill].fill(0x5a);
        Ok(n)
    }
    fn write(&self, _: &str, _: Option<u64>, _: &[u8]) -> io::Result<()> {
        self.write_op()
    }
    fn create_dir(&self, _: &str) -> io::Result<()> {
        self.write_op()
    }
    fn remove(&self, _: &str) -> io::Result<()> {
        self.write_op()
    }
    fn truncate(&self, _: &str, _: u64) -> io::Result<()> {
        self.write_op()
    }
    fn list(&self, _: &str) -> io::Result<Vec<DirEntry>> {
        let (entries, times) = self.script.list.get()?;
        // Enough for any listing to outgrow a reply.
        let times = usize::from(times).min(1 << 14);
        Ok(entries
            .iter()
            .cycle()
            .take(entries.len() * times)
            .map(|(name, is_dir)| DirEntry::new(name.clone(), *is_dir))
            .collect())
    }
    fn rename(&self, _: &str, _: &str) -> io::Result<()> {
        self.write_op()
    }
    fn symlink(&self, _: &str, _: &str) -> io::Result<()> {
        self.write_op()
    }
    fn read_link(&self, _: &str) -> io::Result<String> {
        self.script.read_link.get()
    }
    fn hard_link(&self, _: &str, _: &str) -> io::Result<()> {
        self.write_op()
    }
    fn chmod(&self, _: &str, _: u32) -> io::Result<()> {
        self.write_op()
    }
    fn set_times(&self, _: &str, _: Option<SystemTime>, _: Option<SystemTime>) -> io::Result<()> {
        self.write_op()
    }
}

// ── The run ─────────────────────────────────────────────────────────

/// A directory above the mount, as it was: what the run must leave as it
/// found it.
#[derive(Debug, PartialEq)]
struct Above {
    mode: u32,
    names: Vec<String>,
    secret: Vec<u8>,
    secret_mode: u32,
    secret_modified: SystemTime,
}

fn above(dirs: &[&Path]) -> Vec<Above> {
    dirs.iter()
        .map(|dir| {
            let mut names: Vec<String> = std::fs::read_dir(dir)
                .unwrap()
                .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
            names.sort_unstable();
            let secret = dir.join("secret");
            let meta = std::fs::symlink_metadata(&secret).unwrap();
            Above {
                mode: std::fs::metadata(dir).unwrap().mode(),
                names,
                secret: std::fs::read(&secret).unwrap(),
                secret_mode: meta.mode(),
                secret_modified: meta.modified().unwrap(),
            }
        })
        .collect()
}

fn run(input: Input) {
    let fds = open_fds();
    {
        // top/secret, top/sub/secret, and the mount at top/sub/mnt
        let top = tempfile::tempdir().unwrap();
        let sub = top.path().join("sub");
        let mnt = sub.join("mnt");
        std::fs::create_dir_all(mnt.join("dir")).unwrap();
        for dir in [top.path(), &sub] {
            std::fs::write(dir.join("secret"), SECRET).unwrap();
        }
        std::fs::write(mnt.join("file"), b"file").unwrap();
        std::fs::write(mnt.join("dir/nested"), b"nested").unwrap();
        std::os::unix::fs::symlink("file", mnt.join("link")).unwrap();
        std::os::unix::fs::symlink("../secret", mnt.join("escape")).unwrap();
        std::os::unix::fs::symlink(top.path().join("secret"), mnt.join("absolute")).unwrap();
        let before = above(&[top.path(), &sub]);
        let secrets: Vec<(u64, u64)> = [top.path(), &sub]
            .iter()
            .map(|d| {
                let m = std::fs::metadata(d.join("secret")).unwrap();
                (m.dev(), m.ino())
            })
            .collect();

        let backend = Arc::new(Scripted {
            script: input.script.clone(),
            wrote: AtomicBool::new(false),
        });
        let backend_ro = Arc::new(Scripted {
            script: input.script,
            wrote: AtomicBool::new(false),
        });
        let calls = HostCalls::new(
            &[
                Mount::rw(&mnt, "/mnt/rw"),
                Mount::ro(&mnt, "/mnt/ro"),
                Mount::backend(backend, "/mnt/backend", false),
                Mount::backend(backend_ro.clone(), "/mnt/backend-ro", true),
            ],
            None,
            None,
        )
        .unwrap();

        for op in input.ops {
            make(&calls, op);
        }
        assert!(
            !backend_ro.wrote.load(Ordering::Relaxed),
            "a read-only mount's backend was asked to write"
        );

        restore_permissions(&mnt);
        assert_eq!(
            above(&[top.path(), &sub]),
            before,
            "something outside the mount changed"
        );
        check_mount(&mnt, &secrets);
        drop(calls);
    }
    assert_eq!(open_fds(), fds, "the run left file descriptors open");
}

/// Make every directory in the mount searchable and writable again, as a
/// `fs_chmod` may have left one neither, so that the checks can walk it
/// and the temporary directory be removed.
fn restore_permissions(dir: &Path) {
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)).unwrap();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            restore_permissions(&entry.path());
        }
    }
}

/// No file in the mount may be a hard link to a secret, or be set-user-ID,
/// set-group-ID or sticky.
fn check_mount(dir: &Path, secrets: &[(u64, u64)]) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let meta = std::fs::symlink_metadata(entry.path()).unwrap();
        assert!(
            !secrets.contains(&(meta.dev(), meta.ino())),
            "{} is a hard link to a secret outside the mount",
            entry.path().display()
        );
        if !meta.file_type().is_symlink() {
            assert_eq!(
                meta.mode() & 0o7000,
                0,
                "{} has its mode's setuid, setgid or sticky bit",
                entry.path().display()
            );
        }
        if meta.is_dir() {
            check_mount(&entry.path(), secrets);
        }
    }
}

fn make(calls: &HostCalls, op: Op) {
    let refused = op.writes() && op.mount().is_some_and(MountIdx::readonly);
    // The most bytes a read may return: what it asked for, at least one and
    // at most a chunk.
    let read_max = match &op {
        Op::Read(_, _, _, len) => (*len).clamp(1, FS_CHUNK as u64) as usize,
        _ => FS_CHUNK,
    };
    let (name, reply) = match op {
        Op::Raw(raw) => match raw.call(calls) {
            Some((name, _, reply)) => (name, reply),
            None => return,
        },
        op => {
            let (name, args) = args(&op);
            (name.to_string(), call(calls, name, args))
        }
    };
    check_reply(&name, reply, refused, read_max);
}

/// The host function `op` calls, and its arguments.
fn args(op: &Op) -> (&'static str, Vec<ParameterValue>) {
    let p = |path: &GuestPath| ParameterValue::String(path.text());
    match op {
        Op::Stat(m, path) => ("fs_stat", vec![idx(*m), p(path)]),
        Op::Read(m, path, offset, len) => (
            "fs_read_bytes",
            vec![
                idx(*m),
                p(path),
                ParameterValue::ULong(*offset),
                ParameterValue::ULong(*len),
            ],
        ),
        Op::Write(m, path, offset, append, data) => (
            "fs_write_bytes",
            vec![
                idx(*m),
                p(path),
                ParameterValue::ULong(*offset),
                ParameterValue::Int(*append),
                ParameterValue::VecBytes(data.clone()),
            ],
        ),
        Op::Mkdir(m, path) => ("fs_mkdir", vec![idx(*m), p(path)]),
        Op::Unlink(m, path) => ("fs_unlink", vec![idx(*m), p(path)]),
        Op::Truncate(m, path, len) => (
            "fs_truncate",
            vec![idx(*m), p(path), ParameterValue::ULong(*len)],
        ),
        Op::List(m, path) => ("fs_list", vec![idx(*m), p(path)]),
        Op::Rename(m, from, to) => ("fs_rename", vec![idx(*m), p(from), p(to)]),
        Op::Symlink(m, link, target) => ("fs_symlink", vec![idx(*m), p(link), p(target)]),
        Op::Readlink(m, path) => ("fs_readlink", vec![idx(*m), p(path)]),
        Op::Link(m, src, dst) => ("fs_link", vec![idx(*m), p(src), p(dst)]),
        Op::Chmod(m, path, mode) => (
            "fs_chmod",
            vec![idx(*m), p(path), ParameterValue::Int(*mode)],
        ),
        Op::SetTimes(m, path, which, a_secs, a_nanos, m_secs, m_nanos) => (
            "fs_set_times",
            vec![
                idx(*m),
                p(path),
                ParameterValue::Int(*which),
                ParameterValue::Long(*a_secs),
                ParameterValue::Int(*a_nanos),
                ParameterValue::Long(*m_secs),
                ParameterValue::Int(*m_nanos),
            ],
        ),
        Op::Raw(_) => unreachable!("a raw call names its own function"),
    }
}

fn idx(m: MountIdx) -> ParameterValue {
    ParameterValue::Int(m.index())
}

fn check_reply(name: &str, reply: ReturnValue, refused: bool, read_max: usize) {
    match name {
        "fs_stat" => {
            let reply = bytes(name, reply);
            let status = reply_status(name, &reply);
            if status != 0 {
                return check_error_reply(name, &reply, status);
            }
            assert_eq!(reply.len(), STAT_LEN, "{name}: a reply of the wrong size");
            let mode = u32::from_le_bytes(reply[12..16].try_into().unwrap());
            assert_eq!(
                reply[16],
                u8::from(mode & S_IFMT == S_IFDIR),
                "{name}: is_dir"
            );
            assert_eq!(
                reply[17],
                u8::from(mode & S_IFMT == S_IFREG),
                "{name}: is_file"
            );
            for t in reply[18..].chunks(12) {
                let nanos = u32::from_le_bytes(t[8..].try_into().unwrap());
                assert!(nanos < 1_000_000_000, "{name}: {nanos} nanoseconds");
            }
        }
        "fs_read_bytes" => {
            let reply = bytes(name, reply);
            let status = reply_status(name, &reply);
            if status != 0 {
                return check_error_reply(name, &reply, status);
            }
            let data = &reply[4..];
            assert!(
                data.len() <= read_max,
                "{name}: {} bytes, past the {read_max} asked",
                data.len()
            );
            assert!(
                !data.windows(SECRET.len()).any(|w| w == SECRET),
                "{name}: read a secret from outside the mount"
            );
        }
        "fs_list" => {
            let reply = bytes(name, reply);
            let status = reply_status(name, &reply);
            assert!(reply.len() >= 8, "{name}: a reply without its count");
            let count = u32::from_le_bytes(reply[4..8].try_into().unwrap());
            if status != 0 {
                assert_eq!(
                    (reply.len(), count),
                    (8, 0),
                    "{name}: an error with entries"
                );
                return;
            }
            let mut rest = &reply[8..];
            for _ in 0..count {
                assert!(rest.len() >= 3, "{name}: an entry cut short");
                assert!(rest[0] <= 1, "{name}: is_dir {}", rest[0]);
                let len = usize::from(u16::from_le_bytes([rest[1], rest[2]]));
                let entry = rest.get(3..3 + len).expect("a name cut short");
                let entry = std::str::from_utf8(entry).expect("a name that is not UTF-8");
                assert!(
                    (1..=NAME_MAX).contains(&len)
                        && !entry.contains(['/', '\0'])
                        && entry != "."
                        && entry != "..",
                    "{name}: an entry named {entry:?}"
                );
                rest = &rest[3 + len..];
            }
            assert!(rest.is_empty(), "{name}: bytes after the last entry");
        }
        "fs_readlink" => {
            let reply = bytes(name, reply);
            let status = reply_status(name, &reply);
            if status != 0 {
                return check_error_reply(name, &reply, status);
            }
            assert!(
                reply.len() - 4 <= PATH_MAX,
                "{name}: a target past PATH_MAX"
            );
        }
        _ => {
            let status = match reply {
                ReturnValue::VecBytes(reply) => reply_status(name, &reply),
                reply => int(name, reply),
            };
            if refused {
                assert_eq!(status, -EROFS, "{name}: a read-only mount took a write");
            } else {
                check_status(name, status);
            }
        }
    }
}
