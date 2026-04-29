"""`Status` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class StatusClient(_BaseResource):
    """Backend liveness / readiness probe."""

    async def get(self) -> Any:
        """`GET /status` — returns the backend's self-reported health dict."""

        return await self._client._request("GET", "/status")
