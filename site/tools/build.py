#!/usr/bin/env python3
"""Build the GitHub Pages site into OUT.

    site/tools/build.py OUT [--bench-from DIR]

Copies the page, its assets and recordings, and writes data/templates.json
from templates/ so the code the page shows is what `hluk init` writes.
--bench-from copies a checkout of the gh-pages branch's dev/bench/ into OUT
for a local preview; on Pages that history is already beside the page.
"""

import argparse
import json
import re
import shutil
import sys
import tomllib
from pathlib import Path

SITE = Path(__file__).resolve().parent.parent
REPO = SITE.parent
# What is copied: everything else under site/ is tooling.
PUBLISHED = ["index.html", "style.css", "app.js", "favicon.svg", "casts"]
# The file a template's project is about, in the order to look for it.
MAIN_FILES = ["main.py", "main.js", "server.py", "server.js", "Program.cs", "main.go",
              "src/main.rs", "main.c", "main.sh", "repl.sh", "main.ps1", "Main.java"]
# Files shown next to the main one: how a project extends its rootfs.
EXTRA_FILES = ["Dockerfile", "requirements.txt", "package.json"]


def fill(text: str, name: str, runtime: str) -> str:
    # The placeholders `hluk init` fills, as a release would fill them, with
    # the version left generic so the page doesn't go stale on every release.
    registry = "ghcr.io/hyperlight-dev/hyperlight-unikraft"
    return (text.replace("{{name}}", name)
                .replace("{{image}}", f"{registry}/{runtime}:initrd-vX.Y.Z")
                .replace("{{base}}", f"{registry}/{runtime}:vX.Y.Z")
                .replace("{{registry}}", registry)
                .replace("{{version}}", "X.Y.Z"))


def strip_comments(manifest: str) -> str:
    # Template manifests explain every key in comments, which is right for
    # the project and too long for a card on the page.
    lines = [l for l in manifest.splitlines() if not l.lstrip().startswith("#")]
    return re.sub(r"\n{3,}", "\n\n", "\n".join(lines)).strip() + "\n"


def templates() -> list[dict]:
    out = []
    for d in sorted((REPO / "templates").iterdir()):
        meta_path = d / "template.toml"
        if not meta_path.is_file():
            continue
        meta = tomllib.loads(meta_path.read_text())["template"]
        runtime = meta["runtime"]
        read = lambda f: fill((d / f).read_text(), "hello", runtime)
        main = next((f for f in MAIN_FILES if (d / f).is_file()), None)
        files = [{"path": main, "text": read(main)}] if main else []
        files += [{"path": f, "text": read(f)} for f in EXTRA_FILES if (d / f).is_file()]
        manifest = strip_comments(read("hluk.toml"))
        files.append({"path": "hluk.toml", "text": manifest})
        out.append({
            "name": d.name,
            "runtime": runtime,
            "tier": meta.get("tier"),
            "description": meta["description"],
            "build": "[build]" in manifest or "dockerfile" in manifest,
            "files": files,
        })
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("out", type=Path)
    ap.add_argument("--bench-from", type=Path)
    args = ap.parse_args()

    if args.out.exists():
        shutil.rmtree(args.out)
    args.out.mkdir(parents=True)
    for item in PUBLISHED:
        src = SITE / item
        if src.is_dir():
            shutil.copytree(src, args.out / item)
        else:
            shutil.copy2(src, args.out / item)
    (args.out / "data").mkdir()
    (args.out / "data" / "templates.json").write_text(json.dumps(templates(), indent=1) + "\n")
    if args.bench_from:
        shutil.copytree(args.bench_from, args.out / "dev" / "bench")
    print(f"site built in {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
