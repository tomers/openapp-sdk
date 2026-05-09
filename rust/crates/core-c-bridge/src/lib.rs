//! C ABI around [`openapp_sdk_core`] for language runtimes that link this shared library
//! instead of using PyO3 (for example Node via native addons, Go via cgo).
//!
//! ## Surface
//!
//! * Tokio runtime: [`openapp_sdk_runtime_new`], [`openapp_sdk_runtime_free`].
//! * Client handles: [`openapp_sdk_client_new`], [`openapp_sdk_client_new_with_config`],
//!   [`openapp_sdk_client_free`].
//! * JSON over HTTP — blocking: [`openapp_sdk_client_request`].
//! * JSON over HTTP — async completion callback: [`openapp_sdk_client_request_async`].
//! * Byte streaming (e.g. `text/event-stream`): [`openapp_sdk_client_request_stream_async`],
//!   [`openapp_sdk_client_request_stream_async_with_headers`] (HTTP status before chunks).
//! * Opaque request body + Content-Type (e.g. multipart): [`openapp_sdk_client_request_raw`].
//! * Tracing subscriber bootstrap: [`openapp_sdk_telemetry_init`].
//! * Strings returned from the bridge: free with [`openapp_sdk_string_free`].
//!
//! Design constraints and layering are documented in [`ARCHITECTURE.md`].
//!
//! [`ARCHITECTURE.md`]: ../ARCHITECTURE.md

#![deny(rust_2018_idioms)]
#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{CStr, CString, c_void},
    os::raw::{c_char, c_int},
    sync::Arc,
    time::Duration,
};

use futures::StreamExt;
use openapp_sdk_core::{
    Client, SdkError,
    retry::RetryPolicy,
    telemetry,
    transport::{RequestSpec, Transport},
};
use reqwest::Method;
use tokio::runtime::Runtime;

/// Opaque runtime handle. One per process is recommended.
pub struct BridgeRuntime {
    runtime: Arc<Runtime>,
}

/// Opaque client handle.
pub struct BridgeClient {
    client: Client,
    runtime: Arc<Runtime>,
}

/// Error codes returned by bridge calls.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeStatus {
    Ok = 0,
    InvalidArgument = 1,
    ConfigError = 2,
    TransportError = 3,
    AuthError = 4,
    ApiError = 5,
    InternalError = 99,
}

impl From<&SdkError> for BridgeStatus {
    fn from(err: &SdkError) -> Self {
        match err {
            SdkError::Config(_) => Self::ConfigError,
            SdkError::Auth(_) | SdkError::Token(_) => Self::AuthError,
            SdkError::Api { .. } => Self::ApiError,
            SdkError::Transport(_) => Self::TransportError,
            _ => Self::InternalError,
        }
    }
}

/// C callback invoked exactly once when [`openapp_sdk_client_request_async`] finishes.
/// `body` is allocated by the bridge; free with [`openapp_sdk_string_free`] when non-null.
/// Invoked on a runtime worker thread.
pub type OpenAppSdkRequestComplete = unsafe extern "C" fn(
    bridge_status: c_int,
    http_status: c_int,
    body: *mut c_char,
    user_data: *mut c_void,
);

/// Per-chunk callback for [`openapp_sdk_client_request_stream_async`]. `data` is valid only
/// for the duration of the call; copy if needed. Return `0` to continue, non-zero to stop
/// reading (completion still runs with [`BridgeStatus::Ok`]).
pub type OpenAppSdkStreamChunk =
    unsafe extern "C" fn(data: *const u8, len: usize, user_data: *mut c_void) -> c_int;

/// Invoked once after the last chunk or when the stream fails. `error_message` is
/// bridge-allocated on failure; free with [`openapp_sdk_string_free`] when non-null.
pub type OpenAppSdkStreamComplete = unsafe extern "C" fn(
    bridge_status: c_int,
    http_status: c_int,
    error_message: *mut c_char,
    user_data: *mut c_void,
);

/// Invoked once when the HTTP response status is known, before any chunk callback.
/// Optional for [`openapp_sdk_client_request_stream_async_with_headers`].
pub type OpenAppSdkStreamResponseHeaders =
    unsafe extern "C" fn(http_status: c_int, user_data: *mut c_void);

/// Use with [`OpenAppSdkClientConfig::max_retries`] to keep the default retry policy from
/// [`openapp_sdk_core`].
pub const OPENAPP_SDK_USE_DEFAULT_MAX_RETRIES: u32 = u32::MAX;

