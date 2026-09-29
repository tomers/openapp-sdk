#!/usr/bin/env bash
# After sdk-go has run, every file under the bind-mounted Go caches must belong
# to the invoking user (not root from Docker).
set -euo pipefail

pkg="${1:?package directory}"
uid="$(id -u)"

owner_uid() {
  python3 -c 'import os, sys; print(os.stat(sys.argv[1]).st_uid)' "$1"
}

for d in .gocache .gomodcache; do
  path="${pkg}/${d}"
  if [[ ! -d "$path" ]]; then
    echo "missing ${path} (sdk-go should have created it on the host)" >&2
    exit 1
  fi
  owner="$(owner_uid "$path")"
  if [[ "$owner" != "$uid" ]]; then
    echo "${path} is owned by uid ${owner}, expected ${uid}" >&2
    exit 1
  fi
  while IFS= read -r -d '' f; do
    f_owner="$(owner_uid "$f")"
    if [[ "$f_owner" != "$uid" ]]; then
      echo "${f} is owned by uid ${f_owner}, expected ${uid}" >&2
      exit 1
    fi
  done < <(find "$path" -print0)
done
echo "assert_bind_mount_go_caches_owner.sh: ok"
