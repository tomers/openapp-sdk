//! `Me` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct MeClient {
    transport: Arc<Transport>,
}

impl MeClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn apartments(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/me/apartments",
                ..Default::default()
            })
            .await
    }

    pub async fn invitations(&self) -> Result<Vec<JsonValue>, SdkError> {
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/me/invitations",
                ..Default::default()
            })
            .await
    }

    pub async fn push_subscription_status(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/me/push-subscription-status",
                ..Default::default()
            })
            .await
    }

    pub async fn push_vapid_public_key(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/me/push-vapid-public-key",
                ..Default::default()
            })
            .await
    }

    pub async fn subscribe_push(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/me/push-subscriptions",
                body: Some(body),
                ..Default::default()
            })
            .await
    }
}
