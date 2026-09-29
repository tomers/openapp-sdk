//! Authentication providers.
//!
//! The SDK ships a single [`TokenProvider`] trait so higher layers (bridge, Python
//! wrappers) can plug in custom auth — session cookies, JWT, Vault-minted tokens —
//! without breaking the rest of the client. The v1 default is [`StaticApiKey`], which
//! sends the `OpenApp` API key in the [`API_KEY_HEADER`] header.
//!
//! The gateway (Oathkeeper) reads API keys only from `X-API-Key`; `Authorization:
//! Bearer` is its JWT channel, so an API key sent there is never validated. See
//! `notes/contracts/api-key-authentication.md`.

use std::sync::Arc;

use async_trait::async_trait;
use openapp_sdk_common::ApiKey;
use reqwest::header::{AUTHORIZATION, HeaderName};

use crate::error::SdkError;

/// Header the gateway reads `OpenApp` API keys from.
pub const API_KEY_HEADER: HeaderName = HeaderName::from_static("x-api-key");

/// Credentials returned by a [`TokenProvider`] for a single outgoing request: the
/// header to set and its full value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthToken {
    pub header: HeaderName,
    pub value: String,
}

impl AuthToken {
    /// An `OpenApp` API key, sent verbatim in [`API_KEY_HEADER`].
    #[must_use]
    pub fn api_key(token: impl Into<String>) -> Self {
        Self {
            header: API_KEY_HEADER,
            value: token.into(),
        }
    }

    /// A bearer credential (for example a JWT), sent as `Authorization: Bearer <token>`.
    #[must_use]
    pub fn bearer(token: impl AsRef<str>) -> Self {
        Self {
            header: AUTHORIZATION,
            value: format!("Bearer {}", token.as_ref()),
        }
    }
}

/// Produces the credential header for every outgoing SDK request.
#[async_trait]
pub trait TokenProvider: Send + Sync + std::fmt::Debug {
    /// Return the credentials to attach to the next request. May be called on the hot
    /// path, so implementations should cache aggressively.
    async fn token(&self) -> Result<AuthToken, SdkError>;
}

/// Shared-ownership handle used throughout the SDK.
pub type SharedTokenProvider = Arc<dyn TokenProvider>;

/// Static API-key provider: the token never changes for the lifetime of the client.
#[derive(Debug, Clone)]
pub struct StaticApiKey {
    key: ApiKey,
}

impl StaticApiKey {
    /// Wrap a parsed [`ApiKey`] into a provider.
    #[must_use]
    pub fn new(key: ApiKey) -> Self {
        Self { key }
    }

    /// Parse and wrap a raw token string.
    pub fn from_raw(token: impl Into<String>) -> Result<Self, SdkError> {
        Ok(Self::new(ApiKey::parse(token)?))
    }

    /// Access the underlying [`ApiKey`] (e.g. to derive the base URL).
    #[must_use]
    pub fn api_key(&self) -> &ApiKey {
        &self.key
    }
}

#[async_trait]
impl TokenProvider for StaticApiKey {
    async fn token(&self) -> Result<AuthToken, SdkError> {
        Ok(AuthToken::api_key(self.key.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn static_provider_emits_x_api_key_with_full_token() {
        let provider = StaticApiKey::from_raw("https://openapp.house_openapp_SECRET").unwrap();
        let token = provider.token().await.unwrap();
        assert_eq!(token.header.as_str(), "x-api-key");
        assert_eq!(token.value, "https://openapp.house_openapp_SECRET");
    }

    #[test]
    fn bearer_token_uses_authorization_header() {
        let token = AuthToken::bearer("jwt.payload.sig");
        assert_eq!(token.header, AUTHORIZATION);
        assert_eq!(token.value, "Bearer jwt.payload.sig");
    }
}
