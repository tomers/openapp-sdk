"""Optional provider verification using pacts resolved from a Pact Broker.

Skipped unless ``PACT_BROKER_VERIFY=1`` and broker auth plus a reachable
provider base URL are configured. Intended for staging/canary pipelines, not
default PR CI.
"""

from __future__ import annotations

import os

import pytest
from pact import Verifier

pytestmark = pytest.mark.skipif(
    os.environ.get("PACT_BROKER_VERIFY") != "1",
    reason="PACT_BROKER_VERIFY is not set to 1",
)


@pytest.mark.contract
def test_openapp_http_api_against_broker() -> None:
    broker = os.environ.get("PACT_BROKER_BASE_URL", "").rstrip("/")
    provider = os.environ.get("PACT_PROVIDER_BASE_URL", "").rstrip("/")
    if not broker or not provider:
        pytest.skip("PACT_BROKER_BASE_URL and PACT_PROVIDER_BASE_URL must be set")

    token = os.environ.get("PACT_BROKER_TOKEN")
    user = os.environ.get("PACT_BROKER_USERNAME")
    password = os.environ.get("PACT_BROKER_PASSWORD")

    verifier = Verifier("openapp-http-api").add_transport(url=provider)
    if token:
        verifier = verifier.broker_source(broker, token=token)
    elif user and password:
        verifier = verifier.broker_source(broker, username=user, password=password)
    else:
        pytest.skip("Set PACT_BROKER_TOKEN or PACT_BROKER_USERNAME+PACT_BROKER_PASSWORD")

    verifier.verify()
