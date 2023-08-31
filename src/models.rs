use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::Flashcard)]
pub struct NewFlashcard {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::Flashcard)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Flashcard {
    pub id: i32,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Queryable, Associations, Selectable)]
#[diesel(table_name = crate::schema::Answer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[belongs_to(Flashcard, foreign_key = "flashcard_id")]
pub struct Answer {
    pub id: i32,
    pub flashcard_id: i32,
    pub timestamp: NaiveDateTime,
    pub answer_rating: i32,
}
