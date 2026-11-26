use crate::{AccessKey, ParticipantProfile, RequestId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Admitted,
    Denied,
}

/// A guest waiting in the lobby until the host admits or denies them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinRequest {
    pub id: RequestId,
    pub profile: ParticipantProfile,
    /// Proves the guest owns this request; returned only to the guest.
    pub key: AccessKey,
    pub status: RequestStatus,
}

impl JoinRequest {
    #[must_use]
    pub fn new(id: RequestId, profile: ParticipantProfile, key: AccessKey) -> Self {
        Self {
            id,
            profile,
            key,
            status: RequestStatus::Pending,
        }
    }
}
