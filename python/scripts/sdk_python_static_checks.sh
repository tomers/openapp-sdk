#!/usr/bin/env bash
# Ruff, mypy, README wheel table, Gherkin meta checks.
# OpenAPI model drift is handled by `just openapi-check` (regenerate + git diff) before this script runs.
# Expects cwd `packages/sdk/python` and a ready `.venv` (after `uv sync` + maturin develop).
set -euo pipefail
uv run ruff check .
uv run ruff format --check .
uv run python -m mypy
uv run python scripts/check_wheel_readme_drift.py
uv run python scripts/check_gherkin_tier0.py
uv run python scripts/check_gherkin_story_packs.py
uv run python scripts/check_gherkin_stable_titles.py
