//! Generates the spoken WAV fixtures used as fake microphones in end-to-end tests.
//! Run with `just fixtures` (calls ElevenLabs; needs `DICERI_ELEVENLABS_API_KEY`).
#![allow(clippy::print_stdout, reason = "command-line tool")]

use std::path::Path;

use anyhow::Context as _;
use diceri_application::ports::{SpeechRequest, TTS_SAMPLE_RATE, TextToSpeech as _, VoiceId};
use diceri_infra::synthesis::{ElevenLabsConfig, ElevenLabsTextToSpeech};
use secrecy::SecretString;

const OUTPUT_DIR: &str = "apps/web/tests/e2e/fixtures";
/// Silence after each phrase so endpoint detection closes it.
const TRAILING_SILENCE_MS: u32 = 2_000;

/// (file stem / language, stock voice, phrase). Keep in sync with translation.e2e.ts.
const FIXTURES: &[(&str, &str, &str)] = &[
    (
        "pt-BR",
        "EXAVITQu4vr4xnSDxMaL",
        "Amanhã às três da tarde a gente revisa o contrato.",
    ),
    (
        "en",
        "CwhRBWXzGAHq8TQ4Fs17",
        "The weather is beautiful today, let's walk in the park.",
    ),
    (
        "es",
        "FGY2WhTYpPnrIDTdsKH5",
        "Mi hermano compró una bicicleta roja ayer.",
    ),
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    diceri_infra::install_tls_provider();
    let key = std::env::var("DICERI_ELEVENLABS_API_KEY").context("DICERI_ELEVENLABS_API_KEY")?;
    let tts = ElevenLabsTextToSpeech::new(
        ElevenLabsConfig::new(SecretString::from(key)),
        reqwest::Client::new(),
    );
    for (language, voice, text) in FIXTURES {
        let request = SpeechRequest {
            text,
            language: &language.parse()?,
            voice: &VoiceId((*voice).to_owned()),
        };
        let mut audio = tts.synthesize(request).await?;
        let mut samples = Vec::new();
        while let Some(chunk) = audio.recv().await {
            samples.extend(chunk?);
        }
        let silence = usize::try_from(TTS_SAMPLE_RATE * TRAILING_SILENCE_MS / 1_000)?;
        samples.resize(samples.len() + silence, 0);
        let path = Path::new(OUTPUT_DIR).join(format!("{language}.wav"));
        std::fs::write(&path, wav(&samples, TTS_SAMPLE_RATE)?)?;
        println!("wrote {} ({} samples)", path.display(), samples.len());
    }
    Ok(())
}

/// 16-bit mono PCM WAV file bytes.
fn wav(samples: &[i16], sample_rate: u32) -> anyhow::Result<Vec<u8>> {
    let data_len = u32::try_from(samples.len() * 2)?;
    let mut bytes = Vec::with_capacity(44 + samples.len() * 2);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes()); // fmt chunk size
    bytes.extend_from_slice(&1_u16.to_le_bytes()); // PCM
    bytes.extend_from_slice(&1_u16.to_le_bytes()); // mono
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    bytes.extend_from_slice(&2_u16.to_le_bytes()); // block align
    bytes.extend_from_slice(&16_u16.to_le_bytes()); // bits per sample
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());
    bytes.extend(samples.iter().flat_map(|s| s.to_le_bytes()));
    Ok(bytes)
}
