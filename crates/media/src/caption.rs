use diceri_domain::LanguageCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptionKind {
    /// Live, may still change (source language only).
    Partial,
    /// Final text of a phrase in the speaker's language.
    Original,
    /// The phrase translated into `language`.
    Translation,
}

/// Text shown to listeners, sent alongside (and ahead of) translated audio.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caption {
    pub speaker: String,
    /// Language of `text`.
    pub language: LanguageCode,
    pub kind: CaptionKind,
    /// Sequence number of the phrase within this speaker's stream.
    pub phrase: u64,
    pub text: String,
}
