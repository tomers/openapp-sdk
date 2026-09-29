//! `Devices` resource group.

use std::sync::Arc;

use openapp_sdk_common::NameMatch;
use reqwest::Method;

use super::JsonValue;
use super::types;
use crate::{
    error::SdkError,
    resolve::{fetch_all_paginated, resolve_unique_sdk, use_server_q_prefilter},
    transport::{RequestSpec, Transport},
};

/// Optional filters for [`DevicesClient::list`].
#[derive(Debug, Clone, Default)]
pub struct ListDevicesParams {
    pub integration_id: Option<String>,
    pub zone_id: Option<String>,
    pub q: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DevicesClient {
    transport: Arc<Transport>,
}

impl DevicesClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<JsonValue, SdkError> {
        self.list_with(ListDevicesParams::default()).await
    }

    pub async fn list_with(&self, params: ListDevicesParams) -> Result<JsonValue, SdkError> {
        let query = build_list_query(&params);
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/devices",
                query: &query,
                ..Default::default()
            })
            .await
    }

    /// Resolve a single device by localized display name.
    pub async fn get_by_name(
        &self,
        name: &str,
        mode: NameMatch,
        integration_id: Option<&str>,
    ) -> Result<JsonValue, SdkError> {
        let q = use_server_q_prefilter(mode, name).then(|| name.to_owned());
        let integration = integration_id.map(str::to_owned);
        let client = self.clone();
        let all_items = fetch_all_paginated(move |limit, offset| {
            let client = client.clone();
            let params = ListDevicesParams {
                integration_id: integration.clone(),
                q: q.clone(),
                limit: Some(limit),
                offset: Some(offset),
                ..Default::default()
            };
            async move {
                let page = client.list_with(params).await?;
                Ok(normalize_devices_page(page))
            }
        })
        .await?;
        resolve_unique_sdk(&all_items, "device", "name", name, mode)
    }

    pub async fn create(
        &self,
        body: &types::CreateDeviceRequest,
    ) -> Result<types::DeviceResponse, SdkError> {
        self.transport
            .request_json::<types::CreateDeviceRequest, types::DeviceResponse>(RequestSpec {
                method: Method::POST,
                path: "/devices",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn get(&self, id: &str) -> Result<types::DeviceResponse, SdkError> {
        let path = format!("/devices/{id}");
        self.transport
            .request_json::<(), types::DeviceResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &types::UpdateDeviceRequest,
    ) -> Result<types::DeviceResponse, SdkError> {
        let path = format!("/devices/{id}");
        self.transport
            .request_json::<types::UpdateDeviceRequest, types::DeviceResponse>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/devices/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/devices/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn restore(&self, id: &str) -> Result<types::DeviceResponse, SdkError> {
        let path = format!("/devices/{id}/restore");
        self.transport
            .request_json::<(), types::DeviceResponse>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn door_restrictions(
        &self,
        id: &str,
    ) -> Result<types::DoorRestrictionsResponse, SdkError> {
        let path = format!("/devices/{id}/door-restrictions");
        self.transport
            .request_json::<(), types::DoorRestrictionsResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn metadata_definition(
        &self,
        id: &str,
    ) -> Result<types::DeviceMetadataDefinitionResponse, SdkError> {
        let path = format!("/devices/{id}/metadata-definition");
        self.transport
            .request_json::<(), types::DeviceMetadataDefinitionResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}

fn build_list_query(params: &ListDevicesParams) -> Vec<(&str, Option<String>)> {
    vec![
        (
            "integration_id",
            params
                .integration_id
                .as_ref()
                .map(std::borrow::ToOwned::to_owned),
        ),
        (
            "zone_id",
            params.zone_id.as_ref().map(std::borrow::ToOwned::to_owned),
        ),
        ("q", params.q.as_ref().map(std::borrow::ToOwned::to_owned)),
        ("limit", params.limit.map(|n| n.to_string())),
        ("offset", params.offset.map(|n| n.to_string())),
    ]
}

fn normalize_devices_page(page: JsonValue) -> JsonValue {
    if page.is_array() {
        let len = page.as_array().map_or(0, std::vec::Vec::len);
        serde_json::json!({ "items": page, "total": len })
    } else {
        page
    }
}
