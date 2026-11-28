use std::sync::Arc;

use diceri_domain::{
    AccessKey, AccountId, EntryPolicy, ParticipantProfile, RoomRuleError, RoomSettings,
};

use super::{AdmissionError, AdmissionService, KnockOutcome};
use crate::ports::{MediaAccessIssuer, MediaRole, RoomRepository};
use crate::test_support::{
    CountingRandom, InMemoryRooms, RecordingDispatcher, RecordingIssuer, SequentialIds, owner,
    private_room,
};
use crate::use_cases::RoomService;

pub(super) struct Fixture {
    pub(super) rooms: RoomService,
    pub(super) admission: AdmissionService,
    pub(super) issuer: Arc<RecordingIssuer>,
    dispatcher: Arc<RecordingDispatcher>,
    pub(super) repo: Arc<InMemoryRooms>,
}

pub(super) fn fixture(with_media: bool) -> Fixture {
    let repository = Arc::new(InMemoryRooms::default());
    let random = Arc::new(CountingRandom::default());
    let ids = Arc::new(SequentialIds::default());
    let issuer = Arc::new(RecordingIssuer::default());
    let media = with_media.then(|| -> Arc<dyn MediaAccessIssuer> { issuer.clone() });
    let dispatcher = Arc::new(RecordingDispatcher::default());
    Fixture {
        repo: repository.clone(),
        rooms: RoomService::new(repository.clone(), random.clone(), ids.clone()),
        admission: AdmissionService::new(repository, random, ids, media)
            .with_translation(dispatcher.clone()),
        issuer,
        dispatcher,
    }
}

pub(super) fn name(raw: &str) -> ParticipantProfile {
    ParticipantProfile {
        name: raw.parse().unwrap(),
        output_language: "pt-BR".parse().unwrap(),
        voice_id: None,
    }
}

fn wrong_key() -> AccessKey {
    AccessKey::from_random_bytes([250; 16])
}

#[tokio::test]
async fn host_joins_with_name_and_wrong_key_is_forbidden() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let ticket = f
        .admission
        .join_as_host(&room.code, room.host_key(), &someone(), &name("Ana"))
        .await
        .unwrap();
    assert!(ticket.identity.starts_with("p-"));
    assert_eq!(f.issuer.0.lock().unwrap()[0].1.as_deref(), Some("Ana"));

    let denied = f
        .admission
        .join_as_host(&room.code, &wrong_key(), &someone(), &name("Eve"))
        .await;
    assert!(matches!(denied, Err(AdmissionError::Forbidden)));
}

#[tokio::test]
async fn guest_waits_until_admitted_then_gets_credentials() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Bruno"))
        .await
        .unwrap();

    let waiting = f
        .admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert_eq!(waiting, KnockOutcome::Pending);
    let pending = f
        .admission
        .pending(&room.code, room.host_key())
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);

    f.admission
        .decide(&room.code, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();
    let outcome = f
        .admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert!(matches!(outcome, KnockOutcome::Admitted(_)));
    assert_eq!(f.issuer.0.lock().unwrap()[0].1.as_deref(), Some("Bruno"));
}

#[tokio::test]
async fn denied_guest_learns_it_and_cannot_be_readmitted() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Eve"))
        .await
        .unwrap();
    f.admission
        .decide(&room.code, room.host_key(), &knock.request_id, false)
        .await
        .unwrap();

    let outcome = f
        .admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert_eq!(outcome, KnockOutcome::Denied);
    let again = f
        .admission
        .decide(&room.code, room.host_key(), &knock.request_id, true)
        .await;
    assert!(matches!(
        again,
        Err(AdmissionError::Rule(RoomRuleError::AlreadyDecided))
    ));
}

#[tokio::test]
async fn only_the_host_can_list_or_decide() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Eve"))
        .await
        .unwrap();
    assert!(matches!(
        f.admission.pending(&room.code, &knock.key).await,
        Err(AdmissionError::Forbidden)
    ));
    assert!(matches!(
        f.admission
            .decide(&room.code, &knock.key, &knock.request_id, true)
            .await,
        Err(AdmissionError::Forbidden)
    ));
}

#[tokio::test]
async fn polling_requires_the_request_key() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Bruno"))
        .await
        .unwrap();
    let outcome = f
        .admission
        .outcome(&room.code, &knock.request_id, &wrong_key())
        .await;
    assert!(matches!(outcome, Err(AdmissionError::Forbidden)));
}

