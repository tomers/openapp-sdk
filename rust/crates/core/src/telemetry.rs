//! Tracing / telemetry configuration.
//!
//! The SDK emits structured [`tracing`] events for every request. OpenTelemetry export
//! is available via the (optional) `otel` Cargo feature — kept behind a feature flag so
//! default builds stay lean and language SDKs can opt in as needed.

use tracing_subscriber::EnvFilter;

/// Initialize a minimal `tracing` subscriber with `RUST_LOG` / `OPENAPP_SDK_LOG`
/// filter support. Safe to call multiple times (no-op after the first call).
///
/// Language SDKs are expected to call this at most once; for `PyO3` bindings this is
/// invoked from the bridge's `Client.connect` implementation.
pub fn init_fallback() {
    let filter = EnvFilter::try_from_env("OPENAPP_SDK_LOG")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::new("openapp_sdk_core=info,warn"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();
}
