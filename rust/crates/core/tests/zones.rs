//! Zones are org-owned: org-scoped calls carry the org as `X-Org`, independent of the
//! client's default org.

use std::time::Duration;

use openapp_sdk_common::NameMatch;
use openapp_sdk_core::Client;
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
};

fn build_client(base_url: &str) -> Client {
    Client::builder()
        .api_key("https://api.test_openapp_SECRET")
        .base_url(base_url)
        .unwrap()
        .default_timeout(Duration::from_secs(2))
        .build()
        .unwrap()
}

#[tokio::test]
async fn zones_get_by_name_lists_org_zones() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .and(header("X-Org", "01HORG"))
        .and(query_param("q", "Kitchen"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{"id": "01HZONE", "org_id": "01HORG", "name": {"en": "Kitchen"}}],
            "total": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let zone = build_client(&server.uri())
        .zones()
        .get_by_name("01HORG", "Kitchen", NameMatch::Exact)
        .await
        .unwrap();
    assert_eq!(zone["id"], "01HZONE");
}

#[tokio::test]
async fn zones_tree_is_org_scoped() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/zones/tree"))
        .and(header("X-Org", "01HORG"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"items": []})))
        .expect(1)
        .mount(&server)
        .await;

    let tree = build_client(&server.uri())
        .zones()
        .tree("01HORG")
        .await
        .unwrap();
    assert_eq!(tree["items"], json!([]));
}

#[tokio::test]
async fn zones_merge_posts_source_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/zones/01HZ1/merge"))
        .and(body_json(json!({"source_zone_ids": ["01HZ2"]})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "zone": {"id": "01HZ1"},
            "moved_child_zones": 0,
            "moved_devices": 1,
            "moved_entities": 2,
            "moved_provider_links": 0
        })))
        .expect(1)
        .mount(&server)
        .await;

    let merged = build_client(&server.uri())
        .zones()
        .merge("01HZ1", &["01HZ2".to_owned()])
        .await
        .unwrap();
    assert_eq!(merged["moved_entities"], 2);
}

#[tokio::test]
async fn zones_delete_detach_sets_cascade() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/zones/01HZ1"))
        .and(query_param("cascade", "detach"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    build_client(&server.uri())
        .zones()
        .delete("01HZ1", true)
        .await
        .unwrap();
}
