use diceri_domain::{AccountId, ParticipantId, RequestId, RoomId};

/// What a UUIDv7 is made of: Unix time in milliseconds and 10 random bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdParts {
    pub unix_ms: u64,
    pub random: [u8; 10],
}

/// Source of new ids (clock + randomness). Real adapters use the system clock and
/// the OS random generator; tests inject a deterministic sequence.
pub trait IdGenerator: Send + Sync {
    fn next_parts(&self) -> IdParts;

    fn room_id(&self) -> RoomId {
        let IdParts { unix_ms, random } = self.next_parts();
        RoomId::from_parts(unix_ms, random)
    }

    fn request_id(&self) -> RequestId {
        let IdParts { unix_ms, random } = self.next_parts();
        RequestId::from_parts(unix_ms, random)
    }

    fn account_id(&self) -> AccountId {
        let IdParts { unix_ms, random } = self.next_parts();
        AccountId::from_parts(unix_ms, random)
    }

    fn participant_id(&self) -> ParticipantId {
        let IdParts { unix_ms, random } = self.next_parts();
        ParticipantId::from_parts(unix_ms, random)
    }
}
