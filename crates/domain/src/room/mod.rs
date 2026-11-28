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
    /// The voice asked for is not this person's voice in the room any more (recorded again).
    #[error("that voice was replaced; record or reuse your voice again")]
    VoiceReplaced,
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

    /// Whether `account` can register a voice: a free slot, or the one it already has.
    #[must_use]
    pub fn has_voice_capacity_for(&self, account: &AccountId) -> bool {
        self.has_voice_capacity() || self.voices.iter().any(|(owner, _)| owner == account)
    }

    #[must_use]
    pub fn has_voice_capacity(&self) -> bool {
        self.voices.len() < Self::MAX_VOICES
    }

    /// Registers `account`'s voice in this room, replacing the one it had: one voice per
    /// person, so recording again never uses up another person's slot.
    pub fn register_voice(
        &mut self,
        account: AccountId,
        voice_id: String,
    ) -> Result<(), RoomRuleError> {
        self.voices.retain(|(owner, _)| *owner != account);
        if !self.has_voice_capacity() {
            return Err(RoomRuleError::TooManyVoices);
        }
        self.voices.push((account, voice_id));
        Ok(())
    }

    /// The voice `account` has registered in this room, if any.
    #[must_use]
    pub fn voice_of(&self, account: &AccountId) -> Option<&str> {
        self.voices
            .iter()
            .find(|(owner, _)| owner == account)
            .map(|(_, voice)| voice.as_str())
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

    /// The media identity of the owner's seat, once they have joined.
    #[must_use]
    pub fn owner_seat(&self) -> Option<&str> {
        self.seats.owner_identity()
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
mod tests;
