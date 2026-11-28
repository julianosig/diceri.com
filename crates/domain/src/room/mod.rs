mod seats;

use seats::Seats;

use crate::{
    AccessKey, AccountId, EntryPolicy, JoinRequest, RequestId, RequestStatus, RoomCode, RoomId,
    RoomSettings, Username,
};

/// Business rule violations on a room.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RoomRuleError {
    #[error("too many people are waiting to join")]
    TooManyPending,
    #[error("join request not found")]
    UnknownRequest,
    #[error("join request was already decided")]
    AlreadyDecided,
    #[error("too many voices were recorded in this room")]
    TooManyVoices,
    #[error("the meeting has ended")]
    Ended,
    #[error("the call is full")]
    Full,
}

/// A meeting room joined through a shared link. Its owner (the account that
/// created it) holds the host key, names the room and decides who gets in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub id: RoomId,
    /// The shareable link; a secret-like value, unlike `id`.
    pub code: RoomCode,
    pub max_participants: u8,
    pub settings: RoomSettings,
    /// The account that created the room, shown as its host.
    pub owner: Username,
    host_key: AccessKey,
    requests: Vec<JoinRequest>,
    /// Voice clones used in this room (provider ids), each with the account that recorded it.
    voices: Vec<(AccountId, String)>,
    /// Each account's place in the call (its media identity), kept to the call's size.
    seats: Seats,
    /// Set when the host ends the call for everyone; nobody can join afterwards.
    ended: bool,
    /// Minutes someone has been in this call; counted against the owner's plan.
    pub call_minutes: u32,
}

impl Room {
    /// The initial product supports one-to-one translated calls.
    pub const DEFAULT_MAX_PARTICIPANTS: u8 = 2;
    /// Caps lobby spam from a leaked link.
    pub const MAX_PENDING_REQUESTS: usize = 20;
    /// Caps voice-clone creation (a paid, quota-limited provider call) per room.
    pub const MAX_VOICES: usize = 10;

    #[must_use]
    pub fn new(
        id: RoomId,
        code: RoomCode,
        host_key: AccessKey,
        owner: Username,
        settings: RoomSettings,
    ) -> Self {
        Self {
            id,
            code,
            max_participants: Self::DEFAULT_MAX_PARTICIPANTS,
            settings,
            owner,
            host_key,
            requests: Vec::new(),
            voices: Vec::new(),
            seats: Seats::default(),
            ended: false,
            call_minutes: 0,
        }
    }

    #[must_use]
    pub fn host_key(&self) -> &AccessKey {
        &self.host_key
    }

    #[must_use]
    pub fn is_host(&self, key: &AccessKey) -> bool {
        &self.host_key == key
    }

    /// True for the host key or the key of an admitted guest.
    #[must_use]
    pub fn is_member(&self, key: &AccessKey) -> bool {
        self.is_host(key)
            || self
                .requests
                .iter()
                .any(|r| r.status == RequestStatus::Admitted && &r.key == key)
    }

    /// The seat (media identity) of whoever holds `key`: the host's, or an admitted guest's
    /// once they took their place. Being let in is not enough when the call filled up first.
    #[must_use]
    pub fn seat_of_key(&self, key: &AccessKey) -> Option<&str> {
        if self.is_host(key) {
            return self.seats.owner_identity();
        }
        self.requests
            .iter()
            .find(|r| r.status == RequestStatus::Admitted && &r.key == key)
            .and_then(|r| self.seats.identity_of(&r.account))
    }

    /// Adds a join request; in an open room it is admitted at once.
    pub fn knock(&mut self, mut request: JoinRequest) -> Result<(), RoomRuleError> {
        if self.pending().count() >= Self::MAX_PENDING_REQUESTS {
            return Err(RoomRuleError::TooManyPending);
        }
        if self.settings.entry == EntryPolicy::Open {
            request.status = RequestStatus::Admitted;
        }
        self.requests.push(request);
        Ok(())
    }

    pub fn pending(&self) -> impl Iterator<Item = &JoinRequest> {
        self.requests
            .iter()
            .filter(|r| r.status == RequestStatus::Pending)
    }

