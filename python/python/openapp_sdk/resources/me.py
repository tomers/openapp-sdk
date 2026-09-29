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

    async def push_subscription_status(self) -> dict[str, Any]:
        return await self._client._request("GET", "/me/push-subscription-status")

    async def push_vapid_public_key(self) -> dict[str, Any]:
        return await self._client._request("GET", "/me/push-vapid-public-key")

    async def subscribe_push(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/me/push-subscriptions", body=body)
