//! Which languages each speaker is translated into.

use std::collections::HashMap;

use diceri_domain::LanguageCode;
use diceri_infra::OUTPUT_LANGUAGE_ATTRIBUTE;

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
                OUTPUT_LANGUAGE_ATTRIBUTE
            };
            attribute(&member, key)
        })
        .collect();
    languages.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    languages.dedup();
    languages
}

/// Languages a speaker is likely to use, given as recognition hints: every
/// member's output language (speaker included) plus the speaker's preview.
pub(crate) fn speech_hints<'a>(
    members: impl IntoIterator<Item = Member<'a>>,
    speaker: &str,
) -> Vec<LanguageCode> {
    let mut languages: Vec<LanguageCode> = Vec::new();
    for member in members {
        languages.extend(attribute(&member, OUTPUT_LANGUAGE_ATTRIBUTE));
        if member.identity == speaker {
            languages.extend(attribute(&member, PREVIEW_ATTRIBUTE));
        }
    }
    languages.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    languages.dedup();
    languages
}

/// True when at least one member has an output language (i.e. humans are present).
pub(crate) fn has_listeners<'a>(members: impl IntoIterator<Item = Member<'a>>) -> bool {
    members
        .into_iter()
        .any(|member| attribute(&member, OUTPUT_LANGUAGE_ATTRIBUTE).is_some())
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
        let ana = attrs(&[(OUTPUT_LANGUAGE_ATTRIBUTE, "pt-BR")]);
        let bob = attrs(&[(OUTPUT_LANGUAGE_ATTRIBUTE, "en")]);
        let carla = attrs(&[(OUTPUT_LANGUAGE_ATTRIBUTE, "en")]);
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
    fn hints_cover_every_output_language_and_the_speakers_preview() {
        let ana = attrs(&[(OUTPUT_LANGUAGE_ATTRIBUTE, "en"), (PREVIEW_ATTRIBUTE, "ja")]);
        let bob = attrs(&[
            (OUTPUT_LANGUAGE_ATTRIBUTE, "pt-BR"),
            (PREVIEW_ATTRIBUTE, "es"),
        ]);
        let members = [
            Member {
                identity: "ana",
                attributes: &ana,
            },
            Member {
                identity: "bob",
                attributes: &bob,
            },
        ];
        assert_eq!(codes(&speech_hints(members, "ana")), ["en", "ja", "pt-BR"]);
    }

    #[test]
    fn a_speaker_alone_hears_their_preview_language() {
        let ana = attrs(&[
            (OUTPUT_LANGUAGE_ATTRIBUTE, "pt-BR"),
            (PREVIEW_ATTRIBUTE, "ja"),
        ]);
        let members = [Member {
            identity: "ana",
            attributes: &ana,
        }];
        assert_eq!(codes(&targets_for(members, "ana")), ["ja"]);
    }

    #[test]
    fn an_empty_preview_adds_nothing() {
        let ana = attrs(&[
            (OUTPUT_LANGUAGE_ATTRIBUTE, "pt-BR"),
            (PREVIEW_ATTRIBUTE, ""),
        ]);
        let members = [Member {
            identity: "ana",
            attributes: &ana,
        }];
        assert_eq!(codes(&targets_for(members, "ana")), Vec::<&str>::new());
    }
}
