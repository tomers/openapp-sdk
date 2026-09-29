//! C exports for localized name resolution (`get_by_name`).

use std::{ffi::CStr, future::Future, os::raw::c_int, sync::Arc};

use openapp_sdk_common::{name_match_from_i32, resolve_unique};
use openapp_sdk_core::{SdkError, resolve::resolve_error_to_sdk};
use serde_json::Value;
use tokio::runtime::Runtime;

use crate::{
    BridgeClient, BridgeStatus, OpenAppSdkRequestComplete, set_out_err, stringify_bridge_error,
};

fn read_cstr<'a>(ptr: *const std::os::raw::c_char) -> Result<&'a str, BridgeStatus> {
    if ptr.is_null() {
        return Err(BridgeStatus::InvalidArgument);
    }
    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .map_err(|_| BridgeStatus::InvalidArgument)
    }
}

fn optional_cstr(ptr: *const std::os::raw::c_char) -> Result<Option<String>, BridgeStatus> {
    if ptr.is_null() {
        return Ok(None);
    }
    Ok(Some(read_cstr(ptr)?.to_owned()))
}

fn complete_value_result(
    complete_fn: OpenAppSdkRequestComplete,
    user_data: *mut std::ffi::c_void,
    result: Result<Value, SdkError>,
) {
    match result {
        Ok(value) => {
            let text = serde_json::to_string(&value).unwrap_or_default();
            let cstr = std::ffi::CString::new(text)
                .unwrap_or_else(|_| std::ffi::CString::new("{}").unwrap());
            let ptr = cstr.into_raw();
            unsafe {
                complete_fn(BridgeStatus::Ok as c_int, 200, ptr, user_data);
            }
        }
        Err(ref err) => {
            let http_st = err.status().map(c_int::from).unwrap_or(0);
            let status = BridgeStatus::from(err);
            let mut out: *mut std::os::raw::c_char = std::ptr::null_mut();
            set_out_err(&mut out, &stringify_bridge_error(err));
            unsafe {
                complete_fn(status as c_int, http_st, out, user_data);
            }
        }
    }
}

fn schedule_client_future<F, Fut>(
    client: &BridgeClient,
    complete: OpenAppSdkRequestComplete,
    user_data: *mut std::ffi::c_void,
    op: F,
) -> BridgeStatus
where
    F: FnOnce(openapp_sdk_core::Client) -> Fut + Send + 'static,
    Fut: Future<Output = Result<Value, SdkError>> + Send + 'static,
{
    let client_inner = client.client.clone();
    let runtime: Arc<Runtime> = client.runtime.clone();
    let user_data_bits = user_data as usize;
    runtime.spawn(async move {
        let result = op(client_inner).await;
        let user_data = user_data_bits as *mut std::ffi::c_void;
        complete_value_result(complete, user_data, result);
    });
    BridgeStatus::Ok
}

/// Stateless localized-name resolution over a JSON array of resources.
///
/// # Safety
/// Pointers must be valid UTF-8 C strings; `out_ok` and `out_err` must be non-null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_resolve_unique_json(
    items_json_utf8: *const std::os::raw::c_char,
    name_field_utf8: *const std::os::raw::c_char,
    needle_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    out_ok: *mut *mut std::os::raw::c_char,
    out_err: *mut *mut std::os::raw::c_char,
) -> BridgeStatus {
    if out_ok.is_null() || out_err.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let items_str = match read_cstr(items_json_utf8) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let name_field = match read_cstr(name_field_utf8) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let needle = match read_cstr(needle_utf8) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let items: Vec<Value> = match serde_json::from_str(items_str) {
        Ok(v) => v,
        Err(e) => {
            set_out_err(out_err, &format!("invalid items JSON: {e}"));
            return BridgeStatus::InvalidArgument;
        }
    };
    let mode = name_match_from_i32(match_mode);
    match resolve_unique(&items, "resource", name_field, needle, mode) {
        Ok(value) => {
            set_out_err(out_ok, &serde_json::to_string(&value).unwrap_or_default());
            BridgeStatus::Ok
        }
        Err(err) => {
            let sdk = resolve_error_to_sdk(err, "resource");
            set_out_err(out_err, &stringify_bridge_error(&sdk));
            BridgeStatus::from(&sdk)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_orgs_get_by_name_async(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut std::ffi::c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || name_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    schedule_client_future(client, complete_fn, user_data, move |core| async move {
        core.orgs().get_by_name(&name, mode).await
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_integrations_get_by_name_async(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut std::ffi::c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || name_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    schedule_client_future(client, complete_fn, user_data, move |core| async move {
        core.integrations().get_by_name(&name, mode, None).await
    })
}

/// Like [`openapp_sdk_client_integrations_get_by_name_async`] with optional `provider_type_utf8`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_integrations_get_by_name_with_provider_async(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    provider_type_utf8: *const std::os::raw::c_char,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut std::ffi::c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || name_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let provider_type = match optional_cstr(provider_type_utf8) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    schedule_client_future(client, complete_fn, user_data, move |core| async move {
        core.integrations()
            .get_by_name(&name, mode, provider_type.as_deref())
            .await
    })
}

/// Resolve an access portal by name under an integration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_integrations_get_access_portal_by_name_async(
    client: *mut BridgeClient,
    integration_id_utf8: *const std::os::raw::c_char,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut std::ffi::c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || integration_id_utf8.is_null() || name_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let integration_id = match read_cstr(integration_id_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    schedule_client_future(client, complete_fn, user_data, move |core| async move {
        core.integrations()
            .get_access_portal_by_name(&integration_id, &name, mode)
            .await
    })
}

/// Resolve a device by localized name; pass null `integration_id_utf8` to search all devices.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_devices_get_by_name_async(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    integration_id_utf8: *const std::os::raw::c_char,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut std::ffi::c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || name_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let integration_id = match optional_cstr(integration_id_utf8) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    schedule_client_future(client, complete_fn, user_data, move |core| async move {
        core.devices()
            .get_by_name(&name, mode, integration_id.as_deref())
            .await
    })
}

