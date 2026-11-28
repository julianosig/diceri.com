//! In-memory fakes for unit tests of use cases.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_domain::{
    AccessKey, Account, AccountId, Mascot, Password, Room, RoomCode, RoomId, RoomSettings, Username,
};

use crate::ports::{
    AccountRepository, AccountRepositoryError, Clock, DispatchError, IdGenerator, IdParts,
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole, NewSession,
    PasswordHash, PasswordHasher, PasswordHasherError, RandomSource, RoomChange, RoomChangeError,
    RoomRepository, RoomRepositoryError, StoredAccount, TranslationDispatcher,
};
use crate::use_cases::RoomService;

/// A private, host-approved room.
pub(crate) fn private_room() -> RoomSettings {
    RoomSettings::default()
}

/// The account that owns test rooms.
pub(crate) fn owner() -> Account {
    Account {
        id: AccountId::from_parts(1, [7; 10]),
        username: "owner".parse().unwrap(),
        mascot: Mascot::Robot,
        plan: diceri_domain::Plan::Free,
    }
}

/// Clock starting at 1 000 ms; tests move it forward.
#[derive(Default)]
pub(crate) struct FakeClock(pub(crate) AtomicU64);

impl Clock for FakeClock {
    fn now_ms(&self) -> u64 {
        self.0.load(Ordering::Relaxed) + 1_000
    }
}

/// "Hashes" by prefixing (fast and inspectable); real adapters use Argon2id.
pub(crate) struct PlainHasher;

#[async_trait]
impl PasswordHasher for PlainHasher {
    async fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError> {
        Ok(PasswordHash(format!("plain:{}", password.expose())))
    }

    async fn verify(
        &self,
        password: &Password,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordHasherError> {
        Ok(hash.0 == format!("plain:{}", password.expose()))
    }
}

/// A room service over `repository` with deterministic randomness and ids.
pub(crate) fn room_service(repository: Arc<dyn RoomRepository>) -> RoomService {
    RoomService::new(
        repository,
        Arc::new(CountingRandom::default()),
        Arc::new(SequentialIds::default()),
    )
}

/// Ids one millisecond apart, so they are distinct and ordered.
#[derive(Default)]
pub(crate) struct SequentialIds(AtomicU64);

impl IdGenerator for SequentialIds {
    fn next_parts(&self) -> IdParts {
        IdParts {
            unix_ms: self.0.fetch_add(1, Ordering::Relaxed) + 1,
            random: [0; 10],
        }
    }
}

/// Accounts and sessions keyed by raw values (fine for tests; real adapters hash).
#[derive(Default)]
pub(crate) struct InMemoryAccounts {
    accounts: Mutex<Vec<StoredAccount>>,
    /// `(token, account id, expires at)`.
    sessions: Mutex<Vec<(AccessKey, AccountId, u64)>>,
}

#[async_trait]
impl AccountRepository for InMemoryAccounts {
    async fn set_plan(
        &self,
        username: &Username,
        plan: diceri_domain::Plan,
    ) -> Result<bool, AccountRepositoryError> {
        let mut accounts = self.accounts.lock().unwrap();
        let found = accounts
            .iter_mut()
            .find(|stored| &stored.account.username == username);
        Ok(found.map(|stored| stored.account.plan = plan).is_some())
    }

    async fn create_from(
        &self,
        account: &StoredAccount,
        _network: Option<crate::ports::SignupNetwork>,
    ) -> Result<(), AccountRepositoryError> {
        let mut accounts = self.accounts.lock().unwrap();
        if accounts
            .iter()
            .any(|stored| stored.account.username == account.account.username)
        {
            return Err(AccountRepositoryError::Taken);
        }
        accounts.push(account.clone());
        Ok(())
    }

    async fn count(&self) -> Result<u64, AccountRepositoryError> {
        Ok(u64::try_from(self.accounts.lock().unwrap().len()).unwrap())
    }

    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<StoredAccount>, AccountRepositoryError> {
        let accounts = self.accounts.lock().unwrap();
        Ok(accounts
            .iter()
            .find(|stored| &stored.account.username == username)
            .cloned())
    }

