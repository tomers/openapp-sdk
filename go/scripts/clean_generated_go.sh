#!/usr/bin/env bash
# Delete every file openapi-generator owns in packages/sdk/go before `just sdk go generate` reruns it.
#
# openapi-generator only writes files; it never removes the ones a schema or operation no longer
# produces, so without this step a model dropped from the spec stays in the package (and ships)
# forever. Regenerating into a clean slate makes the output a pure function of the spec.
#
# Only the generator's own filename shapes are removed. Hand-maintained sources (client_close*.go,
# core_transport*.go, bridge/, scripts/, README.md, justfile, gherkin_tier1_test.go, ...) match none
# of them, and `.openapi-generator-ignore` lists the ones the generator must not overwrite.
set -euo pipefail

pkg="${1:?usage: clean_generated_go.sh <packages/sdk/go>}"
cd "$pkg"

find . -maxdepth 1 -type f \( -name 'api_*.go' -o -name 'model_*.go' \) -delete
find docs -maxdepth 1 -type f -name '*.md' -delete 2>/dev/null || true
find test -maxdepth 1 -type f -name 'api_*_test.go' -delete 2>/dev/null || true
