"""Shared pytest fixtures.

The default bridge is forced to ``python`` here so unit tests don't need the
Rust extension to be compiled. Integration tests opt back into whatever bridge
happens to be installed in CI via a dedicated fixture.
"""

from __future__ import annotations

import os

import pytest

os.environ.setdefault("OPENAPP_SDK_BRIDGE", "python")

TEST_TOKEN = "https://api.test_openapp_TEST_SECRET"


@pytest.fixture
def token() -> str:
    return TEST_TOKEN
