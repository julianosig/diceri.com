use std::sync::Arc;

use diceri_domain::{
    AccessKey, AccountId, JoinRequest, ParticipantProfile, RequestId, RequestStatus, Room,
    RoomCode, RoomId, RoomRuleError,
};

use crate::ports::{
    Clock, IdGenerator, MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant,
    MediaPresence, MediaRole, MediaRoomCloser, RandomSource, RoomChangeError, RoomRepository,
    RoomRepositoryError, TranslationDispatcher,
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
    /// Who is in the call could not be checked just now; asking again may work.
    #[error("the call could not be checked, try again")]
    PresenceUnknown,
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
    #[error(transparent)]
    MediaAccess(#[from] MediaAccessError),
}

impl From<RoomChangeError> for AdmissionError {
    fn from(error: RoomChangeError) -> Self {
        match error {
            RoomChangeError::Rule(rule) => Self::Rule(rule),
            RoomChangeError::Repository(repository) => Self::Repository(repository),
        }
    }
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
/// Callers address rooms by their link code; media and logs use the room id.
#[derive(Clone)]
pub struct AdmissionService {
    rooms: Arc<dyn RoomRepository>,
    random: Arc<dyn RandomSource>,
    ids: Arc<dyn IdGenerator>,
    media: Option<Arc<dyn MediaAccessIssuer>>,
    translation: Option<Arc<dyn TranslationDispatcher>>,
    closer: Option<Arc<dyn MediaRoomCloser>>,
    /// Who is in each call and the time, to keep calls to their size; `None`: not checked.
    presence: Option<(Arc<dyn MediaPresence>, Arc<dyn Clock>)>,
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
        ids: Arc<dyn IdGenerator>,
        media: Option<Arc<dyn MediaAccessIssuer>>,
    ) -> Self {
        Self {
            rooms,
            random,
            ids,
            media,
            translation: None,
            closer: None,
            presence: None,
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
        code: &RoomCode,
        host_key: &AccessKey,
    ) -> Result<(), AdmissionError> {
        let room_id = self.load_as_host(code, host_key).await?.id;
        let end = |room: &mut Room| {
            room.end();
            Ok(())
        };
        self.rooms.modify(&room_id, Box::new(end)).await?;
        if let Some(closer) = &self.closer
            && let Err(error) = closer.close(&room_id).await
        {
            tracing::warn!(%error, %room_id, "media room not closed");
        }
        Ok(())
    }

    /// Keeps calls to their size (`Room::max_participants`) using who is connected now.
    #[must_use]
    pub fn with_presence(
        mut self,
        presence: Arc<dyn MediaPresence>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        self.presence = Some((presence, clock));
        self
    }

    /// Enables live translation: the media agent is started when people join.
    #[must_use]
    pub fn with_translation(mut self, dispatcher: Arc<dyn TranslationDispatcher>) -> Self {
        self.translation = Some(dispatcher);
        self
    }

    /// The room, once `host_key` is its host's and `profile` may join it: checked before
    /// anything is spent on the call (its place in the service's capacity).
    pub async fn check_host(
        &self,
        code: &RoomCode,
        host_key: &AccessKey,
        account: &AccountId,
        profile: &ParticipantProfile,
    ) -> Result<Room, AdmissionError> {
        let room = self.load(code).await?;
        if !room.is_host(host_key) {
            return Err(AdmissionError::Forbidden);
        }
        room.ensure_open()?;
        ensure_voice_belongs(&room, account, profile)?;
        Ok(room)
    }

    pub async fn join_as_host(
        &self,
        code: &RoomCode,
        host_key: &AccessKey,
        account: &AccountId,
        profile: &ParticipantProfile,
    ) -> Result<JoinTicket, AdmissionError> {
        let room = self.check_host(code, host_key, account, profile).await?;
        let identity = self.seat(&room, account).await?;
        let ticket = self.grant(&room.id, identity, Some(profile), MediaRole::Speaker)?;
        self.start_translation(&room.id).await;
        self.ensure_still_open(&room.id).await?;
        Ok(ticket)
    }

    pub async fn knock(
        &self,
        code: &RoomCode,
        account: &AccountId,
        profile: ParticipantProfile,
    ) -> Result<Knock, AdmissionError> {
        let room = self.load(code).await?;
        room.ensure_open()?;
        ensure_voice_belongs(&room, account, &profile)?;
        // Said before waiting for the host, not after being let in.
        self.ensure_space(&room, account).await?;
        let request_id = self.ids.request_id();
        let key = AccessKey::from_random_bytes(self.random.random_128());
        let request = JoinRequest::new(request_id, *account, profile, key.clone());
        let knock = move |room: &mut Room| {
            room.ensure_open()?;
            room.knock(request)
        };
        self.rooms.modify(&room.id, Box::new(knock)).await?;
        Ok(Knock { request_id, key })
    }

    pub async fn pending(
        &self,
        code: &RoomCode,
        host_key: &AccessKey,
    ) -> Result<Vec<JoinRequest>, AdmissionError> {
        let room = self.load_as_host(code, host_key).await?;
        Ok(room.pending().cloned().collect())
    }

    pub async fn decide(
        &self,
        code: &RoomCode,
        host_key: &AccessKey,
        request_id: &RequestId,
        admit: bool,
    ) -> Result<(), AdmissionError> {
        let room_id = self.load_as_host(code, host_key).await?.id;
        let request_id = *request_id;
        let decide = move |room: &mut Room| room.decide(&request_id, admit);
        self.rooms.modify(&room_id, Box::new(decide)).await?;
        Ok(())
    }

    /// Polled by the waiting guest; issues media credentials once admitted.
    /// Where `request_id` stands, for its holder only; changes nothing.
    pub async fn request_status(
        &self,
        code: &RoomCode,
        request_id: &RequestId,
        key: &AccessKey,
    ) -> Result<RequestStatus, AdmissionError> {
        let room = self.load(code).await?;
        let request = room.request(request_id).ok_or(AdmissionError::NotFound)?;
        if &request.key != key {
            return Err(AdmissionError::Forbidden);
        }
        // An ended call is said so first: nothing is reserved for it.
        room.ensure_open()?;
        Ok(request.status)
    }

    pub async fn outcome(
        &self,
        code: &RoomCode,
        request_id: &RequestId,
        key: &AccessKey,
    ) -> Result<KnockOutcome, AdmissionError> {
        let room = self.load(code).await?;
        let request = room.request(request_id).ok_or(AdmissionError::NotFound)?;
        if &request.key != key {
            return Err(AdmissionError::Forbidden);
        }
        Ok(match request.status {
            RequestStatus::Pending => KnockOutcome::Pending,
            RequestStatus::Denied => KnockOutcome::Denied,
            RequestStatus::Admitted => {
                room.ensure_open()?;
                let identity = self.seat(&room, &request.account).await?;
                let ticket = self.grant(
                    &room.id,
                    identity,
                    Some(&request.profile),
                    MediaRole::Speaker,
                )?;
                self.start_translation(&room.id).await;
                self.ensure_still_open(&room.id).await?;
                KnockOutcome::Admitted(ticket)
            }
        })
    }

    /// Hidden listener for the host or an admitted guest to hear themselves.
    pub async fn monitor(
        &self,
        code: &RoomCode,
        key: &AccessKey,
    ) -> Result<JoinTicket, AdmissionError> {
        let room = self.load(code).await?;
        // Only someone the media server shows in the call right now may listen to it; the
        // listener is named after their seat, so it goes when they leave.
        let seat = room
            .seat_of_key(key)
            .ok_or(AdmissionError::Forbidden)?
            .to_owned();
        room.ensure_open()?;
        // Without a media server watching calls (development) the seat is enough.
        if self.presence.is_some() {
            let (present, _) = self.who_is_in(&room).await;
            let present = present.ok_or(AdmissionError::PresenceUnknown)?;
            if !present.contains(&seat) {
                return Err(AdmissionError::Forbidden);
            }
        }
        self.ensure_still_open(&room.id).await?;
        let identity = format!("monitor-{seat}");
        self.grant(&room.id, identity, None, MediaRole::Monitor)
    }

    async fn load(&self, code: &RoomCode) -> Result<Room, AdmissionError> {
        self.rooms
            .find_by_code(code)
            .await?
            .ok_or(AdmissionError::NotFound)
    }

    async fn load_as_host(&self, code: &RoomCode, key: &AccessKey) -> Result<Room, AdmissionError> {
        let room = self.load(code).await?;
        if room.is_host(key) {
            Ok(room)
        } else {
            Err(AdmissionError::Forbidden)
        }
    }

    /// Starting translation can take seconds: a call ended meanwhile hands out no credentials.
    async fn ensure_still_open(&self, room: &RoomId) -> Result<(), AdmissionError> {
        let room = self
            .rooms
            .find_by_id(room)
            .await?
            .ok_or(AdmissionError::NotFound)?;
        Ok(room.ensure_open()?)
    }

    /// Best effort: a failure to start translation must not block joining.
    /// With its own short deadline, so a slow agent never makes joining time out.
    async fn start_translation(&self, room_id: &RoomId) {
        let Some(dispatcher) = &self.translation else {
            return;
        };
        match tokio::time::timeout(DISPATCH_TIMEOUT, dispatcher.start(room_id)).await {
            Ok(Ok(())) => {}
            Ok(Err(error)) => warn_not_started(room_id, &error.to_string()),
            Err(_) => warn_not_started(room_id, "translation agent too slow"),
        }
    }

    fn grant(
        &self,
        room_id: &RoomId,
        identity: String,
        profile: Option<&ParticipantProfile>,
        role: MediaRole,
    ) -> Result<JoinTicket, AdmissionError> {
        let media = self
            .media
            .as_ref()
            .ok_or(AdmissionError::MediaUnavailable)?;
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

/// How long joining waits for the translation agent to accept the call.
const DISPATCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);

fn warn_not_started(room_id: &RoomId, reason: &str) {
    tracing::warn!(%room_id, reason, "translation not started");
}

/// A participant may only speak in a voice clone they themselves recorded for this room.
fn ensure_voice_belongs(
    room: &Room,
    account: &AccountId,
    profile: &ParticipantProfile,
) -> Result<(), AdmissionError> {
    match &profile.voice_id {
        Some(voice) if !room.has_voice_of(account, voice) => Err(AdmissionError::Forbidden),
        _ => Ok(()),
    }
}

mod seats;

#[cfg(test)]
mod seat_tests;
#[cfg(test)]
mod tests;
