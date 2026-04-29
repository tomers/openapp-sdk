//! End-to-end tests for the transport layer using [`wiremock`].

use std::time::Duration;

use openapp_sdk_core::{Client, SdkError};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{bearer_token, method, path},
};

fn build_client(base_url: &str, token: &str) -> Client {
    Client::builder()
        .api_key(token)
        .base_url(base_url)
        .unwrap()
        .default_timeout(Duration::from_secs(2))
        .build()
        .unwrap()
}

#[tokio::test]
async fn get_status_attaches_bearer() {
    let server = MockServer::start().await;
    let token = "https://api.test_openapp_SECRET";
    Mock::given(method("GET"))
        .and(path("/status"))
        .and(bearer_token(token))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "backend": "ok",
            "database": "ok"
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), token);
    let body = client.status().get().await.unwrap();
    assert_eq!(body["backend"], "ok");
}

#[tokio::test]
async fn non_json_error_surfaces_as_http() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/status"))
        .respond_with(
            ResponseTemplate::new(502)
                .set_body_bytes("bad gateway")
                .append_header("content-type", "text/plain"),
        )
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let err = client.status().get().await.unwrap_err();
    match err {
        SdkError::Http { status, message } => {
            assert_eq!(status, 502);
            assert!(message.contains("bad gateway"));
        }
        other => panic!("expected SdkError::Http, got {other:?}"),
    }
}

#[tokio::test]
async fn json_error_decodes_into_api_variant() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/orgs"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "code": "validation_error",
            "message": "name is required"
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let err = client.orgs().create(&json!({})).await.unwrap_err();
    match err {
        SdkError::Api { status, body } => {
            assert_eq!(status, 400);
            assert_eq!(body.message, "name is required");
            assert_eq!(body.code.as_deref(), Some("validation_error"));
        }
        other => panic!("expected SdkError::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn unauthorized_surfaces_as_auth() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/orgs"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "code": "unauthorized",
            "message": "token revoked"
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let err = client.orgs().list().await.unwrap_err();
    assert!(matches!(err, SdkError::Auth(_)));
    assert!(err.to_string().contains("token revoked"));
}

#[tokio::test]
async fn malformed_token_surfaces_at_request_time() {
    // Skips URL parsing by supplying an explicit base URL.
    let client = Client::builder()
        .api_key("not a token")
        .base_url("http://localhost:9999")
        .unwrap()
        .build()
        .unwrap();
    let err = client.status().get().await.unwrap_err();
    assert!(matches!(err, SdkError::Auth(_)), "got {err:?}");
}
