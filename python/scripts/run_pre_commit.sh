#!/usr/bin/env bash
# Full Python SDK pre-commit gate inside sdk-python (matches package justfile pre-commit).
set -euo pipefail
cd /workspace/packages/sdk/python

# Keep lockfile in sync with version/dependency metadata changes.
# If this updates uv.lock, pre-commit will stop the commit so the user can
# stage the file and retry without manual editing.
uv lock
uv sync --all-extras
uv run maturin develop --features pyo3/extension-module
bash scripts/sdk_python_static_checks.sh
uv run pytest -q tests/unit

repo_root="$(git rev-parse --show-toplevel)"
if ! git -C "$repo_root" diff --quiet -- packages/sdk/python/Cargo.lock packages/sdk/python/uv.lock; then
  echo "error: uv lock/sync rewrote committed lockfiles under packages/sdk/python/." >&2
  echo "Stage the updates (Cargo.lock / uv.lock) and retry the commit." >&2
  git -C "$repo_root" --no-pager diff -- packages/sdk/python/Cargo.lock packages/sdk/python/uv.lock >&2
  exit 1
fi
