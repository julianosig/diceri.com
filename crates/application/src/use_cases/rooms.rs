use std::sync::Arc;

use diceri_domain::{AccessKey, Room, RoomId};

use crate::ports::{RandomSource, RoomRepository, RoomRepositoryError};

#[derive(Debug, thiserror::Error)]
pub enum RoomError {
    #[error("room not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
}

/// Creates and looks up anonymous rooms.
#[derive(Clone)]
pub struct RoomService {
    repository: Arc<dyn RoomRepository>,
    random: Arc<dyn RandomSource>,
}

impl std::fmt::Debug for RoomService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RoomService")
            .finish_non_exhaustive()
    }
}

impl RoomService {
    #[must_use]
    pub fn new(repository: Arc<dyn RoomRepository>, random: Arc<dyn RandomSource>) -> Self {
        Self { repository, random }
    }

    /// Creates a room; the returned room's host key must be handed to the creator once.
    pub async fn create(&self) -> Result<Room, RoomError> {
        let id = RoomId::from_random_bytes(self.random.random_128());
        let host_key = AccessKey::from_random_bytes(self.random.random_128());
        let room = Room::new(id, host_key);
        self.repository.insert(room.clone()).await?;
        Ok(room)
    }

    pub async fn get(&self, id: &RoomId) -> Result<Room, RoomError> {
        self.repository.find(id).await?.ok_or(RoomError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{CountingRandom, InMemoryRooms};

    fn service() -> RoomService {
        RoomService::new(
            Arc::new(InMemoryRooms::default()),
            Arc::new(CountingRandom::default()),
        )
    }

    #[tokio::test]
    async fn creates_room_that_can_be_fetched() {
        let service = service();
        let room = service.create().await.unwrap();
        assert_eq!(service.get(&room.id).await.unwrap(), room);
    }

    #[tokio::test]
    async fn host_key_differs_from_room_id() {
        let room = service().create().await.unwrap();
        assert_ne!(room.host_key().expose(), room.id.as_str());
    }

    #[tokio::test]
    async fn unknown_room_is_not_found() {
        let unknown = RoomId::from_random_bytes([200; 16]);
        assert!(matches!(
            service().get(&unknown).await,
            Err(RoomError::NotFound)
        ));
    }
}
