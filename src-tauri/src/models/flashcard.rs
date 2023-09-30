use crate::models::answer::Answer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: Option<i32>,
    pub question: String,
    pub answer: String,
}

impl Flashcard {
    pub fn answer(&self, answer_rating: i32) -> Answer {
        Answer {
            id: None,
            flashcard_id: self.id.expect("The flashcard is missing ID!"),
            timestamp: chrono::Local::now().naive_local(),
            answer_rating,
        }
    }
}