/// Optional parameters for [`openapp_sdk_client_new_with_config`].
///
/// * `api_key` — required non-null UTF-8 API key.
/// * `base_url` — optional; when non-null, overrides the URL parsed from the key.
/// * `user_agent` — optional; when non-null, sets the HTTP `User-Agent` header.
/// * `default_timeout_secs` — `0` keeps the core default (30 seconds).
/// * `max_retries` — [`OPENAPP_SDK_USE_DEFAULT_MAX_RETRIES`] keeps the core default;
///   otherwise [`RetryPolicy::max_retries`] is set on a copy of the default policy.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OpenAppSdkClientConfig {
    pub api_key: *const c_char,
    pub base_url: *const c_char,
    pub user_agent: *const c_char,
    pub default_timeout_secs: u64,
    pub max_retries: u32,
}

/// Install the default `tracing` / `OPENAPP_SDK_LOG` subscriber. Safe to call multiple
/// times (subsequent calls are ignored). Matches [`telemetry::init_fallback`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_telemetry_init() {
    telemetry::init_fallback();
}

/// Create a new shared runtime. Must be freed with [`openapp_sdk_runtime_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_runtime_new() -> *mut BridgeRuntime {
    match Runtime::new() {
        Ok(runtime) => Box::into_raw(Box::new(BridgeRuntime {
            runtime: Arc::new(runtime),
        })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a runtime handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_runtime_free(ptr: *mut BridgeRuntime) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

/// Create a client from an API key token. On failure, `out_err` (if non-null) receives
/// a freshly-allocated C string describing the failure; free it with
/// [`openapp_sdk_string_free`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_new(
    runtime: *mut BridgeRuntime,
    api_key: *const c_char,
    out_err: *mut *mut c_char,
) -> *mut BridgeClient {
    if runtime.is_null() || api_key.is_null() {
        set_out_err(out_err, "null argument to openapp_sdk_client_new");
        return std::ptr::null_mut();
    }

    let runtime_for_client = unsafe { (*runtime).runtime.clone() };

    let api_key_str = match unsafe { CStr::from_ptr(api_key) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_out_err(out_err, "api_key is not valid utf-8");
            return std::ptr::null_mut();
        }
    };

    let client = match Client::builder().api_key(api_key_str).build() {
        Ok(c) => c,
        Err(err) => {
            set_out_err(out_err, &err.to_string());
            return std::ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(BridgeClient {
        client,
        runtime: runtime_for_client,
    }))
}

/// Same as [`openapp_sdk_client_new`], but accepts optional base URL, user agent, timeout,
/// and retry overrides. On failure, `out_err` receives an allocated message when non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_new_with_config(
    runtime: *mut BridgeRuntime,
    config: *const OpenAppSdkClientConfig,
    out_err: *mut *mut c_char,
) -> *mut BridgeClient {
    if runtime.is_null() || config.is_null() {
        set_out_err(
            out_err,
            "null argument to openapp_sdk_client_new_with_config",
        );
        return std::ptr::null_mut();
    }

    let runtime_for_client = unsafe { (*runtime).runtime.clone() };

    let cfg = unsafe { &*config };
    if cfg.api_key.is_null() {
        set_out_err(out_err, "OpenAppSdkClientConfig.api_key is null");
        return std::ptr::null_mut();
    }

    let api_key_str = match unsafe { CStr::from_ptr(cfg.api_key) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_out_err(out_err, "api_key is not valid utf-8");
            return std::ptr::null_mut();
        }
    };

    let mut builder = Client::builder().api_key(api_key_str);

    if !cfg.base_url.is_null() {
        let url_str = match unsafe { CStr::from_ptr(cfg.base_url) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_out_err(out_err, "base_url is not valid utf-8");
                return std::ptr::null_mut();
            }
        };
        builder = match builder.base_url(url_str) {
            Ok(b) => b,
            Err(err) => {
                set_out_err(out_err, &err.to_string());
                return std::ptr::null_mut();
            }
        };
    }

    if !cfg.user_agent.is_null() {
        let ua = match unsafe { CStr::from_ptr(cfg.user_agent) }.to_str() {
            Ok(s) => s,
            Err(_) => {
                set_out_err(out_err, "user_agent is not valid utf-8");
                return std::ptr::null_mut();
            }
        };
        builder = builder.user_agent(ua);
    }

    if cfg.default_timeout_secs != 0 {
        builder = builder.default_timeout(Duration::from_secs(cfg.default_timeout_secs));
    }

    if cfg.max_retries != OPENAPP_SDK_USE_DEFAULT_MAX_RETRIES {
        let policy = RetryPolicy {
            max_retries: cfg.max_retries,
            ..RetryPolicy::default()
        };
        builder = builder.retry_policy(policy);
    }

    let client = match builder.build() {
        Ok(c) => c,
        Err(err) => {
            set_out_err(out_err, &err.to_string());
            return std::ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(BridgeClient {
        client,
        runtime: runtime_for_client,
    }))
}

