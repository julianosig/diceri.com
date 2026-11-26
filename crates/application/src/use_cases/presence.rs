use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use diceri_domain::LanguageCode;
use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::ports::{LiveRoom, MediaPresence};

/// How many rooms the public ranking shows.
pub const RANKING_SIZE: usize = 10;
/// The home page polls; one media-server query per window serves everyone.
const SNAPSHOT_TTL: Duration = Duration::from_secs(10);

/// Public, anonymous view of activity: totals plus the biggest rooms.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PresenceSnapshot {
    pub online: usize,
    pub live_rooms: usize,
    pub ranking: Vec<RankedRoom>,
}

/// One ranked room: its size and languages, most listened first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedRoom {
    pub people: usize,
    pub languages: Vec<LanguageCode>,
}

type Cached = Option<(Instant, PresenceSnapshot)>;

/// Who is online now, cached for a few seconds.
#[derive(Clone)]
pub struct PresenceService {
    source: Option<Arc<dyn MediaPresence>>,
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
    /// `None` (no media server configured) always reports an empty snapshot.
    #[must_use]
    pub fn new(source: Option<Arc<dyn MediaPresence>>) -> Self {
        Self {
            source,
            cache: Arc::new(Mutex::new(None)),
        }
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
            Ok(rooms) => summarize(rooms),
            Err(error) => {
                tracing::warn!(%error, "presence unavailable");
                cache.take().map(|(_, last)| last).unwrap_or_default()
            }
        };
        *cache = Some((Instant::now(), snapshot.clone()));
        snapshot
    }
}

fn summarize(rooms: Vec<LiveRoom>) -> PresenceSnapshot {
    let mut live: Vec<LiveRoom> = rooms.into_iter().filter(|room| room.people > 0).collect();
    live.sort_by_key(|room| std::cmp::Reverse(room.people));
    PresenceSnapshot {
        online: live.iter().map(|room| room.people).sum(),
        live_rooms: live.len(),
        ranking: live
            .into_iter()
            .take(RANKING_SIZE)
            .map(|room| RankedRoom {
                people: room.people,
                languages: by_popularity(room.languages),
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
