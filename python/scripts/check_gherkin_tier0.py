#!/usr/bin/env python3
"""Guardrail: shared Gherkin must keep a minimum set of @tier0 scenarios."""

from __future__ import annotations

import re
import sys
from pathlib import Path

# packages/sdk/python/scripts -> packages/sdk
_SDK_ROOT = Path(__file__).resolve().parents[2]
_FEATURES = _SDK_ROOT / "features"
_MIN_TIER0_SCENARIOS = 5
_HEADER_LINES = 30
_SCENARIO = re.compile(r"^\s+Scenario:")


def main() -> int:
    if not _FEATURES.is_dir():
        print(f"Missing features directory: {_FEATURES}", file=sys.stderr)
        return 1
    tier0 = 0
    for path in sorted(_FEATURES.glob("*.feature")):
        lines = path.read_text(encoding="utf-8").splitlines()
        head = "\n".join(lines[:_HEADER_LINES])
        if "@tier0" not in head:
            continue
        for line in lines:
            if _SCENARIO.match(line):
                tier0 += 1
    if tier0 < _MIN_TIER0_SCENARIOS:
        print(
            f"Expected at least {_MIN_TIER0_SCENARIOS} scenarios in @tier0 features "
            f"under {_FEATURES}, found {tier0}.",
            file=sys.stderr,
        )
        return 1
    print(f"Gherkin @tier0 scenario check ok ({tier0} scenarios).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
