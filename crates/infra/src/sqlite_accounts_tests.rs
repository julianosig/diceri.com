use diceri_application::ports::{AccountRepository as _, PasswordHash};
use diceri_domain::AccessKey;
use sqlx::Row as _;

use super::SqliteAccountRepository;
use crate::account_contract::{self, session, stored};

async fn repository() -> SqliteAccountRepository {
    SqliteAccountRepository::connect("sqlite::memory:")
        .await
        .unwrap()
}

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("diceri-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[tokio::test]
async fn honours_the_account_repository_contract() {
    account_contract::check(&repository().await).await;
}

#[tokio::test]
async fn stores_v7_ids_millisecond_times_and_hashed_tokens() {
    let accounts = repository().await;
    let bob = stored(3, "bob");
    accounts.create(&bob).await.unwrap();
    let token = AccessKey::from_random_bytes([4; 16]);
    accounts
        .create_session(session(&bob.account.id, &token))
        .await
        .unwrap();

    let row = sqlx::query("SELECT id, created_at FROM accounts")
        .fetch_one(&accounts.pool)
        .await
        .unwrap();
    let id: uuid::Uuid = row.get("id");
    let created_at: i64 = row.get("created_at");
    assert_eq!(id, bob.account.id.as_uuid());
    assert_eq!(created_at, 4, "the time inside the v7 id");

    let stored_hash: Vec<u8> = sqlx::query("SELECT token_hash FROM sessions")
        .fetch_one(&accounts.pool)
        .await
        .unwrap()
        .get("token_hash");
    assert_eq!(stored_hash.len(), 32);
    assert_ne!(stored_hash, token.expose().as_bytes());
}

#[tokio::test]
async fn the_schema_rejects_what_the_domain_would_reject() {
    let accounts = repository().await;
    let mut plain = stored(5, "carla");
    plain.password_hash = PasswordHash("plaintext".to_owned());
    let error = accounts.create(&plain).await.unwrap_err();
    assert!(error.to_string().contains("CHECK"), "{error}");

    let insert = sqlx::query(
        "INSERT INTO accounts (id, username, password_hash, mascot, created_at) \
         VALUES (?, 'dave', ?, 'pikachu', 1)",
    )
    .bind(stored(6, "dave").account.id.as_uuid())
    .bind(account_contract::HASH)
    .execute(&accounts.pool)
    .await;
    assert!(
        insert
            .unwrap_err()
            .as_database_error()
            .is_some_and(sqlx::error::DatabaseError::is_check_violation)
    );
}

#[tokio::test]
async fn deleting_an_account_deletes_its_sessions() {
    let accounts = repository().await;
    let erin = stored(7, "erin");
    accounts.create(&erin).await.unwrap();
    let token = AccessKey::from_random_bytes([8; 16]);
    accounts
        .create_session(session(&erin.account.id, &token))
        .await
        .unwrap();
    sqlx::query("DELETE FROM accounts")
        .execute(&accounts.pool)
        .await
        .unwrap();
    let left: i64 = sqlx::query_scalar("SELECT count(*) FROM sessions")
        .fetch_one(&accounts.pool)
        .await
        .unwrap();
    assert_eq!(left, 0);
}

#[tokio::test]
async fn snapshots_are_complete_and_checked() {
    let dir = temp_dir("backup");
    let live = dir.join("live.db");
    let accounts = SqliteAccountRepository::connect(&format!("sqlite://{}", live.display()))
        .await
        .unwrap();
    accounts.create(&stored(9, "frank")).await.unwrap();

    let copy = dir.join("copy.db");
    accounts.snapshot(&copy).await.unwrap();
    let restored = SqliteAccountRepository::connect(&format!("sqlite://{}", copy.display()))
        .await
        .unwrap();
    let found = restored
        .find_by_username(&"frank".parse().unwrap())
        .await
        .unwrap();
    assert!(found.is_some());
    std::fs::remove_dir_all(&dir).unwrap();
}
