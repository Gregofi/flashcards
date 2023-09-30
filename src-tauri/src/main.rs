// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use dotenvy::dotenv;
use models::flashcard::Flashcard;
use sqlx::sqlite::{SqliteConnection, SqlitePoolOptions};
use sqlx::{Connection, SqlitePool};
use std::cell::RefCell;
use std::env;
use std::sync::Arc;

mod models;
mod parsers;
mod repetition_algs;
mod sync;
mod db;

use db::Db;
use crate::models::answer::Answer;
use crate::repetition_algs::prelude::*;

struct AppState {
    db: Db,
}

pub async fn estabilish_connection() -> sqlx::Result<SqlitePool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

#[tauri::command]
async fn get_all_cards(state: tauri::State<'_, AppState>) -> Result<Vec<Flashcard>, String> {
    state.db.get_cards().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_cards_to_review(state: tauri::State<'_, AppState>) -> Result<Vec<Flashcard>, String> {
    let ra = NaiveExponentialRA::new(50, 128);
    let cards = state.db.get_cards().await
        .map_err(|e| e.to_string())?;
    let mut result = vec![];
    // We would like filter but async closures and such...
    for card in cards {
        let answers = state.db.get_answers(&card).await
            .map_err(|e| e.to_string())?;
        if ra.repeat_question(&answers) {
            result.push(card);
        }
    }
    Ok(result)
}

#[tauri::command]
async fn answer_question(flashcard_id: i32, answer_rating: i32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    println!("Answering question {} with rating {}", flashcard_id, answer_rating);
    if (answer_rating < 0) || (answer_rating > 100) {
        return Err("Answer rating must be between 0 and 100".to_string());
    }
    let answer = Answer {
        id: None,
        flashcard_id,
        timestamp: chrono::Local::now().naive_local(),
        answer_rating,
    };
    let x = state.db.persist_answer(answer).await.map_err(|e| e.to_string());
    println!("Persisting answer");
    if x.is_err() {
        println!("Error: {}", x.unwrap_err());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let pool = estabilish_connection().await.expect("Failed to connect to database");
    let db = Db::new(pool);
    let state = AppState { db };
    tauri::Builder::default()
        .setup(|app| {Ok(())})
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_all_cards, get_cards_to_review, answer_question])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}