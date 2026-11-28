//! Acting on what the media server reports as it happens (ADR 0018): people beyond a call's
//! size, listeners without their person, and calls that ended are removed at once. Only
//! removing people or closing calls happens here, never letting anyone in.

use std::sync::Arc;

use diceri_domain::{Room, RoomId};

use super::MeteringService;
use crate::ports::{MediaAccessError, MediaEvent, MediaPresence, MediaRoomCloser, RoomRepository};

/// Media identities: people are `p-<id>`, hidden listeners `monitor-<their person's identity>`.
const PERSON_PREFIX: &str = "p-";
const LISTENER_PREFIX: &str = "monitor-";

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
        if let Some(person) = identity.strip_prefix(LISTENER_PREFIX) {
            if !self.is_in_call(room_id, person).await.unwrap_or(true) {
                self.remove(room_id, identity).await;
            }
            return;
        }
        if identity.starts_with(PERSON_PREFIX) {
            if let Some(metering) = &self.metering {
                metering.note_live(room_id, room.owner.clone());
            }
            self.keep_to_size(&room).await;
        }
    }

    /// Removes whoever is beyond the call's size, by the room's one rule (the owner stays,
    /// then the newest grants), so every event and the sweep agree on who goes, whoever
    /// joined last. Unknown presence: nothing (the sweep checks again).
    async fn keep_to_size(&self, room: &Room) {
        let Ok(members) = self.presence.call_members(&room.id).await else {
            return;
        };
        for identity in room.beyond_size(&members.people) {
            self.remove(room.id, &identity).await;
        }
    }

    /// `None` when the media server cannot say.
    async fn is_in_call(&self, room: RoomId, person: &str) -> Option<bool> {
        let members = self.presence.call_members(&room).await.ok()?;
        Some(members.people.iter().any(|identity| identity == person))
    }

    /// Their hidden listener goes with them, unless they are back already (a reload reuses
    /// the same identities, and a report may come late or twice).
    async fn left(&self, room: RoomId, identity: &str) {
        if !identity.starts_with(PERSON_PREFIX)
            || !matches!(self.rooms.find_by_id(&room).await, Ok(Some(_)))
        {
            return;
        }
        if self.is_in_call(room, identity).await == Some(false) {
            self.remove(room, &format!("{LISTENER_PREFIX}{identity}"))
                .await;
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
