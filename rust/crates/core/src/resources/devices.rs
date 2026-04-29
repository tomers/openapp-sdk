//! `Devices` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct DevicesClient {
    transport: Arc<Transport>,
}

impl DevicesClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/devices",
                ..Default::default()
            })
            .await
    }

    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/devices",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn get(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{id}");
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

    pub async fn restore(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{id}/restore");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn door_restrictions(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{id}/door-restrictions");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn metadata_definition(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{id}/metadata-definition");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
