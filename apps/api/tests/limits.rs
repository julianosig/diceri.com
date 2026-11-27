//! Plans and limits: what the home page shows, and calls refused at a limit.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use axum::http::StatusCode;
use diceri_application::use_cases::{Capacity, PlanCatalogue};
use diceri_domain::PlanLimits;
use serde_json::json;
use support::{app, app_with_limits, exchange, headers, sign_up};

#[tokio::test]
async fn the_account_shows_its_plan_and_todays_use() {
    let app = app(false);
    let session = sign_up(&app, "ana").await;
    let me = headers(None, Some(&session));
    let (status, _, account) = exchange(&app, "GET", "/api/v1/accounts/me", &me, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(account["plan"], "free");
    let free = PlanCatalogue::default().free;
    assert_eq!(
        account["usage"],
        json!({
            "usedToday": 0,
            "dailyMinutes": free.daily_minutes,
            "usedThisMonth": 0,
            "monthlyMinutes": free.monthly_minutes,
            "callMinutes": free.call_minutes
        })
    );
}

/// Opens the signed-in account's room; returns the status and body.
async fn open_room(app: &axum::Router, user: &str) -> (StatusCode, serde_json::Value) {
    let session = sign_up(app, user).await;
    let with_session = headers(None, Some(&session));
    let body = json!({ "entry": "approval", "listed": false });
    let (status, _, response) =
        exchange(app, "POST", "/api/v1/rooms", &with_session, Some(body)).await;
    (status, response)
}

#[tokio::test]
async fn no_minutes_left_today_means_no_new_call() {
    let plans = PlanCatalogue {
        free: PlanLimits {
            daily_minutes: 0,
            ..PlanCatalogue::default().free
        },
        ..PlanCatalogue::default()
    };
    let app = app_with_limits(plans, Capacity::default());
    let (status, problem) = open_room(&app, "ana").await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(problem["limit"], "daily");
}

#[tokio::test]
async fn a_full_service_refuses_new_calls() {
    let capacity = Capacity {
        max_live_calls: 1,
        ..Capacity::default()
    };
    let app = app_with_limits(PlanCatalogue::default(), capacity);
    assert_eq!(open_room(&app, "ana").await.0, StatusCode::CREATED);
    let (status, problem) = open_room(&app, "bob").await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(problem["limit"], "busy");
}
