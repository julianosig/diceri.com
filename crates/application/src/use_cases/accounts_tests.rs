use std::sync::Arc;
use std::sync::atomic::Ordering;

use diceri_domain::{Mascot, Username};

use super::{AccountError, AccountService, Registration, SESSION_TTL_MS};
use crate::test_support::{
    CountingRandom, FakeClock, InMemoryAccounts, PlainHasher, SequentialIds,
};

fn service() -> (AccountService, Arc<FakeClock>) {
    let clock = Arc::new(FakeClock::default());
    let service = AccountService::new(
        Arc::new(InMemoryAccounts::default()),
        Arc::new(PlainHasher),
        Arc::new(CountingRandom::default()),
        Arc::new(SequentialIds::default()),
        clock.clone(),
    );
    (service, clock)
}

fn name(raw: &str) -> Username {
    raw.parse().unwrap()
}

fn registration(username: &str, password: &str) -> Registration {
    Registration {
        username: name(username),
        password: password.parse().unwrap(),
        mascot: Mascot::Dragon,
    }
}

#[tokio::test]
async fn registering_signs_in_and_keeps_the_mascot() {
    let (accounts, _) = service();
    let signed_in = accounts
        .register(registration("ana", "correct horse"))
        .await
        .unwrap();
    let account = accounts.authenticate(&signed_in.token).await.unwrap();
    assert_eq!(account.username, name("ana"));
    assert_eq!(account.mascot, Mascot::Dragon);
}

#[tokio::test]
async fn usernames_are_unique() {
    let (accounts, _) = service();
    accounts
        .register(registration("ana", "correct horse"))
        .await
        .unwrap();
    assert!(matches!(
        accounts.register(registration("ANA", "another pass")).await,
        Err(AccountError::Taken)
    ));
}

#[tokio::test]
async fn signing_in_needs_the_right_password() {
    let (accounts, _) = service();
    accounts
        .register(registration("bob", "correct horse"))
        .await
        .unwrap();
    let wrong = "wrong password".parse().unwrap();
    assert!(matches!(
        accounts.sign_in(&name("bob"), &wrong).await,
        Err(AccountError::Unauthorized)
    ));
    assert!(matches!(
        accounts.sign_in(&name("nobody"), &wrong).await,
        Err(AccountError::Unauthorized)
    ));
    let right = "correct horse".parse().unwrap();
    let signed_in = accounts.sign_in(&name("bob"), &right).await.unwrap();
    assert_eq!(signed_in.account.username, name("bob"));
}

#[tokio::test]
async fn sessions_expire_and_can_be_ended() {
    let (accounts, clock) = service();
    let first = accounts
        .register(registration("carla", "correct horse"))
        .await
        .unwrap();
    clock.0.fetch_add(SESSION_TTL_MS + 1, Ordering::Relaxed);
    assert!(matches!(
        accounts.authenticate(&first.token).await,
        Err(AccountError::Unauthorized)
    ));

    let password = "correct horse".parse().unwrap();
    let second = accounts.sign_in(&name("carla"), &password).await.unwrap();
    accounts.sign_out(&second.token).await.unwrap();
    assert!(matches!(
        accounts.authenticate(&second.token).await,
        Err(AccountError::Unauthorized)
    ));
}
