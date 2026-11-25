use std::collections::HashMap;

use diceri_application::ports::TranscriptEvent;
use diceri_domain::LanguageCode;

/// What the segmenter learned from one transcript event.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SegmentUpdate {
    /// Text of the phrase so far (final + provisional), for live captions.
    Progress(String),
    /// A complete phrase, ready to translate, with the language most of it was
    /// detected in (if the recognizer reports languages).
    Phrase {
        text: String,
        language: Option<LanguageCode>,
    },
}

/// Cuts a stream of transcript events into phrases at endpoints.
#[derive(Debug, Default)]
pub(crate) struct Segmenter {
    finals: String,
    /// Characters of final text per detected language in the current phrase.
    language_votes: HashMap<LanguageCode, usize>,
}

impl Segmenter {
    pub(crate) fn push(&mut self, event: TranscriptEvent) -> Option<SegmentUpdate> {
        match event {
            TranscriptEvent::Final { text, language } => {
                if let Some(language) = language {
                    *self.language_votes.entry(language).or_default() += text.trim().len();
                }
                self.finals.push_str(&text);
                Some(SegmentUpdate::Progress(self.finals.trim().to_owned()))
            }
            TranscriptEvent::Partial(text) => {
                let live = format!("{}{text}", self.finals);
                Some(SegmentUpdate::Progress(live.trim().to_owned()))
            }
            TranscriptEvent::Endpoint => {
                let text = std::mem::take(&mut self.finals).trim().to_owned();
                let language = self.dominant_language();
                self.language_votes.clear();
                (!text.is_empty()).then_some(SegmentUpdate::Phrase { text, language })
            }
        }
    }
}

impl Segmenter {
    fn dominant_language(&self) -> Option<LanguageCode> {
        self.language_votes
            .iter()
            .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.as_str().cmp(a.0.as_str())))
            .map(|(language, _)| language.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_(text: &str) -> TranscriptEvent {
        TranscriptEvent::Final {
            text: text.to_owned(),
            language: None,
        }
    }

    fn final_in(text: &str, language: &str) -> TranscriptEvent {
        TranscriptEvent::Final {
            text: text.to_owned(),
            language: Some(language.parse().unwrap()),
        }
    }

    fn phrase(text: &str, language: Option<&str>) -> SegmentUpdate {
        SegmentUpdate::Phrase {
            text: text.to_owned(),
            language: language.map(|l| l.parse().unwrap()),
        }
    }

    #[test]
    fn emits_a_phrase_at_each_endpoint() {
        let mut segmenter = Segmenter::default();
        segmenter.push(final_("Bom"));
        segmenter.push(final_(" dia."));
        assert_eq!(
            segmenter.push(TranscriptEvent::Endpoint),
            Some(phrase("Bom dia.", None))
        );
        segmenter.push(final_(" Tudo bem?"));
        assert_eq!(
            segmenter.push(TranscriptEvent::Endpoint),
            Some(phrase("Tudo bem?", None))
        );
    }

    #[test]
    fn progress_includes_provisional_text() {
        let mut segmenter = Segmenter::default();
        segmenter.push(final_("Bom"));
        assert_eq!(
            segmenter.push(TranscriptEvent::Partial(" di".to_owned())),
            Some(SegmentUpdate::Progress("Bom di".to_owned()))
        );
    }

    #[test]
    fn phrase_language_is_the_one_most_of_it_was_detected_in() {
        let mut segmenter = Segmenter::default();
        segmenter.push(final_in("Amanhã a gente revisa", "pt"));
        segmenter.push(final_in(" OK", "en"));
        assert_eq!(
            segmenter.push(TranscriptEvent::Endpoint),
            Some(phrase("Amanhã a gente revisa OK", Some("pt")))
        );
        segmenter.push(final_in("Next one", "en"));
        assert_eq!(
            segmenter.push(TranscriptEvent::Endpoint),
            Some(phrase("Next one", Some("en")))
        );
    }

    #[test]
    fn empty_endpoints_are_ignored() {
        assert_eq!(Segmenter::default().push(TranscriptEvent::Endpoint), None);
    }
}
