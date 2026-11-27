//! Lobby flow: host joins, guest knocks, host admits or denies.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use serde_json::json;
use support::{app, create_room, exchange, headers, send, sign_up};

/// A freshly signed-up guest asks to join room `id` (only accounts may).
async fn ask_to_join(
    app: &axum::Router,
    id: &str,
    body: Option<serde_json::Value>,
) -> (StatusCode, serde_json::Value) {
    static GUESTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = GUESTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let session = sign_up(app, &format!("guest{n}")).await;
    let uri = format!("/api/v1/rooms/{id}/requests");
    let (status, _, response) =
        exchange(app, "POST", &uri, &headers(None, Some(&session)), body).await;
    (status, response)
}

#[tokio::test]
async fn only_accounts_may_ask_to_join() {
    let app = app(true);
    let (id, _) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/requests");
    let (status, _) = send(&app, "POST", &uri, None, Some(name("Eve"))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

fn name(value: &str) -> serde_json::Value {
    json!({ "name": value, "outputLanguage": "pt-BR" })
}

#[tokio::test]
async fn host_joins_with_host_key_only() {
    let app = app(true);
    let (id, host_key) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/host-session");

    let (status, body) = send(&app, "POST", &uri, Some(&host_key), Some(name("Ana"))).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["token"].as_str().unwrap().contains("|Ana|Speaker|"));

    let (status, _) = send(&app, "POST", &uri, None, Some(name("Ana"))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let wrong = "aaaaaaaaaaaaaaaaaaaaaa";
    let (status, _) = send(&app, "POST", &uri, Some(wrong), Some(name("Ana"))).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn guest_is_admitted_by_host_and_receives_credentials() {
    let app = app(true);
    let (id, host_key) = create_room(&app).await;
    let (status, knock) = ask_to_join(&app, &id, Some(name("Bruno"))).await;
    assert_eq!(status, StatusCode::CREATED);
    let request_id = knock["requestId"].as_str().unwrap();
    let request_key = knock["requestKey"].as_str().unwrap();
    let poll_uri = format!("/api/v1/rooms/{id}/requests/{request_id}");

    let (_, outcome) = send(&app, "GET", &poll_uri, Some(request_key), None).await;
    assert_eq!(outcome["status"], "pending");
    let (_, pending) = send(
        &app,
        "GET",
        &format!("/api/v1/rooms/{id}/requests"),
        Some(&host_key),
        None,
    )
    .await;
    assert_eq!(pending[0]["name"], "Bruno");
    assert_eq!(pending[0]["outputLanguage"], "pt-BR");

    let (status, _) = send(
        &app,
        "POST",
        &format!("{poll_uri}/admit"),
        Some(&host_key),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (_, outcome) = send(&app, "GET", &poll_uri, Some(request_key), None).await;
    assert_eq!(outcome["status"], "admitted");
    assert!(
        outcome["credentials"]["token"]
            .as_str()
            .unwrap()
            .contains("|Bruno|Speaker|")
    );
}

#[tokio::test]
async fn denied_guest_sees_denied_and_decisions_are_final() {
    let app = app(true);
    let (id, host_key) = create_room(&app).await;
    let (_, knock) = ask_to_join(&app, &id, Some(name("Eve"))).await;
    let poll_uri = format!(
        "/api/v1/rooms/{id}/requests/{}",
        knock["requestId"].as_str().unwrap()
    );

    send(
        &app,
        "POST",
        &format!("{poll_uri}/deny"),
        Some(&host_key),
        None,
    )
    .await;
    let (_, outcome) = send(
        &app,
        "GET",
        &poll_uri,
        Some(knock["requestKey"].as_str().unwrap()),
        None,
    )
    .await;
    assert_eq!(outcome["status"], "denied");
    let (status, _) = send(
        &app,
        "POST",
        &format!("{poll_uri}/admit"),
        Some(&host_key),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn guests_cannot_see_or_decide_requests() {
    let app = app(true);
    let (id, _) = create_room(&app).await;
    let (_, knock) = ask_to_join(&app, &id, Some(name("Eve"))).await;
    let guest_key = knock["requestKey"].as_str().unwrap();

    let (status, _) = send(
        &app,
        "GET",
        &format!("/api/v1/rooms/{id}/requests"),
        Some(guest_key),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let admit = format!(
        "/api/v1/rooms/{id}/requests/{}/admit",
        knock["requestId"].as_str().unwrap()
    );
    let (status, _) = send(&app, "POST", &admit, Some(guest_key), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn invalid_languages_are_rejected() {
    let app = app(true);
    let (id, _) = create_room(&app).await;
    let body = json!({ "name": "Ana", "outputLanguage": "portuguese" });
    let (status, _) = ask_to_join(&app, &id, Some(body)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn invalid_names_are_rejected() {
    let app = app(true);
    let (id, _) = create_room(&app).await;
    let (status, body) = ask_to_join(&app, &id, Some(name("   "))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["detail"], "name must not be empty");
}

#[tokio::test]
async fn monitor_session_requires_membership() {
    let app = app(true);
    let (id, host_key) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/monitor-session");
    let (status, body) = send(&app, "POST", &uri, Some(&host_key), None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["identity"].as_str().unwrap().starts_with("monitor-"));

    let (_, knock) = ask_to_join(&app, &id, Some(name("Eve"))).await;
    let (status, _) = send(&app, "POST", &uri, knock["requestKey"].as_str(), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn joining_without_media_server_is_unavailable() {
    let app = app(false);
    let (id, host_key) = create_room(&app).await;
    let uri = format!("/api/v1/rooms/{id}/host-session");
    let (status, _) = send(&app, "POST", &uri, Some(&host_key), Some(name("Ana"))).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn host_ends_the_meeting_and_nobody_can_join_after() {
    let app = app(true);
    let (id, host_key) = create_room(&app).await;
    let end = format!("/api/v1/rooms/{id}/end");

    let (status, _) = send(&app, "POST", &end, None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = send(&app, "POST", &end, Some(&host_key), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let join = format!("/api/v1/rooms/{id}/host-session");
    let (status, _) = send(&app, "POST", &join, Some(&host_key), Some(name("Ana"))).await;
    assert_eq!(status, StatusCode::GONE);
    let (status, _) = ask_to_join(&app, &id, Some(name("Eve"))).await;
    assert_eq!(status, StatusCode::GONE);
}
