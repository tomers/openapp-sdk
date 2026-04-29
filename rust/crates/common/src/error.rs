//! Wire-level error envelope shared with the `OpenApp` backend.
//!
//! The backend surfaces non-2xx responses as JSON bodies shaped like
//! [`ApiErrorResponse`] (see `apps/backend/local_server/src/api_error.rs`).

use serde::{Deserialize, Serialize};

/// JSON body returned by the `OpenApp` API for non-2xx responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiErrorResponse {
    /// Short machine-friendly error code (e.g. `"unauthorized"`, `"validation_error"`).
    #[serde(default)]
    pub code: Option<String>,

    /// Human-readable message suitable for surfacing directly to the user.
    pub message: String,

    /// Optional structured details payload.
    #[serde(default)]
    pub details: Option<serde_json::Value>,

    /// Server-generated correlation id, when present.
    #[serde(default, rename = "correlationId")]
    pub correlation_id: Option<String>,
}

impl ApiErrorResponse {
    /// Build a synthetic envelope when the backend returned a non-2xx response whose body
    /// was not JSON (e.g. a bare `502` from a proxy). This keeps downstream error surfaces
    /// uniform.
    #[must_use]
    pub fn synthesize(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
            details: None,
            correlation_id: None,
        }
    }
}
