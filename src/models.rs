use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::Flashcard)]
pub struct NewFlashcard {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Queryable, Selectable, AsChangeset, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::Flashcard)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Flashcard {
    pub id: i32,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Queryable, Selectable, AsChangeset, Insertable, PartialEq, Associations)]
#[diesel(table_name = crate::schema::Answer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Flashcard, foreign_key = flashcard_id))]
pub struct NewAnswer {
    pub flashcard_id: i32,
    pub timestamp: NaiveDateTime,
    pub answer_rating: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, AsChangeset, Insertable, PartialEq, Associations)]
#[diesel(table_name = crate::schema::Answer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Flashcard, foreign_key = flashcard_id))]
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
