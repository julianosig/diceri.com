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
    assert!(fetched["hostName"].is_null());
    assert!(fetched.get("hostKey").is_none());
}

#[tokio::test]
async fn rooms_need_a_title() {
    let body = serde_json::json!({ "title": "  " });
    let (status, problem) = send(&app(false), "POST", "/api/v1/rooms", None, Some(body)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(problem["detail"], "title must not be empty");
}

#[tokio::test]
async fn the_room_shows_its_host_once_they_join() {
    let app = app(true);
    let body = serde_json::json!({ "title": "Coffee", "listed": true, "entry": "open" });
    let (id, host_key) = support::create_room_with(&app, body).await;
    let profile = serde_json::json!({ "name": "Ana", "outputLanguage": "pt-BR" });
    let join = format!("/api/v1/rooms/{id}/host-session");
    send(&app, "POST", &join, Some(&host_key), Some(profile)).await;

    let (_, room) = send(&app, "GET", &format!("/api/v1/rooms/{id}"), None, None).await;
    assert_eq!(room["hostName"], "Ana");
    assert_eq!(room["entry"], "open");
}

#[tokio::test]
async fn malformed_room_id_is_not_found() {
    let (status, body) = send(&app(false), "GET", "/api/v1/rooms/nope", None, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["status"], 404);
}

#[tokio::test]
async fn too_many_writes_from_one_client_are_rejected() {
    let config = diceri_api::Config {
        write_limit_per_minute: 2,
        client_ip_header: Some("cf-connecting-ip".to_owned()),
        ..diceri_api::Config::for_tests()
    };
    let app = diceri_api::build_router(diceri_api::AppState::in_memory(None), &config);
    let create = |ip: &'static str| {
        let app = app.clone();
        async move {
            let request = axum::http::Request::post("/api/v1/rooms")
                .header("cf-connecting-ip", ip)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"title":"Spam"}"#))
                .unwrap();
            tower::ServiceExt::oneshot(app, request)
                .await
                .unwrap()
                .status()
        }
    };
    assert_eq!(create("203.0.113.1").await, StatusCode::CREATED);
    assert_eq!(create("203.0.113.1").await, StatusCode::CREATED);
    assert_eq!(create("203.0.113.1").await, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(create("203.0.113.2").await, StatusCode::CREATED);
}
