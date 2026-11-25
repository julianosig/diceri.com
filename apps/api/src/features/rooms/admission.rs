//! Lobby endpoints: host session, guest knock/poll, host decisions, monitor.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use super::access_key::CallerKey;
use super::dto::{
    JoinResponse, KnockResponse, NameRequest, OutcomeResponse, PendingRequestResponse,
};
use super::handlers::{parse_request_id, parse_room_id};
use crate::{ApiError, AppState};

pub(crate) async fn host_session(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    CallerKey(key): CallerKey,
    Json(body): Json<NameRequest>,
) -> Result<Json<JoinResponse>, ApiError> {
    let room_id = parse_room_id(&room_id)?;
    let ticket = state
        .admission
        .join_as_host(&room_id, &key, &body.display_name()?)
        .await?;
    tracing::info!(%room_id, identity = %ticket.identity, "host joined");
    Ok(Json(ticket.into()))
}

pub(crate) async fn knock(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    Json(body): Json<NameRequest>,
) -> Result<(StatusCode, Json<KnockResponse>), ApiError> {
    let room_id = parse_room_id(&room_id)?;
    let knock = state
        .admission
        .knock(&room_id, body.display_name()?)
        .await?;
    tracing::info!(%room_id, request_id = %knock.request_id, "guest asked to join");
    Ok((StatusCode::CREATED, Json(knock.into())))
}

pub(crate) async fn pending(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    CallerKey(key): CallerKey,
) -> Result<Json<Vec<PendingRequestResponse>>, ApiError> {
    let requests = state
        .admission
        .pending(&parse_room_id(&room_id)?, &key)
        .await?;
    Ok(Json(requests.into_iter().map(Into::into).collect()))
}

pub(crate) async fn outcome(
    State(state): State<AppState>,
    Path((room_id, request_id)): Path<(String, String)>,
    CallerKey(key): CallerKey,
) -> Result<Json<OutcomeResponse>, ApiError> {
    let outcome = state
        .admission
        .outcome(
            &parse_room_id(&room_id)?,
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
    Path((room_id, request_id)): Path<(String, String)>,
    CallerKey(key): CallerKey,
    admit: bool,
) -> Result<StatusCode, ApiError> {
    let (room_id, request_id) = (parse_room_id(&room_id)?, parse_request_id(&request_id)?);
    state
        .admission
        .decide(&room_id, &key, &request_id, admit)
        .await?;
    tracing::info!(%room_id, %request_id, admit, "join request decided");
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn monitor_session(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
    CallerKey(key): CallerKey,
) -> Result<Json<JoinResponse>, ApiError> {
    let ticket = state
        .admission
        .monitor(&parse_room_id(&room_id)?, &key)
        .await?;
    Ok(Json(ticket.into()))
}
