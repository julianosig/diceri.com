use async_trait::async_trait;
use diceri_domain::{Room, RoomCode, RoomId, Username};

#[derive(Debug, thiserror::Error)]
pub enum RoomRepositoryError {
    #[error("room `{0}` already exists")]
    AlreadyExists(RoomId),
    #[error("room `{0}` does not exist")]
    Missing(RoomId),
    #[error("room storage is unavailable: {0}")]
    Unavailable(String),
}

/// Persistence for the room aggregate (including its join requests). Rooms are
/// reached by their link code from the API and by id from the media side.
#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError>;
    async fn find_by_code(&self, code: &RoomCode) -> Result<Option<Room>, RoomRepositoryError>;
    async fn find_by_id(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError>;
    /// The owner's room that has not ended, if any (one live room per account).
    async fn find_live_by_owner(
        &self,
        owner: &Username,
    ) -> Result<Option<Room>, RoomRepositoryError>;
    async fn update(&self, room: Room) -> Result<(), RoomRepositoryError>;
    /// Every room that has not ended, for the per-minute usage meter.
    async fn live_rooms(&self) -> Result<Vec<Room>, RoomRepositoryError>;
}
