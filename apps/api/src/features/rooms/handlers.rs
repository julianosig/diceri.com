use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diceri_domain::{RequestId, RoomId};

use super::dto::{CreatedRoomResponse, RoomResponse};
use crate::{ApiError, AppState};

pub(crate) async fn create_room(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<CreatedRoomResponse>), ApiError> {
    let room = state.rooms.create().await?;
    tracing::info!(room_id = %room.id, "room created");
    let body = CreatedRoomResponse {
        room: RoomResponse::from(&room),
        host_key: room.host_key().expose().to_owned(),
    };
    Ok((StatusCode::CREATED, Json(body)))
}

pub(crate) async fn get_room(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<Json<RoomResponse>, ApiError> {
    let room = state.rooms.get(&parse_room_id(&raw_id)?).await?;
    Ok(Json(RoomResponse::from(&room)))
}

pub(crate) fn parse_room_id(raw: &str) -> Result<RoomId, ApiError> {
    raw.parse().map_err(|_| ApiError::NotFound)
}

pub(crate) fn parse_request_id(raw: &str) -> Result<RequestId, ApiError> {
    raw.parse().map_err(|_| ApiError::NotFound)
}
