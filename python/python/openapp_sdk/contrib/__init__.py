"""Optional, opt-in extensions for the OpenApp SDK.

Everything in this package is gated behind an extra in ``pyproject.toml``:

* ``openapp-sdk[pydantic]`` — enables :mod:`openapp_sdk.contrib.pydantic`.

Users who do not want the extra dependency never pay for it at import time.
"""

__all__: list[str] = []
