// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.
mod common;

use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};

use common::require_rootfs;
use hyperlight_unikraft::{DirEntry, FileStat, FsBackend, Mount, SandboxBuilder};

/// A filesystem in host memory that holds at most `quota` bytes: what an
/// embedder that accounts for every write would serve.
struct Memory {
    quota: u64,
    /// Path → contents; `None` for a directory.  The root is implicit.
    nodes: Mutex<BTreeMap<String, Option<Vec<u8>>>>,
}

impl Memory {
    fn new(quota: u64) -> Arc<Self> {
        Arc::new(Self {
            quota,
            nodes: Mutex::default(),
        })
    }

    fn used(nodes: &BTreeMap<String, Option<Vec<u8>>>) -> u64 {
        nodes.values().flatten().map(|d| d.len() as u64).sum()
    }

    fn file(&self, path: &str) -> io::Result<Vec<u8>> {
        match self.nodes.lock().unwrap().get(path) {
            Some(Some(data)) => Ok(data.clone()),
            Some(None) => Err(io::ErrorKind::IsADirectory.into()),
            None => Err(io::ErrorKind::NotFound.into()),
        }
    }
}

impl FsBackend for Memory {
    fn stat(&self, path: &str) -> io::Result<FileStat> {
        if path.is_empty() {
            return Ok(FileStat::dir(0o755));
        }
        match self.nodes.lock().unwrap().get(path) {
            Some(Some(data)) => Ok(FileStat::file(data.len() as u64, 0o644)),
            Some(None) => Ok(FileStat::dir(0o755)),
            None => Err(io::ErrorKind::NotFound.into()),
        }
    }

    fn read(&self, path: &str, offset: u64, buf: &mut [u8]) -> io::Result<usize> {
        let data = self.file(path)?;
        let start = (offset as usize).min(data.len());
        let n = buf.len().min(data.len() - start);
        buf[..n].copy_from_slice(&data[start..start + n]);
        Ok(n)
    }

    fn write(&self, path: &str, offset: Option<u64>, data: &[u8]) -> io::Result<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let mut file = match nodes.get(path) {
            Some(Some(file)) => file.clone(),
            Some(None) => return Err(io::ErrorKind::IsADirectory.into()),
            None => Vec::new(),
        };
        // The offset is the guest's: check it against the quota before
        // allocating anything for it.
        let at = offset.unwrap_or(file.len() as u64);
        let end = at
            .checked_add(data.len() as u64)
            .ok_or(io::ErrorKind::FileTooLarge)?;
        let size = (file.len() as u64).max(end);
        if Self::used(&nodes) - file.len() as u64 + size > self.quota {
            return Err(io::ErrorKind::QuotaExceeded.into());
        }
        let (at, end) = (at as usize, end as usize);
        if file.len() < end {
            file.resize(end, 0);
        }
        file[at..end].copy_from_slice(data);
        nodes.insert(path.to_string(), Some(file));
        Ok(())
    }

    fn create_dir(&self, path: &str) -> io::Result<()> {
        let mut nodes = self.nodes.lock().unwrap();
        if nodes.contains_key(path) {
            return Err(io::ErrorKind::AlreadyExists.into());
        }
        nodes.insert(path.to_string(), None);
        Ok(())
    }

    fn remove(&self, path: &str) -> io::Result<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let prefix = format!("{path}/");
        if nodes.keys().any(|k| k.starts_with(&prefix)) {
            return Err(io::ErrorKind::DirectoryNotEmpty.into());
        }
        nodes
            .remove(path)
            .map(drop)
            .ok_or_else(|| io::ErrorKind::NotFound.into())
    }

    fn truncate(&self, path: &str, len: u64) -> io::Result<()> {
        let mut file = self.file(path)?;
        let used = Self::used(&self.nodes.lock().unwrap());
        if (used - file.len() as u64).saturating_add(len) > self.quota {
            return Err(io::ErrorKind::QuotaExceeded.into());
        }
        file.resize(len as usize, 0);
        self.nodes
            .lock()
            .unwrap()
            .insert(path.to_string(), Some(file));
        Ok(())
    }

    fn list(&self, path: &str) -> io::Result<Vec<DirEntry>> {
        let prefix = if path.is_empty() {
            String::new()
        } else {
            format!("{path}/")
        };
        Ok(self
            .nodes
            .lock()
            .unwrap()
            .iter()
            .filter_map(|(k, v)| {
                let name = k.strip_prefix(&prefix)?;
                (!name.contains('/')).then(|| DirEntry::new(name, v.is_none()))
            })
            .collect())
    }

    fn rename(&self, from: &str, to: &str) -> io::Result<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let node = nodes.remove(from).ok_or(io::ErrorKind::NotFound)?;
        nodes.insert(to.to_string(), node);
        Ok(())
    }
}

