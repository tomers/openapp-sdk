#!/usr/bin/env python3
"""Fail if WHEEL_PLATFORMS.md disagrees with openapp-sdk-release.yml.

Canonical matrix: .github/workflows/openapp-sdk-release.yml (build-wheels job).
User-facing README intentionally omits platform details.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

import yaml

REPO_ROOT = Path(__file__).resolve().parents[3]
WORKFLOW = REPO_ROOT / ".github" / "workflows" / "openapp-sdk-release.yml"
PLATFORMS_DOC = REPO_ROOT / "packages" / "sdk-python" / "WHEEL_PLATFORMS.md"
ANCHOR = "Prebuilt wheels are published for:"


def _load_workflow() -> dict:
    if not WORKFLOW.is_file():
        raise FileNotFoundError(f"Missing workflow: {WORKFLOW}")
    with WORKFLOW.open(encoding="utf-8") as f:
        return yaml.safe_load(f)


def _expected_bullets(data: dict) -> list[str]:
    job = data["jobs"]["build-wheels"]
    matrix_rows = job["strategy"]["matrix"]["include"]
    env = job.get("env") or {}
    dt = env.get("MACOSX_DEPLOYMENT_TARGET")
    if not dt:
        raise ValueError("build-wheels.env.MACOSX_DEPLOYMENT_TARGET is required")

    linux: dict[str, str] = {}
    macos_runners: list[str] = []
    for row in matrix_rows:
        os_name = row["os"]
        target = row["target"]
        if os_name.startswith("ubuntu"):
            ml = row.get("manylinux")
            if not ml:
                raise ValueError(f"ubuntu matrix row missing manylinux: {row!r}")
            linux[target] = ml
        elif os_name.startswith("macos"):
            macos_runners.append(os_name)

    for key in ("x86_64", "aarch64"):
        if key not in linux:
            raise ValueError(f"Linux matrix must include target {key!r}, got {sorted(linux)!r}")

    if not macos_runners:
        raise ValueError("No macos matrix rows found")
    if len(set(macos_runners)) != 1:
        raise ValueError(f"Expected a single macOS runner image, got {macos_runners!r}")

    m = re.match(r"macos-(\d+)", macos_runners[0])
    if not m:
        raise ValueError(f"Unrecognized macOS runner: {macos_runners[0]!r}")
    macos_major = m.group(1)

    dt_major = dt.split(".", 1)[0] + "+"

    return [
        f"* Linux: `{linux['x86_64']}` x86_64, `{linux['aarch64']}` aarch64",
        (
            f"* macOS: {dt_major} x86_64 and arm64 (wheels built on macOS {macos_major}; "
            f"compatibility via `MACOSX_DEPLOYMENT_TARGET={dt}`)"
        ),
        "* Windows: x86_64",
    ]


def _platform_doc_bullets(text: str) -> list[str]:
    if ANCHOR not in text:
        raise ValueError(f"{PLATFORMS_DOC.name} missing anchor line: {ANCHOR!r}")
    lines = text.splitlines()
    start = None
    for i, line in enumerate(lines):
        if line.strip() == ANCHOR:
            start = i + 1
            break
    if start is None:
        raise ValueError(f"{PLATFORMS_DOC.name} missing anchor line: {ANCHOR!r}")

    bullets: list[str] = []
    for line in lines[start:]:
        stripped = line.strip()
        if not stripped:
            if bullets:
                break
            continue
        if stripped.startswith("* "):
            bullets.append(line.rstrip())
            if len(bullets) == 3:
                break
        elif bullets:
            break

    if len(bullets) != 3:
        raise ValueError(f"Expected exactly 3 bullet lines after {ANCHOR!r}, found {len(bullets)}")
    return bullets


def main() -> int:
    if not PLATFORMS_DOC.is_file():
        print(f"ERROR: {PLATFORMS_DOC} not found", file=sys.stderr)
        return 1

    data = _load_workflow()
    expected = _expected_bullets(data)
    doc_text = PLATFORMS_DOC.read_text(encoding="utf-8")
    actual = _platform_doc_bullets(doc_text)

    norm = [ln.strip() for ln in actual]
    norm_exp = [ln.strip() for ln in expected]
    if norm != norm_exp:
        print(
            "WHEEL_PLATFORMS.md wheel bullets drift from .github/workflows/openapp-sdk-release.yml.\n",
            file=sys.stderr,
        )
        print("Expected:", file=sys.stderr)
        for ln in expected:
            print(f"  {ln}", file=sys.stderr)
        print("Actual:", file=sys.stderr)
        for ln in actual:
            print(f"  {ln}", file=sys.stderr)
        print(
            f"\nUpdate {PLATFORMS_DOC.relative_to(REPO_ROOT)} to match the release workflow.",
            file=sys.stderr,
        )
        return 1

    print("WHEEL_PLATFORMS.md wheel bullets match openapp-sdk-release.yml.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
