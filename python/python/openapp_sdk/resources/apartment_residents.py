"""`Apartment Residents` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class ApartmentResidentsClient(_BaseResource):
    """Building / apartment resident management per integration."""

    async def list(self, integration_id: str) -> list[dict[str, Any]]:
        return await self._client._request("GET", f"/integrations/{integration_id}/building-users")

    async def add(self, integration_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/integrations/{integration_id}/building-users", body=body
        )

    async def remove(self, integration_id: str, user_id: str) -> None:
        await self._client._request(
            "DELETE", f"/integrations/{integration_id}/building-users/{user_id}"
        )
