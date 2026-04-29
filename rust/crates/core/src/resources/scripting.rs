//! `OpenApp` Scripting resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct ScriptingClient {
    transport: Arc<Transport>,
}

impl ScriptingClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// `POST /scripting/execute` — run an `OpenApp` Scripting program.
    pub async fn execute(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/scripting/execute",
                body: Some(body),
                ..Default::default()
            })
            .await
    }
}
