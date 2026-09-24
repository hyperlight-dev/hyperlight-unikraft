// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! Pull a published rootfs from an OCI registry with no Docker on the host.
//!
//! `publish-images.yml` pushes each runtime's CPIO as a one-layer image,
//! `<registry>/<runtime>:initrd-v<version>`, whose single layer is a tar
//! holding `initrd.cpio`.  This module speaks just enough of the
//! distribution API to fetch that: the anonymous bearer-token dance, the
//! manifest (through an index if the registry serves one), and the layer,
//! checked against its digest and unpacked.

use std::fmt;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use ureq::Agent;
use ureq::http::{Response, StatusCode};

/// Where the project's images live, and what `hluk init` uses unless
/// `--registry` or `HLUK_REGISTRY` says otherwise.
pub const DEFAULT_REGISTRY: &str = "ghcr.io/hyperlight-dev/hyperlight-unikraft";

/// This build's release, which names the image tags it matches.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The registry `init` renders into a project: the flag, else the
/// environment, else the project's own.
pub fn registry(flag: Option<String>) -> String {
    let raw = flag
        .or_else(|| std::env::var("HLUK_REGISTRY").ok())
        .unwrap_or_else(|| DEFAULT_REGISTRY.to_string());
    // An image reference is host/path, never a URL; strip a scheme a
    // user might paste in so the reference parses.
    let stripped = raw
        .strip_prefix("https://")
        .or_else(|| raw.strip_prefix("http://"))
        .unwrap_or(&raw);
    stripped.trim_end_matches('/').to_string()
}

/// The release whose images `init` writes into a project: the flag, else
/// `HLUK_IMAGE_VERSION`, else this build's own.  A build between releases
/// has no published images; pointing it at the last release's works for
/// as long as the driver protocol has not changed since.
pub fn image_version(flag: Option<String>) -> String {
    flag.or_else(|| std::env::var("HLUK_IMAGE_VERSION").ok())
        .map(|v| v.trim().trim_start_matches('v').to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| VERSION.to_string())
}

/// The image a `--runtime` value names: a full reference as given
/// (`ghcr.io/…/python:initrd-v0.14.1`, anything with a `/`), or a runtime
/// name (`python`) composed with the registry and this build's release.
pub fn runtime_image(spec: &str) -> String {
    if spec.contains('/') {
        spec.to_string()
    } else {
        initrd_image(&registry(None), spec, &image_version(None))
    }
}

/// The runtime name a `--runtime` value carries: itself for a name, the
/// repository's last component for a full reference.
pub fn runtime_name(spec: &str) -> Option<&str> {
    if spec.contains('/') {
        Reference::parse(spec)
            .ok()
            .and_then(|_| spec.rsplit('/').next())
            .and_then(|last| last.split([':', '@']).next())
    } else {
        Some(spec)
    }
}

/// The runnable CPIO image for `runtime` at `version`.
pub fn initrd_image(registry: &str, runtime: &str, version: &str) -> String {
    format!("{registry}/{runtime}:initrd-v{version}")
}

/// The filesystem image for `runtime` at `version`, to build `FROM`.
pub fn base_image(registry: &str, runtime: &str, version: &str) -> String {
    format!("{registry}/{runtime}:v{version}")
}

/// A parsed image reference: `host/repository[:tag|@digest]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub host: String,
    pub repository: String,
    /// A tag, or a `sha256:…` digest.
    pub reference: String,
}

