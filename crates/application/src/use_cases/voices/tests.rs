use std::sync::Mutex;

use async_trait::async_trait;

use super::*;
use crate::test_support::{InMemoryRooms, owner, private_room, room_service};
use crate::use_cases::speech_check::fake::FakeRecognizer;
use crate::use_cases::voice_sample::{tone, wav};

#[derive(Default)]
pub(super) struct FakeCloner(Mutex<u32>);

#[async_trait]
impl VoiceCloner for FakeCloner {
    fn provider(&self) -> &'static str {
        "inworld"
    }

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
pub(super) fn recording(loud: bool) -> VoiceSample {
    VoiceSample {
        bytes: wav(&tone(3_000, loud)),
        content_type: "audio/wav".to_owned(),
    }
}

pub(super) async fn setup() -> (VoiceService, Arc<InMemoryRooms>, RoomCode) {
    setup_hearing("The quick brown fox jumps").await
}

/// A service whose recognizer hears `heard` in any recording.
async fn setup_hearing(heard: &'static str) -> (VoiceService, Arc<InMemoryRooms>, RoomCode) {
    let repository = Arc::new(InMemoryRooms::default());
    let rooms = room_service(repository.clone());
    let room = rooms.open(&owner(), private_room(), 2).await.unwrap();
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
        .record_for(
            &room_id,
            &AccountId::from_parts(9, [9; 10]),
            recording(true),
        )
        .await
        .unwrap();
    let room = repository.find_by_code(&room_id).await.unwrap().unwrap();
    assert!(room.has_voice_of(&AccountId::from_parts(9, [9; 10]), &voice.0));
}

#[tokio::test]
async fn clones_are_named_after_their_deployment() {
    let (service, _, room_id) = setup().await;
    let voice = service
        .record_for(
            &room_id,
            &AccountId::from_parts(9, [9; 10]),
            recording(true),
        )
        .await
        .unwrap();
    assert!(voice.0.starts_with(&clone_name_prefix("test")));
    assert_eq!(clone_name_prefix("prod"), "diceri-prod-");
}

#[tokio::test]
async fn rejects_samples_that_are_too_small_or_too_large() {
    let (service, _, room_id) = setup().await;
    assert!(matches!(
        service
            .record_for(&room_id, &AccountId::from_parts(9, [9; 10]), sample(100))
            .await,
        Err(VoiceError::InvalidSample)
    ));
    let huge = sample(MAX_SAMPLE_BYTES + 1);
    assert!(matches!(
        service
            .record_for(&room_id, &AccountId::from_parts(9, [9; 10]), huge)
            .await,
        Err(VoiceError::InvalidSample)
    ));
}

#[tokio::test]
async fn unknown_room_is_not_found() {
    let (service, _, _) = setup().await;
    let unknown = RoomCode::from_random_bytes([200; 16]);
    assert!(matches!(
        service
            .record_for(
                &unknown,
                &AccountId::from_parts(9, [9; 10]),
                recording(true)
            )
            .await,
        Err(VoiceError::NotFound)
    ));
}

#[tokio::test]
async fn silence_or_noise_is_never_cloned() {
    let (service, _, room_id) = setup().await;
    assert!(matches!(
        service
            .record_for(
                &room_id,
                &AccountId::from_parts(9, [9; 10]),
                recording(false)
            )
            .await,
        Err(VoiceError::NoSpeech)
    ));
    // Loud, but the recognizer hears no words (music, noise, breathing).
    let (service, _, room_id) = setup_hearing("").await;
    assert!(matches!(
        service
            .record_for(
                &room_id,
                &AccountId::from_parts(9, [9; 10]),
                recording(true)
            )
            .await,
        Err(VoiceError::NoSpeech)
    ));
}

#[tokio::test]
async fn only_16khz_mono_wav_is_accepted() {
    let (service, _, room_id) = setup().await;
    assert!(matches!(
        service
            .record_for(&room_id, &AccountId::from_parts(9, [9; 10]), sample(20_000))
            .await,
        Err(VoiceError::InvalidSample)
    ));
}

