//! HTTP transport engine shared by every sub-client.
//!
//! `Transport` holds the [`reqwest_middleware::ClientWithMiddleware`] configured with
//! retries, the [`auth::TokenProvider`], and the set of [`interceptor::Interceptor`]s.
//! Every sub-client goes through [`Transport::request_json`] — there is exactly one
//! place in the SDK where an HTTP call is actually made.

use std::time::Duration;

use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::multipart::{Form, Part};
use reqwest::{Method, StatusCode};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::{
    auth::SharedTokenProvider,
    error::{ApiErrorResponse, SdkError},
    interceptor::SharedInterceptor,
};

/// Low-level request descriptor.
#[derive(Debug)]
pub struct RequestSpec<'a, B: ?Sized = ()> {
    pub method: Method,
    /// Path relative to the base URL, e.g. `"/orgs"` or `"/devices/{id}"`. Leading
    /// slash optional.
    pub path: &'a str,
    /// Query-string parameters. `None` values are skipped.
    pub query: &'a [(&'a str, Option<String>)],
    /// Optional JSON body. Set to `None` for GET / DELETE.
    pub body: Option<&'a B>,
    /// Extra headers beyond the defaults (auth, content-type, user-agent).
    pub extra_headers: &'a [(&'a str, String)],
    /// Per-request timeout override (else the transport's default).
    pub timeout: Option<Duration>,
}

impl<B: ?Sized> Default for RequestSpec<'_, B> {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: "",
            query: &[],
            body: None,
            extra_headers: &[],
            timeout: None,
        }
    }
}

/// Multipart file upload (e.g. `POST /devices/{id}/image`).
#[derive(Debug, Clone)]
pub struct MultipartRequestSpec<'a> {
    pub path: &'a str,
    pub query: &'a [(&'a str, Option<String>)],
    pub field_name: &'a str,
    pub filename: &'a str,
    pub content_type: &'a str,
    pub body: &'a [u8],
    pub timeout: Option<Duration>,
}

/// Shared HTTP engine used by every sub-client.
#[derive(Debug, Clone)]
pub struct Transport {
    client: ClientWithMiddleware,
    base_url: Url,
    user_agent: String,
    tokens: SharedTokenProvider,
    interceptors: Vec<SharedInterceptor>,
    default_timeout: Duration,
}

impl Transport {
    pub(crate) fn new(
        client: ClientWithMiddleware,
        base_url: Url,
        user_agent: String,
        tokens: SharedTokenProvider,
        interceptors: Vec<SharedInterceptor>,
        default_timeout: Duration,
    ) -> Self {
        Self {
            client,
            base_url,
            user_agent,
            tokens,
            interceptors,
            default_timeout,
        }
    }

