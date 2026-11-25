use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use diceri_application::ports::{RoomRepository, RoomRepositoryError};
use diceri_domain::{Room, RoomId};

/// In-memory room storage for development and tests.
#[derive(Debug, Default)]
pub struct MemoryRoomRepository {
    rooms: RwLock<HashMap<RoomId, Room>>,
}

fn poisoned<T>(_: T) -> RoomRepositoryError {
    RoomRepositoryError::Unavailable("room store lock poisoned".to_owned())
}

#[async_trait]
impl RoomRepository for MemoryRoomRepository {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError> {
        let mut rooms = self.rooms.write().map_err(poisoned)?;
        if rooms.contains_key(&room.id) {
            return Err(RoomRepositoryError::AlreadyExists(room.id));
        }
        rooms.insert(room.id.clone(), room);
        Ok(())
    }

    async fn find(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
        Ok(self.rooms.read().map_err(poisoned)?.get(id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rejects_duplicate_ids() {
        let repository = MemoryRoomRepository::default();
        let room = Room::new(RoomId::from_random_bytes([1; 16]));
        repository.insert(room.clone()).await.unwrap();
        assert!(matches!(
            repository.insert(room).await,
            Err(RoomRepositoryError::AlreadyExists(_))
        ));
    }
}
