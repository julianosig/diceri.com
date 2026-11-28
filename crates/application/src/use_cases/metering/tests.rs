use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_domain::{Account, AccountId, Mascot, Plan, PlanLimits, Room, RoomId};

use super::{Capacity, LimitError, MeteringService, PlanCatalogue, month_start, utc_day};
use crate::ports::{
    AccountRepository as _, LiveRoom, MediaAccessError, MediaPresence, MediaRoomCloser,
    PasswordHash, RoomRepository as _, StoredAccount, UsageRepository, UsageRepositoryError,
};
use crate::test_support::{FakeClock, InMemoryAccounts, InMemoryRooms, private_room, room_service};

/// 2026-11-27 12:00 UTC.
const NOW_MS: u64 = 1_795_780_800_000;

#[derive(Default)]
/// People in each call, and the hidden listeners (`monitor-<seat>`) in each.
struct People(
    Mutex<HashMap<RoomId, usize>>,
    Mutex<HashMap<RoomId, Vec<String>>>,
);

#[async_trait]
impl MediaPresence for People {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        let rooms = self.0.lock().unwrap();
        Ok(rooms
            .iter()
            .filter(|(_, people)| **people > 0)
            .map(|(room, people)| LiveRoom {
                room: *room,
                people: *people,
                languages: Vec::new(),
            })
            .collect())
    }

    async fn people_in(&self, room: &RoomId) -> Result<usize, MediaAccessError> {
        Ok(self.0.lock().unwrap().get(room).copied().unwrap_or(0))
    }

    async fn call_members(
        &self,
        room: &RoomId,
    ) -> Result<crate::ports::CallMembers, MediaAccessError> {
        let listeners = self.1.lock().unwrap().get(room).cloned();
        Ok(crate::ports::CallMembers {
            people: self.identities_in(room).await?,
            listeners: listeners.unwrap_or_default(),
        })
    }

    async fn open_rooms(&self) -> Result<Vec<RoomId>, MediaAccessError> {
        Ok(self
            .live_rooms()
            .await?
            .into_iter()
            .map(|live| live.room)
            .collect())
    }

    async fn identities_in(&self, room: &RoomId) -> Result<Vec<String>, MediaAccessError> {
        let people = self.people_in(room).await?;
        Ok((0..people).map(|n| format!("p-{n}")).collect())
    }
}

#[derive(Default)]
/// Rooms closed, and listeners removed.
struct Closer(Mutex<Vec<RoomId>>, Mutex<Vec<String>>);

#[async_trait]
impl MediaRoomCloser for Closer {
    async fn close(&self, room: &RoomId) -> Result<(), MediaAccessError> {
        self.0.lock().unwrap().push(*room);
        Ok(())
    }

    async fn remove(&self, _: &RoomId, identity: &str) -> Result<(), MediaAccessError> {
        self.1.lock().unwrap().push(identity.to_owned());
        Ok(())
    }
}

#[derive(Default)]
struct Usage(Mutex<HashMap<(AccountId, u32), u32>>);

#[async_trait]
impl UsageRepository for Usage {
    async fn add_minute(&self, account: &AccountId, day: u32) -> Result<u32, UsageRepositoryError> {
        let mut minutes = self.0.lock().unwrap();
        let total = minutes.entry((*account, day)).or_insert(0);
        *total += 1;
        Ok(*total)
    }

    async fn account_minutes_between(
        &self,
        account: &AccountId,
        first: u32,
        last: u32,
    ) -> Result<u32, UsageRepositoryError> {
        let minutes = self.0.lock().unwrap();
        Ok(minutes
            .iter()
            .filter(|((owner, day), _)| owner == account && (first..=last).contains(day))
            .map(|(_, total)| *total)
            .sum())
    }

    async fn total_between(&self, first: u32, last: u32) -> Result<u64, UsageRepositoryError> {
        let minutes = self.0.lock().unwrap();
        Ok(minutes
            .iter()
            .filter(|((_, day), _)| (first..=last).contains(day))
            .map(|(_, total)| u64::from(*total))
            .sum())
    }
}

