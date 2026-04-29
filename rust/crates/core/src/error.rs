//! Error types surfaced by every SDK call.

pub use openapp_sdk_common::ApiErrorResponse;
use openapp_sdk_common::TokenFormatError;
use thiserror::Error;

/// Exhaustive error returned by the high-level client.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SdkError {
    /// The API returned a non-2xx response with a JSON [`ApiErrorResponse`] body.
    #[error("api error (status {status}): {}", .body.message)]
    Api {
        /// HTTP status code (e.g. 400, 404, 500).
        status: u16,
        /// Parsed error envelope.
        body: ApiErrorResponse,
    },

    /// The API returned a non-2xx response whose body was not parseable JSON.
    #[error("http {status}: {message}")]
    Http { status: u16, message: String },

    /// Authentication failed: missing, malformed, or rejected token.
    #[error("auth error: {0}")]
    Auth(String),

    /// Token parsing failed before any request was sent.
    #[error("invalid api key: {0}")]
    Token(#[from] TokenFormatError),

    /// Transport-level failure (DNS, TLS, connection reset, timeout…).
    #[error("transport error: {0}")]
    Transport(String),

    /// Server reply could not be decoded (unexpected shape, invalid JSON).
    #[error("failed to decode response: {0}")]
    Deserialize(String),

    /// Invalid SDK configuration (base URL missing, conflicting options, …).
    #[error("invalid configuration: {0}")]
    Config(String),

    /// The caller-provided data could not be serialized into a request body.
    #[error("failed to serialize request: {0}")]
    Serialize(String),

    /// Catch-all for anything else; always prefer a more specific variant.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl SdkError {
    /// `true` iff the error is safe to retry as-is.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Transport(_) => true,
            Self::Http { status, .. } | Self::Api { status, .. } => {
                matches!(*status, 408 | 425 | 429 | 500 | 502 | 503 | 504)
            }
            _ => false,
        }
    }

    /// HTTP status when the error carries one, else `None`.
    #[must_use]
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } | Self::Http { status, .. } => Some(*status),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for SdkError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_timeout() {
            Self::Transport(format!("timeout: {value}"))
        } else if value.is_connect() {
            Self::Transport(format!("connect error: {value}"))
        } else if value.is_decode() {
            Self::Deserialize(value.to_string())
        } else {
            Self::Transport(value.to_string())
        }
    }
}

impl From<reqwest_middleware::Error> for SdkError {
    fn from(value: reqwest_middleware::Error) -> Self {
        match value {
            reqwest_middleware::Error::Reqwest(err) => err.into(),
            reqwest_middleware::Error::Middleware(err) => Self::Transport(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for SdkError {
    fn from(value: serde_json::Error) -> Self {
        Self::Deserialize(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retryable_classification() {
        assert!(SdkError::Transport("x".into()).is_retryable());
        assert!(
            SdkError::Http {
                status: 503,
                message: "x".into()
            }
            .is_retryable()
        );
        assert!(
            !SdkError::Http {
                status: 400,
                message: "x".into()
            }
            .is_retryable()
        );
        assert!(!SdkError::Auth("nope".into()).is_retryable());
    }

    #[test]
    fn status_extraction() {
        assert_eq!(
            SdkError::Http {
                status: 404,
                message: String::new()
            }
            .status(),
            Some(404)
        );
        assert_eq!(SdkError::Auth("x".into()).status(), None);
    }
}
