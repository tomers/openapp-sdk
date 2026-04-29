"""`Entities` sub-client."""

from __future__ import annotations

from builtins import list as _list
from typing import Any

from .._image_from_url import fetch_image_for_upload
from ._base import _BaseResource


class EntityHandle:
    """Entity-scoped fluent helper returned by :meth:`EntitiesClient.by_id`."""

    def __init__(self, entities: EntitiesClient, entity_id: str) -> None:
        self._entities = entities
        self._entity_id = entity_id

    async def action(self, action_id: str, **body: Any) -> dict[str, Any]:
        """Invoke any entity action by action id."""

        return await self._entities.actions(self._entity_id, action_id, **body)

    async def open(self, **body: Any) -> dict[str, Any]:
        return await self.action("open", **body)

    async def close(self, **body: Any) -> dict[str, Any]:
        return await self.action("close", **body)

    async def on(self, **body: Any) -> dict[str, Any]:
        return await self.action("on", **body)

    async def off(self, **body: Any) -> dict[str, Any]:
        return await self.action("off", **body)


class EntitiesClient(_BaseResource):
    """Entities: logical objects (apartments, gates, cameras…) attached to devices."""

    async def list(
        self,
        *,
        org_id: str | None = None,
        device_id: str | None = None,
        entity_type: str | None = None,
        include_deleted: bool | None = None,
        limit: int | None = None,
        cursor: str | None = None,
    ) -> dict[str, Any]:
        q = self._query(
            orgId=org_id,
            deviceId=device_id,
            entityType=entity_type,
            includeDeleted=include_deleted,
            limit=limit,
            cursor=cursor,
        )
        return await self._client._request("GET", "/entities", query=q)

    async def create(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/entities", body=body)

    async def get(self, entity_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/entities/{entity_id}")

    async def update(self, entity_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/entities/{entity_id}", body=patch)

    async def patch(self, entity_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PATCH", f"/entities/{entity_id}", body=patch)

    async def delete(self, entity_id: str) -> None:
        await self._client._request("DELETE", f"/entities/{entity_id}")

    async def purge(self, entity_id: str) -> None:
        await self._client._request("DELETE", f"/entities/{entity_id}/purge")

    async def restore(self, entity_id: str) -> dict[str, Any]:
        return await self._client._request("POST", f"/entities/{entity_id}/restore")

    async def actions(self, entity_id: str, action_id: str, **body: Any) -> dict[str, Any]:
        """`POST /entities/{id}/actions/{action_id}` — invoke an entity action.

        Example::

            client.entities.actions(entity_id=eid, action_id="open")
        """

        return await self._client._request(
            "POST", f"/entities/{entity_id}/actions/{action_id}", body=body or {}
        )

    def by_id(self, entity_id: str) -> EntityHandle:
        """Return a fluent entity handle for action-style operations."""

        return EntityHandle(self, entity_id)

    async def metadata_definition(self, entity_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/entities/{entity_id}/metadata-definition")

    async def by_device(self, device_id: str) -> _list[dict[str, Any]]:
        return await self._client._request("GET", f"/devices/{device_id}/entities")

    async def device_entities_metadata_definition(
        self, device_id: str, *, entity_type: str | None = None
    ) -> dict[str, Any]:
        q = self._query(entityType=entity_type)
        return await self._client._request(
            "GET",
            f"/devices/{device_id}/entities/metadata-definition",
            query=q,
        )

    async def apartment_floors(self, device_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/devices/{device_id}/apartment-floors")

    async def upload_image(
        self,
        entity_id: str,
        *,
        data: bytes,
        content_type: str,
        filename: str = "upload",
    ) -> dict[str, Any]:
        """Set the entity image via ``POST /entities/{id}/image`` (multipart ``file``).

        ``content_type`` must be ``image/jpeg``, ``image/png``, or ``image/webp``,
        matching the dashboard upload rules.
        """

        return await self._client._request(
            "POST",
            f"/entities/{entity_id}/image",
            multipart=("file", filename, content_type, data),
        )

    async def upload_image_from_url(
        self,
        entity_id: str,
        *,
        url: str,
        filename: str | None = None,
    ) -> dict[str, Any]:
        """Download an image from ``url`` (HTTP or HTTPS) and set the entity image."""

        data, content_type, default_name = await fetch_image_for_upload(url)
        return await self.upload_image(
            entity_id,
            data=data,
            content_type=content_type,
            filename=filename or default_name,
        )
