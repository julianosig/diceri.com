use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diceri_domain::{RequestId, RoomCode, Username};

use super::dto::{CreatedRoomResponse, OpenRoomRequest, RoomResponse};
use crate::features::CurrentAccount;
use crate::{ApiError, AppState};

/// Opens (or reopens with new settings) the signed-in owner's room.
pub(crate) async fn open_room(
    State(state): State<AppState>,
    CurrentAccount(owner): CurrentAccount,
    Json(body): Json<OpenRoomRequest>,
) -> Result<(StatusCode, Json<CreatedRoomResponse>), ApiError> {
    let room = state.rooms.open(&owner, body.settings()).await?;
    tracing::info!(room_id = %room.id, "room opened");
    let body = CreatedRoomResponse {
        room: RoomResponse::from(&room),
        host_key: room.host_key().expose().to_owned(),
    };
    Ok((StatusCode::CREATED, Json(body)))
}

/// The live room at `/@username`; 404 when its owner has not opened it.
pub(crate) async fn user_room(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<RoomResponse>, ApiError> {
    let owner: Username = username.parse().map_err(|_| ApiError::NotFound)?;
    let room = state.rooms.live(&owner).await?;
    Ok(Json(RoomResponse::from(&room)))
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
