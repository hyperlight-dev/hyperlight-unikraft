# Writing a template

A template is what `hluk init` starts a project from: a directory with a `template.toml` that says what it is, and the files a new project gets. The built-in ones live in [`templates/`](../templates/) and are compiled into `hluk`; any other directory laid out the same way works too, from disk or from GitHub:

```bash
hluk init myapp --template ./my-template                                   # a directory
hluk init myapp --template github.com/OWNER/REPO                           # a repository that is a template
hluk init myapp --template github.com/OWNER/REPO/path/to/template@v1.0     # one directory, at a tag, branch or commit
hluk init myapp --template https://github.com/OWNER/REPO/tree/main/path    # the URL a browser shows
```

A built-in name wins over a directory of the same name, so `./python` reaches a directory called `python`. In a browser URL the segment after `tree/` is taken as the whole ref, so for a branch with a `/` in its name (`feature/x`) use the `@REF` form: `github.com/OWNER/REPO/path@feature/x`. [`examples/templates/word-count`](../examples/templates/word-count/) is a small one to copy from.

## The layout

```text
word-count/
├── template.toml     # what the template is; never copied into the project
├── hluk.toml         # the project's manifest (docs/manifest.md)
├── main.py           # everything else is copied as-is, placeholders filled in
└── data/
    └── hello.txt
```

`template.toml` has one table:

```toml
[template]
description = "Count the words in the files of a host directory"   # one line, shown by `hluk init`
runtime = "python"   # the published rootfs the project runs on
tier = 2             # optional: the runtime's support tier, for ordering
```

`runtime` is one of the images every release publishes: `python`, `python-shell`, `agent`, `node`, `bash`, `c`, `go`, `rust`, `dotnet-aot`, `dotnet-jit`, `powershell`, `quickjs` or `wasmtime`. It decides which image `{{image}}` and `{{base}}` name. Unknown keys are errors.

Every other file is copied into the project with its path kept, and one that is executable stays executable (on Unix), so a `build.sh` the `[build] command` runs works. A file named `X.tmpl` is written as `X`, which is how a template that is itself inside a Rust crate ships a `Cargo.toml` (a real one would make `cargo package` treat the directory as a nested crate and leave it out). Symlinks are skipped, and so are `.git` and `.hluk` (what `hluk build` and `hluk run` leave when you try a template in place); other build output left in the directory is copied like any file, so clean it out first. Files are text: each one goes through placeholder substitution, so a file that is not UTF-8 is refused, and a template is held to 256 files and 4 MiB so a mistyped path does not copy a whole tree.

## Placeholders

Five strings are replaced in every file, and nothing else is touched, so `{{` that belongs to your language (a Go template, a Rust format string) stays as it is:

| Placeholder | Becomes |
|---|---|
| `{{name}}` | the project name (`hluk init myapp` → `myapp`) |
| `{{image}}` | the runnable rootfs of `runtime`: `ghcr.io/hyperlight-dev/hyperlight-unikraft/python:initrd-v0.14.3` |
| `{{base}}` | the same rootfs as a filesystem image to build `FROM`: `…/python:v0.14.3` |
| `{{version}}` | the release those images are pinned to: `0.14.3` |
| `{{registry}}` | the registry they come from: `ghcr.io/hyperlight-dev/hyperlight-unikraft` |

The version is the `hluk` that ran `init` (or its `--image-version`), not the one the template was written for, because a rootfs has to match the `hluk` that runs it. Write `image = "{{image}}"` in the manifest and `FROM {{base}}` in a Dockerfile rather than a fixed tag, and the template keeps working as `hluk` moves on.

## The manifest

The template's `hluk.toml` is the starting point of the project's, and [manifest.md](manifest.md) describes every key. What a template usually sets:

- **What runs:** `[run] script`, the host file the runtime driver runs (or `exec`, or `guest_exec` for a program already in the rootfs).
- **What the guest can reach:** `mounts` for host directories, `env`, and `[run.net]` for the network. Nothing is reachable unless the manifest says so, and a template is a good place to show the smallest set that works.
- **How fast it starts:** `warm = true` (every built-in template sets it) snapshots the booted guest on the first run and restores it after; `warm_exec` runs code once before that snapshot, so imports it does are already done on every restore.
- **Packages:** `[rootfs] dockerfile` with a Dockerfile `FROM {{base}}` that adds them; `hluk build` builds it. The `http-python`, `http-node` and `http-dotnet` templates are the pattern.
- **A compile step:** `[build] command`, run on the host by `hluk build`; the `go`, `rust`, `c` and `dotnet-aot` templates build a binary and mount it into the guest.

A template must have a `hluk.toml`. `hluk init` validates it before writing the project, so a template with a bad one fails at `init` rather than at the first run.

## Trying it

Run the template from its directory before publishing it:

```bash
hluk init /tmp/try --template ./my-template
cd /tmp/try && hluk build && hluk run    # `hluk build` only if the manifest has something to build
```

To try it against a rootfs you built yourself (`just build-rootfs python`), replace the project's `image = …` line with `path = "/path/to/build-elfloader/python-rootfs.cpio"`.

## From GitHub

`--template github.com/OWNER/REPO[/PATH][@REF]` downloads the repository's tarball through the GitHub API (no `git` needed) at REF, or the default branch, and takes the files under PATH. A private repository, or more than the anonymous limit of 60 downloads an hour, needs `GITHUB_TOKEN` in the environment.

A template that is not built in is someone else's code. Rendering it only substitutes the placeholders, so `init` runs nothing. But `hluk build` runs its `[build] command` on your host and its Dockerfile in Docker, and `hluk run` hands the guest the host directories in its `mounts` and the network in `[run.net]`, so `init` prints each of those a template asks for. Read the manifest before building or running, and pin a tag or commit with `@REF` to get the same template every time. `init` checks the manifest before it writes anything, so a template with a bad one leaves no files behind.
