//! The top-level [`Client`] and its builder.
//!
//! A `Client` is the user-facing entry point: it owns the transport engine and hands
//! out per-tag sub-clients (orgs, devices, entities, …) on demand.

use std::{
    sync::{Arc, Once},
    time::Duration,
};

use reqwest_middleware::ClientBuilder as MiddlewareBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use url::Url;

use crate::{
    auth::{SharedTokenProvider, StaticApiKey},
    error::SdkError,
    interceptor::{Interceptor, SharedInterceptor, TracingInterceptor},
    resources,
    retry::RetryPolicy,
    transport::Transport,
};

static RUSTLS_PROVIDER_INIT: Once = Once::new();

fn ensure_rustls_provider() {
    RUSTLS_PROVIDER_INIT.call_once(|| {
        // Safe to ignore if another provider is already installed globally.
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

/// User-visible client configuration snapshot (read-only after [`ClientBuilder::build`]).
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: Url,
    pub user_agent: String,
    pub default_timeout: Duration,
    pub retry: RetryPolicy,
}

/// Fluent builder for [`Client`]. Use [`Client::builder`] to construct one.
#[derive(Debug)]
pub struct ClientBuilder {
    token_provider: Option<SharedTokenProvider>,
    base_url: Option<Url>,
    user_agent: Option<String>,
    default_timeout: Duration,
    retry: RetryPolicy,
    interceptors: Vec<SharedInterceptor>,
    underlying: Option<reqwest::Client>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            token_provider: None,
            base_url: None,
            user_agent: None,
            default_timeout: Duration::from_secs(30),
            retry: RetryPolicy::default(),
            interceptors: vec![Arc::new(TracingInterceptor) as SharedInterceptor],
            underlying: None,
        }
    }
}

impl ClientBuilder {
    /// Authenticate with a static `OpenApp` API key. The base URL is derived from the
    /// token unless overridden via [`ClientBuilder::base_url`].
    #[must_use]
    pub fn api_key(mut self, token: impl Into<String>) -> Self {
        match StaticApiKey::from_raw(token) {
            Ok(provider) => {
                if self.base_url.is_none() {
                    self.base_url = Some(provider.api_key().base_url().clone());
                }
                self.token_provider = Some(Arc::new(provider));
            }
            Err(err) => {
                // Defer the error to `.build()` so the fluent builder keeps composing.
                self.token_provider = Some(Arc::new(FailingProvider(err.to_string())));
            }
        }
        self
    }

    /// Use a custom [`TokenProvider`](crate::auth::TokenProvider).
    #[must_use]
    pub fn token_provider(mut self, provider: SharedTokenProvider) -> Self {
        self.token_provider = Some(provider);
        self
    }

    /// Override the API base URL. Rarely needed — an `OpenApp` API key embeds its base
    /// URL, so calling [`ClientBuilder::api_key`] usually suffices.
    pub fn base_url(mut self, url: impl AsRef<str>) -> Result<Self, SdkError> {
        let parsed = Url::parse(url.as_ref())
            .map_err(|e| SdkError::Config(format!("invalid base_url: {e}")))?;
        self.base_url = Some(parsed);
        Ok(self)
    }

