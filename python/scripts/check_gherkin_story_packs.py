#!/usr/bin/env python3
"""Guardrail: ``*_story_pack.feature`` files must keep a minimum scenario count."""

from __future__ import annotations

import re
import sys
from pathlib import Path

_SDK_ROOT = Path(__file__).resolve().parents[2]
_FEATURES = _SDK_ROOT / "features"
_MIN_SCENARIOS = 9
_SCENARIO = re.compile(r"^\s+Scenario:")


def main() -> int:
    if not _FEATURES.is_dir():
        print(f"Missing features directory: {_FEATURES}", file=sys.stderr)
        return 1
    packs = sorted(_FEATURES.glob("*_story_pack.feature"))
    if not packs:
        print(f"No *_story_pack.feature files under {_FEATURES}", file=sys.stderr)
        return 1
    total = 0
    for path in packs:
        text = path.read_text(encoding="utf-8")
        n = sum(1 for line in text.splitlines() if _SCENARIO.match(line))
        total += n
    if total < _MIN_SCENARIOS:
        print(
            f"Expected at least {_MIN_SCENARIOS} scenarios across *_story_pack.feature "
            f"under {_FEATURES}, found {total}.",
            file=sys.stderr,
        )
        return 1
    print(f"Gherkin story-pack check ok ({total} scenarios in {len(packs)} files).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
