//! Whether a recording holds actual speech: the recognizer must hear words in it.
//! Noise, music, breathing or silence yield no text, so no voice is cloned from them.

use std::time::Duration;

use tokio::time::timeout;

use crate::ports::{SpeechStream, SpeechToText, SpeechToTextError, TranscriptEvent};

/// 100 ms of 16 kHz audio per message.
const CHUNK_SAMPLES: usize = 1_600;
/// A 10 s sample is recognized in a few seconds; past this, give up.
const CHECK_TIMEOUT: Duration = Duration::from_secs(20);
/// Letters the recognizer must hear, in any script, for a recording to count as speech.
pub(crate) const MIN_LETTERS: usize = 8;

/// Letters recognized in `samples` (16 kHz mono PCM).
pub(crate) async fn letters_heard(
    stt: &dyn SpeechToText,
    samples: &[i16],
) -> Result<usize, SpeechToTextError> {
    let SpeechStream { audio, mut events } = stt.open(&[]).await?;
    let feed = async move {
        for chunk in samples.chunks(CHUNK_SAMPLES) {
            if audio.send(chunk.to_vec()).await.is_err() {
                break;
            }
        }
        // Dropping the sender ends the stream; the recognizer then finalizes.
    };
    let collect = async {
        let mut letters = 0;
        while let Some(event) = events.recv().await {
            if let TranscriptEvent::Final { text, .. } = event? {
                letters += text.chars().filter(|c| c.is_alphabetic()).count();
            }
        }
        Ok(letters)
    };
    let ((), letters) = timeout(CHECK_TIMEOUT, async { tokio::join!(feed, collect) })
        .await
        .map_err(|_| SpeechToTextError::Provider("speech check timed out".to_owned()))?;
    letters
}

#[cfg(test)]
pub(crate) mod fake {
    use async_trait::async_trait;
    use diceri_domain::LanguageCode;
    use tokio::sync::mpsc;

    use crate::ports::{SpeechStream, SpeechToText, SpeechToTextError, TranscriptEvent};

    /// Hears `text` once all audio has arrived, like a recognizer finalizing.
    pub(crate) struct FakeRecognizer(pub &'static str);

    #[async_trait]
    impl SpeechToText for FakeRecognizer {
        async fn open(&self, _: &[LanguageCode]) -> Result<SpeechStream, SpeechToTextError> {
            let (audio, mut incoming) = mpsc::channel::<Vec<i16>>(8);
            let (events, receiver) = mpsc::channel(8);
            let text = self.0.to_owned();
            tokio::spawn(async move {
                while incoming.recv().await.is_some() {}
                let _ = events
                    .send(Ok(TranscriptEvent::Final {
                        text,
                        language: None,
                    }))
                    .await;
            });
            Ok(SpeechStream {
                audio,
                events: receiver,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fake::FakeRecognizer;
    use super::*;

    #[tokio::test]
    async fn counts_letters_in_any_script() {
        let samples = vec![0; 16_000];
        assert_eq!(
            letters_heard(&FakeRecognizer("Hello there, 123!"), &samples)
                .await
                .unwrap(),
            10
        );
        assert_eq!(
            letters_heard(&FakeRecognizer("こんにちは"), &samples)
                .await
                .unwrap(),
            5
        );
        assert_eq!(
            letters_heard(&FakeRecognizer(""), &samples).await.unwrap(),
            0
        );
    }
}
