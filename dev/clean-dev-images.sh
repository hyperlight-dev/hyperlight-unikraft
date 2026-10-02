#!/usr/bin/env bash
# Deletes the dev images (dev.yml) of every build but the ones named in KEEP
# from each of the repo's GHCR packages.
#   OWNER, REPO   the packages are <owner>/<repo>/<name>
#   KEEP          the channels to keep, space separated (dev-<sha7> ...)
#   NEWEST        the newest kept commit; run in a checkout with its history
#   DRY_RUN=1     print what would go, delete nothing
# Needs gh (packages:write) and, for multi-platform indexes buildx pushed
# (the kernel), docker logged in to ghcr.io.
#
# A version goes only when every tag it carries is a dev build's
# (dev-<sha7>, initrd-dev-<sha7>, with or without -amd64/-arm64), none is
# kept, and each names a commit before NEWEST: nothing a release tags is
# touched, nor the images of a newer build still on its way to the release.  The untagged platform images
# and attestations of a deleted index go with it, unless a version that stays
# refers to them too.
set -euo pipefail

: "${OWNER:?}" "${REPO:?}" "${KEEP:?}" "${NEWEST:?}"
dev_re='^(initrd-)?dev-([0-9a-f]{7})(-amd64|-arm64)?$'
keep_re="^(initrd-)?($(echo "$KEEP" | tr -s ' ' '|'))(-amd64|-arm64)?$"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

delete() { # package-path id description
    echo "deleting $3"
    [ "${DRY_RUN:-}" = 1 ] || gh api -X DELETE "$1/$2" > /dev/null
}

# The digests an index refers to (none for a single image).
children() { # image-ref
    docker buildx imagetools inspect --raw "$1" | jq -r '.manifests[]?.digest'
}

# The packages are listed rather than named, so a new runtime is cleaned too.
gh api --paginate "/orgs/$OWNER/packages?package_type=container" \
    --jq ".[] | select(.name | startswith(\"$REPO/\")) | .name" > "$tmp/packages"

while read -r pkg; do
    path="/orgs/$OWNER/packages/container/${pkg//\//%2F}/versions"
    image="ghcr.io/${OWNER,,}/$pkg"
    # Every version first: deleting while paginating shifts the pages under
    # the listing and skips versions.
    gh api --paginate "$path" \
        --jq '.[] | [.id, .name, (.metadata.container.tags | join(" "))] | @tsv' > "$tmp/versions"
    : > "$tmp/gone"
    : > "$tmp/stay"
    : > "$tmp/untagged"
    while IFS=$'\t' read -r id digest tags; do
        if [ -z "$tags" ]; then
            printf '%s\t%s\n' "$id" "$digest" >> "$tmp/untagged"
            continue
        fi
        all_dev=true
        kept=false
        for t in $tags; do
            if [[ $t =~ $dev_re ]]; then
                # A commit not in NEWEST's history (newer, or unknown) stays.
                commit=$(git rev-parse -q --verify "${BASH_REMATCH[2]}^{commit}" || true)
                if [ -z "$commit" ] || ! git merge-base --is-ancestor "$commit" "$NEWEST"; then
                    kept=true
                fi
            else
                all_dev=false
            fi
            [[ $t =~ $keep_re ]] && kept=true
        done
        if $all_dev && ! $kept; then
            printf '%s\t%s\t%s\n' "$id" "$digest" "$tags" >> "$tmp/gone"
        else
            echo "$digest" >> "$tmp/stay"
        fi
    done < "$tmp/versions"
    [ -s "$tmp/gone" ] || continue
    # GHCR refuses to delete a package's last tagged version: a package only
    # dev builds ever had (a runtime added and dropped between releases)
    # goes whole.
    if [ ! -s "$tmp/stay" ]; then
        echo "deleting $pkg (only old dev builds)"
        [ "${DRY_RUN:-}" = 1 ] || gh api -X DELETE "/orgs/$OWNER/packages/container/${pkg//\//%2F}" > /dev/null
        continue
    fi

    # Only packages with untagged versions (buildx's indexes) need the
    # indexes read.
    : > "$tmp/orphans"
    if [ -s "$tmp/untagged" ]; then
        : > "$tmp/referenced"
        while read -r digest; do
            children "$image@$digest" >> "$tmp/referenced"
        done < "$tmp/stay"
        while IFS=$'\t' read -r _ digest _; do
            children "$image@$digest"
        done < "$tmp/gone" | sort -u | grep -vxFf "$tmp/referenced" > "$tmp/orphans" || true
    fi

    while IFS=$'\t' read -r id _ tags; do
        delete "$path" "$id" "$pkg $id ($tags)"
    done < "$tmp/gone"
    while IFS=$'\t' read -r id digest; do
        if grep -qxF "$digest" "$tmp/orphans"; then
            delete "$path" "$id" "$pkg $id (untagged $digest)"
        fi
    done < "$tmp/untagged"
done < "$tmp/packages"
