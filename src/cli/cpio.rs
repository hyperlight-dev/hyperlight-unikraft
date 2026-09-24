// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Turn a filesystem tar (what `docker export` streams) into the newc CPIO
//! the kernel takes as its initrd, the way `find . | cpio -o -H newc` lays
//! it out: a `.` entry first, then every path bare of a leading `./`, so
//! the driver detection and the guest see the same names as in the
//! published images.  Pure Rust, so `hluk build` needs Docker and nothing
//! else on the host.

use std::io::{self, Read, Write};

const S_IFREG: u32 = 0o100000;
const S_IFDIR: u32 = 0o040000;
const S_IFLNK: u32 = 0o120000;
const S_IFCHR: u32 = 0o020000;
const S_IFBLK: u32 = 0o060000;
const S_IFIFO: u32 = 0o010000;

/// `docker export` hands these over as the empty files it bind-mounts at
/// run time, so the archive gets working ones instead: a hostname that
/// resolves (`unikraft` is Unikraft's `gethostname()`), a lookup order,
/// and public resolvers.  `single-request` serialises the A and AAAA
/// queries glibc would otherwise send in parallel, which the guest's socket
/// layer does not support.  Matches the justfile's `_export-cpio`; a
/// `--resolv-conf` at run time still overrides the resolver file.
const FIXED_ETC: &[(&str, &str)] = &[
    (
        "etc/hosts",
        "127.0.0.1 localhost unikraft\n::1 localhost unikraft\n",
    ),
    ("etc/nsswitch.conf", "hosts: files dns\n"),
    (
        "etc/resolv.conf",
        "nameserver 8.8.8.8\nnameserver 1.1.1.1\noptions single-request\n",
    ),
];

/// What a conversion produced.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub entries: u64,
    pub bytes: u64,
}

/// The ownership, permissions and time an entry carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attrs {
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub mtime: u32,
}

impl Attrs {
    /// Root-owned, with `mode`, dated the epoch.
    pub const fn root(mode: u32) -> Attrs {
        Attrs {
            mode,
            uid: 0,
            gid: 0,
            mtime: 0,
        }
    }
}

/// A newc CPIO writer.
pub struct Writer<W: Write> {
    out: W,
    ino: u32,
    bytes: u64,
    entries: u64,
}

impl<W: Write> Writer<W> {
    /// Start an archive with its root `.` directory.
    pub fn new(out: W) -> io::Result<Self> {
        let mut w = Writer {
            out,
            ino: 0,
            bytes: 0,
            entries: 0,
        };
        w.header(".", S_IFDIR | 0o755, Attrs::root(0), 2, 0, (0, 0))?;
        Ok(w)
    }

    pub fn dir(&mut self, name: &str, attrs: Attrs) -> io::Result<()> {
        self.header(name, S_IFDIR | (attrs.mode & 0o7777), attrs, 2, 0, (0, 0))
    }

