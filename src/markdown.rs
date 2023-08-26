use std::io::{BufRead, Result};
use crate::models::Flashcard;

fn read_markdown<R: BufRead>(reader: R) -> Result<Vec<Flashcard>> {
    let mut flashcards: Vec<Flashcard> = vec![];
    let mut line_it = reader.lines();
    loop {
        let line = line_it.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap()?;
        // Try to parse the format
        // question text ... [#tags ...] #flashcard [#tags ...]
        // answer text (possibly over multiple lines)
        // ---
        let mut it = line.split('#');
        let question = it.next().unwrap().trim().to_string();
        let tags: Vec<String> = it.map(|s| s.to_string()).collect();
        if !tags.into_iter().any(|s| s.starts_with("flashcard")) {
            continue; 
        }
        let answer = line_it.try_fold(String::from(""), |mut acc, line| {
            let line = line.unwrap();
            if !line.starts_with("---") && !line.starts_with("- - -") {
                acc.push_str(line.as_str());
                Ok(acc)
            } else {
                Err(acc)
            }
        });
        let answer = match answer {
            Ok(_) => break,
            Err(answer) => answer,
        }.trim().to_string();
        flashcards.push(Flashcard { question, answer });
    }
    Ok(flashcards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_empty_markdown() {
        let markdown = "";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).unwrap();
        assert_eq!(flashcards.len(), 0);
    }

    #[test]
    fn test_read_markdown_one_question() {
        let markdown = "question #flashcard\nanswer\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).unwrap();
        assert_eq!(flashcards.len(), 1);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer");
    }

    #[test]
    fn test_multiple_questions() {
        let markdown = "question #flashcard\nanswer\n---\nquestion2 #flashcard\nanswer2\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).unwrap();
        assert_eq!(flashcards.len(), 2);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer");
        assert_eq!(flashcards[1].question, "question2");
        assert_eq!(flashcards[1].answer, "answer2");
    }

    #[test]
    fn test_mix_questions_text() {
        let markdown = "question #flashcard\nanswer\n---\nSome random text\nquestion2 #flashcard\nanswer2\n---\nMore random text\nquestion3 #flashcard\nanswer3\n---\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).unwrap();
        assert_eq!(flashcards.len(), 3);
        assert_eq!(flashcards[0].question, "question");
        assert_eq!(flashcards[0].answer, "answer");
        assert_eq!(flashcards[1].question, "question2");
        assert_eq!(flashcards[1].answer, "answer2");
        assert_eq!(flashcards[2].question, "question3");
        assert_eq!(flashcards[2].answer, "answer3");
    }

    #[test]
    fn test_no_questions() {
        let markdown = "Some random text\nSome more random text\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).unwrap();
        assert_eq!(flashcards.len(), 0);
    }

    #[test]
    fn test_unterminated_question() {
        let markdown = "question #flashcard\nanswer\n";
        let reader = Cursor::new(markdown);
        let flashcards = read_markdown(reader).unwrap();
        assert_eq!(flashcards.len(), 0);
    }
}
