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
use crate::{ApiError, AppState};

pub(crate) async fn host_session(
    State(state): State<AppState>,
    Path(code): Path<String>,
    CallerKey(key): CallerKey,
    Json(body): Json<ProfileRequest>,
) -> Result<Json<JoinResponse>, ApiError> {
    let ticket = state
        .admission
        .join_as_host(&parse_room_code(&code)?, &key, &body.profile()?)
        .await?;
    tracing::info!(identity = %ticket.identity, "host joined");
    Ok(Json(ticket.into()))
}

pub(crate) async fn knock(
    State(state): State<AppState>,
    Path(code): Path<String>,
    Json(body): Json<ProfileRequest>,
) -> Result<(StatusCode, Json<KnockResponse>), ApiError> {
    let knock = state
        .admission
        .knock(&parse_room_code(&code)?, body.profile()?)
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
    let outcome = state
        .admission
        .outcome(
            &parse_room_code(&code)?,
            &parse_request_id(&request_id)?,
            &key,
        )
        .await?;
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
