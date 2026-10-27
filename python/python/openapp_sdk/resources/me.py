"""`Me` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class MeClient(_BaseResource):
    """Endpoints scoped to the caller."""

    async def profile(self) -> dict[str, Any]:
        """The authenticated principal's profile (works for API keys and sessions)."""
        return await self._client._request("GET", "/me/profile")

    async def directory_listings(self) -> list[dict[str, Any]]:
        return await self._client._request("GET", "/me/directory-listings")

    async def invitations(self) -> list[dict[str, Any]]:
        return await self._client._request("GET", "/me/invitations")
