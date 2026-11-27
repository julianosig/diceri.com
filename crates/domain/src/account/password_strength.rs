//! Refuses passwords that are easy to guess, following NIST SP 800-63B §3.1.1.2: no
//! composition rules, but no commonly used, sequential, repetitive or context-specific
//! passwords. Mirrored for instant feedback in `apps/web/src/lib/features/account/password.ts`
//! (keep both lists and rules in step; the tests share their examples).

use super::{Password, PasswordError};

/// Among the most used passwords of 8 or more characters (lowercase).
const COMMON: &[&str] = &[
    "password",
    "password1",
    "password12",
    "password123",
    "passw0rd",
    "p@ssw0rd",
    "qwertyui",
    "qwertyuiop",
    "qwerty123",
    "qwerty12",
    "1q2w3e4r",
    "1qaz2wsx",
    "zaq12wsx",
    "asdfghjk",
    "asdfghjkl",
    "zxcvbnm1",
    "iloveyou",
    "iloveyou1",
    "abcd1234",
    "abc12345",
    "abcdefg1",
    "aa123456",
    "a1b2c3d4",
    "trustno1",
    "sunshine",
    "princess",
    "football",
    "baseball",
    "superman",
    "starwars",
    "whatever",
    "welcome1",
    "welcome123",
    "letmein1",
    "admin123",
    "administrator",
    "changeme",
    "computer",
    "internet",
    "michelle",
    "jennifer",
    "shadow12",
    "master12",
    "monkey12",
    "dragon12",
    "loveyou1",
    "88888888",
    "987654321",
    "123123123",
    "123qweasd",
    "qweasdzxc",
    "senha123",
    "senha1234",
    "mudar123",
    "brasil123",
];

/// Words that make up most guessable passwords once digits and symbols are removed.
const COMMON_WORDS: &[&str] = &[
    "password",
    "passwort",
    "senha",
    "contrasena",
    "motdepasse",
    "qwerty",
    "azerty",
    "asdf",
    "abc",
    "abcd",
    "iloveyou",
    "welcome",
    "admin",
    "letmein",
    "monkey",
    "dragon",
    "master",
    "sunshine",
    "princess",
    "football",
    "baseball",
    "soccer",
    "superman",
    "batman",
    "love",
    "hello",
    "test",
    "user",
    "login",
    "secret",
    "brasil",
    "brazil",
    "diceri",
    "qwe",
    "asd",
];

/// Fewer distinct characters than this is a repetition (`aaaaaaaa`, `abababab`).
const MIN_DISTINCT_CHARS: usize = 5;
/// Digits only is a PIN; long ones are fine.
const MIN_DIGITS_ONLY: usize = 12;

impl Password {
    /// Checks a new password is not easy to guess for this account. Only for choosing a
    /// password: signing in accepts whatever was chosen before.
    ///
    /// # Errors
    /// [`PasswordError::TooEasy`] when it is common, a sequence or repetition, digits only
    /// and short, a common word with digits or symbols, or contains `username`.
    pub fn ensure_hard_to_guess(&self, username: &str) -> Result<(), PasswordError> {
        if is_easy(&self.expose().to_lowercase(), &username.to_lowercase()) {
            return Err(PasswordError::TooEasy);
        }
        Ok(())
    }
}

fn is_easy(password: &str, username: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let letters: String = chars.iter().filter(|c| c.is_alphabetic()).collect();
    let distinct = {
        let mut seen = chars.clone();
        seen.sort_unstable();
        seen.dedup();
        seen.len()
    };
    COMMON.contains(&password)
        || distinct < MIN_DISTINCT_CHARS
        || is_sequence(&chars)
        || (letters.is_empty() && chars.len() < MIN_DIGITS_ONLY)
        || COMMON_WORDS.contains(&letters.as_str())
        || (username.chars().count() >= 3 && password.contains(username))
}

/// Every character one step after (or before) the previous one: `12345678`, `hgfedcba`.
fn is_sequence(chars: &[char]) -> bool {
    let steps: Vec<i64> = chars
        .iter()
        .zip(chars.iter().skip(1))
        .map(|(&before, &after)| i64::from(u32::from(after)) - i64::from(u32::from(before)))
        .collect();
    steps.iter().all(|&step| step == 1) || steps.iter().all(|&step| step == -1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn easy(raw: &str, username: &str) -> bool {
        let password: Password = raw.parse().unwrap();
        password.ensure_hard_to_guess(username).is_err()
    }

    #[test]
    fn refuses_guessable_passwords() {
        for raw in [
            "12345678",
            "87654321",
            "abcdefgh",
            "aaaaaaaa",
            "abababab",
            "11223344",
            "password",
            "Password123!",
            "qwerty123",
            "iloveyou",
            "48291736",
            "Senha@2024",
        ] {
            assert!(easy(raw, "ana"), "{raw} should be refused");
        }
        assert!(easy("ana_92-rocks!", "ana_92"));
    }

    #[test]
    fn accepts_hard_to_guess_passwords() {
        for raw in [
            "correct horse battery",
            "Tr0ub4dor&3",
            "blue-otter-sings",
            "829174650312",
        ] {
            assert!(!easy(raw, "ana"), "{raw} should be accepted");
        }
    }
}
