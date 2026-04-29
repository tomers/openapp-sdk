//! `EULA` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct EulaClient {
    transport: Arc<Transport>,
}

impl EulaClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn get(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/eula",
                ..Default::default()
            })
            .await
    }

    pub async fn accept(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/eula/accept",
                body: Some(body),
                ..Default::default()
            })
            .await
    }
}
