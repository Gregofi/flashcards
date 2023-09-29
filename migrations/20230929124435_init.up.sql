CREATE TABLE flashcard (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question TEXT NOT NULL,
    answer TEXT NOT NULL
);

CREATE TABLE answer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    flashcard_id INTEGER NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    answer_rating INTEGER CHECK(answer_rating >= 0 AND answer_rating <= 100) NOT NULL,
    FOREIGN KEY (flashcard_id) REFERENCES Flashcard(id)
);
