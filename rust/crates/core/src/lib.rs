//! # openapp-sdk-core
//!
//! Language-agnostic core of the `OpenApp` SDK. All language SDKs (Python today; .NET,
//! Ruby, Go, TypeScript in the future) sit on top of this crate — either directly via
//! a native binding (`PyO3` for Python) or through [`openapp-sdk-core-c-bridge`].
//!
//! What lives here:
//!
//! * [`Client`] / [`ClientBuilder`] — the main entry point. `Client` owns an async
//!   [`reqwest::Client`] configured with retries, telemetry, and a pluggable
//!   [`auth::TokenProvider`].
//! * Per-tag sub-clients (e.g. [`resources::OrgsClient`], [`resources::DevicesClient`])
//!   mirror the `OpenAPI` groups defined in `packages/api-spec/openapi.json`.
//! * [`error::SdkError`] — exhaustive error enum. All fallible SDK calls return
//!   `Result<_, SdkError>`.
//! * [`interceptor::Interceptor`] — hook for request/response observability (logging,
//!   tracing, custom headers, tenant pinning).
//!
//! What **does not** live here:
//!
//! * Workflow / determinism layers. `OpenApp` is REST; there is no workflow sandbox.
//! * Language-specific ergonomics. Those live in `packages/sdk-python/openapp_sdk/`.

#![deny(rust_2018_idioms, missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

pub mod auth;
pub mod client;
pub mod error;
pub mod interceptor;
pub mod resources;
pub mod retry;
pub mod telemetry;
pub mod transport;

pub use client::{Client, ClientBuilder, ClientConfig};
pub use error::{ApiErrorResponse, SdkError};
pub use interceptor::Interceptor;
pub use openapp_sdk_common::{ApiKey, SDK_NAME, SDK_VERSION, TokenFormatError};