/// Free a client handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_free(ptr: *mut BridgeClient) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

/// # Safety
/// `method_utf8` and `path_utf8` must be non-null.
unsafe fn parse_request_inputs(
    method_utf8: *const c_char,
    path_utf8: *const c_char,
    body_json_utf8: *const c_char,
) -> Result<(Method, String, Option<String>), BridgeStatus> {
    let method_str = match unsafe { CStr::from_ptr(method_utf8) }.to_str() {
        Ok(s) => s,
        Err(_) => return Err(BridgeStatus::InvalidArgument),
    };
    let path_str = match unsafe { CStr::from_ptr(path_utf8) }.to_str() {
        Ok(s) => s,
        Err(_) => return Err(BridgeStatus::InvalidArgument),
    };
    let body_opt = if body_json_utf8.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(body_json_utf8) }.to_str() {
            Ok(s) => Some(s.to_owned()),
            Err(_) => return Err(BridgeStatus::InvalidArgument),
        }
    };
    let method = match Method::from_bytes(method_str.as_bytes()) {
        Ok(m) => m,
        Err(_) => return Err(BridgeStatus::InvalidArgument),
    };
    Ok((method, path_str.to_owned(), body_opt))
}

async fn transport_json_request(
    transport: Arc<Transport>,
    method: Method,
    path: String,
    body_opt: Option<String>,
) -> Result<serde_json::Value, SdkError> {
    let body_value = match body_opt {
        Some(s) if !s.is_empty() => Some(
            serde_json::from_str::<serde_json::Value>(&s)
                .map_err(|e| SdkError::Serialize(e.to_string()))?,
        ),
        _ => None,
    };

    transport
        .request_json::<serde_json::Value, serde_json::Value>(RequestSpec {
            method,
            path: path.as_str(),
            body: body_value.as_ref(),
            ..Default::default()
        })
        .await
}

async fn transport_raw_request(
    transport: Arc<Transport>,
    method: Method,
    path: String,
    body: Vec<u8>,
    content_type: String,
) -> Result<serde_json::Value, SdkError> {
    transport
        .request_json_raw_body(
            method,
            path.as_str(),
            &[],
            body,
            content_type.as_str(),
            &[],
            None,
        )
        .await
}

/// Perform a JSON request. Returns `BridgeStatus::Ok` and writes the (freshly
/// allocated, nul-terminated) JSON body into `out_body`; caller frees with
/// [`openapp_sdk_string_free`]. On failure, returns a non-`Ok` status and writes an
/// error message into `out_body` instead.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_request(
    client: *mut BridgeClient,
    method_utf8: *const c_char,
    path_utf8: *const c_char,
    body_json_utf8: *const c_char,
    out_body: *mut *mut c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null() || method_utf8.is_null() || path_utf8.is_null() {
        set_out_err(out_body, "null argument to openapp_sdk_client_request");
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let (method, path, body_opt) =
        match unsafe { parse_request_inputs(method_utf8, path_utf8, body_json_utf8) } {
            Ok(x) => x,
            Err(e) => return e,
        };

    let transport = client.client.transport();
    let result = client
        .runtime
        .block_on(transport_json_request(transport, method, path, body_opt));

    match result {
        Ok(value) => {
            if !out_status.is_null() {
                unsafe { *out_status = 200 };
            }
            set_out_err(out_body, &serde_json::to_string(&value).unwrap_or_default());
            BridgeStatus::Ok
        }
        Err(ref err) => {
            if !out_status.is_null() {
                unsafe { *out_status = err.status().map(c_int::from).unwrap_or(0) };
            }
            set_out_err(out_body, &stringify_bridge_error(err));
            BridgeStatus::from(err)
        }
    }
}

