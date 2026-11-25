use axum::Router;
use axum::routing::get;

use crate::AppState;

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/healthz", get(health))
}

async fn health() -> &'static str {
    "ok"
}
