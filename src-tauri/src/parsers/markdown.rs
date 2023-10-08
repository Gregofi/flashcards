use tokio::io::{AsyncBufRead, AsyncBufReadExt};

use crate::models::flashcard::Flashcard;
use std::io::Result;

pub async fn read_markdown<R: AsyncBufRead + Unpin>(reader: R) -> Result<Vec<Flashcard>> {
    let mut flashcards = vec![];
    let mut line_it = reader.lines();
    loop {
        let line = line_it.next_line().await?;
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        // Try to parse the format:
        // ```
        // question text ... [#tags ...] #flashcard [#tags ...]
        // answer text (possibly over multiple lines)
        // ---
        // ```
        let mut it = line.split('#');
        let question = it.next().unwrap().trim().to_string();
        let tags: Vec<String> = it.map(|s| s.to_string()).collect();
        if !tags.into_iter().any(|s| s.starts_with("flashcard")) {
            continue;
        }

        let mut answer = String::new();
        while let Some(line) = line_it.next_line().await? {
            // TODO: Something more generic for MD lines.
            if line.starts_with("---") || line.starts_with("- - -") {
                flashcards.push(Flashcard {
                    id: None,
                    question,
                    answer,
                });
                break;
            }
            answer.push_str(line.as_str());
        }
    }
    Ok(flashcards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_read_empty_markdown() {
        let markdown = "";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 0);
    }

    #[tokio::test]
    async fn test_read_markdown_one_question() {
        let markdown = "question #flashcard\nanswer\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 1);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer");
    }

    #[tokio::test]
    async fn test_multiple_questions() {
        let markdown = "question #flashcard\nanswer\n---\nquestion2 #flashcard\nanswer2\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 2);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer");
        assert_eq!(flashcards[1].question, "question2");
        assert_eq!(flashcards[1].answer, "answer2");
    }

    #[tokio::test]
    async fn test_mix_questions_text() {
        let markdown = "question #flashcard\nanswer\n---\nSome random text\nquestion2 #flashcard\nanswer2\n---\nMore random text\nquestion3 #flashcard\nanswer3\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 3);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer");
        assert_eq!(flashcards[1].question, "question2");
        assert_eq!(flashcards[1].answer, "answer2");
        assert_eq!(flashcards[2].question, "question3");
        assert_eq!(flashcards[2].answer, "answer3");
    }

    #[tokio::test]
    async fn test_no_questions() {
        let markdown = "Some random text\nSome more random text\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 0);
    }

    #[tokio::test]
    async fn test_unterminated_question() {
        let markdown = "question #flashcard\nanswer\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 0);
    }
}
