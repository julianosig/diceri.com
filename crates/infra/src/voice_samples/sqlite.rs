use async_trait::async_trait;
use diceri_application::ports::{VoiceSample, VoiceSampleStore, VoiceSampleStoreError};
use diceri_domain::AccountId;
use sqlx::SqlitePool;

use super::sealed::VoiceKey;
use crate::SqliteAccountRepository;

/// Recordings in the accounts database (`voice_samples`), encrypted under the voice key.
#[derive(Debug)]
pub struct SqliteVoiceSamples {
    pool: SqlitePool,
    key: VoiceKey,
}

impl SqliteVoiceSamples {
    /// Uses the accounts database (whose migrations create `voice_samples`) and `key`, the
    /// 32 bytes of `DICERI_VOICE_KEY`.
    #[must_use]
    pub fn sharing(accounts: &SqliteAccountRepository, key: [u8; 32]) -> Self {
        Self {
            pool: accounts.pool().clone(),
            key: VoiceKey::new(key),
        }
    }
}

fn storage(error: &dyn std::fmt::Display) -> VoiceSampleStoreError {
    VoiceSampleStoreError::Storage(error.to_string())
}

#[async_trait]
impl VoiceSampleStore for SqliteVoiceSamples {
    async fn save(
        &self,
        account: &AccountId,
        sample: &VoiceSample,
        recorded_at_ms: u64,
    ) -> Result<(), VoiceSampleStoreError> {
        let (nonce, sealed) = self
            .key
            .seal(account, &sample.bytes)
            .ok_or_else(|| storage(&"voice sample could not be encrypted"))?;
        // Served by `pk_voice_samples`.
        sqlx::query(
            "INSERT INTO voice_samples (account_id, content_type, nonce, ciphertext, recorded_at) \
             VALUES (?, ?, ?, ?, ?) \
             ON CONFLICT (account_id) DO UPDATE SET content_type = excluded.content_type, \
             nonce = excluded.nonce, ciphertext = excluded.ciphertext, \
             recorded_at = excluded.recorded_at",
        )
        .bind(account.as_uuid())
        .bind(sample.content_type.as_str())
        .bind(nonce.as_slice())
        .bind(sealed)
        .bind(i64::try_from(recorded_at_ms).unwrap_or(i64::MAX))
        .execute(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        Ok(())
    }

    async fn find(
        &self,
        account: &AccountId,
    ) -> Result<Option<VoiceSample>, VoiceSampleStoreError> {
        // Served by `pk_voice_samples`.
        let row: Option<(String, Vec<u8>, Vec<u8>)> = sqlx::query_as(
            "SELECT content_type, nonce, ciphertext FROM voice_samples WHERE account_id = ?",
        )
        .bind(account.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        let Some((content_type, nonce, sealed)) = row else {
            return Ok(None);
        };
        let nonce: [u8; 12] = nonce
            .try_into()
            .map_err(|_| storage(&"stored nonce is not 12 bytes"))?;
        // A recording the key cannot open (another key, altered row) is a storage error.
        let bytes = self
            .key
            .open(account, nonce, &sealed)
            .ok_or_else(|| storage(&"voice sample could not be decrypted"))?;
        Ok(Some(VoiceSample {
            bytes,
            content_type,
        }))
    }

    async fn has(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError> {
        // Served by `pk_voice_samples`.
        let (found,): (i64,) =
            sqlx::query_as("SELECT EXISTS (SELECT 1 FROM voice_samples WHERE account_id = ?)")
                .bind(account.as_uuid())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| storage(&e))?;
        Ok(found == 1)
    }

    async fn delete(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError> {
        // Served by `pk_voice_samples`.
        let result = sqlx::query("DELETE FROM voice_samples WHERE account_id = ?")
            .bind(account.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| storage(&e))?;
        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use diceri_application::ports::AccountRepository as _;

    use super::*;
    use crate::account_contract::stored;

    async fn store() -> (SqliteVoiceSamples, AccountId) {
        let accounts = SqliteAccountRepository::connect("sqlite::memory:")
            .await
            .unwrap();
        let ana = stored(1, "ana");
        accounts.create(&ana).await.unwrap();
        (
            SqliteVoiceSamples::sharing(&accounts, [9; 32]),
            ana.account.id,
        )
    }

    #[tokio::test]
    async fn honours_the_voice_sample_contract() {
        let (samples, account) = store().await;
        super::super::contract::check(&samples, &account).await;
    }

    #[tokio::test]
    async fn keeps_only_ciphertext_in_the_database() {
        let (samples, account) = store().await;
        let audio = VoiceSample {
            bytes: b"RIFF....WAVEfmt ".repeat(8),
            content_type: "audio/wav".to_owned(),
        };
        samples.save(&account, &audio, 1).await.unwrap();
        let (stored,): (Vec<u8>,) = sqlx::query_as("SELECT ciphertext FROM voice_samples")
            .fetch_one(&samples.pool)
            .await
            .unwrap();
        assert!(!stored.windows(4).any(|window| window == b"RIFF"));
        assert_eq!(
            samples.find(&account).await.unwrap().unwrap().bytes,
            audio.bytes
        );
    }
}
