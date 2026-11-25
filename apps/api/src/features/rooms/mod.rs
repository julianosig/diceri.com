//! Anonymous rooms and their lobby.
//!
//! | Method | Path | Key (`x-diceri-key`) |
//! | --- | --- | --- |
//! | POST | `/rooms` | — (returns the host key once) |
//! | GET | `/rooms/{room}` | — |
//! | POST | `/rooms/{room}/host-session` | host |
//! | POST | `/rooms/{room}/requests` | — (returns a request key) |
//! | GET | `/rooms/{room}/requests` | host |
//! | GET | `/rooms/{room}/requests/{request}` | request |
//! | POST | `/rooms/{room}/requests/{request}/admit` and `/deny` | host |
//! | POST | `/rooms/{room}/monitor-session` | host or admitted request |

mod access_key;
mod admission;
mod dto;
mod handlers;

use axum::Router;
use axum::routing::{get, post};

use crate::AppState;

pub(crate) use access_key::ACCESS_KEY_HEADER;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/rooms", post(handlers::create_room))
        .route("/rooms/{room_id}", get(handlers::get_room))
        .route(
            "/rooms/{room_id}/host-session",
            post(admission::host_session),
        )
        .route(
            "/rooms/{room_id}/requests",
            post(admission::knock).get(admission::pending),
        )
        .route(
            "/rooms/{room_id}/requests/{request_id}",
            get(admission::outcome),
        )
        .route(
            "/rooms/{room_id}/requests/{request_id}/admit",
            post(admission::admit),
        )
        .route(
            "/rooms/{room_id}/requests/{request_id}/deny",
            post(admission::deny),
        )
        .route(
            "/rooms/{room_id}/monitor-session",
            post(admission::monitor_session),
        )
}
