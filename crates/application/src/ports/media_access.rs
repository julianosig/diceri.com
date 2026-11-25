use diceri_domain::{DisplayName, RoomId};

/// What a participant is allowed to do in the media room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaRole {
    /// Publishes microphone audio and subscribes to others.
    Speaker,
    /// Subscribe-only and hidden; used to hear your own audio round-trip.
    Monitor,
}

/// Who is joining the media room.
#[derive(Debug, Clone, Copy)]
pub struct MediaParticipant<'a> {
    pub identity: &'a str,
    pub name: Option<&'a DisplayName>,
    pub role: MediaRole,
}

/// Credentials a browser needs to join the media room.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaGrant {
    pub server_url: String,
    pub token: String,
}

#[derive(Debug, thiserror::Error)]
#[error("failed to issue media access: {0}")]
pub struct MediaAccessError(pub String);

/// Issues short-lived, room-scoped media credentials (for example LiveKit JWTs).
pub trait MediaAccessIssuer: Send + Sync {
    fn issue(
        &self,
        room: &RoomId,
        participant: MediaParticipant<'_>,
    ) -> Result<MediaGrant, MediaAccessError>;
}