/// Kept recordings in memory, for the tests of keeping and reusing a voice.
#[derive(Default)]
pub(super) struct Samples(
    Mutex<std::collections::HashMap<AccountId, VoiceSample>>,
    Mutex<std::collections::HashMap<AccountId, KeptClone>>,
);

#[async_trait]
impl VoiceSampleStore for Samples {
    async fn save(
        &self,
        account: &AccountId,
        sample: &VoiceSample,
        _: u64,
    ) -> Result<(), VoiceSampleStoreError> {
        self.0.lock().unwrap().insert(*account, sample.clone());
        Ok(())
    }

    async fn find(
        &self,
        account: &AccountId,
    ) -> Result<Option<VoiceSample>, VoiceSampleStoreError> {
        Ok(self.0.lock().unwrap().get(account).cloned())
    }

    async fn has(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError> {
        Ok(self.0.lock().unwrap().contains_key(account))
    }

    async fn delete(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError> {
        self.1.lock().unwrap().remove(account);
        Ok(self.0.lock().unwrap().remove(account).is_some())
    }

    async fn kept_clone(
        &self,
        account: &AccountId,
    ) -> Result<Option<KeptClone>, VoiceSampleStoreError> {
        Ok(self.1.lock().unwrap().get(account).cloned())
    }

    async fn keep_clone(
        &self,
        account: &AccountId,
        clone: &KeptClone,
        _: u64,
    ) -> Result<(), VoiceSampleStoreError> {
        self.1.lock().unwrap().insert(*account, clone.clone());
        Ok(())
    }
}

#[tokio::test]
async fn a_recording_is_kept_and_cloned_again_for_the_next_call() {
    let (service, rooms, room_id) = setup().await;
    let samples = Arc::new(Samples::default());
    let clock = Arc::new(crate::test_support::FakeClock(
        std::sync::atomic::AtomicU64::new(5),
    ));
    let service = service.with_samples(samples.clone(), clock);
    let ana = AccountId::from_parts(1, [1; 10]);
    assert!(matches!(
        service.clone_saved(&room_id, &ana).await,
        Err(VoiceError::NoSavedVoice)
    ));

    service
        .record_for(&room_id, &ana, recording(true))
        .await
        .unwrap();
    assert!(service.has_saved(&ana).await.unwrap());
    let again = service.clone_saved(&room_id, &ana).await.unwrap();
    let room = rooms.find_by_code(&room_id).await.unwrap().unwrap();
    assert!(room.has_voice_of(&ana, &again.0));

    assert!(service.delete_saved(&ana).await.unwrap());
    assert!(!service.has_saved(&ana).await.unwrap());
}

#[tokio::test]
async fn nothing_is_kept_without_a_store() {
    let (service, _, room_id) = setup().await;
    let ana = AccountId::from_parts(1, [1; 10]);
    service
        .record_for(&room_id, &ana, recording(true))
        .await
        .unwrap();
    assert!(!service.has_saved(&ana).await.unwrap());
}

#[tokio::test]
async fn an_accounts_clone_is_made_once_and_reused_in_every_call() {
    let (service, _, room_id) = setup().await;
    let cloner = Arc::new(FakeCloner::default());
    let service = VoiceService {
        cloner: cloner.clone(),
        ..service
    }
    .with_samples(
        Arc::new(Samples::default()),
        Arc::new(crate::test_support::FakeClock(
            std::sync::atomic::AtomicU64::new(5),
        )),
    );
    let ana = AccountId::from_parts(1, [1; 10]);

    let recorded = service
        .record_for(&room_id, &ana, recording(true))
        .await
        .unwrap();
    assert!(recorded.0.starts_with(&account_clone_prefix("test")));
    // A reload, then another call: the same clone, no new one at the provider.
    assert_eq!(service.clone_saved(&room_id, &ana).await.unwrap(), recorded);
    assert_eq!(service.clone_saved(&room_id, &ana).await.unwrap(), recorded);
    assert_eq!(*cloner.0.lock().unwrap(), 1);

    // Recording again makes a new clone, which is the one reused from then on.
    let again = service
        .record_for(&room_id, &ana, recording(true))
        .await
        .unwrap();
    assert_ne!(again, recorded);
    assert_eq!(service.clone_saved(&room_id, &ana).await.unwrap(), again);
}
