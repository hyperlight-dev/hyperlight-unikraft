# The project site

The source of <https://hyperlight-dev.github.io/hyperlight-unikraft>. It is static HTML, CSS and JavaScript with no build step beyond a copy and a generated `data/templates.json`: `.github/workflows/site.yml` publishes it to the root of the `gh-pages` branch on every push to `main` that touches `site/` or `templates/`. The CI benchmark job owns `dev/bench/` on that branch, and the page reads it.

| Path | What it is |
|---|---|
| `index.html`, `style.css`, `app.js` | The page |
| `hyperlight-logo.png` | The Hyperlight logo (header and favicon), the same file as hyperlight.org's |
| `casts/*.cast` | Terminal recordings (asciinema v2), played by [asciinema-player](https://github.com/asciinema/asciinema-player) |
| `record/` | The [demo-magic](https://github.com/paxtonhare/demo-magic) scenes behind the recordings, and `record.sh` |
| `tools/build.py` | Copies the page and writes `data/templates.json` from `templates/` |
| `tools/density.sh` | Measures the memory of N concurrent Python sandboxes |

## Where the numbers come from

| Section | Source | Updated |
|---|---|---|
| Startup | `dev/bench/linux/python/data.js` on `gh-pages`, the latest push to `main` | On every push, by CI |
| Density | `site/tools/density.sh 1000 20`, run by hand | By hand: edit the figures in `index.html` |

## Preview

```sh
just site-serve        # http://localhost:8000
```

## Re-record the demos

```sh
HLUK_VERSION=vX.Y.Z HLUK_INSTALL_DIR=/tmp/hluk-rel sh install.sh
PATH=/tmp/hluk-rel:$PATH just site-record              # or: just site-record hero
```

Use a release build of `hluk` so the recordings show release image tags. Recording needs a Linux host with KVM, `asciinema` 2.x and `pv`; the `languages` scene needs Go and Rust, and `dockerfile` needs Docker and curl. Each scene runs in an empty cache, so first runs are real cold boots. A scene with a failing command keeps the committed cast.

When a release adds or removes templates, update `UNRELEASED` in `app.js`. When Java ships in a release, add it back to the runtimes named in the hero and the Embed section of `index.html`.
