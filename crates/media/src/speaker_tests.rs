use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_application::ports::{
    ContextTurn, SpeechAudio, SpeechRequest, TextToSpeech, TextToSpeechError, TranscriptEvent,
    TranslationRequest, Translator, TranslatorError, VoiceId,
};
use diceri_domain::LanguageCode;
use tokio::sync::{mpsc, watch};

use super::{SpeakerConfig, SpeakerPipeline};
use crate::{Caption, CaptionKind, Conversation, PipelineOutput};

/// Translates by tagging the text with the target language: "[en] Olá".
struct TaggingTranslator {
    contexts: Mutex<Vec<Vec<ContextTurn>>>,
}

#[async_trait]
impl Translator for TaggingTranslator {
    async fn translate(&self, request: TranslationRequest<'_>) -> Result<String, TranslatorError> {
        self.contexts.lock().unwrap().push(request.context.to_vec());
        if request.text == "fail" {
            return Err(TranslatorError::Empty);
        }
        Ok(format!("[{}] {}", request.target.primary(), request.text))
    }
}

/// Returns one chunk whose length is the text length; refuses the voice
/// `refused` (like a clone that needs verification) and records voices used.
#[derive(Default)]
struct FakeTts {
    refused: Option<&'static str>,
    /// Fails the first request as busy (HTTP 429), then works.
    busy_once: std::sync::atomic::AtomicBool,
    voices_used: Mutex<Vec<String>>,
}

#[async_trait]
impl TextToSpeech for FakeTts {
    async fn synthesize(
        &self,
        request: SpeechRequest<'_>,
    ) -> Result<SpeechAudio, TextToSpeechError> {
        self.voices_used
            .lock()
            .unwrap()
            .push(request.voice.0.clone());
        if self
            .busy_once
            .swap(false, std::sync::atomic::Ordering::Relaxed)
        {
            return Err(TextToSpeechError::from_status(429));
        }
        if self.refused == Some(request.voice.0.as_str()) {
            return Err(TextToSpeechError::Request(
                "voice requires verification".to_owned(),
            ));
        }
        let (tx, rx) = mpsc::channel(1);
        tx.send(Ok(vec![0; request.text.len()])).await.unwrap();
        Ok(rx)
    }
}

#[derive(Default)]
struct RecordingOutput {
    captions: Mutex<Vec<Caption>>,
    spoken: Mutex<Vec<(String, String, usize)>>,
}

#[async_trait]
impl PipelineOutput for RecordingOutput {
    async fn caption(&self, caption: Caption) {
        self.captions.lock().unwrap().push(caption);
    }

    async fn speak(&self, speaker: &str, language: &LanguageCode, mut audio: SpeechAudio) {
        let mut samples = 0;
        while let Some(chunk) = audio.recv().await {
            samples += chunk.unwrap().len();
        }
        self.spoken
            .lock()
            .unwrap()
            .push((speaker.to_owned(), language.to_string(), samples));
    }
}

fn lang(code: &str) -> LanguageCode {
    code.parse().unwrap()
}

struct Harness {
    output: Arc<RecordingOutput>,
    translator: Arc<TaggingTranslator>,
    tts: Arc<FakeTts>,
}

impl Harness {
    async fn run(events: Vec<TranscriptEvent>, targets: &[&str]) -> Self {
        Self::run_with(events, targets, FakeTts::default()).await
    }

    async fn run_with(events: Vec<TranscriptEvent>, targets: &[&str], tts: FakeTts) -> Self {
        Self::run_in(events, targets, tts, Arc::new(Conversation::default())).await
    }

    /// Runs Ana's pipeline in an existing call's conversation.
    async fn run_in(
        events: Vec<TranscriptEvent>,
        targets: &[&str],
        tts: FakeTts,
        conversation: Arc<Conversation>,
    ) -> Self {
        let tts = Arc::new(tts);
        let output = Arc::new(RecordingOutput::default());
        let translator = Arc::new(TaggingTranslator {
            contexts: Mutex::default(),
        });
        let config = SpeakerConfig {
            speaker: "ana".to_owned(),
            fallback_language: lang("pt-BR"),
            voice: Some(VoiceId("clone".to_owned())),
        };
        let pipeline = SpeakerPipeline::new(
            config,
            translator.clone(),
            tts.clone(),
            output.clone(),
            conversation,
        );
        let (tx, rx) = mpsc::channel(64);
        for event in events {
            tx.send(Ok(event)).await.unwrap();
        }
        drop(tx);
        let (_targets_tx, targets_rx) = watch::channel(targets.iter().map(|t| lang(t)).collect());
        pipeline.run(rx, targets_rx).await;
        Self {
            output,
            translator,
            tts,
        }
    }

