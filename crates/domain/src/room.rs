use crate::{AccessKey, JoinRequest, RequestId, RequestStatus, RoomId};

/// Business rule violations on a room.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RoomRuleError {
    #[error("too many people are waiting to join")]
    TooManyPending,
    #[error("join request not found")]
    UnknownRequest,
    #[error("join request was already decided")]
    AlreadyDecided,
}

/// An anonymous meeting room joined through a shared link. The creator holds
/// the host key and decides who gets in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub id: RoomId,
    pub max_participants: u8,
    host_key: AccessKey,
    requests: Vec<JoinRequest>,
}

impl Room {
    /// The initial product supports one-to-one translated calls.
    pub const DEFAULT_MAX_PARTICIPANTS: u8 = 2;
    /// Caps lobby spam from a leaked link.
    pub const MAX_PENDING_REQUESTS: usize = 20;

    #[must_use]
    pub fn new(id: RoomId, host_key: AccessKey) -> Self {
        Self {
            id,
            max_participants: Self::DEFAULT_MAX_PARTICIPANTS,
            host_key,
            requests: Vec::new(),
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

    pub fn knock(&mut self, request: JoinRequest) -> Result<(), RoomRuleError> {
        if self.pending().count() >= Self::MAX_PENDING_REQUESTS {
            return Err(RoomRuleError::TooManyPending);
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
    use crate::DisplayName;

    fn key(seed: u8) -> AccessKey {
        AccessKey::from_random_bytes([seed; 16])
    }

    fn request(seed: u8) -> JoinRequest {
        let name: DisplayName = "Guest".parse().unwrap();
        JoinRequest::new(RequestId::from_random_bytes([seed; 16]), name, key(seed))
    }

    fn room() -> Room {
        Room::new(RoomId::from_random_bytes([1; 16]), key(100))
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
    fn pending_requests_are_capped() {
        let mut room = room();
        for seed in 0..20 {
            room.knock(request(seed)).unwrap();
        }
        assert_eq!(room.knock(request(50)), Err(RoomRuleError::TooManyPending));
    }
}
