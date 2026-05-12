//! Per-OpenAPI-tag sub-clients.
//!
//! Each submodule corresponds to a `tag` group in `packages/api-spec/openapi.json`.
//! The sub-clients are intentionally thin — they exist so Rust callers get
//! ergonomic method names (`client.orgs().list()`) while all transport policy stays
//! centralized. New and unambiguous endpoints should use the generated models in
//! `openapp-sdk-common::generated::types`; older or schema-ambiguous list
//! envelopes still expose `serde_json::Value` until the `OpenAPI` shape is split into
//! distinct generated response types.
//!
//! The Python SDK does not use these sub-clients directly; it calls
//! [`crate::transport::Transport::request_json`] through the bridge and layers its
//! own Pydantic-typed wrappers on top.

mod apartment_residents;
mod api_keys;
mod auth;
mod billing;
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
pub use billing::BillingClient;
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

pub(crate) use openapp_sdk_common::generated::types;

/// Convenience alias for schema-ambiguous or still-untyped wire values.
pub(crate) type JsonValue = serde_json::Value;
