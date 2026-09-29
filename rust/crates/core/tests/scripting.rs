//! Integration tests for the Scripting job façade using [`wiremock`].
//!
//! `execute` polls with a 1s→5s backoff and a 15-minute budget, so every test
//! installs a [`PollSchedule`] with zero delays to stay fast.

use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use openapp_sdk_core::resources::PollSchedule;
use openapp_sdk_core::{Client, SdkError};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, Request, Respond, ResponseTemplate,
    matchers::{body_json, method, path, query_param},
};

const TOKEN: &str = "https://api.test_openapp_SECRET";
const EXECUTION_ID: &str = "exec_1";

fn build_client(base_url: &str) -> Client {
    Client::builder()
        .api_key(TOKEN)
        .base_url(base_url)
        .unwrap()
        .default_timeout(Duration::from_secs(2))
        .build()
        .unwrap()
}

/// Poll without waiting and never back off; `timeout` still bounds the loop.
fn instant_poll(timeout: Duration) -> PollSchedule {
    PollSchedule::new(Duration::ZERO, Duration::ZERO, timeout)
}

fn execution_path() -> String {
    format!("/scripting/executions/{EXECUTION_ID}")
}

async fn mount_create(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/scripting/executions"))
        .and(body_json(json!({ "script": "1 + 1" })))
        .respond_with(
            ResponseTemplate::new(202)
                .set_body_json(json!({ "id": EXECUTION_ID, "status": "pending" })),
        )
        .mount(server)
        .await;
}

/// Answers every call with the next body in `bodies`, repeating the last one.
#[derive(Clone)]
struct SequenceResponder {
    calls: Arc<AtomicUsize>,
    bodies: Arc<Vec<Value>>,
}

impl SequenceResponder {
    fn new(bodies: Vec<Value>) -> Self {
        Self {
            calls: Arc::new(AtomicUsize::new(0)),
            bodies: Arc::new(bodies),
        }
    }

    fn call_count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl Respond for SequenceResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let index = self.calls.fetch_add(1, Ordering::SeqCst);
        let body = self
            .bodies
            .get(index)
            .or_else(|| self.bodies.last())
            .cloned()
            .unwrap_or(Value::Null);
        ResponseTemplate::new(200).set_body_json(body)
    }
}

#[tokio::test]
async fn execute_polls_until_succeeded() {
    let server = MockServer::start().await;
    mount_create(&server).await;

    let poller = SequenceResponder::new(vec![
        json!({ "id": EXECUTION_ID, "status": "running" }),
        json!({ "id": EXECUTION_ID, "status": "succeeded", "result": { "answer": 2 } }),
    ]);
    Mock::given(method("GET"))
        .and(path(execution_path()))
        .respond_with(poller.clone())
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    let result = client
        .scripting()
        .with_poll_schedule(instant_poll(Duration::from_secs(30)))
        .execute(&json!({ "script": "1 + 1" }))
        .await
        .unwrap();

    assert_eq!(result, json!({ "answer": 2 }));
    assert_eq!(
        poller.call_count(),
        2,
        "expected one `running` poll followed by the terminal one"
    );
}

#[tokio::test]
async fn execute_surfaces_failed_job_error() {
    let server = MockServer::start().await;
    mount_create(&server).await;

    Mock::given(method("GET"))
        .and(path(execution_path()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": EXECUTION_ID,
            "status": "failed",
            "error": "division by zero"
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    let err = client
        .scripting()
        .with_poll_schedule(instant_poll(Duration::from_secs(30)))
        .execute(&json!({ "script": "1 + 1" }))
        .await
        .unwrap_err();

    assert!(matches!(err, SdkError::Other(_)), "got {err:?}");
    assert!(err.to_string().contains("failed"), "got {err}");
    assert!(err.to_string().contains("division by zero"), "got {err}");
}

#[tokio::test]
async fn execute_surfaces_canceled_status() {
    let server = MockServer::start().await;
    mount_create(&server).await;

    Mock::given(method("GET"))
        .and(path(execution_path()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": EXECUTION_ID,
            "status": "canceled",
            "error": "canceled by user"
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    let err = client
        .scripting()
        .with_poll_schedule(instant_poll(Duration::from_secs(30)))
        .execute(&json!({ "script": "1 + 1" }))
        .await
        .unwrap_err();

    assert!(matches!(err, SdkError::Other(_)), "got {err:?}");
    assert!(err.to_string().contains("canceled"), "got {err}");
}

#[tokio::test]
async fn execute_gives_up_when_job_never_terminates() {
    let server = MockServer::start().await;
    mount_create(&server).await;

    let poller = SequenceResponder::new(vec![json!({
        "id": EXECUTION_ID,
        "status": "running"
    })]);
    Mock::given(method("GET"))
        .and(path(execution_path()))
        .respond_with(poller.clone())
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    let err = client
        .scripting()
        .with_poll_schedule(instant_poll(Duration::from_millis(50)))
        .execute(&json!({ "script": "1 + 1" }))
        .await
        .unwrap_err();

    assert!(matches!(err, SdkError::Transport(_)), "got {err:?}");
    assert!(err.to_string().contains(EXECUTION_ID), "got {err}");
    assert!(
        poller.call_count() >= 1,
        "expected at least one poll before giving up"
    );
}

#[tokio::test]
async fn create_execution_posts_script() {
    let server = MockServer::start().await;
    mount_create(&server).await;

    let client = build_client(&server.uri());
    let job = client
        .scripting()
        .create_execution(&json!({ "script": "1 + 1" }))
        .await
        .unwrap();

    assert_eq!(job["id"], EXECUTION_ID);
    assert_eq!(job["status"], "pending");
}

#[tokio::test]
async fn get_execution_reads_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(execution_path()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": EXECUTION_ID,
            "status": "succeeded",
            "result": 2
        })))
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    let job = client
        .scripting()
        .get_execution(EXECUTION_ID)
        .await
        .unwrap();

    assert_eq!(job["result"], 2);
}

#[tokio::test]
async fn list_executions_sends_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/scripting/executions"))
        .and(query_param("limit", "7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            { "id": EXECUTION_ID, "status": "succeeded" }
        ])))
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    let jobs = client.scripting().list_executions(Some(7)).await.unwrap();

    assert_eq!(jobs.as_array().map(Vec::len), Some(1));
}

#[tokio::test]
async fn list_executions_omits_absent_limit() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/scripting/executions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    assert_eq!(
        client.scripting().list_executions(None).await.unwrap(),
        json!([])
    );

    let requests = server.received_requests().await.expect("request recording");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url.query(), None, "got {}", requests[0].url);
}

#[tokio::test]
async fn cancel_execution_deletes() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(execution_path()))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = build_client(&server.uri());
    client
        .scripting()
        .cancel_execution(EXECUTION_ID)
        .await
        .unwrap();
}
