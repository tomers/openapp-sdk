#!/usr/bin/env bash
# Unit tests for ensure_host_go_caches.sh (no Docker).
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
script="${root}/ensure_host_go_caches.sh"
uid="$(id -u)"

owner_uid() {
  python3 -c 'import os, sys; print(os.stat(sys.argv[1]).st_uid)' "$1"
}

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/ensure-host-go-caches.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

bash "$script" "$tmpdir"
for d in .gocache .gomodcache; do
  [[ -d "${tmpdir}/${d}" ]] || fail "expected ${d} to exist"
  owner="$(owner_uid "${tmpdir}/${d}")"
  [[ "$owner" == "$uid" ]] || fail "${d} owned by uid ${owner}, expected ${uid}"
done

# Existing user-owned dirs are left in place.
bash "$script" "$tmpdir"

# A non-directory at a cache path must fail rather than mkdir -p over it.
rm -rf "${tmpdir}/.gomodcache"
touch "${tmpdir}/.gomodcache"
if bash "$script" "$tmpdir" >/dev/null 2>&1; then
  fail "expected failure when .gomodcache is a file"
fi

echo "ensure_host_go_caches_test.sh: ok"
