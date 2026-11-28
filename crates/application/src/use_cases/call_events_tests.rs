use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use diceri_domain::Room;

use super::*;
use crate::ports::{CallMembers, LiveRoom, MediaAccessError};
use crate::test_support::{InMemoryRooms, owner, private_room, room_service};

/// People connected to each call; `None` when the media server cannot say.
#[derive(Default)]
struct People(Mutex<HashMap<RoomId, Vec<String>>>);

#[async_trait]
impl MediaPresence for People {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn open_rooms(&self) -> Result<Vec<RoomId>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn identities_in(&self, room: &RoomId) -> Result<Vec<String>, MediaAccessError> {
        Ok(self
            .0
            .lock()
            .unwrap()
            .get(room)
            .cloned()
            .unwrap_or_default())
    }

    async fn call_members(&self, room: &RoomId) -> Result<CallMembers, MediaAccessError> {
        let (listeners, people) = self
            .identities_in(room)
            .await?
            .into_iter()
            .partition(|identity| identity.starts_with("monitor-"));
        Ok(CallMembers { people, listeners })
    }
}

/// What was closed and who was removed.
#[derive(Default)]
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

struct Fixture {
    events: CallEventsService,
    rooms: Arc<InMemoryRooms>,
    people: Arc<People>,
    closer: Arc<Closer>,
    room: Room,
}

async fn fixture() -> Fixture {
    let rooms = Arc::new(InMemoryRooms::default());
    let room = room_service(rooms.clone())
        .open(&owner(), private_room(), 2)
        .await
        .unwrap();
    let (people, closer) = (Arc::new(People::default()), Arc::new(Closer::default()));
    let events = CallEventsService::new(rooms.clone(), people.clone(), closer.clone());
    Fixture {
        events,
        rooms,
        people,
        closer,
        room,
    }
}

impl Fixture {
    fn connected(&self, identities: &[&str]) {
        let identities = identities.iter().map(|&id| id.to_owned()).collect();
        self.people
            .0
            .lock()
            .unwrap()
            .insert(self.room.id, identities);
    }

    async fn join(&self, identity: &str) {
        let event = MediaEvent::Joined {
            room: self.room.id,
            identity: identity.to_owned(),
        };
        self.events.handle(event).await;
    }

    fn removed(&self) -> Vec<String> {
        self.closer.1.lock().unwrap().clone()
    }

    /// Seats the owner in the call as `identity`.
    async fn seat_owner(&self, identity: &'static str) {
        let seat = move |room: &mut Room| {
            room.claim_seat(owner().id, identity.to_owned(), None, 0)
                .map(|_| ())
        };
        self.rooms
            .modify(&self.room.id, Box::new(seat))
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn someone_beyond_the_calls_size_is_removed_but_never_its_owner() {
    let f = fixture().await;
    f.seat_owner("p-owner").await;
    // A reconnect with a refreshed token brings a third person into a call of two.
    f.connected(&["p-owner", "p-guest", "p-late"]);
    f.join("p-late").await;
    assert_eq!(f.removed(), vec!["p-late".to_owned()]);
    // Whoever's report comes next, the same person goes, never the owner.
    f.join("p-owner").await;
    assert_eq!(f.removed(), vec!["p-late".to_owned(); 2]);
}

#[tokio::test]
async fn people_within_the_calls_size_stay() {
    let f = fixture().await;
    f.seat_owner("p-owner").await;
    f.connected(&["p-owner", "p-guest"]);
    f.join("p-guest").await;
    assert_eq!(f.removed(), Vec::<String>::new());
}

#[tokio::test]
async fn a_listener_without_its_person_in_the_call_is_removed() {
    let f = fixture().await;
    f.seat_owner("p-owner").await;
    f.connected(&["p-owner", "monitor-p-owner.01", "monitor-p-gone.01"]);
    f.join("monitor-p-gone.01").await;
    assert_eq!(f.removed(), vec!["monitor-p-gone.01".to_owned()]);
}

#[tokio::test]
async fn only_the_newest_listener_of_a_person_stays() {
    let f = fixture().await;
    f.seat_owner("p-owner").await;
    f.connected(&["p-owner", "monitor-p-owner.01", "monitor-p-owner.02"]);
    f.join("monitor-p-owner.02").await;
    assert_eq!(f.removed(), vec!["monitor-p-owner.01".to_owned()]);
}

#[tokio::test]
async fn joining_a_call_that_ended_closes_it_again() {
    let f = fixture().await;
    let end = |room: &mut Room| {
        room.end();
        Ok(())
    };
    f.rooms.modify(&f.room.id, Box::new(end)).await.unwrap();
    f.join("p-late").await;
    assert_eq!(*f.closer.0.lock().unwrap(), vec![f.room.id]);
}

#[tokio::test]
async fn a_person_leaving_takes_their_listener_along_unless_they_are_back() {
    let f = fixture().await;
    let left = || MediaEvent::Left {
        room: f.room.id,
        identity: "p-guest".to_owned(),
    };
    f.seat_owner("p-owner").await;
    // Back already (a reload): a late report leaves the new listener alone.
    f.connected(&["p-owner", "p-guest", "monitor-p-guest.02"]);
    f.events.handle(left()).await;
    assert_eq!(f.removed(), Vec::<String>::new());
    // Gone: only the listener that look showed is removed.
    f.connected(&["p-owner", "monitor-p-guest.02"]);
    f.events.handle(left()).await;
    assert_eq!(f.removed(), vec!["monitor-p-guest.02".to_owned()]);
}

#[tokio::test]
async fn departures_from_rooms_of_another_deployment_are_left_alone() {
    let f = fixture().await;
    let event = MediaEvent::Left {
        room: RoomId::from_parts(9, [9; 10]),
        identity: "p-x".to_owned(),
    };
    f.events.handle(event).await;
    assert_eq!(f.removed(), Vec::<String>::new());
}

#[tokio::test]
async fn rooms_of_another_deployment_are_left_alone() {
    let f = fixture().await;
    let event = MediaEvent::Joined {
        room: RoomId::from_parts(9, [9; 10]),
        identity: "p-x".to_owned(),
    };
    f.events.handle(event).await;
    assert_eq!(f.removed(), Vec::<String>::new());
    assert_eq!(*f.closer.0.lock().unwrap(), Vec::<RoomId>::new());
}
