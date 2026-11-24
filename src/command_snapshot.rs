// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use hyperlight_host::new_error;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::{Result, Snapshot, command::CommandSnapshot};

pub(crate) const METADATA: &str = "hluk.commands";
// Bump when the serialized execution state changes incompatibly.
const VERSION: u32 = 7;
const STATE_MEDIA: &str = "application/vnd.hyperlight.unikraft.command-state.v1+json";
const GUEST_MEDIA: &str = "application/vnd.hyperlight.unikraft.guest-manifest.v1+json";
const MANIFEST_MEDIA: &str = "application/vnd.oci.image.manifest.v1+json";
const TAG: &str = "org.opencontainers.image.ref.name";
const JSON_LIMIT: usize = 1024 * 1024;

#[derive(Serialize, Deserialize)]
struct State<T> {
    version: u32,
    executions: T,
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn read_bounded(path: &Path, limit: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)
        .and_then(|file| {
            file.take((limit as u64).saturating_add(1))
                .read_to_end(&mut bytes)
        })
        .map_err(|error| new_error!("read snapshot file {:?}: {}", path, error))?;
    check_size(&bytes, limit)?;
    Ok(bytes)
}

fn check_size(bytes: &[u8], limit: usize) -> Result<()> {
    if bytes.len() > limit {
        return Err(new_error!(
            "snapshot artifact of {} bytes exceeds the {} byte limit",
            bytes.len(),
            limit
        )
        .into());
    }
    Ok(())
}

fn json_bytes(value: &impl Serialize, limit: Option<usize>) -> Result<Vec<u8>> {
    let bytes =
        serde_json::to_vec(value).map_err(|error| new_error!("encode snapshot JSON: {}", error))?;
    if let Some(limit) = limit {
        check_size(&bytes, limit)?;
    }
    Ok(bytes)
}

fn parse_json(bytes: &[u8]) -> Result<Value> {
    serde_json::from_slice(bytes)
        .map_err(|error| new_error!("decode snapshot JSON: {}", error).into())
}

fn blob_path(dir: &Path, descriptor: &Value) -> Result<PathBuf> {
    let hex = descriptor["digest"]
        .as_str()
        .and_then(|s| s.strip_prefix("sha256:"))
        .filter(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        })
        .ok_or_else(|| new_error!("invalid snapshot blob digest"))?;
    Ok(dir.join("blobs/sha256").join(hex))
}

fn read_blob(dir: &Path, descriptor: &Value, limit: Option<usize>) -> Result<Vec<u8>> {
    let size = descriptor["size"]
        .as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .filter(|n| limit.is_none_or(|limit| *n <= limit))
        .ok_or_else(|| new_error!("invalid or oversized snapshot blob descriptor"))?;
    let bytes = read_bounded(&blob_path(dir, descriptor)?, size)?;
    if bytes.len() != size || descriptor["digest"] != format!("sha256:{}", digest(&bytes)) {
        return Err(new_error!("snapshot blob size or checksum mismatch").into());
    }
    Ok(bytes)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| new_error!("snapshot path has no parent"))?;
    let mut file = tempfile::NamedTempFile::new_in(parent)
        .map_err(|error| new_error!("create snapshot file {:?}: {}", path, error))?;
    file.write_all(bytes)
        .and_then(|()| file.as_file().sync_all())
        .map_err(|error| new_error!("write snapshot file {:?}: {}", path, error))?;
    file.persist(path)
        .map_err(|error| new_error!("publish snapshot file {:?}: {}", path, error))?;
    Ok(())
}

