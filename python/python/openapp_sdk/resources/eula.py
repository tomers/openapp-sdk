"""`EULA` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class EulaClient(_BaseResource):
    """End-user license agreement surface."""

    async def get(self) -> dict[str, Any]:
        return await self._client._request("GET", "/eula")

    async def accept(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/eula/accept", body=body)