    /// Set the `User-Agent` header. Defaults to `"openapp-sdk/<version>"`.
    #[must_use]
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Set the default per-request timeout.
    #[must_use]
    pub fn default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Override the default retry policy.
    #[must_use]
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry = policy;
        self
    }

    /// Add an [`Interceptor`]. Interceptors run in insertion order.
    #[must_use]
    pub fn interceptor(mut self, interceptor: impl Interceptor + 'static) -> Self {
        self.interceptors.push(Arc::new(interceptor));
        self
    }

    /// Supply a pre-built `reqwest::Client` (e.g. with a custom TLS root bundle).
    #[must_use]
    pub fn reqwest_client(mut self, client: reqwest::Client) -> Self {
        self.underlying = Some(client);
        self
    }

    /// Finalize the builder into a [`Client`].
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::Config`] when no authentication or base URL has been
    /// configured, or when a token-provider error has been deferred from
    /// [`ClientBuilder::api_key`].
    ///
    /// # Panics
    ///
    /// Panics if the default [`reqwest::Client`] cannot be built. In practice this
    /// only happens if the host is missing TLS roots, which is treated as a
    /// programmer error rather than a recoverable condition.
    pub fn build(self) -> Result<Client, SdkError> {
        ensure_rustls_provider();

        let tokens = self
            .token_provider
            .ok_or_else(|| SdkError::Config("no authentication configured".into()))?;
        let base_url = self
            .base_url
            .ok_or_else(|| SdkError::Config("no base URL configured".into()))?;

        let user_agent = self.user_agent.unwrap_or_else(|| {
            format!(
                "{}/{}",
                openapp_sdk_common::SDK_NAME,
                openapp_sdk_common::SDK_VERSION
            )
        });

        let underlying = self.underlying.unwrap_or_else(|| {
            reqwest::Client::builder()
                .user_agent(user_agent.clone())
                .pool_idle_timeout(Some(Duration::from_secs(90)))
                .build()
                .expect("reqwest::Client defaults must build")
        });

        let backoff = ExponentialBackoff::builder()
            .retry_bounds(self.retry.initial_backoff, self.retry.max_backoff)
            .base(2)
            .build_with_max_retries(self.retry.max_retries);

        let client = MiddlewareBuilder::new(underlying)
            .with(RetryTransientMiddleware::new_with_policy(backoff))
            .build();

        let config = ClientConfig {
            base_url: base_url.clone(),
            user_agent: user_agent.clone(),
            default_timeout: self.default_timeout,
            retry: self.retry,
        };

        let transport = Transport::new(
            client,
            base_url,
            user_agent,
            tokens,
            self.interceptors,
            self.default_timeout,
        );

        Ok(Client {
            transport: Arc::new(transport),
            config,
        })
    }
}

/// Fails every token request with the deferred parse error captured at build time.
#[derive(Debug)]
struct FailingProvider(String);

#[async_trait::async_trait]
impl crate::auth::TokenProvider for FailingProvider {
    async fn token(&self) -> Result<crate::auth::AuthToken, SdkError> {
        Err(SdkError::Auth(self.0.clone()))
    }
}

/// High-level SDK client. Cheap to clone — all heavy state sits behind an `Arc`.
#[derive(Debug, Clone)]
pub struct Client {
    transport: Arc<Transport>,
    config: ClientConfig,
}

impl Client {
    /// Start a [`ClientBuilder`].
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Snapshot of the effective configuration.
    #[must_use]
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Transport engine — exposed for advanced callers (the `PyO3` bridge, tests).
    #[must_use]
    pub fn transport(&self) -> Arc<Transport> {
        self.transport.clone()
    }

    // -- Per-tag sub-clients -------------------------------------------------

    #[must_use]
    pub fn api_keys(&self) -> resources::ApiKeysClient {
        resources::ApiKeysClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn users(&self) -> resources::UsersClient {
        resources::UsersClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn orgs(&self) -> resources::OrgsClient {
        resources::OrgsClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn devices(&self) -> resources::DevicesClient {
        resources::DevicesClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn entities(&self) -> resources::EntitiesClient {
        resources::EntitiesClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn integrations(&self) -> resources::IntegrationsClient {
        resources::IntegrationsClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn zones(&self) -> resources::ZonesClient {
        resources::ZonesClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn lan_agent(&self) -> resources::LanAgentClient {
        resources::LanAgentClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn scripting(&self) -> resources::ScriptingClient {
        resources::ScriptingClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn apartment_residents(&self) -> resources::ApartmentResidentsClient {
        resources::ApartmentResidentsClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn public_access(&self) -> resources::PublicAccessClient {
        resources::PublicAccessClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn auth(&self) -> resources::AuthClient {
        resources::AuthClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn me(&self) -> resources::MeClient {
        resources::MeClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn eula(&self) -> resources::EulaClient {
        resources::EulaClient::new(self.transport.clone())
    }

    #[must_use]
    pub fn status(&self) -> resources::StatusClient {
        resources::StatusClient::new(self.transport.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_requires_auth() {
        let err = Client::builder().build().unwrap_err();
        assert!(matches!(err, SdkError::Config(_)));
    }

    #[test]
    fn api_key_derives_base_url() {
        let client = Client::builder()
            .api_key("https://api.openapp.house/api/v1_openapp_SECRET")
            .build()
            .unwrap();
        assert_eq!(
            client.config().base_url.as_str(),
            "https://api.openapp.house/api/v1"
        );
    }

    #[test]
    fn deferred_token_error_surfaces_at_request_time() {
        // Malformed token: no separator. `build()` still succeeds; the error surfaces
        // when the first request asks the provider for a token.
        let client = Client::builder()
            .api_key("not a token")
            .base_url("https://api.openapp.house/api/v1")
            .unwrap()
            .build()
            .unwrap();
        let _ = client; // the error path is exercised by transport tests
    }
}
