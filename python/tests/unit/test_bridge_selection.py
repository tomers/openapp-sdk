"""Bridge selector picks the right implementation based on env."""

from __future__ import annotations

import importlib
import sys

import pytest

pytestmark = pytest.mark.tier_0


def _reload_bridge_module() -> None:
    # Forcing a fresh import re-runs the cache-aware selector.
    mod_names = [n for n in list(sys.modules) if n.startswith("openapp_sdk.bridge")]
    for name in mod_names:
        del sys.modules[name]
    importlib.invalidate_caches()


def test_python_bridge_selected_by_env(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("OPENAPP_SDK_BRIDGE", "python")
    _reload_bridge_module()
    from openapp_sdk.bridge import get_bridge

    with pytest.raises(ValueError, match="expected one of 'rust' / 'auto'"):
        get_bridge()


def test_rust_bridge_required_when_native_extension_unavailable(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    monkeypatch.delenv("OPENAPP_SDK_BRIDGE", raising=False)
    _reload_bridge_module()
    monkeypatch.setitem(sys.modules, "openapp_sdk.bridge._bridge", None)
    from openapp_sdk.bridge import get_bridge

    with pytest.raises(ImportError, match=r"openapp_sdk\.bridge\._bridge is not available"):
        get_bridge()
