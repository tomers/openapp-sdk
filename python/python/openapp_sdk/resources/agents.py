"""`Agents` sub-client — named non-human principals."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class AgentsClient(_BaseResource):
    """Manage Agent principals and their scoped credentials."""

    async def list(self) -> dict[str, Any]:
        return await self._client._request("GET", "/agents")

    async def create(self, *, name: str, **extra: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/agents", body={"name": name, **extra})

    async def get(self, agent_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/agents/{agent_id}")

    async def update(self, agent_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PATCH", f"/agents/{agent_id}", body=patch)

    async def revoke(self, agent_id: str) -> None:
        await self._client._request("DELETE", f"/agents/{agent_id}")

    async def create_credential(self, agent_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/agents/{agent_id}/credentials", body=body or {}
        )
