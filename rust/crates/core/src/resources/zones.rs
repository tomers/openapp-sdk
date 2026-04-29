//! `Zones` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct ZonesClient {
    transport: Arc<Transport>,
}

impl ZonesClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/zones",
                body: Some(body),
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

    pub async fn delete(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/zones/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
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

    pub async fn by_integration(&self, integration_id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{integration_id}/zones");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
