use async_trait::async_trait;
use diceri_domain::{Room, RoomId};

#[derive(Debug, thiserror::Error)]
pub enum RoomRepositoryError {
    #[error("room `{0}` already exists")]
    AlreadyExists(RoomId),
    #[error("room storage is unavailable: {0}")]
    Unavailable(String),
}

/// Persistence for rooms.
#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError>;
    async fn find(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError>;
}
