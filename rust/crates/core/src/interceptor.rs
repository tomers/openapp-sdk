//! Request / response interceptors.
//!
//! A light wrapper that lets callers observe or mutate outgoing requests
//! (logging, tracing span injection, custom headers, tenant pinning) without
//! re-implementing the transport.
//!
//! The default client installs only the built-in telemetry interceptor. Additional
//! interceptors are appended via [`crate::ClientBuilder::interceptor`] and run in the
//! order they were added.

use std::sync::Arc;

use async_trait::async_trait;
use reqwest::{Request, Response};

use crate::error::SdkError;

/// Hook invoked around every outgoing HTTP request.
#[async_trait]
pub trait Interceptor: Send + Sync + std::fmt::Debug {
    /// Called just before the request is dispatched. Implementations may mutate the
    /// request (add headers, extend the URL) by returning a modified [`Request`].
    async fn on_request(&self, request: Request) -> Result<Request, SdkError> {
        Ok(request)
    }

    /// Called after the transport layer produced a response. Implementations are free
    /// to observe the response; transforming it is not supported here on purpose —
    /// retry/decode semantics belong to the core, not to arbitrary interceptors.
    async fn on_response(&self, _response: &Response) -> Result<(), SdkError> {
        Ok(())
    }
}

/// Shared-ownership handle used throughout the SDK.
pub type SharedInterceptor = Arc<dyn Interceptor>;

/// Built-in interceptor that adds a structured [`tracing`] span around every request.
#[derive(Debug, Default, Clone, Copy)]
pub struct TracingInterceptor;

#[async_trait]
impl Interceptor for TracingInterceptor {
    async fn on_request(&self, request: Request) -> Result<Request, SdkError> {
        tracing::debug!(
            method = %request.method(),
            url = %request.url(),
            "openapp-sdk: dispatching request"
        );
        Ok(request)
    }

    async fn on_response(&self, response: &Response) -> Result<(), SdkError> {
        tracing::debug!(
            status = response.status().as_u16(),
            url = %response.url(),
            "openapp-sdk: received response"
        );
        Ok(())
    }
}
