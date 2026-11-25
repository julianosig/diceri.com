use async_trait::async_trait;
use diceri_domain::{Room, RoomId};

#[derive(Debug, thiserror::Error)]
pub enum RoomRepositoryError {
    #[error("room `{0}` already exists")]
    AlreadyExists(RoomId),
    #[error("room `{0}` does not exist")]
    Missing(RoomId),
    #[error("room storage is unavailable: {0}")]
    Unavailable(String),
}

/// Persistence for the room aggregate (including its join requests).
#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError>;
    async fn find(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError>;
    async fn update(&self, room: Room) -> Result<(), RoomRepositoryError>;
}
