//! Voice recording: a few seconds of speech become a voice clone, and the recording is
//! kept with the account to be cloned again later (ADR 0015).

use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode, header};
use diceri_application::ports::VoiceSample;

use super::dto::VoiceResponse;
use super::handlers::parse_room_code;
use crate::features::CurrentAccount;
use crate::{ApiError, AppState};

/// Upload limit for a recording (the use case enforces the real bounds).
pub(crate) const MAX_UPLOAD_BYTES: usize = 6 * 1024 * 1024;

fn voice_service(
    state: &AppState,
) -> Result<&diceri_application::use_cases::VoiceService, ApiError> {
    state
        .voices
        .as_ref()
        .ok_or(ApiError::Unavailable("voice cloning not configured"))
}

pub(crate) async fn record_voice(
    State(state): State<AppState>,
    CurrentAccount(account): CurrentAccount,
    Path(code): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<VoiceResponse>), ApiError> {
    let voices = voice_service(&state)?;
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .filter(|value| value.starts_with("audio/"))
        .ok_or_else(|| ApiError::Validation("Content-Type must be an audio type".to_owned()))?
        .to_owned();
    let sample = VoiceSample {
        bytes: body.to_vec(),
        content_type,
    };
    let voice = voices
        .record_for(&parse_room_code(&code)?, &account.id, sample)
        .await?;
    tracing::info!(account_id = %account.id, "voice recorded");
    Ok((
        StatusCode::CREATED,
        Json(VoiceResponse { voice_id: voice.0 }),
    ))
}

/// Clones the signed-in account's kept recording for this room: no need to record again.
pub(crate) async fn reuse_voice(
    State(state): State<AppState>,
    CurrentAccount(account): CurrentAccount,
    Path(code): Path<String>,
) -> Result<(StatusCode, Json<VoiceResponse>), ApiError> {
    let voice = voice_service(&state)?
        .clone_saved(&parse_room_code(&code)?, &account.id)
        .await?;
    tracing::info!(account_id = %account.id, "kept voice cloned");
    Ok((
        StatusCode::CREATED,
        Json(VoiceResponse { voice_id: voice.0 }),
    ))
}
