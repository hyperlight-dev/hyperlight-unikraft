// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

//! The project workflow end to end: `hluk init` writes a project from a
//! template, `hluk run` runs it, warm and cold.  The template's manifest is
//! pointed at the locally built python rootfs so the tests need no
//! registry, and the cache is a temporary directory so they touch nothing
//! of the developer's.

mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::{require_rootfs, temp_dir};

fn hluk(cwd: &Path, cache: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_hluk"))
        .args(args)
        .current_dir(cwd)
        .env("HLUK_CACHE_DIR", cache)
        .output()
        .expect("failed to run hluk")
}

/// Both streams, with the colour codes tracing writes taken out so a
/// `key=value` field can be matched.
fn text(output: &Output) -> String {
    let raw = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // ESC [ ... m
            for c in chars.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// The complete warm snapshots in `cache`.
fn snapshots(cache: &Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(cache.join("snapshots"))
        .map(|d| {
            d.filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_dir() && p.extension().is_none())
                .collect()
        })
        .unwrap_or_default()
}

/// The stamp of the one snapshot in `cache`.
fn read_stamp(cache: &Path) -> String {
    let dirs = snapshots(cache);
    assert_eq!(dirs.len(), 1, "expected one snapshot, found {dirs:?}");
    std::fs::read_to_string(dirs[0].join("stamp")).unwrap()
}

/// `hluk init` the python template into `dir` and swap its published
/// image for the local rootfs, as a manifest with `[rootfs] path` would.
fn init_python(root: &Path, cache: &Path, dir: &str) -> std::path::PathBuf {
    init_local(root, cache, dir, "python", "python")
}

