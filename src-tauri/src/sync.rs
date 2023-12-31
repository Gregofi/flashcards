use crate::models::prelude::Flashcard;
use strsim::levenshtein;

#[derive(Debug, PartialEq)]
pub enum CardType {
    New(Flashcard),
    Old(Flashcard),
}

/// Sync two collection of flashcards.
/// If a new card has very similar question to an old card, it is considered
/// the same card and the old card with its ID is returned. Otherwise, it is
/// returned as a new card without any ID.
pub async fn sync(old_flashcards: &[Flashcard], new_flashcards: Vec<Flashcard>) -> Vec<CardType> {
    // This function has O(n^2 * string_distance_computation) complexity, which
    // can be devastating. We could and should do something about it, first
    // optimization would be to somehow use the files the flashcards came from
    // or sort them by their texts.
    new_flashcards
        .into_iter()
        .map(|new_card| {
            let similar: Vec<&Flashcard> = old_flashcards
                .iter()
                .filter(|old_card| {
                    let avg_question_len = (old_card.question.len() + new_card.question.len()) / 2;
                    let qe_limit = (avg_question_len as f64 * 0.2).ceil() as usize;

                    // The same card probably exists
                    levenshtein(&old_card.question, &new_card.question) <= qe_limit
                })
                .collect();
            println!("{:?}", similar);
            match similar.len() {
                // TODO: Take the closest.
                0 => CardType::New(new_card),
                _ => CardType::Old(Flashcard {
                    id: similar[0].id,
                    question: new_card.question,
                    answer: new_card.answer,
                    folder: new_card.folder,
                    path: new_card.path,
                }),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Use everywhere
    fn question(q: &str) -> Flashcard {
        Flashcard {
            id: None,
            question: q.to_string(),
            answer: "a1".to_string(),
            folder: None,
            path: None,
        }
    }

    #[tokio::test]
    async fn empty_sync() {
        assert_eq!(sync(&vec![], vec![]).await, vec![]);
    }

    #[tokio::test]
    async fn no_sync() {
        let old = vec![question("q1")];
        let new = vec![question("q1")];
        let synced = sync(&old, new).await;
        assert_eq!(synced.len(), 1);
        assert_eq!(
            synced[0],
            CardType::Old(Flashcard {
                id: None,
                question: "q1".to_string(),
                answer: "a1".to_string(),
                folder: None,
                path: None,
            })
        );
    }

    #[tokio::test]
    async fn sync_one() {
        let old = vec![question("a very loong text that has one typo in it")];
        let new = vec![question("a very long text that has one typo in it")];
        let synced = sync(&old, new).await;
        assert_eq!(synced.len(), 1);
        assert_eq!(
            synced[0],
            CardType::Old(Flashcard {
                id: None,
                question: "a very long text that has one typo in it".to_string(),
                answer: "a1".to_string(),
                folder: None,
                path: None,
            })
        );
    }

    #[tokio::test]
    async fn add_and_remove() {
        let old = vec![question("a flashcard")];
        let new = vec![question("arghargh")];
        let synced = sync(&old, new).await;
        assert_eq!(synced.len(), 1);
        assert_eq!(
            synced[0],
            CardType::New(Flashcard {
                id: None,
                question: "arghargh".to_string(),
                answer: "a1".to_string(),
                folder: None,
                path: None,
            })
        );
    }

    #[tokio::test]
    async fn sync_multiple() {
        let old = vec![
            question("text of question 1 is very interesting"),
            question("text of question 2 is a bore to be honest"),
        ];
        let new = vec![
            question("A new flashcard! What a day."),
            question("text of question 2 is a bore to be honest."),
            question("Some fascinating text"),
        ];
        let synced = sync(&old, new).await;
        assert_eq!(synced.len(), 3);
        assert_eq!(
            synced[0],
            CardType::New(Flashcard {
                id: None,
                question: "A new flashcard! What a day.".to_string(),
                answer: "a1".to_string(),
                folder: None,
                path: None,
            })
        );
        assert_eq!(
            synced[1],
            CardType::Old(Flashcard {
                id: None,
                question: "text of question 2 is a bore to be honest.".to_string(),
                answer: "a1".to_string(),
                folder: None,
                path: None,
            })
        );
        assert_eq!(
            synced[2],
            CardType::New(Flashcard {
                id: None,
                question: "Some fascinating text".to_string(),
                answer: "a1".to_string(),
                folder: None,
                path: None,
            })
        );
    }

    #[tokio::test]
    async fn multiple_similar() {
        let new = [question("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")];
        let old = [
            question("baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            question("abaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        ];
        let synced = sync(&old, new.to_vec()).await;
        assert_eq!(synced.len(), 1);
        assert!(matches!(synced[0], CardType::Old(_)));
    }
}
