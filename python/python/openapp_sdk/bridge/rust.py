"""Rust/PyO3 bridge implementation (requires the compiled :mod:`_bridge`).

Imported lazily by :func:`openapp_sdk.bridge.get_bridge`. If the compiled
extension is unavailable, client construction fails clearly instead of falling
back to a separate Python HTTP transport.
"""

from __future__ import annotations

import json
from typing import Any

from ..errors import SdkError, from_bridge_payload
from .base import Bridge, BridgeClient, BridgeRequest

try:
    from . import _bridge  # type: ignore[attr-defined]
except ImportError as exc:  # pragma: no cover
    raise ImportError(
        "openapp_sdk.bridge._bridge is not available; install a wheel from PyPI or "
        "build the extension with `just sdk::python::build-develop`."
    ) from exc


class _RustClient(BridgeClient):
    def __init__(self, inner: Any) -> None:
        self._inner = inner
        self.base_url: str = inner.base_url

    async def request(self, req: BridgeRequest) -> Any:
        try:
            if req.multipart is not None:
                field, filename, content_type, data = req.multipart
                raw = await self._inner.multipart_post(
                    req.path,
                    field,
                    filename,
                    content_type,
                    data,
                    list(req.query) if req.query else None,
                    req.timeout_secs,
                )
            else:
                raw = await self._inner.request(
                    req.method.upper(),
                    req.path,
                    req.body_json,
                    list(req.query) if req.query else None,
                    req.timeout_secs,
                )
        except ValueError as exc:
            # PyO3 surfaces structured errors via ValueError with a dict payload.
            payload = exc.args[0] if exc.args else {}
            if isinstance(payload, dict):
                raise from_bridge_payload(payload) from None
            raise SdkError(str(exc)) from None

        if raw in (None, "", "null"):
            return None
        if isinstance(raw, str):
            return json.loads(raw)
        return raw

    async def close(self) -> None:
        self._inner = None

    def with_org(self, org: str) -> BridgeClient:
        return _RustClient(self._inner.with_org(org))

    async def integrations_get_by_name(
        self,
        name: str,
        *,
        match: str = "exact",
        provider_type: str | None = None,
    ) -> Any:
        return await self._resolve_call(
            self._inner.integrations_get_by_name(name, match, provider_type)
        )

    async def integrations_get_access_portal_by_name(
        self,
        integration_id: str,
        name: str,
        *,
        match: str = "exact",
    ) -> Any:
        return await self._resolve_call(
            self._inner.integrations_get_access_portal_by_name(integration_id, name, match)
        )

    async def devices_get_by_name(
        self,
        name: str,
        *,
        match: str = "exact",
        integration_id: str | None = None,
    ) -> Any:
        return await self._resolve_call(
            self._inner.devices_get_by_name(name, match, integration_id)
        )

    async def zones_get_by_name(
        self,
        integration_id: str,
        name: str,
        *,
        match: str = "exact",
    ) -> Any:
        return await self._resolve_call(self._inner.zones_get_by_name(integration_id, name, match))

    async def orgs_get_by_name(self, name: str, *, match: str = "exact") -> Any:
        return await self._resolve_call(self._inner.orgs_get_by_name(name, match))

    async def _resolve_call(self, coro: Any) -> Any:
        try:
            raw = await coro
        except ValueError as exc:
            payload = exc.args[0] if exc.args else {}
            if isinstance(payload, dict):
                raise from_bridge_payload(payload) from None
            raise SdkError(str(exc)) from None
        if raw in (None, "", "null"):
            return None
        if isinstance(raw, str):
            return json.loads(raw)
        return raw


class RustBridge(Bridge):
    name = "rust"

    def __init__(self) -> None:
        # Instantiating Runtime lazily so importing the module never fails.
        self._runtime = _bridge.Runtime()

    def new_client(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
        org: str | None = None,
    ) -> BridgeClient:
        inner = _bridge.Client(
            api_key,
            base_url or None,
            user_agent,
            timeout_secs,
            max_retries,
            org,
        )
        return _RustClient(inner)
