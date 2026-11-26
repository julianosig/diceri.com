use std::sync::Arc;

use diceri_domain::{
    AccessKey, JoinRequest, ParticipantProfile, RequestId, RequestStatus, Room, RoomId,
    RoomRuleError,
};

use crate::ports::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole, MediaRoomCloser,
    RandomSource, RoomRepository, RoomRepositoryError, TranslationDispatcher,
};

#[derive(Debug, thiserror::Error)]
pub enum AdmissionError {
    #[error("room or request not found")]
    NotFound,
    #[error("key does not grant this action")]
    Forbidden,
    #[error(transparent)]
    Rule(#[from] RoomRuleError),
    #[error("media server is not configured")]
    MediaUnavailable,
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
    #[error(transparent)]
    MediaAccess(#[from] MediaAccessError),
}

/// A participant's ticket into a room's media session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinTicket {
    pub identity: String,
    pub grant: MediaGrant,
}

/// Returned once to a guest who asks to join; the key proves ownership.
#[derive(Debug, Clone)]
pub struct Knock {
    pub request_id: RequestId,
    pub key: AccessKey,
}

/// What a waiting guest learns when polling their request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnockOutcome {
    Pending,
    Denied,
    Admitted(JoinTicket),
}

/// Lobby flow: the host joins directly, guests knock and wait to be admitted.
#[derive(Clone)]
pub struct AdmissionService {
    rooms: Arc<dyn RoomRepository>,
    random: Arc<dyn RandomSource>,
    media: Option<Arc<dyn MediaAccessIssuer>>,
    translation: Option<Arc<dyn TranslationDispatcher>>,
    closer: Option<Arc<dyn MediaRoomCloser>>,
}

impl std::fmt::Debug for AdmissionService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AdmissionService")
            .finish_non_exhaustive()
    }
}

impl AdmissionService {
    #[must_use]
    pub fn new(
        rooms: Arc<dyn RoomRepository>,
        random: Arc<dyn RandomSource>,
        media: Option<Arc<dyn MediaAccessIssuer>>,
    ) -> Self {
        Self {
            rooms,
            random,
            media,
            translation: None,
            closer: None,
        }
    }

    /// Lets the host end the media session for everyone.
    #[must_use]
    pub fn with_room_closer(mut self, closer: Arc<dyn MediaRoomCloser>) -> Self {
        self.closer = Some(closer);
        self
    }

    /// Ends the meeting for everyone: nobody can join again and the media room
    /// is closed, which disconnects every participant.
    pub async fn end_call(
        &self,
        room_id: &RoomId,
        host_key: &AccessKey,
    ) -> Result<(), AdmissionError> {
        let mut room = self.load_as_host(room_id, host_key).await?;
        room.end();
        self.rooms.update(room).await?;
        if let Some(closer) = &self.closer
            && let Err(error) = closer.close(room_id).await
        {
            tracing::warn!(%error, %room_id, "media room not closed");
        }
        Ok(())
    }

    /// Enables live translation: the media agent is started when people join.
    #[must_use]
    pub fn with_translation(mut self, dispatcher: Arc<dyn TranslationDispatcher>) -> Self {
        self.translation = Some(dispatcher);
        self
    }

    pub async fn join_as_host(
        &self,
        room_id: &RoomId,
        host_key: &AccessKey,
        profile: &ParticipantProfile,
    ) -> Result<JoinTicket, AdmissionError> {
        let mut room = self.load(room_id).await?;
        if !room.is_host(host_key) {
            return Err(AdmissionError::Forbidden);
        }
        room.ensure_open()?;
        ensure_voice_belongs(&room, profile)?;
        let ticket = self.ticket(room_id, Some(profile), MediaRole::Speaker)?;
        room.set_host_name(profile.name.clone());
        self.rooms.update(room).await?;
        self.start_translation(room_id).await;
        Ok(ticket)
    }

    pub async fn knock(
        &self,
        room_id: &RoomId,
        profile: ParticipantProfile,
    ) -> Result<Knock, AdmissionError> {
        let mut room = self.load(room_id).await?;
        room.ensure_open()?;
        ensure_voice_belongs(&room, &profile)?;
        let request_id = RequestId::from_random_bytes(self.random.random_128());
        let key = AccessKey::from_random_bytes(self.random.random_128());
        room.knock(JoinRequest::new(request_id.clone(), profile, key.clone()))?;
        self.rooms.update(room).await?;
        Ok(Knock { request_id, key })
    }

