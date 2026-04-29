"""`Me` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class MeClient(_BaseResource):
    """Endpoints scoped to the caller."""

    async def apartments(self) -> list[dict[str, Any]]:
        return await self._client._request("GET", "/me/apartments")

    async def invitations(self) -> list[dict[str, Any]]:
        return await self._client._request("GET", "/me/invitations")

    async def push_subscription_status(self) -> dict[str, Any]:
        return await self._client._request("GET", "/me/push-subscription-status")

    async def push_vapid_public_key(self) -> dict[str, Any]:
        return await self._client._request("GET", "/me/push-vapid-public-key")

    async def subscribe_push(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/me/push-subscriptions", body=body)
