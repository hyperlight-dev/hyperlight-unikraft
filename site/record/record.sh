#!/usr/bin/env bash
# Record the site's terminal demos into site/casts/<scene>.cast.
#
#   site/record/record.sh                  # every scene
#   site/record/record.sh hero calls       # some of them
#
# Needs a Linux host with KVM, hluk on PATH (a release build, so the images
# shown are release tags), asciinema 2.x, pv, Docker and curl (dockerfile),
# and Go and Rust (languages). Each scene runs in a fresh directory
# with an empty HLUK_CACHE_DIR, so `hluk init` pulls its image on camera and
# a "first run" really is a cold boot.
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
casts=$here/../casts
repo=$here/../..

for tool in hluk asciinema pv; do
  command -v "$tool" >/dev/null || { echo "record.sh: $tool is not on PATH" >&2; exit 1; }
done

scenes=("$@")
[ ${#scenes[@]} -gt 0 ] || scenes=(hero languages dockerfile calls)

# Work the scene can't show without being slow or noisy, done off camera.
prepare() {
  case $1 in
    languages)
      for lang in python node dotnet go rust quickjs; do
        hluk init "hello-$lang" --template "$lang" >/dev/null
        if grep -q "^\[build\]" "hello-$lang/hluk.toml"; then (cd "hello-$lang" && hluk build >/dev/null); fi
        hluk run -f "hello-$lang" >/dev/null   # saves the warm snapshot
      done ;;
    calls)
      cp "$repo/examples/python/handler.py" .
      # The comment block is for readers of the repo; the demo shows the code.
      sed -i '/^#/d; /./,$!d' handler.py
      hluk run --runtime python --exec pass >/dev/null ;;
  esac
}

for scene in "${scenes[@]}"; do
  [ -f "$here/$scene.sh" ] || { echo "record.sh: no scene $scene" >&2; exit 1; }
  # A fixed path rather than mktemp's, since `hluk init` prints the cache
  # path on camera.
  tmp=${TMPDIR:-/tmp}/hluk-demo
  rm -rf "$tmp"
  trap 'rm -rf "$tmp"' EXIT
  # The prompt shows the directory's name.
  work=$tmp/demo
  mkdir -p "$work"
  export HLUK_CACHE_DIR=$tmp/cache
  (cd "$work" && prepare "$scene")
  echo "recording $scene"
  # Narrower scenes sit in half-width columns on the page; fewer columns
  # keep their text legible there.
  case $scene in
    hero) cols=96 rows=24 ;;
    languages) cols=80 rows=22 ;;
    calls) cols=64 rows=14 ;;
    *) cols=84 rows=26 ;;
  esac
  export SCENE_FAILED=$tmp/failed
  (cd "$work" && asciinema rec --quiet --idle-time-limit 1.5 \
     --cols "$cols" --rows "$rows" --title "$scene" \
     --command "bash $here/$scene.sh" "$tmp/$scene.cast")
  if [ -s "$SCENE_FAILED" ]; then
    echo "record.sh: $scene failed, keeping the old cast:" >&2
    cat "$SCENE_FAILED" >&2
    exit 1
  fi
  mv "$tmp/$scene.cast" "$casts/$scene.cast"
  rm -rf "$tmp"
done
