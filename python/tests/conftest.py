"""Shared pytest fixtures."""

from __future__ import annotations

import pytest

TEST_TOKEN = "https://api.test_openapp_TEST_SECRET"


@pytest.fixture
def token() -> str:
    return TEST_TOKEN
