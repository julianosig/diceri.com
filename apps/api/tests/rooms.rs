//! Personal rooms: one per account, named and reached by `@username`.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use serde_json::json;
use support::{app, create_room, exchange, headers, send, sign_up};

#[tokio::test]
async fn health_check_responds_ok() {
    let (status, _) = send(&app(false), "GET", "/healthz", None, None).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn opening_a_room_returns_the_host_key_once() {
    let app = app(false);
    let (code, host_key) = create_room(&app).await;
    assert_eq!(code.len(), 22);
    assert_eq!(host_key.len(), 22);

    let uri = format!("/api/v1/rooms/{code}");
    let (status, fetched) = send(&app, "GET", &uri, None, None).await;
    assert_eq!(status, StatusCode::OK);
    let id = fetched["id"].as_str().unwrap();
    assert_eq!(id.len(), 36);
    assert_eq!(id.as_bytes()[14], b'7', "ids are UUIDv7");
    assert_eq!(fetched["code"], code.as_str());
    assert_eq!(fetched["mascot"], "dragon", "the owner's mascot");
    assert_eq!(fetched["entry"], "approval");
    assert!(fetched["owner"].as_str().unwrap().starts_with("host"));
    assert!(fetched.get("hostKey").is_none());
}

#[tokio::test]
async fn the_room_lives_at_the_owners_username() {
    let app = app(false);
    let (status, _) = send(&app, "GET", "/api/v1/users/ana/room", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "not open yet");

    let session = sign_up(&app, "ana").await;
    let with_session = headers(None, Some(&session));
    let body = json!({ "entry": "open", "listed": true });
    let (status, _, opened) =
        exchange(&app, "POST", "/api/v1/rooms", &with_session, Some(body)).await;
    assert_eq!(status, StatusCode::CREATED);

    let (status, room) = send(&app, "GET", "/api/v1/users/Ana/room", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(room["owner"], "ana");
    assert_eq!(room["code"], opened["code"]);
    assert_eq!(room["entry"], "open");
    // Without a media server nobody can tell who is in the call, so no count is given.
    assert!(room.get("people").is_none());

    // Opening again keeps the same room (and link) with the new settings.
    let (_, _, again) = exchange(
        &app,
        "POST",
        "/api/v1/rooms",
        &with_session,
        Some(json!({})),
    )
    .await;
    assert_eq!(again["code"], opened["code"]);
    assert_eq!(again["entry"], "approval");
}

#[tokio::test]
async fn opening_a_room_needs_an_account() {
    let (status, _) = send(&app(false), "POST", "/api/v1/rooms", None, Some(json!({}))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn too_many_writes_from_one_client_are_rejected() {
    let config = diceri_api::Config {
        write_limit_per_minute: 2,
        client_ip_header: Some("cf-connecting-ip".to_owned()),
        ..diceri_api::Config::for_tests()
    };
    let app = diceri_api::build_router(diceri_api::AppState::in_memory(None), &config);
    let sign_in = |ip: &'static str| {
        let app = app.clone();
        async move {
            let headers = [("cf-connecting-ip", ip.to_owned())];
            let body = json!({ "username": "nobody", "password": "guessing123" });
            exchange(
                &app,
                "POST",
                "/api/v1/accounts/session",
                &headers,
                Some(body),
            )
            .await
            .0
        }
    };
    assert_eq!(sign_in("203.0.113.1").await, StatusCode::UNAUTHORIZED);
    assert_eq!(sign_in("203.0.113.1").await, StatusCode::UNAUTHORIZED);
    assert_eq!(sign_in("203.0.113.1").await, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(sign_in("203.0.113.2").await, StatusCode::UNAUTHORIZED);
}
