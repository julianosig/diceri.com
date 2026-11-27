//! A person's identity: `@username` (also their room's name), password and mascot.

mod network_id;
mod password;
mod password_strength;
mod username;

pub use network_id::NetworkId;
pub use password::{Password, PasswordError};
pub use username::{Username, UsernameError};

use crate::{AccountId, Mascot, Plan};

/// A person's identity: their `@username`, which is also the name of their room
/// (`/@username`), the mascot that room shows, and the plan that sets its limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: AccountId,
    pub username: Username,
    pub mascot: Mascot,
    pub plan: Plan,
}
