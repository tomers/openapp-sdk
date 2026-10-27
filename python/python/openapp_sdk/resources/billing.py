"""`Billing` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class BillingClient(_BaseResource):
    """Read an organization's billing plan and quota usage."""

    async def plan(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}/billing/plan")

    async def usage(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}/billing/usage")
