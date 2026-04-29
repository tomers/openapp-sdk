"""OpenApp Python SDK.

Typical usage::

    from openapp_sdk import Client

    client = Client.connect(api_key="https://api.openapp.house/api/v1_openapp_...")
    for org in client.orgs.list():
        print(org["name"])

See https://openapp.house/docs/sdk/ for the full reference.
"""

from ._version import __version__
from .client import AsyncClient, Client, ClientConfig
from .errors import (
    ApiError,
    AuthError,
    ConfigError,
    HttpError,
    SdkError,
    SerializationError,
    TransportError,
    ValidationError,
)
from .interceptor import Interceptor, RequestSpec, ResponseView
from .token import ApiKey

__all__ = [
    "ApiError",
    "ApiKey",
    "AsyncClient",
    "AuthError",
    "Client",
    "ClientConfig",
    "ConfigError",
    "HttpError",
    "Interceptor",
    "RequestSpec",
    "ResponseView",
    "SdkError",
    "SerializationError",
    "TransportError",
    "ValidationError",
    "__version__",
]
