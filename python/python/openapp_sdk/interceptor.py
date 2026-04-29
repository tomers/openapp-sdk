"""Interceptor protocol for observing and mutating outgoing SDK requests.

Interceptors are additive: each interceptor is handed the current
:class:`RequestSpec` and returns either the same spec or a modified copy.
They run in insertion order, and are **not** allowed to
short-circuit a request — retry and decoding semantics live in the client core.
"""

from __future__ import annotations

from collections.abc import Mapping, Sequence
from dataclasses import dataclass, field, replace
from typing import Any, Protocol, runtime_checkable

__all__ = ["Interceptor", "RequestSpec", "ResponseView"]


@dataclass(frozen=True)
class RequestSpec:
    """Describes an outgoing SDK request before it hits the bridge."""

    method: str
    path: str
    query: tuple[tuple[str, str | None], ...] = ()
    body: Any | None = None
    headers: Mapping[str, str] = field(default_factory=dict)
    timeout_secs: float | None = None
    multipart: tuple[str, str, str, bytes] | None = None

    def with_header(self, name: str, value: str) -> RequestSpec:
        new_headers = dict(self.headers)
        new_headers[name] = value
        return replace(self, headers=new_headers)


@dataclass(frozen=True)
class ResponseView:
    """Read-only snapshot of the response surfaced to interceptors."""

    status: int
    url: str
    body: Any | None


@runtime_checkable
class Interceptor(Protocol):
    """User-supplied hook around every SDK request.

    Implementations typically override only the methods they care about.
    """

    async def on_request(self, request: RequestSpec) -> RequestSpec:  # pragma: no cover
        return request

    async def on_response(
        self, request: RequestSpec, response: ResponseView
    ) -> None:  # pragma: no cover
        return None


def _compose_request(interceptors: Sequence[Interceptor], request: RequestSpec) -> RequestSpec:
    """Utility used by the client to run interceptors sequentially. Returned as a
    plain function so tests can exercise it without spinning a client up."""

    for interceptor in interceptors:
        request = interceptor.on_request(request)  # type: ignore[assignment]
    return request
