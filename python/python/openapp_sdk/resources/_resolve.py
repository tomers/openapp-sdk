"""Shared helpers for name-based resource lookup."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Literal

if TYPE_CHECKING:  # pragma: no cover
    from ..client import AsyncClient

MatchMode = Literal["exact", "fuzzy"]


async def _bridge_get_by_name(
    client: AsyncClient, method: str, *args: Any, **kwargs: Any
) -> dict[str, Any]:
    bridge = client._bridge
    fn = getattr(bridge, method)
    result = await fn(*args, **kwargs)
    if not isinstance(result, dict):
        raise TypeError(f"expected dict from {method}, got {type(result).__name__}")
    return result