/// `hluk init` `template` into `dir`, pointed at the local `runtime` rootfs.
fn init_local(
    root: &Path,
    cache: &Path,
    dir: &str,
    template: &str,
    runtime: &str,
) -> std::path::PathBuf {
    let out = hluk(
        root,
        cache,
        &["init", dir, "--template", template, "--no-pull"],
    );
    assert!(out.status.success(), "init failed: {}", text(&out));
    let project = root.join(dir);
    let manifest = project.join("hluk.toml");
    let rootfs = require_rootfs(runtime);
    let rewritten: String = std::fs::read_to_string(&manifest)
        .unwrap()
        .lines()
        .map(|line| {
            if line.starts_with("image = ") {
                // A literal string, so a Windows path's backslashes stand.
                format!("path = '{}'", rootfs.display())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&manifest, rewritten).unwrap();
    project
}

#[test]
fn init_writes_the_template_and_run_runs_it_warm_and_cold() {
    let root = temp_dir("project");
    let cache = temp_dir("project-cache");
    let project = init_python(root.path(), cache.path(), "hello");

    for file in ["hluk.toml", "main.py", ".gitignore"] {
        assert!(project.join(file).is_file(), "{file} was not written");
    }
    let main = std::fs::read_to_string(project.join("main.py")).unwrap();
    assert!(
        main.contains("Hello from hello!"),
        "name not rendered: {main}"
    );

    // First run: boots, and being `warm = true`, saves the snapshot.
    let out = hluk(&project, cache.path(), &["run"]);
    assert!(out.status.success(), "run failed: {}", text(&out));
    assert!(
        text(&out).contains("Hello from hello!"),
        "got: {}",
        text(&out)
    );
    assert_eq!(
        snapshots(cache.path()).len(),
        1,
        "one warm snapshot in the cache"
    );

    // Second run: restores it.
    let out = hluk(&project, cache.path(), &["run", "--log-level", "info"]);
    assert!(out.status.success(), "warm run failed: {}", text(&out));
    let t = text(&out);
    assert!(t.contains("Hello from hello!"), "got: {t}");
    assert!(t.contains("restored=true"), "expected a restore: {t}");

    // Cold ignores the snapshot without touching it.
    let out = hluk(
        &project,
        cache.path(),
        &["run", "--cold", "--log-level", "info"],
    );
    assert!(out.status.success(), "cold run failed: {}", text(&out));
    let t = text(&out);
    assert!(t.contains("restored=false"), "expected a fresh boot: {t}");
    assert_eq!(
        snapshots(cache.path()).len(),
        1,
        "a cold run leaves the snapshot alone"
    );

    // From elsewhere, by path.
    let out = hluk(root.path(), cache.path(), &["run", "-f", "hello"]);
    assert!(out.status.success(), "run -f failed: {}", text(&out));
}

#[test]
fn other_settings_add_a_snapshot_and_a_rebuilt_rootfs_replaces_them() {
    let root = temp_dir("project-stale");
    let cache = temp_dir("project-stale-cache");
    let project = init_python(root.path(), cache.path(), "app");
    // A private copy of the rootfs, so it can be "rebuilt" below.
    let rootfs = project.join("rootfs.cpio");
    std::fs::copy(require_rootfs("python"), &rootfs).unwrap();
    let manifest = project.join("hluk.toml");
    let base: String = std::fs::read_to_string(&manifest)
        .unwrap()
        .lines()
        .map(|line| {
            if line.starts_with("path = ") {
                "path = 'rootfs.cpio'".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&manifest, &base).unwrap();
    assert!(hluk(&project, cache.path(), &["run"]).status.success());
    assert_eq!(snapshots(cache.path()).len(), 1);

    // Other settings for the same rootfs are another guest: a second
    // snapshot, the first kept for whoever still runs the old settings.
    std::fs::write(
        &manifest,
        base.replace("warm = true", "warm = true\nscratch_mb = 320"),
    )
    .unwrap();
    let out = hluk(&project, cache.path(), &["run", "--log-level", "info"]);
    assert!(text(&out).contains("restored=false"), "{}", text(&out));
    assert_eq!(snapshots(cache.path()).len(), 2);

    // A rebuilt rootfs (same path, new file) supersedes every snapshot of
    // the old one.
    std::fs::File::options()
        .write(true)
        .open(&rootfs)
        .unwrap()
        .set_modified(std::time::SystemTime::now())
        .unwrap();
    let out = hluk(&project, cache.path(), &["run", "--log-level", "info"]);
    assert!(text(&out).contains("restored=false"), "{}", text(&out));
    assert!(read_stamp(cache.path()).contains("scratch_mb=320"));
}

#[test]
fn init_refuses_to_overwrite_without_force() {
    let root = temp_dir("project-twice");
    let cache = temp_dir("project-twice-cache");
    init_python(root.path(), cache.path(), "twice");
    let out = hluk(
        root.path(),
        cache.path(),
        &["init", "twice", "--template", "python", "--no-pull"],
    );
    assert!(!out.status.success());
    assert!(text(&out).contains("--force"), "got: {}", text(&out));
    let out = hluk(
        root.path(),
        cache.path(),
        &[
            "init",
            "twice",
            "--template",
            "python",
            "--no-pull",
            "--force",
        ],
    );
    assert!(out.status.success(), "{}", text(&out));
}

#[test]
fn init_adds_to_an_existing_gitignore() {
    let root = temp_dir("project-gitignore");
    let cache = temp_dir("project-gitignore-cache");
    let project = root.path().join("existing");
    std::fs::create_dir_all(&project).unwrap();
    std::fs::write(project.join(".gitignore"), "target/\n").unwrap();
    let out = hluk(
        root.path(),
        cache.path(),
        &["init", "existing", "--template", "python", "--no-pull"],
    );
    assert!(out.status.success(), "{}", text(&out));
    let ignore = std::fs::read_to_string(project.join(".gitignore")).unwrap();
    assert!(
        ignore.starts_with("target/\n"),
        "the existing lines stay first: {ignore:?}"
    );
    assert!(
        ignore.contains(".hluk/"),
        "the template's lines are added: {ignore:?}"
    );
}

#[test]
fn init_needs_a_template_and_a_directory_without_a_terminal() {
    let root = temp_dir("project-args");
    let cache = temp_dir("project-args-cache");
    let out = hluk(root.path(), cache.path(), &["init", "x", "--no-pull"]);
    assert!(!out.status.success());
    assert!(text(&out).contains("--template"), "got: {}", text(&out));
    let out = hluk(
        root.path(),
        cache.path(),
        &["init", "--template", "python", "--no-pull"],
    );
    assert!(!out.status.success());
    assert!(text(&out).contains("directory"), "got: {}", text(&out));
    let out = hluk(
        root.path(),
        cache.path(),
        &["init", "x", "--template", "nope", "--no-pull"],
    );
    assert!(!out.status.success());
    assert!(text(&out).contains("hluk templates"), "got: {}", text(&out));
    let out = hluk(
        root.path(),
        cache.path(),
        &["init", "1x", "--template", "python", "--no-pull"],
    );
    assert!(!out.status.success());
    assert!(text(&out).contains("--name"), "got: {}", text(&out));
}

#[test]
fn run_without_a_manifest_points_at_init() {
    let root = temp_dir("project-none");
    let cache = temp_dir("project-none-cache");
    let out = hluk(root.path(), cache.path(), &["run"]);
    assert!(!out.status.success());
    assert!(text(&out).contains("hluk init"), "got: {}", text(&out));
}

#[test]
fn templates_lists_every_tier_one_runtime_first() {
    let root = temp_dir("project-templates");
    let cache = temp_dir("project-templates-cache");
    let out = hluk(root.path(), cache.path(), &["templates"]);
    assert!(out.status.success());
    let t = String::from_utf8_lossy(&out.stdout);
    let mut lines = t.lines();
    assert!(lines.next().unwrap().starts_with("NAME"));
    assert!(
        lines.next().unwrap().starts_with("python "),
        "python leads: {t}"
    );
    for name in [
        "node",
        "bash",
        "bash-repl",
        "python-shell",
        "dotnet",
        "go",
        "rust",
        "c",
        "http-python",
    ] {
        assert!(
            t.lines().any(|l| l.starts_with(&format!("{name} "))),
            "{name} missing from:\n{t}"
        );
    }
}

#[test]
fn init_takes_a_template_directory() {
    let root = temp_dir("project-dir-template");
    let cache = temp_dir("project-dir-template-cache");
    let template = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/templates/word-count");
    let project = init_local(
        root.path(),
        cache.path(),
        "counted",
        template.to_str().unwrap(),
        "python",
    );
    assert!(
        !project.join("template.toml").exists(),
        "template.toml is not part of the project"
    );
    assert!(project.join("data/about.txt").is_file());

    let out = hluk(&project, cache.path(), &["run"]);
    assert!(out.status.success(), "run failed: {}", text(&out));
    assert!(text(&out).contains("40  total"), "got: {}", text(&out));
}

/// `hluk` with `input` on its stdin.
fn hluk_stdin(cwd: &Path, cache: &Path, args: &[&str], input: &[u8]) -> Output {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = Command::new(env!("CARGO_BIN_EXE_hluk"))
        .args(args)
        .current_dir(cwd)
        .env("HLUK_CACHE_DIR", cache)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run hluk");
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn the_python_shell_template_runs_shell_commands() {
    let root = temp_dir("project-python-shell");
    let cache = temp_dir("project-python-shell-cache");
    let project = init_local(
        root.path(),
        cache.path(),
        "pysh",
        "python-shell",
        "python-shell",
    );
    let out = hluk(&project, cache.path(), &["run"]);
    assert!(out.status.success(), "run failed: {}", text(&out));
    let t = text(&out);
    assert!(t.contains("Hello from pysh!"), "got: {t}");
    assert!(t.contains("/bin/sh"), "`which sh` did not run: {t}");
}

#[test]
fn the_bash_repl_template_runs_each_line_until_end_of_input() {
    let root = temp_dir("project-bash-repl");
    let cache = temp_dir("project-bash-repl-cache");
    let project = init_local(root.path(), cache.path(), "repl", "bash-repl", "bash");
    // State carries from line to line, and a last line with no newline
    // still runs before the loop stops at end of input.
    // Each check looks for output the command's own text does not contain,
    // since the input is echoed back next to it.
    let input = b"X=4\ncd /etc\necho x=$X\npwd\necho $((6 * 7))";
    let out = hluk_stdin(&project, cache.path(), &["run"], input);
    assert!(out.status.success(), "run failed: {}", text(&out));
    let t = text(&out);
    assert!(t.contains("x=4"), "a variable did not carry over: {t}");
    assert!(
        t.lines().any(|l| l.trim() == "/etc"),
        "cd did not carry over: {t}"
    );
    assert!(
        t.contains("42"),
        "the unterminated last line did not run: {t}"
    );
}

#[test]
fn build_runs_the_command_in_the_project_directory() {
    let root = temp_dir("project-build");
    let cache = temp_dir("project-build-cache");
    let project = init_python(root.path(), cache.path(), "built");
    let manifest = project.join("hluk.toml");
    // The same line is valid for sh and cmd.
    let command = "echo built > built.txt";
    let edited = format!(
        "{}\n[build]\ncommand = \"{command}\"\n",
        std::fs::read_to_string(&manifest).unwrap()
    );
    std::fs::write(&manifest, edited).unwrap();
    let out = hluk(&project, cache.path(), &["build"]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(
        project.join("built.txt").is_file(),
        "the command did not run in the project"
    );

    // `run --build` builds first, then runs.
    std::fs::remove_file(project.join("built.txt")).unwrap();
    let out = hluk(&project, cache.path(), &["run", "--build"]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(project.join("built.txt").is_file());
    assert!(text(&out).contains("Hello from built!"));
}

#[test]
fn warm_exec_runs_before_the_snapshot_and_its_state_is_restored() {
    let root = temp_dir("project-warm-exec");
    let cache = temp_dir("project-warm-exec-cache");
    let project = init_python(root.path(), cache.path(), "warmed");
    let manifest = project.join("hluk.toml");
    let edited = std::fs::read_to_string(&manifest).unwrap().replace(
        "warm = true",
        "warm = true\nwarm_exec = \"import sys; sys.hluk_warmed = 'yes'\"",
    );
    std::fs::write(&manifest, edited).unwrap();
    std::fs::write(
        project.join("main.py"),
        "import sys\nprint('warmed:', getattr(sys, 'hluk_warmed', 'no'))\n",
    )
    .unwrap();

    // The first run boots, runs warm_exec, snapshots, then runs the script
    // in the same guest: the marker is there.
    let out = hluk(&project, cache.path(), &["run"]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(text(&out).contains("warmed: yes"), "{}", text(&out));
    // The restore brings the marker back without running warm_exec again.
    let out = hluk(&project, cache.path(), &["run", "--log-level", "info"]);
    assert!(out.status.success(), "{}", text(&out));
    let t = text(&out);
    assert!(
        t.contains("warmed: yes") && t.contains("restored=true"),
        "{t}"
    );
    assert!(
        !t.contains("warm_exec done"),
        "warm_exec ran on a restore: {t}"
    );
    // A cold run never runs it.
    let out = hluk(&project, cache.path(), &["run", "--cold"]);
    assert!(text(&out).contains("warmed: no"), "{}", text(&out));
    // Changing it invalidates the snapshot.
    let edited = std::fs::read_to_string(&manifest)
        .unwrap()
        .replace("'yes'", "'again'");
    std::fs::write(&manifest, edited).unwrap();
    let out = hluk(&project, cache.path(), &["run", "--log-level", "info"]);
    let t = text(&out);
    assert!(
        t.contains("warmed: again") && t.contains("restored=false"),
        "{t}"
    );
}

#[test]
fn an_explicit_initrd_runs_cold_unless_asked_to_warm() {
    let root = temp_dir("project-initrd");
    let cache = temp_dir("project-initrd-cache");
    let rootfs = require_rootfs("python");
    let rootfs = rootfs.to_str().unwrap();
    let out = hluk(
        root.path(),
        cache.path(),
        &["run", "--initrd", rootfs, "--exec", "print('cold')"],
    );
    assert!(out.status.success(), "{}", text(&out));
    assert!(text(&out).contains("cold"));
    assert!(
        snapshots(cache.path()).is_empty(),
        "a plain run saves no snapshot"
    );

    let out = hluk(
        root.path(),
        cache.path(),
        &[
            "run",
            "--initrd",
            rootfs,
            "--warm",
            "--exec",
            "print('warm')",
        ],
    );
    assert!(out.status.success(), "{}", text(&out));
    assert_eq!(snapshots(cache.path()).len(), 1);
    let out = hluk(
        root.path(),
        cache.path(),
        &[
            "run",
            "--initrd",
            rootfs,
            "--warm",
            "--log-level",
            "info",
            "--exec",
            "print('again')",
        ],
    );
    let t = text(&out);
    assert!(out.status.success(), "{t}");
    assert!(t.contains("restored=true") && t.contains("again"), "{t}");
}

#[test]
fn run_flags_stand_in_for_the_manifests_keys() {
    let root = temp_dir("project-flags");
    let cache = temp_dir("project-flags-cache");
    let project = init_python(root.path(), cache.path(), "flagged");
    // A script on the command line replaces the manifest's, and --env adds.
    std::fs::write(
        project.join("other.py"),
        "import os\nprint('other', os.environ.get('GREETING', '?'))\n",
    )
    .unwrap();
    let out = hluk(
        &project,
        cache.path(),
        &["run", "--cold", "--env", "GREETING=hi", "other.py"],
    );
    assert!(out.status.success(), "{}", text(&out));
    assert!(text(&out).contains("other hi"), "{}", text(&out));
}