fn put_blob(dir: &Path, bytes: &[u8], media: &str) -> Result<Value> {
    let descriptor = json!({"mediaType": media, "digest": format!("sha256:{}", digest(bytes)), "size": bytes.len()});
    let path = blob_path(dir, &descriptor)?;
    if path
        .try_exists()
        .map_err(|error| new_error!("stat snapshot blob {:?}: {}", path, error))?
    {
        read_blob(dir, &descriptor, Some(bytes.len()))?;
    } else {
        let mut file = tempfile::NamedTempFile::new_in(path.parent().unwrap())
            .map_err(|error| new_error!("create snapshot blob: {}", error))?;
        file.write_all(bytes)
            .and_then(|()| file.as_file().sync_all())
            .map_err(|error| new_error!("write snapshot blob: {}", error))?;
        match file.persist_noclobber(&path) {
            Ok(_) => {}
            Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
                read_blob(dir, &descriptor, Some(bytes.len()))?;
            }
            Err(error) => return Err(new_error!("publish snapshot blob: {}", error).into()),
        }
    }
    Ok(descriptor)
}

fn read_index(dir: &Path) -> Result<Value> {
    let marker = parse_json(&read_bounded(&dir.join("oci-layout"), JSON_LIMIT)?)?;
    if marker["imageLayoutVersion"] != "1.0.0" {
        return Err(new_error!("unsupported OCI layout version").into());
    }
    let index = parse_json(&read_bounded(&dir.join("index.json"), JSON_LIMIT)?)?;
    if index["schemaVersion"] != 2 || !index["manifests"].is_array() {
        return Err(new_error!("invalid snapshot OCI index").into());
    }
    Ok(index)
}

fn tagged(index: &Value, tag: &str) -> Result<usize> {
    let manifests = index["manifests"]
        .as_array()
        .ok_or_else(|| new_error!("invalid snapshot index manifests"))?;
    let mut matching = manifests
        .iter()
        .enumerate()
        .filter(|(_, d)| d["annotations"][TAG] == tag);
    match (matching.next(), matching.next()) {
        (Some((i, _)), None) => Ok(i),
        _ => Err(new_error!("snapshot tag {:?} must identify exactly one manifest", tag).into()),
    }
}

fn read_manifest(dir: &Path, descriptor: &Value) -> Result<Value> {
    if descriptor["mediaType"] != MANIFEST_MEDIA {
        return Err(new_error!("unsupported snapshot manifest media type").into());
    }
    let manifest = parse_json(&read_blob(dir, descriptor, Some(JSON_LIMIT))?)?;
    if manifest["schemaVersion"] != 2 || !manifest["layers"].is_array() {
        return Err(new_error!("invalid snapshot OCI manifest").into());
    }
    Ok(manifest)
}

pub(crate) fn is_layered(dir: &Path, tag: &str) -> Result<bool> {
    let index = read_index(dir)?;
    let manifest = read_manifest(dir, &index["manifests"][tagged(&index, tag)?])?;
    Ok(manifest["layers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["mediaType"] == STATE_MEDIA))
}

// Unix views share the immutable blob directory without copying or mapping
// private files. Other platforms use hard links, copying if links are unavailable.
fn blob_view(source: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest.join("blobs"))
        .map_err(|error| new_error!("create snapshot view: {}", error))?;
    let source = fs::canonicalize(source.join("blobs/sha256"))
        .map_err(|error| new_error!("resolve snapshot blobs: {}", error))?;
    let dest = dest.join("blobs/sha256");
    #[cfg(unix)]
    std::os::unix::fs::symlink(source, dest)
        .map_err(|error| new_error!("link snapshot blob directory: {}", error))?;
    #[cfg(not(unix))]
    {
        fs::create_dir(&dest)
            .map_err(|error| new_error!("create snapshot blob view: {}", error))?;
        copy_blobs(&source, &dest)?;
    }
    Ok(())
}

#[cfg(any(not(unix), test))]
fn copy_blobs(source: &Path, dest: &Path) -> Result<()> {
    for entry in
        fs::read_dir(source).map_err(|error| new_error!("list snapshot blobs: {}", error))?
    {
        let entry = entry.map_err(|error| new_error!("read snapshot blob entry: {}", error))?;
        if !entry
            .file_type()
            .map_err(|error| new_error!("stat snapshot blob: {}", error))?
            .is_file()
        {
            return Err(new_error!("snapshot blob is not a regular file").into());
        }
        let target = dest.join(entry.file_name());
        match fs::hard_link(entry.path(), &target) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => {
                tracing::debug!(%error, "hard link unavailable; copying snapshot blob");
                let mut temp = tempfile::NamedTempFile::new_in(dest)
                    .map_err(|error| new_error!("create snapshot blob copy: {}", error))?;
                let mut source = File::open(entry.path())
                    .map_err(|error| new_error!("open snapshot blob: {}", error))?;
                std::io::copy(&mut source, &mut temp)
                    .and_then(|_| temp.as_file().sync_all())
                    .map_err(|error| new_error!("copy snapshot blob: {}", error))?;
                match temp.persist_noclobber(target) {
                    Ok(_) => {}
                    Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => {
                        return Err(new_error!("publish snapshot blob copy: {}", error).into());
                    }
                }
            }
        }
    }
    Ok(())
}

