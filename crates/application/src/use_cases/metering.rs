//! Plans and fair use of the free capacity: call minutes are metered while someone is in
//! a call, charged to the room's owner, and calls end when a plan limit is reached, so no
//! single account can use everyone's minutes.

use std::collections::HashSet;
use std::sync::{Arc, Mutex, PoisonError};

use diceri_domain::{Account, Room, RoomId, Username};

use super::plan_limits::{Allowance, Capacity, LimitError, PlanCatalogue};
use crate::ports::{
    AccountRepository, Clock, MediaAccessError, MediaPresence, MediaRoomCloser, RoomRepository,
    UsageRepository,
};

const DAY_MS: u64 = 86_400_000;

/// Meters call minutes and enforces plan and capacity limits.
#[derive(Clone)]
pub struct MeteringService {
    rooms: Arc<dyn RoomRepository>,
    accounts: Arc<dyn AccountRepository>,
    usage: Arc<dyn UsageRepository>,
    clock: Arc<dyn Clock>,
    plans: PlanCatalogue,
    capacity: Capacity,
    media: Option<(Arc<dyn MediaPresence>, Arc<dyn MediaRoomCloser>)>,
    calls: Arc<Mutex<LiveCalls>>,
}

/// Rooms someone was in at the last tick, and owners admitted since (not counted yet).
#[derive(Debug, Default)]
struct LiveCalls {
    in_call: HashSet<RoomId>,
    admitted: HashSet<Username>,
}

impl std::fmt::Debug for MeteringService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MeteringService")
            .field("plans", &self.plans)
            .field("capacity", &self.capacity)
            .finish_non_exhaustive()
    }
}

impl MeteringService {
    #[must_use]
    pub fn new(
        rooms: Arc<dyn RoomRepository>,
        accounts: Arc<dyn AccountRepository>,
        usage: Arc<dyn UsageRepository>,
        clock: Arc<dyn Clock>,
        plans: PlanCatalogue,
        capacity: Capacity,
    ) -> Self {
        Self {
            rooms,
            accounts,
            usage,
            clock,
            plans,
            capacity,
            media: None,
            calls: Arc::default(),
        }
    }

    /// Meters with the media server: who is in each call, and closing calls at a limit.
    /// Without it nothing is metered (nobody can tell who is in a call).
    #[must_use]
    pub fn with_media(
        mut self,
        presence: Arc<dyn MediaPresence>,
        closer: Arc<dyn MediaRoomCloser>,
    ) -> Self {
        self.media = Some((presence, closer));
        self
    }

    /// Applies the configured plan limits and shared capacity.
    #[must_use]
    pub fn with_limits(mut self, plans: PlanCatalogue, capacity: Capacity) -> Self {
        self.plans = plans;
        self.capacity = capacity;
        self
    }

    /// Uses other account and usage stores (the durable ones), keeping the rooms.
    #[must_use]
    pub fn with_stores(
        mut self,
        accounts: Arc<dyn AccountRepository>,
        usage: Arc<dyn UsageRepository>,
    ) -> Self {
        self.accounts = accounts;
        self.usage = usage;
        self
    }

    /// What each plan allows, for showing the plans.
    #[must_use]
    pub fn plans(&self) -> PlanCatalogue {
        self.plans
    }

    /// The account's plan, limits and minutes used today and this month.
    pub async fn allowance(&self, account: &Account) -> Result<Allowance, LimitError> {
        let today = self.today();
        let usage = &self.usage;
        Ok(Allowance {
            plan: account.plan,
            limits: self.plans.limits(account.plan),
            used_today: usage
                .account_minutes_between(&account.id, today, today)
                .await?,
            used_this_month: usage
                .account_minutes_between(&account.id, month_start(today), today)
                .await?,
        })
    }

    /// Checks the owner may start or rejoin their room's call now.
    pub async fn admit(&self, account: &Account) -> Result<Allowance, LimitError> {
        let allowance = self.allowance(account).await?;
        if let Some(reached) = allowance.reached() {
            return Err(reached);
        }
        let today = self.today();
        let month = self.usage.total_between(month_start(today), today).await?;
        if month >= self.capacity.monthly_minutes {
            return Err(LimitError::MonthlyCapacity);
        }
        let own_call = self.rooms.find_live_by_owner(&account.username).await?;
        let mut calls = self.calls.lock().unwrap_or_else(PoisonError::into_inner);
        let already_live = own_call.is_some_and(|room| calls.in_call.contains(&room.id))
            || calls.admitted.contains(&account.username);
        if !already_live
            && calls.in_call.len() + calls.admitted.len() >= self.capacity.max_live_calls
        {
            return Err(LimitError::Busy);
        }
        calls.admitted.insert(account.username.clone());
        Ok(allowance)
    }

