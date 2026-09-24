// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The `hluk init` templates: a project skeleton per runtime, embedded from
//! `templates/` at build time (see `build.rs`).  A template is a directory
//! whose `template.toml` says what it is and whose other files are copied
//! into the new project with a few placeholders filled in.  The same
//! directory can also come from disk or from GitHub ([`Template::resolve`]),
//! which is how a template that is not built in is used.

use std::borrow::Cow;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use serde::Deserialize;

use hyperlight_unikraft::{RUNTIME_SCRATCH_MB, runtime_scratch_mb};

use super::github::GitHubSpec;
use super::manifest;

/// One template as `build.rs` embedded it: its directory name and every
/// file in it, as (`/`-separated relative path, contents).
pub struct TemplateSource {
    pub name: &'static str,
    pub files: &'static [(&'static str, &'static str)],
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

/// The file in a template that describes it, never copied into a project.
const META_FILE: &str = "template.toml";

/// A file named `X.tmpl` is written to the project as `X`.  This is how a
/// template ships a `Cargo.toml`: `cargo package` takes a directory with
/// one for a nested package and leaves it out of the crate.
const TMPL_SUFFIX: &str = ".tmpl";

/// Bounds on a template read from disk or GitHub.  A template is a handful
/// of starter files; these stop a mistyped path (a home directory, a whole
/// monorepo) from being read into memory and written out as a project.
pub const MAX_FILES: usize = 256;
pub const MAX_BYTES: u64 = 4 << 20;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MetaFile {
    template: Meta,
}

/// What `template.toml` says about a template.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// One line for `hluk templates`.
    pub description: String,
    /// The published rootfs this template runs on (`python`, `node`, …):
    /// it names the image `init` pulls and the base its Dockerfile extends.
    pub runtime: String,
    /// The runtime's support tier (docs/guest-support-tiers.md), for
    /// ordering.  Every built-in template has one; another may leave it out.
    pub tier: Option<u8>,
}

/// Where a template came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    BuiltIn,
    Dir(PathBuf),
    GitHub(String),
}

/// One file of a template.  A built-in template borrows its files from
/// the binary; one read from disk or GitHub owns them.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateFile {
    /// `/`-separated, relative to the template directory.
    pub path: Cow<'static, str>,
    pub text: Cow<'static, str>,
    /// Written with its executable bits set, as a `build.sh` that a
    /// `[build] command` runs needs to be.
    pub executable: bool,
}

/// A file of the project a template writes, placeholders filled in.
pub struct ProjectFile<'a> {
    /// `/`-separated, relative to the project directory.
    pub path: &'a str,
    pub text: String,
    pub executable: bool,
}

/// A template with its metadata read.
pub struct Template {
    pub name: Cow<'static, str>,
    pub meta: Meta,
    pub origin: Origin,
    files: Vec<TemplateFile>,
}

/// The values a template's placeholders take: `{{name}}`, `{{version}}`,
/// `{{registry}}`, `{{image}}` (the runnable `:initrd-v<version>` image)
/// and `{{base}}` (the `:v<version>` filesystem image a Dockerfile starts
/// `FROM`).  Nothing else in a file is touched, so `{{` that belongs to the
/// language (a Rust `println!("{{")`, a Go template) is left alone.
pub struct Vars<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub registry: &'a str,
    pub image: &'a str,
    pub base: &'a str,
}

impl Template {
    /// Every embedded template, by tier and then name, so the tier-1
    /// runtimes lead the list.
    pub fn all() -> Result<Vec<Template>, String> {
        let mut all = TEMPLATES
            .iter()
            .map(Template::from_source)
            .collect::<Result<Vec<_>, _>>()?;
        all.sort_by(|a, b| (a.meta.tier, &a.name).cmp(&(b.meta.tier, &b.name)));
        Ok(all)
    }

    /// The built-in template called `name`, if there is one.
    pub fn find(name: &str) -> Result<Option<Template>, String> {
        TEMPLATES
            .iter()
            .find(|t| t.name == name)
            .map(Template::from_source)
            .transpose()
    }

