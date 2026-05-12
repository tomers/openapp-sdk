"""Unit tests for the Python client over a fake bridge transport."""

from __future__ import annotations

from pathlib import Path
from types import SimpleNamespace
from typing import Any

import httpx
import pytest
from openapp_sdk import ApiError, AsyncClient, AuthError, Client, HttpError
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
    def __init__(self, *, headers: dict[str, str], content: bytes) -> None:
        self.headers = headers
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
    def __init__(self, bridge: _FakeBridge, *, api_key: str, base_url: str) -> None:
        self._bridge = bridge
        self._api_key = api_key
        self.base_url = base_url

    async def request(self, req: BridgeRequest) -> Any:
        route = self._bridge.routes[(req.method.upper(), req.path)]
        headers = {"authorization": f"Bearer {self._api_key}"}
        body = b""
        if req.multipart is None and req.body_json is not None:
            body = req.body_json.encode()
        if req.multipart is not None:
            headers["content-type"] = "multipart/form-data; boundary=openapp-test"

        route.calls.append(SimpleNamespace(request=_RecordedRequest(headers=headers, content=body)))
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

    def new_client(
        self,
        *,
        api_key: str,
        base_url: str,
        user_agent: str,
        timeout_secs: float,
        max_retries: int,
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


@pytest.mark.asyncio
async def test_scripting_execute_sends_script_body(_routes: _FakeBridge) -> None:
    route = _routes.post("/scripting/execute").mock(
        return_value=httpx.Response(200, json={"result": None})
    )
    client = await _connect()
    try:
        result = await client.scripting.execute(
            script='upload_image("entity", "ent_1", "https://x");'
        )
        assert result == {"result": None}
        assert route.called
        request = route.calls[0].request
        assert (
            request.read().decode()
            == '{"script": "upload_image(\\"entity\\", \\"ent_1\\", \\"https://x\\");"}'
        )
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_execute_file_sends_file_contents(
    _routes: _FakeBridge, tmp_path: Path
) -> None:
    route = _routes.post("/scripting/execute").mock(
        return_value=httpx.Response(200, json={"result": {"state": "open"}})
    )
    script_path = tmp_path / "open-door.openapp"
    script_path.write_text(
        'entity_action("01J00000000000000000000000", "switchable.open", #{});\n',
        encoding="utf-8",
    )
    client = await _connect()
    try:
        result = await client.scripting.execute_file(script_path)
        assert result == {"result": {"state": "open"}}
        assert route.called
        request = route.calls[0].request
        assert (
            request.read().decode()
            == '{"script": "entity_action(\\"01J00000000000000000000000\\", '
            '\\"switchable.open\\", #{});\\n"}'
        )
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


def test_bearer_header_sent(_routes: _FakeBridge) -> None:
    route = _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    client = Client.connect(api_key=TOKEN, skip_status_probe=True)
    try:
        client.status.get()
    finally:
        client.close()
    request = route.calls[0].request
    assert request.headers["authorization"] == f"Bearer {TOKEN}"


def test_connect_rejects_malformed_token() -> None:
    from openapp_sdk.errors import ConfigError

    with pytest.raises(ConfigError):
        Client.connect(api_key="not a token")
