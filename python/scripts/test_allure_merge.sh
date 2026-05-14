#!/usr/bin/env bash
# Pytest + Behave with merged Allure inputs under `.tmp/sdk-reports/` (same merge as
# `scripts/release_gate.sh`, pytest scope matches `just python test-pytest`: full `pytest -v`).
# Invoked from Docker (`just python test`) with cwd `packages/sdk/python`.
set -euo pipefail

rep=".tmp/sdk-reports"
mkdir -p "$rep/behave-junit" "$rep/allure-results" "$rep/allure-behave-results"

uv run pytest -v \
  --junitxml="$rep/pytest-junit.xml" \
  --alluredir="$rep/allure-results"

uv run behave ../tests/features --tags=-wip --no-color \
  -f allure_behave.formatter:AllureFormatter -o "$rep/allure-behave-results" \
  -f plain --junit --junit-directory="$rep/behave-junit"

shopt -s nullglob
for f in "$rep/allure-behave-results"/*; do
  cp "$f" "$rep/allure-results/"
done
