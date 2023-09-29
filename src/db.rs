use anyhow::Result;
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;

use crate::models::Answer;
use crate::models::Flashcard;
use crate::models::NewAnswer;
use crate::models::NewFlashcard;
use crate::schema::Answer as schemaAnswer;
use crate::schema::Flashcard as schemaFlashcard;

pub struct Db {
    connection: SqliteConnection,
    cards: Option<Vec<Flashcard>>,
}

impl Db {
    pub fn new(connection: SqliteConnection) -> Self {
        Db {
            connection,
            cards: None,
        }
    }

    pub fn persist_answer(&mut self, answer: NewAnswer) -> Result<()> {
        diesel::insert_into(schemaAnswer::table)
            .values(&answer)
            .execute(&mut self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn add_card(&mut self, card: NewFlashcard) -> Result<()> {
        self.cards = None;
        diesel::insert_into(schemaFlashcard::table)
            .values(&card)
            .execute(&mut self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub fn update_card(&mut self, card: &Flashcard) -> Result<()> {
        self.cards = None;
        use crate::schema::Flashcard::dsl::*;
        diesel::update(schemaFlashcard::table.filter(id.eq(card.id)))
            .set(card)
            .execute(&mut self.connection)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    /// Pulls the cards from the database. Caches the result, pulls only once
    /// for multiple calls.
    pub fn get_cards(&mut self) -> Result<Vec<Flashcard>> {
        if self.cards.is_none() {
            let cards = schemaFlashcard::table.load::<Flashcard>(&mut self.connection);
            match cards {
                Ok(cards) => self.cards = Some(cards),
                Err(e) => return Err(e.into()),
            }
        }
        // TODO: We don't want to clone here.
        Ok(self.cards.clone().unwrap())
    }

    pub fn get_answers(&mut self, card: &Flashcard) -> Result<Vec<Answer>> {
        use crate::schema::Answer::dsl::flashcard_id;
        schemaAnswer::table
            .filter(flashcard_id.eq(card.id))
            .load::<Answer>(&mut self.connection)
            .map_err(|e| e.into())
    }
}
