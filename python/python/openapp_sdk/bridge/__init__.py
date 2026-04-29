"""Private bridge subpackage.

Users never import from here directly. The high-level :mod:`openapp_sdk.client`
module picks the best available implementation:

* :class:`~openapp_sdk.bridge.rust.RustBridge` — backed by the compiled
  :mod:`openapp_sdk.bridge._bridge` Rust/PyO3 extension (shipped via prebuilt
  wheels from ``pip install openapp-sdk``).
* :class:`~openapp_sdk.bridge.python_fallback.PythonBridge` — pure-Python
  :mod:`httpx`-based transport. Used on platforms with no wheel and inside the
  test suite so contributors can iterate without a Rust toolchain.

The selection policy is:

1. If the environment variable ``OPENAPP_SDK_BRIDGE`` is set, it wins
   (``rust`` / ``python``).
2. Otherwise the Rust bridge is preferred when importable; the Python bridge
   is used as a fallback.
"""

from __future__ import annotations

import os

from .base import Bridge, BridgeClient

_BRIDGE_ENV = "OPENAPP_SDK_BRIDGE"

__all__ = ["Bridge", "BridgeClient", "get_bridge"]


def _select_bridge() -> Bridge:
    explicit = os.environ.get(_BRIDGE_ENV, "").strip().lower()
    if explicit == "rust":
        from .rust import RustBridge

        return RustBridge()
    if explicit == "python":
        from .python_fallback import PythonBridge

        return PythonBridge()
    if explicit and explicit not in ("auto", ""):
        raise ValueError(
            f"invalid {_BRIDGE_ENV}={explicit!r}; expected one of 'rust' / 'python' / 'auto'"
        )

    try:
        from .rust import RustBridge

        return RustBridge()
    except ImportError:
        from .python_fallback import PythonBridge

        return PythonBridge()


_cached: Bridge | None = None


def get_bridge() -> Bridge:
    """Return (and memoise) the process-wide bridge implementation."""

    global _cached
    if _cached is None:
        _cached = _select_bridge()
    return _cached
