"""Explicit zone sanity gate (off by default).

Set ``OPENAPP_ZONE_SANITY=1`` and ``OPENAPP_SDK_TEST_API_KEY`` to run the same
minimal live checks as integration tests, but tagged for tier-2 / manual runs.
"""

from __future__ import annotations

import os

import pytest

pytestmark = pytest.mark.skipif(
    os.environ.get("OPENAPP_ZONE_SANITY") != "1",
    reason="OPENAPP_ZONE_SANITY is not set to 1",
)


@pytest.mark.zone_sanity
@pytest.mark.skipif(
    not os.environ.get("OPENAPP_SDK_TEST_API_KEY"),
    reason="OPENAPP_SDK_TEST_API_KEY not set",
)
@pytest.mark.asyncio
async def test_zone_status_reachable() -> None:
    from openapp_sdk import AsyncClient

    token = os.environ["OPENAPP_SDK_TEST_API_KEY"]
    client = await AsyncClient.connect(api_key=token)
    try:
        status = await client.status.get()
        assert isinstance(status, dict)
    finally:
        await client.close()
