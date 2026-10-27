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
