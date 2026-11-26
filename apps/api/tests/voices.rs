//! Recording a voice and joining with it.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use serde_json::json;
use support::{app, app_with_voices, create_room, post_bytes, send, wav};

const SAMPLE: usize = 20_000;
/// A recording long and loud enough to be speech.
fn speech() -> Vec<u8> {
    wav(3_000, true)
}

#[tokio::test]
async fn a_recorded_voice_can_be_used_to_join_the_same_room() {
    let app = app_with_voices();
    let (id, host_key) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/voices");
    let (status, body) = post_bytes(&app, &uri, Some("audio/wav"), speech()).await;
    assert_eq!(status, StatusCode::CREATED);
    let voice = body["voiceId"].as_str().unwrap();

    let profile = json!({ "name": "Ana", "outputLanguage": "en", "voiceId": voice });
    let join = format!("/api/v1/rooms/{id}/host-session");
    let (status, body) = send(&app, "POST", &join, Some(&host_key), Some(profile)).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body["token"]
            .as_str()
            .unwrap()
            .ends_with(&format!("|{voice}"))
    );
}

#[tokio::test]
async fn a_voice_recorded_in_another_room_is_forbidden() {
    let app = app_with_voices();
    let (other, _) = create_room(&app).await;
    let (_, body) = post_bytes(
        &app,
        &format!("/api/v1/rooms/{other}/voices"),
        Some("audio/wav"),
        speech(),
    )
    .await;
    let (id, host_key) = create_room(&app).await;
    let profile = json!({ "name": "Eve", "outputLanguage": "en", "voiceId": body["voiceId"] });
    let join = format!("/api/v1/rooms/{id}/host-session");
    let (status, _) = send(&app, "POST", &join, Some(&host_key), Some(profile)).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn recordings_must_be_audio_of_a_reasonable_size() {
    let app = app_with_voices();
    let (id, _) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/voices");
    let (status, _) = post_bytes(&app, &uri, Some("text/plain"), vec![1; SAMPLE]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, _) = post_bytes(&app, &uri, Some("audio/webm"), vec![1; 100]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn recording_without_a_cloner_is_unavailable() {
    let app = app(true);
    let (id, _) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/voices");
    let (status, _) = post_bytes(&app, &uri, Some("audio/webm"), vec![1; SAMPLE]).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn a_silent_recording_is_never_cloned() {
    let app = app_with_voices();
    let (id, _) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/voices");
    let (status, body) = post_bytes(&app, &uri, Some("audio/wav"), wav(3_000, false)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(body["detail"].as_str().unwrap().contains("hear you"));
    // Compressed audio cannot be checked, so it is refused too.
    let (status, _) = post_bytes(&app, &uri, Some("audio/webm"), vec![1; SAMPLE]).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}