/// Python's file operations on a mount reach the backend, which serves
/// them from host memory: files and directories made, listed, renamed,
/// read back and removed.  A write past the backend's quota fails in the
/// guest with the errno it gave, `EDQUOT`, and the backend keeps nothing
/// of it; links, which the backend does not offer, are refused.
#[tokio::test]
async fn python_on_a_backend_mount() {
    let rootfs = require_rootfs("python");
    let memory = Memory::new(64 * 1024);
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mount(Mount::backend(memory.clone(), "/mnt/mem", false))
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno, os\n",
            "os.mkdir('/mnt/mem/d')\n",
            "with open('/mnt/mem/d/a.txt', 'w') as f:\n",
            "    f.write('hello ' * 10000)\n",
            "os.rename('/mnt/mem/d/a.txt', '/mnt/mem/d/b.txt')\n",
            "print('listed', sorted(os.listdir('/mnt/mem/d')))\n",
            "print('size', os.path.getsize('/mnt/mem/d/b.txt'))\n",
            "print('read', open('/mnt/mem/d/b.txt').read(12))\n",
            "try:\n",
            "    with open('/mnt/mem/big.bin', 'wb') as f:\n",
            "        f.write(b'x' * 32768)\n",
            "    print('quota not enforced')\n",
            "except OSError as e:\n",
            "    print('quota', errno.errorcode.get(e.errno, e.errno))\n",
            "try:\n",
            "    with open('/mnt/mem/far.bin', 'wb') as f:\n",
            "        f.seek(2 ** 62)\n",
            "        f.write(b'x')\n",
            "    print('far write made')\n",
            "except OSError as e:\n",
            "    print('far', errno.errorcode.get(e.errno, e.errno))\n",
            "try:\n",
            "    os.symlink('d/b.txt', '/mnt/mem/link')\n",
            "    print('link made')\n",
            "except OSError:\n",
            "    print('link refused')\n",
            "os.remove('/mnt/mem/d/b.txt')\n",
            "print('after remove', os.listdir('/mnt/mem/d'))\n",
            "with open('/mnt/mem/kept.txt', 'w') as f:\n",
            "    f.write('kept')\n",
        ))
        .await
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("listed ['b.txt']"), "{out:?}");
    assert!(out.contains("size 60000"), "{out:?}");
    assert!(out.contains("read hello hello"), "{out:?}");
    assert!(out.contains("quota EDQUOT"), "{out:?}");
    assert!(out.contains("far EDQUOT"), "{out:?}");
    assert!(out.contains("link refused"), "{out:?}");
    assert!(out.contains("after remove []"), "{out:?}");
    assert_eq!(memory.file("kept.txt").unwrap(), b"kept");
    assert!(memory.used_now() <= 64 * 1024);
}

/// A read-only backend mount is never asked to write: the guest is told
/// `EROFS` first.
#[tokio::test]
async fn read_only_backend_mount_is_not_asked_to_write() {
    let rootfs = require_rootfs("python");
    let memory = Memory::new(1024);
    memory.write("given.txt", None, b"from the host").unwrap();
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mount(Mount::backend(memory.clone(), "/mnt/mem", true))
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno\n",
            "print('read', open('/mnt/mem/given.txt').read())\n",
            "try:\n",
            "    open('/mnt/mem/new.txt', 'w').write('x')\n",
            "    print('written')\n",
            "except OSError as e:\n",
            "    print('write', errno.errorcode.get(e.errno, e.errno))\n",
        ))
        .await
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("read from the host"), "{out:?}");
    assert!(out.contains("write EROFS"), "{out:?}");
    assert!(memory.file("new.txt").is_err());
}

/// A backend that panics on one path and lists names no directory has.
struct Unruly;

impl FsBackend for Unruly {
    fn stat(&self, path: &str) -> io::Result<FileStat> {
        match path {
            "" => Ok(FileStat::dir(0o755)),
            "boom" => panic!("the backend's own bug"),
            _ => Ok(FileStat::file(0, 0o644)),
        }
    }
    fn read(&self, _: &str, _: u64, _: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
    fn write(&self, _: &str, _: Option<u64>, _: &[u8]) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(0))
    }
    fn create_dir(&self, _: &str) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(0))
    }
    fn remove(&self, _: &str) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(0))
    }
    fn truncate(&self, _: &str, _: u64) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(0))
    }
    fn list(&self, _: &str) -> io::Result<Vec<DirEntry>> {
        Ok([
            "ok",
            "a/b",
            "",
            ".",
            "..",
            "nul\0",
            &"x".repeat(256),
            "last",
        ]
        .into_iter()
        .map(|name| DirEntry::new(name, false))
        .collect())
    }
    fn rename(&self, _: &str, _: &str) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(0))
    }
}

/// What a backend gets wrong stays with the guest's call: a panic is
/// `EIO`, a "raw OS error" of 0 is no success, and names the kernel could
/// not take are left out of a listing.
#[tokio::test]
async fn a_backends_mistakes_stay_with_the_call() {
    let rootfs = require_rootfs("python");
    let mut sandbox = SandboxBuilder::from_initrd(rootfs)
        .scratch_mb(256)
        .mount(Mount::backend(Arc::new(Unruly), "/mnt/u", false))
        .boot()
        .unwrap();
    sandbox
        .run(concat!(
            "import errno, os\n",
            "for op in (lambda: os.stat('/mnt/u/boom'),\n",
            "           lambda: open('/mnt/u/f', 'wb').write(b'x')):\n",
            "    try:\n",
            "        op()\n",
            "        print('succeeded')\n",
            "    except OSError as e:\n",
            "        print(errno.errorcode.get(e.errno, e.errno))\n",
            "print(sorted(os.listdir('/mnt/u')))\n",
        ))
        .await
        .unwrap();
    let out = sandbox.drain_output();
    assert!(out.contains("EIO\nEIO\n['last', 'ok']"), "{out:?}");
}

impl Memory {
    fn used_now(&self) -> u64 {
        Self::used(&self.nodes.lock().unwrap())
    }
}
