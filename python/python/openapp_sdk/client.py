"""High-level :class:`Client` and :class:`AsyncClient` entry points.

:meth:`Client.connect` and :meth:`AsyncClient.connect` return a ready-to-use
client after a one-shot ``/status`` probe. Sub-clients are attached as
attributes (``client.orgs``, ``client.devices``, …) and share the
underlying :class:`~openapp_sdk.bridge.BridgeClient`.
"""

from __future__ import annotations

import asyncio
import inspect
import json
from collections.abc import Sequence
from dataclasses import dataclass
from typing import Any

from ._version import __version__
from .bridge import BridgeClient, get_bridge
from .bridge.base import BridgeRequest
from .errors import ConfigError
from .interceptor import Interceptor, RequestSpec, ResponseView
from .resources import (
    ApartmentResidentsClient,
    ApiKeysClient,
    AuthClient,
    DevicesClient,
    EntitiesClient,
    EulaClient,
    IntegrationsClient,
    LanAgentClient,
    MeClient,
    OrgsClient,
    PublicAccessClient,
    ScriptingClient,
    StatusClient,
    UsersClient,
    ZonesClient,
)
from .token import ApiKey

__all__ = ["AsyncClient", "Client", "ClientConfig"]


DEFAULT_TIMEOUT = 30.0
DEFAULT_RETRIES = 3


@dataclass(frozen=True)
class ClientConfig:
    base_url: str
    user_agent: str
    timeout_secs: float
    max_retries: int


class AsyncClient:
    """Async-first client. Construct via :meth:`connect`."""

    api_keys: ApiKeysClient
    users: UsersClient
    orgs: OrgsClient
    devices: DevicesClient
    entities: EntitiesClient
    integrations: IntegrationsClient
    zones: ZonesClient
    lan_agent: LanAgentClient
    scripting: ScriptingClient
    apartment_residents: ApartmentResidentsClient
    public_access: PublicAccessClient
    auth: AuthClient
    me: MeClient
    eula: EulaClient
    status: StatusClient

    def __init__(
        self,
        *,
        bridge_client: BridgeClient,
        config: ClientConfig,
        interceptors: Sequence[Interceptor] = (),
    ):
        self._bridge = bridge_client
        self._config = config
        self._interceptors = tuple(interceptors)

        # Sub-clients: all share the same request dispatcher.
        self.api_keys = ApiKeysClient(self)
        self.users = UsersClient(self)
        self.orgs = OrgsClient(self)
        self.devices = DevicesClient(self)
        self.entities = EntitiesClient(self)
        self.integrations = IntegrationsClient(self)
        self.zones = ZonesClient(self)
        self.lan_agent = LanAgentClient(self)
        self.scripting = ScriptingClient(self)
        self.apartment_residents = ApartmentResidentsClient(self)
        self.public_access = PublicAccessClient(self)
        self.auth = AuthClient(self)
        self.me = MeClient(self)
        self.eula = EulaClient(self)
        self.status = StatusClient(self)

    @classmethod
    async def connect(
        cls,
        *,
        api_key: str,
        base_url: str | None = None,
        user_agent: str | None = None,
        timeout: float = DEFAULT_TIMEOUT,
        max_retries: int = DEFAULT_RETRIES,
        interceptors: Sequence[Interceptor] = (),
        skip_status_probe: bool = False,
    ) -> AsyncClient:
        """Parse credentials, build the bridge client, and verify connectivity."""

        parsed = ApiKey.parse(api_key)
        resolved_base_url = (base_url or parsed.base_url).rstrip("/")
        if not resolved_base_url:
            raise ConfigError("could not determine a base URL from the API key")

        ua = user_agent or f"openapp-sdk/{__version__}"
        config = ClientConfig(
            base_url=resolved_base_url,
            user_agent=ua,
            timeout_secs=float(timeout),
            max_retries=int(max_retries),
        )
        bridge_client = get_bridge().new_client(
            api_key=parsed.bearer,
            base_url=resolved_base_url,
            user_agent=ua,
            timeout_secs=config.timeout_secs,
            max_retries=config.max_retries,
        )
        client = cls(
            bridge_client=bridge_client,
            config=config,
            interceptors=interceptors,
        )
        if not skip_status_probe:
            # One-shot health check — fail fast rather than defer errors to the
            # first real call. Per project policy we do not silently fall back.
            await client.status.get()
        return client

    @property
    def config(self) -> ClientConfig:
        return self._config

    async def close(self) -> None:
        """Release the underlying transport / runtime."""

        await self._bridge.close()

    async def __aenter__(self) -> AsyncClient:
        return self

    async def __aexit__(self, exc_type: Any, exc: Any, tb: Any) -> None:
        await self.close()

    # -- Internal: sub-clients dispatch through these helpers ---------------

    async def _request(
        self,
        method: str,
        path: str,
        *,
        body: Any = None,
        query: Sequence[tuple[str, str | None]] = (),
        timeout: float | None = None,
        multipart: tuple[str, str, str, bytes] | None = None,
    ) -> Any:
        spec = RequestSpec(
            method=method,
            path=path,
            query=tuple(query),
            body=body,
            headers={},
            timeout_secs=timeout,
            multipart=multipart,
        )
        for interceptor in self._interceptors:
            spec = await interceptor.on_request(spec)

        body_json = None if spec.multipart is not None else _serialize_body(spec.body)
        bridge_req = BridgeRequest(
            method=spec.method,
            path=spec.path,
            body_json=body_json,
            query=spec.query,
            headers=dict(spec.headers) if spec.headers else None,
            timeout_secs=spec.timeout_secs,
            multipart=spec.multipart,
        )
        response_body = await self._bridge.request(bridge_req)

        if self._interceptors:
            view = ResponseView(
                status=200,
                url=f"{self._config.base_url}/{spec.path.lstrip('/')}",
                body=response_body,
            )
            for interceptor in self._interceptors:
                await interceptor.on_response(spec, view)

        return response_body


