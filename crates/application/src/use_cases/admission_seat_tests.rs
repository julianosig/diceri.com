//! Calls keep to their size: one seat per account, reused when it rejoins.

use std::sync::Mutex;

use async_trait::async_trait;
use diceri_domain::{AccountId, RoomId, RoomRuleError};

use super::tests::{fixture, name, someone};
use super::{AdmissionError, KnockOutcome};
use crate::ports::{LiveRoom, MediaAccessError, MediaPresence};
use crate::test_support::{owner, private_room};

/// Says how many people are in every call; `None` stands for the media server failing.
struct People(Mutex<Option<usize>>);

#[async_trait]
impl MediaPresence for People {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn people_in(&self, _: &RoomId) -> Result<usize, MediaAccessError> {
        self.0
            .lock()
            .unwrap()
            .ok_or_else(|| MediaAccessError("unreachable".to_owned()))
    }
}

fn with_people(people: Option<usize>) -> (super::tests::Fixture, std::sync::Arc<People>) {
    let presence = std::sync::Arc::new(People(Mutex::new(people)));
    let mut f = fixture(true);
    f.admission = f.admission.clone().with_presence(presence.clone());
    (f, presence)
}

#[tokio::test]
async fn rejoining_after_a_reload_keeps_the_same_seat_even_in_a_full_call() {
    let (f, people) = with_people(Some(0));
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let host = AccountId::from_parts(1, [1; 10]);
    let first = f
        .admission
        .join_as_host(&room.code, room.host_key(), &host, &name("Ana"))
        .await
        .unwrap();
    // Two people in the call, one of them the host's old tab: the host is not a newcomer.
    *people.0.lock().unwrap() = Some(2);
    let again = f
        .admission
        .join_as_host(&room.code, room.host_key(), &host, &name("Ana"))
        .await
        .unwrap();
    assert_eq!(again.identity, first.identity);
}

#[tokio::test]
async fn a_newcomer_cannot_ask_to_join_a_full_call() {
    let (f, _) = with_people(Some(2));
    let room = f.rooms.open(&owner(), private_room(), 2).await.unwrap();
    let result = f.admission.knock(&room.code, &someone(), name("Eve")).await;
    assert!(matches!(
        result,
        Err(AdmissionError::Rule(RoomRuleError::Full))
    ));
}

#[tokio::test]
async fn a_guest_let_in_after_the_call_filled_up_is_refused() {
    let (f, people) = with_people(Some(1));
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
    *people.0.lock().unwrap() = Some(2);
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
