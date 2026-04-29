//! `Users` resource group.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct UsersClient {
    transport: Arc<Transport>,
}

impl UsersClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    /// `POST /users` — create a user (org-admin required).
    pub async fn create(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/users",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `GET /users/search?q=…` — fuzzy search by email / name.
    pub async fn search(&self, query: &str) -> Result<Vec<JsonValue>, SdkError> {
        let params = [("q", Some(query.to_string()))];
        self.transport
            .request_json::<(), Vec<JsonValue>>(RequestSpec {
                method: Method::GET,
                path: "/users/search",
                query: &params,
                ..Default::default()
            })
            .await
    }

    /// `GET /users/{id}`
    pub async fn get(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/users/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `PUT /users/{id}`
    pub async fn update(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/users/{id}");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::PUT,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `DELETE /users/{id}` — soft-delete.
    pub async fn delete(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/users/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `DELETE /users/{id}/purge` — hard-delete.
    pub async fn purge(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/users/{id}/purge");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `POST /users/{id}/roles` — add roles to a user.
    pub async fn add_roles(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/users/{id}/roles");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `DELETE /users/{id}/roles` — remove roles from a user.
    pub async fn remove_roles(&self, id: &str, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let path = format!("/users/{id}/roles");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }
}
