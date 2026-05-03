#!/usr/bin/env bash
# Publish generated Pact JSON files to a Pact Broker / PactFlow (optional).
#
# When PACT_BROKER_BASE_URL is unset, exits 0 (no-op) so CI stays green without broker.
#
# Auth: set PACT_BROKER_TOKEN (bearer) or PACT_BROKER_USERNAME + PACT_BROKER_PASSWORD.
#
# Consumer version: PACT_CONSUMER_VERSION (default: GITHUB_SHA or git rev-parse).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# This file lives at packages/sdk/python/scripts → SDK root is packages/sdk.
SDK_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"
PACTS_DIR="${SDK_ROOT}/contracts/pacts"

if [[ -z "${PACT_BROKER_BASE_URL:-}" ]]; then
  echo "PACT_BROKER_BASE_URL not set; skipping Pact broker publish."
  exit 0
fi

shopt -s nullglob
pact_files=("${PACTS_DIR}"/*.json)
shopt -u nullglob
if [[ ${#pact_files[@]} -eq 0 ]]; then
  echo "No pact JSON files under ${PACTS_DIR}; run contract tests first." >&2
  exit 1
fi

PACT_CLI_VER="${PACT_CLI_VER:-2.6.0}"
PACT_CLI_CACHE="${PACT_CLI_CACHE:-${TMPDIR:-/tmp}/openapp-pact-cli-${PACT_CLI_VER}}"
if [[ ! -x "${PACT_CLI_CACHE}/pact/bin/pact-broker" ]]; then
  mkdir -p "${PACT_CLI_CACHE}"
  url="https://github.com/pact-foundation/pact-ruby-standalone/releases/download/v${PACT_CLI_VER}/pact-${PACT_CLI_VER}-linux-x86_64.tar.gz"
  echo "Downloading Pact CLI ${PACT_CLI_VER}..."
  curl -fsSL "${url}" | tar -xz -C "${PACT_CLI_CACHE}"
fi
export PATH="${PACT_CLI_CACHE}/pact/bin:${PATH}"

consumer_ver="${PACT_CONSUMER_VERSION:-}"
if [[ -z "${consumer_ver}" ]]; then
  if [[ -n "${GITHUB_SHA:-}" ]]; then
    consumer_ver="${GITHUB_SHA}"
  else
    consumer_ver="$(git -C "${REPO_ROOT}" rev-parse HEAD 2>/dev/null || echo local)"
  fi
fi

publish_args=(
  "${PACTS_DIR}"
  --broker-base-url="${PACT_BROKER_BASE_URL}"
  --consumer-app-version="${consumer_ver}"
)

if [[ -n "${PACT_BROKER_TOKEN:-}" ]]; then
  publish_args+=(--broker-token="${PACT_BROKER_TOKEN}")
elif [[ -n "${PACT_BROKER_USERNAME:-}" && -n "${PACT_BROKER_PASSWORD:-}" ]]; then
  publish_args+=(--broker-username="${PACT_BROKER_USERNAME}" --broker-password="${PACT_BROKER_PASSWORD}")
else
  echo "Set PACT_BROKER_TOKEN or PACT_BROKER_USERNAME+PACT_BROKER_PASSWORD for broker auth." >&2
  exit 1
fi

if [[ "${PACT_BROKER_AUTO_DETECT_CI:-}" == "1" ]]; then
  publish_args+=(--auto-detect-version-properties)
fi

echo "Publishing pacts from ${PACTS_DIR} to ${PACT_BROKER_BASE_URL} (consumer version ${consumer_ver})..."
pact-broker publish "${publish_args[@]}"