impl Reference {
    /// Parse `ghcr.io/owner/repo/python:initrd-v0.14.1` and friends.  The
    /// first component must be a registry host (it has a `.` or a `:`, or
    /// is `localhost`): hluk images are never on Docker Hub, so the
    /// implicit `docker.io` would only hide a typo.
    pub fn parse(s: &str) -> Result<Reference, String> {
        let s = s.trim();
        let (host, rest) = s
            .split_once('/')
            .ok_or_else(|| format!("image {s:?}: expected host/repository[:tag]"))?;
        if !(host.contains('.') || host.contains(':') || host == "localhost") {
            return Err(format!(
                "image {s:?}: must start with a registry host, e.g. {DEFAULT_REGISTRY}/python"
            ));
        }
        let (repository, reference) = if let Some((repo, digest)) = rest.split_once('@') {
            (repo, digest.to_string())
        } else if let Some((repo, tag)) = rest.rsplit_once(':') {
            (repo, tag.to_string())
        } else {
            (rest, "latest".to_string())
        };
        if repository.is_empty() || reference.is_empty() {
            return Err(format!("image {s:?}: empty repository or tag"));
        }
        // The registry would refuse these too, but the cache path is built
        // from them, so nothing that is not a plain name gets that far.
        if !valid_host(host) || !valid_repository(repository) || !valid_reference(&reference) {
            return Err(format!(
                "image {s:?}: a repository is lowercase path components, a tag is letters, \
                 digits, `.`, `_` and `-`, a digest is sha256:<64 hex digits>"
            ));
        }
        Ok(Reference {
            host: host.to_string(),
            repository: repository.to_string(),
            reference,
        })
    }

    /// The file this image's CPIO is cached at, under a cache root: the
    /// reference laid out as directories, with `:` (a host's port, a
    /// digest's algorithm) made filename-safe.
    pub fn cache_relpath(&self) -> PathBuf {
        let mut path = PathBuf::from(self.host.replace(':', "_"));
        path.extend(self.repository.split('/'));
        path.push(format!("{}.cpio", self.reference.replace(':', "_")));
        path
    }

    fn manifest_url(&self, reference: &str) -> String {
        format!(
            "https://{}/v2/{}/manifests/{reference}",
            self.host, self.repository
        )
    }

    fn blob_url(&self, digest: &str) -> String {
        format!(
            "https://{}/v2/{}/blobs/{digest}",
            self.host, self.repository
        )
    }
}

/// A registry host: a DNS name, with an optional port.
fn valid_host(host: &str) -> bool {
    let (name, port) = host.split_once(':').unwrap_or((host, "0"));
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
        && !port.is_empty()
        && port.bytes().all(|b| b.is_ascii_digit())
}

/// A repository as the distribution spec allows it: `/`-separated
/// components of lowercase letters, digits and `.`, `_`, `-` separators.
fn valid_repository(repository: &str) -> bool {
    repository.split('/').all(|component| {
        component
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
            && component.bytes().all(|b| {
                b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'.' || b == b'_' || b == b'-'
            })
    })
}

/// A tag (`initrd-v0.14.1`) or a `sha256:` digest.
fn valid_reference(reference: &str) -> bool {
    if let Some(hex) = reference.strip_prefix("sha256:") {
        return hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit());
    }
    reference.len() <= 128
        && reference
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphanumeric() || b == b'_')
        && reference
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'-')
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = if self.reference.contains(':') {
            '@'
        } else {
            ':'
        };
        write!(
            f,
            "{}/{}{sep}{}",
            self.host, self.repository, self.reference
        )
    }
}

/// A layer of an image manifest.
#[derive(Deserialize, Debug, Clone)]
pub struct Layer {
    #[serde(rename = "mediaType", default)]
    pub media_type: String,
    pub digest: String,
    #[serde(default)]
    pub size: u64,
}

#[derive(Deserialize, Debug)]
struct ImageManifest {
    #[serde(default)]
    layers: Vec<Layer>,
    #[serde(default)]
    manifests: Vec<IndexEntry>,
}

#[derive(Deserialize, Debug)]
struct IndexEntry {
    digest: String,
    #[serde(default)]
    platform: Option<Platform>,
}

#[derive(Deserialize, Debug)]
struct Platform {
    architecture: String,
    os: String,
}

