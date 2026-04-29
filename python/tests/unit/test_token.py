"""Token parsing mirrors the Rust-side `ApiKey::parse` behaviour."""

from __future__ import annotations

import pytest
from openapp_sdk import ApiKey
from openapp_sdk.errors import ConfigError


def test_parses_valid_token() -> None:
    key = ApiKey.parse("https://api.openapp.house/api/v1_openapp_SECRET")
    assert key.base_url == "https://api.openapp.house/api/v1"
    assert key.secret == "SECRET"
    assert key.bearer == "https://api.openapp.house/api/v1_openapp_SECRET"


def test_rejects_empty_token() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("")
    with pytest.raises(ConfigError):
        ApiKey.parse("   ")


def test_rejects_missing_separator() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("https://api.openapp.house/api/v1")


def test_rejects_empty_secret() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("https://api.openapp.house/api/v1_openapp_")


def test_rejects_non_url_base() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("not a url_openapp_SECRET")


def test_repr_hides_secret() -> None:
    key = ApiKey.parse("https://api.openapp.house/api/v1_openapp_supersecret")
    rendered = repr(key)
    assert "supersecret" not in rendered
