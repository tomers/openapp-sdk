#!/usr/bin/env python3
"""Guardrail: shared Gherkin scenario titles must stay stable and unique."""

from __future__ import annotations

import re
import sys
from pathlib import Path

_SDK_ROOT = Path(__file__).resolve().parents[2]
_FEATURES = _SDK_ROOT / "features"
_SCENARIO = re.compile(r"^\s+Scenario:\s*(.+?)\s*$")


def _canon(title: str) -> str:
    # Canonical form avoids accidental churn from spacing/casing-only edits.
    return " ".join(title.split()).casefold()


def main() -> int:
    if not _FEATURES.is_dir():
        print(f"Missing features directory: {_FEATURES}", file=sys.stderr)
        return 1

    seen: dict[str, tuple[str, int, str]] = {}
    duplicates: list[tuple[str, tuple[str, int, str], tuple[str, int, str]]] = []
    scenario_count = 0

    feature_files = sorted(_FEATURES.glob("*.feature"))
    for path in feature_files:
        for line_no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            m = _SCENARIO.match(line)
            if not m:
                continue
            scenario_count += 1
            title = m.group(1)
            key = _canon(title)
            location = (path.name, line_no, title)
            first = seen.get(key)
            if first is None:
                seen[key] = location
                continue
            duplicates.append((key, first, location))

    if duplicates:
        print(
            f"Duplicate Gherkin scenario titles found under {_FEATURES} (titles must be globally stable/unique):",
            file=sys.stderr,
        )
        for _key, first, second in duplicates:
            print(
                f"- '{first[2]}' in {first[0]}:{first[1]} and {second[0]}:{second[1]}",
                file=sys.stderr,
            )
        return 1

    print(
        f"Gherkin stable-title check ok ({scenario_count} scenarios across {len(feature_files)} files)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
