//! Username + password accounts (no email). Signing up or in sets an `HttpOnly`
//! session cookie; the username is also the name of the person's room.
//!
//! | Method | Path | Auth |
//! | --- | --- | --- |
//! | POST | `/accounts` | — (`{ username, password, mascot }`) |
//! | POST | `/accounts/session` | — (`{ username, password }`) |
//! | GET | `/accounts/me` | session cookie |
//! | POST | `/accounts/sign-out` | session cookie (optional) |

mod handlers;
mod session;

use axum::Router;
use axum::routing::{get, post};

pub(crate) use session::CurrentAccount;

use crate::AppState;

pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/accounts", post(handlers::register))
        .route("/accounts/session", post(handlers::sign_in))
        .route("/accounts/me", get(handlers::me))
        .route("/accounts/sign-out", post(handlers::sign_out))
}
