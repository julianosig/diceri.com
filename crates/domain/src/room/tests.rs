use super::*;
use crate::ParticipantProfile;

fn key(seed: u8) -> AccessKey {
    AccessKey::from_random_bytes([seed; 16])
}

fn request(seed: u8) -> JoinRequest {
    let profile = ParticipantProfile {
        name: "Guest".parse().unwrap(),
        output_language: "en".parse().unwrap(),
        voice_id: None,
    };
    JoinRequest::new(
        RequestId::from_parts(1, [seed; 10]),
        AccountId::from_parts(4, [seed; 10]),
        profile,
        key(seed),
    )
}

fn room() -> Room {
    let settings = RoomSettings::default();
    let owner = "ana".parse().unwrap();
    Room::new(
        RoomId::from_parts(1, [1; 10]),
        RoomCode::from_random_bytes([1; 16]),
        key(100),
        owner,
        settings,
    )
}

#[test]
fn open_rooms_admit_everyone_who_asks() {
    let mut room = room();
    room.settings.entry = EntryPolicy::Open;
    room.knock(request(1)).unwrap();
    assert!(room.is_member(&key(1)));
    assert_eq!(room.pending().count(), 0);
}

#[test]
fn host_key_identifies_the_host() {
    let room = room();
    assert!(room.is_host(&key(100)));
    assert!(!room.is_host(&key(1)));
}

#[test]
fn admitted_guest_becomes_member() {
    let mut room = room();
    room.knock(request(1)).unwrap();
    assert!(!room.is_member(&key(1)));
    room.decide(&request(1).id, true).unwrap();
    assert!(room.is_member(&key(1)));
    assert_eq!(room.pending().count(), 0);
}

#[test]
fn an_admitted_guest_is_in_the_call_only_once_seated() {
    let mut room = room();
    room.knock(request(1)).unwrap();
    room.decide(&request(1).id, true).unwrap();
    assert_eq!(room.seat_of_key(&key(1)), None);
    room.claim_seat(request(1).account, "p-guest".to_owned(), Some(&[]), 0)
        .unwrap();
    assert_eq!(room.seat_of_key(&key(1)), Some("p-guest"));
}

#[test]
fn requests_can_only_be_decided_once() {
    let mut room = room();
    room.knock(request(1)).unwrap();
    room.decide(&request(1).id, false).unwrap();
    assert_eq!(
        room.decide(&request(1).id, true),
        Err(RoomRuleError::AlreadyDecided)
    );
    assert!(!room.is_member(&key(1)));
}

#[test]
fn unknown_request_cannot_be_decided() {
    assert_eq!(
        room().decide(&request(9).id, true),
        Err(RoomRuleError::UnknownRequest)
    );
}

#[test]
fn an_ended_call_gives_no_seat() {
    let mut room = room();
    room.end();
    let ana = AccountId::from_parts(1, [1; 10]);
    assert_eq!(
        room.claim_seat(ana, "p-ana".to_owned(), Some(&[]), 0),
        Err(RoomRuleError::Ended)
    );
}

#[test]
fn recording_again_replaces_the_voice_instead_of_taking_a_slot() {
    let mut room = room();
    let ana = AccountId::from_parts(1, [1; 10]);
    for n in 0..Room::MAX_VOICES * 2 {
        room.register_voice(ana, format!("v{n}")).unwrap();
    }
    assert!(room.has_voice_capacity());
    assert!(room.has_voice_of(&ana, &format!("v{}", Room::MAX_VOICES * 2 - 1)));
    assert!(!room.has_voice_of(&ana, "v0"));
}

#[test]
fn voices_belong_to_whoever_recorded_them_and_are_capped() {
    let mut room = room();
    let ana = AccountId::from_parts(1, [1; 10]);
    let bob = AccountId::from_parts(2, [2; 10]);
    room.register_voice(ana, "v1".to_owned()).unwrap();
    assert!(room.has_voice_of(&ana, "v1"));
    assert!(!room.has_voice_of(&bob, "v1"));
    assert!(!room.has_voice_of(&ana, "someone-elses"));
    for n in 1..Room::MAX_VOICES {
        let seed = u8::try_from(n).unwrap() + 10;
        room.register_voice(AccountId::from_parts(3, [seed; 10]), format!("v{}", n + 1))
            .unwrap();
    }
    assert_eq!(
        room.register_voice(bob, "extra".to_owned()),
        Err(RoomRuleError::TooManyVoices)
    );
    // Someone with a voice here can still record again.
    assert!(room.has_voice_capacity_for(&ana));
    assert!(room.register_voice(ana, "again".to_owned()).is_ok());
}

#[test]
fn an_ended_room_refuses_everything_new() {
    let mut room = room();
    assert_eq!(room.ensure_open(), Ok(()));
    room.end();
    assert!(room.is_ended());
    assert_eq!(room.ensure_open(), Err(RoomRuleError::Ended));
}

#[test]
fn pending_requests_are_capped() {
    let mut room = room();
    for seed in 0..20 {
        room.knock(request(seed)).unwrap();
    }
    assert_eq!(room.knock(request(50)), Err(RoomRuleError::TooManyPending));
}
