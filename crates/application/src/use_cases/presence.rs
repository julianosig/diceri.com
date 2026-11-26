use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use diceri_domain::{EntryPolicy, LanguageCode, Mascot, Room, RoomCode, Username};
use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::ports::{LiveRoom, MediaPresence, RoomRepository};

/// How many rooms the public ranking shows.
pub const RANKING_SIZE: usize = 10;
/// The home page polls; one media-server query per window serves everyone.
const SNAPSHOT_TTL: Duration = Duration::from_secs(10);

/// Public view of activity: totals over every room of this deployment, and a
/// ranking of only the rooms their hosts chose to list.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PresenceSnapshot {
    pub online: usize,
    pub live_rooms: usize,
    pub ranking: Vec<RankedRoom>,
}

/// One listed room: whose it is (its name), how to get in, and how busy it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedRoom {
    /// The room's code: listing a room publishes it, by the host's choice.
    pub code: RoomCode,
    /// The owner, which is also the room's name (`@owner`, reached at `/@owner`).
    pub owner: Username,
    pub entry: EntryPolicy,
    pub mascot: Mascot,
    pub people: usize,
    /// Most listened first.
    pub languages: Vec<LanguageCode>,
}

type Cached = Option<(Instant, PresenceSnapshot)>;

/// Who is online now, cached for a few seconds.
#[derive(Clone)]
pub struct PresenceService {
    source: Option<Arc<dyn MediaPresence>>,
    rooms: Arc<dyn RoomRepository>,
    cache: Arc<Mutex<Cached>>,
}

impl std::fmt::Debug for PresenceService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PresenceService")
            .finish_non_exhaustive()
    }
}

impl PresenceService {
    /// `source: None` (no media server configured) always reports an empty snapshot.
    #[must_use]
    pub fn new(source: Option<Arc<dyn MediaPresence>>, rooms: Arc<dyn RoomRepository>) -> Self {
        Self {
            source,
            rooms,
            cache: Arc::new(Mutex::new(None)),
        }
    }

    /// Uses `source` to see who is connected (replacing any previous one).
    #[must_use]
    pub fn with_source(mut self, source: Arc<dyn MediaPresence>) -> Self {
        self.source = Some(source);
        self
    }

    /// Current snapshot. On a provider failure, the last one (or empty) is
    /// served and kept for the same window, so failures are not retried per request.
    pub async fn snapshot(&self) -> PresenceSnapshot {
        let Some(source) = &self.source else {
            return PresenceSnapshot::default();
        };
        // Held across the query on purpose: concurrent callers wait for one refresh.
        let mut cache = self.cache.lock().await;
        if let Some((at, snapshot)) = cache.as_ref()
            && at.elapsed() < SNAPSHOT_TTL
        {
            return snapshot.clone();
        }
        let snapshot = match source.live_rooms().await {
            Ok(live) => summarize(self.known(live).await),
            Err(error) => {
                tracing::warn!(%error, "presence unavailable");
                cache.take().map(|(_, last)| last).unwrap_or_default()
            }
        };
        *cache = Some((Instant::now(), snapshot.clone()));
        snapshot
    }

    /// Pairs live media rooms with this deployment's open rooms; others (another
    /// deployment sharing the media project, or ended meetings) are ignored.
    async fn known(&self, live: Vec<LiveRoom>) -> Vec<(Room, LiveRoom)> {
        let mut known = Vec::new();
        for live_room in live.into_iter().filter(|room| room.people > 0) {
            if let Ok(Some(room)) = self.rooms.find_by_id(&live_room.room).await
                && !room.is_ended()
            {
                known.push((room, live_room));
            }
        }
        known
    }
}

fn summarize(mut rooms: Vec<(Room, LiveRoom)>) -> PresenceSnapshot {
    rooms.sort_by_key(|(_, live)| std::cmp::Reverse(live.people));
    PresenceSnapshot {
        online: rooms.iter().map(|(_, live)| live.people).sum(),
        live_rooms: rooms.len(),
        ranking: rooms
            .into_iter()
            .filter(|(room, _)| room.settings.listed)
            .take(RANKING_SIZE)
            .map(|(room, live)| RankedRoom {
                owner: room.owner,
                code: room.code,
                entry: room.settings.entry,
                mascot: room.settings.mascot,
                people: live.people,
                languages: by_popularity(live.languages),
            })
            .collect(),
    }
}

/// Unique languages, most frequent first; ties keep first-seen order.
fn by_popularity(languages: Vec<LanguageCode>) -> Vec<LanguageCode> {
    let mut counts: HashMap<LanguageCode, usize> = HashMap::new();
    let mut unique = Vec::new();
    for language in languages {
        let count = counts.entry(language.clone()).or_insert(0);
        if *count == 0 {
            unique.push(language);
        }
        *count += 1;
    }
    unique.sort_by_key(|language| std::cmp::Reverse(counts.get(language).copied()));
    unique
}

#[cfg(test)]
#[path = "presence_tests.rs"]
mod tests;
