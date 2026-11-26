//! The request path as it may appear in logs: room codes and usernames are capabilities
//! and personal data (AGENTS.md §14 rules 4 and 27), so they are masked, and the query
//! string (which can carry a username) is never included.

/// Path segments whose *next* segment must not be logged, and what replaces it.
const MASKED_AFTER: [(&str, &str); 2] = [("rooms", "{room}"), ("users", "{user}")];

/// `path` with room codes and usernames masked, e.g. `/api/v1/rooms/{room}/voices`.
pub(crate) fn redacted_path(path: &str) -> String {
    let mut masked: Option<&str> = None;
    path.split('/')
        .map(|segment| {
            // Room pages of the web app are `/@username`.
            let shown = masked.take().unwrap_or(if segment.starts_with('@') {
                "@{user}"
            } else {
                segment
            });
            masked = MASKED_AFTER
                .iter()
                .find(|(after, _)| *after == segment)
                .map(|(_, mask)| *mask);
            shown
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_room_codes_and_usernames() {
        assert_eq!(
            redacted_path("/api/v1/rooms/7ZneGbyD70sehr76gaUaAX/voices"),
            "/api/v1/rooms/{room}/voices"
        );
        assert_eq!(
            redacted_path("/api/v1/users/ana/room"),
            "/api/v1/users/{user}/room"
        );
        assert_eq!(redacted_path("/@ana"), "/@{user}");
    }

    #[test]
    fn leaves_other_paths_alone() {
        assert_eq!(redacted_path("/api/v1/presence"), "/api/v1/presence");
        assert_eq!(redacted_path("/api/v1/rooms"), "/api/v1/rooms");
        assert_eq!(redacted_path("/"), "/");
    }
}
