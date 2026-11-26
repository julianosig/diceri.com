use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diceri_domain::{RequestId, RoomCode};

use super::dto::{CreateRoomRequest, CreatedRoomResponse, RoomResponse};
use crate::features::CurrentAccount;
use crate::{ApiError, AppState};

pub(crate) async fn create_room(
    State(state): State<AppState>,
    CurrentAccount(owner): CurrentAccount,
    Json(body): Json<CreateRoomRequest>,
) -> Result<(StatusCode, Json<CreatedRoomResponse>), ApiError> {
    let room = state.rooms.create(owner, body.settings()?).await?;
    tracing::info!(room_id = %room.id, "room created");
    let body = CreatedRoomResponse {
        room: RoomResponse::from(&room),
        host_key: room.host_key().expose().to_owned(),
    };
    Ok((StatusCode::CREATED, Json(body)))
}

pub(crate) async fn get_room(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room = state.rooms.get(&parse_room_code(&code)?).await?;
    Ok(Json(RoomResponse::from(&room)))
}

/// Malformed codes are simply not found (no hint about the expected shape).
pub(crate) fn parse_room_code(raw: &str) -> Result<RoomCode, ApiError> {
    raw.parse().map_err(|_| ApiError::NotFound)
}

/// Request ids are UUIDv7; anything else (other versions, garbage) is a 404.
pub(crate) fn parse_request_id(raw: &str) -> Result<RequestId, ApiError> {
    raw.parse().map_err(|_| ApiError::NotFound)
}