#[derive(Deserialize, Debug)]
struct Token {
    #[serde(default)]
    token: String,
    #[serde(default)]
    access_token: String,
}

const MANIFEST_ACCEPT: &str = "application/vnd.docker.distribution.manifest.v2+json, \
    application/vnd.oci.image.manifest.v1+json, \
    application/vnd.docker.distribution.manifest.list.v2+json, \
    application/vnd.oci.image.index.v1+json";

/// A registry session: one HTTP agent and the bearer token the registry
/// handed out for the repository being pulled.
pub struct Client {
    agent: Agent,
    token: Option<String>,
}

impl Client {
    pub fn new() -> Client {
        // Statuses come back as responses (a 401 carries the challenge we
        // need); a redirect to a blob CDN is followed with the token
        // dropped, which is what the presigned URL wants.
        let config = Agent::config_builder()
            .http_status_as_error(false)
            .timeout_connect(Some(Duration::from_secs(20)))
            .timeout_recv_response(Some(Duration::from_secs(60)))
            // A layer is at most a few hundred MiB; a transfer that takes
            // longer than this has stalled.
            .timeout_recv_body(Some(Duration::from_secs(30 * 60)))
            .user_agent(format!("hluk/{VERSION}"))
            .build();
        Client {
            agent: config.into(),
            token: None,
        }
    }

