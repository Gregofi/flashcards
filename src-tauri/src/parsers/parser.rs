use anyhow::{Context, Result};
use log::debug;
use tokio::fs::File;
use tokio::io::BufReader;
use walkdir::WalkDir;

use crate::models::flashcard::Flashcard;
use crate::parsers::markdown::read_markdown;

pub async fn parse_folder(folder_path: &str) -> Result<Vec<Flashcard>> {
    let mut cards_vec = vec![];
    for entry in WalkDir::new(folder_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let fname = entry.file_name().to_string_lossy();
        debug!("visiting file {}", fname);
        if fname.ends_with(".md") {
            let path = entry.path();
            let file = File::open(path).await?;
            let reader = BufReader::new(file);
            let cards = read_markdown(reader)
                .await
                .with_context(|| format!("Failed to parse markdown: {}", fname))?;
            let cards: Vec<_> = cards
                .into_iter()
                .map(|c| c.with_path(folder_path.to_string(), fname.to_string()))
                .collect();
            cards_vec.extend(cards);
        } else {
            debug!("skipping file {}", fname);
        }
    }
    Ok(cards_vec)
}
