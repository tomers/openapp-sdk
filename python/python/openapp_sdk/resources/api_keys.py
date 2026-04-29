"""`API Keys` sub-client."""

from __future__ import annotations

from builtins import list as _list
from typing import Any

from ._base import _BaseResource


class ApiKeysClient(_BaseResource):
    """Manage the API keys issued to the caller's org.

    Tokens returned by :meth:`create` follow the format
    ``{base_url}_openapp_{secret}``. The SDK can auto-derive the base URL from
    such a token; see :class:`openapp_sdk.ApiKey`.
    """

    async def list(self) -> _list[dict[str, Any]]:
        return await self._client._request("GET", "/api-keys")

    async def create(
        self,
        *,
        label: str | None = None,
        scopes: _list[str] | None = None,
        expires_at: str | None = None,
        **extra: Any,
    ) -> dict[str, Any]:
        body: dict[str, Any] = dict(extra)
        if label is not None:
            body["label"] = label
        if scopes is not None:
            body["scopes"] = scopes
        if expires_at is not None:
            body["expiresAt"] = expires_at
        return await self._client._request("POST", "/api-keys", body=body)

    async def update(
        self,
        key_id: str,
        *,
        label: str | None = None,
        **extra: Any,
    ) -> dict[str, Any]:
        body: dict[str, Any] = dict(extra)
        if label is not None:
            body["label"] = label
        return await self._client._request("PATCH", f"/api-keys/{key_id}", body=body)

    async def revoke(self, key_id: str) -> None:
        await self._client._request("DELETE", f"/api-keys/{key_id}")

    async def restore(self, key_id: str) -> dict[str, Any]:
        return await self._client._request("POST", f"/api-keys/{key_id}/restore")

    async def purge(self, key_id: str) -> None:
        await self._client._request("DELETE", f"/api-keys/{key_id}/purge")
