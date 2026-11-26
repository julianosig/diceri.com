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
    assert_eq!(fetched["title"], "Test room");
    assert_eq!(fetched["entry"], "approval");
    assert!(fetched["owner"].as_str().unwrap().starts_with("host"));
    assert!(fetched.get("hostKey").is_none());
}

#[tokio::test]
async fn creating_a_room_needs_an_account() {
    let body = serde_json::json!({ "title": "Coffee" });
    let (status, _) = send(&app(false), "POST", "/api/v1/rooms", None, Some(body)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn rooms_need_a_title() {
    let app = app(false);
    let cookie = support::sign_up(&app, "ana").await;
    let headers = support::headers(None, Some(&cookie));
    let body = serde_json::json!({ "title": "  " });
    let (status, _, problem) =
        support::exchange(&app, "POST", "/api/v1/rooms", &headers, Some(body)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(problem["detail"], "title must not be empty");
}

#[tokio::test]
async fn too_many_writes_from_one_client_are_rejected() {
    let config = diceri_api::Config {
        write_limit_per_minute: 2,
        client_ip_header: Some("cf-connecting-ip".to_owned()),
        ..diceri_api::Config::for_tests()
    };
    let app = diceri_api::build_router(diceri_api::AppState::in_memory(None), &config);
    let sign_up = |ip: &'static str, username: &'static str| {
        let app = app.clone();
        async move {
            let headers = [("cf-connecting-ip", ip.to_owned())];
            let body = serde_json::json!({ "username": username });
            support::exchange(&app, "POST", "/api/v1/accounts", &headers, Some(body))
                .await
                .0
        }
    };
    assert_eq!(sign_up("203.0.113.1", "spam1").await, StatusCode::CREATED);
    assert_eq!(sign_up("203.0.113.1", "spam2").await, StatusCode::CREATED);
    assert_eq!(
        sign_up("203.0.113.1", "spam3").await,
        StatusCode::TOO_MANY_REQUESTS
    );
    assert_eq!(sign_up("203.0.113.2", "spam4").await, StatusCode::CREATED);
}
