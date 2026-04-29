"""`Zones` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class ZonesClient(_BaseResource):
    """Zones: logical groupings of devices under an integration."""

    async def create(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/zones", body=body)

    async def get(self, zone_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/zones/{zone_id}")

    async def update(self, zone_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/zones/{zone_id}", body=patch)

    async def delete(self, zone_id: str) -> None:
        await self._client._request("DELETE", f"/zones/{zone_id}")

    async def purge(self, zone_id: str) -> None:
        await self._client._request("DELETE", f"/zones/{zone_id}/purge")

    async def by_integration(self, integration_id: str) -> list[dict[str, Any]]:
        return await self._client._request("GET", f"/integrations/{integration_id}/zones")
