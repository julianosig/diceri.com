//! The meter's work with the media server: who is in each call, and closing or tidying
//! media rooms, each with a deadline so a slow server never stalls the meter.

use std::collections::HashSet;
use std::sync::Arc;

use diceri_domain::RoomId;

use crate::ports::{CallMembers, MediaAccessError, MediaPresence, MediaRoomCloser};

/// The rooms with someone in their call, asked all at once.
pub(super) async fn occupied(
    presence: &Arc<dyn MediaPresence>,
    rooms: impl Iterator<Item = RoomId>,
) -> HashSet<RoomId> {
    let mut checks = tokio::task::JoinSet::new();
    for room in rooms {
        let presence = presence.clone();
        checks.spawn(async move { (room, someone_in(presence.as_ref(), &room).await) });
    }
    let mut busy = HashSet::new();
    while let Some(Ok((room, someone))) = checks.join_next().await {
        if someone {
            busy.insert(room);
        }
    }
    busy
}

/// Whether anyone is in `room`'s call (a provider error counts as nobody).
pub(super) async fn someone_in(presence: &dyn MediaPresence, room: &RoomId) -> bool {
    match tokio::time::timeout(CLOSE_TIMEOUT, presence.people_in(room)).await {
        Ok(Ok(people)) => people > 0,
        Ok(Err(error)) => warn_unseen(room, &error.to_string()),
        Err(_) => warn_unseen(room, "media server too slow"),
    }
}

pub(super) fn warn_unseen(room: &RoomId, reason: &str) -> bool {
    tracing::warn!(room_id = %room, reason, "usage meter could not see the call");
    false
}

pub(super) async fn close_ended(closer: &dyn MediaRoomCloser, room: &RoomId) {
    if let Err(error) = closer.close(room).await {
        warn_not_closed(&error, room);
    }
}

/// Hidden listeners are named `monitor-<seat>`: one whose person is not in the call goes.
pub(super) async fn remove_orphan_listeners(
    presence: &dyn MediaPresence,
    closer: &dyn MediaRoomCloser,
    room: &RoomId,
) {
    let Ok(CallMembers { people, listeners }) = presence.call_members(room).await else {
        return;
    };
    for listener in listeners {
        let seat = listener.strip_prefix("monitor-");
        if seat.is_none_or(|seat| !people.iter().any(|person| person == seat))
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
