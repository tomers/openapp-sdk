//! `OpenApp` API-key token parsing.
//!
//! Mirrors `apps/backend/local_server/src/api_key_store.rs`: tokens have the shape
//! `{base_url}_openapp_{secret}`. The base URL embedded in the token is the canonical
//! one the backend was issued for, which lets SDK clients auto-derive the endpoint
//! without extra configuration.

use thiserror::Error;
use url::Url;

/// Separator between `base_url` and `secret` in an API-key token.
pub const API_KEY_SEPARATOR: &str = "_openapp_";

/// Errors raised when parsing an `OpenApp` API-key token.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TokenFormatError {
    #[error("token is empty")]
    Empty,

    #[error("token does not contain the `{API_KEY_SEPARATOR}` separator")]
    MissingSeparator,

    #[error("token secret is empty")]
    EmptySecret,

    #[error("token base URL `{0}` is not a valid absolute URL")]
    InvalidBaseUrl(String),
}

/// A parsed `OpenApp` API-key token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiKey {
    raw: String,
    base_url: Url,
    secret: String,
}

impl ApiKey {
    /// Parse a token string of the form `{base_url}_openapp_{secret}`.
    ///
    /// # Errors
    /// Returns a [`TokenFormatError`] if the token is empty, lacks the separator, has an
    /// empty secret, or if `base_url` is not a parseable absolute URL.
    pub fn parse(token: impl Into<String>) -> Result<Self, TokenFormatError> {
        let raw = token.into();
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(TokenFormatError::Empty);
        }

        let (base_url_str, secret) = trimmed
            .split_once(API_KEY_SEPARATOR)
            .ok_or(TokenFormatError::MissingSeparator)?;

        if secret.is_empty() {
            return Err(TokenFormatError::EmptySecret);
        }

        let base_url = Url::parse(base_url_str)
            .map_err(|_| TokenFormatError::InvalidBaseUrl(base_url_str.to_string()))?;

        Ok(Self {
            raw: trimmed.to_string(),
            base_url,
            secret: secret.to_string(),
        })
    }

    /// The canonical base URL the token was issued for.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// The secret component of the token (never log this).
    #[must_use]
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// The raw token string, suitable for use as an `Authorization: Bearer` value.
    #[must_use]
    pub fn as_bearer(&self) -> &str {
        &self.raw
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print the secret. Show only the base URL and a redacted suffix.
        let suffix = if self.secret.len() > 6 {
            format!("…{}", &self.secret[self.secret.len() - 6..])
        } else {
            "…".to_string()
        };
        write!(f, "ApiKey({} {})", self.base_url, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_token() {
        let tok = ApiKey::parse("https://api.openapp.house/api/v1_openapp_SECRET").unwrap();
        assert_eq!(tok.base_url().as_str(), "https://api.openapp.house/api/v1");
        assert_eq!(tok.secret(), "SECRET");
        assert_eq!(
            tok.as_bearer(),
            "https://api.openapp.house/api/v1_openapp_SECRET"
        );
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(ApiKey::parse("").unwrap_err(), TokenFormatError::Empty);
        assert_eq!(ApiKey::parse("   ").unwrap_err(), TokenFormatError::Empty);
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(
            ApiKey::parse("https://api.openapp.house/api/v1").unwrap_err(),
            TokenFormatError::MissingSeparator
        );
    }

    #[test]
    fn rejects_empty_secret() {
        assert_eq!(
            ApiKey::parse("https://api.openapp.house/api/v1_openapp_").unwrap_err(),
            TokenFormatError::EmptySecret
        );
    }

    #[test]
    fn rejects_invalid_base_url() {
        let err = ApiKey::parse("not a url_openapp_SECRET").unwrap_err();
        assert!(matches!(err, TokenFormatError::InvalidBaseUrl(_)));
    }

    #[test]
    fn display_hides_secret() {
        let tok = ApiKey::parse("https://api.openapp.house/api/v1_openapp_supersecret").unwrap();
        let s = format!("{tok}");
        assert!(!s.contains("supersecret"), "display leaked secret: {s}");
    }
}
