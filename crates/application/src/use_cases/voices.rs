use std::sync::Arc;

use diceri_domain::{RoomId, RoomRuleError};

use crate::ports::{
    RoomRepository, RoomRepositoryError, VoiceCloneError, VoiceCloner, VoiceId, VoiceSample,
};

/// Every clone is named `diceri-<deployment>-…` so each deployment (prod, dev, e2e)
/// can find and delete its own orphans without touching the others' live clones.
#[must_use]
pub fn clone_name_prefix(deployment: &str) -> String {
    format!("diceri-{deployment}-")
}

/// Bounds on an uploaded sample: ~5–15 s of compressed speech fits easily.
const MIN_SAMPLE_BYTES: usize = 8 * 1024;
const MAX_SAMPLE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum VoiceError {
    #[error("room not found")]
    NotFound,
    #[error("voice sample must be between 8 KiB and 5 MiB")]
    InvalidSample,
    #[error(transparent)]
    Rule(#[from] RoomRuleError),
    #[error(transparent)]
    Clone(#[from] VoiceCloneError),
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
}

/// Turns a few seconds of consented speech into a voice clone owned by the room.
#[derive(Clone)]
pub struct VoiceService {
    rooms: Arc<dyn RoomRepository>,
    cloner: Arc<dyn VoiceCloner>,
    name_prefix: String,
}

impl std::fmt::Debug for VoiceService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VoiceService")
            .finish_non_exhaustive()
    }
}

impl VoiceService {
    #[must_use]
    pub fn new(
        rooms: Arc<dyn RoomRepository>,
        cloner: Arc<dyn VoiceCloner>,
        deployment: &str,
    ) -> Self {
        Self {
            rooms,
            cloner,
            name_prefix: clone_name_prefix(deployment),
        }
    }

    pub async fn clone_voice(
        &self,
        room_id: &RoomId,
        sample: VoiceSample,
    ) -> Result<VoiceId, VoiceError> {
        if !(MIN_SAMPLE_BYTES..=MAX_SAMPLE_BYTES).contains(&sample.bytes.len()) {
            return Err(VoiceError::InvalidSample);
        }
        let mut room = self
            .rooms
            .find(room_id)
            .await?
            .ok_or(VoiceError::NotFound)?;
        // Check before the paid provider call; registering re-checks afterwards.
        if !room.has_voice_capacity() {
            return Err(VoiceError::Rule(RoomRuleError::TooManyVoices));
        }
        let voice = self
            .cloner
            .create(&format!("{}{room_id}", self.name_prefix), sample)
            .await?;
        room.register_voice(voice.0.clone())?;
        self.rooms.update(room).await?;
        Ok(voice)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::*;
    use crate::test_support::{CountingRandom, InMemoryRooms, owner, private_room};
    use crate::use_cases::RoomService;

    #[derive(Default)]
    struct FakeCloner(Mutex<u32>);

    #[async_trait]
    impl VoiceCloner for FakeCloner {
        /// Echoes the clone name into the id so tests can see how clones are named.
        async fn create(&self, name: &str, _: VoiceSample) -> Result<VoiceId, VoiceCloneError> {
            let mut count = self.0.lock().unwrap();
            *count += 1;
            Ok(VoiceId(format!("{name}#{count}")))
        }
        async fn delete(&self, _: &VoiceId) -> Result<(), VoiceCloneError> {
            Ok(())
        }

        async fn delete_all_named(&self, _: &str) -> Result<usize, VoiceCloneError> {
            Ok(0)
        }
    }

    fn sample(len: usize) -> VoiceSample {
        VoiceSample {
            bytes: vec![0; len],
            content_type: "audio/webm".to_owned(),
        }
    }

    async fn setup() -> (VoiceService, Arc<InMemoryRooms>, RoomId) {
        let repository = Arc::new(InMemoryRooms::default());
        let rooms = RoomService::new(repository.clone(), Arc::new(CountingRandom::default()));
        let room = rooms.create(owner(), private_room()).await.unwrap();
        (
            VoiceService::new(repository.clone(), Arc::new(FakeCloner::default()), "test"),
            repository,
            room.id,
        )
    }

    #[tokio::test]
    async fn cloned_voice_is_owned_by_the_room() {
        let (service, repository, room_id) = setup().await;
        let voice = service.clone_voice(&room_id, sample(20_000)).await.unwrap();
        let room = repository.find(&room_id).await.unwrap().unwrap();
        assert!(room.owns_voice(&voice.0));
    }

    #[tokio::test]
    async fn clones_are_named_after_their_deployment() {
        let (service, _, room_id) = setup().await;
        let voice = service.clone_voice(&room_id, sample(20_000)).await.unwrap();
        assert!(voice.0.starts_with(&clone_name_prefix("test")));
        assert_eq!(clone_name_prefix("prod"), "diceri-prod-");
    }

    #[tokio::test]
    async fn rejects_samples_that_are_too_small_or_too_large() {
        let (service, _, room_id) = setup().await;
        assert!(matches!(
            service.clone_voice(&room_id, sample(100)).await,
            Err(VoiceError::InvalidSample)
        ));
        let huge = sample(MAX_SAMPLE_BYTES + 1);
        assert!(matches!(
            service.clone_voice(&room_id, huge).await,
            Err(VoiceError::InvalidSample)
        ));
    }

    #[tokio::test]
    async fn unknown_room_is_not_found() {
        let (service, _, _) = setup().await;
        let unknown = RoomId::from_random_bytes([200; 16]);
        assert!(matches!(
            service.clone_voice(&unknown, sample(20_000)).await,
            Err(VoiceError::NotFound)
        ));
    }
}
