//! Shared models and types for the `OpenApp` SDK.
//!
//! This crate holds:
//!
//! * a small set of hand-written envelopes that every language SDK needs (auth token parsing,
//!   the wire-level [`ApiErrorResponse`]), and
//! * a committed [`generated`] module which mirrors the strongly-typed models produced from
//!   `packages/api-spec/openapi.json` via [`progenitor`](https://github.com/oxidecomputer/progenitor).
//!
//! The `generated` module is regenerated on demand with
//!
//! ```text
//! just sdk::core::openapi-gen
//! ```
//!
//! (drift-checked by `just sdk::core::openapi-check`). Default builds do **not** invoke the
//! generator; the committed file is authoritative and compiled as-is.

#![deny(rust_2018_idioms, missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod localized;
pub mod token;

pub mod generated;

pub use error::ApiErrorResponse;
pub use localized::{
    NameMatch, ResolveError, filter_by_name, localized_values, name_match_from_i32, name_matches,
    resolve_unique,
};
pub use token::{API_KEY_SEPARATOR, API_PATH_PREFIX, ApiKey, TokenFormatError};

/// The SDK name we advertise in the `User-Agent` header.
pub const SDK_NAME: &str = "openapp-sdk";

/// The SDK version (kept in sync with `Cargo.toml`).
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");
