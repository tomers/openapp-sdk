"""`Zones` sub-client."""

from __future__ import annotations

from collections.abc import Sequence
from typing import Any, Literal

from ._base import _BaseResource
from ._resolve import _bridge_get_by_name

MatchMode = Literal["exact", "fuzzy"]


class ZonesClient(_BaseResource):
    """Zones: rooms, floors, and areas owned by an organization.

    Org-scoped calls (``list``, ``tree``, ``create``, ``get_by_name``) take the
    org id explicitly and send it as ``X-Org``.
    """

    async def list(
        self,
        org_id: str,
        *,
        parent_zone_id: str | None = None,
        root: bool | None = None,
        q: str | None = None,
        include_deleted: bool | None = None,
        only_deleted: bool | None = None,
        limit: int | None = None,
        offset: int | None = None,
    ) -> dict[str, Any]:
        return await self._client._request(
            "GET",
            "/zones",
            query=self._query(
                parent_zone_id=parent_zone_id,
                root=root,
                q=q,
                include_deleted=include_deleted,
                only_deleted=only_deleted,
                limit=limit,
                offset=offset,
            ),
            headers={"X-Org": org_id},
        )

    async def tree(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", "/zones/tree", headers={"X-Org": org_id})

    async def create(self, org_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/zones", body=body, headers={"X-Org": org_id})

    async def get(self, zone_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/zones/{zone_id}")

    async def summary(self, zone_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/zones/{zone_id}/summary")

    async def update(self, zone_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/zones/{zone_id}", body=patch)

    async def delete(self, zone_id: str, *, detach: bool = False) -> None:
        """Soft-delete. ``detach=True`` clears references first (else 409 ``zone_in_use``)."""
        await self._client._request(
            "DELETE",
            f"/zones/{zone_id}",
            query=self._query(cascade="detach" if detach else None),
        )

    async def purge(self, zone_id: str) -> None:
        await self._client._request("DELETE", f"/zones/{zone_id}/purge")

    async def merge(self, zone_id: str, source_zone_ids: Sequence[str]) -> dict[str, Any]:
        return await self._client._request(
            "POST",
            f"/zones/{zone_id}/merge",
            body={"source_zone_ids": [*source_zone_ids]},
        )

    async def get_by_name(
        self,
        org_id: str,
        name: str,
        *,
        match: MatchMode = "exact",
    ) -> dict[str, Any]:
        return await _bridge_get_by_name(
            self._client,
            "zones_get_by_name",
            org_id,
            name,
            match=match,
        )
