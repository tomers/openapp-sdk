//! `Entities` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct EntitiesClient {
    transport: Arc<Transport>,
}

impl EntitiesClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/entities",
                ..Default::default()
            })
            .await
    }

    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/entities",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn get(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/entities/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/entities/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn patch(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/entities/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PATCH,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/entities/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/entities/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn restore(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/entities/{id}/restore");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `POST /entities/{id}/actions/{action_id}` — invoke a DSL/entity action.
    pub async fn invoke_action(
        &self,
        id: &str,
        action_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/entities/{id}/actions/{action_id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn metadata_definition(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/entities/{id}/metadata-definition");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn by_device(&self, device_id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/devices/{device_id}/entities");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn device_entities_metadata_definition(
        &self,
        device_id: &str,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{device_id}/entities/metadata-definition");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn apartment_floors(&self, device_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/devices/{device_id}/apartment-floors");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
