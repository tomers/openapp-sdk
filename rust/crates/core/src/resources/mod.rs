//! Per-OpenAPI-tag sub-clients.
//!
//! Each submodule corresponds 1:1 to a `tag` group in
//! `packages/api-spec/openapi.json`. The sub-clients are intentionally thin —
//! they exist so Rust callers get ergonomic method names (`client.orgs().list()`)
//! while still exposing `serde_json::Value` for the wire payloads until the
//! progenitor-generated typed models in `openapp-sdk-common::generated` are wired
//! up end-to-end.
//!
//! The Python SDK does not use these sub-clients directly; it calls
//! [`crate::transport::Transport::request_json`] through the bridge and layers its
//! own Pydantic-typed wrappers on top.

mod apartment_residents;
mod api_keys;
mod auth;
mod devices;
mod entities;
mod eula;
mod integrations;
mod lan_agent;
mod me;
mod orgs;
mod public_access;
mod scripting;
mod status;
mod users;
mod zones;

pub use apartment_residents::ApartmentResidentsClient;
pub use api_keys::ApiKeysClient;
pub use auth::AuthClient;
pub use devices::DevicesClient;
pub use entities::EntitiesClient;
pub use eula::EulaClient;
pub use integrations::IntegrationsClient;
pub use lan_agent::LanAgentClient;
pub use me::MeClient;
pub use orgs::OrgsClient;
pub use public_access::PublicAccessClient;
pub use scripting::ScriptingClient;
pub use status::StatusClient;
pub use users::UsersClient;
pub use zones::ZonesClient;

/// Convenience alias: every typed wire value is `serde_json::Value` for now. Typed
/// wrappers can replace this once `openapp-sdk-common::generated` is populated.
pub(crate) type JsonValue = serde_json::Value;
