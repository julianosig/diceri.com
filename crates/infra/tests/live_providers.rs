//! Live tests against the real providers. Ignored by default (they cost money and
//! need keys in `.env`); run with `just test-live`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::print_stderr,
    reason = "live diagnostics"
)]

use std::time::Instant;

use diceri_application::ports::{
    STT_SAMPLE_RATE, SpeechRequest, SpeechToText, TTS_SAMPLE_RATE, TextToSpeech, TranscriptEvent,
    TranslationRequest, Translator, VoiceId,
};
use diceri_domain::LanguageCode;
use diceri_infra::speech::{SonioxConfig, SonioxSpeechToText};
use diceri_infra::synthesis::{ElevenLabsConfig, ElevenLabsTextToSpeech};
use diceri_infra::translation::{GeminiConfig, GeminiTranslator};
use secrecy::SecretString;

const STOCK_VOICE: &str = "EXAVITQu4vr4xnSDxMaL";
const PHRASE_PT: &str = "Amanhã às três da tarde a gente revisa o contrato.";

fn key(name: &str) -> SecretString {
    diceri_infra::install_tls_provider();
    let _ = dotenvy::from_filename("../../.env");
    SecretString::from(std::env::var(name).expect("provider key missing from .env"))
}

fn lang(code: &str) -> LanguageCode {
    code.parse().unwrap()
}

async fn synthesize(text: &str, language: &LanguageCode) -> Vec<i16> {
    let tts = ElevenLabsTextToSpeech::new(
        ElevenLabsConfig::new(key("DICERI_ELEVENLABS_API_KEY")),
        reqwest::Client::new(),
    );
    let voice = VoiceId(STOCK_VOICE.to_owned());
    let started = Instant::now();
    let mut audio = tts
        .synthesize(SpeechRequest {
            text,
            language,
            voice: &voice,
        })
        .await
        .unwrap();
    let mut samples = Vec::new();
    while let Some(chunk) = audio.recv().await {
        if samples.is_empty() {
            eprintln!("tts first audio: {:?}", started.elapsed());
        }
        samples.extend(chunk.unwrap());
    }
    eprintln!("tts total: {:?}", started.elapsed());
    samples
}

/// Nearest-sample 24 kHz → 16 kHz downsampling; good enough for a recognition smoke test.
fn to_stt_rate(samples: &[i16]) -> Vec<i16> {
    assert_eq!((TTS_SAMPLE_RATE, STT_SAMPLE_RATE), (24_000, 16_000));
    (0..samples.len() * 2 / 3)
        .filter_map(|i| samples.get(i * 3 / 2).copied())
        .collect()
}

#[tokio::test]
#[ignore = "live: calls ElevenLabs"]
async fn elevenlabs_streams_portuguese_speech() {
    let samples = synthesize(PHRASE_PT, &lang("pt-BR")).await;
    let min_samples = usize::try_from(TTS_SAMPLE_RATE).unwrap() * 3 / 2;
    assert!(samples.len() > min_samples, "expected over 1.5 s of speech");
}

#[tokio::test]
#[ignore = "live: calls Gemini"]
async fn gemini_translates_portuguese_to_english() {
    let translator = GeminiTranslator::new(
        GeminiConfig::new(key("DICERI_GEMINI_API_KEY")),
        reqwest::Client::new(),
    );
    let started = Instant::now();
    let english = translator
        .translate(TranslationRequest {
            text: PHRASE_PT,
            source: &lang("pt-BR"),
            target: &lang("en"),
            context: &[],
        })
        .await
        .unwrap();
    eprintln!("gemini: {:?} -> {english}", started.elapsed());
    assert!(english.to_lowercase().contains("contract"), "{english}");
}

#[tokio::test]
#[ignore = "live: calls ElevenLabs and Soniox"]
async fn soniox_transcribes_synthesized_portuguese() {
    let audio = to_stt_rate(&synthesize(PHRASE_PT, &lang("pt-BR")).await);
    let stt = SonioxSpeechToText::new(SonioxConfig::new(key("DICERI_SONIOX_API_KEY")));
    let mut stream = stt.open(&lang("pt-BR")).await.unwrap();
    let frame = usize::try_from(STT_SAMPLE_RATE / 50).unwrap();
    let sender = stream.audio.clone();
    let feeder = tokio::spawn(async move {
        for chunk in audio.chunks(frame) {
            sender.send(chunk.to_vec()).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        // One second of silence lets endpoint detection close the phrase.
        for _ in 0..50 {
            sender.send(vec![0; frame]).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    });
    drop(stream.audio);
    let mut transcript = String::new();
    let mut endpoint = false;
    while let Some(event) = stream.events.recv().await {
        match event.expect("speech-to-text stream error") {
            TranscriptEvent::Final(text) => transcript.push_str(&text),
            TranscriptEvent::Endpoint => endpoint = true,
            TranscriptEvent::Partial(_) => {}
        }
    }
    feeder.await.unwrap();
    eprintln!("soniox: {transcript}");
    assert!(endpoint, "expected an endpoint marker");
    assert!(
        transcript.to_lowercase().contains("contrato"),
        "{transcript}"
    );
}
