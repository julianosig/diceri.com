use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier as _, Version};
use async_trait::async_trait;
use diceri_application::ports::{PasswordHash, PasswordHasher, PasswordHasherError};
use diceri_domain::Password;

/// OWASP Password Storage Cheat Sheet minimum for Argon2id: 19 MiB, 2 passes, 1 lane.
const MEMORY_KIB: u32 = 19 * 1024;
const PASSES: u32 = 2;
const LANES: u32 = 1;

/// Argon2id with a random salt per password, stored as a PHC string. Hashing runs on
/// the blocking pool so ~tens of milliseconds of CPU never stall the async runtime.
#[derive(Debug, Clone, Copy, Default)]
pub struct Argon2Hasher;

fn argon2() -> Result<Argon2<'static>, PasswordHasherError> {
    let params = Params::new(MEMORY_KIB, PASSES, LANES, None)
        .map_err(|e| PasswordHasherError(e.to_string()))?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

async fn blocking<T: Send + 'static>(
    job: impl FnOnce() -> Result<T, PasswordHasherError> + Send + 'static,
) -> Result<T, PasswordHasherError> {
    tokio::task::spawn_blocking(job)
        .await
        .map_err(|e| PasswordHasherError(e.to_string()))?
}

#[async_trait]
impl PasswordHasher for Argon2Hasher {
    async fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError> {
        let password = password.clone();
        blocking(move || {
            argon2()?
                .hash_password(password.expose().as_bytes())
                .map(|hash| PasswordHash(hash.to_string()))
                .map_err(|e| PasswordHasherError(e.to_string()))
        })
        .await
    }

    async fn verify(
        &self,
        password: &Password,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordHasherError> {
        let (password, hash) = (password.clone(), hash.0.clone());
        blocking(move || {
            match argon2()?.verify_password(password.expose().as_bytes(), hash.as_str()) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
                Err(e) => Err(PasswordHasherError(e.to_string())),
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn password(raw: &str) -> Password {
        raw.parse().unwrap()
    }

    #[tokio::test]
    async fn hashes_with_argon2id_and_a_fresh_salt() {
        let first = Argon2Hasher.hash(&password("correct horse")).await.unwrap();
        let second = Argon2Hasher.hash(&password("correct horse")).await.unwrap();
        assert!(first.0.starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));
        assert_ne!(first, second, "salts differ");
        assert!(!first.0.contains("correct horse"));
    }

    #[tokio::test]
    async fn verifies_only_the_right_password() {
        let hash = Argon2Hasher.hash(&password("correct horse")).await.unwrap();
        assert!(
            Argon2Hasher
                .verify(&password("correct horse"), &hash)
                .await
                .unwrap()
        );
        assert!(
            !Argon2Hasher
                .verify(&password("wrong horse!"), &hash)
                .await
                .unwrap()
        );
        let garbage = PasswordHash("not a hash".to_owned());
        assert!(
            Argon2Hasher
                .verify(&password("correct horse"), &garbage)
                .await
                .is_err()
        );
    }
}