    /// GET `url`, answering a bearer challenge once with an anonymous token
    /// for the scope the registry names.
    fn get(&mut self, url: &str, accept: &str) -> Result<Response<ureq::Body>, String> {
        let response = self.request(url, accept)?;
        if response.status() != StatusCode::UNAUTHORIZED || self.token.is_some() {
            return Ok(response);
        }
        let challenge = response
            .headers()
            .get("www-authenticate")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| format!("{url}: 401 without a WWW-Authenticate challenge"))?;
        self.token = Some(self.fetch_token(challenge)?);
        self.request(url, accept)
    }

    fn request(&self, url: &str, accept: &str) -> Result<Response<ureq::Body>, String> {
        let mut req = self.agent.get(url).header("Accept", accept);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        req.call().map_err(|e| format!("GET {url}: {e}"))
    }

    /// Trade a `Bearer realm=…,service=…,scope=…` challenge for a token.
    fn fetch_token(&self, challenge: &str) -> Result<String, String> {
        let params = parse_challenge(challenge)
            .ok_or_else(|| format!("unsupported auth challenge {challenge:?}"))?;
        let mut url = params.realm.clone();
        let mut sep = if url.contains('?') { '&' } else { '?' };
        for (key, value) in [("service", &params.service), ("scope", &params.scope)] {
            if let Some(value) = value {
                url.push(sep);
                url.push_str(key);
                url.push('=');
                url.push_str(&encode_query(value));
                sep = '&';
            }
        }
        let mut response = self
            .agent
            .get(&url)
            .call()
            .map_err(|e| format!("GET {url}: {e}"))?;
        if !response.status().is_success() {
            return Err(format!("{url}: HTTP {}", response.status()));
        }
        let body = response
            .body_mut()
            .read_to_string()
            .map_err(|e| format!("{url}: {e}"))?;
        let token: Token = serde_json::from_str(&body).map_err(|e| format!("{url}: {e}"))?;
        let token = if token.token.is_empty() {
            token.access_token
        } else {
            token.token
        };
        if token.is_empty() {
            return Err(format!("{url}: no token in the response"));
        }
        Ok(token)
    }

    /// The layers of `reference`'s image, following an index to the
    /// linux/amd64 manifest, and the manifest's digest when the registry
    /// reports one.
    pub fn manifest(
        &mut self,
        reference: &Reference,
    ) -> Result<(Option<String>, Vec<Layer>), String> {
        let (digest, manifest) = self.fetch_manifest(reference, &reference.reference)?;
        if manifest.manifests.is_empty() {
            return Ok((digest, manifest.layers));
        }
        // An index: only the platform the guest kernel is built for will do.
        let entry = manifest
            .manifests
            .iter()
            .find(|m| {
                m.platform
                    .as_ref()
                    .is_some_and(|p| p.architecture == "amd64" && p.os == "linux")
            })
            .ok_or_else(|| format!("{reference}: the image index has no linux/amd64 manifest"))?;
        let (_, manifest) = self.fetch_manifest(reference, &entry.digest)?;
        Ok((Some(entry.digest.clone()), manifest.layers))
    }

    fn fetch_manifest(
        &mut self,
        reference: &Reference,
        tag_or_digest: &str,
    ) -> Result<(Option<String>, ImageManifest), String> {
        let url = reference.manifest_url(tag_or_digest);
        let mut response = self.get(&url, MANIFEST_ACCEPT)?;
        match response.status() {
            StatusCode::OK => {}
            StatusCode::NOT_FOUND => {
                return Err(format!(
                    "{reference} is not published (HTTP 404). A release publishes it a few \
                     minutes after tagging; a build of an unreleased hluk has no published \
                     rootfs, so pin the last release's with --image-version X.Y.Z (or \
                     HLUK_IMAGE_VERSION), or build one with `just build-rootfs <runtime>` \
                     and point [rootfs] path at it"
                ));
            }
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                return Err(format!(
                    "{reference}: access denied (HTTP {}); hluk pulls anonymously, so the \
                     image must be public",
                    response.status()
                ));
            }
            status => return Err(format!("GET {url}: HTTP {status}")),
        }
        let digest = response
            .headers()
            .get("docker-content-digest")
            .and_then(|v| v.to_str().ok())
            .map(str::to_string);
        let body = response
            .body_mut()
            .read_to_string()
            .map_err(|e| format!("{url}: {e}"))?;
        let manifest: ImageManifest = serde_json::from_str(&body)
            .map_err(|e| format!("{url}: not an image manifest: {e}"))?;
        Ok((digest, manifest))
    }

    /// Download the layer `digest` of `reference` into `dest`, checking it
    /// against the digest as it streams.  `progress` is told (bytes so far,
    /// bytes expected) as it goes.
    pub fn download_layer(
        &mut self,
        reference: &Reference,
        layer: &Layer,
        dest: &mut impl Write,
        progress: &mut dyn FnMut(u64, u64),
    ) -> Result<(), String> {
        let url = reference.blob_url(&layer.digest);
        let mut response = self.get(&url, "application/octet-stream")?;
        if response.status() != StatusCode::OK {
            return Err(format!("GET {url}: HTTP {}", response.status()));
        }
        let expected = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(layer.size);
        let mut body = response.body_mut().as_reader();
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; 256 * 1024];
        let mut done = 0u64;
        loop {
            let n = body.read(&mut buf).map_err(|e| format!("{url}: {e}"))?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            dest.write_all(&buf[..n]).map_err(|e| e.to_string())?;
            done += n as u64;
            progress(done, expected);
        }
        let hex: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        let actual = format!("sha256:{hex}");
        if actual != layer.digest {
            return Err(format!(
                "{reference}: layer digest mismatch (expected {}, got {actual})",
                layer.digest
            ));
        }
        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

struct Challenge {
    realm: String,
    service: Option<String>,
    scope: Option<String>,
}

/// `Bearer realm="https://ghcr.io/token",service="ghcr.io",scope="repository:x:pull"`.
fn parse_challenge(header: &str) -> Option<Challenge> {
    let params = header.strip_prefix("Bearer ")?;
    let mut realm = None;
    let mut service = None;
    let mut scope = None;
    for part in params.split(',') {
        let (key, value) = part.trim().split_once('=')?;
        let value = value.trim().trim_matches('"').to_string();
        match key.trim() {
            "realm" => realm = Some(value),
            "service" => service = Some(value),
            "scope" => scope = Some(value),
            _ => {}
        }
    }
    Some(Challenge {
        realm: realm?,
        service,
        scope,
    })
}

