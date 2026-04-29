"""`Auth` sub-client."""

from __future__ import annotations

from typing import Any

from ._base import _BaseResource


class AuthClient(_BaseResource):
    """Auth-adjacent endpoints (identity introspection, sign-out)."""

    async def whoami(self) -> dict[str, Any]:
        return await self._client._request("GET", "/auth/whoami")

    async def session(self) -> dict[str, Any]:
        return await self._client._request("GET", "/auth/session")

    async def kratos_identity(self) -> dict[str, Any]:
        return await self._client._request("GET", "/auth/kratos-identity")

    async def provisioned(self) -> dict[str, Any]:
        return await self._client._request("GET", "/auth/provisioned")

    async def logout(self) -> None:
        await self._client._request("POST", "/auth/logout")

    async def sign_out(self) -> None:
        await self._client._request("POST", "/auth/sign-out")
