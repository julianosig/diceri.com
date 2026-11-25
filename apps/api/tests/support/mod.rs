//! Shared helpers for router-level tests (in-process, no network).
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    dead_code,
    reason = "test helpers"
)]

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use diceri_api::{AppState, Config, build_router};
use diceri_application::ports::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant,
};
use diceri_domain::RoomId;
use http_body_util::BodyExt as _;
use serde_json::Value;
use tower::ServiceExt as _;

/// Fake media issuer: the token encodes `identity|name|role` for assertions.
pub(crate) struct FakeIssuer;

impl MediaAccessIssuer for FakeIssuer {
    fn issue(&self, _: &RoomId, p: MediaParticipant<'_>) -> Result<MediaGrant, MediaAccessError> {
        let name = p
            .profile
            .map(|n| n.name.as_str().to_owned())
            .unwrap_or_default();
        Ok(MediaGrant {
            server_url: "wss://media.test".to_owned(),
            token: format!("{}|{name}|{:?}", p.identity, p.role),
        })
    }
}

pub(crate) fn app(media: bool) -> Router {
    let issuer = media.then(|| -> Arc<dyn MediaAccessIssuer> { Arc::new(FakeIssuer) });
    build_router(AppState::in_memory(issuer), &Config::for_tests())
}

/// Sends a request; `key` goes in `x-diceri-key`, `body` is JSON.
pub(crate) async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    key: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(key) = key {
        builder = builder.header("x-diceri-key", key);
    }
    let request = match body {
        Some(json) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json.to_string())),
        None => builder.body(Body::empty()),
    }
    .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// Creates a room and returns `(room_id, host_key)`.
pub(crate) async fn create_room(app: &Router) -> (String, String) {
    let (status, body) = send(app, "POST", "/api/v1/rooms", None, None).await;
    assert_eq!(status, StatusCode::CREATED);
    (
        body["id"].as_str().unwrap().to_owned(),
        body["hostKey"].as_str().unwrap().to_owned(),
    )
}
