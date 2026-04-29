"""`Integrations` sub-client."""

from __future__ import annotations

from builtins import list as _list
from typing import Any

from .._image_from_url import fetch_image_for_upload
from ._base import _BaseResource


class IntegrationsClient(_BaseResource):
    """Third-party integrations (ISO-7816, LAN agents, cloud-controlled gates, …)."""

    async def list(
        self,
        *,
        org_id: str | None = None,
        provider_type: str | None = None,
        include_deleted: bool | None = None,
        limit: int | None = None,
        cursor: str | None = None,
    ) -> dict[str, Any]:
        q = self._query(
            orgId=org_id,
            providerType=provider_type,
            includeDeleted=include_deleted,
            limit=limit,
            cursor=cursor,
        )
        return await self._client._request("GET", "/integrations", query=q)

    async def create(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/integrations", body=body)

    async def provider_types(self) -> _list[dict[str, Any]]:
        return await self._client._request("GET", "/integrations/provider-types")

    async def provider_definition(self, provider_type: str) -> dict[str, Any]:
        return await self._client._request(
            "GET", f"/integrations/provider-types/{provider_type}/definition"
        )

    async def get(self, integration_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/integrations/{integration_id}")

    async def update(self, integration_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/integrations/{integration_id}", body=patch)

    async def purge(self, integration_id: str) -> None:
        await self._client._request("DELETE", f"/integrations/{integration_id}/purge")

    async def restore(self, integration_id: str) -> dict[str, Any]:
        return await self._client._request("POST", f"/integrations/{integration_id}/restore")

    async def device_metadata_schema(self, integration_id: str) -> dict[str, Any]:
        return await self._client._request(
            "GET", f"/integrations/{integration_id}/device-metadata-schema"
        )

    async def discovered_devices(self, integration_id: str) -> _list[dict[str, Any]]:
        return await self._client._request(
            "GET", f"/integrations/{integration_id}/discovered-devices"
        )

    async def entities(self, integration_id: str) -> _list[dict[str, Any]]:
        return await self._client._request("GET", f"/integrations/{integration_id}/entities")

    async def ops(self, integration_id: str) -> _list[dict[str, Any]]:
        return await self._client._request("GET", f"/integrations/{integration_id}/ops")

    async def run_op(
        self,
        integration_id: str,
        op_id: str,
        **body: Any,
    ) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/integrations/{integration_id}/ops/{op_id}", body=body or {}
        )

    # -- Access portals / invites -------------------------------------------

    async def list_access_portals(self, integration_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/integrations/{integration_id}/access-portals")

    async def create_access_portal(self, integration_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/integrations/{integration_id}/access-portals", body=body
        )

    async def get_access_portal(self, portal_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/integrations/access-portals/{portal_id}")

    async def update_access_portal(
        self, integration_id: str, portal_id: str, **patch: Any
    ) -> dict[str, Any]:
        return await self._client._request(
            "PUT",
            f"/integrations/{integration_id}/access-portals/{portal_id}",
            body=patch,
        )

    async def delete_access_portal(self, integration_id: str, portal_id: str) -> None:
        await self._client._request(
            "DELETE", f"/integrations/{integration_id}/access-portals/{portal_id}"
        )

    async def update_access_invite(
        self, integration_id: str, invite_link_id: str, **patch: Any
    ) -> dict[str, Any]:
        return await self._client._request(
            "PUT",
            f"/integrations/{integration_id}/access-invites/{invite_link_id}",
            body=patch,
        )

    async def delete_access_invite(self, integration_id: str, invite_link_id: str) -> None:
        await self._client._request(
            "DELETE",
            f"/integrations/{integration_id}/access-invites/{invite_link_id}",
        )

    async def restore_access_invite(
        self, integration_id: str, invite_link_id: str
    ) -> dict[str, Any]:
        return await self._client._request(
            "POST",
            f"/integrations/{integration_id}/access-invites/{invite_link_id}/restore",
        )

    async def upload_image(
        self,
        integration_id: str,
        *,
        data: bytes,
        content_type: str,
        filename: str = "upload",
    ) -> dict[str, Any]:
        """Set the integration image via ``POST /integrations/{id}/image`` (multipart ``file``).

        ``content_type`` must be ``image/jpeg``, ``image/png``, or ``image/webp``,
        matching the dashboard upload rules.
        """

        return await self._client._request(
            "POST",
            f"/integrations/{integration_id}/image",
            multipart=("file", filename, content_type, data),
        )

    async def upload_image_from_url(
        self,
        integration_id: str,
        *,
        url: str,
        filename: str | None = None,
    ) -> dict[str, Any]:
        """Download an image from ``url`` (HTTP or HTTPS) and set the integration image."""

        data, content_type, default_name = await fetch_image_for_upload(url)
        return await self.upload_image(
            integration_id,
            data=data,
            content_type=content_type,
            filename=filename or default_name,
        )
