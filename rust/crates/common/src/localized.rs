//! Localized display-name matching for SDK resource lookup.
//!
//! Dashboard resources store names as plain strings, locale maps, or `OpenAPI`
//! [`LocalizedString`] envelopes (`{ "value": { "en": "…" } }`). Matching scans
//! **all** locale values case-insensitively.

use serde_json::Value;
use thiserror::Error;

/// How a needle is compared against localized name values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NameMatch {
    /// Case-insensitive equality against any locale value.
    #[default]
    Exact,
    /// Case-insensitive substring match against any locale value.
    Fuzzy,
}

/// Outcome of resolving a unique resource by localized name.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ResolveError {
    #[error("no {resource_type} named {name:?}")]
    NotFound { resource_type: String, name: String },
    #[error("{resource_type} name {name:?} is ambiguous ({count} matches)")]
    Ambiguous {
        resource_type: String,
        name: String,
        matches: Vec<Value>,
        count: usize,
    },
}

impl ResolveError {
    #[must_use]
    pub fn ambiguous_count(&self) -> Option<usize> {
        match self {
            Self::Ambiguous { matches, .. } => Some(matches.len()),
            Self::NotFound { .. } => None,
        }
    }
}

/// Parse `match_mode` from C bridge / binding conventions (`0` = exact, `1` = fuzzy).
#[must_use]
pub fn name_match_from_i32(mode: i32) -> NameMatch {
    match mode {
        1 => NameMatch::Fuzzy,
        _ => NameMatch::Exact,
    }
}

/// Collect human-readable strings from a localized name field.
#[must_use]
pub fn localized_values(name_field: &Value) -> Vec<String> {
    match name_field {
        Value::String(s) => vec![s.clone()],
        Value::Object(map) => {
            if let Some(Value::Object(inner)) = map.get("value") {
                return inner
                    .values()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect();
            }
            map.values()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        }
        _ => Vec::new(),
    }
}

fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Returns `true` when `needle` matches any locale value under `name_field`.
#[must_use]
pub fn name_matches(name_field: &Value, needle: &str, mode: NameMatch) -> bool {
    let needle_norm = normalize(needle);
    if needle_norm.is_empty() {
        return false;
    }
    localized_values(name_field).iter().any(|v| match mode {
        NameMatch::Exact => normalize(v) == needle_norm,
        NameMatch::Fuzzy => normalize(v).contains(needle_norm.as_str()),
    })
}

/// Extract the `name_field` from each item and return those that match.
#[must_use]
pub fn filter_by_name(
    items: &[Value],
    name_field: &str,
    needle: &str,
    mode: NameMatch,
) -> Vec<Value> {
    items
        .iter()
        .filter(|item| {
            item.get(name_field)
                .is_some_and(|field| name_matches(field, needle, mode))
        })
        .cloned()
        .collect()
}

/// Resolve a single item by localized name, or return not-found / ambiguous errors.
///
/// # Errors
/// Returns [`ResolveError::NotFound`] when no item matches, or [`ResolveError::Ambiguous`]
/// when more than one item matches.
pub fn resolve_unique(
    items: &[Value],
    resource_type: &str,
    name_field: &str,
    needle: &str,
    mode: NameMatch,
) -> Result<Value, ResolveError> {
    let matches = filter_by_name(items, name_field, needle, mode);
    match matches.len() {
        0 => Err(ResolveError::NotFound {
            resource_type: resource_type.to_owned(),
            name: needle.trim().to_owned(),
        }),
        1 => Ok(matches[0].clone()),
        _ => Err(ResolveError::Ambiguous {
            resource_type: resource_type.to_owned(),
            name: needle.trim().to_owned(),
            count: matches.len(),
            matches,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plain_string_exact() {
        assert!(name_matches(&json!("Lobby"), "lobby", NameMatch::Exact));
        assert!(!name_matches(
            &json!("Lobby"),
            "Main Lobby",
            NameMatch::Exact
        ));
    }

    #[test]
    fn locale_map_any_locale() {
        let name = json!({"en": "Lobby", "he": "לובי"});
        assert!(name_matches(&name, "לובי", NameMatch::Exact));
        assert!(name_matches(&name, "lobby", NameMatch::Exact));
    }

    #[test]
    fn localized_string_value_unwrap() {
        let name = json!({"value": {"en": "Lobby Portal"}});
        assert_eq!(localized_values(&name), vec!["Lobby Portal"]);
        assert!(name_matches(&name, "lobby portal", NameMatch::Exact));
    }

    #[test]
    fn fuzzy_substring() {
        let name = json!({"en": "Main Lobby Door"});
        assert!(name_matches(&name, "lobby", NameMatch::Fuzzy));
        assert!(!name_matches(&name, "lobby", NameMatch::Exact));
    }

    #[test]
    fn resolve_unique_duplicate_names() {
        let items = vec![
            json!({"id": "1", "name": {"en": "Lobby"}}),
            json!({"id": "2", "name": {"en": "Lobby"}}),
        ];
        let err = resolve_unique(&items, "portal", "name", "Lobby", NameMatch::Exact).unwrap_err();
        assert!(matches!(err, ResolveError::Ambiguous { .. }));
        assert_eq!(err.ambiguous_count(), Some(2));
    }

    #[test]
    fn resolve_unique_not_found() {
        let items = vec![json!({"id": "1", "name": {"en": "Other"}})];
        assert!(matches!(
            resolve_unique(&items, "integration", "name", "Lobby", NameMatch::Exact),
            Err(ResolveError::NotFound { .. })
        ));
    }

    #[test]
    fn empty_needle_never_matches() {
        assert!(!name_matches(&json!("Lobby"), "  ", NameMatch::Exact));
    }
}
