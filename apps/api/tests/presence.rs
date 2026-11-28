//! `GET /presence`: totals for every room, names only for listed rooms.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

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

    async fn identities_in(
        &self,
        _: &diceri_domain::RoomId,
    ) -> Result<Vec<String>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn listeners_in(
        &self,
        _: &diceri_domain::RoomId,
    ) -> Result<Vec<String>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn open_rooms(&self) -> Result<Vec<diceri_domain::RoomId>, MediaAccessError> {
        Ok(Vec::new())
    }
}

fn app() -> (Router, Arc<FakePresence>) {
    let media = Arc::new(FakePresence::default());
    let mut state = AppState::in_memory(Some(Arc::new(FakeIssuer)));
    state.presence = state.presence.with_source(media.clone());
    (build_router(state, &Config::for_tests()), media)
}

/// Creates a room (owned by a fresh account), puts `people` in its media room
/// (named by the room id, like LiveKit) and returns the room's link code.
async fn live_room(
    app: &Router,
    media: &FakePresence,
    body: serde_json::Value,
    people: usize,
) -> String {
    let (code, _) = create_room_with(app, body).await;
    let (_, room) = send(app, "GET", &format!("/api/v1/rooms/{code}"), None, None).await;
    media.0.lock().unwrap().push(LiveRoom {
        room: room["id"].as_str().unwrap().parse().unwrap(),
        people,
        languages: vec!["pt-BR".parse().unwrap(), "es".parse().unwrap()],
    });
    code
}

#[tokio::test]
async fn ranks_listed_rooms_by_owner_with_mascot_and_entry() {
    let (app, media) = app();
    let listed = json!({ "listed": true, "entry": "open" });
    let code = live_room(&app, &media, listed, 2).await;
    live_room(&app, &media, json!({}), 3).await;

    let (status, body) = send(&app, "GET", "/api/v1/presence", None, None).await;
    assert_eq!(status, StatusCode::OK);
    let (_, room) = send(&app, "GET", &format!("/api/v1/rooms/{code}"), None, None).await;
    let owner = room["owner"].clone();
    assert_eq!(
        body,
        json!({
            "online": 5,
            "liveRooms": 2,
            "ranking": [{
                "code": code,
                "owner": owner,
                "entry": "open",
                "mascot": "dragon",
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
