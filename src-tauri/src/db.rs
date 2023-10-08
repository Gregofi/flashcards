use anyhow::Result;
use sqlx::prelude::*;
use sqlx::sqlite::{SqlitePool, SqliteRow};

use crate::models::prelude::{Answer, Flashcard};

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn persist_answer(&self, answer: Answer) -> Result<()> {
        sqlx::query(
            "
INSERT INTO answer (flashcard_id, timestamp, answer_rating)
VALUES (?, ?, ?)
            ",
        )
        .bind(answer.flashcard_id)
        .bind(answer.timestamp.to_string())
        .bind(answer.answer_rating)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
    }

    pub async fn add_card(&self, card: Flashcard) -> Result<()> {
        sqlx::query(
            "
INSERT INTO flashcard (question, answer, folder, path)
VALUES (?, ?, ?, ?)
            ",
        )
        .bind(card.question)
        .bind(card.answer)
        .bind(card.folder)
        .bind(card.path)
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
    }

    pub async fn update_card(&self, card: &Flashcard) -> Result<()> {
        match card.id {
            None => Err(anyhow::anyhow!("The card is missing ID!")),
            Some(_) => sqlx::query(
                "
UPDATE flashcard
SET question = ?, answer = ?
WHERE id = ?
            ",
            )
            .bind(card.question.as_str())
            .bind(card.answer.as_str())
            .bind(card.id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.into()),
        }
    }

    pub async fn get_card(&self, id: i32) -> Result<Flashcard> {
        sqlx::query("SELECT * FROM flashcard WHERE id = ?")
            .bind(id)
            .map(|row: SqliteRow| Flashcard {
                id: Some(row.get(0)),
                question: row.get(1),
                answer: row.get(2),
                folder: row.get(3),
                path: row.get(4),
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    /// Pulls the cards from the database. Caches the result, pulls only once
    /// for multiple calls.
    pub async fn get_cards(&self) -> Result<Vec<Flashcard>> {
        sqlx::query("SELECT * FROM flashcard")
            .map(|row: SqliteRow| Flashcard {
                id: Some(row.get(0)),
                question: row.get(1),
                answer: row.get(2),
                folder: row.get(3),
                path: row.get(4),
            })
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_answers(&self, card: &Flashcard) -> Result<Vec<Answer>> {
        sqlx::query(
            "
SELECT * FROM answer
WHERE flashcard_id = ?
            ",
        )
        .bind(card.id)
        .map(|row: SqliteRow| Answer {
            id: Some(row.get(0)),
            flashcard_id: row.get(1),
            timestamp: row.get(2),
            answer_rating: row.get(3),
        })
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
    }
}
