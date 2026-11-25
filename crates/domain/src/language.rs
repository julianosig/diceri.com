use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// BCP 47 language tag restricted to `language` or `language-REGION` (e.g. `en`, `pt-BR`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct LanguageCode(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid language code `{0}`, expected `ll` or `ll-RR`")]
pub struct LanguageCodeError(String);

impl LanguageCode {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_valid(raw: &str) -> bool {
    let mut parts = raw.split('-');
    let language_ok = parts.next().is_some_and(|lang| {
        (2..=3).contains(&lang.len()) && lang.bytes().all(|b| b.is_ascii_lowercase())
    });
    let region_ok = parts
        .next()
        .is_none_or(|region| region.len() == 2 && region.bytes().all(|b| b.is_ascii_uppercase()));
    language_ok && region_ok && parts.next().is_none()
}

impl FromStr for LanguageCode {
    type Err = LanguageCodeError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if is_valid(raw) {
            Ok(Self(raw.to_owned()))
        } else {
            Err(LanguageCodeError(raw.to_owned()))
        }
    }
}

impl TryFrom<String> for LanguageCode {
    type Error = LanguageCodeError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        raw.parse()
    }
}

impl From<LanguageCode> for String {
    fn from(code: LanguageCode) -> Self {
        code.0
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_language_only_and_language_region() {
        for raw in ["en", "pt-BR", "yue"] {
            assert!(raw.parse::<LanguageCode>().is_ok(), "{raw} should be valid");
        }
    }

    #[test]
    fn rejects_malformed_tags() {
        for raw in ["", "EN", "pt-br", "pt-BR-x", "english"] {
            assert!(
                raw.parse::<LanguageCode>().is_err(),
                "{raw} should be invalid"
            );
        }
    }

    #[test]
    fn deserializes_with_validation() {
        let parsed: Result<LanguageCode, _> = serde_json::from_str("\"pt-BR\"");
        assert!(parsed.is_ok());
        let invalid: Result<LanguageCode, _> = serde_json::from_str("\"PT\"");
        assert!(invalid.is_err());
    }
}
