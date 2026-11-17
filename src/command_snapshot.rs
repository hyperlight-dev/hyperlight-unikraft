// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use hyperlight_host::new_error;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{Result, command::CommandSnapshot};

pub(crate) const METADATA: &str = "hluk.commands";
// Bump when the serialized execution state changes incompatibly.
const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct StateDescriptor {
    version: u32,
    sha256: [u8; 32],
    size: u64,
}

impl StateDescriptor {
    fn path(&self, dir: &Path) -> PathBuf {
        let hex: String = self
            .sha256
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        dir.join("commands").join(format!("{hex}.json"))
    }
}

/// Persist a [`CommandSnapshot`] to `dir`, including execution metadata, pipe
/// buffers and retained replies. [`crate::command::CommandSandbox::snapshot_to`]
/// captures and saves in one step.
///
/// The guest uses the same OCI layout and [`crate::SNAPSHOT_KEY`] checks as app
/// snapshots. Command state is a versioned, checksummed companion file referenced
/// by the guest metadata; it is published before the OCI index is replaced.
/// Keep or copy the **whole directory**, including `commands/`: OCI-only export
/// tools do not preserve these companion files. Repeated saves replace the
/// current tag, retaining old blobs just as [`crate::save_snapshot`] does.
///
/// Snapshots contain guest memory, command environments and pipe data, which may
/// contain secrets. Store them in a trusted location.
pub fn save_command_snapshot(snapshot: &CommandSnapshot, dir: impl AsRef<Path>) -> Result<()> {
    snapshot.executions.validate()?;
    let bytes = serde_json::to_vec(&snapshot.executions)
        .map_err(|error| new_error!("serialize command snapshot state: {}", error))?;
    let descriptor = StateDescriptor {
        version: VERSION,
        sha256: Sha256::digest(&bytes).into(),
        size: bytes.len() as u64,
    };
    let guest = snapshot.guest.with_metadata(METADATA, &descriptor)?;
    let dir = dir.as_ref();
    let state_dir = dir.join("commands");
    // Unlike the bounded OCI metadata, pipe data can be arbitrarily large.
    fs::create_dir_all(&state_dir)
        .map_err(|error| new_error!("create command state directory {:?}: {}", state_dir, error))?;
    let mut temp = tempfile::NamedTempFile::new_in(&state_dir)
        .map_err(|error| new_error!("create command state file: {}", error))?;
    temp.write_all(&bytes)
        .and_then(|()| temp.as_file().sync_all())
        .map_err(|error| new_error!("write command state file: {}", error))?;
    temp.persist(descriptor.path(dir))
        .map_err(|error| new_error!("publish command state file: {}", error))?;
    crate::save_snapshot(&guest, dir)
}

/// Load a snapshot written by [`save_command_snapshot`], for in-place
/// [`crate::command::CommandSandbox::restore`] or independent
/// [`crate::SandboxBuilder::from_command_snapshot`] restores.
///
/// App snapshots, incompatible command formats and missing or corrupt command
/// state are rejected. Host functions, mounts and network policy are not saved.
/// Every restored command starts parked, just as with an in-memory snapshot.
///
/// Load only trusted directories. Guest blobs remain memory-mapped: do not
/// overwrite, truncate or remove them while a loaded snapshot or sandbox is alive.
/// The command-state checksum detects corruption, not authenticity.
pub fn load_command_snapshot(dir: impl AsRef<Path>) -> Result<CommandSnapshot> {
    let dir = dir.as_ref();
    let guest = crate::load_snapshot_image(dir)?;
    let descriptor = guest
        .metadata::<StateDescriptor>(METADATA)?
        .ok_or_else(|| new_error!("not a command snapshot: command state metadata is missing"))?;
    if descriptor.version != VERSION {
        return Err(new_error!(
            "unsupported command snapshot version {} (expected {})",
            descriptor.version,
            VERSION
        )
        .into());
    }
    let path = descriptor.path(dir);
    let file = File::open(&path)
        .map_err(|error| new_error!("open command snapshot state {:?}: {}", path, error))?;
    let mut bytes = Vec::new();
    file.take(descriptor.size.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| new_error!("read command snapshot state {:?}: {}", path, error))?;
    let digest: [u8; 32] = Sha256::digest(&bytes).into();
    if bytes.len() as u64 != descriptor.size || digest != descriptor.sha256 {
        return Err(new_error!("command snapshot state size or checksum mismatch").into());
    }
    let executions: crate::execution::ExecutionSnapshot = serde_json::from_slice(&bytes)
        .map_err(|error| new_error!("decode command snapshot state: {}", error))?;
    executions.validate()?;
    Ok(CommandSnapshot { guest, executions })
}
