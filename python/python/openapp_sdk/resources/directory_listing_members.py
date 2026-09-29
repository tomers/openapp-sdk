"""`Directory Listing Members` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class DirectoryListingMembersClient(_BaseResource):
    """People and groups attached to one directory listing."""

    async def list(self, entity_id: str) -> list[dict[str, Any]]:
        return await self._client._request(
            "GET", f"/entities/{entity_id}/directory-listing-members"
        )

    async def add(self, entity_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/entities/{entity_id}/directory-listing-members", body=body
        )

    async def update(self, entity_id: str, member_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "PUT", f"/entities/{entity_id}/directory-listing-members/{member_id}", body=body
        )

    async def remove(self, entity_id: str, member_id: str) -> None:
        await self._client._request(
            "DELETE", f"/entities/{entity_id}/directory-listing-members/{member_id}"
        )