    async fn create_session(&self, session: NewSession<'_>) -> Result<(), AccountRepositoryError> {
        self.sessions.lock().unwrap().push((
            session.token.clone(),
            *session.account,
            session.expires_at_ms,
        ));
        Ok(())
    }

    async fn find_session(
        &self,
        token: &AccessKey,
        now_ms: u64,
    ) -> Result<Option<Account>, AccountRepositoryError> {
        let account_id = self
            .sessions
            .lock()
            .unwrap()
            .iter()
            .find(|(stored, _, expires)| stored == token && *expires > now_ms)
            .map(|(_, id, _)| *id);
        let accounts = self.accounts.lock().unwrap();
        Ok(account_id.and_then(|id| {
            accounts
                .iter()
                .find(|stored| stored.account.id == id)
                .map(|stored| stored.account.clone())
        }))
    }

    async fn delete_session(&self, token: &AccessKey) -> Result<(), AccountRepositoryError> {
        self.sessions
            .lock()
            .unwrap()
            .retain(|(stored, _, _)| stored != token);
        Ok(())
    }
}

/// Deterministic randomness: every call returns the next byte value.
#[derive(Default)]
pub(crate) struct CountingRandom(AtomicU8);

impl RandomSource for CountingRandom {
    fn random_128(&self) -> [u8; 16] {
        [self.0.fetch_add(1, Ordering::Relaxed); 16]
    }
}

#[derive(Default)]
pub(crate) struct InMemoryRooms(Mutex<HashMap<RoomId, Room>>);

#[async_trait]
impl RoomRepository for InMemoryRooms {
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError> {
        self.0.lock().unwrap().insert(room.id, room);
        Ok(())
    }

    async fn find_by_code(&self, code: &RoomCode) -> Result<Option<Room>, RoomRepositoryError> {
        let rooms = self.0.lock().unwrap();
        Ok(rooms.values().find(|room| &room.code == code).cloned())
    }

    async fn find_by_id(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
        Ok(self.0.lock().unwrap().get(id).cloned())
    }

    async fn find_live_by_owner(
        &self,
        owner: &Username,
    ) -> Result<Option<Room>, RoomRepositoryError> {
        let rooms = self.0.lock().unwrap();
        Ok(rooms
            .values()
            .find(|room| &room.owner == owner && !room.is_ended())
            .cloned())
    }

    async fn live_rooms(&self) -> Result<Vec<Room>, RoomRepositoryError> {
        let rooms = self.0.lock().unwrap();
        Ok(rooms
            .values()
            .filter(|room| !room.is_ended())
            .cloned()
            .collect())
    }

    async fn modify(&self, id: &RoomId, change: RoomChange<'_>) -> Result<Room, RoomChangeError> {
        let mut rooms = self.0.lock().unwrap();
        let slot = rooms.get_mut(id).ok_or(RoomRepositoryError::Missing(*id))?;
        let mut changed = slot.clone();
        change(&mut changed)?;
        *slot = changed.clone();
        Ok(changed)
    }
}

/// Records every issued grant as `(identity, name, role)`.
#[derive(Default)]
pub(crate) struct RecordingIssuer(pub(crate) Mutex<Vec<(String, Option<String>, MediaRole)>>);

impl MediaAccessIssuer for RecordingIssuer {
    fn issue(
        &self,
        _: &RoomId,
        participant: MediaParticipant<'_>,
    ) -> Result<MediaGrant, MediaAccessError> {
        self.0.lock().unwrap().push((
            participant.identity.to_owned(),
            participant
                .profile
                .map(|profile| profile.name.as_str().to_owned()),
            participant.role,
        ));
        Ok(MediaGrant {
            server_url: "wss://media.test".to_owned(),
            token: "jwt".to_owned(),
        })
    }
}

/// Records rooms the media agent was asked to translate.
#[derive(Default)]
pub(crate) struct RecordingDispatcher(pub(crate) Mutex<Vec<RoomId>>);

#[async_trait]
impl TranslationDispatcher for RecordingDispatcher {
    async fn start(&self, room: &RoomId) -> Result<(), DispatchError> {
        self.0.lock().unwrap().push(*room);
        Ok(())
    }
}
