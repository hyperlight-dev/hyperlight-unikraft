// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Host filesystem — `fs_*` host functions backed by [`cap_std::fs::Dir`].
//!
//! Every mount is a capability: a `Dir` opened once on the host path,
//! after which guest paths are resolved *beneath* it by cap-std (no
//! `..` escapes, no absolute symlink targets) on every platform.
//!
//! ## Mount indices
//!
//! Every host function takes `mount_idx: i32` as its first parameter,
//! selecting which mount point to operate on. Out-of-range indices
//! return `-EINVAL`.
//!
//! ## Read-only mounts
//!
//! Mounts may be marked read-only. Write operations (`fs_write_bytes`,
//! `fs_mkdir`, `fs_unlink`, `fs_truncate`, `fs_rename`, `fs_symlink`,
//! `fs_link`, `fs_chmod`) return `-EROFS` on read-only mounts. Read
//! operations (`fs_stat`, `fs_read_bytes`, `fs_list`, `fs_readlink`)
//! work regardless.
//!
//! ## Return conventions
//!
//! - **`i32` returns**: 0 on success, `-errno` on error.
//! - **`Vec<u8>` returns**: first 4 bytes are `i32` status (0 or `-errno`),
//!   followed by operation-specific data on success.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;

use cap_std::ambient_authority;
use cap_std::fs::{Dir, OpenOptions};
use hyperlight_host::func::Registerable;
use tracing::{debug, trace};

use crate::errno;

/// Maximum bytes per read/write host call.  The guest queries this
/// value via `GetHostFsChunkSize` at mount time — changing it here
/// does not require a kernel rebuild.
pub(crate) const CHUNK: usize = 32768;

