"""Live-API smoke tests.

Skipped unless ``OPENAPP_SDK_TEST_API_KEY`` is set. In CI this is populated from
the dev-env API key minted by ``apps/backend/cli/src/provision.rs`` so the suite
exercises the real wire contract against a running backend.

These tests are intentionally small — the unit suite covers behavior
exhaustively. The goal here is to catch contract drift, not reimplement unit
coverage.
"""

from __future__ import annotations

import os

import pytest
from openapp_sdk import AsyncClient

pytestmark = pytest.mark.skipif(
    not os.environ.get("OPENAPP_SDK_TEST_API_KEY"),
    reason="OPENAPP_SDK_TEST_API_KEY not set; skipping live-API tests",
)


@pytest.fixture
async def live_client() -> AsyncClient:
    token = os.environ["OPENAPP_SDK_TEST_API_KEY"]
    client = await AsyncClient.connect(api_key=token)
    try:
        yield client
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_status_reachable(live_client: AsyncClient) -> None:
    status = await live_client.status.get()
    assert isinstance(status, dict)


@pytest.mark.asyncio
async def test_whoami_returns_identity(live_client: AsyncClient) -> None:
    who = await live_client.auth.whoami()
    assert isinstance(who, dict)
    assert who  # non-empty


@pytest.mark.asyncio
async def test_orgs_listable(live_client: AsyncClient) -> None:
    result = await live_client.orgs.list()
    # The paginated list response may be a dict with "items" or a list; accept both
    # so the test stays useful even as the spec evolves.
    assert isinstance(result, (dict, list))
