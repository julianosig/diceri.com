//! Cuts a translation arriving in pieces into parts that can be spoken on their own, so
//! speech starts with the first sentence instead of waiting for the whole phrase.

/// Ends of a sentence, in the scripts the call supports.
const SENTENCE_ENDS: [char; 9] = ['.', '!', '?', '…', '。', '！', '？', '؟', '।'];
/// Ends of a clause: used only to split a sentence that runs long.
const CLAUSE_ENDS: [char; 7] = [',', ';', ':', '，', '、', '；', '،'];
/// Full-width punctuation is not followed by a space in Chinese and Japanese.
const FULL_WIDTH: [char; 5] = ['。', '！', '？', '，', '、'];
/// Fewer characters than this wait for more, so speech is not cut into choppy bits.
const MIN_CHARS: usize = 12;
/// A sentence this long is spoken up to its next comma instead of waiting for its end.
const LONG_CLAUSE_CHARS: usize = 80;

#[derive(Debug, Default)]
pub(crate) struct Clauses {
    pending: String,
}

impl Clauses {
    /// Adds a piece of translation; returns the parts now complete enough to speak.
    pub(crate) fn push(&mut self, piece: &str) -> Vec<String> {
        self.pending.push_str(piece);
        let mut ready = Vec::new();
        while let Some(end) = self.boundary() {
            let part: String = self.pending.drain(..end).collect();
            let part = part.trim();
            if !part.is_empty() {
                ready.push(part.to_owned());
            }
        }
        ready
    }

    /// Whatever is left once the translation is complete.
    pub(crate) fn finish(&mut self) -> Option<String> {
        let rest = std::mem::take(&mut self.pending);
        let rest = rest.trim();
        (!rest.is_empty()).then(|| rest.to_owned())
    }

    /// Byte index just past the first place to cut, if any. A mark only counts once the
    /// next character has arrived and confirms it ends something ("3.5" does not).
    fn boundary(&self) -> Option<usize> {
        let mut chars = self.pending.char_indices().peekable();
        let mut count = 0;
        while let Some((at, mark)) = chars.next() {
            count += 1;
            let &(_, next) = chars.peek()?;
            let separated = next.is_whitespace() || FULL_WIDTH.contains(&mark);
            let sentence = SENTENCE_ENDS.contains(&mark) && count >= MIN_CHARS;
            let long_clause = CLAUSE_ENDS.contains(&mark) && count >= LONG_CLAUSE_CHARS;
            if separated && (sentence || long_clause) {
                return Some(at + mark.len_utf8());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(pieces: &[&str]) -> Vec<String> {
        let mut clauses = Clauses::default();
        let mut parts: Vec<String> = pieces
            .iter()
            .flat_map(|piece| clauses.push(piece))
            .collect();
        parts.extend(clauses.finish());
        parts
    }

    #[test]
    fn speaks_each_sentence_as_soon_as_it_is_complete() {
        let mut clauses = Clauses::default();
        assert_eq!(clauses.push("Good morning, every"), Vec::<String>::new());
        assert!(
            clauses.push("one.").is_empty(),
            "waits to see what follows the period"
        );
        assert_eq!(clauses.push(" How are"), vec!["Good morning, everyone."]);
        assert_eq!(clauses.finish().as_deref(), Some("How are"));
    }

    #[test]
    fn does_not_cut_numbers_or_tiny_sentences() {
        assert_eq!(feed(&["It costs 3.5 euros."]), vec!["It costs 3.5 euros."]);
        assert_eq!(
            feed(&["Yes. ", "Of course, why not?"]),
            vec!["Yes. Of course, why not?"]
        );
    }

    #[test]
    fn splits_a_long_sentence_at_a_comma() {
        let long = "When we finish the review of the contract with the lawyers next week, we sign";
        let parts = feed(&[long]);
        assert_eq!(
            parts.len(),
            1,
            "a comma before the threshold does not split"
        );
        let longer = format!("{long} it, and then we celebrate together");
        let parts = feed(&[&longer]);
        assert_eq!(parts[0], format!("{long} it,"));
    }

    #[test]
    fn handles_full_width_punctuation() {
        assert_eq!(
            feed(&["今日はとてもいい天気ですね。明日も晴れるでしょう"]),
            vec!["今日はとてもいい天気ですね。", "明日も晴れるでしょう"]
        );
    }
}
