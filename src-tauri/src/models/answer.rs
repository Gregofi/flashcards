use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Answer {
    pub id: Option<i32>,
    pub flashcard_id: i32,
    pub timestamp: NaiveDateTime,
    pub answer_rating: i32,
}
