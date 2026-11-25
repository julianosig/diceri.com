use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diceri_domain::RoomId;

use super::dto::RoomResponse;
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
    let id: RoomId = raw_id.parse().map_err(|_| ApiError::NotFound)?;
    let room = state.rooms.get(&id).await?;
    Ok(Json(room.into()))
}