struct Fixture {
    metering: MeteringService,
    rooms: Arc<InMemoryRooms>,
    accounts: Arc<InMemoryAccounts>,
    usage: Arc<Usage>,
    people: Arc<People>,
    closer: Arc<Closer>,
}

/// Free: 3-minute calls, 5 minutes a day, 8 a month. At most 1 live call, 100 minutes a month.
fn fixture() -> Fixture {
    let rooms = Arc::new(InMemoryRooms::default());
    let accounts = Arc::new(InMemoryAccounts::default());
    let usage = Arc::new(Usage::default());
    let people = Arc::new(People::default());
    let closer = Arc::new(Closer::default());
    let free = PlanLimits {
        call_minutes: 3,
        daily_minutes: 5,
        monthly_minutes: 8,
        monthly_price_cents: 0,
        participants: 2,
    };
    let plans = PlanCatalogue {
        free,
        ..PlanCatalogue::default()
    };
    let capacity = Capacity {
        max_live_calls: 1,
        monthly_minutes: 100,
    };
    let metering = MeteringService::new(
        rooms.clone(),
        accounts.clone(),
        usage.clone(),
        Arc::new(FakeClock(AtomicU64::new(NOW_MS))),
        plans,
        capacity,
    )
    .with_media(people.clone(), closer.clone());
    Fixture {
        metering,
        rooms,
        accounts,
        usage,
        people,
        closer,
    }
}

impl Fixture {
    async fn account(&self, seed: u8, username: &str) -> Account {
        let account = Account {
            id: AccountId::from_parts(u64::from(seed), [seed; 10]),
            username: username.parse().unwrap(),
            mascot: Mascot::Robot,
            plan: Plan::Free,
        };
        let stored = StoredAccount {
            account: account.clone(),
            password_hash: PasswordHash("hash".to_owned()),
        };
        self.accounts.create(&stored).await.unwrap();
        account
    }

    /// Opens `account`'s room with `people` in its call.
    async fn call(&self, account: &Account, people: usize) -> Room {
        let room = room_service(self.rooms.clone())
            .open(account, private_room(), 2)
            .await
            .unwrap();
        self.people.0.lock().unwrap().insert(room.id, people);
        room
    }

    async fn room(&self, room: &Room) -> Room {
        self.rooms.find_by_id(&room.id).await.unwrap().unwrap()
    }
}

/// Lets calls closed on their own task finish closing.
async fn settle() {
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
}

#[tokio::test]
async fn charges_the_owner_only_while_someone_is_in_the_call() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let room = f.call(&ana, 0).await;
    f.metering.tick().await;
    assert_eq!(f.metering.allowance(&ana).await.unwrap().used_today, 0);
    f.people.0.lock().unwrap().insert(room.id, 2);
    f.metering.tick().await;
    assert_eq!(f.metering.allowance(&ana).await.unwrap().used_today, 1);
    assert_eq!(f.room(&room).await.call_minutes, 1);
    assert_eq!(
        f.metering.minutes_left(&f.room(&room).await).await.unwrap(),
        Some(2)
    );
}

#[tokio::test]
async fn ends_a_call_at_the_plans_call_limit() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let room = f.call(&ana, 2).await;
    for _ in 0..3 {
        f.metering.tick().await;
    }
    assert!(f.room(&room).await.is_ended());
    settle().await;
    assert_eq!(*f.closer.0.lock().unwrap(), vec![room.id]);
}

#[tokio::test]
async fn ends_the_call_and_refuses_new_ones_at_the_daily_limit() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let today = utc_day(NOW_MS);
    for _ in 0..4 {
        f.usage.add_minute(&ana.id, today).await.unwrap();
    }
    let room = f.call(&ana, 2).await;
    f.metering.tick().await;
    assert!(f.room(&room).await.is_ended());
    assert!(matches!(
        f.metering.admit(&ana).await,
        Err(LimitError::DailyMinutes)
    ));
}

