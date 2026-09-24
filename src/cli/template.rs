// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The `hluk init` templates: a project skeleton per runtime, embedded from
//! `templates/` at build time (see `build.rs`).  A template is a directory
//! whose `template.toml` says what it is and whose other files are copied
//! into the new project with a few placeholders filled in.

use std::fmt;

use serde::Deserialize;

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

#[derive(Deserialize)]
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
    /// The runtime's support tier (docs/guest-support-tiers.md), for ordering.
    pub tier: u8,
}

/// A template with its metadata read.
pub struct Template {
    pub name: &'static str,
    pub meta: Meta,
    files: &'static [(&'static str, &'static str)],
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
        all.sort_by(|a, b| (a.meta.tier, a.name).cmp(&(b.meta.tier, b.name)));
        Ok(all)
    }

    /// The template called `name`, if there is one.
    pub fn find(name: &str) -> Result<Option<Template>, String> {
        TEMPLATES
            .iter()
            .find(|t| t.name == name)
            .map(Template::from_source)
            .transpose()
    }

    fn from_source(source: &TemplateSource) -> Result<Template, String> {
        let (_, meta) = source
            .files
            .iter()
            .find(|(path, _)| *path == META_FILE)
            .ok_or_else(|| format!("template {}: no {META_FILE}", source.name))?;
        let meta: MetaFile = toml::from_str(meta)
            .map_err(|e| format!("template {}: {META_FILE}: {e}", source.name))?;
        Ok(Template {
            name: source.name,
            meta: meta.template,
            files: source.files,
        })
    }

    /// The project files this template writes, rendered: (`/`-separated
    /// relative path, contents).
    pub fn render(&self, vars: &Vars) -> Vec<(&'static str, String)> {
        self.files
            .iter()
            .filter(|(path, _)| *path != META_FILE)
            .map(|(path, text)| {
                let path = path.strip_suffix(TMPL_SUFFIX).unwrap_or(path);
                (path, substitute(text, vars))
            })
            .collect()
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.meta.description)
    }
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
                files.iter().any(|(p, _)| *p == "hluk.toml"),
                "{}: no hluk.toml",
                t.name
            );
            assert!(
                files.iter().all(|(p, _)| *p != META_FILE),
                "{}: template.toml leaked into the project",
                t.name
            );
            assert!(!t.meta.runtime.is_empty(), "{}: no runtime", t.name);
        }
    }

    #[test]
    fn templates_are_ordered_by_tier() {
        let all = Template::all().unwrap();
        let tiers: Vec<u8> = all.iter().map(|t| t.meta.tier).collect();
        let mut sorted = tiers.clone();
        sorted.sort();
        assert_eq!(tiers, sorted);
        assert_eq!(all[0].name, "python", "tier 1 leads the list");
    }

    #[test]
    fn tmpl_suffix_is_stripped() {
        let rust = Template::find("rust").unwrap().unwrap();
        assert!(rust.render(&VARS).iter().any(|(p, _)| *p == "Cargo.toml"));
        for t in Template::all().unwrap() {
            for (p, _) in t.render(&VARS) {
                assert!(!p.ends_with(TMPL_SUFFIX), "{}: {p}", t.name);
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
            for (path, text) in t.render(&VARS) {
                for var in ["name", "version", "registry", "image", "base"] {
                    assert!(
                        !text.contains(&format!("{{{{{var}}}}}")),
                        "{}/{path}: {{{{{var}}}}} left unfilled",
                        t.name
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
}
