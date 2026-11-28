//! Recording a voice and joining with it.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use serde_json::json;
use support::{app, app_with_voices, create_room, post_bytes, post_bytes_as, send, wav};

const SAMPLE: usize = 20_000;
/// A recording long and loud enough to be speech.
fn speech() -> Vec<u8> {
    wav(3_000, true)
}

#[tokio::test]
async fn a_recorded_voice_can_be_used_to_join_the_same_room() {
    let app = app_with_voices();
    let host = support::open_room(&app, json!({})).await;
    let uri = format!("/api/v1/rooms/{}/voices", host.code);
    let (status, body) =
        post_bytes_as(&app, &host.session, &uri, Some("audio/wav"), speech()).await;
    assert_eq!(status, StatusCode::CREATED);
    let voice = body["voiceId"].as_str().unwrap();

    let profile = json!({ "name": "Ana", "outputLanguage": "en", "voiceId": voice });
    let (status, body) = support::host_join(&app, &host, profile).await;
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
    let host = support::open_room(&app, json!({})).await;
    let profile = json!({ "name": "Eve", "outputLanguage": "en", "voiceId": body["voiceId"] });
    let (status, _) = support::host_join(&app, &host, profile).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn someone_elses_voice_in_the_same_room_is_forbidden() {
    let app = app_with_voices();
    let host = support::open_room(&app, json!({})).await;
    // A guest records in this room; the host tries to speak in that voice.
    let uri = format!("/api/v1/rooms/{}/voices", host.code);
    let (_, body) = post_bytes(&app, &uri, Some("audio/wav"), speech()).await;
    let profile = json!({ "name": "Eve", "outputLanguage": "en", "voiceId": body["voiceId"] });
    let (status, _) = support::host_join(&app, &host, profile).await;
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

#[tokio::test]
async fn joining_needs_a_recorded_voice_when_required() {
    let app = support::app_with_voices_required(true);
    let (_, capabilities) = send(&app, "GET", "/api/v1/capabilities", None, None).await;
    assert_eq!(capabilities["voiceRequired"], true);
    let host = support::open_room(&app, json!({})).await;
    let without = json!({ "name": "Ana", "outputLanguage": "pt-BR" });
    let (status, _) = support::host_join(&app, &host, without).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let recorded = format!("/api/v1/rooms/{}/voices", host.code);
    let (_, body) =
        post_bytes_as(&app, &host.session, &recorded, Some("audio/wav"), speech()).await;
    let with = json!({ "name": "Ana", "outputLanguage": "pt-BR", "voiceId": body["voiceId"] });
    let (status, _) = support::host_join(&app, &host, with).await;
    assert_eq!(status, StatusCode::OK);
}

/// Whether `/accounts/me` says a voice recording is kept for the signed-in account.
async fn has_voice(app: &axum::Router, me: &[(&'static str, String)]) -> bool {
    let (_, _, body) = support::exchange(app, "GET", "/api/v1/accounts/me", me, None).await;
    body["hasVoice"].as_bool().unwrap()
}

#[tokio::test]
async fn a_kept_recording_is_reused_and_can_be_deleted() {
    let app = app_with_voices();
    let (id, _) = create_room(&app).await;
    let session = support::sign_up(&app, "keeper").await;
    let me = support::headers(None, Some(&session));
    let kept = format!("/api/v1/rooms/{id}/voices/kept");
    assert!(!has_voice(&app, &me).await);
    assert_eq!(
        support::post_bytes_as(&app, &session, &kept, None, vec![])
            .await
            .0,
        StatusCode::NOT_FOUND
    );

    let record = format!("/api/v1/rooms/{id}/voices");
    let (status, _) =
        support::post_bytes_as(&app, &session, &record, Some("audio/wav"), speech()).await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(has_voice(&app, &me).await);
    let (status, body) = support::post_bytes_as(&app, &session, &kept, None, vec![]).await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(body["voiceId"].is_string());

    let (status, _, _) =
        support::exchange(&app, "DELETE", "/api/v1/accounts/me/voice", &me, None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(!has_voice(&app, &me).await);
}
