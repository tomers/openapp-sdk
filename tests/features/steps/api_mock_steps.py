"""HTTP-shaped steps for ``@mock_api`` scenarios (Python fake bridge)."""

from __future__ import annotations

import asyncio
import json
from typing import Any

import httpx
from behave import given, then, when
from openapp_sdk import AsyncClient
from openapp_sdk.errors import ApiError, AuthError, SdkError

_API_KEY = "https://api.test_openapp_TEST_SECRET"


def _run(coro: Any) -> Any:
    return asyncio.run(coro)


@given("the async client is connected with status probe skipped")
def step_async_client_connected(context: Any) -> None:
    async def connect() -> AsyncClient:
        return await AsyncClient.connect(api_key=_API_KEY, skip_status_probe=True)

    context.client = _run(connect())


@given('path "{path}" responds to GET with status {status:d} and JSON')
def step_mock_get_json(context: Any, path: str, status: int) -> None:
    router = context.bridge_router
    text = context.text
    assert text is not None, "Multiline JSON body required after this step"
    payload = json.loads(text)
    router.get(path).mock(return_value=httpx.Response(status, json=payload))


@given('path "{path}" responds to POST with status {status:d} and JSON')
def step_mock_post_json(context: Any, path: str, status: int) -> None:
    router = context.bridge_router
    text = context.text
    assert text is not None, "Multiline JSON body required after this step"
    payload = json.loads(text)
    router.post(path).mock(return_value=httpx.Response(status, json=payload))


@given(
    'path "{path}" responds to GET with status {first:d} once then status {second:d} and JSON'
)
def step_mock_get_transient_then_ok(
    context: Any, path: str, first: int, second: int
) -> None:
    router = context.bridge_router
    text = context.text
    assert text is not None, "Multiline JSON body required after this step"
    payload = json.loads(text)
    router.get(path).mock(
        side_effect=[
            httpx.Response(first, json={"message": "temporarily unavailable"}),
            httpx.Response(second, json=payload),
        ]
    )


@given("the async client is connected with short retries and status probe skipped")
def step_async_client_connected_short_retries(context: Any) -> None:
    async def connect() -> AsyncClient:
        return await AsyncClient.connect(
            api_key=_API_KEY,
            skip_status_probe=True,
            max_retries=3,
        )

    context.client = _run(connect())


@when("the async client fetches status")
def step_fetch_status(context: Any) -> None:
    async def get() -> Any:
        return await context.client.status.get()

    context.last_body = _run(get())


@when('the async client creates an org named "{name}"')
def step_create_org(context: Any, name: str) -> None:
    async def create() -> Any:
        return await context.client.orgs.create(name=name)

    context.last_body = _run(create())


@when("the async client lists orgs")
def step_list_orgs(context: Any) -> None:
    async def lst() -> Any:
        return await context.client.orgs.list()

    context.last_body = _run(lst())
    context.last_error = None


@when("the async client lists orgs and records any error")
def step_list_orgs_catch(context: Any) -> None:
    async def lst() -> Any:
        return await context.client.orgs.list()

    context.last_error = None
    context.last_body = None
    try:
        context.last_body = _run(lst())
    except SdkError as exc:
        context.last_error = exc


@when("the async client creates an org with empty name and records any error")
def step_create_org_empty_catch(context: Any) -> None:
    async def create() -> Any:
        return await context.client.orgs.create(name="")

    context.last_error = None
    context.last_body = None
    try:
        context.last_body = _run(create())
    except SdkError as exc:
        context.last_error = exc


@when('the async client gets org "{org_id}"')
def step_get_org(context: Any, org_id: str) -> None:
    async def get() -> Any:
        return await context.client.orgs.get(org_id)

    context.last_body = _run(get())
    context.last_error = None


@when('the async client lists devices for org "{org_id}"')
def step_list_devices(context: Any, org_id: str) -> None:
    async def lst() -> Any:
        return await context.client.devices.list(org_id=org_id)

    context.last_body = _run(lst())
    context.last_error = None


@when('the async client fetches public invite "{token}"')
def step_public_get_invite(context: Any, token: str) -> None:
    async def get() -> Any:
        return await context.client.public_access.get_invite(token)

    context.last_body = _run(get())
    context.last_error = None


@when('the async client claims public invite "{token}" with empty body')
def step_public_claim_invite(context: Any, token: str) -> None:
    async def claim() -> Any:
        return await context.client.public_access.claim_invite(token)

    context.last_body = _run(claim())
    context.last_error = None


@when("the async client lists invitations")
def step_list_invitations(context: Any) -> None:
    async def inv() -> Any:
        return await context.client.me.invitations()

    context.last_body = _run(inv())


@when('the async client runs entity action "{action}" on entity "{entity_id}"')
def step_entity_action(context: Any, action: str, entity_id: str) -> None:
    async def act() -> Any:
        return await context.client.entities.actions(
            entity_id=entity_id, action_id=action
        )

    context.last_body = _run(act())


@then('the last JSON has key "{key}"')
def step_last_json_has_key(context: Any, key: str) -> None:
    body = context.last_body
    assert isinstance(body, dict), f"expected dict, got {type(body)}"
    assert key in body


@then('the last JSON field "{field}" equals string "{value}"')
def step_field_equals_str(context: Any, field: str, value: str) -> None:
    body = context.last_body
    assert isinstance(body, dict)
    assert body.get(field) == value


@then('the last JSON field "{field}" is boolean true')
def step_field_bool_true(context: Any, field: str) -> None:
    body = context.last_body
    assert isinstance(body, dict)
    assert body.get(field) is True


@then("the last JSON is a non-empty list")
def step_last_is_nonempty_list(context: Any) -> None:
    body = context.last_body
    assert isinstance(body, list)
    assert len(body) >= 1


@then("the last JSON is an empty list")
def step_last_is_empty_list(context: Any) -> None:
    body = context.last_body
    assert isinstance(body, list)
    assert len(body) == 0


@then("the last JSON list has length {n:d}")
def step_list_length(context: Any, n: int) -> None:
    body = context.last_body
    assert isinstance(body, list)
    assert len(body) == n


@then('the invitations list includes status "{status}"')
def step_invite_has_status(context: Any, status: str) -> None:
    body = context.last_body
    assert isinstance(body, list)
    found = any(isinstance(x, dict) and x.get("status") == status for x in body)
    assert found, f"no invitation with status={status!r} in {body!r}"


@then("an AuthError was raised")
def step_auth_error_raised(context: Any) -> None:
    assert getattr(context, "last_error", None) is not None
    assert isinstance(context.last_error, AuthError)


@then('the AuthError message contains "{snippet}"')
def step_auth_error_contains(context: Any, snippet: str) -> None:
    err = context.last_error
    assert isinstance(err, AuthError)
    assert snippet in err.message


@then("an ApiError was raised")
def step_api_error_raised(context: Any) -> None:
    assert getattr(context, "last_error", None) is not None
    assert isinstance(context.last_error, ApiError)


@then('the ApiError has code "{code}"')
def step_api_error_code(context: Any, code: str) -> None:
    err = context.last_error
    assert isinstance(err, ApiError)
    assert err.code == code


@then('the last JSON list at key "{key}" has length {n:d}')
def step_dict_list_len(context: Any, key: str, n: int) -> None:
    body = context.last_body
    assert isinstance(body, dict)
    inner = body.get(key)
    assert isinstance(inner, list)
    assert len(inner) == n
