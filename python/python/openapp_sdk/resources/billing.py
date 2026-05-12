"""`Billing` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class BillingClient(_BaseResource):
    """Access organization billing plan, checkout, portal, and usage endpoints."""

    async def checkout(self, org_id: str, *, tier_slug: str) -> dict[str, Any]:
        return await self._client._request(
            "POST",
            f"/orgs/{org_id}/billing/checkout",
            body={"tier_slug": tier_slug},
        )

    async def plan(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}/billing/plan")

    async def portal(self, org_id: str, *, return_url: str) -> dict[str, Any]:
        return await self._client._request(
            "POST",
            f"/orgs/{org_id}/billing/portal",
            body={"return_url": return_url},
        )

    async def upgrade_options(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}/billing/upgrade-options")

    async def usage(self, org_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/orgs/{org_id}/billing/usage")
