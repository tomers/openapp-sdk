//! PyO3 bridge exposing [`openapp_sdk_core`] to Python.
//!
//! Module layout:
//!
//! * `openapp_sdk.bridge._bridge` is the compiled extension (this crate).
//! * `openapp_sdk.bridge.runtime.Runtime` wraps [`Runtime`] here with a friendly
//!   Python surface.
//! * `openapp_sdk.bridge.client.Client` wraps [`Client`] here.
//!
//! The bridge owns a single tokio runtime per [`Runtime`]; every [`Client`]
//! created from that runtime shares it.

use std::sync::Arc;
use std::time::Duration;

use ::openapp_sdk_core::{
    transport::{MultipartRequestSpec, RequestSpec, Transport},
    Client as CoreClient, SdkError,
};
use once_cell::sync::OnceCell;
use pyo3::{IntoPyObjectExt, exceptions::PyValueError, prelude::*, types::PyDict};
use pyo3_async_runtimes::tokio::{future_into_py, get_runtime};
use reqwest::Method;

/// One-time bootstrap for the tracing subscriber used by the Rust core.
///
/// PyO3 extensions are loaded once per process, so this is naturally one-shot.
fn init_tracing() {
    static ONCE: OnceCell<()> = OnceCell::new();
    let _ = ONCE.get_or_init(|| {
        let filter = tracing_subscriber::EnvFilter::try_from_env("OPENAPP_SDK_LOG")
            .or_else(|_| tracing_subscriber::EnvFilter::try_from_default_env())
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"));
        let _ = tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_writer(std::io::stderr)
            .try_init();
    });
}

/// Wraps the dedicated tokio runtime used by this bridge.
///
/// Python users go through [`openapp_sdk.bridge.runtime.Runtime`] rather than
/// touching this directly.
#[pyclass(module = "openapp_sdk.bridge._bridge", name = "Runtime")]
pub struct Runtime {
    _ctx: Arc<()>,
}

#[pymethods]
impl Runtime {
    #[new]
    fn new() -> PyResult<Self> {
        init_tracing();
        // `get_runtime()` lazily spawns the shared runtime on first call. Calling it
        // here ensures it is up before the Python layer starts scheduling futures.
        let _ = get_runtime();
        Ok(Self { _ctx: Arc::new(()) })
    }
}

/// Wraps [`openapp_sdk_core::Client`].
#[pyclass(module = "openapp_sdk.bridge._bridge", name = "Client")]
pub struct Client {
    core: CoreClient,
    transport: Arc<Transport>,
}

#[pymethods]
impl Client {
    /// Build a client from an API key. The base URL is derived from the token when
    /// not explicitly supplied.
    #[new]
    #[pyo3(signature = (api_key, base_url = None, user_agent = None, timeout_secs = None, max_retries = None))]
    fn new(
        api_key: &str,
        base_url: Option<&str>,
        user_agent: Option<String>,
        timeout_secs: Option<f64>,
        max_retries: Option<u32>,
    ) -> PyResult<Self> {
        init_tracing();

        let mut builder = CoreClient::builder().api_key(api_key);
        if let Some(url) = base_url {
            builder = builder
                .base_url(url)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        }
        if let Some(ua) = user_agent {
            builder = builder.user_agent(ua);
        }
        if let Some(secs) = timeout_secs {
            builder = builder.default_timeout(Duration::from_secs_f64(secs));
        }
        if let Some(max) = max_retries {
            let policy = ::openapp_sdk_core::retry::RetryPolicy {
                max_retries: max,
                ..Default::default()
            };
            builder = builder.retry_policy(policy);
        }

        let core = builder
            .build()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let transport = core.transport();
        Ok(Self { core, transport })
    }

    /// Base URL the client is pinned to.
    #[getter]
    fn base_url(&self) -> String {
        self.core.config().base_url.as_str().to_owned()
    }

    /// Execute a raw JSON request asynchronously and return `(status, body)`.
    ///
    /// * `method`: HTTP verb.
    /// * `path`: relative path (leading `/` optional).
    /// * `body_json`: serialized JSON body, or `None` for GET/DELETE-like calls.
    /// * `query`: list of `(name, value)` pairs; `value = None` is skipped.
    /// * `timeout_secs`: per-request timeout override.
    #[pyo3(signature = (method, path, body_json = None, query = None, timeout_secs = None))]
    fn request<'py>(
        &self,
        py: Python<'py>,
        method: &str,
        path: String,
        body_json: Option<String>,
        query: Option<Vec<(String, Option<String>)>>,
        timeout_secs: Option<f64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let transport = self.transport.clone();
        let method = Method::from_bytes(method.to_ascii_uppercase().as_bytes())
            .map_err(|e| PyValueError::new_err(format!("invalid HTTP method: {e}")))?;

