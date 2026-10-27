//! `Me` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::types;
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

    pub async fn profile(&self) -> Result<types::MeProfileResponse, SdkError> {
        self.transport
            .request_json::<(), types::MeProfileResponse>(RequestSpec {
                method: Method::GET,
                path: "/me/profile",
                ..Default::default()
            })
            .await
    }

    pub async fn directory_listings(&self) -> Result<types::MeListingsResponse, SdkError> {
        self.transport
            .request_json::<(), types::MeListingsResponse>(RequestSpec {
                method: Method::GET,
                path: "/me/directory-listings",
                ..Default::default()
            })
            .await
    }

    pub async fn invitations(&self) -> Result<types::MeInvitationsResponse, SdkError> {
        self.transport
            .request_json::<(), types::MeInvitationsResponse>(RequestSpec {
                method: Method::GET,
                path: "/me/invitations",
                ..Default::default()
            })
            .await
    }
}
