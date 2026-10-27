"""`Users` sub-client."""

from __future__ import annotations

from typing import Any

from .._image_from_url import fetch_image_for_upload
from ._base import _BaseResource


class UsersClient(_BaseResource):
    """Create, search, update, and delete users."""

    async def create(
        self,
        *,
        name: dict[str, str],
        email: str | None = None,
        roles: dict[str, list[str]] | None = None,
        **extra: Any,
    ) -> dict[str, Any]:
        """Create a user (``CreateUserRequest``).

        ``name`` maps locale codes to the display name; ``roles`` maps org ids to role names.
        """
        body: dict[str, Any] = {"name": name, **extra}
        if email is not None:
            body["email"] = email
        if roles is not None:
            body["roles"] = roles
        return await self._client._request("POST", "/users", body=body)

    async def search(
        self,
        query: str,
        *,
        org_id: str | None = None,
        limit: int | None = None,
        offset: int | None = None,
    ) -> dict[str, Any]:
        q = self._query(q=query, org_id=org_id, limit=limit, offset=offset)
        return await self._client._request("GET", "/users/search", query=q)

    async def get(self, user_id: str) -> dict[str, Any]:
        return await self._client._request("GET", f"/users/{user_id}")

    async def update(self, user_id: str, **patch: Any) -> dict[str, Any]:
        return await self._client._request("PUT", f"/users/{user_id}", body=patch)

    async def delete(self, user_id: str) -> None:
        await self._client._request("DELETE", f"/users/{user_id}")

    async def purge(self, user_id: str) -> None:
        await self._client._request("DELETE", f"/users/{user_id}/purge")

    async def add_roles(self, user_id: str, roles: list[str]) -> dict[str, Any]:
        return await self._client._request("POST", f"/users/{user_id}/roles", body={"roles": roles})

    async def remove_roles(self, user_id: str, roles: list[str]) -> dict[str, Any]:
        return await self._client._request(
            "DELETE", f"/users/{user_id}/roles", body={"roles": roles}
        )

    async def upload_image(
        self,
        user_id: str,
        *,
        data: bytes,
        content_type: str,
        filename: str = "upload",
    ) -> dict[str, Any]:
        """Set the user avatar via ``POST /users/{id}/image`` (multipart ``file``).

        ``content_type`` must be ``image/jpeg``, ``image/png``, or ``image/webp``,
        matching the dashboard upload rules.
        """

        return await self._client._request(
            "POST",
            f"/users/{user_id}/image",
            multipart=("file", filename, content_type, data),
        )

    async def upload_image_from_url(
        self,
        user_id: str,
        *,
        url: str,
        filename: str | None = None,
    ) -> dict[str, Any]:
        """Download an image from ``url`` (HTTP or HTTPS) and set the user avatar.

        Same validation as :meth:`upload_image` after the bytes are fetched.
        """

        data, content_type, default_name = await fetch_image_for_upload(url)
        return await self.upload_image(
            user_id,
            data=data,
            content_type=content_type,
            filename=filename or default_name,
        )