    pub fn file(
        &mut self,
        name: &str,
        attrs: Attrs,
        size: u64,
        data: &mut impl Read,
    ) -> io::Result<()> {
        self.header(
            name,
            S_IFREG | (attrs.mode & 0o7777),
            attrs,
            1,
            size,
            (0, 0),
        )?;
        let copied = io::copy(&mut data.take(size), &mut self.out)?;
        if copied != size {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("{name}: {copied} of {size} bytes"),
            ));
        }
        self.bytes += copied;
        self.pad(copied)
    }

    pub fn symlink(&mut self, name: &str, target: &str, attrs: Attrs) -> io::Result<()> {
        let target = target.as_bytes();
        self.header(name, S_IFLNK | 0o777, attrs, 1, target.len() as u64, (0, 0))?;
        self.out.write_all(target)?;
        self.bytes += target.len() as u64;
        self.pad(target.len() as u64)
    }

    /// A character or block device, or a FIFO; `kind` is one of the
    /// `S_IF*` type bits.
    pub fn special(
        &mut self,
        name: &str,
        kind: u32,
        attrs: Attrs,
        rdev: (u32, u32),
    ) -> io::Result<()> {
        self.header(name, kind | (attrs.mode & 0o7777), attrs, 1, 0, rdev)
    }

    /// Write the trailer and hand the writer back.
    pub fn finish(mut self) -> io::Result<(W, Stats)> {
        self.header("TRAILER!!!", 0, Attrs::root(0), 1, 0, (0, 0))?;
        self.out.flush()?;
        let stats = Stats {
            // The root and the trailer are bookkeeping, not content.
            entries: self.entries - 2,
            bytes: self.bytes,
        };
        Ok((self.out, stats))
    }

    fn header(
        &mut self,
        name: &str,
        mode: u32,
        attrs: Attrs,
        nlink: u32,
        filesize: u64,
        rdev: (u32, u32),
    ) -> io::Result<()> {
        self.ino += 1;
        self.entries += 1;
        let filesize = u32::try_from(filesize).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{name}: {filesize} bytes, over the 4 GiB a newc entry can hold"),
            )
        })?;
        let namesize = name.len() as u32 + 1;
        // Thirteen 8-digit hex fields after the magic, then the name: ino,
        // mode, uid, gid, nlink, mtime, filesize, dev major and minor, rdev
        // major and minor, namesize, and a check field unused by 070701.
        let fields = [
            self.ino,
            mode,
            attrs.uid,
            attrs.gid,
            nlink,
            attrs.mtime,
            filesize,
            0,
            0,
            rdev.0,
            rdev.1,
            namesize,
            0,
        ];
        let mut header = String::with_capacity(110 + name.len() + 4);
        header.push_str("070701");
        for field in fields {
            header.push_str(&format!("{field:08X}"));
        }
        header.push_str(name);
        header.push('\0');
        self.out.write_all(header.as_bytes())?;
        // The name is padded so the data starts on a 4-byte boundary.
        self.pad(110 + namesize as u64)
    }

    fn pad(&mut self, written: u64) -> io::Result<()> {
        let pad = (4 - (written % 4)) % 4;
        self.out.write_all(&[0u8; 4][..pad as usize])
    }
}

/// Convert a tar stream to a CPIO archive, fixing the `/etc` files
/// `docker export` empties (see [`FIXED_ETC`]).
pub fn from_tar(tar: impl Read, out: impl Write) -> Result<Stats, String> {
    let mut archive = tar::Archive::new(tar);
    let mut w = Writer::new(out).map_err(|e| e.to_string())?;
    let mut has_etc = false;
    for entry in archive.entries().map_err(|e| format!("reading tar: {e}"))? {
        let mut entry = entry.map_err(|e| format!("reading tar: {e}"))?;
        let Some(name) = normalize(&entry.path().map_err(|e| e.to_string())?.to_string_lossy())
        else {
            continue;
        };
        if FIXED_ETC.iter().any(|(fixed, _)| *fixed == name) {
            continue;
        }
        if name == "etc" {
            has_etc = true;
        }
        let header = entry.header();
        let attrs = Attrs {
            mode: header.mode().unwrap_or(0o644),
            uid: header.uid().unwrap_or(0) as u32,
            gid: header.gid().unwrap_or(0) as u32,
            mtime: header.mtime().unwrap_or(0) as u32,
        };
        let kind = header.entry_type();
        let link = entry
            .link_name()
            .ok()
            .flatten()
            .map(|p| p.to_string_lossy().into_owned());
        let result = match kind {
            tar::EntryType::Directory => w.dir(&name, attrs),
            tar::EntryType::Regular | tar::EntryType::Continuous => {
                let size = header.size().unwrap_or(0);
                w.file(&name, attrs, size, &mut entry)
            }
            tar::EntryType::Symlink => w.symlink(&name, link.as_deref().unwrap_or(""), attrs),
            // A hard link becomes a symlink to its target: the archive is
            // streamed, so the target's data is gone by the time the link
            // arrives, and a read-only rootfs cannot tell the two apart.
            tar::EntryType::Link => {
                let target = link.as_deref().and_then(normalize).unwrap_or_default();
                w.symlink(&name, &format!("/{target}"), attrs)
            }
            tar::EntryType::Char | tar::EntryType::Block | tar::EntryType::Fifo => {
                let kind_bits = match kind {
                    tar::EntryType::Char => S_IFCHR,
                    tar::EntryType::Block => S_IFBLK,
                    _ => S_IFIFO,
                };
                let rdev = (
                    header.device_major().ok().flatten().unwrap_or(0),
                    header.device_minor().ok().flatten().unwrap_or(0),
                );
                w.special(&name, kind_bits, attrs, rdev)
            }
            // Extended headers are consumed by the tar reader; anything
            // else (a sparse file, a GNU volume header) has no place in a
            // rootfs.
            _ => Ok(()),
        };
        result.map_err(|e| format!("writing {name}: {e}"))?;
    }
    if !has_etc {
        w.dir("etc", Attrs::root(0o755))
            .map_err(|e| e.to_string())?;
    }
    for (name, contents) in FIXED_ETC {
        w.file(
            name,
            Attrs::root(0o644),
            contents.len() as u64,
            &mut contents.as_bytes(),
        )
        .map_err(|e| format!("writing {name}: {e}"))?;
    }
    let (_, stats) = w.finish().map_err(|e| e.to_string())?;
    Ok(stats)
}