#[tokio::test]
async fn monitor_is_only_for_members() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Bruno"))
        .await
        .unwrap();
    assert!(matches!(
        f.admission.monitor(&room.code, &knock.key).await,
        Err(AdmissionError::Forbidden)
    ));

    f.admission
        .decide(&room.code, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();
    let ticket = f.admission.monitor(&room.code, &knock.key).await.unwrap();
    assert!(ticket.identity.starts_with("monitor-"));
    assert_eq!(
        f.issuer.0.lock().unwrap().last().unwrap().2,
        MediaRole::Monitor
    );
}

#[tokio::test]
async fn joining_without_media_server_is_unavailable() {
    let f = fixture(false);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let result = f
        .admission
        .join_as_host(&room.code, room.host_key(), &someone(), &name("Ana"))
        .await;
    assert!(matches!(result, Err(AdmissionError::MediaUnavailable)));
}

#[tokio::test]
async fn joining_starts_translation_for_the_room() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    f.admission
        .join_as_host(&room.code, room.host_key(), &someone(), &name("Ana"))
        .await
        .unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Bruno"))
        .await
        .unwrap();
    f.admission
        .decide(&room.code, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();
    f.admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert_eq!(
        f.dispatcher.0.lock().unwrap().as_slice(),
        [room.id, room.id]
    );
}

#[tokio::test]
async fn a_voice_from_outside_the_room_is_forbidden() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let stolen = ParticipantProfile {
        voice_id: Some("someone-else".to_owned()),
        ..name("Eve")
    };
    let result = f
        .admission
        .join_as_host(&room.code, room.host_key(), &someone(), &stolen)
        .await;
    assert!(matches!(result, Err(AdmissionError::Forbidden)));
    assert!(matches!(
        f.admission.knock(&room.code, &someone(), stolen).await,
        Err(AdmissionError::Forbidden)
    ));
}

#[tokio::test]
async fn another_persons_voice_in_the_room_is_forbidden() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let anas = |room: &mut diceri_domain::Room| {
        room.register_voice(AccountId::from_parts(7, [7; 10]), "anas-voice".to_owned())
    };
    f.repo.modify(&room.id, Box::new(anas)).await.unwrap();
    let borrowed = ParticipantProfile {
        voice_id: Some("anas-voice".to_owned()),
        ..name("Eve")
    };
    assert!(matches!(
        f.admission.knock(&room.code, &someone(), borrowed).await,
        Err(AdmissionError::Forbidden)
    ));
}

/// The signed-in account joining (tests do not need a distinct one per person).
pub(super) fn someone() -> AccountId {
    AccountId::from_parts(3, [3; 10])
}

fn ended<T>(result: &Result<T, AdmissionError>) -> bool {
    matches!(result, Err(AdmissionError::Rule(RoomRuleError::Ended)))
}

#[tokio::test]
async fn after_the_host_ends_the_call_nobody_can_join() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Bruno"))
        .await
        .unwrap();
    f.admission
        .decide(&room.code, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();

    f.admission
        .end_call(&room.code, room.host_key())
        .await
        .unwrap();

    assert!(ended(
        &f.admission
            .join_as_host(&room.code, room.host_key(), &someone(), &name("Ana"))
            .await
    ));
    assert!(ended(
        &f.admission.knock(&room.code, &someone(), name("Eve")).await
    ));
    assert!(ended(
        &f.admission
            .outcome(&room.code, &knock.request_id, &knock.key)
            .await
            .map(|_| ())
    ));
}

#[tokio::test]
async fn only_the_host_can_end_the_call() {
    let f = fixture(true);
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let result = f.admission.end_call(&room.code, &wrong_key()).await;
    assert!(matches!(result, Err(AdmissionError::Forbidden)));
}

#[tokio::test]
async fn in_an_open_room_people_get_in_without_waiting() {
    let f = fixture(true);
    let settings = RoomSettings {
        entry: EntryPolicy::Open,
        ..private_room()
    };
    let room = f.rooms.open(&owner(), settings, 2).await.unwrap();
    let knock = f
        .admission
        .knock(&room.code, &someone(), name("Bruno"))
        .await
        .unwrap();
    let outcome = f
        .admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert!(matches!(outcome, KnockOutcome::Admitted(_)));
}