/// Save a command snapshot as an OCI image. After Hyperlight generates its
/// manifest, add command-state and original-manifest layers, then atomically
/// publish the new index. No files outside the OCI layout are required.
///
/// Command state has no fixed size cap; saving and loading it require memory
/// proportional to its size. Hyperlight's 1 MiB config and manifest limits remain.
/// Stdin and retained replies reside in guest memory; host output is excluded.
/// Load only trusted snapshots: digests detect corruption, not authenticity.
pub fn save_command_snapshot(snapshot: &CommandSnapshot, dir: impl AsRef<Path>) -> Result<()> {
    snapshot.executions.validate()?;
    let state = json_bytes(
        &State {
            version: VERSION,
            executions: &snapshot.executions,
        },
        None,
    )?;
    let guest = snapshot
        .guest
        .with_metadata(METADATA, &json!({"version": VERSION}))?;
    let dir = dir.as_ref();
    fs::create_dir_all(dir).map_err(|error| new_error!("create snapshot directory: {}", error))?;
    let stage = tempfile::tempdir_in(dir)
        .map_err(|error| new_error!("create snapshot staging view: {}", error))?;
    for name in ["index.json", "oci-layout"] {
        let source = dir.join(name);
        if source
            .try_exists()
            .map_err(|error| new_error!("stat snapshot layout: {}", error))?
        {
            let bytes = read_bounded(&source, JSON_LIMIT)?;
            atomic_write(&stage.path().join(name), &bytes)?;
        }
    }
    fs::create_dir_all(dir.join("blobs/sha256"))
        .map_err(|error| new_error!("create snapshot blob directory: {}", error))?;
    blob_view(dir, stage.path())?;
    let tag = crate::snapshot_tag();
    guest.save(stage.path(), &tag)?;
    let mut index = read_index(stage.path())?;
    let selected = tagged(&index, tag.as_str())?;
    let original = index["manifests"][selected].clone();
    let mut manifest = read_manifest(stage.path(), &original)?;
    let state_desc = put_blob(stage.path(), &state, STATE_MEDIA)?;
    let mut original_layer = original;
    original_layer["mediaType"] = GUEST_MEDIA.into();
    let layers = manifest["layers"].as_array_mut().unwrap();
    layers.push(original_layer);
    layers.push(state_desc);
    let mut descriptor = put_blob(
        stage.path(),
        &json_bytes(&manifest, Some(JSON_LIMIT))?,
        MANIFEST_MEDIA,
    )?;
    descriptor["annotations"] = index["manifests"][selected]["annotations"].clone();
    index["manifests"][selected] = descriptor;
    let index_bytes = json_bytes(&index, Some(JSON_LIMIT))?;
    #[cfg(not(unix))]
    copy_blobs(
        &stage.path().join("blobs/sha256"),
        &dir.join("blobs/sha256"),
    )?;
    atomic_write(
        &dir.join("oci-layout"),
        &read_bounded(&stage.path().join("oci-layout"), JSON_LIMIT)?,
    )?;
    atomic_write(&dir.join("index.json"), &index_bytes)?;
    Ok(())
}