#[tokio::test]
async fn ends_the_call_and_refuses_new_ones_at_the_plans_monthly_minutes() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let today = utc_day(NOW_MS);
    // 7 minutes on earlier days this month; the month started at least a day before.
    for _ in 0..7 {
        f.usage
            .add_minute(&ana.id, month_start(today))
            .await
            .unwrap();
    }
    let room = f.call(&ana, 2).await;
    assert_eq!(f.metering.minutes_left(&room).await.unwrap(), Some(1));
    f.metering.tick().await;
    assert!(f.room(&room).await.is_ended());
    assert!(matches!(
        f.metering.admit(&ana).await,
        Err(LimitError::PlanMonthly)
    ));
}

#[tokio::test]
async fn refuses_new_calls_when_the_month_is_used_up() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let bob = f.account(2, "bob").await;
    let today = utc_day(NOW_MS);
    for _ in 0..100 {
        f.usage.add_minute(&bob.id, today).await.unwrap();
    }
    assert!(matches!(
        f.metering.admit(&ana).await,
        Err(LimitError::MonthlyCapacity)
    ));
}

#[tokio::test]
async fn refuses_a_new_call_when_too_many_are_live_but_not_the_running_one() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let bob = f.account(2, "bob").await;
    f.call(&ana, 2).await;
    f.metering.tick().await;
    assert!(matches!(
        f.metering.admit(&bob).await,
        Err(LimitError::Busy)
    ));
    assert!(
        f.metering.admit(&ana).await.is_ok(),
        "rejoining your own live call"
    );
}

#[test]
fn finds_the_first_day_of_the_month() {
    // 2026-11-01 is day 20758 since the Unix epoch; 2024-02-29 is a leap day.
    assert_eq!(month_start(20_758), 20_758);
    assert_eq!(month_start(20_758 + 26), 20_758);
    assert_eq!(month_start(20_757), 20_758 - 31);
    assert_eq!(month_start(19_782), 19_754);
    assert_eq!(utc_day(NOW_MS), 20_784);
}

#[tokio::test]
async fn an_ended_call_still_open_at_the_media_server_is_closed() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let room = f.call(&ana, 0).await;
    let end = |room: &mut Room| {
        room.end();
        Ok(())
    };
    f.rooms.modify(&room.id, Box::new(end)).await.unwrap();
    // Someone connected with credentials issued just before the end, reopening the media room.
    f.people.0.lock().unwrap().insert(room.id, 1);
    // A room of another deployment sharing the media server is never touched.
    let other = RoomId::from_parts(9, [9; 10]);
    f.people.0.lock().unwrap().insert(other, 2);
    f.metering.tick().await;
    settle().await;
    assert_eq!(*f.closer.0.lock().unwrap(), vec![room.id]);
}

#[tokio::test]
async fn a_listener_whose_person_left_is_disconnected() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let room = f.call(&ana, 1).await;
    // p-0 is in the call (see `identities_in`); p-9 left but kept listening.
    f.people.1.lock().unwrap().insert(
        room.id,
        vec!["monitor-p-0".to_owned(), "monitor-p-9".to_owned()],
    );
    f.metering.tick().await;
    assert_eq!(*f.closer.1.lock().unwrap(), vec!["monitor-p-9".to_owned()]);
}

#[tokio::test]
async fn an_owner_still_connecting_keeps_their_place_through_a_tick() {
    let f = fixture();
    let ana = f.account(1, "ana").await;
    let bob = f.account(2, "bob").await;
    // Ana is let in; the minute's tick runs before she connects.
    f.call(&ana, 0).await;
    f.metering.admit(&ana).await.unwrap();
    f.metering.tick().await;
    // The service holds one call: her place is still taken.
    assert!(matches!(
        f.metering.admit(&bob).await,
        Err(LimitError::Busy)
    ));
    // Coming back while her place is held is not a second call.
    assert!(f.metering.admit(&ana).await.is_ok());
}
