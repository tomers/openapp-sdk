"""Exception hierarchy for the OpenApp Python SDK.

All exceptions raised by user-facing SDK calls derive from :class:`SdkError`.
The hierarchy mirrors the Rust ``openapp_sdk_core::error::SdkError`` enum so
cross-language logs correlate 1:1.

Example::

    from openapp_sdk import Client
    from openapp_sdk.errors import ApiError, AuthError

    client = Client.connect(api_key=...)
    try:
        client.orgs.create(name="")
    except ApiError as err:
        print(err.status, err.code, err.message)
    except AuthError:
        print("API key revoked, re-issue from the dashboard")
"""

from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
from typing import Any

__all__ = [
    "ApiError",
    "AuthError",
    "ConfigError",
    "HttpError",
    "SdkError",
    "SerializationError",
    "TransportError",
    "ValidationError",
    "from_bridge_payload",
]


class SdkError(Exception):
    """Root of every SDK-raised exception."""

    message: str

    def __init__(self, message: str):
        super().__init__(message)
        self.message = message


@dataclass
class ApiError(SdkError):
    """Raised when the API returned a non-2xx JSON envelope."""

    status: int
    message: str
    code: str | None = None
    correlation_id: str | None = None
    details: Any = None

    def __post_init__(self) -> None:
        # Keep `self.message` equal to the wire message so callers can read
        # it verbatim; format the status prefix only into `Exception.args[0]`
        # for the default `str()` rendering.
        Exception.__init__(self, f"[{self.status}] {self.message}")


@dataclass
class HttpError(SdkError):
    """Non-2xx response with a non-JSON body."""

    status: int
    message: str

    def __post_init__(self) -> None:
        Exception.__init__(self, f"[{self.status}] {self.message}")


class AuthError(SdkError):
    """Missing, malformed, or rejected credentials."""


class TransportError(SdkError):
    """DNS / TCP / TLS / timeout / unreachable."""


class ValidationError(SdkError):
    """Server response could not be decoded into the expected shape."""


class ConfigError(SdkError):
    """Invalid SDK configuration (bad URL, conflicting options, …)."""


class SerializationError(SdkError):
    """Caller-supplied data could not be serialized into the request body."""


# ---------------------------------------------------------------------------
# Bridge → typed-exception translation
# ---------------------------------------------------------------------------


def from_bridge_payload(payload: Mapping[str, Any]) -> SdkError:
    """Translate the structured payload emitted by the Rust bridge into a typed
    Python exception.

    The bridge always returns a dict with ``kind`` and ``message`` keys; extra
    keys (``status``, ``code``, ``correlation_id``, ``details_json``) appear for
    the variants that carry them.
    """

    kind = payload.get("kind", "other")
    message = str(payload.get("message", ""))
    status = payload.get("status")

    if kind == "api":
        details: Any = None
        details_json = payload.get("details_json")
        if isinstance(details_json, str) and details_json:
            import json

            try:
                details = json.loads(details_json)
            except ValueError:
                details = details_json
        return ApiError(
            status=int(status) if status is not None else 0,
            message=message,
            code=payload.get("code"),
            correlation_id=payload.get("correlation_id"),
            details=details,
        )
    if kind == "http":
        return HttpError(
            status=int(status) if status is not None else 0,
            message=message,
        )
    if kind == "auth":
        return AuthError(message)
    if kind == "transport":
        return TransportError(message)
    if kind in ("deserialize", "serialize"):
        return ValidationError(message) if kind == "deserialize" else SerializationError(message)
    if kind == "config":
        return ConfigError(message)
    return SdkError(message)


# Expose the common fields symmetry-friendly for library users.
ApiError.__module__ = __name__
HttpError.__module__ = __name__
