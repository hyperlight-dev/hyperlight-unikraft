// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! `hluk.toml`, the app manifest `hluk init` writes and `hluk run`, `build`
//! and `pull` read.  It names the rootfs the app runs on, what to run in
//! it and with which capabilities, and how to build it; docs/manifest.md is
//! the reference.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use hyperlight_unikraft::{
    ListenPorts, Mount, NetworkPolicy, default_scratch_mb, runtime_scratch_mb,
};

use super::registry::Reference;

use crate::{PortSpec, parse_mounts, parse_net_policy};

/// The manifest's file name, looked for in the current directory.
pub const FILE_NAME: &str = "hluk.toml";

/// The manifest format this build reads and writes.
pub const VERSION: u32 = 1;

/// A parsed, validated `hluk.toml`.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub manifest_version: u32,
    pub app: App,
    pub rootfs: Rootfs,
    pub run: Run,
    pub build: Option<Build>,

    /// The directory the manifest was read from; every relative path in it
    /// resolves against this.
    #[serde(skip)]
    pub dir: PathBuf,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct App {
    pub name: String,
}

/// Where the guest's root filesystem comes from; exactly one is set.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Rootfs {
    /// A published `:initrd` image, pulled into the local cache.
    pub image: Option<String>,
    /// A Dockerfile `hluk build` builds into `.hluk/rootfs.cpio`.
    pub dockerfile: Option<PathBuf>,
    /// A CPIO already on disk.
    pub path: Option<PathBuf>,
}

/// The one rootfs source a manifest names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootfsSource<'a> {
    Image(&'a str),
    Dockerfile(&'a Path),
    Path(&'a Path),
}

/// What to run in the guest, and with what.  The workload keys mirror the
/// `hluk run` flags of the same names.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Run {
    /// A host file the runtime driver runs (`hluk run <script>`).
    pub script: Option<PathBuf>,
    /// Code, or for the exec driver a command line, the driver runs
    /// (`hluk run --exec`).
    pub exec: Option<String>,
    /// A command already in the guest filesystem (`hluk run --guest-exec`).
    pub guest_exec: Option<String>,
    /// The guest's entry point (`hluk run --entry`): a plain program as PID
    /// 1 when the rootfs has no driver.
    pub entry: Option<String>,
    /// Guest memory in MiB; without it, the size the runtime image is
    /// tested with (see [`Manifest::scratch_mb`]).
    pub scratch_mb: Option<usize>,
    /// `HOST:GUEST[:ro]` entries; a relative host path is relative to the
    /// manifest.
    #[serde(default)]
    pub mounts: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Snapshot the booted guest on the first `hluk run` and restore it on
    /// the next ones.
    #[serde(default)]
    pub warm: bool,
    /// Code the runtime driver runs once, after boot and before the warm
    /// snapshot is taken, so what it loads (`import flask`) is in the
    /// snapshot.
    pub warm_exec: Option<String>,
    /// A file to install as the guest's `/etc/resolv.conf`.
    pub resolv_conf: Option<PathBuf>,
    pub net: Option<Net>,
}

/// Networking, off unless this table is present.  At most one of `all`,
/// `allow` and `block`; `ports` needs one of them.
#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Net {
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub allow: Vec<String>,
    #[serde(default)]
    pub block: Vec<String>,
    #[serde(default)]
    pub ports: Vec<Port>,
}

/// A `ports` entry: `8080`, or `"8000-8010"` / `"all"` as a string, the
/// forms `hluk run --port` takes.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Port {
    Number(u16),
    Spec(String),
}

