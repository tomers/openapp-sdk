#!/usr/bin/env bash
# Create bind-mounted Go Docker caches as the invoking user.
#
# GOCACHE/GOMODCACHE live on the repo bind mount (see packages/sdk/docker/compose.yaml).
# If those directories are missing, a root container (or dockerd creating a mount
# point) materializes them as uid 0, and the host user cannot delete them.
set -euo pipefail

pkg="${1:?package directory}"
uid="$(id -u)"

owner_uid() {
  python3 -c 'import os, sys; print(os.stat(sys.argv[1]).st_uid)' "$1"
}

ensure_dir() {
  local path="$1"
  if [[ -e "$path" ]]; then
    if [[ ! -d "$path" ]]; then
      echo "$path exists and is not a directory" >&2
      exit 1
    fi
    local owner
    owner="$(owner_uid "$path")"
    if [[ "$owner" != "$uid" ]]; then
      echo "$path is owned by uid ${owner}, not ${uid}." >&2
      echo "Remove it and rerun: sudo rm -rf '$path'" >&2
      exit 1
    fi
    return 0
  fi
  mkdir -p "$path"
}

ensure_dir "${pkg}/.gocache"
ensure_dir "${pkg}/.gomodcache"
