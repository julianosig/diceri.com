//! `GET /presence`: totals for every room, names only for listed rooms.
#![allow(clippy::unwrap_used, reason = "tests fail loudly")]

mod support;

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::Router;
use axum::http::StatusCode;
use diceri_api::{AppState, Config, build_router};
use diceri_application::ports::{LiveRoom, MediaAccessError, MediaPresence};
use serde_json::json;
use support::{FakeIssuer, create_room_with, send};

#[derive(Default)]
struct FakePresence(Mutex<Vec<LiveRoom>>);

#[async_trait]
impl MediaPresence for FakePresence {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(self.0.lock().unwrap().clone())
    }
}

fn app() -> (Router, Arc<FakePresence>) {
    let media = Arc::new(FakePresence::default());
    let mut state = AppState::in_memory(Some(Arc::new(FakeIssuer)));
    state.presence = state.presence.with_source(media.clone());
    (build_router(state, &Config::for_tests()), media)
}

/// Creates a room, joins as host "Ana" and puts `people` in its media room.
async fn live_room(
    app: &Router,
    media: &FakePresence,
    body: serde_json::Value,
    people: usize,
) -> String {
    let (id, host_key) = create_room_with(app, body).await;
    let profile = json!({ "name": "Ana", "outputLanguage": "pt-BR" });
    let join = format!("/api/v1/rooms/{id}/host-session");
    send(app, "POST", &join, Some(&host_key), Some(profile)).await;
    media.0.lock().unwrap().push(LiveRoom {
        room: id.parse().unwrap(),
        people,
        languages: vec!["pt-BR".parse().unwrap(), "es".parse().unwrap()],
    });
    id
}

#[tokio::test]
async fn ranks_listed_rooms_with_title_host_and_entry() {
    let (app, media) = app();
    let listed = json!({ "title": "Spanish practice", "listed": true, "entry": "open" });
    let id = live_room(&app, &media, listed, 2).await;
    live_room(&app, &media, json!({ "title": "Private call" }), 3).await;

    let (status, body) = send(&app, "GET", "/api/v1/presence", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({
            "online": 5,
            "liveRooms": 2,
            "ranking": [{
                "id": id,
                "title": "Spanish practice",
                "host": "Ana",
                "entry": "open",
                "people": 2,
                "languages": ["pt-BR", "es"]
            }]
        })
    );
}

#[tokio::test]
async fn without_media_nobody_is_online() {
    let (status, body) = send(&support::app(false), "GET", "/api/v1/presence", None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["online"], 0);
}
