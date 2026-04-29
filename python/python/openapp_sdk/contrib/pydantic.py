"""Opt-in Pydantic v2 payload converter.

Installed via the ``pydantic`` extra (``pip install openapp-sdk[pydantic]``).
When this module is imported, request bodies that are Pydantic v2 models are
converted via :meth:`model_dump(mode="json", exclude_none=True)` and responses
can be parsed into specific models via :func:`parse_as`.

Example::

    from pydantic import BaseModel
    from openapp_sdk import Client
    from openapp_sdk.contrib.pydantic import parse_as

    class Org(BaseModel):
        id: str
        name: str

    client = Client.connect(api_key=...)
    raw = client.orgs.get("org_123")
    org = parse_as(Org, raw)
"""

from __future__ import annotations

from typing import Any, TypeVar

try:
    from pydantic import BaseModel, TypeAdapter
except ImportError as exc:  # pragma: no cover
    raise ImportError(
        "openapp_sdk.contrib.pydantic requires pydantic>=2. "
        "Install the extra: `pip install openapp-sdk[pydantic]`."
    ) from exc

T = TypeVar("T", bound=BaseModel)

__all__ = ["TypeAdapter", "dump", "parse_as"]


def parse_as(model: type[T], data: Any) -> T:
    """Parse ``data`` (a dict or JSON-like structure) into ``model``."""

    return model.model_validate(data)


def dump(model: BaseModel) -> dict[str, Any]:
    """Serialise a model into a request-body-ready dict."""

    return model.model_dump(mode="json", exclude_none=True)
