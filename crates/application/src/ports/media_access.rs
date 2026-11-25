use diceri_domain::{ParticipantProfile, RoomId};

/// What a participant is allowed to do in the media room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaRole {
    /// Publishes microphone audio and subscribes to others.
    Speaker,
    /// Subscribe-only and hidden; used to hear your own audio round-trip.
    Monitor,
    /// The media agent: subscribes to speakers, publishes translated audio and captions.
    Translator,
}

/// Who is joining the media room.
#[derive(Debug, Clone, Copy)]
pub struct MediaParticipant<'a> {
    pub identity: &'a str,
    /// Name and language, published as the participant's name and attributes.
    pub profile: Option<&'a ParticipantProfile>,
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

/// Ends a media room for everyone in it.
#[async_trait::async_trait]
pub trait MediaRoomCloser: Send + Sync {
    async fn close(&self, room: &RoomId) -> Result<(), MediaAccessError>;
}

/// Issues short-lived, room-scoped media credentials (for example LiveKit JWTs).
pub trait MediaAccessIssuer: Send + Sync {
    fn issue(
        &self,
        room: &RoomId,
        participant: MediaParticipant<'_>,
    ) -> Result<MediaGrant, MediaAccessError>;
}
