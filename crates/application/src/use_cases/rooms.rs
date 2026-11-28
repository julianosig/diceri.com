use std::sync::Arc;

use diceri_domain::{AccessKey, Account, Room, RoomCode, RoomRuleError, RoomSettings, Username};

/// Looks at the owner's live room again this many times when it changes underneath.
const OPEN_ATTEMPTS: usize = 3;

use crate::ports::{
    IdGenerator, RandomSource, RoomChangeError, RoomRepository, RoomRepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum RoomError {
    #[error("room not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
}

impl From<RoomChangeError> for RoomError {
    fn from(error: RoomChangeError) -> Self {
        match error {
            RoomChangeError::Repository(repository) => Self::Repository(repository),
            // Settings changes are not refused by the room's rules; report it as storage.
            RoomChangeError::Rule(rule) => {
                Self::Repository(RoomRepositoryError::Unavailable(rule.to_string()))
            }
        }
    }
}

/// Personal rooms: every account has one room, named after it (`/@username`).
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

    /// Opens the owner's room with `settings` (its mascot is the owner's) for up to
    /// `participants` people (their plan's call size). If the room is already live it is
    /// reused with the new settings, so its link, code and host key stay valid. The host key
    /// must be handed only to the owner.
    pub async fn open(
        &self,
        owner: &Account,
        settings: RoomSettings,
        participants: u8,
    ) -> Result<Room, RoomError> {
        let settings = RoomSettings {
            mascot: owner.mascot,
            ..settings
        };
        // A live room can end between finding it and reusing it (another tab, the plan
        // limit): then look again, and create a new one if none is live.
        for _ in 0..OPEN_ATTEMPTS {
            if let Some(room) = self.repository.find_live_by_owner(&owner.username).await? {
                if let Some(room) = self.reuse(&room, settings, participants).await? {
                    return Ok(room);
                }
                continue;
            }
            let room = self.new_room(owner, settings, participants);
            match self.repository.insert(room.clone()).await {
                Ok(()) => return Ok(room),
                // Another request (a second tab) opened it first: use that room.
                Err(RoomRepositoryError::OwnerHasLiveRoom) => {}
                Err(error) => return Err(error.into()),
            }
        }
        Err(RoomError::NotFound)
    }

    fn new_room(&self, owner: &Account, settings: RoomSettings, participants: u8) -> Room {
        let code = RoomCode::from_random_bytes(self.random.random_128());
        let host_key = AccessKey::from_random_bytes(self.random.random_128());
        let mut room = Room::new(
            self.ids.room_id(),
            code,
            host_key,
            owner.username.clone(),
            settings,
        );
        room.max_participants = participants;
        room.reserve_owner_seat(owner.id);
        room
    }

    /// The owner's live room with `settings` applied (its link and host key stay valid), or
    /// `None` when it ended meanwhile.
    async fn reuse(
        &self,
        room: &Room,
        settings: RoomSettings,
        participants: u8,
    ) -> Result<Option<Room>, RoomError> {
        let apply = move |room: &mut Room| {
            room.ensure_open()?;
            room.settings = settings;
            room.max_participants = participants;
            Ok(())
        };
        match self.repository.modify(&room.id, Box::new(apply)).await {
            Ok(room) => Ok(Some(room)),
            Err(RoomChangeError::Rule(RoomRuleError::Ended)) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    /// Where rooms are kept, for the services that act on them (media events).
    #[must_use]
    pub fn repository(&self) -> Arc<dyn RoomRepository> {
        self.repository.clone()
    }

    /// The live room at `/@username`.
    pub async fn live(&self, owner: &Username) -> Result<Room, RoomError> {
        self.repository
            .find_live_by_owner(owner)
            .await?
            .ok_or(RoomError::NotFound)
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
    use diceri_domain::{EntryPolicy, Mascot};

    use super::*;
    use crate::test_support::{InMemoryRooms, owner, private_room, room_service};

    fn service() -> RoomService {
        room_service(Arc::new(InMemoryRooms::default()))
    }

    #[tokio::test]
    async fn the_room_is_named_after_its_owner_and_shows_their_mascot() {
        let service = service();
        let room = service.open(&owner(), private_room(), 2).await.unwrap();
        assert_eq!(room.owner, owner().username);
        assert_eq!(room.settings.mascot, Mascot::Robot);
        assert_eq!(service.live(&owner().username).await.unwrap(), room);
        assert_eq!(service.get(&room.code).await.unwrap(), room);
    }

    #[tokio::test]
    async fn opening_again_reuses_the_live_room_with_new_settings() {
        let service = service();
        let first = service.open(&owner(), private_room(), 2).await.unwrap();
        let open = RoomSettings {
            entry: EntryPolicy::Open,
            ..private_room()
        };
        let again = service.open(&owner(), open, 2).await.unwrap();
        assert_eq!(again.code, first.code);
        assert_eq!(again.settings.entry, EntryPolicy::Open);
    }

    /// Answers the next owner lookup with a scripted, outdated result: what another tab saw
    /// just before the room changed underneath it.
    #[derive(Default)]
    struct LateLookup {
        inner: InMemoryRooms,
        next: std::sync::Mutex<Option<Outdated>>,
    }

    enum Outdated {
        /// The live room was not stored yet.
        Missed,
        /// The room was live, and has ended since.
        Found(Room),
    }

    #[async_trait::async_trait]
    impl RoomRepository for LateLookup {
        async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError> {
            self.inner.insert(room).await
        }
        async fn find_by_code(&self, code: &RoomCode) -> Result<Option<Room>, RoomRepositoryError> {
            self.inner.find_by_code(code).await
        }
        async fn find_by_id(
            &self,
            id: &diceri_domain::RoomId,
        ) -> Result<Option<Room>, RoomRepositoryError> {
            self.inner.find_by_id(id).await
        }
        async fn find_live_by_owner(
            &self,
            owner: &Username,
        ) -> Result<Option<Room>, RoomRepositoryError> {
            match self.next.lock().unwrap().take() {
                Some(Outdated::Missed) => return Ok(None),
                Some(Outdated::Found(room)) => return Ok(Some(room)),
                None => {}
            }
            self.inner.find_live_by_owner(owner).await
        }
        async fn modify(
            &self,
            id: &diceri_domain::RoomId,
            change: crate::ports::RoomChange<'_>,
        ) -> Result<Room, RoomChangeError> {
            self.inner.modify(id, change).await
        }
        async fn live_rooms(&self) -> Result<Vec<Room>, RoomRepositoryError> {
            self.inner.live_rooms().await
        }
    }

    #[tokio::test]
    async fn two_tabs_opening_at_once_share_one_room() {
        let repository = Arc::new(LateLookup::default());
        let service = room_service(repository.clone());
        let stored = service.open(&owner(), private_room(), 2).await.unwrap();
        // The first tab's room is stored, but the second tab's lookup missed it.
        *repository.next.lock().unwrap() = Some(Outdated::Missed);
        let second = service.open(&owner(), private_room(), 2).await.unwrap();
        assert_eq!(second.code, stored.code);
        assert_eq!(repository.live_rooms().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn a_room_ended_while_reopening_is_replaced_by_a_new_one() {
        let repository = Arc::new(LateLookup::default());
        let service = room_service(repository.clone());
        let first = service.open(&owner(), private_room(), 2).await.unwrap();
        let end = |room: &mut Room| {
            room.end();
            Ok(())
        };
        repository.modify(&first.id, Box::new(end)).await.unwrap();
        // Another tab found it live a moment before it ended.
        *repository.next.lock().unwrap() = Some(Outdated::Found(first.clone()));
        let reopened = service.open(&owner(), private_room(), 2).await.unwrap();
        assert_ne!(reopened.code, first.code);
        assert!(!reopened.is_ended());
    }

    #[tokio::test]
    async fn the_link_code_and_host_key_are_separate_random_values() {
        let room = service().open(&owner(), private_room(), 2).await.unwrap();
        assert_ne!(room.host_key().expose(), room.code.as_str());
        assert_ne!(room.code.as_str(), room.id.to_string());
    }

    #[tokio::test]
    async fn nobody_is_live_until_they_open_their_room() {
        assert!(matches!(
            service().live(&owner().username).await,
            Err(RoomError::NotFound)
        ));
    }
}
