//! `Billing` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::types;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct BillingClient {
    transport: Arc<Transport>,
}

impl BillingClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn checkout(
        &self,
        org_id: &str,
        body: &types::CheckoutRequest,
    ) -> Result<types::SubscriptionRef, SdkError> {
        let path = format!("/orgs/{org_id}/billing/checkout");
        self.transport
            .request_json::<types::CheckoutRequest, types::SubscriptionRef>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn plan(&self, org_id: &str) -> Result<types::BillingPlanResponse, SdkError> {
        let path = format!("/orgs/{org_id}/billing/plan");
        self.transport
            .request_json::<(), types::BillingPlanResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn portal(
        &self,
        org_id: &str,
        body: &types::PortalRequest,
    ) -> Result<types::PortalResponse, SdkError> {
        let path = format!("/orgs/{org_id}/billing/portal");
        self.transport
            .request_json::<types::PortalRequest, types::PortalResponse>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn upgrade_options(&self, org_id: &str) -> Result<types::PlansResponse, SdkError> {
        let path = format!("/orgs/{org_id}/billing/upgrade-options");
        self.transport
            .request_json::<(), types::PlansResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn usage(&self, org_id: &str) -> Result<types::QuotaReport, SdkError> {
        let path = format!("/orgs/{org_id}/billing/usage");
        self.transport
            .request_json::<(), types::QuotaReport>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
