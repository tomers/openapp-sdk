//! `Integrations` resource group.

use std::sync::Arc;

use openapp_sdk_common::NameMatch;
use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    resolve::{
        fetch_all_paginated, items_from_list_response, resolve_unique_sdk, use_server_q_prefilter,
    },
    transport::{RequestSpec, Transport},
};

/// Optional filters for [`IntegrationsClient::list`].
#[derive(Debug, Clone, Default)]
pub struct ListIntegrationsParams {
    pub provider_type: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct IntegrationsClient {
    transport: Arc<Transport>,
}

impl IntegrationsClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<JsonValue, SdkError> {
        self.list_with(ListIntegrationsParams::default()).await
    }

    pub async fn list_with(&self, params: ListIntegrationsParams) -> Result<JsonValue, SdkError> {
        let query = build_list_query(&params);
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/integrations",
                query: &query,
                ..Default::default()
            })
            .await
    }

    /// Resolve a single integration by localized display name.
    pub async fn get_by_name(
        &self,
        name: &str,
        mode: NameMatch,
        provider_type: Option<&str>,
    ) -> Result<JsonValue, SdkError> {
        let q = use_server_q_prefilter(mode, name).then(|| name.to_owned());
        let provider = provider_type.map(str::to_owned);
        let client = self.clone();
        let items = fetch_all_paginated(move |limit, offset| {
            let client = client.clone();
            let params = ListIntegrationsParams {
                provider_type: provider.clone(),
                q: q.clone(),
                limit: Some(limit),
                offset: Some(offset),
            };
            async move { client.list_with(params).await }
        })
        .await?;
        resolve_unique_sdk(&items, "integration", "name", name, mode)
    }

    /// Resolve a single access portal under an integration by localized display name.
    pub async fn get_access_portal_by_name(
        &self,
        integration_id: &str,
        name: &str,
        mode: NameMatch,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{integration_id}/access-portals");
        let response = self
            .transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await?;
        let items = items_from_list_response(&response, Some("portals"));
        resolve_unique_sdk(&items, "portal", "name", name, mode)
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

    pub async fn list_access_portals(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/access-portals");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
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

    pub async fn create_access_invite(
        &self,
        id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{id}/access-invites");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
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

fn build_list_query(params: &ListIntegrationsParams) -> Vec<(&str, Option<String>)> {
    vec![
        (
            "provider_type",
            params
                .provider_type
                .as_ref()
                .map(std::borrow::ToOwned::to_owned),
        ),
        ("q", params.q.as_ref().map(std::borrow::ToOwned::to_owned)),
        ("limit", params.limit.map(|n| n.to_string())),
        ("offset", params.offset.map(|n| n.to_string())),
    ]
}
