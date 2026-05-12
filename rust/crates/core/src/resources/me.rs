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

    pub async fn apartments(&self) -> Result<types::MeApartmentsResponse, SdkError> {
        self.transport
            .request_json::<(), types::MeApartmentsResponse>(RequestSpec {
                method: Method::GET,
                path: "/me/apartments",
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

    pub async fn push_subscription_status(
        &self,
    ) -> Result<types::MePushSubscriptionStatusResponse, SdkError> {
        self.transport
            .request_json::<(), types::MePushSubscriptionStatusResponse>(RequestSpec {
                method: Method::GET,
                path: "/me/push-subscription-status",
                ..Default::default()
            })
            .await
    }

    pub async fn push_vapid_public_key(
        &self,
    ) -> Result<types::MePushVapidPublicKeyResponse, SdkError> {
        self.transport
            .request_json::<(), types::MePushVapidPublicKeyResponse>(RequestSpec {
                method: Method::GET,
                path: "/me/push-vapid-public-key",
                ..Default::default()
            })
            .await
    }

    pub async fn subscribe_push(
        &self,
        body: &types::PostMePushSubscriptionPayload,
    ) -> Result<(), SdkError> {
        self.transport
            .request_json::<types::PostMePushSubscriptionPayload, ()>(RequestSpec {
                method: Method::POST,
                path: "/me/push-subscriptions",
                body: Some(body),
                ..Default::default()
            })
            .await
    }
}
