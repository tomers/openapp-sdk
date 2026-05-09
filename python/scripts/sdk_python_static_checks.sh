#!/usr/bin/env bash
# Ruff, mypy, OpenAPI models drift, README wheel table, Gherkin meta checks.
# Expects cwd `packages/sdk/python` and a ready `.venv` (after `uv sync` + maturin develop).
set -euo pipefail
uv run ruff check .
uv run ruff format --check .
uv run python -m mypy
bash scripts/openapi_models.sh check
uv run python scripts/check_wheel_readme_drift.py
uv run python scripts/check_gherkin_tier0.py
uv run python scripts/check_gherkin_story_packs.py
uv run python scripts/check_gherkin_stable_titles.py
