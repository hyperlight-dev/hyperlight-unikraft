// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! `hluk init --template github.com/OWNER/REPO[/PATH][@REF]`: a template
//! read out of a GitHub repository.  The repository's tarball comes from the
//! REST API, which needs no git on the host and follows the default branch
//! when no ref is named; only the files under PATH are kept.

use std::fmt;
use std::io::Read;
use std::path::Path;
use std::time::Duration;

use ureq::Agent;
use ureq::http::StatusCode;

use super::template::{Files, TemplateFile, relative};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "github.com/OWNER/REPO[/PATH][@REF]";

/// A template's place on GitHub.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubSpec {
    pub owner: String,
    pub repo: String,
    /// A branch, tag or commit; the default branch when `None`.
    pub git_ref: Option<String>,
    /// The template's directory inside the repository; empty for its root.
    pub path: Vec<String>,
}

impl GitHubSpec {
    /// `spec` as a GitHub template, `None` when it does not name one.
    /// Takes `github.com/OWNER/REPO[/PATH][@REF]`, with or without a
    /// scheme, and the `…/tree/REF/PATH` URL a browser shows for a
    /// directory.  A browser URL cannot say where a REF with a `/` ends
    /// (`tree/feature/x/templates/py`), so its REF is one segment; `@REF`
    /// takes any.
    pub fn parse(spec: &str) -> Result<Option<GitHubSpec>, String> {
        let rest = spec
            .strip_prefix("https://")
            .or_else(|| spec.strip_prefix("http://"))
            .unwrap_or(spec);
        let Some(rest) = rest.strip_prefix("github.com/") else {
            return Ok(None);
        };
        let bad = |why: &str| format!("template {spec:?}: {why}; expected {USAGE}");
        let (rest, mut git_ref) = match rest.split_once('@') {
            Some((rest, r)) => (rest, Some(r.to_string())),
            None => (rest, None),
        };
        let parts: Vec<&str> = rest.split('/').filter(|p| !p.is_empty()).collect();
        let [owner, repo, path @ ..] = parts.as_slice() else {
            return Err(bad("no repository"));
        };
        let repo = repo.strip_suffix(".git").unwrap_or(repo);
        let path = match path {
            ["tree", r, path @ ..] if git_ref.is_none() => {
                git_ref = Some(r.to_string());
                path
            }
            _ => path,
        };
        let name_ok = |s: &str| {
            !s.is_empty()
                && s != "."
                && s != ".."
                && s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        };
        if !name_ok(owner) || !name_ok(repo) {
            return Err(bad("not an owner and repository name"));
        }
        if path.iter().any(|p| *p == "." || *p == "..") {
            return Err(bad("the path must stay inside the repository"));
        }
        // The ref goes into the request path as-is; branch names with `/`
        // are fine there, anything that needs escaping is not a ref.
        if let Some(r) = &git_ref
            && (r.is_empty()
                || r.contains("..")
                || !r
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/')))
        {
            return Err(bad("not a branch, tag or commit after @"));
        }
        Ok(Some(GitHubSpec {
            owner: owner.to_string(),
            repo: repo.to_string(),
            git_ref,
            path: path.iter().map(|p| p.to_string()).collect(),
        }))
    }

    /// What the template is called: its directory, or the repository when
    /// it is the whole repository.
    pub fn template_name(&self) -> String {
        self.path.last().unwrap_or(&self.repo).clone()
    }

    fn tarball_url(&self) -> String {
        let mut url = format!(
            "https://api.github.com/repos/{}/{}/tarball",
            self.owner, self.repo
        );
        if let Some(r) = &self.git_ref {
            url.push('/');
            url.push_str(r);
        }
        url
    }

    /// Download the repository and return the template's files.
    /// `GITHUB_TOKEN`, when set, is sent to the API: it reaches a private
    /// repository and lifts the anonymous rate limit (60 requests an hour).
    pub fn fetch(&self) -> Result<Vec<TemplateFile>, String> {
        let url = self.tarball_url();
        // The API answers with a redirect to codeload, which is followed
        // with the Authorization header dropped (ureq's default); the
        // redirect URL carries its own short-lived token for a private
        // repository.
        let agent: Agent = Agent::config_builder()
            .http_status_as_error(false)
            .timeout_connect(Some(Duration::from_secs(20)))
            .timeout_recv_response(Some(Duration::from_secs(60)))
            .timeout_recv_body(Some(Duration::from_secs(10 * 60)))
            .user_agent(format!("hluk/{VERSION}"))
            .build()
            .into();
        let mut req = agent
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28");
        let token = std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty());
        if let Some(token) = &token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }
        eprintln!("Fetching template {self}");
        let response = req.call().map_err(|e| format!("GET {url}: {e}"))?;
        match response.status() {
            StatusCode::OK => {}
            StatusCode::NOT_FOUND => {
                let hint = if token.is_some() {
                    ""
                } else {
                    " (a private repository needs GITHUB_TOKEN)"
                };
                return Err(format!(
                    "{self}: no such repository, or no such branch, tag or commit{hint}"
                ));
            }
            // A token that has expired or lost access fails even a public
            // repository, which would download without it.
            StatusCode::UNAUTHORIZED => {
                return Err(format!(
                    "{self}: GitHub rejected GITHUB_TOKEN (HTTP 401); renew it, or unset it for a \
                     public repository"
                ));
            }
            StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => {
                return Err(format!(
                    "{self}: GitHub refused the download (HTTP {}); the anonymous API limit is \
                     60 requests an hour, set GITHUB_TOKEN to lift it",
                    response.status()
                ));
            }
            status => return Err(format!("GET {url}: HTTP {status}")),
        }
        let body = response.into_body().into_reader();
        self.extract(flate2::read::GzDecoder::new(body))
    }

    /// The template's files out of a repository tarball: every regular file
    /// under `path`, relative to it.  GitHub's tarballs put everything
    /// under one `OWNER-REPO-SHA/` directory, which is dropped.
    fn extract(&self, tarball: impl Read) -> Result<Vec<TemplateFile>, String> {
        let mut files = Files::default();
        let mut archive = tar::Archive::new(tarball);
        let entries = archive
            .entries()
            .map_err(|e| format!("{self}: not a tarball: {e}"))?;
        for entry in entries {
            let mut entry = entry.map_err(|e| format!("{self}: reading the tarball: {e}"))?;
            if !entry.header().entry_type().is_file() {
                continue;
            }
            let path = entry
                .path()
                .map_err(|e| format!("{self}: reading the tarball: {e}"))?
                .into_owned();
            let Some(rel) = self.inside(&path) else {
                continue;
            };
            let len = entry.header().size().unwrap_or(0);
            let executable = entry.header().mode().is_ok_and(|m| m & 0o111 != 0);
            files.push(rel, len, &mut entry, executable)?;
        }
        if files.is_empty() {
            return Err(format!(
                "{self}: no files at {:?} in the repository",
                self.path.join("/")
            ));
        }
        Ok(files.into_vec())
    }

    /// A tarball entry's path relative to the template directory, `None`
    /// when it is outside it.
    fn inside(&self, path: &Path) -> Option<String> {
        let rel = relative(path)?;
        let (_top, rel) = rel.split_once('/')?;
        let mut rel = rel;
        for part in &self.path {
            rel = rel.strip_prefix(part.as_str())?.strip_prefix('/')?;
        }
        Some(rel.to_string())
    }
}

