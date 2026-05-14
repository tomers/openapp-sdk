"""Behave hooks: in-process bridge mock for scenarios tagged ``@mock_api``."""

from __future__ import annotations

import asyncio
import contextlib
from types import SimpleNamespace
from typing import Any

from behave.model import Scenario
from openapp_sdk.bridge.base import BridgeClient, BridgeRequest
from openapp_sdk.errors import ApiError, AuthError, HttpError


class _RecordedRequest:
    def __init__(self, headers: dict[str, str], content: bytes) -> None:
        self.headers = headers
        self._content = content

    def read(self) -> bytes:
        return self._content


class _FakeRoute:
    def __init__(self, router: _FakeBridge, method: str, path: str) -> None:
        self._router = router
        self._method = method
        self._path = path
        self._responses: list[Any] = []
        self.calls: list[Any] = []

    @property
    def called(self) -> bool:
        return bool(self.calls)

    def mock(
        self, *, return_value: Any = None, side_effect: list[Any] | None = None
    ) -> _FakeRoute:
        self._responses = (
            list(side_effect) if side_effect is not None else [return_value]
        )
        self._router.routes[(self._method, self._path)] = self
        return self

    def next_response(self) -> Any:
        if len(self._responses) > 1:
            return self._responses.pop(0)
        return self._responses[0]


class _FakeBridgeClient(BridgeClient):
    def __init__(
        self,
        bridge: _FakeBridge,
        *,
        api_key: str,
        base_url: str,
        max_retries: int,
    ) -> None:
        self._bridge = bridge
        self._api_key = api_key
        self._max_retries = max_retries
        self.base_url = base_url

    async def request(self, req: BridgeRequest) -> Any:
        route = self._bridge.routes[(req.method.upper(), req.path)]
        headers = {"authorization": f"Bearer {self._api_key}"}
        content = b""
        if req.multipart is None and req.body_json is not None:
            content = req.body_json.encode()
        if req.multipart is not None:
            headers["content-type"] = "multipart/form-data; boundary=openapp-behave"
        route.calls.append(
            SimpleNamespace(request=_RecordedRequest(headers=headers, content=content))
        )
        response = route.next_response()
        attempts = 0
        while (
            _is_retryable(response.status_code)
            and route._responses
            and attempts < self._max_retries
        ):
            attempts += 1
            response = route.next_response()
        if response.status_code >= 400:
            _raise_response_error(response)
        if not response.content:
            return None
        return response.json()

    async def close(self) -> None:
        return None


class _FakeBridge:
    name = "fake"

    def __init__(self) -> None:
        self.routes: dict[tuple[str, str], _FakeRoute] = {}

    def get(self, path: str) -> _FakeRoute:
        return _FakeRoute(self, "GET", path)

    def post(self, path: str) -> _FakeRoute:
        return _FakeRoute(self, "POST", path)

    def new_client(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
    ) -> BridgeClient:
        return _FakeBridgeClient(
            self, api_key=api_key, base_url=base_url, max_retries=max_retries
        )


def _is_retryable(status: int) -> bool:
    return status in (408, 425, 429) or 500 <= status < 600


def _raise_response_error(response: Any) -> None:
    try:
        payload = response.json()
    except ValueError:
        raise HttpError(response.status_code, response.text) from None
    message = str(payload.get("message", ""))
    if response.status_code in (401, 403):
        raise AuthError(message)
    raise ApiError(
        status=response.status_code,
        message=message,
        code=payload.get("code"),
        details=payload.get("details"),
    )


def before_scenario(context: Any, scenario: Scenario) -> None:
    if "mock_api" not in scenario.effective_tags:
        return
    import openapp_sdk.client as client_module

    context._original_get_bridge = client_module.get_bridge
    context.bridge_router = _FakeBridge()
    client_module.get_bridge = lambda: context.bridge_router


def after_scenario(context: Any, scenario: Scenario) -> None:
    if "mock_api" not in scenario.effective_tags:
        return
    client = getattr(context, "client", None)
    if client is not None:
        with contextlib.suppress(Exception):
            asyncio.run(client.close())
    original = getattr(context, "_original_get_bridge", None)
    if original is not None:
        import openapp_sdk.client as client_module

        client_module.get_bridge = original
