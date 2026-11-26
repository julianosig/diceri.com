use diceri_domain::LanguageCode;

use super::MediaAccessError;

/// A media room as seen right now, reduced to what is safe to show publicly:
/// no room id, no names, only how many people are in and what they listen in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveRoom {
    pub people: usize,
    /// Output language of each person that has one (duplicates kept).
    pub languages: Vec<LanguageCode>,
}

/// Who is connected to the media server at this moment.
#[async_trait::async_trait]
pub trait MediaPresence: Send + Sync {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError>;
}
