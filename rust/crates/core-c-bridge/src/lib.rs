//! C ABI bridge around [`openapp_sdk_core`].
//!
//! ## Status: skeleton
//!
//! This crate is intentionally minimal in v1. It exists so the *architecture* is in
//! place — every future non-PyO3 language SDK (.NET, Ruby, Go, Swift) can link against
//! a single shared library produced here.
//!
//! The v1 surface covers:
//!
//! * a single shared tokio runtime ([`openapp_sdk_runtime_new`] /
//!   [`openapp_sdk_runtime_free`]);
//! * a [`Client`](openapp_sdk_core::Client) handle registry
//!   ([`openapp_sdk_client_new`] / [`openapp_sdk_client_free`]);
//! * a synchronous `perform_request` call for proof-of-concept
//!   ([`openapp_sdk_client_request`]).
//!
//! The full, callback-based async API is tracked in
//! [`ARCHITECTURE.md`](../ARCHITECTURE.md) as a follow-up.

#![deny(rust_2018_idioms)]
#![allow(clippy::missing_safety_doc)]

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
    sync::Arc,
};

use openapp_sdk_core::{Client, SdkError, transport::RequestSpec};
use parking_lot::Mutex;
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
#[derive(Debug, Clone, Copy)]
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

/// Free a client handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_free(ptr: *mut BridgeClient) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
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
    let method_str = match unsafe { CStr::from_ptr(method_utf8) }.to_str() {
        Ok(s) => s,
        Err(_) => return BridgeStatus::InvalidArgument,
    };
    let path_str = match unsafe { CStr::from_ptr(path_utf8) }.to_str() {
        Ok(s) => s,
        Err(_) => return BridgeStatus::InvalidArgument,
    };
    let body_opt = if body_json_utf8.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(body_json_utf8) }.to_str() {
            Ok(s) => Some(s.to_owned()),
            Err(_) => return BridgeStatus::InvalidArgument,
        }
    };
    let method = match Method::from_bytes(method_str.as_bytes()) {
        Ok(m) => m,
        Err(_) => return BridgeStatus::InvalidArgument,
    };

    let transport = client.client.transport();
    let result = client.runtime.block_on(async move {
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
                path: path_str,
                body: body_value.as_ref(),
                ..Default::default()
            })
            .await
    });

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
            set_out_err(out_body, &err.to_string());
            BridgeStatus::from(err)
        }
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

// Keep the registry lock compiled in for future async completion support, even though
// v1 runs `block_on` synchronously. This keeps the ABI stable as we add completions.
#[allow(dead_code)]
static HANDLES: once_cell::sync::Lazy<Mutex<Vec<()>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));