    #[must_use]
    pub fn request(&self, id: &RequestId) -> Option<&JoinRequest> {
        self.requests.iter().find(|r| &r.id == id)
    }

    #[must_use]
    pub fn is_ended(&self) -> bool {
        self.ended
    }

    /// Ends the meeting for everyone; idempotent.
    pub fn end(&mut self) {
        self.ended = true;
    }

    /// Joining, knocking and admitting are only possible while the meeting runs.
    pub fn ensure_open(&self) -> Result<(), RoomRuleError> {
        if self.ended {
            Err(RoomRuleError::Ended)
        } else {
            Ok(())
        }
    }

    #[must_use]
    pub fn has_voice_capacity(&self) -> bool {
        self.voices.len() < Self::MAX_VOICES
    }

    pub fn register_voice(
        &mut self,
        account: AccountId,
        voice_id: String,
    ) -> Result<(), RoomRuleError> {
        if !self.has_voice_capacity() {
            return Err(RoomRuleError::TooManyVoices);
        }
        self.voices.push((account, voice_id));
        Ok(())
    }

    /// Takes back a voice registered for `account` (its clone could not be kept), freeing its slot.
    pub fn unregister_voice(&mut self, account: &AccountId, voice_id: &str) {
        self.voices
            .retain(|(owner, voice)| !(owner == account && voice == voice_id));
    }

    /// A participant may only speak in a voice they recorded themselves: another person's
    /// clone id (visible to everyone in the call) is never accepted.
    #[must_use]
    pub fn has_voice_of(&self, account: &AccountId, voice_id: &str) -> bool {
        self.voices
            .iter()
            .any(|(owner, voice)| owner == account && voice == voice_id)
    }

    /// Keeps a place for the owner's account: guests never fill the call before the owner.
    pub fn reserve_owner_seat(&mut self, owner: AccountId) {
        self.seats.reserve_for_owner(owner);
    }

    /// The media identity of `account`'s seat, if it has joined this call before.
    #[must_use]
    pub fn seat_of(&self, account: &AccountId) -> Option<&str> {
        self.seats.identity_of(account)
    }

    /// Whether `account` could join now, given who is connected (`present`; `None` when the
    /// media server cannot say) and who was just given credentials.
    #[must_use]
    pub fn has_room_for(
        &self,
        account: &AccountId,
        present: Option<&[String]>,
        now_ms: u64,
    ) -> bool {
        self.seats
            .has_room_for(account, self.max_participants, present, now_ms)
    }

    /// Gives `account` its place in the open call: its existing seat (a reload) or a new one
    /// with `identity`; refused when the call has ended or is full.
    pub fn claim_seat(
        &mut self,
        account: AccountId,
        identity: String,
        present: Option<&[String]>,
        now_ms: u64,
    ) -> Result<String, RoomRuleError> {
        self.ensure_open()?;
        self.seats
            .claim(account, identity, self.max_participants, present, now_ms)
            .ok_or(RoomRuleError::Full)
    }

    pub fn decide(&mut self, id: &RequestId, admit: bool) -> Result<(), RoomRuleError> {
        let request = self
            .requests
            .iter_mut()
            .find(|r| &r.id == id)
            .ok_or(RoomRuleError::UnknownRequest)?;
        if request.status != RequestStatus::Pending {
            return Err(RoomRuleError::AlreadyDecided);
        }
        request.status = if admit {
            RequestStatus::Admitted
        } else {
            RequestStatus::Denied
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
    fn voices_belong_to_whoever_recorded_them_and_are_capped() {
        let mut room = room();
        let ana = AccountId::from_parts(1, [1; 10]);
        let bob = AccountId::from_parts(2, [2; 10]);
        room.register_voice(ana, "v1".to_owned()).unwrap();
        assert!(room.has_voice_of(&ana, "v1"));
        assert!(!room.has_voice_of(&bob, "v1"));
        assert!(!room.has_voice_of(&ana, "someone-elses"));
        for n in 1..Room::MAX_VOICES {
            room.register_voice(ana, format!("v{}", n + 1)).unwrap();
        }
        assert_eq!(
            room.register_voice(bob, "extra".to_owned()),
            Err(RoomRuleError::TooManyVoices)
        );
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
}