    /// The template `--template` names: a built-in one by name, a
    /// directory on disk, or `github.com/OWNER/REPO[/PATH][@REF]` (a
    /// browser's `https://github.com/OWNER/REPO/tree/REF/PATH` too).  A
    /// built-in name wins over a directory of the same name; `./NAME`
    /// reaches the directory.
    pub fn resolve(spec: &str) -> Result<Template, String> {
        if let Some(github) = GitHubSpec::parse(spec)? {
            let files = github.fetch()?;
            return Template::from_files(
                github.template_name().into(),
                Origin::GitHub(github.to_string()),
                files,
            );
        }
        if let Some(t) = Template::find(spec)? {
            return Ok(t);
        }
        let dir = Path::new(spec);
        if dir.is_dir() {
            return Template::from_dir(dir);
        }
        Err(format!(
            "no template {spec:?}: not a built-in one (`hluk templates` lists them), a \
             directory, or github.com/OWNER/REPO[/PATH][@REF]"
        ))
    }

    /// A template directory on disk.
    pub fn from_dir(dir: &Path) -> Result<Template, String> {
        let canonical = dir
            .canonicalize()
            .map_err(|e| format!("{}: {e}", dir.display()))?;
        let name = canonical
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "template".into());
        let mut files = Files::default();
        read_dir_files(&canonical, &canonical, &mut files)?;
        Template::from_files(name.into(), Origin::Dir(canonical), files.into_vec())
    }

    fn from_source(source: &TemplateSource) -> Result<Template, String> {
        let files = source
            .files
            .iter()
            .map(|&(path, text)| TemplateFile {
                path: Cow::Borrowed(path),
                text: Cow::Borrowed(text),
                executable: false,
            })
            .collect();
        Template::from_files(Cow::Borrowed(source.name), Origin::BuiltIn, files)
    }

    /// A template from its files.
    pub fn from_files(
        name: Cow<'static, str>,
        origin: Origin,
        files: Vec<TemplateFile>,
    ) -> Result<Template, String> {
        let meta = files
            .iter()
            .find(|f| f.path == META_FILE)
            .ok_or_else(|| format!("template {name}: no {META_FILE}"))?;
        let meta: MetaFile =
            toml::from_str(&meta.text).map_err(|e| format!("template {name}: {META_FILE}: {e}"))?;
        let meta = meta.template;
        // Without a manifest the project has nothing for `hluk run` to run.
        if !files.iter().any(|f| f.path == manifest::FILE_NAME) {
            return Err(format!("template {name}: no {}", manifest::FILE_NAME));
        }
        // The runtime names the image init pulls; one that is not published
        // would only fail later, at the pull, with a less useful message.
        if runtime_scratch_mb(&meta.runtime).is_none() {
            let known: Vec<&str> = RUNTIME_SCRATCH_MB.iter().map(|(r, _)| *r).collect();
            return Err(format!(
                "template {name}: {META_FILE}: runtime {:?} is not one hluk publishes ({})",
                meta.runtime,
                known.join(", ")
            ));
        }
        Ok(Template {
            name,
            meta,
            origin,
            files,
        })
    }

    /// The project files this template writes, rendered.
    pub fn render(&self, vars: &Vars) -> Vec<ProjectFile<'_>> {
        self.files
            .iter()
            .filter(|f| f.path != META_FILE)
            .map(|f| ProjectFile {
                path: f.path.strip_suffix(TMPL_SUFFIX).unwrap_or(&f.path),
                text: substitute(&f.text, vars),
                executable: f.executable,
            })
            .collect()
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.meta.description)
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Origin::BuiltIn => write!(f, "built-in"),
            Origin::Dir(dir) => write!(f, "{}", dir.display()),
            Origin::GitHub(spec) => write!(f, "{spec}"),
        }
    }
}

/// A template's files as they are read, held to [`MAX_FILES`] and
/// [`MAX_BYTES`].
#[derive(Default)]
pub struct Files {
    files: Vec<TemplateFile>,
    bytes: u64,
}

