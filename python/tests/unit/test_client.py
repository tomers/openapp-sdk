"""Unit tests for the Python client over a fake bridge transport."""

from __future__ import annotations

import asyncio
from collections.abc import Sequence
from types import SimpleNamespace
from typing import Any

import httpx
import pytest
from openapp_sdk import (
    ApiError,
    AsyncClient,
    AuthError,
    Client,
    HttpError,
    SdkError,
    TransportError,
)
from openapp_sdk.bridge.base import BridgeClient, BridgeRequest

BASE = "https://api.test"
TOKEN = f"{BASE}_openapp_SECRET"


@pytest.fixture
def _routes(monkeypatch: pytest.MonkeyPatch) -> _FakeBridge:
    fake = _FakeBridge()

    import openapp_sdk.client as client_module

    monkeypatch.setattr(client_module, "get_bridge", lambda: fake)
    return fake


class _RecordedRequest:
    def __init__(
        self,
        *,
        headers: dict[str, str],
        content: bytes,
        query: Sequence[tuple[str, str | None]],
    ) -> None:
        self.headers = headers
        self.query = tuple(query)
        self._content = content

    def read(self) -> bytes:
        return self._content


class _FakeRoute:
    def __init__(self, router: _FakeBridge, method: str, path: str) -> None:
        self._router = router
        self._method = method
        self._path = path
        self._response: httpx.Response | None = None
        self.calls: list[Any] = []

    @property
    def called(self) -> bool:
        return bool(self.calls)

    def mock(self, *, return_value: httpx.Response) -> _FakeRoute:
        self._response = return_value
        self._router.routes[(self._method, self._path)] = self
        return self


class _FakeBridgeClient(BridgeClient):
    def __init__(
        self,
        bridge: _FakeBridge,
        *,
        api_key: str,
        base_url: str,
        org: str | None = None,
    ) -> None:
        self._bridge = bridge
        self._api_key = api_key
        self.base_url = base_url
        self._org = org

    def with_org(self, org: str) -> BridgeClient:
        return _FakeBridgeClient(
            self._bridge, api_key=self._api_key, base_url=self.base_url, org=org
        )

    async def request(self, req: BridgeRequest) -> Any:
        route = self._bridge.routes[(req.method.upper(), req.path)]
        headers = {"x-api-key": self._api_key}
        if self._org is not None:
            headers["x-org"] = self._org
        body = b""
        if req.multipart is None and req.body_json is not None:
            body = req.body_json.encode()
        if req.multipart is not None:
            headers["content-type"] = "multipart/form-data; boundary=openapp-test"

        route.calls.append(
            SimpleNamespace(
                request=_RecordedRequest(headers=headers, content=body, query=req.query)
            )
        )
        response = route._response
        assert response is not None
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

    def delete(self, path: str) -> _FakeRoute:
        return _FakeRoute(self, "DELETE", path)

    def new_client(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
        org: str | None = None,
    ) -> BridgeClient:
        return _FakeBridgeClient(self, api_key=api_key, base_url=base_url)


def _raise_response_error(response: httpx.Response) -> None:
    try:
        payload = response.json()
    except ValueError:
        raise HttpError(response.status_code, response.text) from None

    message = str(payload.get("message", ""))
    if response.status_code == 401:
        raise AuthError(message)
    raise ApiError(
        status=response.status_code,
        message=message,
        code=payload.get("code"),
        details=payload.get("details"),
    )


async def _connect(**kwargs: Any) -> AsyncClient:
    return await AsyncClient.connect(
        api_key=TOKEN,
        skip_status_probe=True,
        **kwargs,
    )


@pytest.mark.tier_0
@pytest.mark.asyncio
async def test_status_get(_routes: _FakeBridge) -> None:
    _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    client = await _connect()
    try:
        body = await client.status.get()
        assert body == {"backend": "ok"}
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_with_org_scopes_requests(_routes: _FakeBridge) -> None:
    route = _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    client = await _connect()
    scoped = client.with_org("org_1")
    try:
        await scoped.status.get()
    finally:
        await client.close()
    assert route.calls[0].request.headers["x-org"] == "org_1"


