use crate::models::Answer;

trait SPA {
    /// Returns true if the question should be offered, otherwise
    /// returns false.
    /// The answers must be sorted by timestamp, with the most recent
    /// answer being the last one.
    fn repeat_question(&self, answers: &Vec<Answer>) -> bool;
}

/// A naive exponential spaced repetition algorithm.
/// The questions are repeated after 2^N days, where N is the number of
/// consequent good questions. It does not take into account the
/// previous results, only the last N consequent good answers.
struct NaiveExponentialSPA {
    minimum_rating: i32,
    /// A maximum day limit which overrides the normal exponential procedure.
    /// If the number of days between the last answer and now is greater than
    /// this, then the question will always be repeated.
    limit: i32,
}

impl NaiveExponentialSPA {
    fn is_correct(&self, answer: &Answer) -> bool {
        answer.answer_rating >= self.minimum_rating
    }
}

impl SPA for NaiveExponentialSPA {
    fn repeat_question(&self, answers: &Vec<Answer>) -> bool {
        match answers.len() {
            0 => true,
            len => {
                if !self.is_correct(&answers[len - 1]) {
                    true
                } else {
                    // Find the last incorrect answer => all following answers are correct
                    let good_questions =
                        match answers.into_iter().rev().position(|a| !self.is_correct(a)) {
                            Some(idx) => idx,
                            None => len,
                        };
                    // The questions are repeated after 2^N days, where N is the number of consequent good questions
                    let day_limit = 2i32.pow(good_questions as u32 - 1);
                    if day_limit >= self.limit {
                        return true;
                    }
                    let now = chrono::Utc::now().naive_utc();
                    let days_between = now
                        .signed_duration_since(answers.last().unwrap().timestamp)
                        .num_days();
                    println!("days_between > day_limit: {} > {}", days_between, day_limit);
                    days_between >= day_limit as i64
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RIGHT: i32 = 100;
    const WRONG: i32 = 0;

    #[test]
    fn last_incorrect() {
        // Last incorrect always leads to repetition.
        let spa = NaiveExponentialSPA {
            minimum_rating: 50,
            limit: 128,
        };
        let answers = vec![
            Answer {
                id: 1,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(1))
                    .unwrap(),
            },
            Answer {
                id: 2,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(2))
                    .unwrap(),
            },
            Answer {
                id: 3,
                flashcard_id: 1,
                answer_rating: WRONG,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(3))
                    .unwrap(),
            },
        ];
        assert!(spa.repeat_question(&answers));
    }

    #[test]
    fn repeat_one() {
        let spa = NaiveExponentialSPA {
            minimum_rating: 50,
            limit: 128,
        };
        let answers = vec![Answer {
            id: 1,
            flashcard_id: 1,
            answer_rating: RIGHT,
            timestamp: chrono::Utc::now()
                .naive_utc()
                .checked_sub_days(chrono::Days::new(1))
                .unwrap(),
        }];
        assert!(spa.repeat_question(&answers));
    }

    #[test]
    fn repeat_question() {
        let spa = NaiveExponentialSPA {
            minimum_rating: 50,
            limit: 128,
        };
        let answers = vec![
            Answer {
                id: 1,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(7))
                    .unwrap(),
            },
            Answer {
                id: 2,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(6))
                    .unwrap(),
            },
            Answer {
                id: 3,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(4))
                    .unwrap(),
            },
        ];
        assert!(spa.repeat_question(&answers));
    }

    #[test]
    fn dont_repeat_question() {
        let spa = NaiveExponentialSPA {
            minimum_rating: 50,
            limit: 128,
        };
        let answers = vec![
            Answer {
                id: 1,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(5))
                    .unwrap(),
            },
            Answer {
                id: 2,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(4))
                    .unwrap(),
            },
            Answer {
                id: 3,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(3))
                    .unwrap(),
            },
        ];
        assert!(!spa.repeat_question(&answers));
    }

    #[test]
    fn last_over_limit() {
        let spa = NaiveExponentialSPA {
            minimum_rating: 50,
            limit: 8,
        };
        let answers = vec![
            Answer {
                id: 1,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(8))
                    .unwrap(),
            },
            Answer {
                id: 2,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(6))
                    .unwrap(),
            },
            Answer {
                id: 3,
                flashcard_id: 1,
                answer_rating: RIGHT,
                timestamp: chrono::Utc::now()
                    .naive_utc()
                    .checked_sub_days(chrono::Days::new(4))
                    .unwrap(),
            },
        ];
        assert!(spa.repeat_question(&answers));
    }

    #[test]
    fn no_questions() {
        let spa = NaiveExponentialSPA {
            minimum_rating: 50,
            limit: 128,
        };
        let answers = vec![];
        assert!(spa.repeat_question(&answers));
    }
}
