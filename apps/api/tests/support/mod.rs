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
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, SpeechStream, SpeechToText,
    SpeechToTextError, TranscriptEvent, VoiceCloneError, VoiceCloner, VoiceId, VoiceSample,
};
use diceri_domain::LanguageCode;
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

    async fn delete_all_named(&self, _: &str) -> Result<usize, VoiceCloneError> {
        Ok(0)
    }
}

/// Hears the same sentence in any recording, like a recognizer finalizing at the end.
pub(crate) struct FakeRecognizer;

#[async_trait]
impl SpeechToText for FakeRecognizer {
    async fn open(&self, _: &[LanguageCode]) -> Result<SpeechStream, SpeechToTextError> {
        let (audio, mut incoming) = tokio::sync::mpsc::channel::<Vec<i16>>(8);
        let (events, receiver) = tokio::sync::mpsc::channel(8);
        tokio::spawn(async move {
            while incoming.recv().await.is_some() {}
            let heard = TranscriptEvent::Final {
                text: "The quick brown fox jumps".to_owned(),
                language: None,
            };
            let _ = events.send(Ok(heard)).await;
        });
        Ok(SpeechStream {
            audio,
            events: receiver,
        })
    }
}

/// `ms` of 16 kHz mono 16-bit WAV: a loud square wave (speech-like level) or near silence.
pub(crate) fn wav(ms: usize, loud: bool) -> Vec<u8> {
    let amplitude: i16 = if loud { 4_000 } else { 30 };
    let data: Vec<u8> = (0..ms * 16)
        .map(|i| {
            if (i / 20) % 2 == 0 {
                amplitude
            } else {
                -amplitude
            }
        })
        .flat_map(i16::to_le_bytes)
        .collect();
    let data_len = u32::try_from(data.len()).unwrap();
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    for field in [16_u32.to_le_bytes(), [1, 0, 1, 0], 16_000_u32.to_le_bytes()] {
        out.extend_from_slice(&field);
    }
    out.extend_from_slice(&32_000_u32.to_le_bytes());
    out.extend_from_slice(&[2, 0, 16, 0]);
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    out.extend_from_slice(&data);
    out
}

/// App with media and the given plan limits and shared capacity.
pub(crate) fn app_with_limits(
    plans: diceri_application::use_cases::PlanCatalogue,
    capacity: diceri_application::use_cases::Capacity,
) -> Router {
    let mut state = AppState::in_memory(Some(Arc::new(FakeIssuer)));
    state.metering = state.metering.with_limits(plans, capacity);
    build_router(state, &Config::for_tests())
}

pub(crate) fn app(media: bool) -> Router {
    let issuer = media.then(|| -> Arc<dyn MediaAccessIssuer> { Arc::new(FakeIssuer) });
    build_router(AppState::in_memory(issuer), &Config::for_tests())
}

/// App with media, a fake voice cloner and a recognizer that always hears words.
pub(crate) fn app_with_voices() -> Router {
    app_with_voices_required(false)
}

/// Voice cloning on; `required`: joining needs a recorded voice (ADR 0014).
pub(crate) fn app_with_voices_required(required: bool) -> Router {
    let issuer: Arc<dyn MediaAccessIssuer> = Arc::new(FakeIssuer);
    let cloner: Arc<dyn VoiceCloner> = Arc::new(FakeCloner::default());
    let mut state = AppState::in_memory_with(
        Some(issuer),
        Some(cloner),
        Some(Arc::new(FakeRecognizer)),
        "test",
    );
    state.voice_required = required;
    build_router(state, &Config::for_tests())
}

/// Posts a raw body with a content type, as a freshly signed-up account.
pub(crate) async fn post_bytes(
    app: &Router,
    uri: &str,
    content_type: Option<&str>,
    body: Vec<u8>,
) -> (StatusCode, Value) {
    static UPLOADERS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = UPLOADERS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let session = sign_up(app, &format!("uploader{n}")).await;
    post_bytes_as(app, &session, uri, content_type, body).await
}

/// Posts a raw body as the account signed in with `session` (a cookie pair).
pub(crate) async fn post_bytes_as(
    app: &Router,
    session: &str,
    uri: &str,
    content_type: Option<&str>,
    body: Vec<u8>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::COOKIE, session);
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
    let (status, _, body) = exchange(app, method, uri, &headers(key, None), body).await;
    (status, body)
}

/// Request headers: an access key and/or a session cookie.
pub(crate) fn headers(key: Option<&str>, cookie: Option<&str>) -> Vec<(&'static str, String)> {
    let mut headers = Vec::new();
    if let Some(key) = key {
        headers.push(("x-diceri-key", key.to_owned()));
    }
    if let Some(cookie) = cookie {
        headers.push(("cookie", cookie.to_owned()));
    }
    headers
}

/// Sends a request with arbitrary headers; returns status, `Set-Cookie` and JSON body.
pub(crate) async fn exchange(
    app: &Router,
    method: &str,
    uri: &str,
    headers: &[(&'static str, String)],
    body: Option<Value>,
) -> (StatusCode, Option<String>, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    for (name, value) in headers {
        builder = builder.header(*name, value);
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
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .map(|value| value.to_str().unwrap().to_owned());
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        cookie,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

/// The password every test account uses.
pub(crate) const PASSWORD: &str = "correct horse battery";

/// Signs `username` up (dragon mascot) and returns the `Cookie` header of its session.
pub(crate) async fn sign_up(app: &Router, username: &str) -> String {
    let body = serde_json::json!({
        "username": username,
        "password": PASSWORD,
        "mascot": "dragon"
    });
    let (status, cookie, _) = exchange(app, "POST", "/api/v1/accounts", &[], Some(body)).await;
    assert_eq!(status, StatusCode::CREATED);
    cookie.unwrap().split(';').next().unwrap().to_owned()
}

/// Opens a fresh account's private, host-approved room; returns `(room_code, host_key)`.
pub(crate) async fn create_room(app: &Router) -> (String, String) {
    create_room_with(app, serde_json::json!({})).await
}

/// Signs up a fresh account (`host<n>`), opens its room with a `POST /rooms` body and
/// returns `(room_code, host_key)`; lobby paths take the code.
pub(crate) async fn create_room_with(app: &Router, body: Value) -> (String, String) {
    static ACCOUNTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = ACCOUNTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let cookie = sign_up(app, &format!("host{n}")).await;
    let headers = headers(None, Some(&cookie));
    let (status, _, body) = exchange(app, "POST", "/api/v1/rooms", &headers, Some(body)).await;
    assert_eq!(status, StatusCode::CREATED);
    (
        body["code"].as_str().unwrap().to_owned(),
        body["hostKey"].as_str().unwrap().to_owned(),
    )
}
