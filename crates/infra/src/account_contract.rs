//! Contract every `AccountRepository` adapter must honour, run against both the
//! in-memory and the SQLite implementation so they cannot drift apart.

use diceri_application::ports::{
    AccountRepository, AccountRepositoryError, NewSession, PasswordHash, StoredAccount,
};
use diceri_domain::{AccessKey, Account, AccountId, Mascot, Plan, SessionId, Username};

/// A syntactically valid Argon2id PHC string (adapters never verify it).
pub(crate) const HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$c2FsdHNhbHQ$aGFzaGhhc2hoYXNo";

pub(crate) fn stored(seed: u8, username: &str) -> StoredAccount {
    StoredAccount {
        account: Account {
            id: AccountId::from_parts(u64::from(seed) + 1, [seed; 10]),
            username: username.parse().unwrap(),
            mascot: Mascot::Pirate,
            plan: Plan::Free,
        },
        password_hash: PasswordHash(HASH.to_owned()),
    }
}

fn token(seed: u8) -> AccessKey {
    AccessKey::from_random_bytes([seed; 16])
}

pub(crate) fn session<'a>(account: &'a AccountId, token: &'a AccessKey) -> NewSession<'a> {
    NewSession {
        id: SessionId::from_parts(5, [token.expose().len().to_le_bytes()[0]; 10]),
        account,
        token,
        created_at_ms: 1_000,
        expires_at_ms: 2_000,
    }
}

fn name(raw: &str) -> Username {
    raw.parse().unwrap()
}

pub(crate) async fn check(accounts: &dyn AccountRepository) {
    let ana = stored(1, "ana");
    accounts.create(&ana).await.unwrap();
    check_accounts(accounts, &ana).await;
    check_sessions(accounts, &ana).await;
    check_plans(accounts).await;
}

async fn check_plans(accounts: &dyn AccountRepository) {
    // New accounts are free; a plan change sticks; unknown usernames report false.
    assert_eq!(found(accounts, "ana").await.plan, Plan::Free);
    assert!(accounts.set_plan(&name("ana"), Plan::Pro).await.unwrap());
    assert_eq!(found(accounts, "ana").await.plan, Plan::Pro);
    assert!(!accounts.set_plan(&name("nobody"), Plan::Pro).await.unwrap());
}

async fn found(accounts: &dyn AccountRepository, username: &str) -> Account {
    accounts
        .find_by_username(&name(username))
        .await
        .unwrap()
        .unwrap()
        .account
}

async fn check_accounts(accounts: &dyn AccountRepository, ana: &StoredAccount) {
    // Found by username, with mascot and hash; unknown names find nothing.
    let found = accounts
        .find_by_username(&name("ana"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found.account, ana.account);
    assert_eq!(found.password_hash, ana.password_hash);
    assert!(
        accounts
            .find_by_username(&name("nobody"))
            .await
            .unwrap()
            .is_none()
    );

    // Usernames are unique, whatever the id.
    assert!(matches!(
        accounts.create(&stored(2, "ana")).await,
        Err(AccountRepositoryError::Taken)
    ));
}

/// A session finds its account until it expires or is deleted.
async fn check_sessions(accounts: &dyn AccountRepository, ana: &StoredAccount) {
    let token_a = token(10);
    accounts
        .create_session(session(&ana.account.id, &token_a))
        .await
        .unwrap();
    let live = accounts.find_session(&token_a, 1_500).await.unwrap();
    assert_eq!(live, Some(ana.account.clone()));
    assert_eq!(accounts.find_session(&token_a, 2_000).await.unwrap(), None);
    assert_eq!(
        accounts.find_session(&token(11), 1_500).await.unwrap(),
        None
    );
    accounts.delete_session(&token_a).await.unwrap();
    assert_eq!(accounts.find_session(&token_a, 1_500).await.unwrap(), None);
    accounts.delete_session(&token(12)).await.unwrap();
}
