use std::sync::Arc;

use diceri_domain::{Room, RoomId};

use crate::ports::{RoomIdGenerator, RoomRepository, RoomRepositoryError};

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
    id_generator: Arc<dyn RoomIdGenerator>,
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
        id_generator: Arc<dyn RoomIdGenerator>,
    ) -> Self {
        Self {
            repository,
            id_generator,
        }
    }

    pub async fn create(&self) -> Result<Room, RoomError> {
        let room = Room::new(self.id_generator.generate());
        self.repository.insert(room.clone()).await?;
        Ok(room)
    }

    pub async fn get(&self, id: &RoomId) -> Result<Room, RoomError> {
        self.repository.find(id).await?.ok_or(RoomError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;

    struct FixedIdGenerator;

    impl RoomIdGenerator for FixedIdGenerator {
        fn generate(&self) -> RoomId {
            RoomId::from_random_bytes([7; 16])
        }
    }

    #[derive(Default)]
    struct FakeRepository {
        rooms: Mutex<HashMap<RoomId, Room>>,
    }

    #[async_trait]
    impl RoomRepository for FakeRepository {
        async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError> {
            self.rooms.lock().unwrap().insert(room.id.clone(), room);
            Ok(())
        }

        async fn find(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
            Ok(self.rooms.lock().unwrap().get(id).cloned())
        }
    }

    fn service() -> RoomService {
        RoomService::new(
            Arc::new(FakeRepository::default()),
            Arc::new(FixedIdGenerator),
        )
    }

    #[tokio::test]
    async fn creates_room_that_can_be_fetched() {
        let service = service();
        let room = service.create().await.unwrap();
        assert_eq!(service.get(&room.id).await.unwrap(), room);
    }

    #[tokio::test]
    async fn unknown_room_is_not_found() {
        let unknown = RoomId::from_random_bytes([9; 16]);
        assert!(matches!(
            service().get(&unknown).await,
            Err(RoomError::NotFound)
        ));
    }
}
