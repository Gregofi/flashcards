use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: Option<i32>,
    pub question: String,
    pub answer: String,
    // The root folder from where the flashcard was loaded.
    // Is primarily used for resolving markdown images.
    pub folder: Option<String>,
    // The **absolute** path to the flashcard.
    pub path: Option<String>,
}

impl Flashcard {
    pub fn with_path(mut self, folder: String, path: String) -> Self {
        self.folder = Some(folder);
        self.path = Some(path);
        self
    }
}
