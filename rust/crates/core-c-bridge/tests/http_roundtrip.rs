//! Integration tests for the C ABI: real HTTP through `openapp-sdk-core` transport.
//!
//! `openapp_sdk_client_request` uses `tokio::Runtime::block_on` on the bridge runtime.
//! Those calls must not run on Tokio's async worker threads (nested runtime issue), so we
//! run FFI inside [`tokio::task::spawn_blocking`].

#![allow(clippy::unwrap_used)]

use std::ffi::{CStr, CString, c_void};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use openapp_sdk_core_c_bridge::{
    BridgeStatus, OPENAPP_SDK_USE_DEFAULT_MAX_RETRIES, OpenAppSdkClientConfig,
};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn cstr_to_string(p: *mut c_char) -> String {
    assert!(!p.is_null());
    unsafe { CStr::from_ptr(p).to_str().expect("utf-8").to_owned() }
}

unsafe fn free_cstr(p: *mut c_char) {
    if !p.is_null() {
        // SAFETY: `p` was returned from the bridge and not yet freed.
        unsafe { openapp_sdk_core_c_bridge::openapp_sdk_string_free(p) };
    }
}

struct AsyncCbCtx {
    tx: mpsc::Sender<(i32, i32, String)>,
}

extern "C" fn async_request_complete(
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

#[tokio::test]
async fn c_bridge_get_roundtrip_returns_json() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ping"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "hello": "world" })),
        )
        .mount(&server)
        .await;

    let base = server.uri().to_string();
    let base = base.trim_end_matches('/');
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null(), "openapp_sdk_runtime_new");

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

        let method_c = CString::new("GET").unwrap();
        let path_c = CString::new("/ping").unwrap();
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_status: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_request(
            client,
            method_c.as_ptr(),
            path_c.as_ptr(),
            ptr::null(),
            &mut out_body,
            &mut http_status,
        );
        assert_eq!(code, BridgeStatus::Ok);
        assert_eq!(http_status, 200);
        let body = cstr_to_string(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_string_free(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        body
    })
    .await
    .expect("spawn_blocking");

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json body");
    assert_eq!(v["hello"], "world");
}

#[tokio::test]
async fn c_bridge_async_get_roundtrip_returns_json_via_callback() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ping"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "hello": "async" })),
        )
        .mount(&server)
        .await;

    let base = server.uri().to_string();
    let base = base.trim_end_matches('/');
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null(), "openapp_sdk_runtime_new");

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

        let (tx, rx) = mpsc::channel();
        let ctx = Box::new(AsyncCbCtx { tx });
        let user_data = Box::into_raw(ctx).cast::<c_void>();

        let method_c = CString::new("GET").unwrap();
        let path_c = CString::new("/ping").unwrap();
        let schedule = openapp_sdk_core_c_bridge::openapp_sdk_client_request_async(
            client,
            method_c.as_ptr(),
            path_c.as_ptr(),
            ptr::null(),
            Some(async_request_complete),
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

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json body");
    assert_eq!(v["hello"], "async");
}

#[tokio::test]
async fn c_bridge_non_2xx_sets_api_status_and_error_body() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/gone"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "message": "not found"
        })))
        .mount(&server)
        .await;

    let base = server.uri().to_string();
    let base = base.trim_end_matches('/');
    let token = format!("{base}_openapp_xyzzy");

    let (code, http_status, msg) = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null());

        let mut err: *mut c_char = ptr::null_mut();
        let token_c = CString::new(token).expect("token cstring");
        let client =
            openapp_sdk_core_c_bridge::openapp_sdk_client_new(runtime, token_c.as_ptr(), &mut err);
        assert!(!client.is_null());
        free_cstr(err);

        let method_c = CString::new("GET").unwrap();
        let path_c = CString::new("/gone").unwrap();
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_status: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_request(
            client,
            method_c.as_ptr(),
            path_c.as_ptr(),
            ptr::null(),
            &mut out_body,
            &mut http_status,
        );
        let msg = cstr_to_string(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_string_free(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        (code, http_status, msg)
    })
    .await
    .expect("spawn_blocking");

    assert_eq!(code, BridgeStatus::ApiError);
    assert_eq!(http_status, 404);
    assert!(
        msg.contains("404") || msg.contains("not found"),
        "unexpected error text: {msg}"
    );
}

#[tokio::test]
async fn c_bridge_config_custom_user_agent_is_sent() {
    const UA: &str = "openapp-c-bridge-config-test/1.0";
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ping"))
        .and(header("user-agent", UA))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "ok": true })))
        .mount(&server)
        .await;

    let base = server.uri().to_string();
    let base = base.trim_end_matches('/');
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null());

        let token_c = CString::new(token).expect("token cstring");
        let ua_c = CString::new(UA).expect("ua cstring");
        let conf = OpenAppSdkClientConfig {
            api_key: token_c.as_ptr(),
            base_url: ptr::null(),
            user_agent: ua_c.as_ptr(),
            default_timeout_secs: 0,
            max_retries: OPENAPP_SDK_USE_DEFAULT_MAX_RETRIES,
        };

        let mut err: *mut c_char = ptr::null_mut();
        let client = openapp_sdk_core_c_bridge::openapp_sdk_client_new_with_config(
            runtime,
            &raw const conf,
            &mut err,
        );
        assert!(
            !client.is_null(),
            "openapp_sdk_client_new_with_config: {}",
            cstr_to_string(err)
        );
        free_cstr(err);

        let method_c = CString::new("GET").unwrap();
        let path_c = CString::new("/ping").unwrap();
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_status: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_request(
            client,
            method_c.as_ptr(),
            path_c.as_ptr(),
            ptr::null(),
            &mut out_body,
            &mut http_status,
        );
        assert_eq!(code, BridgeStatus::Ok);
        assert_eq!(http_status, 200);
        let body = cstr_to_string(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_string_free(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        body
    })
    .await
    .expect("spawn_blocking");

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json body");
    assert_eq!(v["ok"], true);
}

