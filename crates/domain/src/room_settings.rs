use crate::Mascot;

/// How people get in once they reach the room (`/@<owner>` or the live board).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EntryPolicy {
    /// Anyone who asks is let in right away.
    Open,
    /// The host admits or denies each person.
    #[default]
    Approval,
}

/// How the owner runs their room. A room is named after its owner (`@username`);
/// the default is private and host-approved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RoomSettings {
    /// Shown on the public live board. Private by default.
    pub listed: bool,
    pub entry: EntryPolicy,
    /// The room's animated symbol (the owner's mascot).
    pub mascot: Mascot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rooms_are_private_and_approved_by_default() {
        let settings = RoomSettings::default();
        assert!(!settings.listed);
        assert_eq!(settings.entry, EntryPolicy::Approval);
    }
}
