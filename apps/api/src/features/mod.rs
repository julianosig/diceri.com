//! HTTP features. One folder per feature: `routes`, `handlers`, `dto`.

pub(crate) mod health;
mod rooms;

use axum::Router;

pub(crate) use rooms::ACCESS_KEY_HEADER;

use crate::AppState;

/// Versioned API routes mounted under `/api/v1`.
pub(crate) fn api_routes() -> Router<AppState> {
    Router::new().merge(rooms::routes())
}
