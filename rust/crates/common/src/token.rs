//! `OpenApp` API-key token parsing.
//!
//! Mirrors `apps/backend/local_server/src/api_key_store.rs`: tokens have the shape
//! `{origin}_openapp_{secret}`, where `origin` is the deployment's public origin
//! (`OPENAPP_API_BASE_URL` / `OPENAPP_BASE_URL` on the backend — scheme, host and
//! optional port, never a path). The backend rejects a token whose origin differs from
//! its own, so the origin is also where the SDK sends requests: the API root is
//! `{origin}{API_PATH_PREFIX}`. See `notes/contracts/api-key-authentication.md`.

use thiserror::Error;
use url::Url;

/// Separator between `origin` and `secret` in an API-key token.
pub const API_KEY_SEPARATOR: &str = "_openapp_";

/// Path prefix of the versioned API on every deployment origin (gateway rules match
/// `/api/v1/...`; see `ory/oathkeeper/rules.json.tmpl`).
pub const API_PATH_PREFIX: &str = "/api/v1";

/// Errors raised when parsing an `OpenApp` API-key token.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TokenFormatError {
    #[error("token is empty")]
    Empty,

    #[error("token does not contain the `{API_KEY_SEPARATOR}` separator")]
    MissingSeparator,

    #[error("token secret is empty")]
    EmptySecret,

    #[error("token origin `{0}` is not a valid absolute http(s) URL")]
    InvalidOrigin(String),

    #[error(
        "token origin `{0}` must be a bare origin (scheme, host, optional port) with no path, \
         query or fragment"
    )]
    OriginHasPath(String),
}

/// A parsed `OpenApp` API-key token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiKey {
    raw: String,
    origin: Url,
    secret: String,
}

impl ApiKey {
    /// Parse a token string of the form `{origin}_openapp_{secret}`.
    ///
    /// # Errors
    /// Returns a [`TokenFormatError`] if the token is empty, lacks the separator, has an
    /// empty secret, or if `origin` is not a bare absolute `http(s)` origin.
    pub fn parse(token: impl Into<String>) -> Result<Self, TokenFormatError> {
        let raw = token.into();
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(TokenFormatError::Empty);
        }

        let (origin_str, secret) = trimmed
            .split_once(API_KEY_SEPARATOR)
            .ok_or(TokenFormatError::MissingSeparator)?;

        if secret.is_empty() {
            return Err(TokenFormatError::EmptySecret);
        }

        let origin = Url::parse(origin_str)
            .map_err(|_| TokenFormatError::InvalidOrigin(origin_str.to_string()))?;
        if !matches!(origin.scheme(), "http" | "https") || origin.host().is_none() {
            return Err(TokenFormatError::InvalidOrigin(origin_str.to_string()));
        }
        // `Url` normalizes an empty path to `/`; anything else means the token was not
        // minted by the backend (which embeds a bare origin).
        if origin.path() != "/" || origin.query().is_some() || origin.fragment().is_some() {
            return Err(TokenFormatError::OriginHasPath(origin_str.to_string()));
        }

        Ok(Self {
            raw: trimmed.to_string(),
            origin,
            secret: secret.to_string(),
        })
    }

    /// The deployment origin the token was issued by.
    #[must_use]
    pub fn origin(&self) -> &Url {
        &self.origin
    }

    /// The versioned API root requests are resolved against: `{origin}/api/v1`.
    #[must_use]
    pub fn api_base_url(&self) -> Url {
        let mut url = self.origin.clone();
        url.set_path(API_PATH_PREFIX);
        url
    }

    /// The secret component of the token (never log this).
    #[must_use]
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// The full token string, exactly as the backend issued it. This is the value
    /// sent in the `X-API-Key` header.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print the secret. Show only the origin and a redacted suffix.
        let suffix = if self.secret.len() > 6 {
            format!("…{}", &self.secret[self.secret.len() - 6..])
        } else {
            "…".to_string()
        };
        write!(f, "ApiKey({} {})", self.origin, suffix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_token() {
        let tok = ApiKey::parse("https://openapp.house_openapp_SECRET").unwrap();
        assert_eq!(tok.origin().as_str(), "https://openapp.house/");
        assert_eq!(tok.secret(), "SECRET");
        assert_eq!(tok.as_str(), "https://openapp.house_openapp_SECRET");
    }

    #[test]
    fn api_base_url_appends_versioned_prefix() {
        let tok = ApiKey::parse("https://openapp.house_openapp_SECRET").unwrap();
        assert_eq!(tok.api_base_url().as_str(), "https://openapp.house/api/v1");
    }

    #[test]
    fn parses_origin_with_port() {
        let tok = ApiKey::parse("http://oathkeeper:4455_openapp_SECRET").unwrap();
        assert_eq!(tok.api_base_url().as_str(), "http://oathkeeper:4455/api/v1");
    }

    #[test]
    fn accepts_origin_with_trailing_slash() {
        let tok = ApiKey::parse("https://openapp.house/_openapp_SECRET").unwrap();
        assert_eq!(tok.api_base_url().as_str(), "https://openapp.house/api/v1");
    }

    #[test]
    fn rejects_origin_with_path() {
        assert_eq!(
            ApiKey::parse("https://openapp.house/api/v1_openapp_SECRET").unwrap_err(),
            TokenFormatError::OriginHasPath("https://openapp.house/api/v1".into())
        );
    }

    #[test]
    fn rejects_origin_with_query() {
        assert!(matches!(
            ApiKey::parse("https://openapp.house?x=1_openapp_SECRET").unwrap_err(),
            TokenFormatError::OriginHasPath(_)
        ));
    }

    #[test]
    fn rejects_non_http_scheme() {
        assert!(matches!(
            ApiKey::parse("ftp://openapp.house_openapp_SECRET").unwrap_err(),
            TokenFormatError::InvalidOrigin(_)
        ));
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(ApiKey::parse("").unwrap_err(), TokenFormatError::Empty);
        assert_eq!(ApiKey::parse("   ").unwrap_err(), TokenFormatError::Empty);
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(
            ApiKey::parse("https://openapp.house").unwrap_err(),
            TokenFormatError::MissingSeparator
        );
    }

    #[test]
    fn rejects_empty_secret() {
        assert_eq!(
            ApiKey::parse("https://openapp.house_openapp_").unwrap_err(),
            TokenFormatError::EmptySecret
        );
    }

    #[test]
    fn rejects_invalid_origin() {
        let err = ApiKey::parse("not a url_openapp_SECRET").unwrap_err();
        assert!(matches!(err, TokenFormatError::InvalidOrigin(_)));
    }

    #[test]
    fn display_hides_secret() {
        let tok = ApiKey::parse("https://openapp.house_openapp_supersecret").unwrap();
        let s = format!("{tok}");
        assert!(!s.contains("supersecret"), "display leaked secret: {s}");
    }
}
