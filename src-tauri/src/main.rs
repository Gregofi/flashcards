// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use dotenvy::dotenv;
use models::flashcard::Flashcard;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::env;
use std::fs;

mod models;
mod parsers;
mod repetition_algs;
mod sync;
mod db;

use db::Db;
use log::debug;
use crate::models::answer::Answer;
use crate::repetition_algs::prelude::*;

struct AppState {
    db: Db,
}

pub async fn estabilish_connection() -> sqlx::Result<SqlitePool> {
    // TODO: Windows
    let home = env::var("HOME").expect("Could not find home directory");
    let app_path = env::var("DATABASE_URL").unwrap_or(format!("{}/.flashcards", home));
    // TODO: We deliberately ignore this since the folder will often exists
    let _ = fs::create_dir_all(&app_path);
    let database_url = format!("{}/flashcards.db", app_path);
    debug!("Opening database at {}", database_url);

    if !sqlx::Sqlite::database_exists(&database_url).await? {
        debug!("Database does not exist, creating");
        sqlx::Sqlite::create_database(&database_url).await?;
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    sqlx::migrate!().run(&mut pool.acquire().await?).await?;
    Ok(pool)
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

#[tauri::command]
async fn sync_flashcards(folder: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let new_cards = parsers::parser::parse_folder(&folder).await
        .map_err(|e| e.to_string())?;
    let old_cards = state.db.get_cards().await
        .map_err(|e| e.to_string())?;
    let synced_cards = sync::sync(&old_cards, new_cards).await;
    for card in synced_cards {
        match card {
            sync::CardType::New(card) => {
                state.db.add_card(card).await
                    .map_err(|e| e.to_string())?;
            }
            sync::CardType::Old(card) => {
                state.db.update_card(&card).await
                    .map_err(|e| e.to_string())?;
            }
        }
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
        .setup(|_| {Ok(())})
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_all_cards, get_cards_to_review, answer_question, sync_flashcards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
