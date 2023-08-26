use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::Flashcard)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Flashcard {
    pub question: String,
    pub answer: String,
}

#[derive(Queryable, Associations, Selectable)]
#[diesel(table_name = crate::schema::Answer)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[belongs_to(Flashcard, foreign_key = "flashcard_id")]
pub struct Answer {
    pub flashcard_id: i32,
    pub timestamp: NaiveDateTime,
    pub answer_rating: i32,
}
