use std::sync::Arc;

use diceri_domain::{AccessKey, ParticipantProfile, RoomRuleError};

use super::{AdmissionError, AdmissionService, KnockOutcome};
use crate::ports::{MediaAccessIssuer, MediaRole};
use crate::test_support::{CountingRandom, InMemoryRooms, RecordingDispatcher, RecordingIssuer};
use crate::use_cases::RoomService;

struct Fixture {
    rooms: RoomService,
    admission: AdmissionService,
    issuer: Arc<RecordingIssuer>,
    dispatcher: Arc<RecordingDispatcher>,
}

fn fixture(with_media: bool) -> Fixture {
    let repository = Arc::new(InMemoryRooms::default());
    let random = Arc::new(CountingRandom::default());
    let issuer = Arc::new(RecordingIssuer::default());
    let media = with_media.then(|| -> Arc<dyn MediaAccessIssuer> { issuer.clone() });
    let dispatcher = Arc::new(RecordingDispatcher::default());
    Fixture {
        rooms: RoomService::new(repository.clone(), random.clone()),
        admission: AdmissionService::new(repository, random, media)
            .with_translation(dispatcher.clone()),
        issuer,
        dispatcher,
    }
}

fn name(raw: &str) -> ParticipantProfile {
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
    let room = f.rooms.create().await.unwrap();
    let ticket = f
        .admission
        .join_as_host(&room.id, room.host_key(), &name("Ana"))
        .await
        .unwrap();
    assert!(ticket.identity.starts_with("p-"));
    assert_eq!(f.issuer.0.lock().unwrap()[0].1.as_deref(), Some("Ana"));

    let denied = f
        .admission
        .join_as_host(&room.id, &wrong_key(), &name("Eve"))
        .await;
    assert!(matches!(denied, Err(AdmissionError::Forbidden)));
}

#[tokio::test]
async fn guest_waits_until_admitted_then_gets_credentials() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    let knock = f.admission.knock(&room.id, name("Bruno")).await.unwrap();

    let waiting = f
        .admission
        .outcome(&room.id, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert_eq!(waiting, KnockOutcome::Pending);
    let pending = f
        .admission
        .pending(&room.id, room.host_key())
        .await
        .unwrap();
    assert_eq!(pending.len(), 1);

    f.admission
        .decide(&room.id, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();
    let outcome = f
        .admission
        .outcome(&room.id, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert!(matches!(outcome, KnockOutcome::Admitted(_)));
    assert_eq!(f.issuer.0.lock().unwrap()[0].1.as_deref(), Some("Bruno"));
}

#[tokio::test]
async fn denied_guest_learns_it_and_cannot_be_readmitted() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    let knock = f.admission.knock(&room.id, name("Eve")).await.unwrap();
    f.admission
        .decide(&room.id, room.host_key(), &knock.request_id, false)
        .await
        .unwrap();

    let outcome = f
        .admission
        .outcome(&room.id, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert_eq!(outcome, KnockOutcome::Denied);
    let again = f
        .admission
        .decide(&room.id, room.host_key(), &knock.request_id, true)
        .await;
    assert!(matches!(
        again,
        Err(AdmissionError::Rule(RoomRuleError::AlreadyDecided))
    ));
}

#[tokio::test]
async fn only_the_host_can_list_or_decide() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    let knock = f.admission.knock(&room.id, name("Eve")).await.unwrap();
    assert!(matches!(
        f.admission.pending(&room.id, &knock.key).await,
        Err(AdmissionError::Forbidden)
    ));
    assert!(matches!(
        f.admission
            .decide(&room.id, &knock.key, &knock.request_id, true)
            .await,
        Err(AdmissionError::Forbidden)
    ));
}

#[tokio::test]
async fn polling_requires_the_request_key() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    let knock = f.admission.knock(&room.id, name("Bruno")).await.unwrap();
    let outcome = f
        .admission
        .outcome(&room.id, &knock.request_id, &wrong_key())
        .await;
    assert!(matches!(outcome, Err(AdmissionError::Forbidden)));
}

#[tokio::test]
async fn monitor_is_only_for_members() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    let knock = f.admission.knock(&room.id, name("Bruno")).await.unwrap();
    assert!(matches!(
        f.admission.monitor(&room.id, &knock.key).await,
        Err(AdmissionError::Forbidden)
    ));

    f.admission
        .decide(&room.id, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();
    let ticket = f.admission.monitor(&room.id, &knock.key).await.unwrap();
    assert!(ticket.identity.starts_with("monitor-"));
    assert_eq!(
        f.issuer.0.lock().unwrap().last().unwrap().2,
        MediaRole::Monitor
    );
}

#[tokio::test]
async fn joining_without_media_server_is_unavailable() {
    let f = fixture(false);
    let room = f.rooms.create().await.unwrap();
    let result = f
        .admission
        .join_as_host(&room.id, room.host_key(), &name("Ana"))
        .await;
    assert!(matches!(result, Err(AdmissionError::MediaUnavailable)));
}

#[tokio::test]
async fn joining_starts_translation_for_the_room() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    f.admission
        .join_as_host(&room.id, room.host_key(), &name("Ana"))
        .await
        .unwrap();
    let knock = f.admission.knock(&room.id, name("Bruno")).await.unwrap();
    f.admission
        .decide(&room.id, room.host_key(), &knock.request_id, true)
        .await
        .unwrap();
    f.admission
        .outcome(&room.id, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert_eq!(
        f.dispatcher.0.lock().unwrap().as_slice(),
        [room.id.clone(), room.id]
    );
}

#[tokio::test]
async fn a_voice_from_outside_the_room_is_forbidden() {
    let f = fixture(true);
    let room = f.rooms.create().await.unwrap();
    let stolen = ParticipantProfile {
        voice_id: Some("someone-else".to_owned()),
        ..name("Eve")
    };
    let result = f
        .admission
        .join_as_host(&room.id, room.host_key(), &stolen)
        .await;
    assert!(matches!(result, Err(AdmissionError::Forbidden)));
    assert!(matches!(
        f.admission.knock(&room.id, stolen).await,
        Err(AdmissionError::Forbidden)
    ));
}
