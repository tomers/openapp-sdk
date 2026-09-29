"""Tests for get_by_name resource client helpers."""

from __future__ import annotations

from typing import Any

import pytest
from openapp_sdk import AsyncClient
from openapp_sdk.bridge.base import BridgeClient, BridgeRequest
from openapp_sdk.errors import AmbiguousResourceError, ResourceNotFoundError


class _ResolveBridgeClient(BridgeClient):
    def __init__(self) -> None:
        self.base_url = "https://api.test"

    def with_org(self, org: str) -> BridgeClient:
        # Name resolution in this fake is org-independent, so it is already its
        # own org-scoped equivalent.
        return self

    async def request(self, req: BridgeRequest) -> Any:
        raise AssertionError(f"unexpected HTTP request: {req.method} {req.path}")

    async def close(self) -> None:
        return None

    async def integrations_get_by_name(
        self,
        name: str,
        *,
        match: str = "exact",
        provider_type: str | None = None,
    ) -> dict[str, Any]:
        if name == "Missing":
            raise ResourceNotFoundError(resource_type="integration", name="Missing")
        if name == "Lobby":
            raise AmbiguousResourceError(
                resource_type="integration",
                name="Lobby",
                matches=[{"id": "1"}, {"id": "2"}],
            )
        return {"id": "01HINT", "name": {"en": name}}

    async def devices_get_by_name(
        self,
        name: str,
        *,
        match: str = "exact",
        integration_id: str | None = None,
    ) -> dict[str, Any]:
        return {"id": "01HDEV", "name": {"en": name}, "integration_id": integration_id}

    async def zones_get_by_name(
        self,
        integration_id: str,
        name: str,
        *,
        match: str = "exact",
    ) -> dict[str, Any]:
        return {"id": "01HZONE", "name": {"en": name}, "integration_id": integration_id}

    async def orgs_get_by_name(self, name: str, *, match: str = "exact") -> dict[str, Any]:
        return {"id": "01HORG", "name": {"en": name}}


@pytest.fixture
def client(monkeypatch: pytest.MonkeyPatch) -> AsyncClient:
    bridge = _ResolveBridgeClient()

    import openapp_sdk.client as client_module

    monkeypatch.setattr(
        client_module,
        "get_bridge",
        lambda: type("Bridge", (), {"new_client": lambda *a, **k: bridge})(),
    )
    return AsyncClient(
        bridge_client=bridge,
        config=client_module.ClientConfig(
            base_url="https://api.test",
            user_agent="test",
            timeout_secs=1.0,
            max_retries=0,
        ),
    )


@pytest.mark.asyncio
async def test_integrations_get_by_name_happy_path(client: AsyncClient) -> None:
    integration = await client.integrations.get_by_name("Lobby Demo")
    assert integration["id"] == "01HINT"


@pytest.mark.asyncio
async def test_integrations_get_by_name_not_found(client: AsyncClient) -> None:
    with pytest.raises(ResourceNotFoundError):
        await client.integrations.get_by_name("Missing")


@pytest.mark.asyncio
async def test_integrations_get_by_name_ambiguous(client: AsyncClient) -> None:
    with pytest.raises(AmbiguousResourceError) as exc:
        await client.integrations.get_by_name("Lobby")
    assert len(exc.value.matches) == 2


@pytest.mark.asyncio
async def test_devices_get_by_name(client: AsyncClient) -> None:
    device = await client.devices.get_by_name("Front Door", integration_id="01HINT")
    assert device["id"] == "01HDEV"


@pytest.mark.asyncio
async def test_zones_get_by_name(client: AsyncClient) -> None:
    zone = await client.zones.get_by_name("01HINT", "Building A")
    assert zone["id"] == "01HZONE"


@pytest.mark.asyncio
async def test_orgs_get_by_name(client: AsyncClient) -> None:
    org = await client.orgs.get_by_name("Acme HQ")
    assert org["id"] == "01HORG"
