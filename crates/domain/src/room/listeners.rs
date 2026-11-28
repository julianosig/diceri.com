//! Hidden listeners' media identities: `monitor-<their person's seat>.<connection id>`. Each
//! connection has its own id (a UUIDv7, so a later one sorts after an earlier one): removing
//! an old connection can never hit the one that replaced it.

const PREFIX: &str = "monitor-";

/// The identity of a new listener for the person seated as `seat`.
#[must_use]
pub fn listener_identity(seat: &str, connection: &str) -> String {
    format!("{PREFIX}{seat}.{connection}")
}

/// The seat of the person a listener belongs to; `None` for anything that is not a listener.
#[must_use]
pub fn listener_seat(identity: &str) -> Option<&str> {
    let rest = identity.strip_prefix(PREFIX)?;
    Some(rest.split_once('.').map_or(rest, |(seat, _)| seat))
}

/// The listeners to remove: those whose person is not among `people`, and all but the newest
/// of each person's (one listener per person, as one hearing themselves needs).
#[must_use]
pub fn surplus_listeners(people: &[String], listeners: &[String]) -> Vec<String> {
    let mut newest: std::collections::HashMap<&str, &String> = std::collections::HashMap::new();
    let mut surplus = Vec::new();
    for listener in listeners {
        match listener_seat(listener) {
            Some(seat) if people.iter().any(|person| person == seat) => {
                match newest.insert(seat, listener) {
                    Some(older) if older > listener => {
                        newest.insert(seat, older);
                        surplus.push(listener.clone());
                    }
                    Some(older) => surplus.push(older.clone()),
                    None => {}
                }
            }
            _ => surplus.push(listener.clone()),
        }
    }
    surplus
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(items: &[&str]) -> Vec<String> {
        items.iter().map(|&item| item.to_owned()).collect()
    }

    #[test]
    fn a_listener_names_its_persons_seat() {
        let identity = listener_identity("p-1", "0197");
        assert_eq!(identity, "monitor-p-1.0197");
        assert_eq!(listener_seat(&identity), Some("p-1"));
        assert_eq!(listener_seat("monitor-p-1"), Some("p-1"));
        assert_eq!(listener_seat("p-1"), None);
    }

    #[test]
    fn only_the_newest_listener_of_a_present_person_stays() {
        let people = owned(&["p-1"]);
        let listeners = owned(&["monitor-p-1.02", "monitor-p-1.01", "monitor-p-2.09", "x"]);
        let mut surplus = surplus_listeners(&people, &listeners);
        surplus.sort();
        assert_eq!(surplus, owned(&["monitor-p-1.01", "monitor-p-2.09", "x"]));
    }
}
