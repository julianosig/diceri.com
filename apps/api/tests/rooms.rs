//! Room creation and lookup.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use support::{app, create_room, send};

#[tokio::test]
async fn health_check_responds_ok() {
    let (status, _) = send(&app(false), "GET", "/healthz", None, None).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn created_room_returns_host_key_once() {
    let app = app(false);
    let (id, host_key) = create_room(&app).await;
    assert_eq!(id.len(), 22);
    assert_eq!(host_key.len(), 22);

    let (status, fetched) = send(&app, "GET", &format!("/api/v1/rooms/{id}"), None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched["maxParticipants"], 2);
    assert!(fetched.get("hostKey").is_none());
}

#[tokio::test]
async fn malformed_room_id_is_not_found() {
    let (status, body) = send(&app(false), "GET", "/api/v1/rooms/nope", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["status"], 404);
}