    /// The base URL every path is resolved against.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Execute a request and decode the JSON body as `R`.
    ///
    /// Empty-body responses (204 No Content) decode into `R = ()`. Non-2xx responses
    /// are turned into [`SdkError::Api`] when the body is a parseable
    /// [`ApiErrorResponse`], else [`SdkError::Http`].
    pub async fn request_json<B, R>(&self, spec: RequestSpec<'_, B>) -> Result<R, SdkError>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned + 'static,
    {
        let url = self.resolve_url(spec.path)?;

        let mut builder = self.client.request(spec.method.clone(), url.clone());
        builder = builder.header(reqwest::header::USER_AGENT, &self.user_agent);

        let token = self.tokens.token().await?;
        builder = builder.header(reqwest::header::AUTHORIZATION, &token.authorization);

        for (name, value) in spec.extra_headers {
            builder = builder.header(*name, value);
        }

        // Filter out `None` query values (convenient for optional params).
        let pairs: Vec<(&str, String)> = spec
            .query
            .iter()
            .filter_map(|(k, v)| v.clone().map(|vv| (*k, vv)))
            .collect();
        if !pairs.is_empty() {
            builder = builder.query(&pairs);
        }

        if let Some(body) = spec.body {
            builder = builder.json(body);
        }

        if let Some(timeout) = spec.timeout {
            builder = builder.timeout(timeout);
        } else {
            builder = builder.timeout(self.default_timeout);
        }

        let mut request = builder
            .build()
            .map_err(|e| SdkError::Serialize(e.to_string()))?;
        for interceptor in &self.interceptors {
            request = interceptor.on_request(request).await?;
        }

        let response = self.client.execute(request).await.map_err(SdkError::from)?;

        for interceptor in &self.interceptors {
            interceptor.on_response(&response).await?;
        }

        let status = response.status();
        if status.is_success() {
            decode_success::<R>(response).await
        } else {
            Err(decode_error(status, response).await)
        }
    }

    /// JSON response body; request body is **opaque bytes** with an explicit
    /// `Content-Type` (for example a pre-encoded `multipart/form-data` body from `OpenAPI`
    /// generator clients). Same auth, interceptors, and error handling as
    /// [`Self::request_json`].
    #[allow(clippy::too_many_arguments)] // mirrors `request_json` inputs plus explicit body/type
    pub async fn request_json_raw_body(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, Option<String>)],
        body: Vec<u8>,
        content_type: &str,
        extra_headers: &[(&str, String)],
        timeout: Option<Duration>,
    ) -> Result<serde_json::Value, SdkError> {
        let url = self.resolve_url(path)?;

        let ct = reqwest::header::HeaderValue::from_str(content_type)
            .map_err(|e| SdkError::Serialize(format!("content-type: {e}")))?;

        let mut builder = self.client.request(method.clone(), url.clone());
        builder = builder.header(reqwest::header::USER_AGENT, &self.user_agent);
        builder = builder.header(reqwest::header::CONTENT_TYPE, ct);

        let token = self.tokens.token().await?;
        builder = builder.header(reqwest::header::AUTHORIZATION, &token.authorization);

        for (name, value) in extra_headers {
            builder = builder.header(*name, value);
        }

        let pairs: Vec<(&str, String)> = query
            .iter()
            .filter_map(|(k, v)| v.clone().map(|vv| (*k, vv)))
            .collect();
        if !pairs.is_empty() {
            builder = builder.query(&pairs);
        }

        builder = builder.body(body);

        if let Some(t) = timeout {
            builder = builder.timeout(t);
        } else {
            builder = builder.timeout(self.default_timeout);
        }

        let mut request = builder
            .build()
            .map_err(|e| SdkError::Serialize(e.to_string()))?;
        for interceptor in &self.interceptors {
            request = interceptor.on_request(request).await?;
        }

        let response = self.client.execute(request).await.map_err(SdkError::from)?;

        for interceptor in &self.interceptors {
            interceptor.on_response(&response).await?;
        }

        let status = response.status();
        if status.is_success() {
            decode_success::<serde_json::Value>(response).await
        } else {
            Err(decode_error(status, response).await)
        }
    }

    /// Execute a request and return the success HTTP status plus the **raw** body as a
    /// stream of byte chunks.
    ///
    /// Use this for `text/event-stream` (SSE) and other non-JSON bodies. Auth, retries
    /// (on the initial connect), interceptors, and error mapping for the status line
    /// match [`Self::request_json`]. Chunk errors from the wire are propagated as
    /// [`SdkError::Transport`].
    pub async fn request_stream<B>(
        &self,
        spec: RequestSpec<'_, B>,
    ) -> Result<
        (
            StatusCode,
            impl Stream<Item = Result<Bytes, SdkError>> + Send,
        ),
        SdkError,
    >
    where
        B: Serialize + ?Sized,
    {
        let url = self.resolve_url(spec.path)?;

        let mut builder = self.client.request(spec.method.clone(), url.clone());
        builder = builder.header(reqwest::header::USER_AGENT, &self.user_agent);

        let token = self.tokens.token().await?;
        builder = builder.header(reqwest::header::AUTHORIZATION, &token.authorization);

        for (name, value) in spec.extra_headers {
            builder = builder.header(*name, value);
        }

        let pairs: Vec<(&str, String)> = spec
            .query
            .iter()
            .filter_map(|(k, v)| v.clone().map(|vv| (*k, vv)))
            .collect();
        if !pairs.is_empty() {
            builder = builder.query(&pairs);
        }

        if let Some(body) = spec.body {
            builder = builder.json(body);
        }

        if let Some(timeout) = spec.timeout {
            builder = builder.timeout(timeout);
        } else {
            builder = builder.timeout(self.default_timeout);
        }

        let mut request = builder
            .build()
            .map_err(|e| SdkError::Serialize(e.to_string()))?;
        for interceptor in &self.interceptors {
            request = interceptor.on_request(request).await?;
        }

        let response = self.client.execute(request).await.map_err(SdkError::from)?;

        for interceptor in &self.interceptors {
            interceptor.on_response(&response).await?;
        }

        let status = response.status();
        if status.is_success() {
            let stream = response.bytes_stream().map(|r| r.map_err(SdkError::from));
            Ok((status, stream))
        } else {
            Err(decode_error(status, response).await)
        }
    }

    /// `POST` with a single multipart file field; decodes a JSON body on success.
    pub async fn request_multipart<R>(&self, spec: MultipartRequestSpec<'_>) -> Result<R, SdkError>
    where
        R: DeserializeOwned + 'static,
    {
        let url = self.resolve_url(spec.path)?;

        let part = Part::bytes(spec.body.to_vec())
            .file_name(spec.filename.to_string())
            .mime_str(spec.content_type)
            .map_err(|e| SdkError::Serialize(format!("multipart: {e}")))?;
        let form = Form::new().part(spec.field_name.to_string(), part);

        let mut builder = self.client.request(Method::POST, url.clone());
        builder = builder.header(reqwest::header::USER_AGENT, &self.user_agent);

        let token = self.tokens.token().await?;
        builder = builder.header(reqwest::header::AUTHORIZATION, &token.authorization);

        let pairs: Vec<(&str, String)> = spec
            .query
            .iter()
            .filter_map(|(k, v)| v.clone().map(|vv| (*k, vv)))
            .collect();
        if !pairs.is_empty() {
            builder = builder.query(&pairs);
        }

        builder = builder.multipart(form);

        if let Some(timeout) = spec.timeout {
            builder = builder.timeout(timeout);
        } else {
            builder = builder.timeout(self.default_timeout);
        }

        let mut request = builder
            .build()
            .map_err(|e| SdkError::Serialize(e.to_string()))?;
        for interceptor in &self.interceptors {
            request = interceptor.on_request(request).await?;
        }

        let response = self.client.execute(request).await.map_err(SdkError::from)?;

        for interceptor in &self.interceptors {
            interceptor.on_response(&response).await?;
        }

        let status = response.status();
        if status.is_success() {
            decode_success::<R>(response).await
        } else {
            Err(decode_error(status, response).await)
        }
    }

    fn resolve_url(&self, path: &str) -> Result<Url, SdkError> {
        let path = path.strip_prefix('/').unwrap_or(path);
        // Ensure the base URL ends with `/` so `join` treats it as a directory.
        let mut base = self.base_url.clone();
        if !base.path().ends_with('/') {
            let p = format!("{}/", base.path());
            base.set_path(&p);
        }
        base.join(path)
            .map_err(|e| SdkError::Config(format!("could not build URL from path {path}: {e}")))
    }
}

