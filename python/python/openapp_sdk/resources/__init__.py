"""Per-OpenAPI-tag sub-clients.

Each sub-client takes an :class:`~openapp_sdk.client.AsyncClient` and surfaces the
endpoints grouped under the corresponding OpenAPI tag. The sync :class:`Client`
wraps these with a small shim that blocks on the async calls.
"""

from .agents import AgentsClient
from .api_keys import ApiKeysClient
from .auth import AuthClient
from .billing import BillingClient
from .devices import DevicesClient
from .directory_listing_members import DirectoryListingMembersClient
from .entities import EntitiesClient
from .eula import EulaClient
from .integrations import IntegrationsClient
from .lan_agent import LanAgentClient
from .me import MeClient
from .orgs import OrgsClient
from .public_access import PublicAccessClient
from .scripting import ScriptingClient
from .site_people import SitePeopleClient
from .status import StatusClient
from .users import UsersClient
from .zones import ZonesClient

__all__ = [
    "AgentsClient",
    "ApiKeysClient",
    "AuthClient",
    "BillingClient",
    "DevicesClient",
    "DirectoryListingMembersClient",
    "EntitiesClient",
    "EulaClient",
    "IntegrationsClient",
    "LanAgentClient",
    "MeClient",
    "OrgsClient",
    "PublicAccessClient",
    "ScriptingClient",
    "SitePeopleClient",
    "StatusClient",
    "UsersClient",
    "ZonesClient",
]
