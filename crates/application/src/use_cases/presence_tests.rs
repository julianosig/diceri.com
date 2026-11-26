use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;

use super::{PresenceService, RANKING_SIZE};
use crate::ports::{LiveRoom, MediaAccessError, MediaPresence};

struct FakePresence {
    rooms: Vec<LiveRoom>,
    calls: AtomicUsize,
    fail: bool,
}

#[async_trait]
impl MediaPresence for FakePresence {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        if self.fail {
            return Err(MediaAccessError("down".to_owned()));
        }
        Ok(self.rooms.clone())
    }
}

fn room(people: usize, languages: &[&str]) -> LiveRoom {
    LiveRoom {
        people,
        languages: languages.iter().map(|code| code.parse().unwrap()).collect(),
    }
}

fn service(rooms: Vec<LiveRoom>, fail: bool) -> (PresenceService, Arc<FakePresence>) {
    let fake = Arc::new(FakePresence {
        rooms,
        calls: AtomicUsize::new(0),
        fail,
    });
    (PresenceService::new(Some(fake.clone())), fake)
}

#[tokio::test]
async fn ranks_rooms_by_size_and_counts_everyone_online() {
    let (presence, _) = service(
        vec![
            room(2, &["en", "pt-BR"]),
            room(0, &[]),
            room(3, &["es", "en", "en"]),
        ],
        false,
    );
    let snapshot = presence.snapshot().await;
    assert_eq!(snapshot.online, 5);
    assert_eq!(snapshot.live_rooms, 2);
    let sizes: Vec<usize> = snapshot.ranking.iter().map(|r| r.people).collect();
    assert_eq!(sizes, [3, 2]);
    let first: Vec<&str> = snapshot.ranking[0]
        .languages
        .iter()
        .map(diceri_domain::LanguageCode::as_str)
        .collect();
    assert_eq!(first, ["en", "es"]);
}

#[tokio::test]
async fn ranking_is_capped_but_totals_are_not() {
    let rooms = (0..RANKING_SIZE + 5).map(|_| room(1, &["en"])).collect();
    let (presence, _) = service(rooms, false);
    let snapshot = presence.snapshot().await;
    assert_eq!(snapshot.ranking.len(), RANKING_SIZE);
    assert_eq!(snapshot.online, RANKING_SIZE + 5);
}

#[tokio::test]
async fn snapshots_are_cached_between_polls() {
    let (presence, fake) = service(vec![room(1, &["en"])], false);
    presence.snapshot().await;
    presence.snapshot().await;
    assert_eq!(fake.calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn failures_and_missing_media_report_nobody_online() {
    let (presence, _) = service(vec![room(1, &["en"])], true);
    assert_eq!(presence.snapshot().await.online, 0);
    assert_eq!(PresenceService::new(None).snapshot().await.online, 0);
}
