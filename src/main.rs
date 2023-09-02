use clap::{Command, Parser};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use log::{debug, error, info, warn};
use walkdir::WalkDir;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;

mod markdown;
mod models;
mod schema;
mod spa;
mod sync;

use crate::models::Flashcard;

pub async fn estabilish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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

    let db = &mut estabilish_connection().await;

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
        }
        Some(_) => unreachable!(),
        None => todo!(),
    }

    Ok(())
}
