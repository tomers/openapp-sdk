//! C bridge roundtrip tests for localized name resolution exports.

#![allow(clippy::unwrap_used)]

use std::ffi::{CStr, CString, c_void};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::mpsc;
use std::time::Duration;

use openapp_sdk_core_c_bridge::BridgeStatus;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn cstr_to_string(p: *mut c_char) -> String {
    assert!(!p.is_null());
    unsafe { CStr::from_ptr(p).to_str().expect("utf-8").to_owned() }
}

unsafe fn free_cstr(p: *mut c_char) {
    if !p.is_null() {
        unsafe { openapp_sdk_core_c_bridge::openapp_sdk_string_free(p) };
    }
}

struct AsyncCbCtx {
    tx: mpsc::Sender<(i32, i32, String)>,
}

extern "C" fn async_resolve_complete(
    bridge_status: c_int,
    http_status: c_int,
    body: *mut c_char,
    user_data: *mut c_void,
) {
    assert!(!user_data.is_null());
    let ctx = unsafe { Box::from_raw(user_data.cast::<AsyncCbCtx>()) };
    let msg = if body.is_null() {
        String::new()
    } else {
        let s = cstr_to_string(body);
        unsafe { openapp_sdk_core_c_bridge::openapp_sdk_string_free(body) };
        s
    };
    let _ = ctx.tx.send((bridge_status, http_status, msg));
}

fn new_bridge_client(
    runtime: *mut openapp_sdk_core_c_bridge::BridgeRuntime,
    token: &str,
) -> *mut openapp_sdk_core_c_bridge::BridgeClient {
    unsafe {
        let mut err: *mut c_char = ptr::null_mut();
        let token_c = CString::new(token).expect("token cstring");
        let client =
            openapp_sdk_core_c_bridge::openapp_sdk_client_new(runtime, token_c.as_ptr(), &mut err);
        assert!(
            !client.is_null(),
            "openapp_sdk_client_new: {}",
            cstr_to_string(err)
        );
        free_cstr(err);
        client
    }
}

#[tokio::test]
async fn sync_integrations_get_by_name_exact_match() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/integrations"))
        .and(query_param("q", "Lobby Demo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{"id": "01HINT", "name": {"en": "Lobby Demo"}}],
            "total": 1
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let (code, http_status, body) = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null());
        let client = new_bridge_client(runtime, &token);

        let name_c = CString::new("Lobby Demo").unwrap();
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_st: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_integrations_get_by_name(
            client,
            name_c.as_ptr(),
            0,
            &mut out_body,
            &mut http_st,
        );
        let body = cstr_to_string(out_body);
        free_cstr(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        (code, http_st, body)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(code, BridgeStatus::Ok);
    assert_eq!(http_status, 200);
    let v: serde_json::Value = serde_json::from_str(&body).expect("json");
    assert_eq!(v["id"], "01HINT");
}

#[tokio::test]
async fn sync_integrations_get_by_name_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/integrations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [],
            "total": 0
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let (code, body) = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        let client = new_bridge_client(runtime, &token);
        let name_c = CString::new("Missing").unwrap();
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_st: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_integrations_get_by_name(
            client,
            name_c.as_ptr(),
            0,
            &mut out_body,
            &mut http_st,
        );
        let body = cstr_to_string(out_body);
        free_cstr(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        (code, body)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(code, BridgeStatus::ResourceNotFound);
    assert!(
        body.contains("resource_not_found") || body.contains("Missing"),
        "{body}"
    );
}

#[tokio::test]
async fn sync_integrations_get_by_name_ambiguous() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/integrations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [
                {"id": "01HA", "name": {"en": "Lobby"}},
                {"id": "01HB", "name": {"en": "Lobby"}}
            ],
            "total": 2
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let (code, body) = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        let client = new_bridge_client(runtime, &token);
        let name_c = CString::new("Lobby").unwrap();
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_st: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_integrations_get_by_name(
            client,
            name_c.as_ptr(),
            0,
            &mut out_body,
            &mut http_st,
        );
        let body = cstr_to_string(out_body);
        free_cstr(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        (code, body)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(code, BridgeStatus::AmbiguousResource);
    assert!(
        body.contains("ambiguous_resource") || body.contains("ambiguous"),
        "{body}"
    );
}

#[tokio::test]
async fn async_devices_get_by_name_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/devices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{"id": "01HDEV", "name": {"en": "Front Door"}}],
            "total": 1
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        let client = new_bridge_client(runtime, &token);
        let (tx, rx) = mpsc::channel();
        let ctx = Box::new(AsyncCbCtx { tx });
        let user_data = Box::into_raw(ctx).cast::<c_void>();
        let name_c = CString::new("Front Door").unwrap();
        let schedule = openapp_sdk_core_c_bridge::openapp_sdk_client_devices_get_by_name_async(
            client,
            name_c.as_ptr(),
            0,
            ptr::null(),
            Some(async_resolve_complete),
            user_data,
        );
        assert_eq!(schedule, BridgeStatus::Ok);
        let (st, http_st, body) = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("async completion");
        assert_eq!(st, BridgeStatus::Ok as i32);
        assert_eq!(http_st, 200);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        body
    })
    .await
    .expect("spawn_blocking");

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json");
    assert_eq!(v["id"], "01HDEV");
}

