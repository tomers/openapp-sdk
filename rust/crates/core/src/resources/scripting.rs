//! `OpenApp` Scripting resource group.
//!
//! Scripting is an asynchronous job API: `POST /scripting/executions` accepts a
//! program and returns a job, which the caller polls until `status` reaches a
//! terminal value. [`ScriptingClient::execute`] wraps that submit-then-poll
//! cycle for callers that just want the result.

use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use reqwest::Method;

use super::JsonValue;
use crate::{
    error::SdkError,
    transport::{RequestSpec, Transport},
};

/// How [`ScriptingClient::execute`] polls a job to completion.
///
/// The defaults follow the backend contract: poll every second, back off to
/// five seconds, and give up at the 15-minute server-side runtime cap (an
/// execution that runs longer is failed server-side, so waiting past it can
/// never observe a result). Override via [`ScriptingClient::with_poll_schedule`]
/// when a caller needs a different budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollSchedule {
    /// Wait before the first poll.
    pub initial_delay: Duration,
    /// Upper bound the delay backs off to.
    pub max_delay: Duration,
    /// Give up with an error once this much wall time has elapsed without a
    /// terminal status.
    pub timeout: Duration,
}

impl PollSchedule {
    /// Build a schedule from its three parts.
    #[must_use]
    pub const fn new(initial_delay: Duration, max_delay: Duration, timeout: Duration) -> Self {
        Self {
            initial_delay,
            max_delay,
            timeout,
        }
    }
}

impl Default for PollSchedule {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            timeout: Duration::from_mins(15),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptingClient {
    transport: Arc<Transport>,
    poll: PollSchedule,
}

impl ScriptingClient {
    pub(crate) fn new(transport: Arc<Transport>) -> Self {
        Self {
            transport,
            poll: PollSchedule::default(),
        }
    }

    /// Override how [`Self::execute`] polls for a terminal status.
    #[must_use]
    pub fn with_poll_schedule(mut self, poll: PollSchedule) -> Self {
        self.poll = poll;
        self
    }

    /// `POST /scripting/executions` — submit a program; returns the `202` job.
    pub async fn create_execution(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        self.transport
            .request_json::<JsonValue, JsonValue>(RequestSpec {
                method: Method::POST,
                path: "/scripting/executions",
                body: Some(body),
                ..Default::default()
            })
            .await
    }

    /// `GET /scripting/executions/{id}` — the only route that carries `result`.
    pub async fn get_execution(&self, id: &str) -> Result<JsonValue, SdkError> {
        let path = format!("/scripting/executions/{id}");
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// `GET /scripting/executions?limit=…` — summaries (no `result`), newest first.
    pub async fn list_executions(&self, limit: Option<i64>) -> Result<JsonValue, SdkError> {
        let query = [("limit", limit.map(|n| n.to_string()))];
        self.transport
            .request_json::<(), JsonValue>(RequestSpec {
                method: Method::GET,
                path: "/scripting/executions",
                query: &query,
                ..Default::default()
            })
            .await
    }

    /// `DELETE /scripting/executions/{id}` — request cooperative cancellation.
    ///
    /// Cancellation is not a rollback: effects the program already applied stay
    /// applied, and the worker only notices within a few seconds.
    pub async fn cancel_execution(&self, id: &str) -> Result<(), SdkError> {
        let path = format!("/scripting/executions/{id}");
        self.transport
            .request_json::<(), ()>(RequestSpec {
                method: Method::DELETE,
                path: &path,
                ..Default::default()
            })
            .await
    }

    /// Submit a program and wait for it to finish, returning its `result`.
    ///
    /// Polls [`Self::get_execution`] every `initial_delay`, doubling up to
    /// `max_delay`, until the job is terminal or `timeout` elapses — see
    /// [`PollSchedule`]. A script fault is **not** an HTTP error: the submission
    /// still succeeds and the failure lands in the job, so `failed` / `canceled`
    /// are surfaced here as [`SdkError::Other`] carrying the job's `error` text.
    pub async fn execute(&self, body: &JsonValue) -> Result<JsonValue, SdkError> {
        let created = self.create_execution(body).await?;
        let id = created
            .get("id")
            .and_then(JsonValue::as_str)
            .ok_or_else(|| {
                SdkError::Deserialize(format!(
                    "POST /scripting/executions returned no string 'id': {created}"
                ))
            })?
            .to_owned();

        let deadline = tokio::time::Instant::now() + self.poll.timeout;
        let mut delay = self.poll.initial_delay;
        loop {
            tokio::time::sleep(delay).await;

            let job = self.get_execution(&id).await?;
            let status = job
                .get("status")
                .and_then(JsonValue::as_str)
                .unwrap_or_default();
            if is_terminal(status) {
                return terminal_outcome(&job, status);
            }

            if tokio::time::Instant::now() >= deadline {
                return Err(SdkError::Transport(format!(
                    "execution {id} did not finish within {:?}",
                    self.poll.timeout
                )));
            }
            delay = (delay * 2).min(self.poll.max_delay);
        }
    }
}

/// `true` once no further polling can change the job's outcome.
fn is_terminal(status: &str) -> bool {
    matches!(status, "succeeded" | "failed" | "canceled")
}

fn terminal_outcome(job: &JsonValue, status: &str) -> Result<JsonValue, SdkError> {
    if status == "succeeded" {
        return Ok(job.get("result").cloned().unwrap_or(JsonValue::Null));
    }

    let error = job
        .get("error")
        .and_then(JsonValue::as_str)
        .unwrap_or("no error reported");
    Err(SdkError::Other(anyhow!(
        "script execution {status}: {error}"
    )))
}
