//! Calls keep to their size: one seat per account, reused when it rejoins.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_domain::{AccountId, RoomId, RoomRuleError};

use super::tests::{Fixture, fixture, name, someone};
use super::{AdmissionError, KnockOutcome};
use crate::ports::{LiveRoom, MediaAccessError, MediaPresence};
use crate::test_support::{FakeClock, owner, private_room};

/// The identities connected to every call; `None` stands for the media server failing.
struct People(Mutex<Option<Vec<String>>>);

impl People {
    fn set(&self, identities: &[&str]) {
        *self.0.lock().unwrap() = Some(identities.iter().map(|&id| id.to_owned()).collect());
    }
}

#[async_trait]
impl MediaPresence for People {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn identities_in(&self, _: &RoomId) -> Result<Vec<String>, MediaAccessError> {
        self.0
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| MediaAccessError("unreachable".to_owned()))
    }
}

/// A fixture whose calls are watched by `People`, at a fixed time.
fn with_people(identities: Option<&[&str]>) -> (Fixture, Arc<People>) {
    let people = Arc::new(People(Mutex::new(None)));
    if let Some(identities) = identities {
        people.set(identities);
    }
    let clock = Arc::new(FakeClock(std::sync::atomic::AtomicU64::new(1_000)));
    let mut f = fixture(true);
    f.admission = f.admission.clone().with_presence(people.clone(), clock);
    (f, people)
}

#[tokio::test]
async fn rejoining_after_a_reload_keeps_the_same_seat_even_in_a_full_call() {
    let (f, people) = with_people(Some(&[]));
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let host = AccountId::from_parts(1, [1; 10]);
    let first = f
        .admission
        .join_as_host(&room.code, room.host_key(), &host, &name("Ana"))
        .await
        .unwrap();
    // Two people connected, one of them the host's old tab: the host is not a newcomer.
    people.set(&[&first.identity, "p-guest"]);
    let again = f
        .admission
        .join_as_host(&room.code, room.host_key(), &host, &name("Ana"))
        .await
        .unwrap();
    assert_eq!(again.identity, first.identity);
}

#[tokio::test]
async fn a_newcomer_cannot_ask_to_join_a_full_call() {
    let (f, _) = with_people(Some(&["p-a", "p-b"]));
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let result = f.admission.knock(&room.code, &someone(), name("Eve")).await;
    assert!(matches!(
        result,
        Err(AdmissionError::Rule(RoomRuleError::Full))
    ));
}

#[tokio::test]
async fn a_guest_let_in_after_the_call_filled_up_is_refused() {
    let (f, people) = with_people(Some(&["p-host"]));
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
    people.set(&["p-host", "p-carla"]);
    let outcome = f
        .admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await;
    assert!(matches!(
        outcome,
        Err(AdmissionError::Rule(RoomRuleError::Full))
    ));
}

#[tokio::test]
async fn an_ended_call_gives_no_credentials_even_to_someone_seated() {
    let (f, _) = with_people(Some(&[]));
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let host = AccountId::from_parts(1, [1; 10]);
    f.admission
        .join_as_host(&room.code, room.host_key(), &host, &name("Ana"))
        .await
        .unwrap();
    f.admission
        .end_call(&room.code, room.host_key())
        .await
        .unwrap();
    let again = f
        .admission
        .join_as_host(&room.code, room.host_key(), &host, &name("Ana"))
        .await;
    assert!(matches!(
        again,
        Err(AdmissionError::Rule(RoomRuleError::Ended))
    ));
}

#[tokio::test]
async fn an_unreachable_media_server_does_not_lock_people_out() {
    let (f, _) = with_people(None);
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
    let outcome = f
        .admission
        .outcome(&room.code, &knock.request_id, &knock.key)
        .await
        .unwrap();
    assert!(matches!(outcome, KnockOutcome::Admitted(_)));
}
