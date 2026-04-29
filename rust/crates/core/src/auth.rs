//! Authentication providers.
//!
//! The SDK ships a single [`TokenProvider`] trait so higher layers (bridge, Python
//! wrappers) can plug in custom auth — session cookies, JWT, Vault-minted tokens —
//! without breaking the rest of the client. The v1 default is [`StaticApiKey`], which
//! forwards the parsed `OpenApp` API key as an `Authorization: Bearer` header.

use std::sync::Arc;

use async_trait::async_trait;
use openapp_sdk_common::ApiKey;

use crate::error::SdkError;

/// Credentials returned by a [`TokenProvider`] for a single outgoing request.
#[derive(Debug, Clone)]
pub struct AuthToken {
    /// Full value of the `Authorization` header (typically `Bearer <token>`).
    pub authorization: String,
}

/// Produces the `Authorization` header for every outgoing SDK request.
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
        Ok(AuthToken {
            authorization: format!("Bearer {}", self.key.as_bearer()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn static_provider_emits_bearer() {
        let provider =
            StaticApiKey::from_raw("https://api.openapp.house/api/v1_openapp_SECRET").unwrap();
        let token = provider.token().await.unwrap();
        assert_eq!(
            token.authorization,
            "Bearer https://api.openapp.house/api/v1_openapp_SECRET"
        );
    }
}
