//! Typed entity ids. Every id is a UUIDv7 (RFC 9562): time-ordered, so database
//! indexes stay compact. Ids are identifiers, never secrets: anything that grants
//! access (room links, keys) is a separate 128-bit random value.

use std::fmt;
use std::str::FromStr;

use uuid::{Uuid, Version};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("malformed id: expected a UUIDv7")]
pub struct IdError;

/// Accepts only version 7, in any letter case (RFC 9562 parsing rule).
fn parse_v7(raw: &str) -> Result<Uuid, IdError> {
    let uuid = Uuid::try_parse(raw).map_err(|_| IdError)?;
    v7(uuid)
}

fn v7(uuid: Uuid) -> Result<Uuid, IdError> {
    if uuid.get_version() == Some(Version::SortRand) {
        Ok(uuid)
    } else {
        Err(IdError)
    }
}

macro_rules! entity_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            /// Wraps a UUID produced by the id generator; rejects other versions.
            pub fn from_uuid(uuid: Uuid) -> Result<Self, IdError> {
                v7(uuid).map(Self)
            }

            /// Builds the v7 id for a Unix time in milliseconds and 10 random bytes
            /// (pure; callers supply the clock and the randomness).
            #[must_use]
            pub fn from_parts(unix_ms: u64, random: [u8; 10]) -> Self {
                Self(uuid::Builder::from_unix_timestamp_millis(unix_ms, &random).into_uuid())
            }

            #[must_use]
            pub fn as_uuid(&self) -> Uuid {
                self.0
            }
        }

        impl FromStr for $name {
            type Err = IdError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                parse_v7(raw).map(Self)
            }
        }

        /// Lowercase hyphenated form, e.g. `01936f2e-…`.
        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.as_hyphenated().fmt(formatter)
            }
        }
    };
}

entity_id!(
    /// A meeting room. Internal and in logs and media-room names; the shareable
    /// link uses the room's random `RoomCode` instead.
    RoomId
);
entity_id!(
    /// A guest's request to join a room (guarded by the request key).
    RequestId
);
entity_id!(
    /// An account (the public handle is its `Username`).
    AccountId
);
entity_id!(
    /// One connection of a person to a media room (the media identity).
    ParticipantId
);

#[cfg(test)]
mod tests {
    use super::*;

    const V7: &str = "01936f2e-7b4c-7d2a-9c1e-3f5a6b7c8d9e";

    #[test]
    fn round_trips_in_lowercase_hyphenated_form() {
        let id: RoomId = V7.to_uppercase().parse().unwrap();
        assert_eq!(id.to_string(), V7);
    }

    #[test]
    fn ids_from_parts_are_v7_and_ordered_by_time() {
        let early = RoomId::from_parts(1_000, [9; 10]);
        let late = RoomId::from_parts(2_000, [0; 10]);
        assert!(early < late);
        assert_eq!(early.to_string().parse::<RoomId>(), Ok(early));
    }

    #[test]
    fn rejects_other_versions_and_garbage() {
        let v4 = "9b2c1f4e-3a5d-4c6b-8e7f-0a1b2c3d4e5f";
        assert_eq!(v4.parse::<RoomId>(), Err(IdError));
        assert_eq!("not-an-id".parse::<RequestId>(), Err(IdError));
        assert_eq!(AccountId::from_uuid(Uuid::nil()), Err(IdError));
    }
}
