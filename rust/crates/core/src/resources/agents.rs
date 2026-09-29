//! `Agents` resource group — named non-human principals.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct AgentsClient {
    transport: Arc<Transport>,
}

impl AgentsClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// `GET /agents`
    pub async fn list(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/agents",
                ..Default::default()
            })
            .await
    }

    /// `POST /agents`
    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/agents",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `GET /agents/{id}`
    pub async fn get(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/agents/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `PATCH /agents/{id}`
    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/agents/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PATCH,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `DELETE /agents/{id}` — revoke the agent.
    pub async fn revoke(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/agents/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `POST /agents/{id}/credentials` — mint a scoped API key bound to the agent.
    pub async fn create_credential(
        &self,
        id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/agents/{id}/credentials");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }
}