class Client:
    """Synchronous facade around :class:`AsyncClient`. Every call runs the
    corresponding coroutine on a private event loop so user code does not need
    to be async-aware.

    Example::

        from openapp_sdk import Client

        client = Client.connect(api_key=...)
        for org in client.orgs.list():
            print(org)
    """

    api_keys: ApiKeysClient
    users: UsersClient
    orgs: OrgsClient
    devices: DevicesClient
    entities: EntitiesClient
    integrations: IntegrationsClient
    zones: ZonesClient
    lan_agent: LanAgentClient
    scripting: ScriptingClient
    apartment_residents: ApartmentResidentsClient
    public_access: PublicAccessClient
    auth: AuthClient
    me: MeClient
    eula: EulaClient
    status: StatusClient

    def __init__(
        self,
        inner: AsyncClient,
        loop: asyncio.AbstractEventLoop | None = None,
    ) -> None:
        self._inner = inner
        self._loop: asyncio.AbstractEventLoop = loop or asyncio.new_event_loop()
        self.api_keys = _wrap(inner.api_keys, self)
        self.users = _wrap(inner.users, self)
        self.orgs = _wrap(inner.orgs, self)
        self.devices = _wrap(inner.devices, self)
        self.entities = _wrap(inner.entities, self)
        self.integrations = _wrap(inner.integrations, self)
        self.zones = _wrap(inner.zones, self)
        self.lan_agent = _wrap(inner.lan_agent, self)
        self.scripting = _wrap(inner.scripting, self)
        self.apartment_residents = _wrap(inner.apartment_residents, self)
        self.public_access = _wrap(inner.public_access, self)
        self.auth = _wrap(inner.auth, self)
        self.me = _wrap(inner.me, self)
        self.eula = _wrap(inner.eula, self)
        self.status = _wrap(inner.status, self)

    @classmethod
    def connect(
        cls,
        *,
        api_key: str,
        base_url: str | None = None,
        user_agent: str | None = None,
        timeout: float = DEFAULT_TIMEOUT,
        max_retries: int = DEFAULT_RETRIES,
        interceptors: Sequence[Interceptor] = (),
        skip_status_probe: bool = False,
    ) -> Client:
        loop = asyncio.new_event_loop()
        try:
            inner = loop.run_until_complete(
                AsyncClient.connect(
                    api_key=api_key,
                    base_url=base_url,
                    user_agent=user_agent,
                    timeout=timeout,
                    max_retries=max_retries,
                    interceptors=interceptors,
                    skip_status_probe=skip_status_probe,
                )
            )
        except BaseException:
            loop.close()
            raise
        return cls(inner, loop=loop)

    @property
    def config(self) -> ClientConfig:
        return self._inner.config

    def close(self) -> None:
        try:
            self._loop.run_until_complete(self._inner.close())
        finally:
            self._loop.close()

    def __enter__(self) -> Client:
        return self

    def __exit__(self, exc_type: Any, exc: Any, tb: Any) -> None:
        self.close()

    # Internal: run a coroutine to completion on the private loop.
    def _run(self, coro: Any) -> Any:
        return self._loop.run_until_complete(coro)