/// Load a command snapshot without modifying its OCI layout. A private index
/// selects the original Hyperlight manifest; the added layers are validated and
/// consumed by this wrapper. Host functions, mounts and policy must be supplied
/// again. Only load trusted layouts, and keep source blobs intact while in use.
pub fn load_command_snapshot(dir: impl AsRef<Path>) -> Result<CommandSnapshot> {
    let dir = dir.as_ref();
    let tag = crate::snapshot_reference(dir)?;
    let index = read_index(dir)?;
    let manifest = read_manifest(dir, &index["manifests"][tagged(&index, tag.as_str())?])?;
    let config = parse_json(&read_blob(dir, &manifest["config"], Some(JSON_LIMIT))?)?;
    let metadata = config["metadata"]
        .get(METADATA)
        .ok_or_else(|| new_error!("not a command snapshot: command state metadata is missing"))?;
    let version = metadata["version"]
        .as_u64()
        .ok_or_else(|| new_error!("invalid command snapshot version"))?;
    if version != u64::from(VERSION) {
        return Err(new_error!(
            "unsupported command snapshot version {} (expected {})",
            version,
            VERSION
        )
        .into());
    }
    let mut base = manifest.clone();
    let mut state = None;
    let mut original = None;
    let mut ordinary = Vec::new();
    for descriptor in manifest["layers"].as_array().unwrap() {
        let slot = if descriptor["mediaType"] == STATE_MEDIA {
            &mut state
        } else if descriptor["mediaType"] == GUEST_MEDIA {
            &mut original
        } else {
            ordinary.push(descriptor.clone());
            continue;
        };
        if slot.replace(descriptor.clone()).is_some() {
            return Err(new_error!("duplicate command snapshot extension layer").into());
        }
    }
    let state = state.ok_or_else(|| new_error!("missing command snapshot state layer"))?;
    let mut original =
        original.ok_or_else(|| new_error!("missing original Hyperlight manifest layer"))?;
    original["mediaType"] = MANIFEST_MEDIA.into();
    base["layers"] = ordinary.into();
    if read_manifest(dir, &original)? != base {
        return Err(
            new_error!("command snapshot does not match its original Hyperlight manifest").into(),
        );
    }
    let state: State<crate::execution::ExecutionSnapshot> =
        serde_json::from_slice(&read_blob(dir, &state, None)?)
            .map_err(|error| new_error!("decode command snapshot state: {}", error))?;
    if state.version != VERSION {
        return Err(new_error!(
            "unsupported command snapshot state version {}",
            state.version
        )
        .into());
    }
    state.executions.validate()?;
    let view = Arc::new(
        tempfile::tempdir()
            .map_err(|error| new_error!("create snapshot loading view: {}", error))?,
    );
    blob_view(dir, view.path())?;
    atomic_write(
        &view.path().join("oci-layout"),
        &read_bounded(&dir.join("oci-layout"), JSON_LIMIT)?,
    )?;
    original["annotations"] = json!({TAG: tag.as_str()});
    let index = json!({"schemaVersion": 2, "mediaType": "application/vnd.oci.image.index.v1+json", "manifests": [original]});
    atomic_write(
        &view.path().join("index.json"),
        &json_bytes(&index, Some(JSON_LIMIT))?,
    )?;
    let guest = Arc::new(Snapshot::load(view.path(), tag)?);
    Ok(CommandSnapshot {
        guest,
        executions: state.executions,
        files: vec![view],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builder() -> crate::SandboxBuilder {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("build-elfloader");
        crate::SandboxBuilder::from_initrd(root.join("c-rootfs.cpio"))
            .mount(crate::Mount::ro(root.join("bins/c"), "/mnt/bin"))
            .scratch_mb(128)
            .echo_output(false)
    }

    fn rewrite_manifest(dir: &Path, manifest: &Value) {
        let mut index = read_index(dir).unwrap();
        let mut desc = put_blob(
            dir,
            &json_bytes(manifest, Some(JSON_LIMIT)).unwrap(),
            MANIFEST_MEDIA,
        )
        .unwrap();
        desc["annotations"] = index["manifests"][0]["annotations"].clone();
        index["manifests"][0] = desc;
        atomic_write(
            &dir.join("index.json"),
            &json_bytes(&index, Some(JSON_LIMIT)).unwrap(),
        )
        .unwrap();
    }

    #[tokio::test]
    #[ignore = "requires a hypervisor and build-elfloader C fixtures"]
    async fn compaction_preserves_pending_input_and_loaded_lifetimes() {
        use futures::io::AsyncWriteExt;

        let sandbox = builder().boot_commands().unwrap();
        let base = sandbox.snapshot().await.unwrap();
        let base_dir = tempfile::tempdir().unwrap();
        save_command_snapshot(&base, base_dir.path()).unwrap();
        let base = load_command_snapshot(base_dir.path()).unwrap();
        let mut child = sandbox
            .command("/mnt/bin/command")
            .name("compact-echo")
            .arg("echo")
            .stdin(crate::command::Stdio::piped())
            .stdout(crate::command::Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let id = child.execution_id();
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(b"preserved input\n").await.unwrap();
        let snapshot = sandbox.snapshot().await.unwrap();
        let input = tempfile::tempdir().unwrap();
        save_command_snapshot(&snapshot, input.path()).unwrap();
        let expected = serde_json::to_value(&snapshot.executions).unwrap();
        drop(snapshot);

        for preserve in [false, true] {
            let loaded = load_command_snapshot(input.path()).unwrap();
            let private_path = loaded.files[0].path().to_path_buf();
            let compacted = loaded.compact(preserve.then_some(&base)).unwrap();
            assert_eq!(
                serde_json::to_value(&compacted.executions).unwrap(),
                expected
            );
            drop(loaded);
            assert!(private_path.exists());
            let output = tempfile::tempdir().unwrap();
            save_command_snapshot(&compacted, output.path()).unwrap();
            let index = read_index(output.path()).unwrap();
            let manifest = read_manifest(output.path(), &index["manifests"][0]).unwrap();
            let memory_layers = manifest["layers"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|layer| {
                    layer["mediaType"] == "application/vnd.hyperlight.snapshot.memory.v2"
                })
                .count();
            if preserve {
                let base_index = read_index(base_dir.path()).unwrap();
                let base_manifest =
                    read_manifest(base_dir.path(), &base_index["manifests"][0]).unwrap();
                let base_layers: Vec<_> = base_manifest["layers"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|layer| {
                        layer["mediaType"] == "application/vnd.hyperlight.snapshot.memory.v2"
                    })
                    .collect();
                assert!(memory_layers <= base_layers.len() + 1);
                assert!(base_layers.iter().any(|base| {
                    manifest["layers"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|layer| layer["digest"] == base["digest"])
                }));
            } else {
                assert_eq!(memory_layers, 1);
            }
            drop(compacted);
            assert!(!private_path.exists());
            let reloaded = load_command_snapshot(output.path()).unwrap();
            assert_eq!(
                serde_json::to_value(&reloaded.executions).unwrap(),
                expected
            );
            let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("build-elfloader");
            let restored = crate::SandboxBuilder::from_command_snapshot(&reloaded)
                .mount(crate::Mount::ro(root.join("bins/c"), "/mnt/bin"))
                .echo_output(false)
                .boot_commands()
                .unwrap();
            let restored_child = restored
                .children()
                .by_id(id)
                .unwrap()
                .restore()
                .await
                .unwrap();
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                restored_child.wait_with_output(),
            )
            .await
            .unwrap()
            .unwrap();
            assert!(result.status.success());
            assert_eq!(result.stdout, b"preserved input\n");
        }
        drop(stdin);
        let result = child.wait_with_output().await.unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, b"preserved input\n");
    }

    #[tokio::test]
    #[ignore = "requires a hypervisor and build-elfloader C fixtures"]
    async fn layered_state_exceeds_former_limit_and_preserves_failed_save() {
        let sandbox = builder().boot_commands().unwrap();
        let _child = sandbox
            .command("/mnt/bin/command")
            .name("x".repeat(JSON_LIMIT))
            .arg("echo")
            .stdin(crate::command::Stdio::piped())
            .spawn()
            .await
            .unwrap();
        let mut snapshot = sandbox.snapshot().await.unwrap();
        let dir = tempfile::tempdir().unwrap();
        save_command_snapshot(&snapshot, dir.path()).unwrap();
        let loaded = load_command_snapshot(dir.path()).unwrap();
        let state = serde_json::to_value(&loaded.executions).unwrap();
        assert_eq!(
            state["entries"][0]["command"]["name"]
                .as_str()
                .unwrap()
                .len(),
            JSON_LIMIT
        );
        let kept_tag = "retained".parse::<crate::OciTag>().unwrap();
        let kept_digest = snapshot.guest.save(dir.path(), &kept_tag).unwrap();
        save_command_snapshot(&snapshot, dir.path()).unwrap();
        let index = read_index(dir.path()).unwrap();
        assert_eq!(
            index["manifests"][tagged(&index, "retained").unwrap()]["digest"],
            kept_digest.to_string()
        );
        let manifest = read_manifest(dir.path(), &index["manifests"][0]).unwrap();
        assert!(
            read_blob(dir.path(), &manifest["config"], Some(JSON_LIMIT))
                .unwrap()
                .len()
                < JSON_LIMIT
        );
        assert!(Snapshot::load(dir.path(), crate::snapshot_tag()).is_err());
        drop(loaded);

        let mut state = serde_json::to_value(&snapshot.executions).unwrap();
        let name_len = 16 * 1024 * 1024 + 1;
        state["entries"][0]["command"]["name"] = "x".repeat(name_len).into();
        snapshot.executions = serde_json::from_value(state.clone()).unwrap();
        save_command_snapshot(&snapshot, dir.path()).unwrap();
        assert!(
            json_bytes(
                &State {
                    version: VERSION,
                    executions: &snapshot.executions
                },
                None
            )
            .unwrap()
            .len()
                > 16 * 1024 * 1024
        );
        let loaded = load_command_snapshot(dir.path()).unwrap();
        assert_eq!(serde_json::to_value(&loaded.executions).unwrap(), state);
        drop(loaded);
        let index = fs::read(dir.path().join("index.json")).unwrap();

        // Fail after Hyperlight has generated its staged manifest, not just
        // during input validation; the published index must still be intact.
        state["entries"][0]["command"]["name"] = "blocked-publication".into();
        snapshot.executions = serde_json::from_value(state).unwrap();
        let bytes = json_bytes(
            &State {
                version: VERSION,
                executions: &snapshot.executions,
            },
            None,
        )
        .unwrap();
        fs::write(
            dir.path().join("blobs/sha256").join(digest(&bytes)),
            vec![b'x'; bytes.len()],
        )
        .unwrap();
        assert!(
            save_command_snapshot(&snapshot, dir.path())
                .unwrap_err()
                .to_string()
                .contains("checksum")
        );
        assert_eq!(fs::read(dir.path().join("index.json")).unwrap(), index);
        assert!(load_command_snapshot(dir.path()).is_ok());
    }

    #[tokio::test]
    #[ignore = "requires a hypervisor and build-elfloader C fixtures"]
    async fn layered_loading_checks_extensions_versions_and_read_only_lifetime() {
        let sandbox = builder().boot_commands().unwrap();
        let snapshot = sandbox.snapshot().await.unwrap();
        let dir = tempfile::tempdir().unwrap();
        save_command_snapshot(&snapshot, dir.path()).unwrap();
        let initial_index = fs::read(dir.path().join("index.json")).unwrap();
        let index = read_index(dir.path()).unwrap();
        let manifest = read_manifest(dir.path(), &index["manifests"][0]).unwrap();
        for media in [STATE_MEDIA, GUEST_MEDIA] {
            let descriptor = manifest["layers"]
                .as_array()
                .unwrap()
                .iter()
                .find(|d| d["mediaType"] == media)
                .unwrap();
            let path = blob_path(dir.path(), descriptor).unwrap();
            let bytes = fs::read(&path).unwrap();
            fs::write(&path, vec![b'x'; bytes.len()]).unwrap();
            assert!(
                load_command_snapshot(dir.path())
                    .err()
                    .unwrap()
                    .to_string()
                    .contains("checksum")
            );
            fs::remove_file(&path).unwrap();
            assert!(load_command_snapshot(dir.path()).is_err());
            fs::write(path, bytes).unwrap();
        }

        let loaded = load_command_snapshot(dir.path()).unwrap();
        let private_path = loaded.files[0].path().to_path_buf();
        sandbox.restore(&loaded).await.unwrap();
        let builder = crate::SandboxBuilder::from_command_snapshot(&loaded).echo_output(false);
        drop(loaded);
        assert!(private_path.exists());
        let restored = builder.boot_commands().unwrap();
        let derived = restored.snapshot().await.unwrap();
        drop(restored);
        assert!(private_path.exists());
        let copy_dir = tempfile::tempdir().unwrap();
        save_command_snapshot(&derived, copy_dir.path()).unwrap();
        drop(derived);
        assert!(private_path.exists());
        drop(sandbox);
        assert!(!private_path.exists());
        assert_eq!(
            fs::read(dir.path().join("index.json")).unwrap(),
            initial_index
        );

        for version in [1, 2, 3, 4, 5, 6, 999] {
            let guest = snapshot
                .guest
                .with_metadata(METADATA, &json!({"version": version}))
                .unwrap();
            crate::save_snapshot(&guest, dir.path()).unwrap();
            assert!(
                load_command_snapshot(dir.path())
                    .err()
                    .unwrap()
                    .to_string()
                    .contains("unsupported command snapshot version")
            );
        }
        atomic_write(&dir.path().join("index.json"), &initial_index).unwrap();
        for media in [STATE_MEDIA, GUEST_MEDIA] {
            let mut broken = manifest.clone();
            let layers = broken["layers"].as_array_mut().unwrap();
            let extra = layers
                .iter()
                .find(|d| d["mediaType"] == media)
                .unwrap()
                .clone();
            layers.push(extra);
            rewrite_manifest(dir.path(), &broken);
            assert!(
                load_command_snapshot(dir.path())
                    .err()
                    .unwrap()
                    .to_string()
                    .contains("duplicate")
            );
            broken["layers"].as_array_mut().unwrap().pop();
            broken["layers"]
                .as_array_mut()
                .unwrap()
                .retain(|d| d["mediaType"] != media);
            rewrite_manifest(dir.path(), &broken);
            assert!(load_command_snapshot(dir.path()).is_err());
        }
        let mut broken = manifest.clone();
        broken["annotations"] = json!({"altered": "not the original"});
        rewrite_manifest(dir.path(), &broken);
        assert!(
            load_command_snapshot(dir.path())
                .err()
                .unwrap()
                .to_string()
                .contains("does not match")
        );

        let mut broken = manifest.clone();
        let bad = put_blob(
            dir.path(),
            &json_bytes(
                &State {
                    version: 999,
                    executions: &snapshot.executions,
                },
                None,
            )
            .unwrap(),
            STATE_MEDIA,
        )
        .unwrap();
        *broken["layers"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|d| d["mediaType"] == STATE_MEDIA)
            .unwrap() = bad;
        rewrite_manifest(dir.path(), &broken);
        assert!(
            load_command_snapshot(dir.path())
                .err()
                .unwrap()
                .to_string()
                .contains("unsupported command snapshot state version")
        );
        let mut broken = manifest.clone();
        let bad = put_blob(dir.path(), br#"{"version":7}"#, STATE_MEDIA).unwrap();
        *broken["layers"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|d| d["mediaType"] == STATE_MEDIA)
            .unwrap() = bad;
        rewrite_manifest(dir.path(), &broken);
        assert!(
            load_command_snapshot(dir.path())
                .err()
                .unwrap()
                .to_string()
                .contains("decode command snapshot state")
        );
        atomic_write(&dir.path().join("index.json"), &initial_index).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let paths = [
                dir.path().to_path_buf(),
                dir.path().join("blobs"),
                dir.path().join("blobs/sha256"),
            ];
            let files: Vec<_> = fs::read_dir(dir.path().join("blobs/sha256"))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .chain([dir.path().join("index.json"), dir.path().join("oci-layout")])
                .collect();
            for path in &files {
                fs::set_permissions(path, fs::Permissions::from_mode(0o444)).unwrap();
            }
            for path in &paths {
                fs::set_permissions(path, fs::Permissions::from_mode(0o555)).unwrap();
            }
            let result = load_command_snapshot(dir.path());
            for path in &paths {
                fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
            }
            for path in &files {
                fs::set_permissions(path, fs::Permissions::from_mode(0o644)).unwrap();
            }
            assert!(result.is_ok());
            assert_eq!(
                fs::read(dir.path().join("index.json")).unwrap(),
                initial_index
            );
        }
    }

    #[test]
    fn portable_blob_view_preserves_sources_and_rejects_nonfiles() {
        let source = tempfile::tempdir().unwrap();
        let dest = tempfile::tempdir().unwrap();
        fs::write(source.path().join("blob"), b"kept").unwrap();
        copy_blobs(source.path(), dest.path()).unwrap();
        assert_eq!(fs::read(dest.path().join("blob")).unwrap(), b"kept");
        drop(dest);
        assert_eq!(fs::read(source.path().join("blob")).unwrap(), b"kept");
        fs::create_dir(source.path().join("not-a-blob")).unwrap();
        let dest = tempfile::tempdir().unwrap();
        assert!(copy_blobs(source.path(), dest.path()).is_err());
    }

    #[test]
    fn invalid_blob_descriptors_are_rejected_before_path_or_data_use() {
        let dir = tempfile::tempdir().unwrap();
        for descriptor in [
            json!({"digest": "sha256:../../etc/passwd", "size": 0}),
            json!({"digest": format!("sha256:{}", "a".repeat(64)), "size": JSON_LIMIT + 1}),
            json!({"digest": format!("sha256:{}", "a".repeat(64)), "size": -1}),
        ] {
            assert!(read_blob(dir.path(), &descriptor, Some(JSON_LIMIT)).is_err());
        }
    }

    #[test]
    fn uncapped_blob_reads_still_check_declared_size_and_digest() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("blobs/sha256")).unwrap();
        let descriptor = put_blob(dir.path(), b"state", STATE_MEDIA).unwrap();
        for size in [0, 4, 6, u64::MAX] {
            let mut invalid = descriptor.clone();
            invalid["size"] = size.into();
            assert!(read_blob(dir.path(), &invalid, None).is_err());
        }
        fs::write(blob_path(dir.path(), &descriptor).unwrap(), b"other").unwrap();
        assert!(
            read_blob(dir.path(), &descriptor, None)
                .unwrap_err()
                .to_string()
                .contains("checksum")
        );
    }
}
