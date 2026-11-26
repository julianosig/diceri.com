//! `GET /presence`: live activity for the home page. Totals cover every room;
//! only rooms their hosts listed appear by name.

use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use diceri_application::use_cases::{PresenceSnapshot, RankedRoom};
use serde::Serialize;

use super::rooms::dto::EntryDto;
use crate::AppState;

/// Browsers and edges may reuse a snapshot as long as the server does.
const CACHE_CONTROL: &str = "public, max-age=10";

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/presence", get(presence))
}

async fn presence(State(state): State<AppState>) -> impl IntoResponse {
    let snapshot = state.presence.snapshot().await;
    (
        [(header::CACHE_CONTROL, CACHE_CONTROL)],
        Json(PresenceDto::from(snapshot)),
    )
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PresenceDto {
    online: usize,
    live_rooms: usize,
    ranking: Vec<RankedRoomDto>,
}

#[derive(Debug, Serialize)]
struct RankedRoomDto {
    /// Link code of a listed room (its host chose to publish it).
    code: String,
    owner: String,
    entry: EntryDto,
    mascot: &'static str,
    people: usize,
    languages: Vec<String>,
}

impl From<PresenceSnapshot> for PresenceDto {
    fn from(snapshot: PresenceSnapshot) -> Self {
        Self {
            online: snapshot.online,
            live_rooms: snapshot.live_rooms,
            ranking: snapshot
                .ranking
                .into_iter()
                .map(RankedRoomDto::from)
                .collect(),
        }
    }
}

impl From<RankedRoom> for RankedRoomDto {
    fn from(room: RankedRoom) -> Self {
        Self {
            code: room.code.to_string(),

            owner: room.owner.to_string(),
            entry: room.entry.into(),
            mascot: room.mascot.as_str(),
            people: room.people,
            languages: room
                .languages
                .iter()
                .map(|language| language.as_str().to_owned())
                .collect(),
        }
    }
}
