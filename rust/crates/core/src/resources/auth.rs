//! `Auth` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct AuthClient {
    transport: Arc<Transport>,
}

impl AuthClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn whoami(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/auth/whoami",
                ..Default::default()
            })
            .await
    }

    pub async fn session(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/auth/session",
                ..Default::default()
            })
            .await
    }

    pub async fn kratos_identity(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/auth/kratos-identity",
                ..Default::default()
            })
            .await
    }

    pub async fn provisioned(&self) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/auth/provisioned",
                ..Default::default()
            })
            .await
    }

    pub async fn logout(&self) -> Result<(), SdkError> {
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::POST,
                path: "/auth/logout",
                ..Default::default()
            })
            .await
    }

    pub async fn sign_out(&self) -> Result<(), SdkError> {
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::POST,
                path: "/auth/sign-out",
                ..Default::default()
            })
            .await
    }
}
