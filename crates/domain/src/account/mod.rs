//! A person's identity: `@username` (also their room's name), password and mascot.

mod password;
mod username;

pub use password::{Password, PasswordError};
pub use username::{Username, UsernameError};

use crate::{AccountId, Mascot};

/// A person's identity: their `@username`, which is also the name of their room
/// (`/@username`), and the mascot that room shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub username: Username,
    pub mascot: Mascot,
}