impl Port {
    fn to_spec(&self) -> Result<PortSpec, String> {
        match self {
            Port::Number(n) => Ok(PortSpec::One(*n)),
            Port::Spec(s) => s.parse(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Build {
    /// Run in the manifest's directory through the shell (`sh -c`, or
    /// `cmd /C` on Windows) before the rootfs is built.
    pub command: String,
}

/// What `run` asks the guest to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Workload {
    Script(PathBuf),
    Exec(String),
    GuestExec(String),
    /// Nothing named: the rootfs's conventional entrypoint, or with no
    /// driver the entry point program itself.
    None,
}

impl Manifest {
    /// Locate the manifest: `explicit` (a file, or a directory holding
    /// `hluk.toml`), else `hluk.toml` in the current directory.
    pub fn locate(explicit: Option<&Path>) -> Result<PathBuf, String> {
        let path = match explicit {
            Some(p) if p.is_dir() => p.join(FILE_NAME),
            Some(p) => p.to_path_buf(),
            None => PathBuf::from(FILE_NAME),
        };
        if path.is_file() {
            Ok(path)
        } else if explicit.is_some() {
            Err(format!("no manifest at {}", path.display()))
        } else {
            Err(format!(
                "no {FILE_NAME} in the current directory: start a project with `hluk init`, \
                 pass -f <path> to one, or name a guest with --runtime <name> or --initrd <cpio>"
            ))
        }
    }

    /// Read and validate the manifest at `path`.
    pub fn load(path: &Path) -> Result<Manifest, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let mut manifest =
            Manifest::parse(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        manifest.dir = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        Ok(manifest)
    }

    /// Parse manifest text and check what TOML cannot: the version, that
    /// exactly one rootfs source and at most one workload are named, and
    /// that the network table is consistent.  `dir` is left for the caller.
    pub fn parse(text: &str) -> Result<Manifest, String> {
        let manifest: Manifest = toml::from_str(text).map_err(|e| e.to_string())?;
        if manifest.manifest_version != VERSION {
            return Err(format!(
                "manifest_version {} is not one this hluk reads (it reads {VERSION})",
                manifest.manifest_version
            ));
        }
        if manifest.app.name.trim().is_empty() {
            return Err("[app] name is empty".into());
        }
        let sources = [
            manifest.rootfs.image.is_some(),
            manifest.rootfs.dockerfile.is_some(),
            manifest.rootfs.path.is_some(),
        ]
        .iter()
        .filter(|set| **set)
        .count();
        if sources != 1 {
            return Err("[rootfs] needs exactly one of image, dockerfile or path".into());
        }
        let workloads = [
            manifest.run.script.is_some(),
            manifest.run.exec.is_some(),
            manifest.run.guest_exec.is_some(),
        ]
        .iter()
        .filter(|set| **set)
        .count();
        if workloads > 1 {
            return Err("[run] names more than one of script, exec and guest_exec".into());
        }
        if let Some(net) = &manifest.run.net {
            let policies = [net.all, !net.allow.is_empty(), !net.block.is_empty()]
                .iter()
                .filter(|set| **set)
                .count();
            if policies > 1 {
                return Err("[run.net] sets more than one of all, allow and block".into());
            }
            if policies == 0 && !net.ports.is_empty() {
                return Err(
                    "[run.net] ports need networking: set all = true, allow = [...] or block = [...]"
                        .into(),
                );
            }
            for port in &net.ports {
                port.to_spec()?;
            }
        }
        Ok(manifest)
    }

    pub fn rootfs(&self) -> RootfsSource<'_> {
        if let Some(image) = &self.rootfs.image {
            RootfsSource::Image(image)
        } else if let Some(dockerfile) = &self.rootfs.dockerfile {
            RootfsSource::Dockerfile(dockerfile)
        } else {
            RootfsSource::Path(self.rootfs.path.as_deref().expect("validated: one source"))
        }
    }

    /// `path` as the manifest means it: relative to its directory.
    pub fn resolve(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.dir.join(path)
        }
    }

    /// The guest's memory: the manifest's `scratch_mb`; else the size the
    /// runtime image is tested with, by the image's name
    /// (`…/agent:initrd-v0.14.1` is `agent`) when the rootfs is a published
    /// image, else by the driver found in `rootfs`.
    pub fn scratch_mb(&self, rootfs: &Path) -> usize {
        if let Some(mb) = self.run.scratch_mb {
            return mb;
        }
        let by_name = self
            .rootfs
            .image
            .as_deref()
            .and_then(|image| Reference::parse(image).ok())
            .and_then(|r| r.repository.rsplit('/').next().and_then(runtime_scratch_mb));
        by_name.unwrap_or_else(|| default_scratch_mb(rootfs))
    }

    /// Where `hluk build` writes this project's Dockerfile rootfs.
    pub fn state_dir(&self) -> PathBuf {
        self.dir.join(".hluk")
    }

    pub fn workload(&self) -> Workload {
        if let Some(script) = &self.run.script {
            Workload::Script(self.resolve(script))
        } else if let Some(code) = &self.run.exec {
            Workload::Exec(code.clone())
        } else if let Some(cmd) = &self.run.guest_exec {
            Workload::GuestExec(cmd.clone())
        } else {
            Workload::None
        }
    }

    /// The mounts, with host paths resolved against the manifest.
    pub fn mounts(&self) -> Result<Vec<Mount>, String> {
        Ok(parse_mounts(&self.run.mounts)?
            .into_iter()
            .map(|m| Mount {
                host_path: self.resolve(&m.host_path),
                ..m
            })
            .collect())
    }

    /// The network policy and listen ports, as `hluk run` would build them
    /// from `--net`/`--net-allow`/`--net-block`/`--port`.
    pub fn network(&self) -> Result<(Option<NetworkPolicy>, Option<ListenPorts>), String> {
        let Some(net) = &self.run.net else {
            return Ok((None, None));
        };
        let ports = net
            .ports
            .iter()
            .map(Port::to_spec)
            .collect::<Result<Vec<_>, _>>()?;
        parse_net_policy(net.all, &net.allow, &net.block, &ports)
    }

    /// Whether `hluk build` has anything to do for this project.
    pub fn needs_build(&self) -> bool {
        self.build.is_some() || self.rootfs.dockerfile.is_some()
    }
}

impl fmt::Display for RootfsSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootfsSource::Image(image) => write!(f, "image {image}"),
            RootfsSource::Dockerfile(path) => write!(f, "Dockerfile {}", path.display()),
            RootfsSource::Path(path) => write!(f, "{}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r#"
        manifest_version = 1
        [app]
        name = "demo"
        [rootfs]
        image = "ghcr.io/example/hluk/python:initrd-v1.0.0"
        [run]
        script = "main.py"
    "#;

    fn with_dir(text: &str) -> Manifest {
        let mut m = Manifest::parse(text).unwrap();
        m.dir = PathBuf::from("/proj");
        m
    }

    #[test]
    fn minimal_manifest_parses_with_defaults() {
        let m = with_dir(MINIMAL);
        assert_eq!(m.app.name, "demo");
        assert_eq!(m.run.scratch_mb, None);
        assert!(!m.run.warm);
        assert_eq!(
            m.rootfs(),
            RootfsSource::Image("ghcr.io/example/hluk/python:initrd-v1.0.0")
        );
        assert_eq!(
            m.workload(),
            Workload::Script(PathBuf::from("/proj/main.py"))
        );
        assert!(m.mounts().unwrap().is_empty());
        assert!(matches!(m.network().unwrap(), (None, None)));
        assert!(!m.needs_build());
    }

    #[test]
    fn memory_is_explicit_else_the_images_tested_size_else_the_libraries() {
        let m = with_dir(MINIMAL);
        // The python image, by name; the rootfs file need not exist for that.
        assert_eq!(m.scratch_mb(Path::new("/nonexistent")), 256);
        let agent = with_dir(&MINIMAL.replace("python:initrd", "agent:initrd"));
        assert_eq!(agent.scratch_mb(Path::new("/nonexistent")), 1536);
        let explicit = with_dir(&MINIMAL.replace(
            "script = \"main.py\"",
            "script = \"main.py\"\nscratch_mb = 64",
        ));
        assert_eq!(explicit.scratch_mb(Path::new("/nonexistent")), 64);
        // An image whose name is no runtime falls back to the library's default.
        let other = with_dir(&MINIMAL.replace("python:initrd", "stats-api:initrd"));
        assert_eq!(
            other.scratch_mb(Path::new("/nonexistent")),
            hyperlight_unikraft::DEFAULT_SCRATCH_MB
        );
    }

    #[test]
    fn relative_paths_resolve_against_the_manifest_dir() {
        let m = with_dir(
            r#"
            manifest_version = 1
            [app]
            name = "demo"
            [rootfs]
            path = "out/rootfs.cpio"
            [run]
            exec = "/mnt/app/app"
            mounts = ["./bin:/mnt/app:ro", "/abs:/mnt/abs"]
        "#,
        );
        assert_eq!(m.rootfs(), RootfsSource::Path(Path::new("out/rootfs.cpio")));
        assert_eq!(
            m.resolve(Path::new("out/rootfs.cpio")),
            PathBuf::from("/proj/out/rootfs.cpio")
        );
        let mounts = m.mounts().unwrap();
        assert_eq!(mounts[0].host_path, PathBuf::from("/proj/./bin"));
        assert_eq!(mounts[0].guest_path, "/mnt/app");
        assert!(mounts[0].readonly);
        assert_eq!(mounts[1].host_path, PathBuf::from("/abs"));
        assert_eq!(m.workload(), Workload::Exec("/mnt/app/app".into()));
    }

    #[test]
    fn net_table_builds_a_policy_and_ports() {
        let m = with_dir(
            r#"
            manifest_version = 1
            [app]
            name = "demo"
            [rootfs]
            dockerfile = "Dockerfile"
            [run]
            script = "server.py"
            [run.net]
            all = true
            ports = [8080, "9000-9010"]
        "#,
        );
        let (policy, listen) = m.network().unwrap();
        assert!(matches!(policy, Some(NetworkPolicy::AllowAll)));
        let listen = listen.unwrap();
        assert!(listen.allows(8080));
        assert!(listen.allows(9005));
        assert!(!listen.allows(80));
        assert!(m.needs_build());
    }

    #[test]
    fn rejects_the_inconsistent() {
        let cases: &[(&str, &str)] = &[
            (
                "manifest_version = 2\n[app]\nname = \"d\"\n[rootfs]\npath = \"r\"\n[run]\n",
                "manifest_version 2",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"d\"\n[rootfs]\n[run]\n",
                "exactly one of image, dockerfile or path",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"d\"\n[rootfs]\npath = \"r\"\nimage = \"i\"\n[run]\n",
                "exactly one of image, dockerfile or path",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"d\"\n[rootfs]\npath = \"r\"\n[run]\nscript = \"a\"\nexec = \"b\"\n",
                "more than one of script, exec and guest_exec",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"d\"\n[rootfs]\npath = \"r\"\n[run]\n[run.net]\nports = [80]\n",
                "ports need networking",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"d\"\n[rootfs]\npath = \"r\"\n[run]\n[run.net]\nall = true\nallow = [\"x\"]\n",
                "more than one of all, allow and block",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"d\"\n[rootfs]\npath = \"r\"\n[run]\nbogus = 1\n",
                "bogus",
            ),
            (
                "manifest_version = 1\n[app]\nname = \"\"\n[rootfs]\npath = \"r\"\n[run]\n",
                "name is empty",
            ),
        ];
        for (text, needle) in cases {
            let err = Manifest::parse(text).unwrap_err();
            assert!(err.contains(needle), "for {text:?}: got {err}");
        }
    }

    #[test]
    fn every_template_manifest_validates() {
        use crate::cli::template::{Template, Vars};
        let vars = Vars {
            name: "demo",
            version: "1.0.0",
            registry: "ghcr.io/example/hluk",
            image: "ghcr.io/example/hluk/x:initrd-v1.0.0",
            base: "ghcr.io/example/hluk/x:v1.0.0",
        };
        for t in Template::all().unwrap() {
            let (_, text) = t
                .render(&vars)
                .into_iter()
                .find(|(p, _)| *p == FILE_NAME)
                .unwrap();
            let m = Manifest::parse(&text).unwrap_or_else(|e| panic!("{}: {e}", t.name));
            assert_eq!(m.app.name, "demo");
            assert!(m.run.warm, "{}: templates start warm", t.name);
        }
    }
}
