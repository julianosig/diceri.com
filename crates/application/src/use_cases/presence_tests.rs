use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_domain::{EntryPolicy, Room, RoomSettings};

use super::{PresenceService, RANKING_SIZE};
use crate::ports::{LiveRoom, MediaAccessError, MediaPresence, RoomRepository as _};
use crate::test_support::{CountingRandom, InMemoryRooms};
use crate::use_cases::RoomService;

#[derive(Default)]
struct FakePresence {
    rooms: Mutex<Vec<LiveRoom>>,
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
        Ok(self.rooms.lock().unwrap().clone())
    }
}

struct Fixture {
    presence: PresenceService,
    media: Arc<FakePresence>,
    rooms: RoomService,
    repository: Arc<InMemoryRooms>,
}

fn fixture(fail: bool) -> Fixture {
    let repository = Arc::new(InMemoryRooms::default());
    let media = Arc::new(FakePresence {
        fail,
        ..FakePresence::default()
    });
    Fixture {
        presence: PresenceService::new(Some(media.clone()), repository.clone()),
        rooms: RoomService::new(repository.clone(), Arc::new(CountingRandom::default())),
        media,
        repository,
    }
}

impl Fixture {
    /// Creates a room and puts `people` listening in `languages` in it.
    async fn live(&self, title: &str, listed: bool, people: usize, languages: &[&str]) -> Room {
        let settings = RoomSettings {
            listed,
            entry: EntryPolicy::Open,
            ..RoomSettings::private(title.parse().unwrap())
        };
        let room = self.rooms.create(settings).await.unwrap();
        self.media.rooms.lock().unwrap().push(LiveRoom {
            room: room.id.clone(),
            people,
            languages: languages.iter().map(|code| code.parse().unwrap()).collect(),
        });
        room
    }
}

#[tokio::test]
async fn ranks_listed_rooms_by_size_and_counts_everyone() {
    let f = fixture(false);
    f.live("Small", true, 2, &["en", "pt-BR"]).await;
    f.live("Private", false, 4, &["de"]).await;
    let big = f.live("Big", true, 3, &["es", "en", "en"]).await;

    let snapshot = f.presence.snapshot().await;
    assert_eq!(snapshot.online, 9);
    assert_eq!(snapshot.live_rooms, 3);
    let titles: Vec<&str> = snapshot.ranking.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(titles, ["Big", "Small"]);
    let first = &snapshot.ranking[0];
    assert_eq!(first.id, big.id);
    assert_eq!(first.entry, EntryPolicy::Open);
    let languages: Vec<&str> = first
        .languages
        .iter()
        .map(diceri_domain::LanguageCode::as_str)
        .collect();
    assert_eq!(languages, ["en", "es"]);
}

#[tokio::test]
async fn shows_the_host_name_once_known() {
    let f = fixture(false);
    let mut room = f.live("Chat", true, 1, &["en"]).await;
    room.set_host_name("Ana".parse().unwrap());
    f.repository.update(room).await.unwrap();
    let snapshot = f.presence.snapshot().await;
    assert_eq!(
        snapshot.ranking[0]
            .host
            .as_ref()
            .map(diceri_domain::DisplayName::as_str),
        Some("Ana")
    );
}

#[tokio::test]
async fn ignores_unknown_ended_and_empty_rooms() {
    let f = fixture(false);
    let mut ended = f.live("Ended", true, 2, &["en"]).await;
    ended.end();
    f.repository.update(ended).await.unwrap();
    f.live("Empty", true, 0, &[]).await;
    f.media.rooms.lock().unwrap().push(LiveRoom {
        room: diceri_domain::RoomId::from_random_bytes([99; 16]),
        people: 5,
        languages: Vec::new(),
    });
    assert_eq!(
        f.presence.snapshot().await,
        super::PresenceSnapshot::default()
    );
}

#[tokio::test]
async fn ranking_is_capped_but_totals_are_not() {
    let f = fixture(false);
    for _ in 0..RANKING_SIZE + 5 {
        f.live("Room", true, 1, &["en"]).await;
    }
    let snapshot = f.presence.snapshot().await;
    assert_eq!(snapshot.ranking.len(), RANKING_SIZE);
    assert_eq!(snapshot.online, RANKING_SIZE + 5);
}

#[tokio::test]
async fn snapshots_are_cached_between_polls() {
    let f = fixture(false);
    f.live("Room", true, 1, &["en"]).await;
    f.presence.snapshot().await;
    f.presence.snapshot().await;
    assert_eq!(f.media.calls.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn failures_and_missing_media_report_nobody_online() {
    let f = fixture(true);
    assert_eq!(f.presence.snapshot().await.online, 0);
    let without_media = PresenceService::new(None, f.repository.clone());
    assert_eq!(without_media.snapshot().await.online, 0);
}
