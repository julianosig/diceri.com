//! Router-level integration tests (no network, via `tower::ServiceExt::oneshot`).
#![allow(clippy::unwrap_used, reason = "test helpers should fail loudly")]

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use diceri_api::{AppState, Config, build_router};
use http_body_util::BodyExt as _;
use serde_json::Value;
use tower::ServiceExt as _;

fn app() -> Router {
    build_router(AppState::in_memory(None), &Config::for_tests())
}

async fn send(app: Router, method: &str, uri: &str) -> (StatusCode, Value) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, body)
}

#[tokio::test]
async fn health_check_responds_ok() {
    let (status, _) = send(app(), "GET", "/healthz").await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn created_room_can_be_fetched() {
    let app = app();
    let (status, created) = send(app.clone(), "POST", "/api/v1/rooms").await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(created["maxParticipants"], 2);

    let id = created["id"].as_str().unwrap();
    assert_eq!(id.len(), 22);
    let (status, fetched) = send(app, "GET", &format!("/api/v1/rooms/{id}")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["id"], id);
}

#[tokio::test]
async fn malformed_room_id_is_not_found() {
    let (status, body) = send(app(), "GET", "/api/v1/rooms/not-a-room").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["status"], 404);
}