    /// Checks the owner of `room` may start or rejoin its call (the host joining).
    pub async fn admit_owner_of(&self, room: &Room) -> Result<(), LimitError> {
        if let Some(owner) = self.accounts.find_by_username(&room.owner).await? {
            self.admit(&owner.account).await?;
        }
        Ok(())
    }

    /// Minutes before a limit ends `room`'s call; `None` when its owner is unknown.
    pub async fn minutes_left(&self, room: &Room) -> Result<Option<u32>, LimitError> {
        let Some(stored) = self.accounts.find_by_username(&room.owner).await? else {
            return Ok(None);
        };
        let allowance = self.allowance(&stored.account).await?;
        let call_left = allowance
            .limits
            .call_minutes
            .saturating_sub(room.call_minutes);
        Ok(Some(call_left.min(allowance.minutes_left())))
    }

    /// Runs once a minute: charges a minute to every call someone is in, and ends the
    /// calls that reached their plan's limit.
    pub async fn tick(&self) {
        let Some((presence, closer)) = &self.media else {
            return;
        };
        let in_call = self
            .charge_live_calls(presence.as_ref(), closer.as_ref())
            .await;
        let mut calls = self.calls.lock().unwrap_or_else(PoisonError::into_inner);
        calls.in_call = in_call;
        calls.admitted.clear();
    }

    /// Charges every call someone is in; returns those rooms.
    async fn charge_live_calls(
        &self,
        presence: &dyn MediaPresence,
        closer: &dyn MediaRoomCloser,
    ) -> HashSet<RoomId> {
        let rooms = self
            .rooms
            .live_rooms()
            .await
            .inspect_err(|error| tracing::warn!(%error, "usage meter could not list rooms"))
            .unwrap_or_default();
        let mut in_call = HashSet::new();
        for room in rooms {
            if !someone_in(presence, &room.id).await {
                continue;
            }
            in_call.insert(room.id);
            if let Err(error) = self.charge(room.id, closer).await {
                tracing::warn!(%error, room_id = %room.id, "call minute not charged");
            }
        }
        in_call
    }

    /// One minute of `room`'s call, charged to its owner; ends it at a plan limit.
    async fn charge(&self, room: RoomId, closer: &dyn MediaRoomCloser) -> Result<(), LimitError> {
        let Some(stored) = self.rooms.find_by_id(&room).await? else {
            return Ok(());
        };
        let owner = self.accounts.find_by_username(&stored.owner).await?;
        let mut limit = None;
        if let Some(owner) = &owner {
            self.usage
                .add_minute(&owner.account.id, self.today())
                .await?;
            let allowance = self.allowance(&owner.account).await?;
            limit = Some((allowance.limits.call_minutes, allowance.reached().is_some()));
        }
        // Applied to the room as it is now: the account lookups above take time, and a copy
        // read before them would undo a guest let in or a voice recorded meanwhile.
        let charge = move |room: &mut Room| {
            room.call_minutes += 1;
            if let Some((call_minutes, plan_reached)) = limit
                && (room.call_minutes >= call_minutes || plan_reached)
            {
                room.end();
            }
            Ok(())
        };
        let charged = self.rooms.modify(&room, Box::new(charge)).await?;
        if charged.is_ended() && !stored.is_ended() {
            close_at_limit(closer, &room).await;
        }
        Ok(())
    }

    fn today(&self) -> u32 {
        utc_day(self.clock.now_ms())
    }
}

/// Whether anyone is in `room`'s call (a provider error counts as nobody).
async fn someone_in(presence: &dyn MediaPresence, room: &RoomId) -> bool {
    match presence.people_in(room).await {
        Ok(people) => people > 0,
        Err(error) => {
            tracing::warn!(%error, room_id = %room, "usage meter could not see the call");
            false
        }
    }
}

/// Disconnects everyone from a call that reached its plan's limit.
async fn close_at_limit(closer: &dyn MediaRoomCloser, room: &RoomId) {
    tracing::info!(room_id = %room, "call ended: plan limit reached");
    if let Err(error) = closer.close(room).await {
        warn_not_closed(&error, room);
    }
}

fn warn_not_closed(error: &MediaAccessError, room: &RoomId) {
    tracing::warn!(%error, room_id = %room, "media room not closed");
}

/// Days since the Unix epoch (UTC) of an instant in milliseconds.
fn utc_day(ms: u64) -> u32 {
    u32::try_from(ms / DAY_MS).unwrap_or(u32::MAX)
}

/// The first day of `day`'s calendar month (UTC), as days since the Unix epoch. Uses the
/// civil-from-days algorithm (H. Hinnant) to find the day of the month.
fn month_start(day: u32) -> u32 {
    let z = i64::from(day) + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day_of_month = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    u32::try_from(i64::from(day) - (day_of_month - 1)).unwrap_or(0)
}

#[cfg(test)]
#[path = "metering_tests.rs"]
mod tests;
