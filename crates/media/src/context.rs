use std::collections::VecDeque;

/// The last few phrases of a speaker, given to the translator for coherence.
#[derive(Debug)]
pub(crate) struct RollingContext {
    capacity: usize,
    phrases: VecDeque<String>,
}

impl RollingContext {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            phrases: VecDeque::with_capacity(capacity),
        }
    }

    pub(crate) fn snapshot(&self) -> Vec<String> {
        self.phrases.iter().cloned().collect()
    }

    pub(crate) fn push(&mut self, phrase: String) {
        if self.phrases.len() == self.capacity {
            self.phrases.pop_front();
        }
        self.phrases.push_back(phrase);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_the_most_recent_phrases() {
        let mut context = RollingContext::new(2);
        for phrase in ["a", "b", "c"] {
            context.push(phrase.to_owned());
        }
        assert_eq!(context.snapshot(), vec!["b".to_owned(), "c".to_owned()]);
    }
}