    pub async fn pending(
        &self,
        room_id: &RoomId,
        host_key: &AccessKey,
    ) -> Result<Vec<JoinRequest>, AdmissionError> {
        let room = self.load_as_host(room_id, host_key).await?;
        Ok(room.pending().cloned().collect())
    }

    pub async fn decide(
        &self,
        room_id: &RoomId,
        host_key: &AccessKey,
        request_id: &RequestId,
        admit: bool,
    ) -> Result<(), AdmissionError> {
        let mut room = self.load_as_host(room_id, host_key).await?;
        room.decide(request_id, admit)?;
        self.rooms.update(room).await?;
        Ok(())
    }

    /// Polled by the waiting guest; issues media credentials once admitted.
    pub async fn outcome(
        &self,
        room_id: &RoomId,
        request_id: &RequestId,
        key: &AccessKey,
    ) -> Result<KnockOutcome, AdmissionError> {
        let room = self.load(room_id).await?;
        let request = room.request(request_id).ok_or(AdmissionError::NotFound)?;
        if &request.key != key {
            return Err(AdmissionError::Forbidden);
        }
        Ok(match request.status {
            RequestStatus::Pending => KnockOutcome::Pending,
            RequestStatus::Denied => KnockOutcome::Denied,
            RequestStatus::Admitted => {
                room.ensure_open()?;
                let ticket = self.ticket(room_id, Some(&request.profile), MediaRole::Speaker)?;
                self.start_translation(room_id).await;
                KnockOutcome::Admitted(ticket)
            }
        })
    }

    /// Hidden listener for the host or an admitted guest to hear themselves.
    pub async fn monitor(
        &self,
        room_id: &RoomId,
        key: &AccessKey,
    ) -> Result<JoinTicket, AdmissionError> {
        let room = self.load(room_id).await?;
        if !room.is_member(key) {
            return Err(AdmissionError::Forbidden);
        }
        room.ensure_open()?;
        self.ticket(room_id, None, MediaRole::Monitor)
    }

    async fn load(&self, room_id: &RoomId) -> Result<Room, AdmissionError> {
        self.rooms
            .find(room_id)
            .await?
            .ok_or(AdmissionError::NotFound)
    }

    async fn load_as_host(
        &self,
        room_id: &RoomId,
        key: &AccessKey,
    ) -> Result<Room, AdmissionError> {
        let room = self.load(room_id).await?;
        if room.is_host(key) {
            Ok(room)
        } else {
            Err(AdmissionError::Forbidden)
        }
    }

    /// Best effort: a failure to start translation must not block joining.
    async fn start_translation(&self, room_id: &RoomId) {
        if let Some(dispatcher) = &self.translation
            && let Err(error) = dispatcher.start(room_id).await
        {
            tracing::warn!(%error, %room_id, "translation not started");
        }
    }

    fn ticket(
        &self,
        room_id: &RoomId,
        profile: Option<&ParticipantProfile>,
        role: MediaRole,
    ) -> Result<JoinTicket, AdmissionError> {
        let media = self
            .media
            .as_ref()
            .ok_or(AdmissionError::MediaUnavailable)?;
        let prefix = if role == MediaRole::Monitor {
            "monitor"
        } else {
            "p"
        };
        let identity = format!(
            "{prefix}-{}",
            RequestId::from_random_bytes(self.random.random_128())
        );
        let grant = media.issue(
            room_id,
            MediaParticipant {
                identity: &identity,
                profile,
                role,
            },
        )?;
        Ok(JoinTicket { identity, grant })
    }
}

/// A participant may only use a voice clone recorded in this room.
fn ensure_voice_belongs(room: &Room, profile: &ParticipantProfile) -> Result<(), AdmissionError> {
    match &profile.voice_id {
        Some(voice) if !room.owns_voice(voice) => Err(AdmissionError::Forbidden),
        _ => Ok(()),
    }
}

#[cfg(test)]
#[path = "admission_tests.rs"]
mod tests;
