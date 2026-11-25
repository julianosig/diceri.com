//! Voice recording: a few seconds of consented speech become a voice clone.

use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode, header};
use diceri_application::ports::VoiceSample;

use super::dto::VoiceResponse;
use super::handlers::parse_room_id;
use crate::{ApiError, AppState};

/// Upload limit for a recording (the use case enforces the real bounds).
pub(crate) const MAX_UPLOAD_BYTES: usize = 6 * 1024 * 1024;

pub(crate) async fn record_voice(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<VoiceResponse>), ApiError> {
    let voices = state
        .voices
        .as_ref()
        .ok_or(ApiError::Unavailable("voice cloning not configured"))?;
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .filter(|value| value.starts_with("audio/"))
        .ok_or_else(|| ApiError::Validation("Content-Type must be an audio type".to_owned()))?
        .to_owned();
    let room_id = parse_room_id(&room_id)?;
    let sample = VoiceSample {
        bytes: body.to_vec(),
        content_type,
    };
    let voice = voices.clone_voice(&room_id, sample).await?;
    tracing::info!(%room_id, "voice recorded");
    Ok((
        StatusCode::CREATED,
        Json(VoiceResponse { voice_id: voice.0 }),
    ))
}
