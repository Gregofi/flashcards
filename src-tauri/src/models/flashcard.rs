use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: Option<i32>,
    pub question: String,
    pub answer: String,
}
