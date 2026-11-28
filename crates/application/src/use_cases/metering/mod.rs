//! Plans and fair use of the free capacity: call minutes are metered while someone is in
//! a call, charged to the room's owner, and calls end when a plan limit is reached, so no
//! single account can use everyone's minutes.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, PoisonError};

use diceri_domain::{Account, Room, RoomId};

use super::plan_limits::{Allowance, Capacity, LimitError, PlanCatalogue};
use crate::ports::{
    AccountRepository, CallMembers, Clock, MediaPresence, MediaRoomCloser, RoomRepository,
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

/// What one tick saw: calls someone is in, calls it ended, and who is in each call it looked at.
#[derive(Default)]
struct Checked {
    in_call: HashSet<RoomId>,
    ended_now: HashSet<RoomId>,
    members: HashMap<RoomId, CallMembers>,
    /// Calls the media server could not say anything about this time.
    unseen: HashSet<RoomId>,
    /// Calls it did answer about (empty or not).
    seen: HashSet<RoomId>,
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
        let now_ms = self.clock.now_ms();
        let mut calls = self.calls.lock().unwrap_or_else(PoisonError::into_inner);
        calls.release_expired(now_ms);
        let owner = &account.username;
        let already_live = calls.live_owners.contains(owner) || calls.admitted.contains_key(owner);
        if !already_live && calls.live_count() >= self.capacity.max_live_calls {
            return Err(LimitError::Busy);
        }
        calls.admitted.insert(owner.clone(), now_ms);
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
        // What the looks below show is true from now on (the time is read before asking).
        let looked_at = self.clock.now_ms();
        // One listing and one look at each call serve both charging and tidying up.
        let open = open_media_rooms(presence.as_ref()).await;
        // Charging first: tidying up never delays a call's charge or its limit.
        let checked = self
            .charge_live_calls(presence, closer, open.as_deref())
            .await;
        // Calls ended just now are already being closed; within a budget, so a slow media
        // server never runs into the next minute.
        within_budget(
            self.tidy_media_rooms(presence, closer, open, &checked),
            "tidying media rooms",
        )
        .await;
        // A call that could not be seen keeps counting as it did: capacity never drops on a
        // failed look.
        let mut in_call = checked.in_call;
        let previous = self
            .calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .in_call
            .clone();
        in_call.extend(checked.unseen.intersection(&previous).copied());
        let repository = self.rooms.as_ref();
        let live_owners = owners_of(repository, &in_call).await;
        let mut seen_owners = owners_of(repository, &checked.seen).await;
        // An owner with no room open any more (a call ended) is known to have no call.
        let reserved = self
            .calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .reserved();
        seen_owners.extend(without_live_room(repository, reserved).await);
        self.calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .update(in_call, live_owners, &seen_owners, looked_at);
    }

    /// Charges every call someone is in. Each call is looked at once, all at once with a
    /// deadline each, and charged as soon as its answer comes; a call this ends is closed on
    /// its own task, so neither a slow media server nor a slow close holds up another call.
    async fn charge_live_calls(
        &self,
        presence: &Arc<dyn MediaPresence>,
        closer: &Arc<dyn MediaRoomCloser>,
        open: Option<&[RoomId]>,
    ) -> Checked {
        let rooms = self
            .rooms
            .live_rooms()
            .await
            .inspect_err(|error| tracing::warn!(%error, "usage meter could not list rooms"))
            .unwrap_or_default();
        // Only rooms with a media room open can have anyone in them.
        let mut checked = Checked::default();
        let mut looks = tokio::task::JoinSet::new();
        for room in rooms.iter().map(|room| room.id) {
            if open.is_some_and(|open| !open.contains(&room)) {
                // No media room open: known to be empty.
                checked.seen.insert(room);
                continue;
            }
            let presence = presence.clone();
            looks.spawn(async move { (room, members_of(presence.as_ref(), &room).await) });
        }
        // Every answer is read: one call the media server cannot see never stops the others.
        while let Some(answer) = looks.join_next().await {
            let Ok((room, members)) = answer else {
                continue;
            };
            let Some(members) = members else {
                checked.unseen.insert(room);
                continue;
            };
            let someone = !members.people.is_empty();
            checked.seen.insert(room);
            checked.members.insert(room, members);
            if someone {
                checked.in_call.insert(room);
                if self.charge_or_close(room, closer).await {
                    checked.ended_now.insert(room);
                }
            }
        }
        checked
    }

    /// `true` when this charge ended the call (it is then closed on its own task).
    async fn charge_or_close(&self, room: RoomId, closer: &Arc<dyn MediaRoomCloser>) -> bool {
        match self.charge(room).await {
            Ok(true) => {
                tokio::spawn(close_within_deadline(closer.clone(), room));
                true
            }
            Ok(false) => false,
            Err(error) => {
                warn_not_charged(&error, &room);
                false
            }
        }
    }

    /// Closes media rooms still open for calls that ended (a close that failed, or someone
    /// connecting with credentials issued just before the end), and disconnects hidden
    /// listeners whose person left the call, from what charging already looked at. Only rooms
    /// this service knows are touched, never another deployment's; each with its own deadline.
    async fn tidy_media_rooms(
        &self,
        presence: &Arc<dyn MediaPresence>,
        closer: &Arc<dyn MediaRoomCloser>,
        open: Option<Vec<RoomId>>,
        checked: &Checked,
    ) {
        let mut work = tokio::task::JoinSet::new();
        for room in open.unwrap_or_default() {
            if checked.ended_now.contains(&room) {
                continue;
            }
            let Ok(Some(stored)) = self.rooms.find_by_id(&room).await else {
                continue;
            };
            let closer = closer.clone();
            if stored.is_ended() {
                work.spawn(within_deadline(
                    async move { close_ended(closer.as_ref(), &room).await },
                    room,
                ));
            } else if checked.members.get(&room).is_some_and(has_orphan_listeners) {
                // Looked at again just before: someone may have rejoined with a new listener.
                let presence = presence.clone();
                work.spawn(within_deadline(
                    async move {
                        if let Some(fresh) = members_of(presence.as_ref(), &room).await {
                            remove_orphan_listeners(closer.as_ref(), &room, fresh).await;
                        }
                    },
                    room,
                ));
            }
        }
        while work.join_next().await.is_some() {}
    }

    /// One minute of `room`'s call, charged to its owner; `true` when this ended it.
    async fn charge(&self, room: RoomId) -> Result<bool, LimitError> {
        let Some(stored) = self.rooms.find_by_id(&room).await? else {
            return Ok(false);
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
        // Only a call this charge ends is closed here; one the host ended meanwhile is theirs.
        let mut ended_here = false;
        let charge = |room: &mut Room| {
            room.call_minutes += 1;
            if let Some((call_minutes, plan_reached)) = limit
                && !room.is_ended()
                && (room.call_minutes >= call_minutes || plan_reached)
            {
                room.end();
                ended_here = true;
            }
            Ok(())
        };
        self.rooms.modify(&room, Box::new(charge)).await?;
        Ok(ended_here)
    }

    fn today(&self) -> u32 {
        utc_day(self.clock.now_ms())
    }
}

fn warn_not_charged(error: &LimitError, room: &RoomId) {
    tracing::warn!(%error, room_id = %room, "call minute not charged");
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

mod capacity;
mod media;

use capacity::{LiveCalls, owners_of, without_live_room};

use media::{
    close_ended, close_within_deadline, has_orphan_listeners, members_of, open_media_rooms,
    remove_orphan_listeners, within_budget, within_deadline,
};

#[cfg(test)]
mod tests;
