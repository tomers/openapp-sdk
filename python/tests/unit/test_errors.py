"""Unit tests for bridge-payload → typed-exception translation."""

from __future__ import annotations

from openapp_sdk.errors import (
    ApiError,
    AuthError,
    ConfigError,
    HttpError,
    SerializationError,
    TransportError,
    ValidationError,
    from_bridge_payload,
)


def test_api_payload() -> None:
    err = from_bridge_payload(
        {
            "kind": "api",
            "status": 404,
            "message": "not found",
            "code": "not_found",
            "correlation_id": "abc",
            "details_json": '{"hint": "missing id"}',
        }
    )
    assert isinstance(err, ApiError)
    assert err.status == 404
    assert err.code == "not_found"
    assert err.correlation_id == "abc"
    assert err.details == {"hint": "missing id"}


def test_http_payload() -> None:
    err = from_bridge_payload({"kind": "http", "status": 503, "message": "down"})
    assert isinstance(err, HttpError)
    assert err.status == 503


def test_auth_payload() -> None:
    assert isinstance(from_bridge_payload({"kind": "auth", "message": "nope"}), AuthError)


def test_transport_payload() -> None:
    assert isinstance(
        from_bridge_payload({"kind": "transport", "message": "reset"}), TransportError
    )


def test_deserialize_payload() -> None:
    assert isinstance(
        from_bridge_payload({"kind": "deserialize", "message": "bad"}), ValidationError
    )


def test_serialize_payload() -> None:
    assert isinstance(
        from_bridge_payload({"kind": "serialize", "message": "bad"}),
        SerializationError,
    )


def test_config_payload() -> None:
    assert isinstance(from_bridge_payload({"kind": "config", "message": "bad"}), ConfigError)
