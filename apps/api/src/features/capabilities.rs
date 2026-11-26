//! What this deployment offers, so the web app shows only what works.
//!
//! | Method | Path | Auth |
//! | --- | --- | --- |
//! | GET | `/capabilities` | — (`{ voiceCloning }`) |

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CapabilitiesResponse {
    /// Recording your own voice for translations (a paid feature, off until it ships).
    voice_cloning: bool,
}

pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/capabilities", get(capabilities))
}

async fn capabilities(State(state): State<AppState>) -> Json<CapabilitiesResponse> {
    Json(CapabilitiesResponse {
        voice_cloning: state.voices.is_some(),
    })
}
