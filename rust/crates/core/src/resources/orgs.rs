//! `Orgs` resource group.

use std::sync::Arc;

use openapp_sdk_common::NameMatch;
use reqwest::Method;

use super::JsonValue;
use super::types;
use crate::{
    error::SdkError,
    resolve::{fetch_all_paginated, resolve_unique_sdk},
    transport::{RequestSpec, Transport},
};

/// Optional filters for [`OrgsClient::list`].
#[derive(Debug, Clone, Default)]
pub struct ListOrgsParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct OrgsClient {
    transport: Arc<Transport>,
}

impl OrgsClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<JsonValue, SdkError> {
        self.list_with(ListOrgsParams::default()).await
    }

    pub async fn list_with(&self, params: ListOrgsParams) -> Result<JsonValue, SdkError> {
        let query = vec![
            ("limit", params.limit.map(|n| n.to_string())),
            ("offset", params.offset.map(|n| n.to_string())),
        ];
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/orgs",
                query: &query,
                ..Default::default()
            })
            .await
    }

    /// Resolve a single organization by localized display name.
    pub async fn get_by_name(&self, name: &str, mode: NameMatch) -> Result<JsonValue, SdkError> {
        let client = self.clone();
        let items = fetch_all_paginated(move |limit, offset| {
            let client = client.clone();
            let params = ListOrgsParams {
                limit: Some(limit),
                offset: Some(offset),
            };
            async move { client.list_with(params).await }
        })
        .await?;
        resolve_unique_sdk(&items, "org", "name", name, mode)
    }

    pub async fn create(
        &self,
        body: &types::CreateOrganizationRequest,
    ) -> Result<types::OrganizationResponse, SdkError> {
        self.transport
            .request_json::<types::CreateOrganizationRequest, types::OrganizationResponse>(
                RequestSpec {
                    method: Method::POST,
                    path: "/orgs",
                    body: Some(body),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn get(&self, id: &str) -> Result<types::OrganizationResponse, SdkError> {
        let path = format!("/orgs/{id}");
        self.transport
            .request_json::<(), types::OrganizationResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &types::UpdateOrganizationRequest,
    ) -> Result<types::OrganizationResponse, SdkError> {
        let path = format!("/orgs/{id}");
        self.transport
            .request_json::<types::UpdateOrganizationRequest, types::OrganizationResponse>(
                RequestSpec {
                    method: Method::PUT,
                    path: &path,
                    body: Some(body),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/orgs/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/orgs/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn permissions(&self, id: &str) -> Result<types::OrgPermissionsResponse, SdkError> {
        let path = format!("/orgs/{id}/permissions");
        self.transport
            .request_json::<(), types::OrgPermissionsResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn users(&self, org_id: &str) -> Result<types::PaginatedResponse, SdkError> {
        let path = format!("/orgs/{org_id}/users");
        self.transport
            .request_json::<(), types::PaginatedResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `POST /orgs/{id}/resolve` — natural-language entity/portal resolution.
    pub async fn resolve(&self, org_id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/orgs/{org_id}/resolve");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `POST /orgs/{id}/aliases`
    pub async fn create_alias(
        &self,
        org_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/orgs/{org_id}/aliases");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn list_aliases(&self, org_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/orgs/{org_id}/aliases");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn copilot_chat(
        &self,
        org_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/orgs/{org_id}/copilot/chat");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn copilot_commit(
        &self,
        org_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/orgs/{org_id}/copilot/commit");
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
