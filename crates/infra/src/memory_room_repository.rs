use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use diceri_application::ports::{RoomRepository, RoomRepositoryError};
use diceri_domain::{Room, RoomCode, RoomId, Username};

/// In-memory room storage for development and tests, indexed by id and link code.
#[derive(Debug, Default)]
pub struct MemoryRoomRepository {
    rooms: RwLock<HashMap<RoomId, Room>>,
    /// Link code → id; a room's code never changes.
    codes: RwLock<HashMap<RoomCode, RoomId>>,
}

fn poisoned<T>(_: T) -> RoomRepositoryError {
    RoomRepositoryError::Unavailable("room store lock poisoned".to_owned())
}

#[async_trait]
impl RoomRepository for MemoryRoomRepository {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError> {
        let mut rooms = self.rooms.write().map_err(poisoned)?;
        let mut codes = self.codes.write().map_err(poisoned)?;
        if rooms.contains_key(&room.id) || codes.contains_key(&room.code) {
            return Err(RoomRepositoryError::AlreadyExists(room.id));
        }
        codes.insert(room.code.clone(), room.id);
        rooms.insert(room.id, room);
        Ok(())
    }

    async fn find_by_code(&self, code: &RoomCode) -> Result<Option<Room>, RoomRepositoryError> {
        let rooms = self.rooms.read().map_err(poisoned)?;
        let id = self.codes.read().map_err(poisoned)?.get(code).copied();
        Ok(id.and_then(|id| rooms.get(&id).cloned()))
    }

    async fn find_by_id(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
        Ok(self.rooms.read().map_err(poisoned)?.get(id).cloned())
    }

    async fn find_live_by_owner(
        &self,
        owner: &Username,
    ) -> Result<Option<Room>, RoomRepositoryError> {
        let rooms = self.rooms.read().map_err(poisoned)?;
        Ok(rooms
            .values()
            .find(|room| &room.owner == owner && !room.is_ended())
            .cloned())
    }

    async fn live_rooms(&self) -> Result<Vec<Room>, RoomRepositoryError> {
        let rooms = self.rooms.read().map_err(poisoned)?;
        Ok(rooms
            .values()
            .filter(|room| !room.is_ended())
            .cloned()
            .collect())
    }

    async fn update(&self, room: Room) -> Result<(), RoomRepositoryError> {
        let mut rooms = self.rooms.write().map_err(poisoned)?;
        let slot = rooms
            .get_mut(&room.id)
            .ok_or(RoomRepositoryError::Missing(room.id))?;
        *slot = room;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use diceri_domain::{AccessKey, RoomSettings};

    use super::*;

    fn room() -> Room {
        Room::new(
            RoomId::from_parts(1, [1; 10]),
            RoomCode::from_random_bytes([3; 16]),
            AccessKey::from_random_bytes([2; 16]),
            "ana".parse().unwrap(),
            RoomSettings::default(),
        )
    }

    #[tokio::test]
    async fn update_requires_existing_room() {
        let repository = MemoryRoomRepository::default();
        assert!(matches!(
            repository.update(room()).await,
            Err(RoomRepositoryError::Missing(_))
        ));
        repository.insert(room()).await.unwrap();
        assert!(repository.update(room()).await.is_ok());
    }

    #[tokio::test]
    async fn finds_rooms_by_code_and_by_id() {
        let repository = MemoryRoomRepository::default();
        let room = room();
        repository.insert(room.clone()).await.unwrap();
        assert_eq!(
            repository.find_by_code(&room.code).await.unwrap(),
            Some(room.clone())
        );
        assert_eq!(repository.find_by_id(&room.id).await.unwrap(), Some(room));
    }

    #[tokio::test]
    async fn rejects_duplicate_ids() {
        let repository = MemoryRoomRepository::default();
        let room = room();
        repository.insert(room.clone()).await.unwrap();
        assert!(matches!(
            repository.insert(room).await,
            Err(RoomRepositoryError::AlreadyExists(_))
        ));
    }
}
