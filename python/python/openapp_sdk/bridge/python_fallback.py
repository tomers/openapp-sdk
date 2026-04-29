"""Pure-Python bridge implementation, used when the Rust wheel is unavailable.

Behaves the same as the Rust bridge from a Python caller's perspective:
``Authorization: Bearer`` header, retry on 408/425/429/5xx, error mapping to
:mod:`openapp_sdk.errors`. The retry loop here is deliberately small — heavy
retry logic belongs to the Rust core; the Python fallback is a safety net.
"""

from __future__ import annotations

import asyncio
import json
from collections.abc import Iterable
from typing import Any

import httpx

from .._version import __version__
from ..errors import (
    ApiError,
    AuthError,
    HttpError,
    TransportError,
    ValidationError,
)
from .base import Bridge, BridgeClient, BridgeRequest

_RETRY_STATUS: frozenset[int] = frozenset({408, 425, 429, 500, 502, 503, 504})


class _PythonClient(BridgeClient):
    def __init__(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
    ) -> None:
        self.base_url = base_url.rstrip("/") + "/"
        self._client = httpx.AsyncClient(
            base_url=self.base_url,
            timeout=timeout_secs,
            headers={
                "Authorization": f"Bearer {api_key}",
                "User-Agent": user_agent or f"openapp-sdk/{__version__}",
            },
        )
        self._max_retries = max_retries

    async def request(self, req: BridgeRequest) -> Any:
        path = req.path.lstrip("/")
        params = _pairs_to_params(req.query)
        content = None
        files: tuple[tuple[str, tuple[str, bytes, str]], ...] | None = None
        headers: dict[str, str] = dict(req.headers or {})
        if req.multipart is not None:
            field, filename, ctype, data = req.multipart
            files = ((field, (filename, data, ctype)),)
        elif req.body_json is not None and req.body_json != "":
            content = req.body_json
            headers.setdefault("content-type", "application/json")

        last_exc: Exception | None = None
        for attempt in range(self._max_retries + 1):
            try:
                response = await self._client.request(
                    req.method.upper(),
                    path,
                    params=params,
                    content=content if files is None else None,
                    files=files,
                    headers=headers or None,
                    timeout=req.timeout_secs or None,
                )
            except httpx.TimeoutException as exc:
                last_exc = TransportError(f"timeout: {exc}")
            except httpx.HTTPError as exc:
                last_exc = TransportError(str(exc))
            else:
                if response.status_code in _RETRY_STATUS and attempt < self._max_retries:
                    await _sleep_for_retry(response, attempt)
                    continue
                return _decode(response)

            if attempt < self._max_retries:
                await asyncio.sleep(_exp_backoff(attempt))
                continue
            break

        assert last_exc is not None
        raise last_exc

    async def close(self) -> None:
        await self._client.aclose()


class PythonBridge(Bridge):
    name = "python"

    def new_client(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
    ) -> BridgeClient:
        return _PythonClient(
            api_key=api_key,
            base_url=base_url,
            user_agent=user_agent,
            timeout_secs=timeout_secs,
            max_retries=max_retries,
        )


def _pairs_to_params(
    query: Iterable[tuple[str, str | None]],
) -> list[tuple[str, str | int | float | bool | None]]:
    # `httpx.AsyncClient.request(params=...)` accepts a list of
    # `tuple[str, str | int | float | bool | None]`, so widen the element type
    # at the boundary (list is invariant in its parameter).
    return [(k, v) for k, v in query if v is not None]


def _decode(response: httpx.Response) -> Any:
    status = response.status_code
    body_bytes = response.content
    if status < 400:
        if not body_bytes:
            return None
        try:
            return json.loads(body_bytes)
        except json.JSONDecodeError as exc:
            raise ValidationError(f"failed to decode response: {exc}") from None

    if status in (401, 403):
        msg = _extract_message(body_bytes) or "authentication failed"
        raise AuthError(msg)

    parsed: dict[str, Any] | None = None
    try:
        parsed = json.loads(body_bytes)
    except (json.JSONDecodeError, ValueError):
        parsed = None
    if isinstance(parsed, dict) and "message" in parsed:
        raise ApiError(
            status=status,
            message=str(parsed.get("message", "")),
            code=parsed.get("code"),
            correlation_id=parsed.get("correlationId"),
            details=parsed.get("details"),
        )
    raise HttpError(status=status, message=body_bytes.decode("utf-8", errors="replace"))


def _extract_message(body_bytes: bytes) -> str | None:
    try:
        data = json.loads(body_bytes)
    except (json.JSONDecodeError, ValueError):
        return None
    if isinstance(data, dict):
        msg = data.get("message")
        if isinstance(msg, str):
            return msg
    return None


def _exp_backoff(attempt: int) -> float:
    return float(min(0.25 * (2**attempt), 8.0))


async def _sleep_for_retry(response: httpx.Response, attempt: int) -> None:
    retry_after = response.headers.get("retry-after")
    if retry_after:
        try:
            seconds = float(retry_after)
            await asyncio.sleep(min(seconds, 30.0))
            return
        except ValueError:
            pass
    await asyncio.sleep(_exp_backoff(attempt))
