use tokio::io::{AsyncBufRead, AsyncBufReadExt, Lines};

use crate::models::flashcard::Flashcard;
use anyhow::{bail, Result};

pub async fn read_until_hr<R: AsyncBufRead + Unpin>(lines: &mut Lines<R>) -> Result<String> {
    let mut text = String::new();
    while let Some(line) = lines.next_line().await? {
        // TODO: Something more generic for MD lines.
        if line.starts_with("---")
            || line.starts_with("- - -")
            || line.starts_with("***")
            || line.starts_with("* * *")
        {
            break;
        }
        text.push_str(line.trim());
        text.push('\n');
    }
    Ok(text)
}

/**
 * Parse a long question, expecting a following format:
 * #flashcard [#tags ...] (This line should already be read and tags passed!)
 * question text (possibly over multiple lines)
 * - - - (A line in markdown)
 * answer text (possibly over multiple lines)
 * - - - (A line in markdown)
 */
pub async fn long_question<R: AsyncBufRead + Unpin>(
    reader: &mut Lines<R>,
    _tags: Vec<String>,
) -> Result<Flashcard> {
    let question = read_until_hr(reader).await?;
    let answer = read_until_hr(reader).await?;

    if question.is_empty() || answer.is_empty() {
        bail!("Card cannot have an empty question or answer text");
    }

    Ok(Flashcard {
        id: None,
        question,
        answer,
        folder: None,
        path: None,
    })
}

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
        // - - - (A line in markdown)
        // Or the format
        // #flashcard [#tags ...]
        // question text (possibly over multiple lines)
        // - - - (A line in markdown)
        // answer text (possibly over multiple lines)
        // - - - (A line in markdown)
        // ```
        if line.trim().starts_with("#flashcard") {
            let it = line.split('#');
            let tags: Vec<String> = it.map(|s| s.to_string()).collect();
            let card = long_question(&mut line_it, tags).await?;
            flashcards.push(card);
        } else {
            let mut it = line.split('#');
            let question = it.next().unwrap().trim().to_string();
            let tags: Vec<String> = it.map(|s| s.to_string()).collect();
            // Only after this we are certain that this is a flashcard
            if !tags.into_iter().any(|s| s.starts_with("flashcard")) {
                continue;
            }

            if question.is_empty() {
                bail!("Card cannot have empty question text");
            }

            let answer = read_until_hr(&mut line_it).await?;
            if answer.is_empty() {
                bail!("Card cannot have empty answer text");
            }
            flashcards.push(Flashcard {
                id: None,
                question,
                answer,
                folder: None,
                path: None,
            });
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
        assert_eq!(flashcards[0].answer, "answer\n");
    }

    #[tokio::test]
    async fn test_multiple_questions() {
        let markdown = "question #flashcard\nanswer\n---\nquestion2 #flashcard\nanswer2\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 2);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer\n");
        assert_eq!(flashcards[1].question, "question2");
        assert_eq!(flashcards[1].answer, "answer2\n");
    }

    #[tokio::test]
    async fn test_mix_questions_text() {
        let markdown = "question #flashcard\nanswer\n---\nSome random text\nquestion2 #flashcard\nanswer2\n---\nMore random text\nquestion3 #flashcard\nanswer3\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 3);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer\n");
        assert_eq!(flashcards[1].question, "question2");
        assert_eq!(flashcards[1].answer, "answer2\n");
        assert_eq!(flashcards[2].question, "question3");
        assert_eq!(flashcards[2].answer, "answer3\n");
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
        assert_eq!(flashcards.len(), 1);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer\n");
    }

    #[tokio::test]
    async fn test_long_question() {
        let markdown = r#"
Some text
#flashcard
a text
of a question
$x = x + 1$
- - -
An answer
to the question
- - -
Some more text
a normal card #flashcard
a normal answer
- - -
"#;
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await.unwrap();
        assert_eq!(flashcards.len(), 2);
        assert_eq!(
            flashcards[0].question,
            "a text\nof a question\n$x = x + 1$\n"
        );
        assert_eq!(flashcards[0].answer, "An answer\nto the question\n");
        assert_eq!(flashcards[1].question, "a normal card");
        assert_eq!(flashcards[1].answer, "a normal answer\n");
    }

    #[tokio::test]
    async fn test_empty_question() {
        let markdown = "#flashcard\n---\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).await;
        assert!(flashcards.is_err());
    }
}
