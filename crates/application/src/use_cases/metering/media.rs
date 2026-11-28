//! The meter's work with the media server: who is in each call, and closing or tidying
//! media rooms, each with a deadline so a slow server never stalls the meter.

use std::sync::Arc;

use diceri_domain::RoomId;

use crate::ports::{CallMembers, MediaAccessError, MediaPresence, MediaRoomCloser};

/// Whether anyone is in `room`'s call (a provider error counts as nobody).
/// Whether anyone is in `room`'s call (a provider error counts as nobody).
pub(super) async fn someone_in(presence: &dyn MediaPresence, room: &RoomId) -> bool {
    match presence.people_in(room).await {
        Ok(people) => people > 0,
        Err(error) => {
            tracing::warn!(%error, room_id = %room, "usage meter could not see the call");
            false
        }
    }
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

/// Disconnects everyone from the calls that reached their plan's limit, all at once, each
/// with its own deadline.
pub(super) async fn close_at_limit(closer: &Arc<dyn MediaRoomCloser>, rooms: Vec<RoomId>) {
    let mut closing = tokio::task::JoinSet::new();
    for room in rooms {
        tracing::info!(room_id = %room, "call ended: plan limit reached");
        let closer = closer.clone();
        closing.spawn(async move {
            if tokio::time::timeout(CLOSE_TIMEOUT, close_ended(closer.as_ref(), &room))
                .await
                .is_err()
            {
                warn_close_timed_out(&room);
            }
        });
    }
    while closing.join_next().await.is_some() {}
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
