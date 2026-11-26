//! Username + password accounts: sign up with a mascot, stay signed in by cookie,
//! sign in elsewhere, sign out.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use serde_json::{Value, json};
use support::{PASSWORD, app, exchange, headers, send, sign_up};

fn signup(username: &str, password: &str, mascot: &str) -> Value {
    json!({ "username": username, "password": password, "mascot": mascot })
}

#[tokio::test]
async fn signing_up_returns_the_account_and_a_session_cookie() {
    let app = app(false);
    let body = signup("@Ana_92", PASSWORD, "ninja");
    let (status, cookie, account) =
        exchange(&app, "POST", "/api/v1/accounts", &[], Some(body)).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(account, json!({ "username": "ana_92", "mascot": "ninja" }));
    let cookie = cookie.unwrap();
    assert!(cookie.contains("HttpOnly") && cookie.contains("SameSite=Lax"));
    assert!(!cookie.contains(PASSWORD));

    let session = cookie.split(';').next().unwrap().to_owned();
    let me = headers(None, Some(&session));
    let (status, _, account) = exchange(&app, "GET", "/api/v1/accounts/me", &me, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(account["username"], "ana_92");
}

#[tokio::test]
async fn sign_up_is_validated() {
    let app = app(false);
    sign_up(&app, "ana").await;
    let cases = [
        (signup("ANA", PASSWORD, "robot"), StatusCode::CONFLICT),
        (signup("a b", PASSWORD, "robot"), StatusCode::BAD_REQUEST),
        (signup("bob", "short", "robot"), StatusCode::BAD_REQUEST),
        (signup("bob", PASSWORD, "pikachu"), StatusCode::BAD_REQUEST),
    ];
    for (body, expected) in cases {
        let (status, _) = send(&app, "POST", "/api/v1/accounts", None, Some(body)).await;
        assert_eq!(status, expected);
    }
}

#[tokio::test]
async fn signing_in_needs_the_right_password() {
    let app = app(false);
    sign_up(&app, "bob").await;
    let right = json!({ "username": "BOB", "password": PASSWORD });
    let (status, cookie, account) =
        exchange(&app, "POST", "/api/v1/accounts/session", &[], Some(right)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(account["mascot"], "dragon");
    assert!(cookie.is_some());

    for wrong in [
        json!({ "username": "bob", "password": "not the password" }),
        json!({ "username": "nobody", "password": PASSWORD }),
    ] {
        let (status, _) = send(&app, "POST", "/api/v1/accounts/session", None, Some(wrong)).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
}

#[tokio::test]
async fn signing_out_ends_the_session() {
    let app = app(false);
    let session = sign_up(&app, "carla").await;
    let with_session = headers(None, Some(&session));
    let (status, cookie, _) = exchange(
        &app,
        "POST",
        "/api/v1/accounts/sign-out",
        &with_session,
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert!(cookie.unwrap().contains("Max-Age=0"));
    let (status, _, _) = exchange(&app, "GET", "/api/v1/accounts/me", &with_session, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
