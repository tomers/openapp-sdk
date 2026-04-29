"""Shared helpers for sub-clients."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:  # pragma: no cover
    from ..client import AsyncClient


class _BaseResource:
    """Mix-in that caches the parent client reference and exposes helpers every
    sub-client uses (query string assembly, etc.).
    """

    __slots__ = ("_client",)

    def __init__(self, client: AsyncClient) -> None:
        self._client = client

    @staticmethod
    def _query(**kwargs: Any) -> tuple[tuple[str, str | None], ...]:
        """Build a query-string pair list, dropping ``None`` values.

        Lists are serialised with comma separation, matching the backend's
        `utoipa`-generated query parsers.
        """

        pairs: list[tuple[str, str | None]] = []
        for key, value in kwargs.items():
            if value is None:
                continue
            if isinstance(value, bool):
                pairs.append((key, "true" if value else "false"))
            elif isinstance(value, (list, tuple, set)):
                pairs.append((key, ",".join(str(v) for v in value)))
            else:
                pairs.append((key, str(value)))
        return tuple(pairs)
