//! Retry policy configuration.
//!
//! The default policy mirrors the behaviour of most modern `SaaS` SDKs: exponential
//! backoff on 408 / 425 / 429 / 5xx responses and transport errors, honouring the
//! `Retry-After` header when present.

use std::time::Duration;

/// Retry policy applied by the core transport layer.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts after the initial request. `0` disables retries.
    pub max_retries: u32,
    /// Initial backoff between the first and second attempt.
    pub initial_backoff: Duration,
    /// Backoff multiplier between attempts.
    pub backoff_multiplier: f64,
    /// Cap on a single backoff interval.
    pub max_backoff: Duration,
    /// Upper bound on total time spent retrying a single logical request.
    pub total_deadline: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(250),
            backoff_multiplier: 2.0,
            max_backoff: Duration::from_secs(8),
            total_deadline: Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    /// Policy that disables all retries. Used primarily for determinism in tests.
    #[must_use]
    pub fn none() -> Self {
        Self {
            max_retries: 0,
            ..Self::default()
        }
    }
}
