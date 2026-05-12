//! `LAN agent` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::{JsonValue, types};
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct LanAgentClient {
    transport: Arc<Transport>,
}

impl LanAgentClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn meta(&self) -> Result<types::LanAgentMetaResponse, SdkError> {
        self.transport
            .request_json::<(), types::LanAgentMetaResponse>(RequestSpec {
                method: Method::GET,
                path: "/lan-agent/meta",
                ..Default::default()
            })
            .await
    }

    pub async fn bootstrap_script(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/lan-agent/cli/bootstrap.sh",
                ..Default::default()
            })
            .await
    }

    pub async fn bootstrap_token(
        &self,
        body: &types::LanAgentBootstrapTokenRequest,
    ) -> Result<types::LanAgentBootstrapTokenResponse, SdkError> {
        self.transport
            .request_json::<
                types::LanAgentBootstrapTokenRequest,
                types::LanAgentBootstrapTokenResponse,
            >(RequestSpec {
                method: Method::POST,
                path: "/lan-agent/cli/bootstrap-token",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn token(&self) -> Result<types::LanAgentCliTokenResponse, SdkError> {
        self.transport
            .request_json::<(), types::LanAgentCliTokenResponse>(RequestSpec {
                method: Method::POST,
                path: "/lan-agent/cli/token",
                ..Default::default()
            })
            .await
    }

    pub async fn submit_task_spec(
        &self,
        integration_id: &str,
        body: &types::LanAgentTaskSpecRequest,
    ) -> Result<types::LanAgentTaskSpecResponse, SdkError> {
        let path = format!("/integrations/{integration_id}/lan-agent/task-spec");
        self.transport
            .request_json::<types::LanAgentTaskSpecRequest, types::LanAgentTaskSpecResponse>(
                RequestSpec {
                    method: Method::POST,
                    path: &path,
                    body: Some(body),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn list_tasks(&self, integration_id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{integration_id}/lan-agent/tasks");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