/// Register all `fs_*` host functions.
///
/// Opens a [`Dir`] for each mount point in `mounts`. Each host function
/// takes `mount_idx` as its first parameter to select the mount.
/// The `bool` in each tuple indicates whether the mount is read-only.
pub(crate) fn register(
    target: &mut impl Registerable,
    mounts: &[(String, PathBuf, bool)],
) -> hyperlight_host::Result<()> {
    if mounts.is_empty() {
        return Ok(());
    }

    let mut dirs_vec = Vec::with_capacity(mounts.len());
    let mut ro_vec = Vec::with_capacity(mounts.len());
    for (i, (guest_path, host_path, readonly)) in mounts.iter().enumerate() {
        let d = Dir::open_ambient_dir(host_path, ambient_authority()).map_err(|e| {
            hyperlight_host::HyperlightError::Error(format!(
                "hostfs: failed to open mount {i} ({} -> {}): {e}",
                host_path.display(),
                guest_path,
            ))
        })?;
        let ro_str = if *readonly { "ro" } else { "rw" };
        debug!(
            idx = i,
            host = %host_path.display(),
            guest = %guest_path,
            mode = ro_str,
            "hostfs: opened mount",
        );
        dirs_vec.push(Arc::new(d));
        ro_vec.push(*readonly);
    }
    let dirs: Arc<Vec<Arc<Dir>>> = Arc::new(dirs_vec);
    let ro_flags: Arc<Vec<bool>> = Arc::new(ro_vec);

    // ── fs_stat ─────────────────────────────────────────────────
    //
    // Returns Vec<u8>:
    //   [0..4]   i32  status (0 or -errno)
    //   [4..12]  u64  size
    //   [12..16] u32  mode (synthetic)
    //   [16]     u8   is_dir
    //   [17]     u8   is_file
    {
        let dirs = dirs.clone();
        target.register_host_function(
            "fs_stat",
            move |mount_idx: i32, path: String| -> hyperlight_host::Result<Vec<u8>> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok({ -errno::EINVAL }.to_le_bytes().to_vec());
                };
                // Empty path = stat the mount root itself.
                let meta = if path.is_empty() {
                    d.dir_metadata()
                } else {
                    d.metadata(&path)
                };
                Ok(match meta {
                    Ok(m) => {
                        let mut buf = Vec::with_capacity(18);
                        buf.extend(0i32.to_le_bytes());
                        buf.extend(m.len().to_le_bytes());
                        buf.extend(synth_mode(&m).to_le_bytes());
                        buf.push(m.is_dir() as u8);
                        buf.push(m.is_file() as u8);
                        buf
                    }
                    Err(e) => errno_vec(e),
                })
            },
        )?;
    }

    // ── fs_read_bytes ───────────────────────────────────────────
    //
    // Returns Vec<u8>:
    //   [0..4]  i32   status (0 or -errno)
    //   [4..]   bytes data (length = returned_len - 4)
    //
    // EOF is implicit: data shorter than requested → at end.
    {
        let dirs = dirs.clone();
        target.register_host_function(
            "fs_read_bytes",
            move |mount_idx: i32,
                  path: String,
                  offset: u64,
                  len: u64|
                  -> hyperlight_host::Result<Vec<u8>> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok({ -errno::EINVAL }.to_le_bytes().to_vec());
                };
                let len = (len.min(CHUNK as u64) as usize).max(1);

                Ok(match d.open(&path) {
                    Ok(mut file) => {
                        if offset > 0
                            && let Err(e) = file.seek(SeekFrom::Start(offset))
                        {
                            return Ok(errno_vec(e));
                        }
                        let mut buf = vec![0u8; 4 + len];
                        match file.read(&mut buf[4..]) {
                            Ok(n) => {
                                buf[..4].copy_from_slice(&0i32.to_le_bytes());
                                buf.truncate(4 + n);
                                buf
                            }
                            Err(e) => errno_vec(e),
                        }
                    }
                    Err(e) => errno_vec(e),
                })
            },
        )?;
    }

    // ── fs_write_bytes ──────────────────────────────────────────
    //
    // `append`: 0 = write at offset, nonzero = append (O_APPEND).
    // Returns i32: 0 or -errno.
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_write_bytes",
            move |mount_idx: i32,
                  path: String,
                  offset: u64,
                  append: i32,
                  data: Vec<u8>|
                  -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                let result = if append != 0 {
                    d.open_with(&path, OpenOptions::new().append(true).create(true))
                        .and_then(|mut f| f.write_all(&data))
                } else {
                    d.open_with(&path, OpenOptions::new().write(true).create(true))
                        .and_then(|mut f| {
                            if offset > 0 {
                                f.seek(SeekFrom::Start(offset))?;
                            }
                            f.write_all(&data)
                        })
                };
                Ok(match result {
                    Ok(()) => 0,
                    Err(e) => neg_errno(e),
                })
            },
        )?;
    }

    // ── fs_mkdir ────────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_mkdir",
            move |mount_idx: i32, path: String| -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                Ok(match d.create_dir(&path) {
                    Ok(()) => 0,
                    Err(e) => neg_errno(e),
                })
            },
        )?;
    }

    // ── fs_unlink ───────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_unlink",
            move |mount_idx: i32, path: String| -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                // Try file first, then directory.
                Ok(match d.remove_file(&path) {
                    Ok(()) => 0,
                    Err(_) => match d.remove_dir(&path) {
                        Ok(()) => 0,
                        Err(e) => neg_errno(e),
                    },
                })
            },
        )?;
    }

    // ── fs_truncate ─────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_truncate",
            move |mount_idx: i32, path: String, length: u64| -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                Ok(match d.open_with(&path, OpenOptions::new().write(true)) {
                    Ok(f) => match f.set_len(length) {
                        Ok(()) => 0,
                        Err(e) => neg_errno(e),
                    },
                    Err(e) => neg_errno(e),
                })
            },
        )?;
    }

    // ── fs_list ─────────────────────────────────────────────────
    //
    // Returns Vec<u8>:
    //   [0..4]  i32  status (0 or -errno)
    //   [4..8]  u32  entry_count
    //   [8..]   entries, each:
    //     u8   is_dir
    //     u16  name_len (LE)
    //     [name_len bytes] name (UTF-8, no NUL)
    {
        let dirs = dirs.clone();
        target.register_host_function(
            "fs_list",
            move |mount_idx: i32, path: String| -> hyperlight_host::Result<Vec<u8>> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok({ -errno::EINVAL }.to_le_bytes().to_vec());
                };
                let path = if path.is_empty() {
                    ".".to_string()
                } else {
                    path
                };
                Ok(match d.read_dir(&path) {
                    Ok(entries) => {
                        let mut buf = Vec::with_capacity(256);
                        buf.extend(0i32.to_le_bytes()); // status
                        buf.extend(0u32.to_le_bytes()); // count placeholder
                        let mut count = 0u32;
                        for entry in entries.flatten() {
                            let is_dir = entry.metadata().map(|m| m.is_dir()).unwrap_or(false);
                            let name = entry.file_name().to_string_lossy().into_owned();
                            let name_bytes = name.as_bytes();
                            buf.push(is_dir as u8);
                            buf.extend((name_bytes.len() as u16).to_le_bytes());
                            buf.extend_from_slice(name_bytes);
                            count += 1;
                        }
                        buf[4..8].copy_from_slice(&count.to_le_bytes());
                        buf
                    }
                    Err(e) => errno_vec(e),
                })
            },
        )?;
    }

    // ── fs_rename ───────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_rename",
            move |mount_idx: i32, from: String, to: String| -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                Ok(match d.rename(&from, d, &to) {
                    Ok(()) => 0,
                    Err(e) => neg_errno(e),
                })
            },
        )?;
    }

    // ── fs_symlink ──────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_symlink",
            move |mount_idx: i32,
                  link_path: String,
                  target_path: String|
                  -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                #[cfg(unix)]
                let result = d.symlink(&target_path, &link_path);
                #[cfg(windows)]
                let result = d
                    .symlink_file(&target_path, &link_path)
                    .or_else(|_| d.symlink_dir(&target_path, &link_path));
                Ok(match result {
                    Ok(()) => 0,
                    Err(e) => neg_errno(e),
                })
            },
        )?;
    }

    // ── fs_readlink ─────────────────────────────────────────────
    //
    // Returns Vec<u8>:
    //   [0..4]  i32   status (0 or -errno)
    //   [4..]   bytes target path (UTF-8)
    {
        let dirs = dirs.clone();
        target.register_host_function(
            "fs_readlink",
            move |mount_idx: i32, path: String| -> hyperlight_host::Result<Vec<u8>> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok({ -errno::EINVAL }.to_le_bytes().to_vec());
                };
                Ok(match d.read_link(&path) {
                    Ok(target_path) => {
                        let target_bytes = target_path.to_string_lossy().as_bytes().to_vec();
                        let mut buf = Vec::with_capacity(4 + target_bytes.len());
                        buf.extend(0i32.to_le_bytes());
                        buf.extend_from_slice(&target_bytes);
                        buf
                    }
                    Err(e) => errno_vec(e),
                })
            },
        )?;
    }

    // ── fs_link ─────────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_link",
            move |mount_idx: i32, src: String, dst: String| -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                Ok(match d.hard_link(&src, d, &dst) {
                    Ok(()) => 0,
                    Err(e) => neg_errno(e),
                })
            },
        )?;
    }

    // ── fs_chmod ────────────────────────────────────────────────
    {
        let dirs = dirs.clone();
        let ro = ro_flags.clone();
        target.register_host_function(
            "fs_chmod",
            move |mount_idx: i32, path: String, mode: u32| -> hyperlight_host::Result<i32> {
                let Some(d) = dirs.get(mount_idx as usize) else {
                    return Ok(-errno::EINVAL);
                };
                if check_ro(&ro, mount_idx as usize) {
                    return Ok(-errno::EROFS);
                }
                #[cfg(unix)]
                {
                    use cap_std::fs::PermissionsExt;
                    let perms = cap_std::fs::Permissions::from_mode(mode);
                    Ok(match d.set_permissions(&path, perms) {
                        Ok(()) => 0,
                        Err(e) => neg_errno(e),
                    })
                }
                #[cfg(windows)]
                {
                    // On Windows only the readonly bit matters.
                    let readonly = mode & 0o222 == 0;
                    let mut perms = match d.metadata(&path) {
                        Ok(m) => m.permissions(),
                        Err(e) => return Ok(neg_errno(e)),
                    };
                    perms.set_readonly(readonly);
                    Ok(match d.set_permissions(&path, perms) {
                        Ok(()) => 0,
                        Err(e) => neg_errno(e),
                    })
                }
            },
        )?;
    }

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────

/// Check if a mount is read-only.
fn check_ro(ro_flags: &[bool], idx: usize) -> bool {
    ro_flags.get(idx).copied().unwrap_or(false)
}

/// Synthesize a POSIX-ish mode from cap_std metadata.
fn synth_mode(m: &cap_std::fs::Metadata) -> u32 {
    let kind = if m.is_dir() {
        0o40000u32
    } else if m.is_symlink() {
        0o120000u32
    } else {
        0o100000u32
    };
    let perm = if m.permissions().readonly() {
        0o555u32
    } else {
        0o755u32
    };
    kind | perm
}

/// Encode an I/O error as a 4-byte `-errno` Vec.
fn errno_vec(e: std::io::Error) -> Vec<u8> {
    neg_errno(e).to_le_bytes().to_vec()
}

/// Convert an I/O error to `-errno` as i32.
fn neg_errno(e: std::io::Error) -> i32 {
    let code = errno::from_io(&e);
    trace!(errno = code, err = %e, "hostfs: operation failed");
    -code
}