def _serialize_body(body: Any) -> str | None:
    if body is None:
        return None
    if isinstance(body, (bytes, bytearray)):
        return body.decode("utf-8")
    if isinstance(body, str):
        # Allow callers to hand us pre-serialized JSON.
        return body
    try:
        return json.dumps(body, default=_json_default)
    except TypeError as exc:
        from .errors import SerializationError

        raise SerializationError(str(exc)) from None


def _json_default(value: Any) -> Any:
    # Accept dataclasses, pydantic models, enums, datetimes.
    from dataclasses import asdict, is_dataclass

    if is_dataclass(value) and not isinstance(value, type):
        return asdict(value)
    if hasattr(value, "model_dump"):  # pydantic v2
        return value.model_dump(mode="json", exclude_none=True)
    if hasattr(value, "isoformat"):
        return value.isoformat()
    raise TypeError(f"cannot serialize {type(value).__name__} to JSON")


def _wrap(async_sub: Any, sync_client: Client) -> Any:
    """Return a thin sync-shim around an async sub-client.

    Every coroutine method on ``async_sub`` becomes a blocking method on the
    returned object that runs on the sync client's private event loop.
    """

    class _SyncSub:
        pass

    inst = _SyncSub()
    for name in dir(async_sub):
        if name.startswith("_"):
            continue
        attr = getattr(async_sub, name)
        if inspect.iscoroutinefunction(attr):

            def make(method: Any) -> Any:
                def wrapper(*args: Any, **kwargs: Any) -> Any:
                    return sync_client._run(method(*args, **kwargs))

                wrapper.__name__ = method.__name__
                wrapper.__doc__ = method.__doc__
                return wrapper

            setattr(inst, name, make(attr))
        elif callable(attr):

            def make_sync(method: Any) -> Any:
                def wrapper(*args: Any, **kwargs: Any) -> Any:
                    result = method(*args, **kwargs)
                    if _has_coroutine_methods(result):
                        return _wrap(result, sync_client)
                    return result

                wrapper.__name__ = method.__name__
                wrapper.__doc__ = method.__doc__
                return wrapper

            setattr(inst, name, make_sync(attr))
        else:
            setattr(inst, name, attr)
    inst.__class__.__name__ = f"Sync{async_sub.__class__.__name__}"
    return inst


def _has_coroutine_methods(value: Any) -> bool:
    try:
        for name in dir(value):
            if name.startswith("_"):
                continue
            if inspect.iscoroutinefunction(getattr(value, name)):
                return True
    except Exception:
        return False
    return False
