//! `Integrations` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct IntegrationsClient {
    transport: Arc<Transport>,
}

impl IntegrationsClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/integrations",
                ..Default::default()
            })
            .await
    }

    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/integrations",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn provider_types(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/integrations/provider-types",
                ..Default::default()
            })
            .await
    }

    pub async fn provider_definition(&self, provider_type: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/provider-types/{provider_type}/definition");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn get(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/integrations/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn restore(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/restore");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn device_metadata_schema(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/device-metadata-schema");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn discovered_devices(&self, id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{id}/discovered-devices");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn entities(&self, id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{id}/entities");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn ops(&self, id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{id}/ops");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn run_op(
        &self,
        id: &str,
        op_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/ops/{op_id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    // -- Access portals / invites ------------------------------------------------

    pub async fn list_access_portals(&self, id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{id}/access-portals");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn create_access_portal(
        &self,
        id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/access-portals");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn get_access_portal(&self, portal_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/access-portals/{portal_id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update_access_portal(
        &self,
        id: &str,
        portal_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/access-portals/{portal_id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn delete_access_portal(&self, id: &str, portal_id: &str) -> Result<(), SdkError> {
        let path = format!("/integrations/{id}/access-portals/{portal_id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update_access_invite(
        &self,
        id: &str,
        invite_link_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/access-invites/{invite_link_id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn delete_access_invite(
        &self,
        id: &str,
        invite_link_id: &str,
    ) -> Result<(), SdkError> {
        let path = format!("/integrations/{id}/access-invites/{invite_link_id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn restore_access_invite(
        &self,
        id: &str,
        invite_link_id: &str,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/access-invites/{invite_link_id}/restore");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
