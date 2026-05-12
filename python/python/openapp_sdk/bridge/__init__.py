"""Private bridge subpackage.

Users never import from here directly. The high-level :mod:`openapp_sdk.client`
module always uses :class:`~openapp_sdk.bridge.rust.RustBridge`, backed by the
compiled :mod:`openapp_sdk.bridge._bridge` Rust/PyO3 extension shipped in
``openapp-sdk`` wheels.
"""

from __future__ import annotations

import os

from .base import Bridge, BridgeClient

_BRIDGE_ENV = "OPENAPP_SDK_BRIDGE"

__all__ = ["Bridge", "BridgeClient", "get_bridge"]


def _select_bridge() -> Bridge:
    explicit = os.environ.get(_BRIDGE_ENV, "").strip().lower()
    if explicit in ("", "auto", "rust"):
        from .rust import RustBridge

        return RustBridge()

    raise ValueError(f"invalid {_BRIDGE_ENV}={explicit!r}; expected one of 'rust' / 'auto'")


_cached: Bridge | None = None


def get_bridge() -> Bridge:
    """Return (and memoise) the process-wide bridge implementation."""

    global _cached
    if _cached is None:
        _cached = _select_bridge()
    return _cached
