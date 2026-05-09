"""Post-process openapi-generator Go output for OpenApp core-only HTTP.

OpenAPI Generator emits `NewAPIClient(cfg *Configuration)` with a default
`net/http` client. This module uses `newAPIClientFromConfig` plus `NewAPIClient`
from core_transport (cgo). Patches:

- `client.go`: rename constructor, drop DefaultClient fallback, set doc comment.
- `test/*.go`, `docs/*.md`: examples use `NewAPIClient(apiKey)` + error handling.
- Strip trailing whitespace and normalize EOF newline so `openapi-check` matches pre-commit.
"""

from __future__ import annotations

import sys
from pathlib import Path

_OLD_TEST_SNIPPET = (
    "\tconfiguration := openapiclient.NewConfiguration()\n"
    "\tapiClient := openapiclient.NewAPIClient(configuration)\n"
)

_NEW_TEST_SNIPPET = (
    '\tapiKey := "http://127.0.0.1:1/api/v1_openapp_testsecret"\n'
    "\tapiClient, err := openapiclient.NewAPIClient(apiKey)\n"
    "\trequire.NoError(t, err)\n"
    "\tt.Cleanup(func() { _ = apiClient.Close() })\n"
)

_OLD_DOC_SNIPPET = (
    "\tconfiguration := openapiclient.NewConfiguration()\n"
    "\tapiClient := openapiclient.NewAPIClient(configuration)\n"
)

_NEW_DOC_SNIPPET = (
    '\tapiClient, err := openapiclient.NewAPIClient("http://127.0.0.1:8080/api/v1_openapp_example_secret")\n'
    "\tif err != nil {\n"
    '\t\tfmt.Fprintf(os.Stderr, "NewAPIClient: %v\\n", err)\n'
    "\t\tos.Exit(1)\n"
    "\t}\n"
    "\tdefer apiClient.Close()\n"
)

_NEW_HEADER = """// newAPIClientFromConfig constructs an [APIClient] from cfg. [Configuration.HTTPClient]
// must already implement OpenApp HTTP policy—typically via [NewAPIClient] in core_transport.go
// (openapp-sdk-core through cgo).
func newAPIClientFromConfig(cfg *Configuration) *APIClient {"""

_DEFAULT_CLIENT_BLOCK = (
    "\n\tif cfg.HTTPClient == nil {\n\t\tcfg.HTTPClient = http.DefaultClient\n\t}\n\n"
)


def _patch_client_go(text: str) -> tuple[str, bool]:
    sig = "func NewAPIClient(cfg *Configuration) *APIClient {"
    if sig not in text:
        return text, False

    i = text.index(sig)
    j = i
    while j > 0:
        prev_nl = text.rfind("\n", 0, j - 1)
        line_start = 0 if prev_nl == -1 else prev_nl + 1
        line = text[line_start:j]
        stripped = line.strip()
        if stripped.startswith("//") or stripped == "":
            j = line_start
            if j == 0:
                break
            continue
        break

    out = text[:j] + _NEW_HEADER + text[i + len(sig) :]
    if _DEFAULT_CLIENT_BLOCK in out:
        out = out.replace(_DEFAULT_CLIENT_BLOCK, "\n", 1)
    return out, True


def _patch_examples(path: Path, text: str) -> tuple[str, bool]:
    if path.suffix == ".go":
        old, new = _OLD_TEST_SNIPPET, _NEW_TEST_SNIPPET
    else:
        old, new = _OLD_DOC_SNIPPET, _NEW_DOC_SNIPPET
    if old not in text:
        return text, False
    return text.replace(old, new), True


def _normalize_generated_whitespace(root: Path) -> None:
    """Match repo pre-commit trailing-whitespace + end-of-file-fixer on generator output."""
    for rel in ("docs", "test"):
        pattern = "*.md" if rel == "docs" else "*.go"
        for path in sorted((root / rel).glob(pattern)):
            raw = path.read_text(encoding="utf-8")
            lines = [line.rstrip() for line in raw.splitlines()]
            while lines and lines[-1] == "":
                lines.pop()
            path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    client = root / "client.go"
    if not client.is_file():
        print("skip: client.go not found", file=sys.stderr)
        return

    text = client.read_text()
    new_text, changed = _patch_client_go(text)
    if changed:
        client.write_text(new_text)

    for rel in ("test", "docs"):
        d = root / rel
        if not d.is_dir():
            continue
        pattern = "*.md" if rel == "docs" else "*.go"
        for path in sorted(d.glob(pattern)):
            t = path.read_text()
            nt, ch = _patch_examples(path, t)
            if ch:
                path.write_text(nt)

    _normalize_generated_whitespace(root)


if __name__ == "__main__":
    main()
