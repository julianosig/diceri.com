//! What happens when the provider or the call fails underneath a voice operation.

use std::sync::Mutex;

use async_trait::async_trait;

use super::tests::{FakeCloner, Samples, recording, setup};
use super::*;
use crate::test_support::InMemoryRooms;

/// Ends every room while "cloning", like a host ending the call during the seconds a
/// provider takes; records the clones it is asked to delete.
struct EndingCloner {
    rooms: Arc<InMemoryRooms>,
    deleted: Mutex<Vec<String>>,
}

#[async_trait]
impl VoiceCloner for EndingCloner {
    fn provider(&self) -> &'static str {
        "inworld"
    }

    async fn create(&self, _: &str, _: VoiceSample) -> Result<VoiceId, VoiceCloneError> {
        for room in self.rooms.live_rooms().await.unwrap() {
            let end = |room: &mut Room| {
                room.end();
                Ok(())
            };
            self.rooms.modify(&room.id, Box::new(end)).await.unwrap();
        }
        Ok(VoiceId("late".to_owned()))
    }

    async fn delete(&self, voice: &VoiceId) -> Result<(), VoiceCloneError> {
        self.deleted.lock().unwrap().push(voice.0.clone());
        Ok(())
    }

    async fn delete_all_named(&self, _: &str) -> Result<usize, VoiceCloneError> {
        Ok(0)
    }
}

#[tokio::test]
async fn a_call_ended_while_cloning_stays_ended_and_the_clone_is_deleted() {
    let (service, rooms, room_id) = setup().await;
    let cloner = Arc::new(EndingCloner {
        rooms: rooms.clone(),
        deleted: Mutex::default(),
    });
    let service = VoiceService {
        cloner: cloner.clone(),
        ..service
    };
    let ana = AccountId::from_parts(1, [1; 10]);
    let result = service.record_for(&room_id, &ana, recording(true)).await;
    assert!(matches!(
        result,
        Err(VoiceError::Rule(RoomRuleError::Ended))
    ));
    // The stale copy read before cloning never reopens the call.
    let room = rooms.find_by_code(&room_id).await.unwrap().unwrap();
    assert!(room.is_ended());
    assert_eq!(*cloner.deleted.lock().unwrap(), vec!["late".to_owned()]);
}

/// Clones like `FakeCloner` but cannot delete (the provider is down).
#[derive(Default)]
struct UndeletableCloner(FakeCloner);

#[async_trait]
impl VoiceCloner for UndeletableCloner {
    fn provider(&self) -> &'static str {
        "inworld"
    }

    async fn create(&self, name: &str, sample: VoiceSample) -> Result<VoiceId, VoiceCloneError> {
        self.0.create(name, sample).await
    }

    async fn delete(&self, _: &VoiceId) -> Result<(), VoiceCloneError> {
        Err(VoiceCloneError::Request("HTTP 503".to_owned()))
    }

    async fn delete_all_named(&self, _: &str) -> Result<usize, VoiceCloneError> {
        Ok(0)
    }
}

#[tokio::test]
async fn deleting_a_voice_the_provider_cannot_delete_keeps_everything_for_a_retry() {
    let (service, _, room_id) = setup().await;
    let service = VoiceService {
        cloner: Arc::new(UndeletableCloner::default()),
        ..service
    }
    .with_samples(
        Arc::new(Samples::default()),
        Arc::new(crate::test_support::FakeClock(
            std::sync::atomic::AtomicU64::new(5),
        )),
    );
    let ana = AccountId::from_parts(1, [1; 10]);
    let voice = service
        .record_for(&room_id, &ana, recording(true))
        .await
        .unwrap();
    assert!(service.delete_saved(&ana).await.is_err());
    // Nothing was lost: the recording and the clone are still there to delete later.
    assert!(service.has_saved(&ana).await.unwrap());
    assert_eq!(service.clone_saved(&room_id, &ana).await.unwrap(), voice);
}
