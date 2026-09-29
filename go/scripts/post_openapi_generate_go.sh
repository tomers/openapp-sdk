#!/usr/bin/env bash
# Run inside the Go toolchain container with cwd = packages/sdk/go (after openapi-generator).
#
# - sed: generator still pins go 1.18 in go.mod; this module targets go 1.22 until templates catch up.
# - go mod tidy: refresh dependencies/sum after large codegen churn.
# - gofmt: format generated .go only; prune .gocache/.gomodcache so we do not walk the module
#   download tree (noisy permissions; wrong targets).
# GOCACHE/GOMODCACHE: container-local /tmp for this generate step (justfile `generate`).
# sdk-go compose caches stay on the bind mount and are mkdir'd on the host as the user.
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