#[tokio::test]
async fn async_devices_get_by_name_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/devices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [],
            "total": 0
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let (st, body) = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        let client = new_bridge_client(runtime, &token);
        let (tx, rx) = mpsc::channel();
        let ctx = Box::new(AsyncCbCtx { tx });
        let user_data = Box::into_raw(ctx).cast::<c_void>();
        let name_c = CString::new("Missing Device").unwrap();
        let schedule = openapp_sdk_core_c_bridge::openapp_sdk_client_devices_get_by_name_async(
            client,
            name_c.as_ptr(),
            0,
            ptr::null(),
            Some(async_resolve_complete),
            user_data,
        );
        assert_eq!(schedule, BridgeStatus::Ok);
        let (st, _http_st, body) = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("async completion");
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        (st, body)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(st, BridgeStatus::ResourceNotFound as i32);
    assert!(body.contains("Missing Device"), "{body}");
}

#[tokio::test]
async fn async_zones_get_by_name_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/integrations/01HINT/zones"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{"id": "01HZONE", "name": {"en": "Building A"}}],
            "total": 1
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        let client = new_bridge_client(runtime, &token);
        let (tx, rx) = mpsc::channel();
        let ctx = Box::new(AsyncCbCtx { tx });
        let user_data = Box::into_raw(ctx).cast::<c_void>();
        let integration_c = CString::new("01HINT").unwrap();
        let name_c = CString::new("Building A").unwrap();
        let schedule = openapp_sdk_core_c_bridge::openapp_sdk_client_zones_get_by_name_async(
            client,
            integration_c.as_ptr(),
            name_c.as_ptr(),
            0,
            Some(async_resolve_complete),
            user_data,
        );
        assert_eq!(schedule, BridgeStatus::Ok);
        let (st, http_st, body) = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("async completion");
        assert_eq!(st, BridgeStatus::Ok as i32);
        assert_eq!(http_st, 200);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        body
    })
    .await
    .expect("spawn_blocking");

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json");
    assert_eq!(v["id"], "01HZONE");
}

#[tokio::test]
async fn async_orgs_get_by_name_happy_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/orgs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{"id": "01HORG", "name": {"en": "Acme HQ"}}],
            "total": 1
        })))
        .mount(&server)
        .await;

    let base = server.uri().trim_end_matches('/').to_owned();
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        let client = new_bridge_client(runtime, &token);
        let (tx, rx) = mpsc::channel();
        let ctx = Box::new(AsyncCbCtx { tx });
        let user_data = Box::into_raw(ctx).cast::<c_void>();
        let name_c = CString::new("Acme HQ").unwrap();
        let schedule = openapp_sdk_core_c_bridge::openapp_sdk_client_orgs_get_by_name_async(
            client,
            name_c.as_ptr(),
            0,
            Some(async_resolve_complete),
            user_data,
        );
        assert_eq!(schedule, BridgeStatus::Ok);
        let (st, http_st, body) = rx
            .recv_timeout(Duration::from_secs(10))
            .expect("async completion");
        assert_eq!(st, BridgeStatus::Ok as i32);
        assert_eq!(http_st, 200);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        body
    })
    .await
    .expect("spawn_blocking");

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json");
    assert_eq!(v["id"], "01HORG");
}

#[tokio::test]
async fn resolve_unique_json_not_found_and_ambiguous() {
    let (not_found_code, not_found_body) = tokio::task::spawn_blocking(|| unsafe {
        let items = CString::new(r#"[{"id":"1","name":{"en":"Other"}}]"#).unwrap();
        let field = CString::new("name").unwrap();
        let needle = CString::new("Lobby").unwrap();
        let mut out_ok: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let code = openapp_sdk_core_c_bridge::openapp_sdk_resolve_unique_json(
            items.as_ptr(),
            field.as_ptr(),
            needle.as_ptr(),
            0,
            &mut out_ok,
            &mut out_err,
        );
        let body = cstr_to_string(out_err);
        free_cstr(out_err);
        (code, body)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(not_found_code, BridgeStatus::ResourceNotFound);
    assert!(not_found_body.contains("Lobby"), "{not_found_body}");

    let (ambiguous_code, ambiguous_body) = tokio::task::spawn_blocking(|| unsafe {
        let items =
            CString::new(r#"[{"id":"1","name":{"en":"Lobby"}},{"id":"2","name":{"en":"Lobby"}}]"#)
                .unwrap();
        let field = CString::new("name").unwrap();
        let needle = CString::new("Lobby").unwrap();
        let mut out_ok: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let code = openapp_sdk_core_c_bridge::openapp_sdk_resolve_unique_json(
            items.as_ptr(),
            field.as_ptr(),
            needle.as_ptr(),
            0,
            &mut out_ok,
            &mut out_err,
        );
        let body = cstr_to_string(out_err);
        free_cstr(out_err);
        (code, body)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(ambiguous_code, BridgeStatus::AmbiguousResource);
    assert!(
        ambiguous_body.contains("ambiguous") || ambiguous_body.contains("2"),
        "{ambiguous_body}"
    );
}
