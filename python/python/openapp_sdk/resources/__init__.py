"""Per-OpenAPI-tag sub-clients.

Each sub-client takes an :class:`~openapp_sdk.client.AsyncClient` and surfaces the
endpoints grouped under the corresponding OpenAPI tag. The sync :class:`Client`
wraps these with a small shim that blocks on the async calls.
"""

from .apartment_residents import ApartmentResidentsClient
from .api_keys import ApiKeysClient
from .auth import AuthClient
from .devices import DevicesClient
from .entities import EntitiesClient
from .eula import EulaClient
from .integrations import IntegrationsClient
from .lan_agent import LanAgentClient
from .me import MeClient
from .orgs import OrgsClient
from .public_access import PublicAccessClient
from .scripting import ScriptingClient
from .status import StatusClient
from .users import UsersClient
from .zones import ZonesClient

__all__ = [
    "ApartmentResidentsClient",
    "ApiKeysClient",
    "AuthClient",
    "DevicesClient",
    "EntitiesClient",
    "EulaClient",
    "IntegrationsClient",
    "LanAgentClient",
    "MeClient",
    "OrgsClient",
    "PublicAccessClient",
    "ScriptingClient",
    "StatusClient",
    "UsersClient",
    "ZonesClient",
]
