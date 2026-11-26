use std::sync::Arc;

use diceri_domain::{AccessKey, Room, RoomCode, RoomSettings, Username};

use crate::ports::{IdGenerator, RandomSource, RoomRepository, RoomRepositoryError};

#[derive(Debug, thiserror::Error)]
pub enum RoomError {
    #[error("room not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
}

/// Creates rooms and looks them up by their link code.
#[derive(Clone)]
pub struct RoomService {
    repository: Arc<dyn RoomRepository>,
    random: Arc<dyn RandomSource>,
    ids: Arc<dyn IdGenerator>,
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
    pub fn new(
        repository: Arc<dyn RoomRepository>,
        random: Arc<dyn RandomSource>,
        ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            repository,
            random,
            ids,
        }
    }

    /// Creates a room owned by `owner`. It gets a UUIDv7 id plus two random
    /// values: the link code and the host key (handed to the creator once).
    pub async fn create(&self, owner: Username, settings: RoomSettings) -> Result<Room, RoomError> {
        let code = RoomCode::from_random_bytes(self.random.random_128());
        let host_key = AccessKey::from_random_bytes(self.random.random_128());
        let room = Room::new(self.ids.room_id(), code, host_key, owner, settings);
        self.repository.insert(room.clone()).await?;
        Ok(room)
    }

    pub async fn get(&self, code: &RoomCode) -> Result<Room, RoomError> {
        self.repository
            .find_by_code(code)
            .await?
            .ok_or(RoomError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{InMemoryRooms, owner, private_room, room_service};

    fn service() -> RoomService {
        room_service(Arc::new(InMemoryRooms::default()))
    }

    #[tokio::test]
    async fn creates_room_that_can_be_fetched_by_its_code() {
        let service = service();
        let room = service.create(owner(), private_room()).await.unwrap();
        assert_eq!(service.get(&room.code).await.unwrap(), room);
    }

    #[tokio::test]
    async fn the_link_code_and_host_key_are_separate_random_values() {
        let room = service().create(owner(), private_room()).await.unwrap();
        assert_ne!(room.host_key().expose(), room.code.as_str());
        assert_ne!(room.code.as_str(), room.id.to_string());
    }

    #[tokio::test]
    async fn unknown_room_is_not_found() {
        let unknown = RoomCode::from_random_bytes([200; 16]);
        assert!(matches!(
            service().get(&unknown).await,
            Err(RoomError::NotFound)
        ));
    }
}
