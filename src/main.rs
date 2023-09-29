use clap::{Command, Parser};
use dotenvy::dotenv;
use log::{debug, info};
use walkdir::WalkDir;
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;

use crate::models::Flashcard;
use crate::tui::tui;

mod db;
mod markdown;
mod models;
mod session;
mod spa;
mod sync;
mod tui;

pub async fn estabilish_connection() -> sqlx::Result<sqlx::SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::connect(&database_url).await
}

#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let _db = estabilish_connection().await.unwrap();
    let mut db = db::Db::new(_db);

    let matches = Command::new("flashy")
        .about("Flashcard application")
        .subcommand_required(false)
        .author("Filip Gregor")
        .subcommand(
            Command::new("markdown")
                .short_flag('m')
                .long_flag("markdown")
                .about("Parse markdown flashcards from folder and sync them with database")
                .arg(
                    clap::Arg::new("folder")
                        .help("Folder with markdown files")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("markdown", md_matches)) => {
            let mut cards_vec = vec![];
            let folder: &String = md_matches.get_one("folder").expect("path is required");
            info!("inspecting folder {}", folder);
            for entry in WalkDir::new(folder)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let fname = entry.file_name().to_string_lossy();
                debug!("visiting file {}", fname);
                if fname.ends_with(".md") {
                    let path = entry.path();
                    let file = File::open(path)?;
                    let reader = BufReader::new(file);
                    let cards = markdown::read_markdown(reader).await?;
                    cards_vec.extend(cards);
                } else {
                    debug!("skipping file {}", fname);
                }
            }
            cards_vec.iter().for_each(|card| {
                info!("card");
                info!("question: {}", card.question);
                info!("answer: {}", card.answer);
            });
            let cards: Vec<Flashcard> = db
                .get_cards()
                .await
                .unwrap()
                .into_iter()
                .collect();
            let synced = sync::sync(&cards, cards_vec).await;
            for card in synced {
                match card {
                    sync::CardType::New(c) => {
                        db.add_card(c).await.unwrap();
                    }
                    sync::CardType::Old(c) => {
                        db.update_card(&c).await.unwrap();
                    }
                }
            }
        }
        Some(_) => unreachable!(),
        None => {
            tui(db).await.unwrap();
        }
    }

    Ok(())
}
