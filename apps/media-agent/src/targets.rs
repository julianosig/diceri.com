//! Which languages each speaker is translated into.

use std::collections::HashMap;

use diceri_domain::LanguageCode;
use diceri_infra::LANGUAGE_ATTRIBUTE;

/// Participant attribute a speaker sets to hear themselves translated
/// (the "Translated" self-monitor); empty or absent when off.
pub(crate) const PREVIEW_ATTRIBUTE: &str = "preview_language";

/// The identity and attributes of a human participant in the room.
pub(crate) struct Member<'a> {
    pub identity: &'a str,
    pub attributes: &'a HashMap<String, String>,
}

fn attribute(member: &Member<'_>, key: &str) -> Option<LanguageCode> {
    member.attributes.get(key)?.parse().ok()
}

/// Languages to translate `speaker` into: every other member's language plus
/// the speaker's own preview language, deduplicated.
pub(crate) fn targets_for<'a>(
    members: impl IntoIterator<Item = Member<'a>>,
    speaker: &str,
) -> Vec<LanguageCode> {
    let mut languages: Vec<LanguageCode> = members
        .into_iter()
        .filter_map(|member| {
            let key = if member.identity == speaker {
                PREVIEW_ATTRIBUTE
            } else {
                LANGUAGE_ATTRIBUTE
            };
            attribute(&member, key)
        })
        .collect();
    languages.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    languages.dedup();
    languages
}

/// True when at least one member declared a language (i.e. humans are present).
pub(crate) fn has_listeners<'a>(members: impl IntoIterator<Item = Member<'a>>) -> bool {
    members
        .into_iter()
        .any(|member| attribute(&member, LANGUAGE_ATTRIBUTE).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attrs(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    fn codes(languages: &[LanguageCode]) -> Vec<&str> {
        languages.iter().map(LanguageCode::as_str).collect()
    }

    #[test]
    fn other_members_languages_are_targets_but_not_the_speakers_own() {
        let ana = attrs(&[(LANGUAGE_ATTRIBUTE, "pt-BR")]);
        let bob = attrs(&[(LANGUAGE_ATTRIBUTE, "en")]);
        let carla = attrs(&[(LANGUAGE_ATTRIBUTE, "en")]);
        let members = [
            Member {
                identity: "ana",
                attributes: &ana,
            },
            Member {
                identity: "bob",
                attributes: &bob,
            },
            Member {
                identity: "carla",
                attributes: &carla,
            },
        ];
        assert_eq!(codes(&targets_for(members, "ana")), ["en"]);
    }

    #[test]
    fn a_speaker_alone_hears_their_preview_language() {
        let ana = attrs(&[(LANGUAGE_ATTRIBUTE, "pt-BR"), (PREVIEW_ATTRIBUTE, "ja")]);
        let members = [Member {
            identity: "ana",
            attributes: &ana,
        }];
        assert_eq!(codes(&targets_for(members, "ana")), ["ja"]);
    }

    #[test]
    fn an_empty_preview_adds_nothing() {
        let ana = attrs(&[(LANGUAGE_ATTRIBUTE, "pt-BR"), (PREVIEW_ATTRIBUTE, "")]);
        let members = [Member {
            identity: "ana",
            attributes: &ana,
        }];
        assert_eq!(codes(&targets_for(members, "ana")), Vec::<&str>::new());
    }
}
