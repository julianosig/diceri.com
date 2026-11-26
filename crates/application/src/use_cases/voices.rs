use std::sync::Arc;

use diceri_domain::{RoomCode, RoomRuleError};

use super::speech_check::{MIN_LETTERS, letters_heard};
use super::voice_sample::{MIN_VOICED_MS, speech_window, voiced_ms, wav_samples};
use crate::ports::{
    RoomRepository, RoomRepositoryError, SpeechToText, SpeechToTextError, VoiceCloneError,
    VoiceCloner, VoiceId, VoiceSample,
};

/// Every clone is named `diceri-<deployment>-…` so each deployment (prod, dev, e2e)
/// can find and delete its own orphans without touching the others' live clones.
#[must_use]
pub fn clone_name_prefix(deployment: &str) -> String {
    format!("diceri-{deployment}-")
}

/// Bounds on an uploaded sample: 5–15 s of 16 kHz mono 16-bit WAV is 160–480 KiB.
const MIN_SAMPLE_BYTES: usize = 8 * 1024;
const MAX_SAMPLE_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum VoiceError {
    #[error("room not found")]
    NotFound,
    #[error("voice sample must be 16 kHz mono 16-bit WAV between 8 KiB and 5 MiB")]
    InvalidSample,
    /// The recording holds no clear speech (silence, noise, music): nothing is cloned.
    #[error("no clear speech in the recording")]
    NoSpeech,
    /// Speech could not be checked, so nothing is sent to the cloning provider.
    #[error(transparent)]
    SpeechCheck(#[from] SpeechToTextError),
    #[error(transparent)]
    Rule(#[from] RoomRuleError),
    #[error(transparent)]
    Clone(#[from] VoiceCloneError),
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
}

/// Turns a few seconds of consented speech into a voice clone owned by the room. Only a
/// recording that is clearly someone speaking reaches the cloning provider.
#[derive(Clone)]
pub struct VoiceService {
    rooms: Arc<dyn RoomRepository>,
    cloner: Arc<dyn VoiceCloner>,
    recognizer: Arc<dyn SpeechToText>,
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
        recognizer: Arc<dyn SpeechToText>,
        deployment: &str,
    ) -> Self {
        Self {
            rooms,
            cloner,
            recognizer,
            name_prefix: clone_name_prefix(deployment),
        }
    }

    pub async fn clone_voice(
        &self,
        code: &RoomCode,
        sample: VoiceSample,
    ) -> Result<VoiceId, VoiceError> {
        if !(MIN_SAMPLE_BYTES..=MAX_SAMPLE_BYTES).contains(&sample.bytes.len()) {
            return Err(VoiceError::InvalidSample);
        }
        // Free checks first: the format, then enough loud audio to be speech at all.
        let samples = wav_samples(&sample.bytes).ok_or(VoiceError::InvalidSample)?;
        if voiced_ms(&samples) < MIN_VOICED_MS {
            return Err(VoiceError::NoSpeech);
        }
        let mut room = self
            .rooms
            .find_by_code(code)
            .await?
            .ok_or(VoiceError::NotFound)?;
        // Check before the paid provider call; registering re-checks afterwards.
        if !room.has_voice_capacity() {
            return Err(VoiceError::Rule(RoomRuleError::TooManyVoices));
        }
        // The recognizer must hear words: noise or music that passed the loudness check
        // still never reaches the cloning provider.
        let started = std::time::Instant::now();
        let letters = letters_heard(self.recognizer.as_ref(), speech_window(&samples)).await?;
        let elapsed_ms = started.elapsed().as_millis();
        tracing::info!(room_id = %room.id, elapsed_ms, speech = letters >= MIN_LETTERS, "voice sample checked");
        if letters < MIN_LETTERS {
            return Err(VoiceError::NoSpeech);
        }
        // Named by the room id: the provider never sees the room's link code.
        let voice = self
            .cloner
            .create(&format!("{}{}", self.name_prefix, room.id), sample)
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
    use crate::test_support::{InMemoryRooms, owner, private_room, room_service};
    use crate::use_cases::speech_check::fake::FakeRecognizer;
    use crate::use_cases::voice_sample::{tone, wav};

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
            content_type: "audio/wav".to_owned(),
        }
    }

    /// Three seconds of 16 kHz WAV, voiced or near silent.
    fn recording(loud: bool) -> VoiceSample {
        VoiceSample {
            bytes: wav(&tone(3_000, loud)),
            content_type: "audio/wav".to_owned(),
        }
    }

    async fn setup() -> (VoiceService, Arc<InMemoryRooms>, RoomCode) {
        setup_hearing("The quick brown fox jumps").await
    }

    /// A service whose recognizer hears `heard` in any recording.
    async fn setup_hearing(heard: &'static str) -> (VoiceService, Arc<InMemoryRooms>, RoomCode) {
        let repository = Arc::new(InMemoryRooms::default());
        let rooms = room_service(repository.clone());
        let room = rooms.open(&owner(), private_room()).await.unwrap();
        (
            VoiceService::new(
                repository.clone(),
                Arc::new(FakeCloner::default()),
                Arc::new(FakeRecognizer(heard)),
                "test",
            ),
            repository,
            room.code,
        )
    }

    #[tokio::test]
    async fn cloned_voice_is_owned_by_the_room() {
        let (service, repository, room_id) = setup().await;
        let voice = service
            .clone_voice(&room_id, recording(true))
            .await
            .unwrap();
        let room = repository.find_by_code(&room_id).await.unwrap().unwrap();
        assert!(room.owns_voice(&voice.0));
    }

    #[tokio::test]
    async fn clones_are_named_after_their_deployment() {
        let (service, _, room_id) = setup().await;
        let voice = service
            .clone_voice(&room_id, recording(true))
            .await
            .unwrap();
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
        let unknown = RoomCode::from_random_bytes([200; 16]);
        assert!(matches!(
            service.clone_voice(&unknown, recording(true)).await,
            Err(VoiceError::NotFound)
        ));
    }

    #[tokio::test]
    async fn silence_or_noise_is_never_cloned() {
        let (service, _, room_id) = setup().await;
        assert!(matches!(
            service.clone_voice(&room_id, recording(false)).await,
            Err(VoiceError::NoSpeech)
        ));
        // Loud, but the recognizer hears no words (music, noise, breathing).
        let (service, _, room_id) = setup_hearing("").await;
        assert!(matches!(
            service.clone_voice(&room_id, recording(true)).await,
            Err(VoiceError::NoSpeech)
        ));
    }

    #[tokio::test]
    async fn only_16khz_mono_wav_is_accepted() {
        let (service, _, room_id) = setup().await;
        assert!(matches!(
            service.clone_voice(&room_id, sample(20_000)).await,
            Err(VoiceError::InvalidSample)
        ));
    }
}