impl fmt::Display for GitHubSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "github.com/{}/{}", self.owner, self.repo)?;
        for part in &self.path {
            write!(f, "/{part}")?;
        }
        if let Some(r) = &self.git_ref {
            write!(f, "@{r}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(spec: &str) -> GitHubSpec {
        GitHubSpec::parse(spec).unwrap().unwrap()
    }

    #[test]
    fn parses_the_short_form() {
        let s = parse("github.com/octo/hello");
        assert_eq!((s.owner.as_str(), s.repo.as_str()), ("octo", "hello"));
        assert_eq!(s.git_ref, None);
        assert!(s.path.is_empty());
        assert_eq!(s.template_name(), "hello");

        let s = parse("https://github.com/octo/hello.git/templates/py@v1.2");
        assert_eq!(s.repo, "hello");
        assert_eq!(s.path, ["templates", "py"]);
        assert_eq!(s.git_ref.as_deref(), Some("v1.2"));
        assert_eq!(s.template_name(), "py");
        assert_eq!(s.to_string(), "github.com/octo/hello/templates/py@v1.2");
    }

    #[test]
    fn parses_a_browser_url() {
        let s = parse("https://github.com/octo/hello/tree/main/examples/templates/x/");
        assert_eq!(s.git_ref.as_deref(), Some("main"));
        assert_eq!(s.path, ["examples", "templates", "x"]);
        // An explicit @REF wins, and `tree` is then a directory name.
        let s = parse("github.com/octo/hello/tree/x@dev");
        assert_eq!(s.git_ref.as_deref(), Some("dev"));
        assert_eq!(s.path, ["tree", "x"]);
    }

    #[test]
    fn leaves_other_specs_alone_and_refuses_bad_ones() {
        assert_eq!(GitHubSpec::parse("python").unwrap(), None);
        assert_eq!(GitHubSpec::parse("./templates/x").unwrap(), None);
        assert_eq!(GitHubSpec::parse("gitlab.com/a/b").unwrap(), None);
        for bad in [
            "github.com/octo",
            "github.com/octo/hello/../x",
            "github.com/octo/hello@",
            "github.com/octo/hello@a b",
            "github.com/octo/hello@../x",
            "github.com/oc to/hello",
        ] {
            assert!(GitHubSpec::parse(bad).is_err(), "{bad} parsed");
        }
    }

    /// A gzip'd tarball laid out the way GitHub's are.
    fn tarball(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut builder = tar::Builder::new(Vec::new());
        for (path, data) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder.append_data(&mut header, path, *data).unwrap();
        }
        let tar = builder.into_inner().unwrap();
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        std::io::Write::write_all(&mut gz, &tar).unwrap();
        gz.finish().unwrap()
    }

    #[test]
    fn extracts_only_the_template_directory() {
        let tgz = tarball(&[
            ("octo-hello-abc123/README.md", b"repo readme"),
            ("octo-hello-abc123/t/x/template.toml", b"[template]"),
            ("octo-hello-abc123/t/x/src/main.py", b"print(1)"),
            ("octo-hello-abc123/t/xy/other.py", b"not this"),
            ("octo-hello-abc123/t/other.py", b"nor this"),
        ]);
        let spec = parse("github.com/octo/hello/t/x");
        let files = spec
            .extract(flate2::read::GzDecoder::new(tgz.as_slice()))
            .unwrap();
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_ref()).collect();
        assert_eq!(paths, ["src/main.py", "template.toml"]);
    }

    #[test]
    fn a_missing_directory_or_binary_file_is_an_error() {
        let tgz = tarball(&[("octo-hello-abc/a.txt", b"a")]);
        let err = parse("github.com/octo/hello/nope")
            .extract(flate2::read::GzDecoder::new(tgz.as_slice()))
            .unwrap_err();
        assert!(err.contains("no files"), "{err}");

        let tgz = tarball(&[("octo-hello-abc/logo.png", &[0x89, 0x50, 0xff, 0xfe])]);
        let err = parse("github.com/octo/hello")
            .extract(flate2::read::GzDecoder::new(tgz.as_slice()))
            .unwrap_err();
        assert!(err.contains("not UTF-8"), "{err}");
    }
}
