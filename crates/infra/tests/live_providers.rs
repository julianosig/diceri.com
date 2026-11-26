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
use diceri_infra::speech::{ScribeConfig, ScribeSpeechToText, SonioxConfig, SonioxSpeechToText};
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

/// Streams synthesized Portuguese through `stt` in real time and checks the result.
async fn transcribes_synthesized_portuguese(stt: &dyn SpeechToText, name: &str) {
    let audio = to_stt_rate(&synthesize(PHRASE_PT, &lang("pt-BR")).await);
    let speech_ms = u128::try_from(audio.len() / 16).unwrap();
    let started = Instant::now();
    let mut stream = stt.open(&[lang("pt-BR"), lang("en")]).await.unwrap();
    let frame = usize::try_from(STT_SAMPLE_RATE / 50).unwrap();
    let sender = stream.audio.clone();
    let feeder = tokio::spawn(async move {
        for chunk in audio.chunks(frame) {
            sender.send(chunk.to_vec()).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        // Silence lets endpoint detection close the phrase.
        for _ in 0..150 {
            sender.send(vec![0; frame]).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    });
    drop(stream.audio);
    let mut transcript = String::new();
    let mut detected = None;
    let mut endpoint_at = None;
    while let Some(event) = stream.events.recv().await {
        match event.expect("speech-to-text stream error") {
            TranscriptEvent::Final { text, language } => {
                transcript.push_str(&text);
                detected = detected.or(language);
            }
            TranscriptEvent::Endpoint => {
                endpoint_at.get_or_insert_with(|| started.elapsed());
            }
            TranscriptEvent::Partial(_) => {}
        }
    }
    feeder.await.unwrap();
    let after_speech = endpoint_at.map(|at| at.as_millis().saturating_sub(speech_ms));
    eprintln!(
        "{name}: {transcript:?} language={detected:?} phrase closed {after_speech:?} ms after speech"
    );
    assert!(endpoint_at.is_some(), "expected an endpoint");
    assert_eq!(
        detected.map(|l| l.primary().to_owned()).as_deref(),
        Some("pt")
    );
    assert!(
        transcript.to_lowercase().contains("contrato"),
        "{transcript}"
    );
}

#[tokio::test]
#[ignore = "live: calls ElevenLabs and Soniox"]
async fn soniox_transcribes_synthesized_portuguese() {
    let stt = SonioxSpeechToText::new(SonioxConfig::new(key("DICERI_SONIOX_API_KEY")));
    transcribes_synthesized_portuguese(&stt, "soniox").await;
}

#[tokio::test]
#[ignore = "live: calls ElevenLabs Scribe"]
async fn scribe_transcribes_synthesized_portuguese() {
    let stt = ScribeSpeechToText::new(ScribeConfig::new(key("DICERI_ELEVENLABS_API_KEY")));
    transcribes_synthesized_portuguese(&stt, "scribe").await;
}

#[tokio::test]
#[ignore = "live: needs LiveKit credentials"]
async fn livekit_presence_lists_live_rooms() {
    use diceri_application::ports::MediaPresence as _;
    let secret = key("DICERI_LIVEKIT_API_SECRET");
    let presence = diceri_infra::LiveKitPresence::new(
        &std::env::var("DICERI_LIVEKIT_URL").unwrap(),
        std::env::var("DICERI_LIVEKIT_API_KEY").unwrap(),
        secret,
        reqwest::Client::new(),
    );
    let rooms = presence.live_rooms().await.unwrap();
    eprintln!("live rooms: {rooms:?}");
}
