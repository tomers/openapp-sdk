"""OpenApp API-key token parsing.

Mirrors ``apps/backend/local_server/src/api_key_store.rs`` and
``openapp_sdk_common::token::ApiKey`` so the SDK can derive the base URL from a
token without a round-trip to the backend.
"""

from __future__ import annotations

from dataclasses import dataclass
from urllib.parse import urlparse

from .errors import ConfigError

__all__ = ["API_KEY_SEPARATOR", "ApiKey"]

API_KEY_SEPARATOR = "_openapp_"


@dataclass(frozen=True)
class ApiKey:
    """Parsed OpenApp API-key token."""

    base_url: str
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
        base_url, _, secret = token.partition(API_KEY_SEPARATOR)
        if not secret:
            raise ConfigError("API key secret is empty")

        parsed = urlparse(base_url)
        if not parsed.scheme or not parsed.netloc:
            raise ConfigError(f"API key base URL is not an absolute URL: {base_url!r}")

        return cls(base_url=base_url, secret=secret, raw=token)

    @property
    def bearer(self) -> str:
        return self.raw

    def __repr__(self) -> str:
        suffix = f"…{self.secret[-6:]}" if len(self.secret) > 6 else "…"
        return f"ApiKey(base_url={self.base_url!r}, secret={suffix!r})"
