//! Integration tests for localized name resolution (`get_by_name`).

use std::time::Duration;

use openapp_sdk_common::NameMatch;
use openapp_sdk_core::{Client, SdkError};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
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
async fn integrations_get_by_name_exact_match() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/integrations"))
        .and(query_param("q", "Lobby Demo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{
                "id": "01HINT",
                "name": {"en": "Lobby Demo"}
            }],
            "total": 1
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let integration = client
        .integrations()
        .get_by_name("Lobby Demo", NameMatch::Exact, None)
        .await
        .unwrap();
    assert_eq!(integration["id"], "01HINT");
}

#[tokio::test]
async fn integrations_get_by_name_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/integrations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [],
            "total": 0
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let err = client
        .integrations()
        .get_by_name("Missing", NameMatch::Exact, None)
        .await
        .unwrap_err();
    assert!(matches!(
        err,
        SdkError::ResourceNotFound {
            resource_type: "integration",
            ..
        }
    ));
}

#[tokio::test]
async fn integrations_get_by_name_ambiguous() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/integrations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                {"id": "01HA", "name": {"en": "Lobby"}},
                {"id": "01HB", "name": {"en": "Lobby"}}
            ],
            "total": 2
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let err = client
        .integrations()
        .get_by_name("Lobby", NameMatch::Exact, None)
        .await
        .unwrap_err();
    match err {
        SdkError::AmbiguousResource {
            resource_type,
            name,
            match_count,
            ..
        } => {
            assert_eq!(resource_type, "integration");
            assert_eq!(name, "Lobby");
            assert_eq!(match_count, 2);
        }
        other => panic!("expected AmbiguousResource, got {other:?}"),
    }
}

#[tokio::test]
async fn integrations_get_by_name_fuzzy_substring() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/integrations"))
        .and(query_param("q", "Lobby"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                {"id": "01HA", "name": {"en": "Main Lobby"}},
                {"id": "01HB", "name": {"en": "Side Door"}}
            ],
            "total": 2
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let integration = client
        .integrations()
        .get_by_name("Lobby", NameMatch::Fuzzy, None)
        .await
        .unwrap();
    assert_eq!(integration["id"], "01HA");
}

#[tokio::test]
async fn integrations_get_by_name_paginates() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/integrations"))
        .and(query_param("offset", "0"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{"id": "page1", "name": {"en": "Other"}}],
            "total": 2
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/integrations"))
        .and(query_param("offset", "200"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{"id": "target", "name": {"en": "Target Org"}}],
            "total": 2
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let integration = client
        .integrations()
        .get_by_name("Target Org", NameMatch::Exact, None)
        .await
        .unwrap();
    assert_eq!(integration["id"], "target");
}

#[tokio::test]
async fn devices_get_by_name_resolves() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/devices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{"id": "01HDEV", "name": {"en": "Front Door"}}],
            "total": 1
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let device = client
        .devices()
        .get_by_name("Front Door", NameMatch::Exact, None)
        .await
        .unwrap();
    assert_eq!(device["id"], "01HDEV");
}

#[tokio::test]
async fn orgs_get_by_name_resolves() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/orgs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{"id": "01HORG", "name": {"en": "Acme HQ"}}],
            "total": 1
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri(), "https://api.test_openapp_SECRET");
    let org = client
        .orgs()
        .get_by_name("Acme HQ", NameMatch::Exact)
        .await
        .unwrap();
    assert_eq!(org["id"], "01HORG");
}
