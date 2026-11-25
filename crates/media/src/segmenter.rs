use diceri_application::ports::TranscriptEvent;

/// What the segmenter learned from one transcript event.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SegmentUpdate {
    /// Text of the phrase so far (final + provisional), for live captions.
    Progress(String),
    /// A complete phrase, ready to translate.
    Phrase(String),
}

/// Cuts a stream of transcript events into phrases at endpoints.
#[derive(Debug, Default)]
pub(crate) struct Segmenter {
    finals: String,
}

impl Segmenter {
    pub(crate) fn push(&mut self, event: TranscriptEvent) -> Option<SegmentUpdate> {
        match event {
            TranscriptEvent::Final(text) => {
                self.finals.push_str(&text);
                Some(SegmentUpdate::Progress(self.finals.trim().to_owned()))
            }
            TranscriptEvent::Partial(text) => {
                let live = format!("{}{text}", self.finals);
                Some(SegmentUpdate::Progress(live.trim().to_owned()))
            }
            TranscriptEvent::Endpoint => {
                let phrase = std::mem::take(&mut self.finals).trim().to_owned();
                (!phrase.is_empty()).then_some(SegmentUpdate::Phrase(phrase))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_(text: &str) -> TranscriptEvent {
        TranscriptEvent::Final(text.to_owned())
    }

    #[test]
    fn emits_a_phrase_at_each_endpoint() {
        let mut segmenter = Segmenter::default();
        segmenter.push(final_("Bom"));
        segmenter.push(final_(" dia."));
        assert_eq!(
            segmenter.push(TranscriptEvent::Endpoint),
            Some(SegmentUpdate::Phrase("Bom dia.".to_owned()))
        );
        segmenter.push(final_(" Tudo bem?"));
        assert_eq!(
            segmenter.push(TranscriptEvent::Endpoint),
            Some(SegmentUpdate::Phrase("Tudo bem?".to_owned()))
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
    fn empty_endpoints_are_ignored() {
        assert_eq!(Segmenter::default().push(TranscriptEvent::Endpoint), None);
    }
}
