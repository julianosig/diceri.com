//! The web app asks what this deployment offers.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use support::{app, app_with_voices, send};

#[tokio::test]
async fn voice_cloning_is_offered_only_when_it_is_set_up() {
    let (status, body) = send(&app(true), "GET", "/api/v1/capabilities", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["voiceCloning"], false);
    let (_, body) = send(
        &app_with_voices(),
        "GET",
        "/api/v1/capabilities",
        None,
        None,
    )
    .await;
    assert_eq!(body["voiceCloning"], true);
}

#[tokio::test]
async fn lists_every_plan_free_first() {
    let (_, body) = send(&app(false), "GET", "/api/v1/capabilities", None, None).await;
    let free = &body["plans"][0];
    assert_eq!(free["plan"], "free");
    assert_eq!(free["callMinutes"], 60);
    assert_eq!(free["monthlyMinutes"], 60);
    assert_eq!(free["monthlyPriceCents"], 0);
    assert_eq!(body["plans"][1]["monthlyPriceCents"], 2_900);
    assert_eq!(body["plans"].as_array().unwrap().len(), 3);
}

#[tokio::test]
async fn a_private_beta_shows_its_free_spots() {
    let (_, open) = send(&app(false), "GET", "/api/v1/capabilities", None, None).await;
    assert!(open.get("beta").is_none());
    let mut state = diceri_api::AppState::in_memory(None);
    state.accounts = state.accounts.clone().with_max_accounts(Some(3));
    let app = diceri_api::build_router(state, &diceri_api::Config::for_tests());
    support::sign_up(&app, "ana").await;
    let (_, body) = send(&app, "GET", "/api/v1/capabilities", None, None).await;
    assert_eq!(body["beta"]["spots"], 3);
    assert_eq!(body["beta"]["spotsLeft"], 2);
}
