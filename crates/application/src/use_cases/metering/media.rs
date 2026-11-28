//! The meter's work with the media server: who is in each call, and closing or tidying
//! media rooms, each with a deadline so a slow server never stalls the meter.

use std::sync::Arc;

use diceri_domain::{Room, RoomId, surplus_listeners};

use crate::ports::{CallMembers, MediaAccessError, MediaPresence, MediaRoomCloser, RoomRepository};

/// Every media room open now, from one listing; `None` when the media server cannot say.
pub(super) async fn open_media_rooms(presence: &dyn MediaPresence) -> Option<Vec<RoomId>> {
    match tokio::time::timeout(CLOSE_TIMEOUT, presence.open_rooms()).await {
        Ok(Ok(open)) => Some(open),
        Ok(Err(error)) => warn_unseen(None, &error.to_string()),
        Err(_) => warn_unseen(None, "media server too slow"),
    }
}

/// Who is in `room`'s call; `None` when the media server cannot say (counted as nobody).
pub(super) async fn members_of(presence: &dyn MediaPresence, room: &RoomId) -> Option<CallMembers> {
    match tokio::time::timeout(CLOSE_TIMEOUT, presence.call_members(room)).await {
        Ok(Ok(members)) => Some(members),
        Ok(Err(error)) => warn_unseen(Some(room), &error.to_string()),
        Err(_) => warn_unseen(Some(room), "media server too slow"),
    }
}

fn warn_unseen<T>(room: Option<&RoomId>, reason: &str) -> Option<T> {
    tracing::warn!(room_id = ?room, reason, "usage meter could not see the calls");
    None
}

/// Runs `tidy` for at most `CLOSE_TIMEOUT`.
pub(super) async fn within_deadline(tidy: impl Future<Output = ()>, room: RoomId) {
    if tokio::time::timeout(CLOSE_TIMEOUT, tidy).await.is_err() {
        tracing::warn!(room_id = %room, "media room tidy-up timed out");
    }
}

pub(super) async fn close_ended(closer: &dyn MediaRoomCloser, room: &RoomId) {
    if let Err(error) = closer.close(room).await {
        warn_not_closed(&error, room);
    }
}

/// Whether `room`'s call holds people beyond its size or listeners to remove.
pub(super) fn needs_tidy(room: &Room, members: &CallMembers) -> bool {
    !room.beyond_size(&members.people).is_empty()
        || !surplus_listeners(&members.people, &members.listeners).is_empty()
}

/// Removes people beyond the call's size and surplus listeners from `members` (a fresh
/// look), by the same rules as media events; the room is read after that look, so a seat
/// granted meanwhile counts.
pub(super) async fn tidy_members(
    closer: &dyn MediaRoomCloser,
    rooms: &dyn RoomRepository,
    room: &RoomId,
    members: CallMembers,
) {
    let Ok(Some(stored)) = rooms.find_by_id(room).await else {
        return;
    };
    let beyond = stored.beyond_size(&members.people);
    let surplus = surplus_listeners(&members.people, &members.listeners);
    for identity in beyond.iter().chain(&surplus) {
        if let Err(error) = closer.remove(room, identity).await {
            tracing::warn!(%error, room_id = %room, "not removed from the call");
        }
    }
}

/// Runs `work` for at most `SWEEP_BUDGET`: a slow media server never stalls the meter.
pub(super) async fn within_budget(work: impl Future<Output = ()>, what: &str) {
    if tokio::time::timeout(SWEEP_BUDGET, work).await.is_err() {
        tracing::warn!(what, "took too long; the rest waits for the next minute");
    }
}

/// Disconnects everyone from a call that reached its plan's limit, within a deadline.
pub(super) async fn close_within_deadline(closer: Arc<dyn MediaRoomCloser>, room: RoomId) {
    tracing::info!(room_id = %room, "call ended: plan limit reached");
    if tokio::time::timeout(CLOSE_TIMEOUT, close_ended(closer.as_ref(), &room))
        .await
        .is_err()
    {
        warn_close_timed_out(&room);
    }
}

pub(super) fn warn_close_timed_out(room: &RoomId) {
    tracing::warn!(room_id = %room, "media room close timed out");
}

pub(super) fn warn_not_closed(error: &MediaAccessError, room: &RoomId) {
    tracing::warn!(%error, room_id = %room, "media room not closed");
}

/// How long tidying one media room (closing it, or removing listeners) may take.
pub(super) const CLOSE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// How long the minute's sweep of ended calls may take before charging goes ahead.
pub(super) const SWEEP_BUDGET: std::time::Duration = std::time::Duration::from_secs(10);
