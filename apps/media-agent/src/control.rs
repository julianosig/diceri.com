//! Internal control API, called by `diceri-api`. Bind it to a private address.

use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use diceri_domain::RoomId;
use diceri_infra::AGENT_KEY_HEADER;
use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;

use crate::sessions::Sessions;

#[derive(Clone)]
struct ControlState {
    sessions: Arc<Sessions>,
    key: Arc<SecretString>,
}

#[derive(Deserialize)]
struct StartSession {
    room: String,
}

pub(crate) fn router(sessions: Arc<Sessions>, key: SecretString) -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/sessions", post(start_session))
        .with_state(ControlState {
            sessions,
            key: Arc::new(key),
        })
}

async fn start_session(
    State(state): State<ControlState>,
    headers: HeaderMap,
    Json(body): Json<StartSession>,
) -> StatusCode {
    let provided = headers.get(AGENT_KEY_HEADER).and_then(|v| v.to_str().ok());
    if provided != Some(state.key.expose_secret()) {
        return StatusCode::UNAUTHORIZED;
    }
    let Ok(room) = body.room.parse::<RoomId>() else {
        return StatusCode::BAD_REQUEST;
    };
    if state.sessions.ensure(room) {
        StatusCode::ACCEPTED
    } else {
        StatusCode::OK
    }
}
