//! In-memory fakes for unit tests of use cases.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU8, Ordering};

use async_trait::async_trait;
use diceri_domain::{Room, RoomId, RoomSettings};

use crate::ports::{
    DispatchError, MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole,
    RandomSource, RoomRepository, RoomRepositoryError, TranslationDispatcher,
};

/// A private, host-approved room titled "Test room".
pub(crate) fn private_room() -> RoomSettings {
    RoomSettings::private("Test room".parse().unwrap())
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
        self.0.lock().unwrap().insert(room.id.clone(), room);
        Ok(())
    }

    async fn find(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
        Ok(self.0.lock().unwrap().get(id).cloned())
    }

    async fn update(&self, room: Room) -> Result<(), RoomRepositoryError> {
        self.0.lock().unwrap().insert(room.id.clone(), room);
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
        self.0.lock().unwrap().push(room.clone());
        Ok(())
    }
}