/// A tar path as a cpio name: no leading `./` or `/`, no trailing `/`;
/// `None` for the root itself.
fn normalize(path: &str) -> Option<String> {
    let mut name = path;
    loop {
        if let Some(rest) = name.strip_prefix("./") {
            name = rest;
        } else if let Some(rest) = name.strip_prefix('/') {
            name = rest;
        } else {
            break;
        }
    }
    let name = name.trim_end_matches('/');
    if name.is_empty() || name == "." {
        None
    } else {
        Some(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Read a newc archive back as (name, mode, data) triples.
    fn read_cpio(bytes: &[u8]) -> Vec<(String, u32, Vec<u8>)> {
        let mut out = Vec::new();
        let mut pos = 0;
        loop {
            let header = std::str::from_utf8(&bytes[pos..pos + 110]).unwrap();
            assert_eq!(&header[..6], "070701");
            let field = |i: usize| u32::from_str_radix(&header[6 + i * 8..14 + i * 8], 16).unwrap();
            let (mode, filesize, namesize) = (field(1), field(6) as usize, field(11) as usize);
            let name_start = pos + 110;
            let name = std::str::from_utf8(&bytes[name_start..name_start + namesize - 1])
                .unwrap()
                .to_string();
            let data_start = (name_start + namesize).next_multiple_of(4);
            let data = bytes[data_start..data_start + filesize].to_vec();
            pos = (data_start + filesize).next_multiple_of(4);
            if name == "TRAILER!!!" {
                break;
            }
            out.push((name, mode, data));
        }
        out
    }

    fn tar_with(entries: &[(&str, tar::EntryType, &[u8], Option<&str>)]) -> Vec<u8> {
        let mut b = tar::Builder::new(Vec::new());
        for (name, kind, data, link) in entries {
            let mut h = tar::Header::new_gnu();
            h.set_entry_type(*kind);
            h.set_mode(if kind.is_dir() { 0o755 } else { 0o644 });
            h.set_uid(0);
            h.set_gid(0);
            h.set_mtime(1_700_000_000);
            h.set_size(data.len() as u64);
            if let Some(link) = link {
                b.append_link(&mut h, name, link).unwrap();
            } else {
                b.append_data(&mut h, name, *data).unwrap();
            }
        }
        b.into_inner().unwrap()
    }

    #[test]
    fn lays_out_like_find_cpio() {
        let tar = tar_with(&[
            ("./etc/", tar::EntryType::Directory, b"", None),
            ("./etc/hosts", tar::EntryType::Regular, b"", None),
            ("usr/", tar::EntryType::Directory, b"", None),
            ("usr/local/", tar::EntryType::Directory, b"", None),
            ("usr/local/bin/", tar::EntryType::Directory, b"", None),
            (
                "usr/local/bin/hl_pydriver",
                tar::EntryType::Regular,
                b"ELF\x7f",
                None,
            ),
            ("bin/sh", tar::EntryType::Symlink, b"", Some("busybox")),
            ("bin/ash", tar::EntryType::Link, b"", Some("bin/busybox")),
        ]);
        let mut out = Vec::new();
        let stats = from_tar(&tar[..], &mut out).unwrap();
        let entries = read_cpio(&out);
        let names: Vec<&str> = entries.iter().map(|(n, _, _)| n.as_str()).collect();
        assert_eq!(
            names,
            [
                ".",
                "etc",
                "usr",
                "usr/local",
                "usr/local/bin",
                "usr/local/bin/hl_pydriver",
                "bin/sh",
                "bin/ash",
                "etc/hosts",
                "etc/nsswitch.conf",
                "etc/resolv.conf",
            ]
        );
        let driver = &entries[5];
        assert_eq!(driver.1, S_IFREG | 0o644);
        assert_eq!(driver.2, b"ELF\x7f");
        let sh = &entries[6];
        assert_eq!(sh.1, S_IFLNK | 0o777);
        assert_eq!(sh.2, b"busybox");
        assert_eq!(
            entries[7].2, b"/bin/busybox",
            "a hard link becomes an absolute symlink"
        );
        assert!(
            std::str::from_utf8(&entries[8].2)
                .unwrap()
                .contains("unikraft")
        );
        assert!(
            std::str::from_utf8(&entries[10].2)
                .unwrap()
                .contains("single-request")
        );
        assert_eq!(stats.entries, 10);
        // Every header is 4-byte aligned, which the reader above asserts by
        // parsing; the archive itself ends aligned too.
        assert_eq!(out.len() % 4, 0);
    }

    #[test]
    fn adds_etc_when_the_image_has_none() {
        let tar = tar_with(&[("app", tar::EntryType::Regular, b"x", None)]);
        let mut out = Vec::new();
        from_tar(&tar[..], &mut out).unwrap();
        let names: Vec<String> = read_cpio(&out).into_iter().map(|(n, _, _)| n).collect();
        assert_eq!(names[..3], [".", "app", "etc"]);
        assert!(names.contains(&"etc/resolv.conf".to_string()));
    }

    #[test]
    fn the_library_finds_the_driver_in_the_result() {
        // The same detection `SandboxBuilder::boot` runs on an initrd.
        let tar = tar_with(&[
            ("usr/", tar::EntryType::Directory, b"", None),
            ("usr/local/", tar::EntryType::Directory, b"", None),
            ("usr/local/bin/", tar::EntryType::Directory, b"", None),
            (
                "usr/local/bin/hl_nodedriver",
                tar::EntryType::Regular,
                b"ELF",
                None,
            ),
        ]);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("rootfs.cpio");
        let mut file = std::fs::File::create(&path).unwrap();
        from_tar(&tar[..], &mut file).unwrap();
        drop(file);
        // Three bytes of ELF cannot run, but the kernel names the file it
        // was told to run and that it opened it: the host's detection read
        // the entry out of our archive, and the kernel found it in the
        // unpacked initrd.  An archive the host could not read would boot
        // nothing and say so differently.
        let sandbox = hyperlight_unikraft::SandboxBuilder::from_initrd(&path)
            .boot()
            .unwrap();
        let output = sandbox.drain_output();
        assert!(
            output.contains("/usr/local/bin/hl_nodedriver: Image format not recognized"),
            "the kernel did not try our driver: {output}"
        );
    }

    #[test]
    fn normalizes_names() {
        assert_eq!(normalize("./a/b/"), Some("a/b".into()));
        assert_eq!(normalize("/a"), Some("a".into()));
        assert_eq!(normalize("./"), None);
        assert_eq!(normalize("."), None);
        assert_eq!(normalize("a"), Some("a".into()));
    }
}
