//! In-memory fakes for unit tests of use cases.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_domain::{AccessKey, AccountId, Room, RoomCode, RoomId, RoomSettings, Username};

use crate::ports::{
    AccountRepository, AccountRepositoryError, DispatchError, IdGenerator, IdParts,
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole, RandomSource,
    RoomRepository, RoomRepositoryError, TranslationDispatcher,
};
use crate::use_cases::RoomService;

/// A private, host-approved room titled "Test room".
pub(crate) fn private_room() -> RoomSettings {
    RoomSettings::private("Test room".parse().unwrap())
}

/// The account that owns test rooms.
pub(crate) fn owner() -> Username {
    "owner".parse().unwrap()
}

/// A room service over `repository` with deterministic randomness and ids.
pub(crate) fn room_service(repository: Arc<dyn RoomRepository>) -> RoomService {
    RoomService::new(
        repository,
        Arc::new(CountingRandom::default()),
        Arc::new(SequentialIds::default()),
    )
}

/// Ids one millisecond apart, so they are distinct and ordered.
#[derive(Default)]
pub(crate) struct SequentialIds(AtomicU64);

impl IdGenerator for SequentialIds {
    fn next_parts(&self) -> IdParts {
        IdParts {
            unix_ms: self.0.fetch_add(1, Ordering::Relaxed) + 1,
            random: [0; 10],
        }
    }
}

/// Accounts keyed by the raw key (fine for tests; real adapters store hashes).
#[derive(Default)]
pub(crate) struct InMemoryAccounts(Mutex<Vec<(Username, AccessKey)>>);

#[async_trait]
impl AccountRepository for InMemoryAccounts {
    async fn create(
        &self,
        _: &AccountId,
        username: &Username,
        key: &AccessKey,
    ) -> Result<(), AccountRepositoryError> {
        let mut accounts = self.0.lock().unwrap();
        if accounts.iter().any(|(name, _)| name == username) {
            return Err(AccountRepositoryError::Taken);
        }
        accounts.push((username.clone(), key.clone()));
        Ok(())
    }

    async fn find_by_key(
        &self,
        key: &AccessKey,
    ) -> Result<Option<Username>, AccountRepositoryError> {
        let accounts = self.0.lock().unwrap();
        Ok(accounts
            .iter()
            .find(|(_, k)| k == key)
            .map(|(name, _)| name.clone()))
    }
}

/// Deterministic randomness: every call returns the next byte value.
#[derive(Default)]
pub(crate) struct CountingRandom(AtomicU8);

impl RandomSource for CountingRandom {
    fn random_128(&self) -> [u8; 16] {
        [self.0.fetch_add(1, Ordering::Relaxed); 16]
    }
}

#[derive(Default)]
pub(crate) struct InMemoryRooms(Mutex<HashMap<RoomId, Room>>);

#[async_trait]
impl RoomRepository for InMemoryRooms {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError> {
        self.0.lock().unwrap().insert(room.id, room);
        Ok(())
    }

    async fn find_by_code(&self, code: &RoomCode) -> Result<Option<Room>, RoomRepositoryError> {
        let rooms = self.0.lock().unwrap();
        Ok(rooms.values().find(|room| &room.code == code).cloned())
    }

    async fn find_by_id(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
        Ok(self.0.lock().unwrap().get(id).cloned())
    }

    async fn update(&self, room: Room) -> Result<(), RoomRepositoryError> {
        self.0.lock().unwrap().insert(room.id, room);
        Ok(())
    }
}

/// Records every issued grant as `(identity, name, role)`.
#[derive(Default)]
pub(crate) struct RecordingIssuer(pub(crate) Mutex<Vec<(String, Option<String>, MediaRole)>>);

impl MediaAccessIssuer for RecordingIssuer {
    fn issue(
        &self,
        _: &RoomId,
        participant: MediaParticipant<'_>,
    ) -> Result<MediaGrant, MediaAccessError> {
        self.0.lock().unwrap().push((
            participant.identity.to_owned(),
            participant
                .profile
                .map(|profile| profile.name.as_str().to_owned()),
            participant.role,
        ));
        Ok(MediaGrant {
            server_url: "wss://media.test".to_owned(),
            token: "jwt".to_owned(),
        })
    }
}

/// Records rooms the media agent was asked to translate.
#[derive(Default)]
pub(crate) struct RecordingDispatcher(pub(crate) Mutex<Vec<RoomId>>);

#[async_trait]
impl TranslationDispatcher for RecordingDispatcher {
    async fn start(&self, room: &RoomId) -> Result<(), DispatchError> {
        self.0.lock().unwrap().push(*room);
        Ok(())
    }
}
