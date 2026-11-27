//! Each account's own voice recording (ADR 0015): in memory for tests and local runs,
//! encrypted in the accounts database in production.

#[cfg(test)]
mod contract;
mod memory;
mod sealed;
mod sqlite;

pub use memory::MemoryVoiceSamples;
pub use sqlite::SqliteVoiceSamples;
