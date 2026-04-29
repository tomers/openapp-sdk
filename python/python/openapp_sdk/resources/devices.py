"""`Devices` sub-client."""

from __future__ import annotations

from typing import Any

from .._image_from_url import fetch_image_for_upload
from ._base import _BaseResource


class DevicesClient(_BaseResource):
    """Manage devices (gates, doors, controllers, …)."""

    async def list(
        self,
        *,
        org_id: str | None = None,
        integration_id: str | None = None,
        zone_id: str | None = None,
        include_deleted: bool | None = None,
        limit: int | None = None,
        cursor: str | None = None,
    ) -> dict[str, Any]:
        q = self._query(
            orgId=org_id,
            integrationId=integration_id,
            zoneId=zone_id,
            includeDeleted=include_deleted,
            limit=limit,
            cursor=cursor,
        )
        return await self._client._request("GET", "/devices", query=q)

    async def create(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/devices", body=body)

    async def get(self, device_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/devices/{device_id}")

    async def update(self, device_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/devices/{device_id}", body=patch)

    async def delete(self, device_id: str) -> None:
        await self._client._request("DELETE", f"/devices/{device_id}")

    async def purge(self, device_id: str) -> None:
        await self._client._request("DELETE", f"/devices/{device_id}/purge")

    async def restore(self, device_id: str) -> dict[str, Any]:
        return await self._client._request("POST", f"/devices/{device_id}/restore")

    async def door_restrictions(self, device_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/devices/{device_id}/door-restrictions")

    async def set_door_restrictions(self, device_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "PUT",
            f"/devices/{device_id}/door-restrictions",
            body=body,
        )

    async def metadata_definition(self, device_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/devices/{device_id}/metadata-definition")

    async def upload_image(
        self,
        device_id: str,
        *,
        data: bytes,
        content_type: str,
        filename: str = "upload",
    ) -> dict[str, Any]:
        """Set the device image via ``POST /devices/{id}/image`` (multipart ``file``).

        ``content_type`` must be ``image/jpeg``, ``image/png``, or ``image/webp``,
        matching the dashboard upload rules.
        """

        return await self._client._request(
            "POST",
            f"/devices/{device_id}/image",
            multipart=("file", filename, content_type, data),
        )

    async def upload_image_from_url(
        self,
        device_id: str,
        *,
        url: str,
        filename: str | None = None,
    ) -> dict[str, Any]:
        """Download an image from ``url`` (HTTP or HTTPS) and set the device image."""

        data, content_type, default_name = await fetch_image_for_upload(url)
        return await self.upload_image(
            device_id,
            data=data,
            content_type=content_type,
            filename=filename or default_name,
        )
