"""Bridge abstraction: the contract the client uses to talk to the transport."""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from dataclasses import dataclass
from typing import Any, Protocol

__all__ = ["Bridge", "BridgeClient", "BridgeRequest"]


@dataclass(frozen=True)
class BridgeRequest:
    method: str
    path: str
    body_json: str | None = None
    query: Sequence[tuple[str, str | None]] = ()
    headers: Mapping[str, str] | None = None
    timeout_secs: float | None = None
    #: When set, ``POST`` multipart (``field_name``, ``filename``, ``content_type``, ``bytes``).
    multipart: tuple[str, str, str, bytes] | None = None


class BridgeClient(Protocol):
    """A low-level transport handle. One per :class:`openapp_sdk.Client`."""

    base_url: str

    async def request(self, req: BridgeRequest) -> Any:  # pragma: no cover
        """Execute the request and return the decoded JSON body (or ``None``).

        Errors surface as typed exceptions from :mod:`openapp_sdk.errors`.
        """

    async def close(self) -> None:  # pragma: no cover
        ...


class Bridge(Protocol):
    """Factory for bridge clients. Parameterless — implementations may hold a
    shared tokio runtime or httpx client pool, but do not leak it here.
    """

    name: str

    def new_client(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
    ) -> BridgeClient:  # pragma: no cover
        ...
