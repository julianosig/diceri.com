//! Join endpoint tests with a fake media issuer (no LiveKit, no network).
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test helpers should fail loudly"
)]

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use diceri_api::{AppState, Config, build_router};
use diceri_application::ports::{MediaAccessError, MediaAccessIssuer, MediaGrant, MediaRole};
use diceri_domain::RoomId;
use http_body_util::BodyExt as _;
use serde_json::Value;
use tower::ServiceExt as _;

struct FakeIssuer;

impl MediaAccessIssuer for FakeIssuer {
    fn issue(
        &self,
        _: &RoomId,
        identity: &str,
        role: MediaRole,
    ) -> Result<MediaGrant, MediaAccessError> {
        Ok(MediaGrant {
            server_url: "wss://media.test".to_owned(),
            token: format!("{identity}:{role:?}"),
        })
    }
}

fn app(media: bool) -> Router {
    let issuer = media.then(|| -> Arc<dyn MediaAccessIssuer> { Arc::new(FakeIssuer) });
    build_router(AppState::in_memory(issuer), &Config::for_tests())
}

async fn post(app: Router, uri: &str, body: Option<&str>) -> (StatusCode, Value) {
    let builder = Request::builder().method("POST").uri(uri);
    let request = match body {
        Some(json) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json.to_owned())),
        None => builder.body(Body::empty()),
    }
    .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

async fn create_room(app: &Router) -> String {
    let (_, room) = post(app.clone(), "/api/v1/rooms", None).await;
    room["id"].as_str().unwrap().to_owned()
}

#[tokio::test]
async fn joining_returns_media_credentials() {
    let app = app(true);
    let id = create_room(&app).await;
    let (status, body) = post(app, &format!("/api/v1/rooms/{id}/join"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["serverUrl"], "wss://media.test");
    assert!(body["identity"].as_str().unwrap().starts_with("p-"));
}

#[tokio::test]
async fn monitor_role_is_forwarded() {
    let app = app(true);
    let id = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/join");
    let (status, body) = post(app, &uri, Some(r#"{"role":"monitor"}"#)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["token"].as_str().unwrap().ends_with(":Monitor"));
}

#[tokio::test]
async fn joining_unknown_room_is_not_found() {
    let (status, _) = post(app(true), "/api/v1/rooms/aaaaaaaaaaaaaaaaaaaaaa/join", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn joining_without_media_server_is_unavailable() {
    let app = app(false);
    let id = create_room(&app).await;
    let (status, body) = post(app, &format!("/api/v1/rooms/{id}/join"), None).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["status"], 503);
}
