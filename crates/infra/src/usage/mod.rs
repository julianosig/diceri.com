//! Call minutes per account and day: the store behind the free plan's limits.

#[cfg(test)]
mod contract;
mod memory;
mod sqlite;

pub use memory::MemoryUsageRepository;
pub use sqlite::SqliteUsageRepository;
