//! Lobby endpoints: host session, guest knock/poll, host decisions, monitor.
//!
//! Paths carry the room's link code. Codes grant access, so they are never
//! logged here; use cases log the room id instead.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use super::access_key::CallerKey;
use super::dto::{
    JoinResponse, KnockResponse, OutcomeResponse, PendingRequestResponse, ProfileRequest,
};
use super::handlers::{parse_request_id, parse_room_code};
use diceri_application::use_cases::KnockOutcome;
use diceri_domain::RequestStatus;

use crate::features::CurrentAccount;
use crate::{ApiError, AppState};

/// Everyone speaks in their own trained voice (ADR 0014): with the requirement on,
/// joining without a recorded voice is refused.
fn require_voice(state: &AppState, body: &ProfileRequest) -> Result<(), ApiError> {
    if state.voice_required && body.voice_id.is_none() {
        return Err(ApiError::Validation(
            "record your voice before joining the call".to_owned(),
        ));
    }
    Ok(())
}

pub(crate) async fn host_session(
    State(state): State<AppState>,
    CurrentAccount(account): CurrentAccount,
    Path(code): Path<String>,
    CallerKey(key): CallerKey,
    Json(body): Json<ProfileRequest>,
) -> Result<Json<JoinResponse>, ApiError> {
    require_voice(&state, &body)?;
    let code = parse_room_code(&code)?;
    let profile = body.profile()?;
    // Only the host, with a voice of their own, takes a place in the service's capacity.
    let room = state
        .admission
        .check_host(&code, &key, &account.id, &profile)
        .await?;
    // The host starting (or rejoining) the call spends the owner's plan minutes.
    state.metering.admit_owner_of(&room).await?;
    let ticket = state
        .admission
        .join_as_host(&code, &key, &account.id, &profile)
        .await?;
    tracing::info!(identity = %ticket.identity, "host joined");
    Ok(Json(ticket.into()))
}

/// Asks to join (or, in an open room, joins): only signed-in accounts may.
pub(crate) async fn knock(
    State(state): State<AppState>,
    CurrentAccount(account): CurrentAccount,
    Path(code): Path<String>,
    Json(body): Json<ProfileRequest>,
) -> Result<(StatusCode, Json<KnockResponse>), ApiError> {
    require_voice(&state, &body)?;
    let knock = state
        .admission
        .knock(&parse_room_code(&code)?, &account.id, body.profile()?)
        .await?;
    tracing::info!(request_id = %knock.request_id, "guest asked to join");
    Ok((StatusCode::CREATED, Json(knock.into())))
}

pub(crate) async fn pending(
    State(state): State<AppState>,
    Path(code): Path<String>,
    CallerKey(key): CallerKey,
) -> Result<Json<Vec<PendingRequestResponse>>, ApiError> {
    let requests = state
        .admission
        .pending(&parse_room_code(&code)?, &key)
        .await?;
    Ok(Json(requests.into_iter().map(Into::into).collect()))
}

pub(crate) async fn outcome(
    State(state): State<AppState>,
    Path((code, request_id)): Path<(String, String)>,
    CallerKey(key): CallerKey,
) -> Result<Json<OutcomeResponse>, ApiError> {
    let (code, request_id) = (parse_room_code(&code)?, parse_request_id(&request_id)?);
    // Credentials for the call hold (or renew) its place in the service's capacity, as for
    // its host, checked before the guest takes a seat: a guest refused here holds nothing.
    let status = state
        .admission
        .request_status(&code, &request_id, &key)
        .await?;
    // Answered from that one look: a host deciding meanwhile is seen at the next poll, so
    // credentials are never issued without the check above.
    match status {
        RequestStatus::Pending => return Ok(Json(KnockOutcome::Pending.into())),
        RequestStatus::Denied => return Ok(Json(KnockOutcome::Denied.into())),
        RequestStatus::Admitted => {}
    }
    state
        .metering
        .admit_owner_of(&state.rooms.get(&code).await?)
        .await?;
    // A decision is final: the request is still admitted here.
    let outcome = state.admission.outcome(&code, &request_id, &key).await?;
    Ok(Json(outcome.into()))
}

pub(crate) async fn admit(
    state: State<AppState>,
    ids: Path<(String, String)>,
    key: CallerKey,
) -> Result<StatusCode, ApiError> {
    decide(state, ids, key, true).await
}

pub(crate) async fn deny(
    state: State<AppState>,
    ids: Path<(String, String)>,
    key: CallerKey,
) -> Result<StatusCode, ApiError> {
    decide(state, ids, key, false).await
}

async fn decide(
    State(state): State<AppState>,
    Path((code, request_id)): Path<(String, String)>,
    CallerKey(key): CallerKey,
    admit: bool,
) -> Result<StatusCode, ApiError> {
    let request_id = parse_request_id(&request_id)?;
    state
        .admission
        .decide(&parse_room_code(&code)?, &key, &request_id, admit)
        .await?;
    tracing::info!(%request_id, admit, "join request decided");
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn monitor_session(
    State(state): State<AppState>,
    Path(code): Path<String>,
    CallerKey(key): CallerKey,
) -> Result<Json<JoinResponse>, ApiError> {
    let ticket = state
        .admission
        .monitor(&parse_room_code(&code)?, &key)
        .await?;
    Ok(Json(ticket.into()))
}

/// Host ends the meeting for everyone (`POST /rooms/{code}/end`).
pub(crate) async fn end_call(
    State(state): State<AppState>,
    Path(code): Path<String>,
    CallerKey(key): CallerKey,
) -> Result<StatusCode, ApiError> {
    state
        .admission
        .end_call(&parse_room_code(&code)?, &key)
        .await?;
    tracing::info!("meeting ended by host");
    Ok(StatusCode::NO_CONTENT)
}