/// Opaque request body (for example `multipart/form-data` with boundary in `content_type`)
/// and JSON response. `body` must point to `body_len` bytes; `content_type` is full
/// header value UTF-8 (e.g. including `boundary=...`).
///
/// # Safety
/// See [`openapp_sdk_client_request`]. `body` must be valid for `body_len` when non-zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_request_raw(
    client: *mut BridgeClient,
    method_utf8: *const c_char,
    path_utf8: *const c_char,
    content_type_utf8: *const c_char,
    body: *const u8,
    body_len: usize,
    out_body: *mut *mut c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null()
        || method_utf8.is_null()
        || path_utf8.is_null()
        || content_type_utf8.is_null()
        || out_body.is_null()
    {
        set_out_err(out_body, "null argument to openapp_sdk_client_request_raw");
        return BridgeStatus::InvalidArgument;
    }
    if body_len == 0 {
        set_out_err(
            out_body,
            "openapp_sdk_client_request_raw requires a non-empty body",
        );
        return BridgeStatus::InvalidArgument;
    }
    if body.is_null() {
        set_out_err(out_body, "null body with non-zero length");
        return BridgeStatus::InvalidArgument;
    }

    let client = unsafe { &*client };
    let (method, path, _) =
        match unsafe { parse_request_inputs(method_utf8, path_utf8, std::ptr::null()) } {
            Ok(x) => x,
            Err(e) => return e,
        };

    let content_type = match unsafe { CStr::from_ptr(content_type_utf8) }.to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => {
            set_out_err(out_body, "content_type is not valid utf-8");
            return BridgeStatus::InvalidArgument;
        }
    };

    let body_vec = unsafe { std::slice::from_raw_parts(body, body_len) }.to_vec();

    let transport = client.client.transport();
    let result = client.runtime.block_on(transport_raw_request(
        transport,
        method,
        path,
        body_vec,
        content_type,
    ));

    match result {
        Ok(value) => {
            if !out_status.is_null() {
                unsafe { *out_status = 200 };
            }
            set_out_err(out_body, &serde_json::to_string(&value).unwrap_or_default());
            BridgeStatus::Ok
        }
        Err(ref err) => {
            if !out_status.is_null() {
                unsafe { *out_status = err.status().map(c_int::from).unwrap_or(0) };
            }
            set_out_err(out_body, &stringify_bridge_error(err));
            BridgeStatus::from(err)
        }
    }
}

/// Schedule a JSON request and invoke `complete` when it finishes. Returns
/// [`BridgeStatus::Ok`] if the work was **queued**; the callback runs later. The caller
/// must keep the [`BridgeClient`] alive until `complete` runs.
///
/// # Safety
/// `client` must be a valid client handle. `complete` must be a valid function pointer.
/// `user_data` is passed through to `complete` unchanged.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_request_async(
    client: *mut BridgeClient,
    method_utf8: *const c_char,
    path_utf8: *const c_char,
    body_json_utf8: *const c_char,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || method_utf8.is_null() || path_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let (method, path, body_opt) =
        match unsafe { parse_request_inputs(method_utf8, path_utf8, body_json_utf8) } {
            Ok(x) => x,
            Err(e) => return e,
        };

    let client_inner = client.client.clone();
    let runtime = client.runtime.clone();
    // `usize` is `Send`; raw pointers are not, and must not be held across `.await`.
    let user_data_bits = user_data as usize;
    runtime.spawn(async move {
        let transport = client_inner.transport();
        let result = transport_json_request(transport, method, path, body_opt).await;
        let user_data = user_data_bits as *mut c_void;
        match result {
            Ok(value) => {
                let text = serde_json::to_string(&value).unwrap_or_default();
                let cstr = match CString::new(text) {
                    Ok(s) => s,
                    Err(_) => CString::new("{}").expect("static"),
                };
                let ptr = cstr.into_raw();
                unsafe {
                    complete_fn(BridgeStatus::Ok as c_int, 200, ptr, user_data);
                }
            }
            Err(ref err) => {
                let http_st = err.status().map(c_int::from).unwrap_or(0);
                let status = BridgeStatus::from(err);
                let mut out: *mut c_char = std::ptr::null_mut();
                set_out_err(&mut out, &stringify_bridge_error(err));
                unsafe {
                    complete_fn(status as c_int, http_st, out, user_data);
                }
            }
        }
    });

    BridgeStatus::Ok
}