    fn texts(&self, kind: CaptionKind) -> Vec<String> {
        let captions = self.output.captions.lock().unwrap();
        captions
            .iter()
            .filter(|c| c.kind == kind)
            .map(|c| c.text.clone())
            .collect()
    }
}

fn phrase(text: &str) -> Vec<TranscriptEvent> {
    vec![
        TranscriptEvent::Final {
            text: text.to_owned(),
            language: None,
        },
        TranscriptEvent::Endpoint,
    ]
}

fn phrase_in(text: &str, language: &str) -> Vec<TranscriptEvent> {
    let language = Some(language.parse().unwrap());
    vec![
        TranscriptEvent::Final {
            text: text.to_owned(),
            language,
        },
        TranscriptEvent::Endpoint,
    ]
}

#[tokio::test]
async fn translates_each_phrase_into_every_other_language() {
    let harness = Harness::run(phrase("Olá"), &["pt-BR", "en", "es"]).await;
    let mut translations = harness.texts(CaptionKind::Translation);
    translations.sort();
    assert_eq!(translations, vec!["[en] Olá", "[es] Olá"]);
    assert_eq!(harness.texts(CaptionKind::Original), vec!["Olá"]);
    assert_eq!(harness.output.spoken.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn never_translates_into_the_speakers_own_language() {
    let harness = Harness::run(phrase("Olá"), &["pt", "pt-PT"]).await;
    assert_eq!(
        harness.texts(CaptionKind::Translation),
        Vec::<String>::new()
    );
    assert_eq!(harness.output.spoken.lock().unwrap().len(), 0);
}

#[tokio::test]
async fn keeps_phrase_order_per_language_and_passes_context() {
    let events = [phrase("um"), phrase("dois"), phrase("três")].concat();
    let harness = Harness::run(events, &["en"]).await;
    assert_eq!(
        harness.texts(CaptionKind::Translation),
        vec!["[en] um", "[en] dois", "[en] três"]
    );
    let contexts = harness.translator.contexts.lock().unwrap();
    let last: Vec<&str> = contexts
        .last()
        .unwrap()
        .iter()
        .map(|t| t.text.as_str())
        .collect();
    assert_eq!(last, ["um", "dois"]);
}

#[tokio::test]
async fn a_failed_translation_does_not_stop_the_stream() {
    let events = [phrase("fail"), phrase("ok")].concat();
    let harness = Harness::run(events, &["en"]).await;
    assert_eq!(harness.texts(CaptionKind::Translation), vec!["[en] ok"]);
}

#[tokio::test]
async fn publishes_live_partial_captions() {
    let events = vec![TranscriptEvent::Partial("Ol".to_owned())];
    let harness = Harness::run(events, &["en"]).await;
    assert_eq!(harness.texts(CaptionKind::Partial), vec!["Ol"]);
}

#[tokio::test]
async fn translates_from_the_detected_language_not_the_declared_one() {
    // Declared pt-BR, but this phrase was detected as English.
    let harness = Harness::run(phrase_in("Hello", "en"), &["en", "es"]).await;
    assert_eq!(harness.texts(CaptionKind::Translation), vec!["[es] Hello"]);
    let captions = harness.output.captions.lock().unwrap();
    let original = captions
        .iter()
        .find(|c| c.kind == CaptionKind::Original)
        .unwrap();
    assert_eq!(original.language.as_str(), "en");
}

#[tokio::test]
async fn a_refused_clone_leaves_the_speaker_captions_only() {
    let tts = FakeTts {
        refused: Some("clone"),
        ..FakeTts::default()
    };
    let events = [phrase("um"), phrase("dois")].concat();
    let harness = Harness::run_with(events, &["en"], tts).await;
    // Never a stock voice: the clone is tried once, then translations are captions only.
    assert!(harness.output.spoken.lock().unwrap().is_empty());
    let used = harness.tts.voices_used.lock().unwrap().clone();
    assert_eq!(used, vec!["clone"]);
    assert_eq!(harness.texts(CaptionKind::Translation).len(), 2);
}

#[tokio::test]
async fn a_busy_provider_loses_one_phrase_not_the_voice() {
    let tts = FakeTts {
        busy_once: std::sync::atomic::AtomicBool::new(true),
        ..FakeTts::default()
    };
    let events = [phrase("um"), phrase("dois")].concat();
    let harness = Harness::run_with(events, &["en"], tts).await;
    // The first phrase is captioned only; the second is spoken in the clone again.
    assert_eq!(harness.output.spoken.lock().unwrap().len(), 1);
    assert_eq!(harness.texts(CaptionKind::Translation).len(), 2);
}

/// Translates to exactly the same text, as happens with a misdetected "Alô.".
struct EchoTranslator;

#[async_trait]
impl Translator for EchoTranslator {
    async fn translate(&self, request: TranslationRequest<'_>) -> Result<String, TranslatorError> {
        Ok(request.text.to_owned())
    }
}

#[tokio::test]
async fn an_unchanged_translation_is_captioned_but_not_spoken() {
    let output = Arc::new(RecordingOutput::default());
    let config = SpeakerConfig {
        speaker: "ana".to_owned(),
        fallback_language: lang("pt-BR"),
        voice: Some(VoiceId("clone".to_owned())),
    };
    let pipeline = SpeakerPipeline::new(
        config,
        Arc::new(EchoTranslator),
        Arc::new(FakeTts::default()),
        output.clone(),
        Arc::new(Conversation::default()),
    );
    let (tx, rx) = mpsc::channel(8);
    for event in phrase_in("Alô.", "es") {
        tx.send(Ok(event)).await.unwrap();
    }
    drop(tx);
    let (_targets, targets_rx) = watch::channel(vec![lang("pt-BR")]);
    pipeline.run(rx, targets_rx).await;
    let translations = output
        .captions
        .lock()
        .unwrap()
        .iter()
        .filter(|c| c.kind == CaptionKind::Translation)
        .count();
    assert_eq!(translations, 1);
    assert_eq!(output.spoken.lock().unwrap().len(), 0);
}

#[tokio::test]
async fn translates_knowing_what_the_others_said() {
    let conversation = Arc::new(Conversation::default());
    conversation.record("bob", "Shall we meet at the park?");
    let harness = Harness::run_in(phrase("sim"), &["en"], FakeTts::default(), conversation).await;
    let contexts = harness.translator.contexts.lock().unwrap();
    let turn = &contexts.last().unwrap()[0];
    assert_eq!((turn.label.as_str(), turn.by_speaker), ("A", false));
    assert_eq!(turn.text, "Shall we meet at the park?");
}

/// Streams a two-sentence translation in pieces, as Gemini does.
struct StreamingTranslator;

#[async_trait]
impl Translator for StreamingTranslator {
    async fn translate(&self, _: TranslationRequest<'_>) -> Result<String, TranslatorError> {
        Err(TranslatorError::Empty)
    }

    async fn translate_stream(
        &self,
        _: TranslationRequest<'_>,
        pieces: mpsc::Sender<String>,
    ) -> Result<(), TranslatorError> {
        for piece in ["Good morning, every", "one. How are", " you today?"] {
            pieces.send(piece.to_owned()).await.unwrap();
        }
        Ok(())
    }
}

#[tokio::test]
async fn speaks_the_first_sentence_before_the_translation_ends() {
    let output = Arc::new(RecordingOutput::default());
    let config = SpeakerConfig {
        speaker: "ana".to_owned(),
        fallback_language: lang("pt-BR"),
        voice: Some(VoiceId("clone".to_owned())),
    };
    let pipeline = SpeakerPipeline::new(
        config,
        Arc::new(StreamingTranslator),
        Arc::new(FakeTts::default()),
        output.clone(),
        Arc::new(Conversation::default()),
    );
    let (tx, rx) = mpsc::channel(8);
    for event in phrase("Bom dia a todos. Como vocês estão hoje?") {
        tx.send(Ok(event)).await.unwrap();
    }
    drop(tx);
    let (_targets, targets_rx) = watch::channel(vec![lang("en")]);
    pipeline.run(rx, targets_rx).await;
    let spoken: Vec<usize> = output.spoken.lock().unwrap().iter().map(|s| s.2).collect();
    assert_eq!(
        spoken,
        ["Good morning, everyone.".len(), "How are you today?".len()]
    );
    let captions = output.captions.lock().unwrap();
    let last = captions
        .iter()
        .rfind(|c| c.kind == CaptionKind::Translation)
        .unwrap();
    assert_eq!(last.text, "Good morning, everyone. How are you today?");
}
