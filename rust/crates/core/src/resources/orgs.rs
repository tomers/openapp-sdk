//! `Orgs` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::types;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct OrgsClient {
    transport: Arc<Transport>,
}

impl OrgsClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    pub async fn list(&self) -> Result<types::PaginatedResponse, SdkError> {
        self.transport
            .request_json::<(), types::PaginatedResponse>(RequestSpec {
                method: Method::GET,
                path: "/orgs",
                ..Default::default()
            })
            .await
    }

    pub async fn create(
        &self,
        body: &types::CreateOrganizationRequest,
    ) -> Result<types::OrganizationResponse, SdkError> {
        self.transport
            .request_json::<types::CreateOrganizationRequest, types::OrganizationResponse>(
                RequestSpec {
                    method: Method::POST,
                    path: "/orgs",
                    body: Some(body),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn get(&self, id: &str) -> Result<types::OrganizationResponse, SdkError> {
        let path = format!("/orgs/{id}");
        self.transport
            .request_json::<(), types::OrganizationResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        body: &types::UpdateOrganizationRequest,
    ) -> Result<types::OrganizationResponse, SdkError> {
        let path = format!("/orgs/{id}");
        self.transport
            .request_json::<types::UpdateOrganizationRequest, types::OrganizationResponse>(
                RequestSpec {
                    method: Method::PUT,
                    path: &path,
                    body: Some(body),
                    ..Default::default()
                },
            )
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/orgs/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/orgs/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn permissions(&self, id: &str) -> Result<types::OrgPermissionsResponse, SdkError> {
        let path = format!("/orgs/{id}/permissions");
        self.transport
            .request_json::<(), types::OrgPermissionsResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn users(&self, org_id: &str) -> Result<types::PaginatedResponse, SdkError> {
        let path = format!("/orgs/{org_id}/users");
        self.transport
            .request_json::<(), types::PaginatedResponse>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
