use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diceri_domain::RoomId;

use super::dto::{JoinRequest, JoinResponse, RoomResponse};
use crate::{ApiError, AppState};

pub(crate) async fn create_room(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<RoomResponse>), ApiError> {
    let room = state.rooms.create().await?;
    tracing::info!(room_id = %room.id, "room created");
    Ok((StatusCode::CREATED, Json(room.into())))
}

pub(crate) async fn get_room(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room = state.rooms.get(&parse_room_id(&raw_id)?).await?;
    Ok(Json(room.into()))
}

pub(crate) async fn join_room(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
    body: Option<Json<JoinRequest>>,
) -> Result<Json<JoinResponse>, ApiError> {
    let join = state
        .join
        .as_ref()
        .ok_or(ApiError::Unavailable("media server not configured"))?;
    let role = body.map(|Json(request)| request.role).unwrap_or_default();
    let ticket = join.join(&parse_room_id(&raw_id)?, role.into()).await?;
    tracing::info!(identity = %ticket.identity, "participant joined");
    Ok(Json(ticket.into()))
}

fn parse_room_id(raw: &str) -> Result<RoomId, ApiError> {
    raw.parse().map_err(|_| ApiError::NotFound)
}