@pytest.mark.tier_0
@pytest.mark.asyncio
async def test_orgs_create_propagates_body(_routes: _FakeBridge) -> None:
    route = _routes.post("/orgs").mock(
        return_value=httpx.Response(201, json={"id": "org_1", "name": "Acme"})
    )
    client = await _connect()
    try:
        org = await client.orgs.create(name="Acme")
        assert org == {"id": "org_1", "name": "Acme"}
        assert route.called
        request = route.calls[0].request
        assert request.read().decode() == '{"name": "Acme"}'
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_billing_plan_uses_org_billing_path(_routes: _FakeBridge) -> None:
    route = _routes.get("/orgs/org_1/billing/plan").mock(
        return_value=httpx.Response(200, json={"org_id": "org_1", "tier_id": "starter"})
    )
    client = await _connect()
    try:
        plan = await client.billing.plan("org_1")
        assert plan["org_id"] == "org_1"
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_non_json_error_maps_to_http_error(_routes: _FakeBridge) -> None:
    _routes.get("/status").mock(return_value=httpx.Response(502, text="bad gateway"))
    client = await _connect(max_retries=0)
    try:
        with pytest.raises(HttpError) as excinfo:
            await client.status.get()
        assert excinfo.value.status == 502
        assert "bad gateway" in excinfo.value.message
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_json_error_maps_to_api_error(_routes: _FakeBridge) -> None:
    _routes.post("/orgs").mock(
        return_value=httpx.Response(
            400,
            json={"code": "validation_error", "message": "name is required"},
        )
    )
    client = await _connect()
    try:
        with pytest.raises(ApiError) as excinfo:
            await client.orgs.create(name="")
        assert excinfo.value.status == 400
        assert excinfo.value.code == "validation_error"
        assert excinfo.value.message == "name is required"
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_401_maps_to_auth_error(_routes: _FakeBridge) -> None:
    _routes.get("/orgs").mock(return_value=httpx.Response(401, json={"message": "token revoked"}))
    client = await _connect()
    try:
        with pytest.raises(AuthError) as excinfo:
            await client.orgs.list()
        assert "token revoked" in excinfo.value.message
    finally:
        await client.close()


