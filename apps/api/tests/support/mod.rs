//! Shared helpers for router-level tests (in-process, no network).
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    dead_code,
    reason = "test helpers"
)]

use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use diceri_api::{AppState, Config, build_router};
use diceri_application::ports::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, VoiceCloneError,
    VoiceCloner, VoiceId, VoiceSample,
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
        let voice = p
            .profile
            .and_then(|n| n.voice_id.clone())
            .unwrap_or_default();
        Ok(MediaGrant {
            server_url: "wss://media.test".to_owned(),
            token: format!("{}|{name}|{:?}|{voice}", p.identity, p.role),
        })
    }
}

/// Clones every sample into `voice-1`, `voice-2`, …
#[derive(Default)]
pub(crate) struct FakeCloner(std::sync::Mutex<u32>);

#[async_trait]
impl VoiceCloner for FakeCloner {
    async fn create(&self, _: &str, _: VoiceSample) -> Result<VoiceId, VoiceCloneError> {
        let mut count = self.0.lock().unwrap();
        *count += 1;
        Ok(VoiceId(format!("voice-{count}")))
    }

    async fn delete(&self, _: &VoiceId) -> Result<(), VoiceCloneError> {
        Ok(())
    }
}

pub(crate) fn app(media: bool) -> Router {
    let issuer = media.then(|| -> Arc<dyn MediaAccessIssuer> { Arc::new(FakeIssuer) });
    build_router(AppState::in_memory(issuer), &Config::for_tests())
}

/// App with media and a fake voice cloner.
pub(crate) fn app_with_voices() -> Router {
    let issuer: Arc<dyn MediaAccessIssuer> = Arc::new(FakeIssuer);
    let cloner: Arc<dyn VoiceCloner> = Arc::new(FakeCloner::default());
    build_router(
        AppState::in_memory_with(Some(issuer), Some(cloner)),
        &Config::for_tests(),
    )
}

/// Posts a raw body with a content type.
pub(crate) async fn post_bytes(
    app: &Router,
    uri: &str,
    content_type: Option<&str>,
    body: Vec<u8>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method("POST").uri(uri);
    if let Some(content_type) = content_type {
        builder = builder.header(header::CONTENT_TYPE, content_type);
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body)).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
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
