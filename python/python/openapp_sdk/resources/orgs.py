"""`Orgs` sub-client."""

from __future__ import annotations

from typing import Any, Literal

from .._image_from_url import fetch_image_for_upload
from ._base import _BaseResource
from ._resolve import _bridge_get_by_name

MatchMode = Literal["exact", "fuzzy"]


class OrgsClient(_BaseResource):
    """Manage organizations."""

    async def list(
        self,
        *,
        include_deleted: bool | None = None,
        limit: int | None = None,
        cursor: str | None = None,
    ) -> dict[str, Any]:
        q = self._query(
            includeDeleted=include_deleted,
            limit=limit,
            cursor=cursor,
        )
        return await self._client._request("GET", "/orgs", query=q)

    async def get_by_name(
        self,
        name: str,
        *,
        match: MatchMode = "exact",
    ) -> dict[str, Any]:
        return await _bridge_get_by_name(
            self._client,
            "orgs_get_by_name",
            name,
            match=match,
        )

    async def create(self, *, name: str, **extra: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/orgs", body={"name": name, **extra})

    async def get(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}")

    async def update(self, org_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/orgs/{org_id}", body=patch)

    async def delete(self, org_id: str) -> None:
        await self._client._request("DELETE", f"/orgs/{org_id}")

    async def purge(self, org_id: str) -> None:
        await self._client._request("DELETE", f"/orgs/{org_id}/purge")

    async def permissions(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}/permissions")

    async def resolve(
        self, org_id: str, *, query: str, resource_type: str | None = None
    ) -> dict[str, Any]:
        body: dict[str, Any] = {"query": query}
        if resource_type is not None:
            body["resource_type"] = resource_type
        return await self._client._request("POST", f"/orgs/{org_id}/resolve", body=body)

    async def create_alias(
        self,
        org_id: str,
        *,
        resource_type: str,
        resource_id: str,
        alias: str,
    ) -> dict[str, Any]:
        return await self._client._request(
            "POST",
            f"/orgs/{org_id}/aliases",
            body={
                "resource_type": resource_type,
                "resource_id": resource_id,
                "alias": alias,
            },
        )

    async def copilot_chat(self, org_id: str, *, message: str) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/orgs/{org_id}/copilot/chat", body={"message": message}
        )

    async def users(
        self, org_id: str, *, limit: int | None = None, cursor: str | None = None
    ) -> dict[str, Any]:
        q = self._query(limit=limit, cursor=cursor)
        return await self._client._request("GET", f"/orgs/{org_id}/users", query=q)

    async def upload_image(
        self,
        org_id: str,
        *,
        data: bytes,
        content_type: str,
        filename: str = "upload",
    ) -> dict[str, Any]:
        """Set the organization image via ``POST /orgs/{id}/image`` (multipart ``file``).

        ``content_type`` must be ``image/jpeg``, ``image/png``, or ``image/webp``,
        matching the dashboard upload rules.
        """

        return await self._client._request(
            "POST",
            f"/orgs/{org_id}/image",
            multipart=("file", filename, content_type, data),
        )

    async def upload_image_from_url(
        self,
        org_id: str,
        *,
        url: str,
        filename: str | None = None,
    ) -> dict[str, Any]:
        """Download an image from ``url`` (HTTP or HTTPS) and set the organization image."""

        data, content_type, default_name = await fetch_image_for_upload(url)
        return await self.upload_image(
            org_id,
            data=data,
            content_type=content_type,
            filename=filename or default_name,
        )
