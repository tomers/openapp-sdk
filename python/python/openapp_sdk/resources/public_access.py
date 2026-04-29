"""`Public Access` sub-client.

These endpoints run on opaque session / invite tokens embedded in the URL, not
on the OpenApp API key. They are exposed here so tools can drive them
programmatically (e.g. a concierge script that opens a door for a delivery
driver via a public invite).
"""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class PublicAccessClient(_BaseResource):
    # -- Invites -------------------------------------------------------------

    async def get_invite(self, invite_token: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/public/access/invites/{invite_token}")

    async def claim_invite(self, invite_token: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/invites/{invite_token}/claim", body=body
        )

    async def execute_invite(self, invite_token: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/invites/{invite_token}/execute", body=body
        )

    async def start_invite_session(self, invite_token: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/invites/{invite_token}/session", body=body
        )

    # -- Portals -------------------------------------------------------------

    async def get_portal(self, public_portal_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/public/access/portals/{public_portal_id}")

    async def portal_lights(self, public_portal_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/portals/{public_portal_id}/lights", body=body
        )

    async def portal_open(self, public_portal_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/portals/{public_portal_id}/open", body=body
        )

    async def portal_reachable(self, public_portal_id: str) -> dict[str, Any]:
        return await self._client._request(
            "GET", f"/public/access/portals/{public_portal_id}/reachable"
        )

    async def portal_start_session(self, public_portal_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/portals/{public_portal_id}/sessions", body=body
        )

    async def portal_targets(self, public_portal_id: str) -> dict[str, Any]:
        return await self._client._request(
            "GET", f"/public/access/portals/{public_portal_id}/targets"
        )

    # -- Sessions ------------------------------------------------------------

    async def get_session(self, session_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/public/access/sessions/{session_id}")

    async def cancel_session(self, session_id: str) -> dict[str, Any]:
        return await self._client._request("POST", f"/public/access/sessions/{session_id}/cancel")

    async def decline_session(self, session_id: str) -> dict[str, Any]:
        return await self._client._request("POST", f"/public/access/sessions/{session_id}/decline")

    async def session_lights(self, session_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/sessions/{session_id}/lights", body=body
        )

    async def session_notify_message(self, session_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/sessions/{session_id}/notify-message", body=body
        )

    async def session_open(self, session_id: str, **body: Any) -> dict[str, Any]:
        return await self._client._request(
            "POST", f"/public/access/sessions/{session_id}/open", body=body
        )

    async def session_streams(self, session_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/public/access/sessions/{session_id}/streams")