@pytest.mark.tier_0
@pytest.mark.asyncio
async def test_device_action_invokes_path_segments(_routes: _FakeBridge) -> None:
    route = _routes.post("/entities/ent_1/actions/open").mock(
        return_value=httpx.Response(200, json={"ok": True})
    )
    client = await _connect()
    try:
        result = await client.entities.actions(entity_id="ent_1", action_id="open")
        assert result == {"ok": True}
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_entity_handle_action_invokes_path_segments(_routes: _FakeBridge) -> None:
    route = _routes.post("/entities/ent_1/actions/custom.action").mock(
        return_value=httpx.Response(200, json={"ok": True})
    )
    client = await _connect()
    try:
        result = await client.entities.by_id("ent_1").action("custom.action")
        assert result == {"ok": True}
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
@pytest.mark.parametrize(
    ("method_name", "action_id"),
    [("open", "open"), ("close", "close"), ("on", "on"), ("off", "off")],
)
async def test_entity_handle_aliases_map_to_actions(
    _routes: _FakeBridge, method_name: str, action_id: str
) -> None:
    route = _routes.post(f"/entities/ent_1/actions/{action_id}").mock(
        return_value=httpx.Response(200, json={"ok": True})
    )
    client = await _connect()
    try:
        handle = client.entities.by_id("ent_1")
        result = await getattr(handle, method_name)()
        assert result == {"ok": True}
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_device_channel_count_refresh_posts_to_device_action(
    _routes: _FakeBridge,
) -> None:
    route = _routes.post("/devices/dev_1/channel-count/refresh").mock(
        return_value=httpx.Response(200, json={"channel_count": 2})
    )
    client = await _connect()
    try:
        result = await client.devices.refresh_channel_count("dev_1")
        assert result == {"channel_count": 2}
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_device_upload_image_sends_multipart(_routes: _FakeBridge) -> None:
    route = _routes.post("/devices/dev_1/image").mock(
        return_value=httpx.Response(
            200,
            json={"url": "https://signed.example/img", "expires_in_seconds": 3600},
        )
    )
    client = await _connect()
    try:
        result = await client.devices.upload_image(
            "dev_1",
            data=b"\xff\xd8\xff\xe0\x00\x10JFIF",
            content_type="image/jpeg",
            filename="door.jpg",
        )
        assert result["url"] == "https://signed.example/img"
        assert route.called
        req = route.calls[0].request
        assert req.headers.get("content-type", "").startswith("multipart/form-data")
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_entity_upload_image_sends_multipart(_routes: _FakeBridge) -> None:
    route = _routes.post("/entities/ent_1/image").mock(
        return_value=httpx.Response(
            200,
            json={"url": "https://signed.example/face", "expires_in_seconds": 3600},
        )
    )
    client = await _connect()
    try:
        result = await client.entities.upload_image(
            "ent_1",
            data=b"\xff\xd8\xff",
            content_type="image/jpeg",
            filename="face.jpg",
        )
        assert result["url"] == "https://signed.example/face"
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_user_upload_image_sends_multipart(_routes: _FakeBridge) -> None:
    route = _routes.post("/users/usr_1/image").mock(
        return_value=httpx.Response(
            200,
            json={"url": "https://signed.example/avatar", "expires_in_seconds": 3600},
        )
    )
    client = await _connect()
    try:
        result = await client.users.upload_image(
            "usr_1",
            data=b"\xff\xd8\xff",
            content_type="image/jpeg",
            filename="avatar.jpg",
        )
        assert result["url"] == "https://signed.example/avatar"
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_org_upload_image_sends_multipart(_routes: _FakeBridge) -> None:
    route = _routes.post("/orgs/org_1/image").mock(
        return_value=httpx.Response(
            200,
            json={"url": "https://signed.example/org", "expires_in_seconds": 3600},
        )
    )
    client = await _connect()
    try:
        result = await client.orgs.upload_image(
            "org_1",
            data=b"\xff\xd8\xff",
            content_type="image/jpeg",
            filename="logo.jpg",
        )
        assert result["url"] == "https://signed.example/org"
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_integration_channel_count_refresh_posts_to_discovery_action(
    _routes: _FakeBridge,
) -> None:
    route = _routes.post("/integrations/int_1/discovered-devices/refresh-channel-counts").mock(
        return_value=httpx.Response(
            200,
            json={"updated_devices": 1, "devices": [{"external_id": "shelly-1"}]},
        )
    )
    client = await _connect()
    try:
        response = await client.integrations.refresh_device_channel_counts("int_1")
        assert response["updated_devices"] == 1
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_integration_upload_image_sends_multipart(_routes: _FakeBridge) -> None:
    route = _routes.post("/integrations/int_1/image").mock(
        return_value=httpx.Response(
            200,
            json={"url": "https://signed.example/int", "expires_in_seconds": 3600},
        )
    )
    client = await _connect()
    try:
        result = await client.integrations.upload_image(
            "int_1",
            data=b"\xff\xd8\xff",
            content_type="image/jpeg",
            filename="icon.jpg",
        )
        assert result["url"] == "https://signed.example/int"
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_device_upload_image_from_url_get_then_post(
    _routes: _FakeBridge, monkeypatch: pytest.MonkeyPatch
) -> None:
    async def fake_fetch(url: str) -> tuple[bytes, str, str]:
        assert url == "https://cdn.example/door.jpg"
        return b"\xff\xd8\xff\xe0", "image/jpeg", "door.jpg"

    import openapp_sdk.resources.devices as devices_module

    monkeypatch.setattr(devices_module, "fetch_image_for_upload", fake_fetch)
    post_route = _routes.post("/devices/dev_1/image").mock(
        return_value=httpx.Response(200, json={"url": "https://signed.example/door"})
    )
    client = await _connect()
    try:
        result = await client.devices.upload_image_from_url(
            "dev_1", url="https://cdn.example/door.jpg"
        )
        assert result["url"] == "https://signed.example/door"
        assert post_route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_upload_image_from_url_rejects_non_http(_routes: _FakeBridge) -> None:
    client = await _connect()
    try:
        with pytest.raises(ValueError, match="only supports http"):
            await client.users.upload_image_from_url("usr_1", url="s3://bucket/object")
    finally:
        await client.close()


