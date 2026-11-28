//! The meter's work with the media server: who is in each call, and closing or tidying
//! media rooms, each with a deadline so a slow server never stalls the meter.

use std::sync::Arc;

use diceri_domain::RoomId;

use crate::ports::{CallMembers, MediaAccessError, MediaPresence, MediaRoomCloser};

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

/// Whether a listener (named `monitor-<seat>`) has no person of its seat in the call.
fn is_orphan(listener: &str, people: &[String]) -> bool {
    listener
        .strip_prefix("monitor-")
        .is_none_or(|seat| !people.iter().any(|person| person == seat))
}

pub(super) fn has_orphan_listeners(members: &CallMembers) -> bool {
    members
        .listeners
        .iter()
        .any(|listener| is_orphan(listener, &members.people))
}

/// Hidden listeners are named `monitor-<seat>`: one whose person is not in the call goes.
pub(super) async fn remove_orphan_listeners(
    closer: &dyn MediaRoomCloser,
    room: &RoomId,
    CallMembers { people, listeners }: CallMembers,
) {
    for listener in listeners {
        if is_orphan(&listener, &people)
            && let Err(error) = closer.remove(room, &listener).await
        {
            tracing::warn!(%error, room_id = %room, "listener not removed");
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
