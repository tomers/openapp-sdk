"""Remove openapi-generator collision: HasExternalId method vs HasExternalId field.

OpenAPI Generator emits `HasExternalId()` as a presence helper for `external_id`, but the
same struct also has a query field named `has_external_id` → field `HasExternalId`.
Go forbids a field and method with the same name. We drop the redundant helper; callers
can use ExternalId.IsSet() or GetExternalIdOk(), and use HasHasExternalId() for the filter.
"""

from __future__ import annotations

import sys
from pathlib import Path

_MARK_BEGIN = "// HasExternalId returns a boolean if a field has been set.\n"
_MARK_FUNC = "func (o *ListDevicesQuery) HasExternalId() bool {\n"


def _remove_collision_helper(text: str) -> tuple[str, bool]:
    i = text.find(_MARK_BEGIN)
    if i == -1:
        return text, False
    j = i + len(_MARK_BEGIN)
    if text[j : j + len(_MARK_FUNC)] != _MARK_FUNC:
        return text, False
    depth = 0
    k = j
    while k < len(text):
        c = text[k]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                end = k + 1
                if end < len(text) and text[end] == "\n":
                    end += 1
                return text[:i] + text[end:], True
        k += 1
    raise SystemExit("Unbalanced braces while stripping HasExternalId helper")


def main() -> None:
    path = Path(__file__).resolve().parent.parent / "model_list_devices_query.go"
    if len(sys.argv) > 1:
        path = Path(sys.argv[1])
    text = path.read_text()
    new_text, removed = _remove_collision_helper(text)
    if removed:
        path.write_text(new_text)


if __name__ == "__main__":
    main()
