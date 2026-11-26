use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, PoisonError};

use diceri_application::ports::ContextTurn;

/// Turns of the call the translator sees: enough to follow a reply or a reference.
const TURNS: usize = 8;

/// The recent conversation of one call, shared by every speaker's pipeline so each
/// phrase is translated knowing what the others said. Participants appear only under
/// neutral labels (A, B, …): their names and ids never reach the translator.
#[derive(Debug, Default)]
pub struct Conversation {
    inner: Mutex<Turns>,
}

#[derive(Debug, Default)]
struct Turns {
    /// (speaker id, what they said), oldest first.
    recent: VecDeque<(String, String)>,
    labels: HashMap<String, String>,
}

impl Conversation {
    /// The conversation so far, as `speaker`'s phrase about to be translated sees it.
    #[must_use]
    pub fn context_for(&self, speaker: &str) -> Vec<ContextTurn> {
        let turns = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
        turns
            .recent
            .iter()
            .map(|(id, text)| ContextTurn {
                label: turns.labels.get(id).cloned().unwrap_or_default(),
                by_speaker: id == speaker,
                text: text.clone(),
            })
            .collect()
    }

    /// Adds what `speaker` just said.
    pub fn record(&self, speaker: &str, text: &str) {
        let mut turns = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
        let next = turns.labels.len();
        turns
            .labels
            .entry(speaker.to_owned())
            .or_insert_with(|| label(next));
        if turns.recent.len() == TURNS {
            turns.recent.pop_front();
        }
        turns
            .recent
            .push_back((speaker.to_owned(), text.to_owned()));
    }
}

/// A, B, … Z, then A2, B2, …
fn label(index: usize) -> String {
    let letter = char::from(b'A' + u8::try_from(index % 26).unwrap_or(0));
    match index / 26 {
        0 => letter.to_string(),
        round => format!("{letter}{}", round + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_people_in_order_and_marks_the_speaker() {
        let conversation = Conversation::default();
        conversation.record("p-ana", "Bom dia!");
        conversation.record("p-bob", "Good morning.");
        let context = conversation.context_for("p-bob");
        assert_eq!(context.len(), 2);
        assert_eq!(
            (context[0].label.as_str(), context[0].by_speaker),
            ("A", false)
        );
        assert_eq!(
            (context[1].label.as_str(), context[1].by_speaker),
            ("B", true)
        );
        assert_eq!(context[1].text, "Good morning.");
    }

    #[test]
    fn keeps_only_the_most_recent_turns() {
        let conversation = Conversation::default();
        for n in 0..TURNS + 3 {
            conversation.record("p-ana", &n.to_string());
        }
        let context = conversation.context_for("p-ana");
        assert_eq!(context.len(), TURNS);
        assert_eq!(context[0].text, "3");
    }

    #[test]
    fn labels_never_run_out() {
        assert_eq!(label(0), "A");
        assert_eq!(label(25), "Z");
        assert_eq!(label(26), "A2");
    }
}
