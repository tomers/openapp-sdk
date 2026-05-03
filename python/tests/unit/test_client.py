"""End-to-end tests for the Python client using :mod:`respx`.

These run against the pure-Python bridge fallback (httpx-based), which is what
`OPENAPP_SDK_BRIDGE=python` in ``conftest.py`` selects. The Rust bridge has its
own coverage in ``packages/sdk/core``.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

import httpx
import pytest
import respx
from openapp_sdk import ApiError, AsyncClient, AuthError, Client, HttpError

BASE = "https://api.test"
TOKEN = f"{BASE}_openapp_SECRET"


@pytest.fixture
def _routes() -> respx.MockRouter:
    with respx.mock(base_url=BASE) as mock:
        yield mock


async def _connect(**kwargs: Any) -> AsyncClient:
    return await AsyncClient.connect(
        api_key=TOKEN,
        skip_status_probe=True,
        **kwargs,
    )


@pytest.mark.asyncio
async def test_status_get(_routes: respx.MockRouter) -> None:
    _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    client = await _connect()
    try:
        body = await client.status.get()
        assert body == {"backend": "ok"}
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_orgs_create_propagates_body(_routes: respx.MockRouter) -> None:
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
async def test_non_json_error_maps_to_http_error(_routes: respx.MockRouter) -> None:
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
async def test_json_error_maps_to_api_error(_routes: respx.MockRouter) -> None:
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
async def test_401_maps_to_auth_error(_routes: respx.MockRouter) -> None:
    _routes.get("/orgs").mock(return_value=httpx.Response(401, json={"message": "token revoked"}))
    client = await _connect()
    try:
        with pytest.raises(AuthError) as excinfo:
            await client.orgs.list()
        assert "token revoked" in excinfo.value.message
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_device_action_invokes_path_segments(_routes: respx.MockRouter) -> None:
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
async def test_entity_handle_action_invokes_path_segments(_routes: respx.MockRouter) -> None:
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
    _routes: respx.MockRouter, method_name: str, action_id: str
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
async def test_device_upload_image_sends_multipart(_routes: respx.MockRouter) -> None:
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
async def test_entity_upload_image_sends_multipart(_routes: respx.MockRouter) -> None:
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
async def test_user_upload_image_sends_multipart(_routes: respx.MockRouter) -> None:
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
async def test_org_upload_image_sends_multipart(_routes: respx.MockRouter) -> None:
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
async def test_integration_upload_image_sends_multipart(_routes: respx.MockRouter) -> None:
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
async def test_device_upload_image_from_url_get_then_post(_routes: respx.MockRouter) -> None:
    img_route = _routes.get("https://cdn.example/door.jpg").mock(
        return_value=httpx.Response(
            200,
            content=b"\xff\xd8\xff\xe0",
            headers={"content-type": "image/jpeg"},
        )
    )
    post_route = _routes.post("/devices/dev_1/image").mock(
        return_value=httpx.Response(200, json={"url": "https://signed.example/door"})
    )
    client = await _connect()
    try:
        result = await client.devices.upload_image_from_url(
            "dev_1", url="https://cdn.example/door.jpg"
        )
        assert result["url"] == "https://signed.example/door"
        assert img_route.called
        assert post_route.called
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_upload_image_from_url_rejects_non_http(_routes: respx.MockRouter) -> None:
    client = await _connect()
    try:
        with pytest.raises(ValueError, match="only supports http"):
            await client.users.upload_image_from_url("usr_1", url="s3://bucket/object")
    finally:
        await client.close()


@pytest.mark.asyncio
async def test_scripting_execute_sends_script_body(_routes: respx.MockRouter) -> None:
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
    _routes: respx.MockRouter, tmp_path: Path
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


def test_sync_client_mirrors_async(_routes: respx.MockRouter) -> None:
    _routes.get("/status").mock(return_value=httpx.Response(200, json={"backend": "ok"}))
    _routes.get("/orgs").mock(return_value=httpx.Response(200, json=[{"id": "org_1"}]))
    client = Client.connect(api_key=TOKEN, skip_status_probe=False)
    try:
        result = client.orgs.list()
        assert result == [{"id": "org_1"}]
    finally:
        client.close()


def test_sync_entity_handle_alias(_routes: respx.MockRouter) -> None:
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


def test_bearer_header_sent(_routes: respx.MockRouter) -> None:
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
