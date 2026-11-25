use serde::{Deserialize, Serialize};

use crate::RoomId;

/// An anonymous meeting room joined through a shared link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub max_participants: u8,
}

impl Room {
    /// The initial product supports one-to-one translated calls.
    pub const DEFAULT_MAX_PARTICIPANTS: u8 = 2;

    #[must_use]
    pub fn new(id: RoomId) -> Self {
        Self {
            id,
            max_participants: Self::DEFAULT_MAX_PARTICIPANTS,
        }
    }
}
