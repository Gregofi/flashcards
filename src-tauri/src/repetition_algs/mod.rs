mod naive_exponential;

use crate::models::answer::Answer;

pub trait RepetitionAlgorithm {
    /// Returns true if the question should be offered, otherwise
    /// returns false.
    /// The answers must be sorted by timestamp, with the most recent
    /// answer being the last one.
    fn repeat_question(&self, answers: &[Answer]) -> bool;
}

pub mod prelude {
    pub use super::naive_exponential::NaiveExponentialRA;
    pub use super::RepetitionAlgorithm;
}
