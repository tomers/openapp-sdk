"""Bridge selector picks the right implementation based on env."""

from __future__ import annotations

import importlib
import sys

import pytest


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

    assert get_bridge().name == "python"


def test_falls_back_when_rust_unavailable(monkeypatch: pytest.MonkeyPatch) -> None:
    # Simulate a wheel without the compiled Rust extension: mask out the
    # `openapp_sdk.bridge._bridge` module so `from .rust import RustBridge`
    # raises `ImportError`, triggering the python-fallback branch. The mask
    # must be applied *after* `_reload_bridge_module` — otherwise the module
    # teardown wipes out our `None` entry before selection runs.
    monkeypatch.delenv("OPENAPP_SDK_BRIDGE", raising=False)
    _reload_bridge_module()
    monkeypatch.setitem(sys.modules, "openapp_sdk.bridge._bridge", None)
    from openapp_sdk.bridge import get_bridge

    assert get_bridge().name == "python"
