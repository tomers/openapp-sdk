#!/usr/bin/env bash
# Run inside the Go toolchain container with cwd = packages/sdk/go (after openapi-generator).
#
# - sed: generator still pins go 1.18 in go.mod; this module targets go 1.22 until templates catch up.
# - go mod tidy: refresh dependencies/sum after large codegen churn.
# - gofmt: format generated .go only; prune .gocache/.gomodcache so we do not walk the module
#   download tree (noisy permissions; wrong targets).
# GOCACHE/GOMODCACHE: use container-local paths (see packages/sdk/go/justfile `generate`) so bind-mount
# caches are never mixed with root-owned artifacts from older Docker runs.
set -euo pipefail
: "${GOCACHE:=/tmp/gocache}"
: "${GOMODCACHE:=/tmp/gomodcache}"
mkdir -p "$GOCACHE" "$GOMODCACHE"
sed -i 's/^go 1\.18$/go 1.22/' go.mod
# Submodule layout: generator may emit `module github.com/tomers/openapp-sdk` without `/go`.
sed -i 's|^module github.com/tomers/openapp-sdk$|module github.com/tomers/openapp-sdk/go|' go.mod
go mod tidy
find . \( -path ./.gocache -o -path ./.gomodcache \) -prune -o -name '*.go' -print0 \
  | xargs -0 -r gofmt -s -w
