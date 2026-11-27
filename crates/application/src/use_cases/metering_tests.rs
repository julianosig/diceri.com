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
struct People(Mutex<HashMap<RoomId, usize>>);

#[async_trait]
impl MediaPresence for People {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn people_in(&self, room: &RoomId) -> Result<usize, MediaAccessError> {
        Ok(self.0.lock().unwrap().get(room).copied().unwrap_or(0))
    }
}

#[derive(Default)]
struct Closer(Mutex<Vec<RoomId>>);

#[async_trait]
impl MediaRoomCloser for Closer {
    async fn close(&self, room: &RoomId) -> Result<(), MediaAccessError> {
        self.0.lock().unwrap().push(*room);
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
            .open(account, private_room())
            .await
            .unwrap();
        self.people.0.lock().unwrap().insert(room.id, people);
        room
    }

    async fn room(&self, room: &Room) -> Room {
        self.rooms.find_by_id(&room.id).await.unwrap().unwrap()
    }
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
