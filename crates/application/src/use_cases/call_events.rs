//! Acting on what the media server reports as it happens (ADR 0018): people beyond a call's
//! size, listeners without their person, and calls that ended are removed at once. Only
//! removing people or closing calls happens here, never letting anyone in.

use std::sync::Arc;

use diceri_domain::{RoomId, surplus_listeners};

use super::MeteringService;
use crate::ports::{MediaAccessError, MediaEvent, MediaPresence, MediaRoomCloser, RoomRepository};

/// People's media identities; listeners are named after them (`diceri_domain::listener_identity`).
const PERSON_PREFIX: &str = "p-";

#[derive(Clone)]
pub struct CallEventsService {
    rooms: Arc<dyn RoomRepository>,
    presence: Arc<dyn MediaPresence>,
    closer: Arc<dyn MediaRoomCloser>,
    /// Told when calls start and finish, so the service's capacity sees them at once.
    metering: Option<MeteringService>,
}

impl std::fmt::Debug for CallEventsService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CallEventsService")
            .finish_non_exhaustive()
    }
}

impl CallEventsService {
    #[must_use]
    pub fn new(
        rooms: Arc<dyn RoomRepository>,
        presence: Arc<dyn MediaPresence>,
        closer: Arc<dyn MediaRoomCloser>,
    ) -> Self {
        Self {
            rooms,
            presence,
            closer,
            metering: None,
        }
    }

    #[must_use]
    pub fn with_metering(mut self, metering: MeteringService) -> Self {
        self.metering = Some(metering);
        self
    }

    /// Acts on one event. Every action is safe to repeat: events may arrive twice or late.
    pub async fn handle(&self, event: MediaEvent) {
        match event {
            MediaEvent::Joined { room, identity } => self.joined(room, &identity).await,
            MediaEvent::Left { room, identity } => self.left(room, &identity).await,
            MediaEvent::Finished { room } => self.finished(room).await,
        }
    }

    async fn joined(&self, room_id: RoomId, identity: &str) {
        // Rooms of another deployment sharing the media server are none of ours.
        let Ok(Some(room)) = self.rooms.find_by_id(&room_id).await else {
            return;
        };
        if room.is_ended() {
            // Credentials issued just before the end: the call stays ended.
            self.close(room_id).await;
            return;
        }
        if identity.starts_with(PERSON_PREFIX)
            && let Some(metering) = &self.metering
        {
            metering.note_live(room_id, room.owner.clone());
        }
        self.tidy(room_id).await;
    }

    /// A person leaving takes their listener along; anything else is tidied the same way.
    async fn left(&self, room_id: RoomId, identity: &str) {
        if identity.starts_with(PERSON_PREFIX) {
            self.tidy(room_id).await;
        }
    }

    /// Removes people beyond the call's size and listeners without their person (or beyond
    /// one per person), all from one look at the call. The room is read after that look, so
    /// a seat granted meanwhile is counted, and by the room's one rule (owner kept, newest
    /// grants next), so every report and the sweep agree on who goes whatever their order.
    /// Only connections that look showed are removed: a listener connecting since has its
    /// own identity and is never hit. Unknown presence or room: nothing (the sweep retries).
    async fn tidy(&self, room_id: RoomId) {
        let Ok(members) = self.presence.call_members(&room_id).await else {
            return;
        };
        let Ok(Some(room)) = self.rooms.find_by_id(&room_id).await else {
            return;
        };
        let beyond = room.beyond_size(&members.people);
        let surplus = surplus_listeners(&members.people, &members.listeners);
        for identity in beyond.iter().chain(&surplus) {
            self.remove(room_id, identity).await;
        }
    }

    async fn finished(&self, room_id: RoomId) {
        let (Some(metering), Ok(Some(room))) =
            (&self.metering, self.rooms.find_by_id(&room_id).await)
        else {
            return;
        };
        // A late report of an older call never stops the owner's new one from counting.
        let newer = matches!(
            self.rooms.find_live_by_owner(&room.owner).await,
            Ok(Some(live)) if live.id != room_id
        );
        metering.note_finished(room_id, (!newer).then_some(&room.owner));
    }

    async fn remove(&self, room: RoomId, identity: &str) {
        log_action(room, identity);
        if let Err(error) = self.closer.remove(&room, identity).await {
            warn_failed(room, &error);
        }
    }

    async fn close(&self, room: RoomId) {
        log_action(room, "everyone (the call had ended)");
        if let Err(error) = self.closer.close(&room).await {
            warn_failed(room, &error);
        }
    }
}

fn log_action(room: RoomId, removed: &str) {
    tracing::info!(room_id = %room, removed, "removed from the call");
}

fn warn_failed(room: RoomId, error: &MediaAccessError) {
    tracing::warn!(%error, room_id = %room, "could not act on a media event");
}

#[cfg(test)]
#[path = "call_events_tests.rs"]
mod tests;
