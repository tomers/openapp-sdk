#!/usr/bin/env python3
"""
Merge Python SDK Allure raw results with JUnit / go test JSON from other SDK packages
into a single allure-results directory for `just sdk test`.

Python: copies `packages/sdk/python/.tmp/sdk-reports/allure-results/` (pytest + Behave
native Allure JSON). Other SDKs: converts JUnit XML (core, rust, node) and Go
`go-test.jsonl` into synthetic Allure 2 `*-result.json` files so `allure generate` produces
one combined HTML report.

Uses only the Python standard library.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import uuid
import xml.etree.ElementTree as ET
from datetime import datetime, timezone
from pathlib import Path


def _now_ms() -> int:
    return int(datetime.now(timezone.utc).timestamp() * 1000)


def _stable_history_id(*parts: str) -> str:
    h = hashlib.md5("|".join(parts).encode(), usedforsecurity=False).hexdigest()
    return h[:16]


def _write_result(
    out_dir: Path,
    *,
    name: str,
    full_name: str,
    status: str,
    suite_label: str,
    start: int | None = None,
    stop: int | None = None,
    framework: str,
) -> None:
    sid = str(uuid.uuid4())
    hid = _stable_history_id(full_name, suite_label)
    body: dict = {
        "uuid": sid,
        "historyId": hid,
        "fullName": full_name,
        "name": name,
        "status": status,
        "stage": "finished",
        "labels": [
            {"name": "suite", "value": suite_label},
            {"name": "framework", "value": framework},
        ],
    }
    if start is not None:
        body["start"] = start
    if stop is not None:
        body["stop"] = stop
    out_path = out_dir / f"{sid}-result.json"
    out_path.write_text(json.dumps(body) + "\n", encoding="utf-8")


def _parse_junit_file(path: Path, out_dir: Path, suite_label: str) -> int:
    if not path.is_file() or path.stat().st_size == 0:
        return 0
    try:
        tree = ET.parse(path)
    except ET.ParseError:
        return 0
    root = tree.getroot()
    n = 0
    for tc in root.iter():
        tag = tc.tag.split("}")[-1] if "}" in tc.tag else tc.tag
        if tag != "testcase":
            continue
        classname = tc.attrib.get("classname", "")
        tname = tc.attrib.get("name", "test")
        full = f"{classname}.{tname}" if classname else tname
        failed = False
        skipped = False
        for child in tc:
            ctag = child.tag.split("}")[-1] if "}" in child.tag else child.tag
            if ctag in ("failure", "error"):
                failed = True
            if ctag in ("skipped", "skipped_test"):
                skipped = True
        if skipped:
            status = "skipped"
        elif failed:
            status = "failed"
        else:
            status = "passed"
        _write_result(
            out_dir,
            name=tname,
            full_name=full,
            status=status,
            suite_label=suite_label,
            framework="junit",
        )
        n += 1
    return n


def _parse_go_test_jsonl(path: Path, out_dir: Path, suite_label: str) -> int:
    if not path.is_file() or path.stat().st_size == 0:
        return 0

    def parse_time_ms(raw: str | None) -> int | None:
        if not raw:
            return None
        s = raw.replace("Z", "+00:00") if raw.endswith("Z") else raw
        try:
            dt = datetime.fromisoformat(s)
            if dt.tzinfo is None:
                dt = dt.replace(tzinfo=timezone.utc)
            return int(dt.timestamp() * 1000)
        except ValueError:
            return None

    events: list[dict] = []
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            events.append(json.loads(line))
        except json.JSONDecodeError:
            continue

    # Pair run -> terminal action per (Package, Test)
    terminal = {"pass", "fail", "skip"}
    pending_start: dict[tuple[str, str], int | None] = {}
    count = 0
    for ev in events:
        action = ev.get("Action")
        pkg = ev.get("Package") or ""
        test = ev.get("Test")
        if not test or action not in ("run", *terminal):
            continue
        key = (pkg, test)
        if action == "run":
            pending_start[key] = parse_time_ms(ev.get("Time"))
            continue
        start = pending_start.pop(key, None)
        if action == "pass":
            status = "passed"
        elif action == "fail":
            status = "failed"
        elif action == "skip":
            status = "skipped"
        else:
            continue
        stop = parse_time_ms(ev.get("Time")) or _now_ms()
        if start is None:
            start = stop
        full = f"{pkg}.{test}"
        _write_result(
            out_dir,
            name=test,
            full_name=full,
            status=status,
            suite_label=suite_label,
            start=start,
            stop=stop,
            framework="go test",
        )
        count += 1
    return count


def _copy_python_allure(py_results: Path, out_dir: Path) -> int:
    if not py_results.is_dir():
        return 0
    n = 0
    for p in sorted(py_results.iterdir()):
        if p.is_file():
            shutil.copy2(p, out_dir / p.name)
            n += 1
    return n


def _layout_paths(repo: Path, layout: str) -> tuple[Path, list[tuple[Path, str]], Path]:
    if layout == "mirror":
        py_results = repo / "python" / ".tmp" / "sdk-reports" / "allure-results"
        junit_inputs: list[tuple[Path, str]] = []
        go_jsonl = repo / "go" / ".tmp" / "sdk-reports" / "go-test.jsonl"
        return py_results, junit_inputs, go_jsonl

    sdk = repo / "packages" / "sdk"
    py_results = sdk / "python" / ".tmp" / "sdk-reports" / "allure-results"
    junit_inputs = [
        (sdk / "core" / ".tmp" / "sdk-reports" / "junit-sdk-core.xml", "sdk-core"),
        (sdk / "rust" / ".tmp" / "sdk-reports" / "junit-sdk-rust.xml", "sdk-rust"),
        (
            sdk / "rust" / ".tmp" / "sdk-reports" / "junit-sdk-rust-gherkin-tier1.xml",
            "sdk-rust (gherkin)",
        ),
        (
            sdk / "node" / ".tmp" / "sdk-reports" / "junit-node-smoke.xml",
            "sdk-node (smoke)",
        ),
        (
            sdk / "node" / ".tmp" / "sdk-reports" / "junit-node-gherkin.xml",
            "sdk-node (gherkin)",
        ),
    ]
    go_jsonl = sdk / "go" / ".tmp" / "sdk-reports" / "go-test.jsonl"
    return py_results, junit_inputs, go_jsonl


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--repo",
        type=Path,
        required=True,
        help="Repository root (git root)",
    )
    ap.add_argument(
        "--out",
        type=Path,
        required=True,
        help="Output directory (allure-results)",
    )
    ap.add_argument(
        "--layout",
        choices=("monorepo", "mirror"),
        default="monorepo",
        help="Path layout: OpenApp monorepo (default) or tomers/openapp-sdk mirror",
    )
    args = ap.parse_args()
    repo: Path = args.repo.resolve()
    out: Path = args.out.resolve()
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True, exist_ok=True)

    py_results, junit_inputs, go_jsonl = _layout_paths(repo, args.layout)
    copied = _copy_python_allure(py_results, out)

    # JUnit → synthetic Allure `*-result.json` (one row per testcase).
    # Language coverage vs `just sdk test` (monorepo layout):
    #   Python — pytest + Behave write **native Allure** files into `allure-results/`; those
    #     are copied above (`_copy_python_allure`), not listed here. Optional JUnit files
    #     under `python/.tmp/sdk-reports/` are not merged (would duplicate Behave/pytest).
    #   packages/sdk/core — Rust workspace `cargo nextest` JUnit.
    #   packages/sdk/rust — nextest JUnit + tier1 Gherkin aggregate JUnit.
    #   packages/sdk/node — smoke (`node --test`) + Cucumber JUnit.
    #   packages/sdk/go — handled below via `go test -json` (not JUnit).
    extra = 0
    for path, label in junit_inputs:
        extra += _parse_junit_file(path, out, label)

    extra += _parse_go_test_jsonl(go_jsonl, out, "sdk-go")

    print(
        f"sdk_merge_allure_results: python_files={copied} converted_other={extra} -> {out}",
        flush=True,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
