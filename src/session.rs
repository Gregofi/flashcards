use crate::models::Flashcard;

/// One session of flashcard learning.
pub struct Session {
    /// TODO: Aim for these guys to just be references,
    /// however since the session are created only a few times
    /// it shouldn't be a big problem for now.
    cards: Vec<Flashcard>,
    current_card: usize,
}

impl Session {
    pub fn new(cards: Vec<Flashcard>) -> Self {
        Self {
            cards,
            current_card: 0,
        }
    }

    pub fn advance_questions(&mut self) {
        self.current_card += 1;
    }

    #[allow(dead_code)]
    pub fn next_question(&mut self) -> Option<&Flashcard> {
        if self.current_card < self.cards.len() {
            let card = &self.cards[self.current_card];
            self.current_card += 1;
            Some(card)
        } else {
            None
        }
    }

    pub fn current_question(&mut self) -> Option<&Flashcard> {
        if self.current_card < self.cards.len() {
            Some(&self.cards[self.current_card])
        } else {
            None
        }
    }
}
