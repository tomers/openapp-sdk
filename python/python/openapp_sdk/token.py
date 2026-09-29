"""OpenApp API-key token parsing.

Mirrors ``apps/backend/local_server/src/api_key_store.rs`` and
``openapp_sdk_common::token::ApiKey``: tokens are ``{origin}_openapp_{secret}``,
where ``origin`` is the deployment's bare public origin (scheme, host, optional
port — never a path). The API root is ``{origin}/api/v1``, so the SDK can derive
it from a token without a round-trip to the backend. The full token travels in
the ``X-API-Key`` header (see ``notes/contracts/api-key-authentication.md``).
"""

from __future__ import annotations

from dataclasses import dataclass
from urllib.parse import urlparse

from .errors import ConfigError

__all__ = ["API_KEY_SEPARATOR", "API_PATH_PREFIX", "ApiKey"]

API_KEY_SEPARATOR = "_openapp_"
API_PATH_PREFIX = "/api/v1"


@dataclass(frozen=True)
class ApiKey:
    """Parsed OpenApp API-key token."""

    origin: str
    secret: str
    raw: str

    @classmethod
    def parse(cls, token: str) -> ApiKey:
        if not token or not token.strip():
            raise ConfigError("API key is empty")
        token = token.strip()
        if API_KEY_SEPARATOR not in token:
            raise ConfigError(
                "API key does not contain the `_openapp_` separator — did you paste the full token?"
            )
        origin, _, secret = token.partition(API_KEY_SEPARATOR)
        if not secret:
            raise ConfigError("API key secret is empty")

        parsed = urlparse(origin)
        if parsed.scheme not in ("http", "https") or not parsed.netloc:
            raise ConfigError(f"API key origin is not an absolute http(s) URL: {origin!r}")
        if parsed.path not in ("", "/") or parsed.query or parsed.fragment or parsed.params:
            raise ConfigError(f"API key origin must be a bare origin with no path: {origin!r}")

        return cls(origin=origin.rstrip("/"), secret=secret, raw=token)

    @property
    def api_base_url(self) -> str:
        """The versioned API root requests are resolved against: ``{origin}/api/v1``."""
        return f"{self.origin}{API_PATH_PREFIX}"

    def __repr__(self) -> str:
        suffix = f"…{self.secret[-6:]}" if len(self.secret) > 6 else "…"
        return f"ApiKey(origin={self.origin!r}, secret={suffix!r})"
