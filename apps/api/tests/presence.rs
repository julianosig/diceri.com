//! `GET /presence` exposes only anonymous totals and a size ranking.
#![allow(clippy::unwrap_used, reason = "tests fail loudly")]

mod support;

use std::sync::Arc;

use async_trait::async_trait;
use axum::http::StatusCode;
use diceri_api::{AppState, Config, build_router};
use diceri_application::ports::{LiveRoom, MediaAccessError, MediaPresence};
use diceri_application::use_cases::PresenceService;
use serde_json::json;
use support::send;

struct TwoRooms;

#[async_trait]
impl MediaPresence for TwoRooms {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(vec![
            LiveRoom {
                people: 1,
                languages: vec!["en".parse().unwrap()],
            },
            LiveRoom {
                people: 2,
                languages: vec!["pt-BR".parse().unwrap(), "es".parse().unwrap()],
            },
        ])
    }
}

#[tokio::test]
async fn reports_people_online_and_the_biggest_rooms_first() {
    let mut state = AppState::in_memory(None);
    state.presence = PresenceService::new(Some(Arc::new(TwoRooms)));
    let app = build_router(state, &Config::for_tests());

    let (status, body) = send(&app, "GET", "/api/v1/presence", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({
            "online": 3,
            "liveRooms": 2,
            "ranking": [
                { "people": 2, "languages": ["pt-BR", "es"] },
                { "people": 1, "languages": ["en"] }
            ]
        })
    );
}

#[tokio::test]
async fn without_media_nobody_is_online() {
    let (status, body) = send(&support::app(false), "GET", "/api/v1/presence", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["online"], 0);
}
