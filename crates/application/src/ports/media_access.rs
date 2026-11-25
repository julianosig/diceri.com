use diceri_domain::RoomId;

/// What a participant is allowed to do in the media room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaRole {
    /// Publishes microphone audio and subscribes to others.
    Speaker,
    /// Subscribe-only and hidden; used to hear your own audio round-trip.
    Monitor,
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

/// Issues short-lived, room-scoped media credentials (e.g. LiveKit JWTs).
pub trait MediaAccessIssuer: Send + Sync {
    fn issue(
        &self,
        room: &RoomId,
        identity: &str,
        role: MediaRole,
    ) -> Result<MediaGrant, MediaAccessError>;
}
