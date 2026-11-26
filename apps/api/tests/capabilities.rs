//! The web app asks what this deployment offers.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use serde_json::json;
use support::{app, app_with_voices, send};

#[tokio::test]
async fn voice_cloning_is_offered_only_when_it_is_set_up() {
    let (status, body) = send(&app(true), "GET", "/api/v1/capabilities", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "voiceCloning": false }));
    let (_, body) = send(
        &app_with_voices(),
        "GET",
        "/api/v1/capabilities",
        None,
        None,
    )
    .await;
    assert_eq!(body, json!({ "voiceCloning": true }));
}