impl Files {
    /// Read one file of `len` bytes.  The limits are checked against the
    /// size before anything is read, so a stray large file is refused
    /// without being loaded.  Templates are text: every file goes through
    /// placeholder substitution, so one that is not UTF-8 is refused.
    pub fn push(
        &mut self,
        rel: String,
        len: u64,
        contents: impl Read,
        executable: bool,
    ) -> Result<(), String> {
        self.bytes += len;
        if self.files.len() >= MAX_FILES || self.bytes > MAX_BYTES {
            return Err(format!(
                "the template has more than {MAX_FILES} files or {} MiB: is it the right directory?",
                MAX_BYTES >> 20
            ));
        }
        let mut bytes = Vec::with_capacity(len as usize);
        contents
            .take(len)
            .read_to_end(&mut bytes)
            .map_err(|e| format!("{rel}: {e}"))?;
        let text = String::from_utf8(bytes)
            .map_err(|_| format!("{rel}: not UTF-8 text; a template's files are all text"))?;
        self.files.push(TemplateFile {
            path: rel.into(),
            text: text.into(),
            executable,
        });
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn into_vec(mut self) -> Vec<TemplateFile> {
        self.files.sort();
        self.files
    }
}

/// A path inside a template as the `/`-separated relative path the files
/// are keyed by, or `None` when it could leave the template (`..`, a root,
/// a prefix) or is not UTF-8.
pub fn relative(path: &Path) -> Option<String> {
    let mut parts = Vec::new();
    for c in path.components() {
        match c {
            Component::Normal(part) => parts.push(part.to_str()?),
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

/// Directories never part of a template on disk: version control, and the
/// state `hluk build` and `hluk run` leave when the author tries the
/// template in place.
const SKIPPED_DIRS: &[&str] = &[".git", ".hluk"];

/// Read every regular file under `dir` into `files`.  Symlinks are skipped
/// rather than followed, so nothing outside `root` is read.
fn read_dir_files(root: &Path, dir: &Path, files: &mut Files) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = entry.path();
        let kind = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if kind.is_dir() {
            if !SKIPPED_DIRS.iter().any(|d| entry.file_name() == *d) {
                read_dir_files(root, &path, files)?;
            }
        } else if kind.is_file() {
            let rel = path
                .strip_prefix(root)
                .ok()
                .and_then(relative)
                .ok_or_else(|| format!("{}: not a UTF-8 path", path.display()))?;
            let metadata = entry
                .metadata()
                .map_err(|e| format!("{}: {e}", path.display()))?;
            let file = fs::File::open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            files.push(rel, metadata.len(), file, is_executable(&metadata))?;
        }
    }
    Ok(())
}

#[cfg(unix)]
fn is_executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable(_: &fs::Metadata) -> bool {
    false
}

/// Fill the placeholders in one file.
fn substitute(text: &str, vars: &Vars) -> String {
    text.replace("{{name}}", vars.name)
        .replace("{{version}}", vars.version)
        .replace("{{registry}}", vars.registry)
        .replace("{{image}}", vars.image)
        .replace("{{base}}", vars.base)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VARS: Vars<'static> = Vars {
        name: "demo",
        version: "1.2.3",
        registry: "ghcr.io/example/hluk",
        image: "ghcr.io/example/hluk/python:initrd-v1.2.3",
        base: "ghcr.io/example/hluk/python:v1.2.3",
    };

    #[test]
    fn every_template_has_metadata_and_a_manifest() {
        let all = Template::all().unwrap();
        assert!(!all.is_empty());
        for t in &all {
            let files = t.render(&VARS);
            assert!(
                files.iter().any(|f| f.path == "hluk.toml"),
                "{}: no hluk.toml",
                t.name
            );
            assert!(
                files.iter().all(|f| f.path != META_FILE),
                "{}: template.toml leaked into the project",
                t.name
            );
            assert!(!t.meta.runtime.is_empty(), "{}: no runtime", t.name);
        }
    }

    #[test]
    fn templates_are_ordered_by_tier() {
        let all = Template::all().unwrap();
        assert!(
            all.iter().all(|t| t.meta.tier.is_some()),
            "every built-in template has a tier"
        );
        let tiers: Vec<Option<u8>> = all.iter().map(|t| t.meta.tier).collect();
        let mut sorted = tiers.clone();
        sorted.sort();
        assert_eq!(tiers, sorted);
        assert_eq!(all[0].name, "python", "tier 1 leads the list");
    }

    #[test]
    fn tmpl_suffix_is_stripped() {
        let rust = Template::find("rust").unwrap().unwrap();
        assert!(rust.render(&VARS).iter().any(|f| f.path == "Cargo.toml"));
        for t in Template::all().unwrap() {
            for f in t.render(&VARS) {
                assert!(!f.path.ends_with(TMPL_SUFFIX), "{}: {}", t.name, f.path);
            }
        }
    }

    #[test]
    fn find_is_by_directory_name() {
        assert!(Template::find("python").unwrap().is_some());
        assert!(Template::find("no-such-template").unwrap().is_none());
    }

    #[test]
    fn rendering_fills_every_placeholder() {
        for t in Template::all().unwrap() {
            for f in t.render(&VARS) {
                for var in ["name", "version", "registry", "image", "base"] {
                    assert!(
                        !f.text.contains(&format!("{{{{{var}}}}}")),
                        "{}/{}: {{{{{var}}}}} left unfilled",
                        t.name,
                        f.path
                    );
                }
            }
        }
    }

    #[test]
    fn substitute_leaves_other_braces_alone() {
        assert_eq!(
            substitute("{{name}} says {{ hi }} and {{version}}", &VARS),
            "demo says {{ hi }} and 1.2.3"
        );
    }

    fn write(dir: &Path, rel: &str, text: &[u8]) {
        let path = dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn a_directory_is_a_template() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("greeter");
        write(
            &root,
            META_FILE,
            b"[template]\ndescription = \"d\"\nruntime = \"python\"\n",
        );
        write(&root, "hluk.toml", b"[app]\nname = \"{{name}}\"\n");
        write(&root, "data/words.txt", b"a b c");
        write(&root, ".git/HEAD", b"not part of it");
        write(&root, ".hluk/rootfs.cpio", &[0xff; 16]);

        let t = Template::resolve(root.to_str().unwrap()).unwrap();
        assert_eq!(t.name, "greeter");
        assert_eq!(t.meta.tier, None);
        assert!(matches!(t.origin, Origin::Dir(_)));
        let files = t.render(&VARS);
        let paths: Vec<&str> = files.iter().map(|f| f.path).collect();
        assert_eq!(paths, ["data/words.txt", "hluk.toml"]);
        assert_eq!(files[1].text, "[app]\nname = \"demo\"\n");
    }

    #[test]
    fn a_built_in_name_wins_over_a_directory() {
        // `python` is a built-in name whatever the current directory holds.
        assert_eq!(Template::resolve("python").unwrap().origin, Origin::BuiltIn);
        let err = Template::resolve("no-such-template").err().unwrap();
        assert!(err.contains("hluk templates"), "{err}");
    }

    #[test]
    fn a_template_must_describe_itself_and_name_a_published_runtime() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "main.py", b"");
        let err = Template::from_dir(dir.path()).err().unwrap();
        assert!(err.contains("no template.toml"), "{err}");