async def _no_sleep(_delay: float) -> None:
    return None


@pytest.mark.asyncio
async def test_scripting_create_execution_posts_script(_routes: _FakeBridge) -> None:
    route = _routes.post("/scripting/executions").mock(
        return_value=httpx.Response(202, json={"id": "exec_1", "status": "pending"})
    )
    client = await _connect()
    try:
        job = await client.scripting.create_execution(script="1 + 1")
        assert job == {"id": "exec_1", "status": "pending"}
        assert route.called
        assert route.calls[0].request.read().decode() == '{"script": "1 + 1"}'
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_get_execution_reads_by_id(_routes: _FakeBridge) -> None:
    route = _routes.get("/scripting/executions/exec_1").mock(
        return_value=httpx.Response(200, json={"id": "exec_1", "status": "succeeded", "result": 2})
    )
    client = await _connect()
    try:
        job = await client.scripting.get_execution("exec_1")
        assert job["result"] == 2
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_list_executions_passes_limit(_routes: _FakeBridge) -> None:
    route = _routes.get("/scripting/executions").mock(
        return_value=httpx.Response(200, json=[{"id": "exec_1", "status": "succeeded"}])
    )
    client = await _connect()
    try:
        jobs = await client.scripting.list_executions(limit=5)
        assert jobs == [{"id": "exec_1", "status": "succeeded"}]
        assert route.calls[0].request.query == (("limit", "5"),)
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_list_executions_omits_absent_limit(_routes: _FakeBridge) -> None:
    route = _routes.get("/scripting/executions").mock(return_value=httpx.Response(200, json=[]))
    client = await _connect()
    try:
        assert await client.scripting.list_executions() == []
        assert route.calls[0].request.query == ()
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_cancel_execution_deletes(_routes: _FakeBridge) -> None:
    route = _routes.delete("/scripting/executions/exec_1").mock(return_value=httpx.Response(204))
    client = await _connect()
    try:
        await client.scripting.cancel_execution("exec_1")
        assert route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_execute_sends_script_body(
    _routes: _FakeBridge, monkeypatch: pytest.MonkeyPatch
) -> None:
    monkeypatch.setattr(asyncio, "sleep", _no_sleep)
    route = _routes.post("/scripting/executions").mock(
        return_value=httpx.Response(202, json={"id": "exec_1", "status": "pending"})
    )
    _routes.get("/scripting/executions/exec_1").mock(
        return_value=httpx.Response(
            200, json={"id": "exec_1", "status": "succeeded", "result": None}
        )
    )
    client = await _connect()
    try:
        result = await client.scripting.execute(
            script='upload_image("entity", "ent_1", "https://x");'
        )
        assert result is None
        assert route.called
        request = route.calls[0].request
        assert (
            request.read().decode()
            == '{"script": "upload_image(\\"entity\\", \\"ent_1\\", \\"https://x\\");"}'
        )
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_execute_raises_on_failed_job(
    _routes: _FakeBridge, monkeypatch: pytest.MonkeyPatch
) -> None:
    monkeypatch.setattr(asyncio, "sleep", _no_sleep)
    _routes.post("/scripting/executions").mock(
        return_value=httpx.Response(202, json={"id": "exec_1", "status": "pending"})
    )
    _routes.get("/scripting/executions/exec_1").mock(
        return_value=httpx.Response(200, json={"id": "exec_1", "status": "failed", "error": "boom"})
    )
    client = await _connect()
    try:
        with pytest.raises(SdkError, match="script execution failed: boom"):
            await client.scripting.execute(script="nope")
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_execute_gives_up_when_not_terminal(
    _routes: _FakeBridge, monkeypatch: pytest.MonkeyPatch
) -> None:
    monkeypatch.setattr(asyncio, "sleep", _no_sleep)
    import openapp_sdk.resources.scripting as scripting_module

    monkeypatch.setattr(scripting_module, "_EXECUTION_TIMEOUT", 0.0)
    _routes.post("/scripting/executions").mock(
        return_value=httpx.Response(202, json={"id": "exec_1", "status": "pending"})
    )
    _routes.get("/scripting/executions/exec_1").mock(
        return_value=httpx.Response(200, json={"id": "exec_1", "status": "running"})
    )
    client = await _connect()
    try:
        with pytest.raises(TransportError, match="exec_1 did not finish"):
            await client.scripting.execute(script="sleep(60)")
    finally:
        await client.close()


