"""`LAN agent` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class LanAgentClient(_BaseResource):
    """LAN agent lifecycle — bootstrap tokens, task queue, metadata."""

    async def meta(self) -> dict[str, Any]:
        return await self._client._request("GET", "/lan-agent/meta")

    async def bootstrap_script(self) -> dict[str, Any]:
        return await self._client._request("GET", "/lan-agent/cli/bootstrap.sh")

    async def bootstrap_token(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/lan-agent/cli/bootstrap-token", body=body)

    async def token(self, **body: Any) -> dict[str, Any]:
        return await self._client._request("POST", "/lan-agent/cli/token", body=body)

    async def submit_task_spec(self, integration_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/integrations/{integration_id}/lan-agent/task-spec", body=body
        )

    async def list_tasks(self, integration_id: str) -> list[dict[str, Any]]:
        return await self._client._request("GET", f"/integrations/{integration_id}/lan-agent/tasks")
