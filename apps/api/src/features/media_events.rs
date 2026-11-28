//! `POST /api/v1/livekit/webhook`: the media server's signed reports (ADR 0018). Only acts
//! to remove people or close calls, so a forged report could do nothing even if it passed.

use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::routing::post;
use diceri_application::ports::{MediaEventError, MediaEventVerifier};
use diceri_application::use_cases::CallEventsService;

use crate::AppState;

/// Checks reports and acts on them.
#[derive(Clone)]
pub struct MediaEvents {
    verifier: Arc<dyn MediaEventVerifier>,
    events: CallEventsService,
}

impl std::fmt::Debug for MediaEvents {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MediaEvents")
            .finish_non_exhaustive()
    }
}

impl MediaEvents {
    #[must_use]
    pub fn new(verifier: Arc<dyn MediaEventVerifier>, events: CallEventsService) -> Self {
        Self { verifier, events }
    }
}

/// Outside the per-address write limit: every report comes from the media server's few
/// addresses, and its signature is what is checked.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/livekit/webhook", post(receive))
}

async fn receive(State(state): State<AppState>, headers: HeaderMap, body: String) -> StatusCode {
    let Some(media) = state.media_events else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    match media.verifier.verify(&body, authorization) {
        Err(MediaEventError::Unsigned) => StatusCode::UNAUTHORIZED,
        Err(MediaEventError::Unreadable) => StatusCode::BAD_REQUEST,
        Ok(None) => StatusCode::OK,
        Ok(Some(event)) => {
            // Answered at once (the media server waits on it); acting may ask it questions.
            tokio::spawn(async move { media.events.handle(event).await });
            StatusCode::OK
        }
    }
}
