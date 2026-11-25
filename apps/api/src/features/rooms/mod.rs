//! Anonymous rooms: create, look up by shareable id, and join the media session.

mod dto;
mod handlers;

use axum::Router;
use axum::routing::{get, post};

use crate::AppState;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/rooms", post(handlers::create_room))
        .route("/rooms/{room_id}", get(handlers::get_room))
        .route("/rooms/{room_id}/join", post(handlers::join_room))
}