        write(
            dir.path(),
            META_FILE,
            b"[template]\nruntime = \"python\"\ndescription = \"d\"\n[extra]\n",
        );
        let err = Template::from_dir(dir.path()).err().unwrap();
        assert!(err.contains("unknown field `extra`"), "{err}");

        write(
            dir.path(),
            META_FILE,
            b"[template]\ndescription = \"d\"\nruntime = \"cobol\"\n",
        );
        let err = Template::from_dir(dir.path()).err().unwrap();
        assert!(err.contains("no hluk.toml"), "{err}");

        write(dir.path(), "hluk.toml", b"");
        let err = Template::from_dir(dir.path()).err().unwrap();
        assert!(err.contains("\"cobol\" is not one hluk publishes"), "{err}");
    }

    #[test]
    fn relative_paths_stay_inside_the_template() {
        assert_eq!(relative(Path::new("a/./b")).as_deref(), Some("a/b"));
        assert_eq!(relative(Path::new("a/../b")), None);
        assert_eq!(relative(Path::new("/etc/passwd")), None);
        assert_eq!(relative(Path::new(".")), None);
    }

    #[test]
    fn files_are_bounded() {
        let mut files = Files::default();
        for i in 0..MAX_FILES {
            files
                .push(format!("f{i}"), 0, std::io::empty(), false)
                .unwrap();
        }
        assert!(
            files
                .push("one-more".into(), 0, std::io::empty(), false)
                .is_err()
        );
        // Refused on the size alone, before anything is read.
        struct Unread;
        impl Read for Unread {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                panic!("a file over the limit was read");
            }
        }
        let mut files = Files::default();
        assert!(
            files
                .push("big".into(), MAX_BYTES + 1, Unread, false)
                .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn the_executable_bit_is_kept() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            META_FILE,
            b"[template]\ndescription = \"d\"\nruntime = \"bash\"\n",
        );
        write(dir.path(), "hluk.toml", b"");
        write(dir.path(), "build.sh", b"true");
        fs::set_permissions(
            dir.path().join("build.sh"),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        let t = Template::from_dir(dir.path()).unwrap();
        let files = t.render(&VARS);
        let exec: Vec<(&str, bool)> = files.iter().map(|f| (f.path, f.executable)).collect();
        assert_eq!(exec, [("build.sh", true), ("hluk.toml", false)]);
    }
}
