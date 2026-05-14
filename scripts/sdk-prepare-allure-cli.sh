#!/usr/bin/env bash
# Install the Allure command-line tool under /tmp and prepend it to PATH.
# Used by SDK CI workflows and `scripts/sdk-release-gates.sh` — keep in sync.
#
# Usage (bash):  source path/to/sdk-prepare-allure-cli.sh
# Optional env:  ALLURE_VER (default 2.30.0)
#
# Idempotent: skips the download when /tmp/allure-${ALLURE_VER}/bin/allure exists.

if [[ -n "${BASH_VERSION:-}" ]] && [[ "${BASH_SOURCE[0]:-}" == "${0}" ]]; then
  echo "sdk-prepare-allure-cli.sh: source this file; do not execute it directly." >&2
  exit 1
fi

ALLURE_VER="${ALLURE_VER:-2.30.0}"
ALLURE_HOME="/tmp/allure-${ALLURE_VER}"

if [[ -x "${ALLURE_HOME}/bin/allure" ]]; then
  export PATH="${ALLURE_HOME}/bin:${PATH}"
  return 0
fi

curl -fsSL "https://github.com/allure-framework/allure2/releases/download/${ALLURE_VER}/allure-${ALLURE_VER}.tgz" \
  | tar -xz -C /tmp
export PATH="${ALLURE_HOME}/bin:${PATH}"