/// Resolve a zone by localized name under an integration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_zones_get_by_name_async(
    client: *mut BridgeClient,
    integration_id_utf8: *const std::os::raw::c_char,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    complete: Option<OpenAppSdkRequestComplete>,
    user_data: *mut std::ffi::c_void,
) -> BridgeStatus {
    let Some(complete_fn) = complete else {
        return BridgeStatus::InvalidArgument;
    };
    if client.is_null() || integration_id_utf8.is_null() || name_utf8.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let integration_id = match read_cstr(integration_id_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    schedule_client_future(client, complete_fn, user_data, move |core| async move {
        core.zones().get_by_name(&integration_id, &name, mode).await
    })
}

fn block_on_get_by_name<F, Fut>(
    client: &BridgeClient,
    out_body: *mut *mut std::os::raw::c_char,
    out_status: *mut c_int,
    op: F,
) -> BridgeStatus
where
    F: FnOnce(openapp_sdk_core::Client) -> Fut,
    Fut: Future<Output = Result<Value, SdkError>>,
{
    let result = client.runtime.block_on(op(client.client.clone()));
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

/// Blocking resolve — same semantics as the async variant; for koffi/cgo sync bindings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_integrations_get_by_name(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    out_body: *mut *mut std::os::raw::c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null() || name_utf8.is_null() || out_body.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    block_on_get_by_name(client, out_body, out_status, move |core| async move {
        core.integrations().get_by_name(&name, mode, None).await
    })
}

/// Blocking resolve for access portals under an integration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_integrations_get_access_portal_by_name(
    client: *mut BridgeClient,
    integration_id_utf8: *const std::os::raw::c_char,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    out_body: *mut *mut std::os::raw::c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null()
        || integration_id_utf8.is_null()
        || name_utf8.is_null()
        || out_body.is_null()
    {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let integration_id = match read_cstr(integration_id_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    block_on_get_by_name(client, out_body, out_status, move |core| async move {
        core.integrations()
            .get_access_portal_by_name(&integration_id, &name, mode)
            .await
    })
}

/// Blocking resolve for devices; pass null `integration_id_utf8` to search all devices.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_devices_get_by_name(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    integration_id_utf8: *const std::os::raw::c_char,
    out_body: *mut *mut std::os::raw::c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null() || name_utf8.is_null() || out_body.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let integration_id = match optional_cstr(integration_id_utf8) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    block_on_get_by_name(client, out_body, out_status, move |core| async move {
        core.devices()
            .get_by_name(&name, mode, integration_id.as_deref())
            .await
    })
}

/// Blocking resolve for zones under an integration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_zones_get_by_name(
    client: *mut BridgeClient,
    integration_id_utf8: *const std::os::raw::c_char,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    out_body: *mut *mut std::os::raw::c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null()
        || integration_id_utf8.is_null()
        || name_utf8.is_null()
        || out_body.is_null()
    {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let integration_id = match read_cstr(integration_id_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    block_on_get_by_name(client, out_body, out_status, move |core| async move {
        core.zones().get_by_name(&integration_id, &name, mode).await
    })
}

/// Blocking resolve for organizations by localized name.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn openapp_sdk_client_orgs_get_by_name(
    client: *mut BridgeClient,
    name_utf8: *const std::os::raw::c_char,
    match_mode: c_int,
    out_body: *mut *mut std::os::raw::c_char,
    out_status: *mut c_int,
) -> BridgeStatus {
    if client.is_null() || name_utf8.is_null() || out_body.is_null() {
        return BridgeStatus::InvalidArgument;
    }
    let client = unsafe { &*client };
    let name = match read_cstr(name_utf8) {
        Ok(s) => s.to_owned(),
        Err(e) => return e,
    };
    let mode = name_match_from_i32(match_mode);
    block_on_get_by_name(client, out_body, out_status, move |core| async move {
        core.orgs().get_by_name(&name, mode).await
    })
}
