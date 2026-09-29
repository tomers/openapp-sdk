"""Tests for localized name resolution error mapping."""

from __future__ import annotations

import json

from openapp_sdk.errors import (
    AmbiguousResourceError,
    ResourceNotFoundError,
    from_bridge_payload,
)


def test_resource_not_found_from_bridge_payload() -> None:
    err = from_bridge_payload(
        {
            "kind": "resource_not_found",
            "resource_type": "integration",
            "name": "Missing",
            "message": 'no integration named "Missing"',
        }
    )
    assert isinstance(err, ResourceNotFoundError)
    assert err.resource_type == "integration"
    assert err.name == "Missing"


def test_ambiguous_resource_from_bridge_payload() -> None:
    matches = [{"id": "1", "name": {"en": "Lobby"}}, {"id": "2", "name": {"en": "Lobby"}}]
    err = from_bridge_payload(
        {
            "kind": "ambiguous_resource",
            "resource_type": "portal",
            "name": "Lobby",
            "message": "portal name 'Lobby' is ambiguous (2 matches)",
            "matches_json": json.dumps(matches),
        }
    )
    assert isinstance(err, AmbiguousResourceError)
    assert err.resource_type == "portal"
    assert len(err.matches) == 2
