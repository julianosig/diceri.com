//! Username-only accounts. The key returned on sign-up is the session (an
//! `HttpOnly` cookie) and the code to sign in on another device.
//!
//! | Method | Path | Auth |
//! | --- | --- | --- |
//! | POST | `/accounts` | — (`{ username }`; returns the key once) |
//! | POST | `/accounts/session` | — (`{ key }`: sign in on this device) |
//! | GET | `/accounts/me` | session cookie |
//! | POST | `/accounts/sign-out` | — |

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
