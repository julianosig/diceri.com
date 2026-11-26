use crate::{
    AccessKey, EntryPolicy, JoinRequest, RequestId, RequestStatus, RoomId, RoomSettings, Username,
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
}

/// A meeting room joined through a shared link. Its owner (the account that
/// created it) holds the host key, names the room and decides who gets in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub id: RoomId,
    pub max_participants: u8,
    pub settings: RoomSettings,
    /// The account that created the room, shown as its host.
    pub owner: Username,
    host_key: AccessKey,
    requests: Vec<JoinRequest>,
    /// Voice clones recorded for this room (provider ids), deleted when people leave.
    voices: Vec<String>,
    /// Set when the host ends the call for everyone; nobody can join afterwards.
    ended: bool,
}

impl Room {
    /// The initial product supports one-to-one translated calls.
    pub const DEFAULT_MAX_PARTICIPANTS: u8 = 2;
    /// Caps lobby spam from a leaked link.
    pub const MAX_PENDING_REQUESTS: usize = 20;
    /// Caps voice-clone creation (a paid, quota-limited provider call) per room.
    pub const MAX_VOICES: usize = 10;

    #[must_use]
    pub fn new(id: RoomId, host_key: AccessKey, owner: Username, settings: RoomSettings) -> Self {
        Self {
            id,
            max_participants: Self::DEFAULT_MAX_PARTICIPANTS,
            settings,
            owner,
            host_key,
            requests: Vec::new(),
            voices: Vec::new(),
            ended: false,
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

    pub fn register_voice(&mut self, voice_id: String) -> Result<(), RoomRuleError> {
        if !self.has_voice_capacity() {
            return Err(RoomRuleError::TooManyVoices);
        }
        self.voices.push(voice_id);
        Ok(())
    }

    /// Only voices recorded in this room may be attached to its participants.
    #[must_use]
    pub fn owns_voice(&self, voice_id: &str) -> bool {
        self.voices.iter().any(|voice| voice == voice_id)
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
        JoinRequest::new(RequestId::from_random_bytes([seed; 16]), profile, key(seed))
    }

    fn room() -> Room {
        let settings = RoomSettings::private("Chat".parse().unwrap());
        let owner = "ana".parse().unwrap();
        Room::new(
            RoomId::from_random_bytes([1; 16]),
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
    fn voices_are_owned_by_the_room_and_capped() {
        let mut room = room();
        room.register_voice("v1".to_owned()).unwrap();
        assert!(room.owns_voice("v1"));
        assert!(!room.owns_voice("someone-elses"));
        for n in 1..Room::MAX_VOICES {
            room.register_voice(format!("v{}", n + 1)).unwrap();
        }
        assert_eq!(
            room.register_voice("extra".to_owned()),
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
