//! Helpers for resolving resources by localized display name.

use std::future::Future;

use openapp_sdk_common::{NameMatch, ResolveError, resolve_unique};
use serde_json::Value;

use crate::error::SdkError;

/// Convert a [`ResolveError`] into the public [`SdkError`] surface.
#[must_use]
pub fn resolve_error_to_sdk(err: ResolveError, resource_type: &'static str) -> SdkError {
    match err {
        ResolveError::NotFound { name, .. } => SdkError::ResourceNotFound {
            resource_type,
            name,
        },
        ResolveError::Ambiguous {
            name,
            matches,
            count,
            ..
        } => SdkError::AmbiguousResource {
            resource_type,
            name,
            matches,
            match_count: count,
        },
    }
}

/// Run [`resolve_unique`] and map errors to [`SdkError`].
pub fn resolve_unique_sdk(
    items: &[Value],
    resource_type: &'static str,
    name_field: &str,
    needle: &str,
    mode: NameMatch,
) -> Result<Value, SdkError> {
    resolve_unique(items, resource_type, name_field, needle, mode)
        .map_err(|e| resolve_error_to_sdk(e, resource_type))
}

/// Extract `items` array from a paginated list response `{ items, total }`.
#[must_use]
pub fn paginated_items(response: &Value) -> Vec<Value> {
    response
        .get("items")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

/// Normalize list responses: bare array, paginated `{ items }`, or `{ portals }` etc.
#[must_use]
pub fn items_from_list_response(response: &Value, nested_key: Option<&str>) -> Vec<Value> {
    if let Some(arr) = response.as_array() {
        return arr.clone();
    }
    if let Some(key) = nested_key
        && let Some(arr) = response.get(key).and_then(|v| v.as_array())
    {
        return arr.clone();
    }
    paginated_items(response)
}

/// Whether to pass `q` to list endpoints as a server-side prefilter.
#[must_use]
pub fn use_server_q_prefilter(_mode: NameMatch, needle: &str) -> bool {
    !needle.trim().is_empty()
}

const DEFAULT_PAGE_SIZE: u32 = 200;

/// Fetch every page from a paginated list endpoint.
pub async fn fetch_all_paginated<F, Fut>(mut fetch_page: F) -> Result<Vec<Value>, SdkError>
where
    F: FnMut(u32, u32) -> Fut,
    Fut: Future<Output = Result<Value, SdkError>>,
{
    let mut offset = 0u32;
    let mut all = Vec::new();
    loop {
        let page = fetch_page(DEFAULT_PAGE_SIZE, offset).await?;
        let batch = paginated_items(&page);
        let batch_len = batch.len();
        all.extend(batch);
        let total = page
            .get("total")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(u64::try_from(all.len()).unwrap_or(0));
        if all.len() as u64 >= total || batch_len == 0 {
            break;
        }
        offset += DEFAULT_PAGE_SIZE;
    }
    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn paginated_items_extracts_array() {
        let page = json!({"items": [{"id": "1"}], "total": 1});
        assert_eq!(paginated_items(&page).len(), 1);
    }

    #[test]
    fn items_from_list_response_portals() {
        let wrapped = json!({"portals": [{"id": "p1"}]});
        assert_eq!(items_from_list_response(&wrapped, Some("portals")).len(), 1);
    }
}