        let body_value: Option<serde_json::Value> = match body_json {
            Some(ref raw) if !raw.is_empty() => Some(
                serde_json::from_str(raw)
                    .map_err(|e| PyValueError::new_err(format!("invalid JSON body: {e}")))?,
            ),
            _ => None,
        };
        let query_pairs = query.unwrap_or_default();
        let timeout = timeout_secs.map(Duration::from_secs_f64);

        future_into_py(py, async move {
            let query_refs: Vec<(&str, Option<String>)> = query_pairs
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect();

            let spec = RequestSpec {
                method,
                path: &path,
                query: &query_refs,
                body: body_value.as_ref(),
                extra_headers: &[],
                timeout,
            };

            let result = transport
                .request_json::<serde_json::Value, serde_json::Value>(spec)
                .await;
            translate(result)
        })
    }

    /// `POST` multipart upload (e.g. device or entity image).
    #[pyo3(signature = (
        path,
        field_name,
        filename,
        content_type,
        file_bytes,
        query = None,
        timeout_secs = None,
    ))]
    fn multipart_post<'py>(
        &self,
        py: Python<'py>,
        path: String,
        field_name: String,
        filename: String,
        content_type: String,
        file_bytes: Vec<u8>,
        query: Option<Vec<(String, Option<String>)>>,
        timeout_secs: Option<f64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let transport = self.transport.clone();
        let timeout = timeout_secs.map(Duration::from_secs_f64);
        let query_pairs = query.unwrap_or_default();

        future_into_py(py, async move {
            let query_refs: Vec<(&str, Option<String>)> = query_pairs
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone()))
                .collect();

            let spec = MultipartRequestSpec {
                path: path.as_str(),
                query: &query_refs,
                field_name: field_name.as_str(),
                filename: filename.as_str(),
                content_type: content_type.as_str(),
                body: &file_bytes,
                timeout,
            };

            let result = transport
                .request_multipart::<serde_json::Value>(spec)
                .await;
            translate(result)
        })
    }
}

/// Convert `Result<T, SdkError>` into a Python-friendly result, preserving the
/// error shape (kind + message + optional status) for the Python wrapper to
/// translate into typed exceptions.
fn translate(result: Result<serde_json::Value, SdkError>) -> PyResult<Py<PyAny>> {
    Python::attach(|py| match result {
        Ok(value) => {
            let json = serde_json::to_string(&value)
                .map_err(|e| PyValueError::new_err(format!("serde error: {e}")))?;
            json.into_py_any(py)
        }
        Err(err) => Err(sdk_error_to_py(py, &err)),
    })
}

/// Build a structured Python exception for an [`SdkError`]. The Python wrapper in
/// `openapp_sdk.bridge.client` converts this into the typed hierarchy declared in
/// `openapp_sdk.errors`.
fn sdk_error_to_py(py: Python<'_>, err: &SdkError) -> PyErr {
    let kind = match err {
        SdkError::Api { .. } => "api",
        SdkError::Http { .. } => "http",
        SdkError::Auth(_) | SdkError::Token(_) => "auth",
        SdkError::Transport(_) => "transport",
        SdkError::Deserialize(_) => "deserialize",
        SdkError::Config(_) => "config",
        SdkError::Serialize(_) => "serialize",
        SdkError::Other(_) => "other",
        _ => "other",
    };

    let payload = PyDict::new(py);
    let _ = payload.set_item("kind", kind);
    let _ = payload.set_item("message", err.to_string());
    if let Some(status) = err.status() {
        let _ = payload.set_item("status", status);
    }
    if let SdkError::Api { body, .. } = err {
        if let Some(code) = body.code.as_deref() {
            let _ = payload.set_item("code", code);
        }
        if let Some(cid) = body.correlation_id.as_deref() {
            let _ = payload.set_item("correlation_id", cid);
        }
        if let Some(details) = &body.details {
            let _ = payload.set_item("details_json", details.to_string());
        }
    }
    PyValueError::new_err(payload.unbind())
}

/// Bridge module entry point. Imported as `openapp_sdk.bridge._bridge`.
#[pymodule]
fn _bridge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Runtime>()?;
    m.add_class::<Client>()?;
    Ok(())
}
