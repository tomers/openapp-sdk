# openapp-sdk

[![PyPI version](https://img.shields.io/pypi/v/openapp-sdk.svg)](https://pypi.org/project/openapp-sdk/)
[![Python versions](https://img.shields.io/pypi/pyversions/openapp-sdk.svg)](https://pypi.org/project/openapp-sdk/)

Official Python SDK for [OpenApp](https://openapp.house/) — the open IoT device
management platform for buildings, homes, and access control.

The SDK wraps a shared Rust core
([`openapp-sdk-core`](https://github.com/tomers/openapp-sdk/tree/main/rust)): Rust
handles HTTP, auth, retries, and telemetry; Python gives you a typed,
ergonomic surface.

Supported Python versions are **3.10 and above** (see `requires-python` in
`pyproject.toml`).

## Install

```sh
pip install openapp-sdk
```

## Quick start

**Docs:** The canonical reference and tutorials live in the
[Python SDK documentation](https://openapp.house/docs/sdk/python/); for the
onboarding quick start (same flow as the HTTP API guide), see
[Getting started — The same call from the SDK](https://openapp.house/docs/guides/getting-started/sdk/).

```python
from openapp_sdk import Client

client = Client.connect(
    api_key="YOUR_API_KEY",  # from the OpenApp dashboard
)

# List orgs
orgs = client.orgs.list()

# Create an org and user
org = client.orgs.create(name="Acme")
user = client.users.create(email="alice@example.com", org_id=org["id"])

# Invoke entity actions via a fluent entity handle
entity = client.entities.by_id("01J00000000000000000000000")
entity.open()

# Fallback for any action id (including custom/new ones)
entity.action("switchable.open", reason="visitor buzzed in")
```

Async variant:

```python
import asyncio
from openapp_sdk import AsyncClient

async def main():
    async with await AsyncClient.connect(api_key="...") as client:
        print(await client.status.get())

asyncio.run(main())
```

## Authentication

Create API keys from the OpenApp dashboard or via `POST /api-keys`. Paste the
key into `Client.connect(api_key=...)`.

Never commit API keys. The SDK never logs them, and `repr(ApiKey(...))` elides
the secret.

## Sub-clients

The full OpenAPI surface is available under `client.<tag>`:

| Attribute                     | OpenAPI tag         |
|-------------------------------|---------------------|
| `client.api_keys`             | API Keys            |
| `client.users`                | Users               |
| `client.orgs`                 | Orgs                |
| `client.devices`              | Devices             |
| `client.entities`             | Entities            |
| `client.integrations`         | Integrations        |
| `client.zones`                | Zones               |
| `client.lan_agent`            | LAN agent           |
| `client.scripting`            | Scripting           |
| `client.apartment_residents`  | Apartment Residents |
| `client.public_access`        | Public Access       |
| `client.auth`                 | Auth                |
| `client.me`                   | Me                  |
| `client.eula`                 | EULA                |
| `client.status`               | Status              |

## Error handling

```python
from openapp_sdk.errors import ApiError, AuthError, TransportError

try:
    client.orgs.create(name="")
except ApiError as err:
    print(err.status, err.code, err.message)
except AuthError:
    print("API key revoked — re-issue from the dashboard.")
except TransportError:
    print("Network hiccup — retry or check connectivity.")
```

## Pydantic typing (optional)

Install the extra and parse payloads into Pydantic v2 models:

```sh
pip install openapp-sdk[pydantic]
```

```python
from pydantic import BaseModel
from openapp_sdk.contrib.pydantic import parse_as

class Org(BaseModel):
    id: str
    name: str

raw = client.orgs.get("org_123")
org = parse_as(Org, raw)
```

## Logging / observability

The SDK emits structured `tracing` events from Rust and standard `logging`
records from Python. Enable via `OPENAPP_SDK_LOG=info` (Rust) or the usual
`logging.basicConfig(level=logging.DEBUG)` for Python.

## License

MIT
