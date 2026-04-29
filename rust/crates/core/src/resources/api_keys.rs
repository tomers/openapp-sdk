//! `API Keys` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct ApiKeysClient {
    transport: Arc<Transport>,
}

impl ApiKeysClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// `GET /api-keys` — list API keys for the authenticated user's org.
    pub async fn list(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/api-keys",
                ..Default::default()
            })
            .await
    }

    /// `POST /api-keys` — mint a new API key.
    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/api-keys",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `PATCH /api-keys/{id}` — rename or adjust scopes.
    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/api-keys/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PATCH,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `DELETE /api-keys/{id}` — soft-revoke the key.
    pub async fn revoke(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/api-keys/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `POST /api-keys/{id}/restore` — un-revoke a previously revoked key.
    pub async fn restore(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/api-keys/{id}/restore");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `DELETE /api-keys/{id}/purge` — hard-delete a revoked key.
    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/api-keys/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