/// Like [`openapp_sdk_client_request_stream_async`], but invokes `response_headers`
/// once with the HTTP status after the response line is known and before any `chunk`
/// call. Pass `None` for `response_headers` to match the legacy behavior.
///
/// # Safety
/// Same as [`openapp_sdk_client_request_async`]: valid client, callbacks, and `user_data`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_request_stream_async_with_headers(
    client: *mut BridgeClient,
    method_utf8: *const c_char,
    path_utf8: *const c_char,
    body_json_utf8: *const c_char,
    chunk: Option<OpenAppSdkStreamChunk>,
    complete: Option<OpenAppSdkStreamComplete>,
    response_headers: Option<OpenAppSdkStreamResponseHeaders>,
    user_data: *mut c_void,
) -> BridgeStatus {
    let Some(chunk_fn) = chunk else {
        return BridgeStatus::InvalidArgument;
    };
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || method_utf8.is_null() || path_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let (method, path, body_opt) =
        match unsafe { parse_request_inputs(method_utf8, path_utf8, body_json_utf8) } {
            Ok(x) => x,
            Err(e) => return e,
        };

    let client_inner = client.client.clone();
    let runtime = client.runtime.clone();
    let user_data_bits = user_data as usize;

    runtime.spawn(async move {
        let body_value = match body_opt {
            Some(s) if !s.is_empty() => match serde_json::from_str::<serde_json::Value>(&s) {
                Ok(v) => Some(v),
                Err(e) => {
                    let mut out: *mut c_char = std::ptr::null_mut();
                    set_out_err(&mut out, &e.to_string());
                    unsafe {
                        complete_fn(
                            BridgeStatus::InvalidArgument as c_int,
                            0,
                            out,
                            user_data_bits as *mut c_void,
                        );
                    }
                    return;
                }
            },
            _ => None,
        };

        let transport = client_inner.transport();
        let stream_result = transport
            .request_stream(RequestSpec {
                method,
                path: path.as_str(),
                body: body_value.as_ref(),
                ..Default::default()
            })
            .await;

        match stream_result {
            Err(err) => {
                let http_st = err.status().map(c_int::from).unwrap_or(0);
                let status = BridgeStatus::from(&err);
                let mut out: *mut c_char = std::ptr::null_mut();
                set_out_err(&mut out, &stringify_bridge_error(&err));
                unsafe {
                    complete_fn(status as c_int, http_st, out, user_data_bits as *mut c_void);
                }
            }
            Ok((http_status, mut stream)) => {
                let success_http = c_int::from(http_status.as_u16());
                if let Some(hdr_cb) = response_headers {
                    unsafe {
                        hdr_cb(success_http, user_data_bits as *mut c_void);
                    }
                }
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(bytes) => {
                            if bytes.is_empty() {
                                continue;
                            }
                            let keep_going = unsafe {
                                chunk_fn(bytes.as_ptr(), bytes.len(), user_data_bits as *mut c_void)
                            };
                            if keep_going != 0 {
                                break;
                            }
                        }
                        Err(err) => {
                            let http_st = err.status().map(c_int::from).unwrap_or(0);
                            let status = BridgeStatus::from(&err);
                            let mut out: *mut c_char = std::ptr::null_mut();
                            set_out_err(&mut out, &stringify_bridge_error(&err));
                            unsafe {
                                complete_fn(
                                    status as c_int,
                                    http_st,
                                    out,
                                    user_data_bits as *mut c_void,
                                );
                            }
                            return;
                        }
                    }
                }
                unsafe {
                    complete_fn(
                        BridgeStatus::Ok as c_int,
                        success_http,
                        std::ptr::null_mut(),
                        user_data_bits as *mut c_void,
                    );
                }
            }
        }
    });

    BridgeStatus::Ok
}

/// Schedule a streaming request (same routing as [`openapp_sdk_client_request_async`]).
/// Response bytes are delivered to `chunk`; `complete` runs once at end-of-stream,
/// on chunk-level failure, or when `chunk` returns non-zero (early cancel — still
/// [`BridgeStatus::Ok`] with a null error message).
///
/// # Safety
/// Same as [`openapp_sdk_client_request_async`]: valid client, callbacks, and `user_data`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_request_stream_async(
    client: *mut BridgeClient,
    method_utf8: *const c_char,
    path_utf8: *const c_char,
    body_json_utf8: *const c_char,
    chunk: Option<OpenAppSdkStreamChunk>,
    complete: Option<OpenAppSdkStreamComplete>,
    user_data: *mut c_void,
) -> BridgeStatus {
    unsafe {
        openapp_sdk_client_request_stream_async_with_headers(
            client,
            method_utf8,
            path_utf8,
            body_json_utf8,
            chunk,
            complete,
            None,
            user_data,
        )
    }
}

/// Free a C string previously handed back by any bridge call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}

fn set_out_err(out: *mut *mut c_char, msg: &str) {
    if out.is_null() {
        return;
    }
    let cstr = match CString::new(msg) {
        Ok(s) => s,
        Err(_) => CString::new("<unrepresentable error>").unwrap(),
    };
    unsafe {
        *out = cstr.into_raw();
    }
}

/// Wire-shaped JSON for [`SdkError::Api`] so language bindings can recover `code` /
/// `message`; other variants keep [`SdkError::to_string`] for logs.
fn stringify_bridge_error(err: &SdkError) -> String {
    match err {
        SdkError::Api { body, .. } => {
            serde_json::to_string(body).unwrap_or_else(|_| err.to_string())
        }
        _ => err.to_string(),
    }
}
