use chrono::NaiveDateTime;

#[derive(PartialEq, Debug)]
pub struct NewFlashcard {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flashcard {
    pub id: i32,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewAnswer {
    pub flashcard_id: i32,
    pub timestamp: NaiveDateTime,
    pub answer_rating: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
    pub id: i32,
    pub flashcard_id: i32,
    pub timestamp: NaiveDateTime,
    pub answer_rating: i32,
}

impl Flashcard {
    pub fn answer(&self, rating: i32) -> NewAnswer {
        NewAnswer {
            flashcard_id: self.id,
            timestamp: chrono::Local::now().naive_local(),
            answer_rating: rating,
        }
    }
}
