"""Token parsing mirrors the Rust-side `ApiKey::parse` behaviour."""

from __future__ import annotations

import pytest
from openapp_sdk import ApiKey
from openapp_sdk.errors import ConfigError


def test_parses_valid_token() -> None:
    key = ApiKey.parse("https://openapp.house_openapp_SECRET")
    assert key.origin == "https://openapp.house"
    assert key.secret == "SECRET"
    assert key.raw == "https://openapp.house_openapp_SECRET"


def test_api_base_url_appends_versioned_prefix() -> None:
    assert ApiKey.parse("https://openapp.house_openapp_S").api_base_url == (
        "https://openapp.house/api/v1"
    )
    assert ApiKey.parse("http://oathkeeper:4455_openapp_S").api_base_url == (
        "http://oathkeeper:4455/api/v1"
    )


def test_rejects_origin_with_path() -> None:
    with pytest.raises(ConfigError, match="bare origin"):
        ApiKey.parse("https://openapp.house/api/v1_openapp_SECRET")


def test_rejects_non_http_scheme() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("ftp://openapp.house_openapp_SECRET")


def test_rejects_empty_token() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("")
    with pytest.raises(ConfigError):
        ApiKey.parse("   ")


def test_rejects_missing_separator() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("https://openapp.house")


def test_rejects_empty_secret() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("https://openapp.house_openapp_")


def test_rejects_non_url_base() -> None:
    with pytest.raises(ConfigError):
        ApiKey.parse("not a url_openapp_SECRET")


def test_repr_hides_secret() -> None:
    key = ApiKey.parse("https://openapp.house_openapp_supersecret")
    rendered = repr(key)
    assert "supersecret" not in rendered