async fn decode_success<R: DeserializeOwned + 'static>(
    response: reqwest::Response,
) -> Result<R, SdkError> {
    // `()` is the conventional "I don't care about the body" marker — handle 204 /
    // empty responses without forcing callers to wrap everything in `Option`.
    if std::any::TypeId::of::<R>() == std::any::TypeId::of::<()>() {
        // Drop the body; we still need to consume it so the connection can be reused.
        let _ = response.bytes().await.map_err(SdkError::from)?;
        // SAFETY: `R` is `()`, so `serde_json::from_str("null")` decodes to `()`.
        return serde_json::from_str::<R>("null").map_err(SdkError::from);
    }

    let bytes = response.bytes().await.map_err(SdkError::from)?;
    if bytes.is_empty() {
        // Try to decode `null` first; if `R` expects a concrete value this will error out
        // with a useful `Deserialize` message.
        return serde_json::from_str::<R>("null").map_err(SdkError::from);
    }
    serde_json::from_slice::<R>(&bytes).map_err(SdkError::from)
}

async fn decode_error(status: StatusCode, response: reqwest::Response) -> SdkError {
    let status_code = status.as_u16();
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(err) => {
            return SdkError::Http {
                status: status_code,
                message: format!("failed to read error body: {err}"),
            };
        }
    };

    if status_code == 401 || status_code == 403 {
        let message = serde_json::from_slice::<ApiErrorResponse>(&bytes).map_or_else(
            |_| String::from_utf8_lossy(&bytes).to_string(),
            |b| b.message,
        );
        return SdkError::Auth(message);
    }

    match serde_json::from_slice::<ApiErrorResponse>(&bytes) {
        Ok(body) => SdkError::Api {
            status: status_code,
            body,
        },
        Err(_) => SdkError::Http {
            status: status_code,
            message: String::from_utf8_lossy(&bytes).to_string(),
        },
    }
}
