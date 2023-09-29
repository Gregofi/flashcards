use anyhow::{Result, Context};
use chrono::NaiveDateTime;
use sqlx::sqlite::SqliteRow;

use crate::models::Answer;
use crate::models::Flashcard;
use crate::models::NewAnswer;
use crate::models::NewFlashcard;
use sqlx::prelude::*;
use sqlx::sqlite::SqliteConnection;

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

    pub async fn persist_answer(&mut self, answer: NewAnswer) -> Result<()> {
        sqlx::query(
            "
INSERT INTO answer (flashcard_id, timestamp, answer_rating)
VALUES (?, ?, ?)
            ")
            .bind(answer.flashcard_id)
            .bind(answer.timestamp.to_string())
            .bind(answer.answer_rating)
            .execute(&mut self.connection).await
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub async fn add_card(&mut self, card: NewFlashcard) -> Result<()> {
        sqlx::query(
            "
INSERT INTO flashcard (question, answer)
VALUES (?, ?)
            ")
            .bind(card.question)
            .bind(card.answer)
            .execute(&mut self.connection).await
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub async fn update_card(&mut self, card: &Flashcard) -> Result<()> {
        sqlx::query(
            "
UPDATE flashcard
SET question = ?, answer = ?
WHERE id = ?
            ")
            .bind(card.question.as_str())
            .bind(card.answer.as_str())
            .bind(card.id)
            .execute(&mut self.connection).await
            .map(|_| ())
            .map_err(|e| e.into())
    }

    /// Pulls the cards from the database. Caches the result, pulls only once
    /// for multiple calls.
    pub async fn get_cards(&mut self) -> Result<Vec<Flashcard>> {
        sqlx::query( "SELECT * FROM flashcard")
            .map(|row: SqliteRow| {
                Flashcard {
                    id: row.get(0),
                    question: row.get(1),
                    answer: row.get(2),
                }
            })
            .fetch_all(&mut self.connection)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_answers(&mut self, card: &Flashcard) -> Result<Vec<Answer>> {
        sqlx::query(
            "
SELECT * FROM answer
WHERE flashcard_id = ?
            ")
            .bind(card.id)
            .map(|row: SqliteRow| {
                Answer {
                    id: row.get(0),
                    flashcard_id: row.get(1),
                    timestamp: row.get(2),
                    answer_rating: row.get(3),
                }
            })
            .fetch_all(&mut self.connection)
            .await
            .map_err(|e| e.into())
    }
}
