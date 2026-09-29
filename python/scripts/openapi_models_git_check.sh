#!/usr/bin/env bash
# After `openapi_models.sh gen`, fail pre-commit when _generated.py is unstaged.
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
generated="packages/sdk/python/python/openapp_sdk/models/_generated.py"
if ! git -C "$repo_root" diff --quiet -- "$generated"; then
  echo "$generated was regenerated from packages/api-spec/openapi.json." >&2
  echo "Run: git add $generated" >&2
  exit 1
fi
echo "$generated is up to date."