def test_sync_client_mirrors_async(_routes: _FakeBridge) -> None:
    _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    _routes.get("/orgs").mock(return_value=httpx.Response(200, json=[{"id": "org_1"}]))
    client = Client.connect(api_key=TOKEN, skip_status_probe=False)
    try:
        result = client.orgs.list()
        assert result == [{"id": "org_1"}]
    finally:
        client.close()


def test_sync_entity_handle_alias(_routes: _FakeBridge) -> None:
    _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    route = _routes.post("/entities/ent_1/actions/open").mock(
        return_value=httpx.Response(200, json={"ok": True})
    )
    client = Client.connect(api_key=TOKEN, skip_status_probe=False)
    try:
        result = client.entities.by_id("ent_1").open()
        assert result == {"ok": True}
        assert route.called
    finally:
        client.close()


@pytest.mark.asyncio
async def test_me_profile_gets_profile_route(_routes: _FakeBridge) -> None:
    route = _routes.get("/me/profile").mock(
        return_value=httpx.Response(200, json={"email": "a@example.com"})
    )
    client = await _connect()
    try:
        assert await client.me.profile() == {"email": "a@example.com"}
        assert route.called
    finally:
        await client.close()


def test_full_api_key_is_passed_to_bridge(_routes: _FakeBridge) -> None:
    route = _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    client = Client.connect(api_key=TOKEN, skip_status_probe=True)
    try:
        client.status.get()
    finally:
        client.close()
    request = route.calls[0].request
    assert request.headers["x-api-key"] == TOKEN
    assert "authorization" not in request.headers


def test_base_url_is_derived_as_origin_plus_api_prefix(_routes: _FakeBridge) -> None:
    client = Client.connect(api_key=TOKEN, skip_status_probe=True)
    try:
        assert client.config.base_url == f"{BASE}/api/v1"
    finally:
        client.close()


def test_explicit_base_url_overrides_derived_root(_routes: _FakeBridge) -> None:
    client = Client.connect(
        api_key=TOKEN, base_url="http://localhost:4455/api/v1/", skip_status_probe=True
    )
    try:
        assert client.config.base_url == "http://localhost:4455/api/v1"
    finally:
        client.close()


def test_connect_rejects_malformed_token() -> None:
    from openapp_sdk.errors import ConfigError

    with pytest.raises(ConfigError):
        Client.connect(api_key="not a token")
