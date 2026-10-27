//! `Zones` resource group.
//!
//! Zones are owned by an organization. Org-scoped calls (`list`, `tree`, `create`,
//! `get_by_name`) take the org id explicitly and send it as `X-Org`, so the target org never
//! depends on the client's default org.

use std::sync::Arc;

use openapp_sdk_common::NameMatch;
use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    resolve::{fetch_all_paginated, resolve_unique_sdk, use_server_q_prefilter},
    transport::{RequestSpec, Transport},
};

const ORG_HEADER: &str = "X-Org";

/// Optional filters for [`ZonesClient::list_with`].
#[derive(Debug, Clone, Default)]
pub struct ListZonesParams {
    /// Only direct children of this zone. Cannot be combined with `root`.
    pub parent_zone_id: Option<String>,
    /// Only root zones (no parent). Cannot be combined with `parent_zone_id`.
    pub root: bool,
    pub q: Option<String>,
    pub include_deleted: bool,
    pub only_deleted: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ZonesClient {
    transport: Arc<Transport>,
}

impl ZonesClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// `GET /zones` for the org subtree of `org_id` (first page, server defaults).
    pub async fn list(&self, org_id: &str) -> Result<JsonValue, SdkError> {
        self.list_with(org_id, ListZonesParams::default()).await
    }

    pub async fn list_with(
        &self,
        org_id: &str,
        params: ListZonesParams,
    ) -> Result<JsonValue, SdkError> {
        let query = build_list_query(&params);
        let headers = org_headers(org_id);
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/zones",
                query: &query,
                extra_headers: &headers,
                ..Default::default()
            })
            .await
    }

    /// `GET /zones/tree`: the org zone forest with direct device / entity counts.
    pub async fn tree(&self, org_id: &str) -> Result<JsonValue, SdkError> {
        let headers = org_headers(org_id);
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/zones/tree",
                extra_headers: &headers,
                ..Default::default()
            })
            .await
    }

    /// Resolve a single zone in `org_id` by localized display name.
    pub async fn get_by_name(
        &self,
        org_id: &str,
        name: &str,
        mode: NameMatch,
    ) -> Result<JsonValue, SdkError> {
        let q = use_server_q_prefilter(mode, name).then(|| name.to_owned());
        let org = org_id.to_owned();
        let client = self.clone();
        let zones = fetch_all_paginated(move |limit, offset| {
            let client = client.clone();
            let org = org.clone();
            let params = ListZonesParams {
                q: q.clone(),
                limit: Some(limit),
                offset: Some(offset),
                ..Default::default()
            };
            async move { client.list_with(&org, params).await }
        })
        .await?;
        resolve_unique_sdk(&zones, "zone", "name", name, mode)
    }

    /// `POST /zones` in `org_id`. Body: `{ name, parent_zone_id? }`.
    pub async fn create(&self, org_id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let headers = org_headers(org_id);
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/zones",
                body: Some(body),
                extra_headers: &headers,
                ..Default::default()
            })
            .await
    }

    pub async fn get(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/zones/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `GET /zones/{id}/summary`: path, children, and placement counts.
    pub async fn summary(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/zones/{id}/summary");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/zones/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// Soft-delete. With `detach`, references (child zones, devices, explicit entities) are
    /// cleared first; without it a referenced zone is rejected with 409 `zone_in_use`.
    pub async fn delete(&self, id: &str, detach: bool) -> Result<(), SdkError> {
        let path = format!("/zones/{id}");
        let query = [("cascade", detach.then(|| "detach".to_owned()))];
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                query: &query,
                ..Default::default()
            })
            .await
    }

    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/zones/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `POST /zones/{id}/merge`: move everything from `source_zone_ids` onto `id`.
    pub async fn merge(&self, id: &str, source_zone_ids: &[String]) -> Result<JsonValue, SdkError> {
        let path = format!("/zones/{id}/merge");
        let body = serde_json::json!({ "source_zone_ids": source_zone_ids });
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(&body),
                ..Default::default()
            })
            .await
    }
}

fn org_headers(org_id: &str) -> [(&'static str, String); 1] {
    [(ORG_HEADER, org_id.to_owned())]
}

fn build_list_query(params: &ListZonesParams) -> Vec<(&'static str, Option<String>)> {
    vec![
        ("parent_zone_id", params.parent_zone_id.clone()),
        ("root", params.root.then(|| "true".to_owned())),
        ("q", params.q.clone()),
        (
            "include_deleted",
            params.include_deleted.then(|| "true".to_owned()),
        ),
        (
            "only_deleted",
            params.only_deleted.then(|| "true".to_owned()),
        ),
        ("limit", params.limit.map(|n| n.to_string())),
        ("offset", params.offset.map(|n| n.to_string())),
    ]
}
