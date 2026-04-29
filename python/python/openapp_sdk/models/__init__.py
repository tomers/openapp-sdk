"""Generated Pydantic v2 data models for every OpenAPI schema.

The file :mod:`openapp_sdk.models._generated` is produced by
``datamodel-code-generator`` from ``packages/api-spec/openapi.json``. Running
``just sdk-python openapi-gen`` rewrites it; ``just sdk-python openapi-check``
verifies it is up to date (and is wired into pre-commit).

Until the first generator run has produced committed output, this package
exposes no typed models — callers use plain ``dict[str, Any]`` payloads and opt
into typing via :mod:`openapp_sdk.contrib.pydantic` with their own Pydantic
models.
"""

from __future__ import annotations

try:  # pragma: no cover — populated post-generation
    from . import _generated as _gen
    from ._generated import *  # noqa: F403

    # `datamodel-code-generator` doesn't emit a module-level `__all__`, so
    # derive the public API from the module's own public names.
    __all__ = [name for name in dir(_gen) if not name.startswith("_")]
    del _gen
except ImportError:
    __all__ = []
