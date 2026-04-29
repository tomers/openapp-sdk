//! `Status` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct StatusClient {
    transport: Arc<Transport>,
}

impl StatusClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// `GET /status` — backend liveness probe.
    pub async fn get(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/status",
                ..Default::default()
            })
            .await
    }
}
