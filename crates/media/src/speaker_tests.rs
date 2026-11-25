use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diceri_application::ports::{
    SpeechAudio, SpeechRequest, TextToSpeech, TextToSpeechError, TranscriptEvent,
    TranslationRequest, Translator, TranslatorError, VoiceId,
};
use diceri_domain::LanguageCode;
use tokio::sync::{mpsc, watch};

use super::{SpeakerConfig, SpeakerPipeline};
use crate::{Caption, CaptionKind, PipelineOutput};

/// Translates by tagging the text with the target language: "[en] Olá".
struct TaggingTranslator {
    contexts: Mutex<Vec<Vec<String>>>,
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

/// Returns one chunk whose length is the text length.
struct FakeTts;

#[async_trait]
impl TextToSpeech for FakeTts {
    async fn synthesize(
        &self,
        request: SpeechRequest<'_>,
    ) -> Result<SpeechAudio, TextToSpeechError> {
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
}

impl Harness {
    async fn run(events: Vec<TranscriptEvent>, targets: &[&str]) -> Self {
        let output = Arc::new(RecordingOutput::default());
        let translator = Arc::new(TaggingTranslator {
            contexts: Mutex::default(),
        });
        let config = SpeakerConfig {
            speaker: "ana".to_owned(),
            language: lang("pt-BR"),
            voice: VoiceId("voice".to_owned()),
        };
        let pipeline = SpeakerPipeline::new(
            config,
            translator.clone(),
            Arc::new(FakeTts),
            output.clone(),
        );
        let (tx, rx) = mpsc::channel(64);
        for event in events {
            tx.send(Ok(event)).await.unwrap();
        }
        drop(tx);
        let (_targets_tx, targets_rx) = watch::channel(targets.iter().map(|t| lang(t)).collect());
        pipeline.run(rx, targets_rx).await;
        Self { output, translator }
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
    assert_eq!(
        contexts.last().unwrap(),
        &vec!["um".to_owned(), "dois".to_owned()]
    );
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
