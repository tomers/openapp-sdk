//! `Public Access` resource group.
//!
//! These endpoints are authentication-free (they run on opaque session / invite
//! tokens in the path) but are still exposed through the SDK so tooling can drive
//! them programmatically.

use std::sync::Arc;

use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

#[derive(Debug, Clone)]
pub struct PublicAccessClient {
    transport: Arc<Transport>,
}

impl PublicAccessClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self { transport }
    }

    // -- Invites -----------------------------------------------------------------

    pub async fn get_invite(&self, invite_token: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/invites/{invite_token}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn claim_invite(
        &self,
        invite_token: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/invites/{invite_token}/claim");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn execute_invite(
        &self,
        invite_token: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/invites/{invite_token}/execute");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn start_invite_session(
        &self,
        invite_token: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/invites/{invite_token}/session");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    // -- Portals -----------------------------------------------------------------

    pub async fn get_portal(&self, public_portal_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/portals/{public_portal_id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn portal_lights(
        &self,
        public_portal_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/portals/{public_portal_id}/lights");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn portal_open(
        &self,
        public_portal_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/portals/{public_portal_id}/open");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn portal_reachable(&self, public_portal_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/portals/{public_portal_id}/reachable");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn portal_start_session(
        &self,
        public_portal_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/portals/{public_portal_id}/sessions");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn portal_targets(&self, public_portal_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/portals/{public_portal_id}/targets");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    // -- Sessions ----------------------------------------------------------------

    pub async fn get_session(&self, session_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn cancel_session(&self, session_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}/cancel");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn decline_session(&self, session_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}/decline");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                ..Default::default()
            })
            .await
    }

    pub async fn session_lights(
        &self,
        session_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}/lights");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn session_notify_message(
        &self,
        session_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}/notify-message");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn session_open(
        &self,
        session_id: &str,
        body: &JsonValue,
    ) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}/open");
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: &path,
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    pub async fn session_streams(&self, session_id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/public/access/sessions/{session_id}/streams");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }
}
