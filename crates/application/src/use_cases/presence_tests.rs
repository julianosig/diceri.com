use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_domain::{Account, AccountId, EntryPolicy, Mascot, Room, RoomSettings};

use super::{PresenceService, RANKING_SIZE};
use crate::ports::{LiveRoom, MediaAccessError, MediaPresence, RoomRepository as _};
use crate::test_support::{InMemoryRooms, room_service};
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
        rooms: room_service(repository.clone()),
        media,
        repository,
    }
}

impl Fixture {
    /// Opens `owner`'s room and puts `people` listening in `languages` in it.
    async fn live(&self, owner: &str, listed: bool, people: usize, languages: &[&str]) -> Room {
        let settings = RoomSettings {
            listed,
            entry: EntryPolicy::Open,
            ..RoomSettings::default()
        };
        let account = Account {
            id: AccountId::from_parts(1, [owner.len().to_le_bytes()[0]; 10]),
            username: owner.parse().unwrap(),
            mascot: Mascot::Ghost,
        };
        let room = self.rooms.open(&account, settings).await.unwrap();
        self.media.rooms.lock().unwrap().push(LiveRoom {
            room: room.id,
            people,
            languages: languages.iter().map(|code| code.parse().unwrap()).collect(),
        });
        room
    }
}

#[tokio::test]
async fn ranks_listed_rooms_by_size_and_counts_everyone() {
    let f = fixture(false);
    f.live("small", true, 2, &["en", "pt-BR"]).await;
    f.live("private", false, 4, &["de"]).await;
    let big = f.live("big", true, 3, &["es", "en", "en"]).await;

    let snapshot = f.presence.snapshot().await;
    assert_eq!(snapshot.online, 9);
    assert_eq!(snapshot.live_rooms, 3);
    let owners: Vec<&str> = snapshot.ranking.iter().map(|r| r.owner.as_str()).collect();
    assert_eq!(owners, ["big", "small"]);
    let first = &snapshot.ranking[0];
    assert_eq!(first.code, big.code);
    assert_eq!(first.entry, EntryPolicy::Open);
    assert_eq!(first.mascot, Mascot::Ghost);
    let languages: Vec<&str> = first
        .languages
        .iter()
        .map(diceri_domain::LanguageCode::as_str)
        .collect();
    assert_eq!(languages, ["en", "es"]);
}

#[tokio::test]
async fn ignores_unknown_ended_and_empty_rooms() {
    let f = fixture(false);
    let mut ended = f.live("Ended", true, 2, &["en"]).await;
    ended.end();
    f.repository.update(ended).await.unwrap();
    f.live("Empty", true, 0, &[]).await;
    f.media.rooms.lock().unwrap().push(LiveRoom {
        room: diceri_domain::RoomId::from_parts(99, [99; 10]),
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
    for n in 0..RANKING_SIZE + 5 {
        f.live(&format!("room{n}"), true, 1, &["en"]).await;
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

#[tokio::test]
async fn tells_how_many_people_are_in_a_room_call() {
    let f = fixture(false);
    let busy = f.live("busy", false, 3, &["en"]).await;
    let quiet = f.live("quiet", false, 0, &[]).await;
    assert_eq!(f.presence.people_in(&busy.id).await, Some(3));
    assert_eq!(f.presence.people_in(&quiet.id).await, Some(0));
    let without_media = PresenceService::new(None, f.repository.clone());
    assert_eq!(without_media.people_in(&busy.id).await, None);
}
