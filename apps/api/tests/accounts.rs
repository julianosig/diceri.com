//! Username-only accounts: sign up, stay signed in by cookie, sign in elsewhere.
#![allow(clippy::unwrap_used, reason = "tests fail loudly")]

mod support;

use axum::http::StatusCode;
use serde_json::json;
use support::{app, exchange, headers, send, sign_up};

#[tokio::test]
async fn signing_up_returns_the_key_once_and_a_session_cookie() {
    let app = app(false);
    let body = json!({ "username": "@Ana_92" });
    let (status, cookie, account) =
        exchange(&app, "POST", "/api/v1/accounts", &[], Some(body)).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(account["username"], "ana_92");
    assert_eq!(account["key"].as_str().unwrap().len(), 22);
    let cookie = cookie.unwrap();
    assert!(cookie.contains("HttpOnly") && cookie.contains("SameSite=Lax"));

    let session = cookie.split(';').next().unwrap().to_owned();
    let (status, _, me) = exchange(
        &app,
        "GET",
        "/api/v1/accounts/me",
        &headers(None, Some(&session)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["username"], "ana_92");
}

#[tokio::test]
async fn usernames_are_unique_and_validated() {
    let app = app(false);
    sign_up(&app, "ana").await;
    let (status, _) = send(
        &app,
        "POST",
        "/api/v1/accounts",
        None,
        Some(json!({ "username": "ANA" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    let (status, problem) = send(
        &app,
        "POST",
        "/api/v1/accounts",
        None,
        Some(json!({ "username": "a b" })),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        problem["detail"]
            .as_str()
            .unwrap()
            .contains("letters, numbers")
    );
}

#[tokio::test]
async fn the_key_signs_in_on_another_device() {
    let app = app(false);
    let body = json!({ "username": "bob" });
    let (_, _, account) = exchange(&app, "POST", "/api/v1/accounts", &[], Some(body)).await;
    let key = json!({ "key": account["key"] });
    let (status, cookie, me) =
        exchange(&app, "POST", "/api/v1/accounts/session", &[], Some(key)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(me["username"], "bob");
    assert!(cookie.is_some());

    let wrong = json!({ "key": "aaaaaaaaaaaaaaaaaaaaaa" });
    let (status, _) = send(&app, "POST", "/api/v1/accounts/session", None, Some(wrong)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn without_a_session_nobody_is_signed_in() {
    let (status, _) = send(&app(false), "GET", "/api/v1/accounts/me", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, cookie, _) =
        exchange(&app(false), "POST", "/api/v1/accounts/sign-out", &[], None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(cookie.unwrap().contains("Max-Age=0"));
}