struct StreamCbCtx {
    chunks: Arc<Mutex<Vec<u8>>>,
    done_tx: mpsc::Sender<(i32, i32, Option<String>)>,
}

extern "C" fn stream_chunk_cb(data: *const u8, len: usize, user_data: *mut c_void) -> c_int {
    assert!(!user_data.is_null());
    let ctx = unsafe { &*(user_data as *const StreamCbCtx) };
    let sl = unsafe { std::slice::from_raw_parts(data, len) };
    ctx.chunks
        .lock()
        .expect("chunk mutex")
        .extend_from_slice(sl);
    0
}

extern "C" fn stream_complete_cb(
    bridge_status: c_int,
    http_status: c_int,
    error_message: *mut c_char,
    user_data: *mut c_void,
) {
    assert!(!user_data.is_null());
    let ctx = unsafe { Box::from_raw(user_data.cast::<StreamCbCtx>()) };
    let err = if error_message.is_null() {
        None
    } else {
        let s = cstr_to_string(error_message);
        unsafe { openapp_sdk_core_c_bridge::openapp_sdk_string_free(error_message) };
        Some(s)
    };
    let _ = ctx.done_tx.send((bridge_status, http_status, err));
}

#[tokio::test]
async fn c_bridge_stream_get_delivers_body_via_chunks() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/sse"))
        .respond_with(ResponseTemplate::new(200).set_body_string("event: ping\ndata: hi\n\n"))
        .mount(&server)
        .await;

    let base = server.uri().to_string();
    let base = base.trim_end_matches('/');
    let token = format!("{base}_openapp_testsecret");

    let (collected, done) = tokio::task::spawn_blocking(move || unsafe {
        openapp_sdk_core_c_bridge::openapp_sdk_telemetry_init();

        let chunks = Arc::new(Mutex::new(Vec::new()));
        let (done_tx, done_rx) = mpsc::channel::<(i32, i32, Option<String>)>();
        let ctx = Box::new(StreamCbCtx {
            chunks: Arc::clone(&chunks),
            done_tx,
        });
        let user_data = Box::into_raw(ctx).cast::<c_void>();

        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null());

        let mut err: *mut c_char = ptr::null_mut();
        let token_c = CString::new(token).expect("token cstring");
        let client =
            openapp_sdk_core_c_bridge::openapp_sdk_client_new(runtime, token_c.as_ptr(), &mut err);
        assert!(!client.is_null(), "{}", cstr_to_string(err));
        free_cstr(err);

        let method_c = CString::new("GET").unwrap();
        let path_c = CString::new("/sse").unwrap();
        let schedule = openapp_sdk_core_c_bridge::openapp_sdk_client_request_stream_async(
            client,
            method_c.as_ptr(),
            path_c.as_ptr(),
            ptr::null(),
            Some(stream_chunk_cb),
            Some(stream_complete_cb),
            user_data,
        );
        assert_eq!(schedule, BridgeStatus::Ok);

        let summary = done_rx
            .recv_timeout(Duration::from_secs(10))
            .expect("stream completion");

        let buf = chunks.lock().expect("chunks").clone();

        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);

        (buf, summary)
    })
    .await
    .expect("spawn_blocking");

    let body = String::from_utf8(collected).expect("utf-8 stream");
    assert!(
        body.contains("event: ping") && body.contains("data: hi"),
        "unexpected stream body: {body:?}"
    );

    assert_eq!(done.0, BridgeStatus::Ok as i32);
    assert_eq!(done.1, 200);
    assert!(done.2.is_none(), "unexpected error: {:?}", done.2);
}

#[tokio::test]
async fn c_bridge_raw_post_returns_json() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/upload"))
        .and(header("Content-Type", "application/octet-stream"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "raw": true
        })))
        .mount(&server)
        .await;

    let base = server.uri().to_string();
    let base = base.trim_end_matches('/');
    let token = format!("{base}_openapp_testsecret");

    let body_text = tokio::task::spawn_blocking(move || unsafe {
        let runtime = openapp_sdk_core_c_bridge::openapp_sdk_runtime_new();
        assert!(!runtime.is_null());

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

        let method_c = CString::new("POST").unwrap();
        let path_c = CString::new("/upload").unwrap();
        let ct_c = CString::new("application/octet-stream").unwrap();
        let body: &[u8] = b"plain-bytes";
        let mut out_body: *mut c_char = ptr::null_mut();
        let mut http_status: c_int = 0;
        let code = openapp_sdk_core_c_bridge::openapp_sdk_client_request_raw(
            client,
            method_c.as_ptr(),
            path_c.as_ptr(),
            ct_c.as_ptr(),
            body.as_ptr(),
            body.len(),
            &mut out_body,
            &mut http_status,
        );
        assert_eq!(code, BridgeStatus::Ok);
        assert_eq!(http_status, 200);
        let s = cstr_to_string(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_string_free(out_body);
        openapp_sdk_core_c_bridge::openapp_sdk_client_free(client);
        openapp_sdk_core_c_bridge::openapp_sdk_runtime_free(runtime);
        s
    })
    .await
    .expect("spawn_blocking");

    let v: serde_json::Value = serde_json::from_str(&body_text).expect("json");
    assert_eq!(v["raw"], true);
}
