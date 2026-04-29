//! `Apartment Residents` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct ApartmentResidentsClient {
    transport: Arc<Transport>,
}

impl ApartmentResidentsClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self, integration_id: &str) -> Result<Vec<JsonValue>, SdkError> {
        let path = format!("/integrations/{integration_id}/building-users");
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn add(&self, integration_id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/integrations/{integration_id}/building-users");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn remove(&self, integration_id: &str, user_id: &str) -> Result<(), SdkError> {
        let path = format!("/integrations/{integration_id}/building-users/{user_id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