/// Percent-encode a query value; a scope has `:` and `/`, which are fine,
/// but nothing else is assumed.
fn encode_query(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b':' | b'/' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Unpack the `initrd.cpio` inside a downloaded layer (a gzip'd tar, or a
/// plain tar for a `…tar` media type) into `dest`, returning its size.
pub fn extract_initrd(layer: &Layer, blob: impl Read, dest: &Path) -> Result<u64, String> {
    let gzipped = layer.media_type.ends_with("gzip") || layer.media_type.is_empty();
    let reader: Box<dyn Read> = if gzipped {
        Box::new(flate2::read::GzDecoder::new(blob))
    } else {
        Box::new(blob)
    };
    let mut archive = tar::Archive::new(reader);
    for entry in archive
        .entries()
        .map_err(|e| format!("layer is not a tar: {e}"))?
    {
        let mut entry = entry.map_err(|e| format!("reading layer: {e}"))?;
        let is_initrd = entry
            .path()
            .ok()
            .and_then(|p| p.file_name().map(|n| n == "initrd.cpio"))
            .unwrap_or(false);
        if !is_initrd || !entry.header().entry_type().is_file() {
            continue;
        }
        let mut file = std::fs::File::create(dest)
            .map_err(|e| format!("cannot write {}: {e}", dest.display()))?;
        let bytes =
            io::copy(&mut entry, &mut file).map_err(|e| format!("unpacking initrd: {e}"))?;
        return Ok(bytes);
    }
    Err("the image's layer has no initrd.cpio: not a hluk `:initrd` image".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_references() {
        let r =
            Reference::parse("ghcr.io/hyperlight-dev/hyperlight-unikraft/python:initrd-v0.14.1")
                .unwrap();
        assert_eq!(r.host, "ghcr.io");
        assert_eq!(r.repository, "hyperlight-dev/hyperlight-unikraft/python");
        assert_eq!(r.reference, "initrd-v0.14.1");
        assert_eq!(
            r.to_string(),
            "ghcr.io/hyperlight-dev/hyperlight-unikraft/python:initrd-v0.14.1"
        );
        assert_eq!(
            r.cache_relpath(),
            PathBuf::from("ghcr.io/hyperlight-dev/hyperlight-unikraft/python/initrd-v0.14.1.cpio")
        );

        let r = Reference::parse("localhost:5000/hluk/python").unwrap();
        assert_eq!(r.host, "localhost:5000");
        assert_eq!(r.reference, "latest");
        assert_eq!(
            r.cache_relpath(),
            PathBuf::from("localhost_5000/hluk/python/latest.cpio")
        );

        let digest = format!("sha256:{}", "ab".repeat(32));
        let r = Reference::parse(&format!("ghcr.io/o/r@{digest}")).unwrap();
        assert_eq!(r.reference, digest);
        assert_eq!(r.to_string(), format!("ghcr.io/o/r@{digest}"));
        assert_eq!(
            r.cache_relpath(),
            PathBuf::from(format!("ghcr.io/o/r/{}.cpio", digest.replace(':', "_")))
        );
    }

    #[test]
    fn rejects_references_without_a_host() {
        assert!(Reference::parse("python:latest").is_err());
        assert!(Reference::parse("hyperlight/python").is_err());
        assert!(Reference::parse("ghcr.io/").is_err());
    }

    #[test]
    fn rejects_references_that_are_not_plain_names() {
        // Nothing that could leave the cache directory, or that a registry
        // would not serve, becomes a cache path.
        for bad in [
            "ghcr.io/../etc:tag",
            "ghcr.io/a/../b:tag",
            "ghcr.io/a/b:../../x",
            "ghcr.io/A/b:tag",
            "ghcr.io/a/b:tag/x",
            "ghcr.io/a/b@sha256:abc",
            "ghcr.io/a/b@md5:00",
            "gh cr.io/a/b:tag",
        ] {
            assert!(Reference::parse(bad).is_err(), "{bad} accepted");
        }
        assert!(Reference::parse("localhost:5000/a.b/c_d-e:v1.0_rc-2").is_ok());
        assert!(Reference::parse(&format!("ghcr.io/a/b@sha256:{}", "ab".repeat(32))).is_ok());
    }

    #[test]
    fn image_names_follow_the_release() {
        let v = image_version(None);
        assert_eq!(v, VERSION);
        assert_eq!(
            initrd_image("ghcr.io/x/y", "python", &v),
            format!("ghcr.io/x/y/python:initrd-v{VERSION}")
        );
        assert_eq!(
            base_image("ghcr.io/x/y", "node", "0.14.1"),
            "ghcr.io/x/y/node:v0.14.1"
        );
        assert_eq!(image_version(Some("v0.14.1".into())), "0.14.1");
        assert_eq!(image_version(Some("  ".into())), VERSION);
        assert_eq!(
            registry(Some("https://r.example/".into())),
            "r.example",
            "the scheme is stripped so the reference parses as host/path"
        );
        assert_eq!(
            registry(Some("http://localhost:5000/".into())),
            "localhost:5000"
        );
    }

    #[test]
    fn a_runtime_spec_is_a_name_or_a_reference() {
        assert_eq!(
            runtime_image("python"),
            format!("{DEFAULT_REGISTRY}/python:initrd-v{VERSION}")
        );
        assert_eq!(
            runtime_image("ghcr.io/o/r/node:initrd-v0.14.0"),
            "ghcr.io/o/r/node:initrd-v0.14.0"
        );
        assert_eq!(runtime_name("agent"), Some("agent"));
        assert_eq!(
            runtime_name("ghcr.io/o/r/node:initrd-v0.14.0"),
            Some("node")
        );
        assert_eq!(
            runtime_name(&format!("ghcr.io/o/r/agent@sha256:{}", "ab".repeat(32))),
            Some("agent")
        );
    }

    #[test]
    fn parses_a_bearer_challenge() {
        let c = parse_challenge(
            r#"Bearer realm="https://ghcr.io/token",service="ghcr.io",scope="repository:a/b:pull""#,
        )
        .unwrap();
        assert_eq!(c.realm, "https://ghcr.io/token");
        assert_eq!(c.service.as_deref(), Some("ghcr.io"));
        assert_eq!(c.scope.as_deref(), Some("repository:a/b:pull"));
        assert!(parse_challenge("Basic realm=x").is_none());
        assert_eq!(
            encode_query("repository:a/b:pull c"),
            "repository:a/b:pull%20c"
        );
    }

    #[test]
    fn extracts_the_initrd_from_a_layer() {
        let dir = tempfile::tempdir().unwrap();
        let mut tar = tar::Builder::new(Vec::new());
        let mut header = tar::Header::new_gnu();
        header.set_size(5);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "initrd.cpio", &b"07070"[..])
            .unwrap();
        let tar = tar.into_inner().unwrap();
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        gz.write_all(&tar).unwrap();
        let gz = gz.finish().unwrap();
        let layer = Layer {
            media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".into(),
            digest: String::new(),
            size: gz.len() as u64,
        };
        let dest = dir.path().join("out.cpio");
        assert_eq!(extract_initrd(&layer, &gz[..], &dest).unwrap(), 5);
        assert_eq!(std::fs::read(&dest).unwrap(), b"07070");

        let empty = tar::Builder::new(Vec::new()).into_inner().unwrap();
        let plain = Layer {
            media_type: "application/vnd.oci.image.layer.v1.tar".into(),
            ..layer
        };
        assert!(extract_initrd(&plain, &empty[..], &dest).is_err());
    }
}
