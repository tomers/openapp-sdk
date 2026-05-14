#!/usr/bin/env bash
# Python SDK checks for openapp-sdk-release.yml (pytest unit+contract, Behave, JUnit + Allure inputs).
# Run from repo: `just sdk python release-gate`.
set -euo pipefail
cd "$(dirname "$0")/.."
rm -rf .venv
uv sync --all-extras
uv run maturin develop --features pyo3/extension-module
bash scripts/sdk_python_static_checks.sh
mkdir -p .tmp/sdk-reports/behave-junit .tmp/sdk-reports/allure-results .tmp/sdk-reports/allure-behave-results
uv run pytest -q tests/unit tests/contract \
  --junitxml=.tmp/sdk-reports/junit-release-gates.xml \
  --alluredir=.tmp/sdk-reports/allure-results
uv run behave ../tests/features --tags=-wip --no-color \
  -f allure_behave.formatter:AllureFormatter -o .tmp/sdk-reports/allure-behave-results \
  -f plain --junit --junit-directory=.tmp/sdk-reports/behave-junit
shopt -s nullglob
for f in .tmp/sdk-reports/allure-behave-results/*; do
  cp "$f" .tmp/sdk-reports/allure-results/
done
